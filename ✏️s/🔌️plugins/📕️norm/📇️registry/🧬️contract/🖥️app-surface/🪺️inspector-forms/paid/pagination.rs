//! 🍱 Field and scalar-choice pages share closed offsets and keep the current choice resident.
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"norm.inspector.page")}
/// 📄️ An exact content window leaves two independent navigation slots.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NormInspectorFieldPage{pub start:usize,pub end:usize,pub previous:Option<usize>,pub next:Option<usize>}
impl NormInspectorFieldPage{
 pub fn admit(count:usize,offset:usize)->Result<Self,ValueError>{
  if offset>count||offset==count&&offset!=0{return Err(invalid())}
  let end=offset+(count-offset).min(16);
  Ok(Self{start:offset,end,previous:(offset>0).then(||offset.saturating_sub(16)),next:(end<count).then_some(end)})
 }
}
/// 🔖️ Thirty-one ordinary choices leave room for one current value from another page.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NormInspectorChoicePage{pub start:usize,pub end:usize,pub previous:Option<usize>,pub next:Option<usize>,pub resident:Option<usize>}
impl NormInspectorChoicePage{
 pub fn admit(count:usize,offset:usize,selected:Option<usize>)->Result<Self,ValueError>{
  let stride=if count>32{31}else{32};
  if count==0||offset>=count||count<=32&&offset!=0||offset%stride!=0||selected.is_some_and(|selected|selected>=count){return Err(invalid())}
  let end=offset+(count-offset).min(stride);
  Ok(Self{start:offset,end,previous:(offset>0).then(||offset-stride),next:(end<count).then_some(end),resident:selected.filter(|selected|*selected<offset||*selected>=end)})
 }
 pub fn len(self)->usize{self.end-self.start+usize::from(self.resident.is_some())}
 pub fn is_empty(self)->bool{self.len()==0}
 pub fn index(self,index:usize)->Option<usize>{
  if index>=self.len(){return None}
  if let Some(resident)=self.resident{if index==0{return Some(resident)}return Some(self.start+index-1)}
  Some(self.start+index)
 }
}
#[cfg(test)]
#[path="pagination-tests.rs"]
mod tests;
