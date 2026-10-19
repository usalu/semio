#![allow(unused_braces)]
pub struct Snap; pub struct SDiff; pub struct Out<D>(pub D); pub struct Err;
pub trait Mutation<P>: Sized { type Diff; const DESCRIPTORS: &'static [u8]; fn diff(&self, base: &P) -> Out<Self::Diff>; fn inverse(&self, base: &P) -> Result<Vec<Self>, Err>; }
pub trait MutationKind<P, Op: Mutation<P>>: Clone { const SEMANTICS: &'static str; fn diff(&self, base: &P) -> Out<<Op as Mutation<P>>::Diff>; fn inverse(&self, base: &P) -> Result<Vec<Op>, Err>; }
mod leaves {
    use super::*;
    pub mod a { use super::*; #[derive(Clone)] pub struct A;
      impl<Op: SnapOp> MutationKind<Snap, Op> for A { const SEMANTICS: &'static str = "a"; fn diff(&self, b: &Snap) -> Out<SDiff> { inner::diff(self, b) } fn inverse(&self, b: &Snap) -> Result<Vec<Op>, Err> { Ok({ inner::inverse(self, b)? }) } }
      mod inner { use super::*; pub fn diff(_: &A, _: &Snap) -> Out<SDiff> { Out(SDiff) } pub fn inverse<Op: SnapOp>(_: &A, _: &Snap) -> Result<Vec<Op>, Err> { Ok((|| { vec![Op::from(crate::leaves::b::B)] })()) } }
    }
    pub mod b { use super::*; #[derive(Clone)] pub struct B;
      impl<Op: SnapOp> MutationKind<Snap, Op> for B { const SEMANTICS: &'static str = "b"; fn diff(&self, _: &Snap) -> Out<SDiff> { Out(SDiff) } fn inverse(&self, _: &Snap) -> Result<Vec<Op>, Err> { Ok(vec![Op::from(a::A)]) } }
    }
    pub trait SnapOp: Mutation<Snap, Diff = SDiff> + From<a::A> + From<b::B> {}
    impl<T> SnapOp for T where T: Mutation<Snap, Diff = SDiff> + From<a::A> + From<b::B> {}
}
use leaves::{a::A, b::B};
pub enum Agg { A(A), B(B) }
impl From<A> for Agg { fn from(x: A) -> Self { let _ = <Self as Mutation<Snap>>::DESCRIPTORS; Agg::A(x) } }
impl From<B> for Agg { fn from(x: B) -> Self { Agg::B(x) } }
impl Mutation<Snap> for Agg {
    type Diff = SDiff;
    const DESCRIPTORS: &'static [u8] = { const _K: [&str; 2] = [<A as MutationKind<Snap, Agg>>::SEMANTICS, <B as MutationKind<Snap, Agg>>::SEMANTICS]; &[1, 2] };
    fn diff(&self, b: &Snap) -> Out<SDiff> { match self { Agg::A(p) => <A as MutationKind<Snap, Agg>>::diff(p, b), Agg::B(p) => <B as MutationKind<Snap, Agg>>::diff(p, b) } }
    fn inverse(&self, b: &Snap) -> Result<Vec<Self>, Err> { match self { Agg::A(p) => <A as MutationKind<Snap, Agg>>::inverse(p, b), Agg::B(p) => <B as MutationKind<Snap, Agg>>::inverse(p, b) } }
}
fn main() { let _ = Agg::A(A).inverse(&Snap).ok().map(|v| v.len()); }
