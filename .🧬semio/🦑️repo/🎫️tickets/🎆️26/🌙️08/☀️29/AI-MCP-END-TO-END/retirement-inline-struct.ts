import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),f="🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs",s=readFileSync(f,"utf8");let n=s;const a=n.indexOf("#[macro_export]\nmacro_rules! artifact_retire_struct"),b=n.indexOf("\nartifact_retire_leaf!",a);if(a<0||b<0)throw Error("struct macro");n=n.slice(0,a)+`/// 🪑️ Static field access is valid only for the selected still-initialized original field.
pub unsafe trait RetirementStructFields:RetireOwned+Sized{
 const FIELD_COUNT:usize;
 unsafe fn field_demand(original:*const Self,index:usize)->Result<crate::RetirementDemand,crate::ValueError>;
 unsafe fn admit_field(original:*mut Self,index:usize,target:&mut Option<Box<dyn RetirementCursor>>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome;
}
/// 🧱️ Holds partially retired struct storage without constructing references to invalid whole values.
pub struct StructRetirementCursor<T:RetirementStructFields>{original:std::mem::MaybeUninit<T>,index:usize,pending:Option<Box<dyn RetirementCursor>>}
impl<T:RetirementStructFields> StructRetirementCursor<T>{pub fn new(original:T)->Self{Self{original:std::mem::MaybeUninit::new(original),index:0,pending:None}}}
impl<T:RetirementStructFields> RetirementCursor for StructRetirementCursor<T>{
 fn next_computational_work(&self,_:usize)->Result<u64,crate::ValueError>{Ok(if self.terminal_is_empty(){0}else if self.pending.is_some(){1}else{retirement_cursor_birth_work()})}
 fn next_demand(&self,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if self.pending.is_some(){return Ok(crate::RetirementDemand{copy_bytes:size_of::<Option<Box<dyn RetirementCursor>>>()+size_of::<usize>(),depth:1,..Default::default()})}unsafe{T::field_demand(self.original.as_ptr(),self.index)}}
 fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep{if self.terminal_is_empty(){return RetirementStep::Complete(0)}let d=match self.next_demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetirementStep::Failure(error.into(),0)};if !crate::retained_clone::retained_clone_grant_admits(grant,d){return RetirementStep::BudgetExhausted}if let Some(child)=self.pending.take(){self.index+=1;return RetirementStep::ChildProgress(child,crate::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},1)}let turn=unsafe{T::admit_field(self.original.as_mut_ptr(),self.index,&mut self.pending,grant,actual)};match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error.into(),turn.work)}}
 fn terminal_is_empty(&self)->bool{self.index==T::FIELD_COUNT&&self.pending.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<T:RetirementStructFields> Drop for StructRetirementCursor<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"struct retirement retains initialized original fields")}}
/// 🏷️ Infers the exact authored field cursor from its still initialized original.
pub fn retirement_field_demand<T:RetireOwned>(original:&T)->Result<crate::RetirementDemand,crate::ValueError>{if !original.retirement_ready(){return Err(crate::ValueError::invariant("original struct field requires recovery before retirement"))}retirement_cursor_birth_demand::<T>()}
#[macro_export]
macro_rules! artifact_retire_struct {
 ($type:ty { $($field:ident),+ $(,)? }) => {
  unsafe impl $crate::retirement::RetirementStructFields for $type{
   const FIELD_COUNT:usize=[$(stringify!($field)),+].len();
   unsafe fn field_demand(original:*const Self,index:usize)->Result<$crate::RetirementDemand,$crate::ValueError>{let mut rank=0;$(if index==rank{return $crate::retirement::retirement_field_demand(unsafe{&(*original).$field})}rank+=1;)+let _=rank;Err($crate::ValueError::invariant("original struct field is outside its authored extent"))}
   unsafe fn admit_field(original:*mut Self,index:usize,target:&mut Option<Box<dyn $crate::retirement::RetirementCursor>>,grant:$crate::retained_clone::RetainedCloneGrant,actual:&mut Option<usize>)->$crate::RetainedWorkOutcome{let mut rank=0;$(if index==rank{return $crate::retirement::admit_retirement_cursor_from(target,grant,actual,||unsafe{std::ptr::addr_of!((*original).$field).read()})}rank+=1;)+let _=rank;$crate::RetainedWorkOutcome{result:Err($crate::ValueError::invariant("original struct field is outside its authored extent")),work:0}}
  }
  impl $crate::retirement::RetireOwned for $type{
   type Cursor=$crate::retirement::StructRetirementCursor<Self>;
   fn retirement(self)->Self::Cursor{$crate::retirement::StructRetirementCursor::new(self)}
   fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self::Cursor>())}
   fn controlled_retirement_supported()->bool{true}
  }
 };
}
`+n.slice(b);
const tmp=join(t,"🗑️generated/retirement-struct-"+Date.now()+".rs");writeFileSync(tmp,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent retirement");renameSync(tmp,f);console.log("Struct macro inline cursor preserves initialized field custody through funded child admissions");
