//! 🔎️ Borrowed semantic targets resolve by original row ordinal without allocating key strings.
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
/// 👁 Each collection lends its original keys through constant-time source addressing.
pub trait LayoutTargetSource{fn identity(&self)->usize;fn count(&self)->usize;fn key(&self,index:usize)->&str;}
/// 🔢 The lookup owns only fixed-size coordinates and source witnesses.
pub struct LayoutTargetLookup{identity:usize,count:usize,target_identity:usize,target_pointer:usize,target_len:usize,index:usize,offset:usize,row_pointer:usize,row_len:usize,found:Option<usize>,started:bool,done:bool,closing:bool}
impl Default for LayoutTargetLookup{fn default()->Self{Self::new()}}
impl LayoutTargetLookup{
 pub fn new()->Self{Self{identity:0,count:0,target_identity:0,target_pointer:0,target_len:0,index:0,offset:0,row_pointer:0,row_len:0,found:None,started:false,done:false,closing:false}}
 fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout target source identity changed")}
 pub fn demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:768,depth:1,..Default::default()}}
 pub fn advance(&mut self,s:&impl LayoutTargetSource,target_identity:usize,target:&str,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.closing||self.done{return Err(Self::fault())}let d=self.demand();if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if !self.started{self.identity=s.identity();self.count=s.count();self.target_identity=target_identity;self.target_pointer=target.as_ptr()as usize;self.target_len=target.len();self.started=true}else if self.identity!=s.identity()||self.count!=s.count()||self.target_identity!=target_identity||self.target_pointer!=target.as_ptr()as usize||self.target_len!=target.len(){return Err(Self::fault())}let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};if self.index==self.count{self.done=true;return Ok(RetainedCloneStep::Complete(p))}let key=s.key(self.index);if self.offset==0{self.row_pointer=key.as_ptr()as usize;self.row_len=key.len()}else if self.row_pointer!=key.as_ptr()as usize||self.row_len!=key.len(){return Err(Self::fault())}if key.len()!=target.len(){self.index+=1;self.offset=0;return Ok(RetainedCloneStep::Progress(p))}let end=target.len().min(self.offset.saturating_add(256));if key.as_bytes()[self.offset..end]!=target.as_bytes()[self.offset..end]{self.index+=1;self.offset=0}else if end==target.len(){self.found=Some(self.index);self.done=true;return Ok(RetainedCloneStep::Complete(p))}else{self.offset=end}Ok(RetainedCloneStep::Progress(p))}
 pub fn result(&self)->Option<Option<usize>>{self.done.then_some(self.found)}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:std::mem::size_of::<Self>()*2,depth:1,..Default::default()}}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->RetainedCloneStep{let d=self.close_demand();if !self.closing||!retained_clone_grant_admits(g,d){return RetainedCloneStep::Progress(Default::default())}self.started=false;self.done=false;self.found=None;RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})}
 pub fn terminal_is_empty(&self)->bool{self.closing&&!self.started&&!self.done&&self.found.is_none()}
}
