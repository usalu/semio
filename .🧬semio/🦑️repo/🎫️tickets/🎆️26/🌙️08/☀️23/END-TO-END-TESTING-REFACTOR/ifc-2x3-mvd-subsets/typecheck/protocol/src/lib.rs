//! 🧪️ Faithful stand-in for the framework's `protocol` surface that the IFC2X3 MVD mutation
//! modules touch: `Mutation`, `MutationOutcome`, `MutationDiff`, `MutationApplyError` and the
//! `os_spr::command::DiffAlgebra` re-export. Signatures copied from
//! `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`.

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MutationApplyError {
    pub code: String,
    pub message: String,
    pub target: Vec<String>,
}

pub type MutationApplyResult<P> = Result<P, MutationApplyError>;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MutationMessage {
    pub code: String,
    pub message: String,
    pub target: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MutationOutcome<D> {
    diff: D,
    messages: Vec<MutationMessage>,
}

impl<D: Default> MutationOutcome<D> {
    pub fn error(code: impl Into<String>, message: impl Into<String>, target: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self { diff: D::default(), messages: vec![MutationMessage { code: code.into(), message: message.into(), target: target.into_iter().map(Into::into).collect() }] }
    }
}

impl<D> MutationOutcome<D> {
    pub fn new(diff: D) -> Self {
        Self { diff, messages: Vec::new() }
    }
    pub fn diff(&self) -> &D {
        &self.diff
    }
    pub fn messages(&self) -> &[MutationMessage] {
        &self.messages
    }
    pub fn absorb_messages(mut self, messages: impl IntoIterator<Item = MutationMessage>) -> Self {
        self.messages.extend(messages);
        self
    }
}

pub trait MutationDiff<P>: Clone + Default + serde::Serialize + serde::de::DeserializeOwned {
    fn apply(&self, base: &P) -> MutationApplyResult<P>;
    fn absorb(&mut self, other: Self);
}

pub trait DiffAlgebra<P>: Sized {
    fn inverse(&self, base: &P) -> Self;
    fn between(base: &P, other: &P) -> Self;
    fn is_empty(&self) -> bool;
}

pub trait Mutation<P>: Clone + serde::Serialize + serde::de::DeserializeOwned {
    type Diff: MutationDiff<P>;
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Vec<Self>;
}

pub mod os_spr {
    pub mod command {
        pub use crate::DiffAlgebra;
    }
}

/// 🧪️ Minimal non-suspending executor for the harness's rewritten async tests.
pub fn block_on<F: std::future::Future>(mut future: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RawWaker::new(std::ptr::null(), &VTABLE), |_| {}, |_| {}, |_| {});
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    let mut pinned = unsafe { std::pin::Pin::new_unchecked(&mut future) };
    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => panic!("the harness executor only drives non-suspending futures"),
        }
    }
}
