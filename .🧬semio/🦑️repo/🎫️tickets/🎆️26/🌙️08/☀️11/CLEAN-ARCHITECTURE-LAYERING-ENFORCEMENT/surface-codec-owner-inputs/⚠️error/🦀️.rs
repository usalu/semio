//! 🧯️ Owned NodeGraph categories retain private diagnostic causes.
use std::{error::Error,fmt};
/// 🧭️ The graph boundary that refused an operation.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum NodeGraphErrorKind { Json, Pack, Dag }
/// 🧯️ An owned graph refusal with an inspectable standard diagnostic chain.
#[derive(Debug)]
pub struct NodeGraphError { kind:NodeGraphErrorKind,cause:Box<dyn Error+Send+Sync> }
impl NodeGraphError {
 /// 🪪️ Returns the defining graph failure category.
 pub const fn kind(&self)->NodeGraphErrorKind {self.kind}
 pub(super) fn from_cause(kind:NodeGraphErrorKind,cause:impl Error+Send+Sync+'static)->Self {Self{kind,cause:Box::new(cause)}}
}
impl fmt::Display for NodeGraphError {fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {self.cause.fmt(f)}}
impl Error for NodeGraphError {fn source(&self)->Option<&(dyn Error+'static)> {Some(self.cause.as_ref())}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
