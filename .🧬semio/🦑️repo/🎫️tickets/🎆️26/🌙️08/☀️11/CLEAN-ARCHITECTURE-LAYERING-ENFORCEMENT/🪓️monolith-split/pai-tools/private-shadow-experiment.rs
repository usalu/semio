#![deny(warnings, unconditional_recursion)]
#![allow(async_fn_in_trait)]
#![allow(dead_code)]
pub trait PluginApp: Send + 'static {
    fn close_terminal_is_empty(&self) -> bool;
    async fn render(&mut self, key: &str, cx: &mut Vec<u8>) -> Result<Option<()>, String>;
    fn begin(&mut self, op: u64, archive: Vec<u8>) -> Result<(), String>;
}
pub struct App<A, M> { a: A, m: M, flag: bool }
mod imp {
    use super::*;
    mod child {
        use super::*;
        impl<A: Send + 'static, M: Send + 'static> App<A, M> {
            pub(super) fn close_terminal_is_empty(&self) -> bool { self.flag && self.helper() }
            pub(super) async fn render(&mut self, key: &str, cx: &mut Vec<u8>) -> Result<Option<()>, String> { cx.push(key.len() as u8); Ok(Some(())) }
            pub(super) fn begin(&mut self, op: u64, mut archive: Vec<u8>) -> Result<(), String> { archive.push(op as u8); Ok(()) }
            fn helper(&self) -> bool { true }
        }
    }
    impl<A: Send + 'static, M: Send + 'static> PluginApp for App<A, M> {
        fn close_terminal_is_empty(&self) -> bool { self.close_terminal_is_empty() }
        async fn render(&mut self, key: &str, cx: &mut Vec<u8>) -> Result<Option<()>, String> { self.render(key, cx).await }
        fn begin(&mut self, op: u64, archive: Vec<u8>) -> Result<(), String> { self.begin(op, archive) }
    }
}
mod sibling {
    use super::*;
    pub fn call<A: Send + 'static, M: Send + 'static>(a: &App<A, M>) -> bool { a.close_terminal_is_empty() }
    pub fn call_mut<A: Send + 'static, M: Send + 'static>(a: &mut App<A, M>) -> Result<(), String> { a.begin(1, vec![]) }
    pub async fn r<A: Send + 'static, M: Send + 'static>(a: &mut App<A, M>) { let mut v = vec![]; let _ = a.render("k", &mut v).await; }
}
fn main() {
    let mut a = App { a: 1u8, m: 2u8, flag: true };
    assert!(sibling::call(&a));
    sibling::call_mut(&mut a).unwrap();
    let _ = sibling::r(&mut a);
    println!("ok");
}
