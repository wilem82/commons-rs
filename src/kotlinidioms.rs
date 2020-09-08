pub trait Also {
    fn also(self, f: impl Fn(&mut Self)) -> Self;
}

impl<T> Also for T {
    fn also(mut self, f: impl Fn(&mut Self)) -> Self {
        f(&mut self);
        self
    }
}
