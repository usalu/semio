#![allow(dead_code)]
#[derive(Debug,PartialEq)] pub struct ValueError;
#[derive(Clone,Copy)] pub enum ValueRefusalKind{OwnershipLimit,AllocationFailed}
#[derive(Debug,Default,Clone,Copy)] pub struct RetainedCloneProgress{pub retained_capacity_bytes:usize,pub released_bytes:usize}
impl ValueError{pub fn literal(_:ValueRefusalKind,_:&'static str)->Self{Self}pub fn retained_progress(&self)->RetainedCloneProgress{Default::default()}pub fn with_retained_progress(self,_:RetainedCloneProgress)->Self{self}}
impl std::fmt::Display for ValueError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.write_str("value")}}
#[derive(Debug)]pub struct RetainedWorkOutcome<T,E=ValueError>{pub result:Result<T,E>,pub work:u64}
pub enum RetainedCloneStep{Progress}
impl RetainedCloneStep{pub fn progress(&self)->RetainedCloneProgress{Default::default()}}
#[path="/home/deck/semio/🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/♻️failure/🦀️.rs"]mod failure;
