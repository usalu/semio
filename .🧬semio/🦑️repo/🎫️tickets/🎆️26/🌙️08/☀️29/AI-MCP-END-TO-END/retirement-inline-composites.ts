import{readFileSync,writeFileSync,renameSync}from"node:fs";import{dirname,join}from"node:path";const t=dirname(import.meta.path),f="🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs",s=readFileSync(f,"utf8");let n=s;function rep(a:string,z:string){if(n.split(a).length!==2)throw Error("anchor "+a.slice(0,120));n=n.replace(a,z)}function region(a:string,b:string,z:string){const i=n.indexOf(a),j=n.indexOf(b,i+a.length);if(i<0||j<0)throw Error("region "+a);n=n.slice(0,i)+z+n.slice(j)}
region("macro_rules! retire_tuple {","retire_tuple!(A:0,B:1);",`/// 🧺️ Keeps every tuple field inline until its own funded cursor birth.
pub struct TupleRetirementFields<T:Send+'static>(ManuallyDrop<T>);
macro_rules! retire_tuple {
 ($($type:ident:$index:tt),+) => {
  impl<$($type:RetireOwned),+> RetirementFields for TupleRetirementFields<($(Option<$type>,)+)>{
   fn field_count(&self)->usize{[$(stringify!($index)),+].len()}
   fn field_demand(&self,index:usize)->Result<crate::RetirementDemand,crate::ValueError>{match index{$($index=>retirement_cursor_birth_demand::<$type>(),)+_=>Err(crate::ValueError::invariant("tuple retirement field is outside its original extent"))}}
   fn admit_field(&mut self,index:usize,target:&mut Option<Box<dyn RetirementCursor>>,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome{match index{$($index=>admit_retirement_cursor(&mut self.0.$index,target,grant,actual),)+_=>crate::RetainedWorkOutcome{result:Err(crate::ValueError::invariant("tuple retirement field is outside its original extent")),work:0}}}
  }
  impl<$($type:RetireOwned),+> RetireOwned for ($($type,)+){
   type Cursor=FieldsRetirementCursor<TupleRetirementFields<($(Option<$type>,)+)>>;
   fn retirement(self)->Self::Cursor{FieldsRetirementCursor::new(TupleRetirementFields(ManuallyDrop::new(($(Some(self.$index),)+))))}
   fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::Cursor>())}
   fn controlled_retirement_supported()->bool{support::controlled_retirement_supported::<Self>()}
   fn controlled_retirement_supported_in(path:&RetirementSupport<'_>)->bool{true$(&&path.supports::<$type>())+}
   fn retirement_element_copy_bytes()->usize{composite_copy_bytes::<Self>(&[$($type::retirement_element_copy_bytes()),+])}
  }
 };
}
`);
rep("fn retirement(self) -> Box<dyn RetirementCursor> {\n        if plain_elements::<T>() { return Box::new(PlainArray { items: ManuallyDrop::new(self), remaining: N }); }\n        Box::new(FixedArray { items: ManuallyDrop::new(self.map(Some)) })\n    }","type Cursor=ArrayRetirement<T,N>;\n    fn retirement(self)->Self::Cursor{if plain_elements::<T>(){ArrayRetirement::Plain(PlainArray {items:ManuallyDrop::new(self),remaining:N})}else{ArrayRetirement::Fields(FixedArray {items:ManuallyDrop::new(self.map(Some))})}}");
rep("Some(if plain_elements::<T>() { size_of::<PlainArray<T, N>>() } else { size_of::<FixedArray<T, N>>() })","Some(size_of::<Self::Cursor>())");
rep("impl<T: RetireOwned, const N: usize> RetireOwned for [T; N] {",`/// 🧱️ Selects the original array's concrete inline retirement strategy without allocating.
pub enum ArrayRetirement<T:RetireOwned,const N:usize>{Plain(PlainArray<T,N>),Fields(FixedArray<T,N>)}
impl<T:RetireOwned,const N:usize> RetirementCursor for ArrayRetirement<T,N>{
 fn next_computational_work(&self,body:usize)->Result<u64,crate::ValueError>{match self{Self::Plain(c)=>c.next_computational_work(body),Self::Fields(c)=>c.next_computational_work(body)}}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{match self{Self::Plain(c)=>c.next_demand(body),Self::Fields(c)=>c.next_demand(body)}}
 fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->RetirementStep{match self{Self::Plain(c)=>c.close_step(grant,actual),Self::Fields(c)=>c.close_step(grant,actual)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Plain(c)=>c.terminal_is_empty(),Self::Fields(c)=>c.terminal_is_empty()}}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{match self{Self::Plain(c)=>c.next_work_byte_demand(),Self::Fields(c)=>c.next_work_byte_demand()}}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{match self{Self::Plain(c)=>c.next_selected_work_byte_demand(body),Self::Fields(c)=>c.next_selected_work_byte_demand(body)}}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{match self{Self::Plain(c)=>c.next_birth_bytes(body),Self::Fields(c)=>c.next_birth_bytes(body)}}
 fn next_close_byte_demand(&self)->Option<usize>{match self{Self::Plain(c)=>c.next_close_byte_demand(),Self::Fields(c)=>c.next_close_byte_demand()}}
 fn next_depth_demand(&self)->Result<usize,crate::ValueError>{match self{Self::Plain(c)=>c.next_depth_demand(),Self::Fields(c)=>c.next_depth_demand()}}
 fn allows_admitted_narrow_work(&self)->bool{match self{Self::Plain(c)=>c.allows_admitted_narrow_work(),Self::Fields(c)=>c.allows_admitted_narrow_work()}}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
impl<T: RetireOwned, const N: usize> RetireOwned for [T; N] {`);
const tmp=join(t,"🗑️generated/retirement-inline-composite-"+Date.now()+".rs");writeFileSync(tmp,n);if(readFileSync(f,"utf8")!==s)throw Error("concurrent retirement");renameSync(tmp,f);console.log("Tuple and array concrete inline cursor construction authored");
