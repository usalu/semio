pub mod open {
    pub fn f() {}
    pub struct S;
    pub enum E { A, B(u8) }
    pub(crate) fn crate_only() {}
    fn private_fn() {}
}
mod hidden {
    pub fn secret() {}
}
pub mod reexport {
    pub use crate::hidden::secret as revealed;
    pub use crate::open::*;
}
pub trait Tr { fn t(&self) {} }
pub fn top() {}
