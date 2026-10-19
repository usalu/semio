extern crate alpha as al;
use alpha::open::{self, E, S};
use alpha::reexport::{revealed, f};
use self::inner::g;
mod inner { pub fn g() {} pub(super) fn h() {} }
#[cfg(test)]
mod tests { use super::*; #[test] fn t() { g(); } }
pub fn run() {
    open::f();
    let _ = S;
    let _ = E::A;
    let _ = E::B(1);
    revealed();
    f();
    al::top();
    inner::h();
    crate::inner::g();
}
