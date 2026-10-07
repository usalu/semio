//! 🧭️ Retained paged symbol identities resolved through the same immutable operation source.
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind};
use semio_framework_value::list::PagedList;
use std::cmp::Ordering;

const SYMBOL_CAPACITY:usize=isize::MAX as usize;

/// 📍️ An ordinal source identity owns no copied text and stores no borrowed reference.
#[derive(Clone,Copy)]
pub struct SourceTextLocator{path:[usize;64],depth:u8,kind:SourceTextKind}
/// 🏷️ Literal text roles preserve the original Pack walker's key and intrinsic distinction.
#[derive(Clone,Copy)]
pub enum SourceTextKind{Text,IntrinsicText,Key(usize)}
impl SourceTextLocator{
 /// 🌱️ Captures a bounded source path without reading or owning the semantic payload.
 pub fn new(path:&[usize],kind:SourceTextKind)->Result<Self,ValueError>{
  if path.len()>64{return Err(invalid("symbol locator exceeds immutable path capacity"))}
  let mut locator=Self{path:[0;64],depth:path.len()as u8,kind};locator.path[..path.len()].copy_from_slice(path);Ok(locator)
 }
 /// 🔎️ Resolves text only through its actual original source identity.
 pub fn resolve<'a,T:FieldProjectionSource>(&self,source:&'a T)->Result<&'a str,ValueError>{
  let path=&self.path[..usize::from(self.depth)];match self.kind{
   SourceTextKind::Key(index)=>source.projection_key(path,index),
   SourceTextKind::Text=>match source.projection_view(path)?{FieldProjectionView::Text(text)=>Ok(text),_=>Err(invalid("ordinary symbol source identity changed"))},
   SourceTextKind::IntrinsicText=>match source.projection_view(path)?{FieldProjectionView::IntrinsicText(text)=>Ok(text),_=>Err(invalid("intrinsic symbol source identity changed"))},
  }
 }
}

#[derive(Clone,Copy)]
struct SourceSymbol{locator:SourceTextLocator,forced:bool}
#[derive(Clone,Copy,PartialEq,Eq)]
enum ScratchPhase{Discovering,Sorting,Ready,Retiring}
/// 🎒️ The caller retains every symbol and scalar index backing on success and refusal.
pub struct ProjectedSymbolScratch{
 symbols:PagedList<SourceSymbol,SYMBOL_CAPACITY>,indices:PagedList<usize,SYMBOL_CAPACITY>,phase:ScratchPhase,
}
fn compare_text(left:&str,right:&str,control:&mut NativeEncodeControl<'_>)->Result<Ordering,ValueError>{
 control.scoped_stage(|control|{let length=left.len().min(right.len());control.begin_stage(length)?;for start in(0..length).step_by(256){control.checkpoint()?;let end=(start+256).min(length);let order=left.as_bytes()[start..end].cmp(&right.as_bytes()[start..end]);control.advance(end-start)?;if order!=Ordering::Equal{return Ok(order)}}Ok(left.len().cmp(&right.len()))})
}
fn invalid(reason:&str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,reason)}
fn paged_refusal(reason:&'static str)->ValueError{ValueError::new(ValueRefusalKind::AllocationFailed,reason)}
fn push<T>(owner:&mut PagedList<T,SYMBOL_CAPACITY>,value:T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
 while !owner.has_reserved_slot(){
  let required=owner.next_allocation_bytes().map_err(|error|paged_refusal(error.reason))?;
  if required>4096{return Err(invalid("symbol scratch page exceeds its physical grant"))}
  control.charge(required)?;control.checkpoint()?;
  let progress=owner.reserve_one(required).map_err(|error|paged_refusal(error.reason))?;
  if !progress.progressed||progress.allocated_bytes>required{return Err(invalid("symbol scratch allocation disagrees with paid admission"))}
 }
 owner.push_reserved(value).map_err(|_|invalid("admitted symbol scratch slot disappeared"))
}
impl ProjectedSymbolScratch{
 /// 🫙️ Starts with no backing, no default recipient and no text shadow.
 pub const fn empty()->Self{Self{symbols:PagedList::empty(),indices:PagedList::empty(),phase:ScratchPhase::Discovering}}
 /// 📏️ Reports the actual retained symbol and index backing independently of semantic text.
 pub fn allocated_bytes(&self)->usize{self.symbols.allocated_bytes()+self.indices.allocated_bytes()}
 /// 📝️ Records one occurrence without deduplicating away the first source authority.
 pub fn note<T:FieldProjectionSource>(&mut self,source:&T,locator:SourceTextLocator,forced:bool,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if self.phase!=ScratchPhase::Discovering{return Err(invalid("symbol discovery phase is closed"))}
  control.checkpoint()?;let _=locator.resolve(source)?;push(&mut self.symbols,SourceSymbol{locator,forced},control)
 }
 fn compare<T:FieldProjectionSource>(&self,source:&T,left:usize,right:usize,control:&mut NativeEncodeControl<'_>)->Result<Ordering,ValueError>{
  let order=compare_text(self.symbols[left].locator.resolve(source)?,self.symbols[right].locator.resolve(source)?,control)?;
  Ok(order.then_with(||left.cmp(&right)))
 }
 fn sift<T:FieldProjectionSource>(&mut self,source:&T,mut root:usize,length:usize,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  loop{
   control.checkpoint()?;control.step()?;let Some(left)=root.checked_mul(2).and_then(|index|index.checked_add(1)).filter(|index|*index<length)else{return Ok(())};
   let right=left+1;let largest=if right<length&&self.compare(source,self.indices[left],self.indices[right],control)?==Ordering::Less{right}else{left};
   if self.compare(source,self.indices[root],self.indices[largest],control)?!=Ordering::Less{return Ok(())}
   self.indices.swap(root,largest);root=largest;
  }
 }
 /// 🔤️ Heap-sorts paid scalar indices and retains the first ordinal of every selected text.
 pub fn finish<T:FieldProjectionSource>(&mut self,source:&T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if self.phase!=ScratchPhase::Discovering{return Err(invalid("symbol finalization phase changed"))}
  self.phase=ScratchPhase::Sorting;
  for index in 0..self.symbols.len(){control.checkpoint()?;push(&mut self.indices,index,control)?;}
  let length=self.indices.len();for root in(0..length/2).rev(){self.sift(source,root,length,control)?;}
  for end in(1..length).rev(){self.indices.swap(0,end);self.sift(source,0,end,control)?;}
  let(mut start,mut selected)=(0,0);
  while start<length{
   control.checkpoint()?;control.step()?;let first=self.indices[start];let text=self.symbols[first].locator.resolve(source)?;let mut end=start+1;let mut forced=self.symbols[first].forced;
   while end<length&&compare_text(text,self.symbols[self.indices[end]].locator.resolve(source)?,control)?==Ordering::Equal{forced|=self.symbols[self.indices[end]].forced;end+=1;control.step()?;}
   if forced||text.len()<=128||end-start>=2{self.indices[selected]=first;selected+=1;}
   start=end;
  }
  while self.indices.len()>selected{control.checkpoint()?;control.step()?;self.indices.pop();}
  self.phase=ScratchPhase::Ready;Ok(())
 }
 /// 🧾️ Reads only finalized immutable symbols while the original source remains retained.
 pub fn len(&self)->Result<usize,ValueError>{if self.phase!=ScratchPhase::Ready{return Err(invalid("symbol scratch has no complete grant"))}Ok(self.indices.len())}
 /// 🌿️ Exposes an exact retained original locator without moving semantic ownership.
 pub fn locator(&self,index:usize)->Result<SourceTextLocator,ValueError>{let length=self.len()?;if index>=length{return Err(invalid("selected symbol index is outside its source grant"))}Ok(self.symbols[self.indices[index]].locator)}
 /// 🔍️ Finds a canonical symbol index using paid bounded UTF8 comparison.
 pub fn index<T:FieldProjectionSource>(&self,source:&T,text:&str,control:&mut NativeEncodeControl<'_>)->Result<Option<u64>,ValueError>{
  let(mut low,mut high)=(0,self.len()?);while low<high{let middle=low+(high-low)/2;match compare_text(self.locator(middle)?.resolve(source)?,text,control)?{Ordering::Less=>low=middle+1,Ordering::Greater=>high=middle,Ordering::Equal=>return Ok(Some(middle as u64))}}Ok(None)
 }
 /// ♻️ Drains one scalar item or one real empty page within the explicit physical byte grant.
 pub fn retire_one(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<(bool,usize,usize),ValueError>{
  if maximum_items==0||maximum_bytes==0{return Ok((false,0,0))}self.phase=ScratchPhase::Retiring;
  if !self.indices.is_empty(){self.indices.pop();return Ok((true,1,0))}
  if self.indices.allocated_bytes()>0{let step=self.indices.release_empty_page(maximum_bytes).map_err(|error|paged_refusal(error.reason))?;return Ok((step.progressed,usize::from(step.progressed),step.released_allocation_bytes))}
  if !self.symbols.is_empty(){self.symbols.pop();return Ok((true,1,0))}
  if self.symbols.allocated_bytes()>0{let step=self.symbols.release_empty_page(maximum_bytes).map_err(|error|paged_refusal(error.reason))?;return Ok((step.progressed,usize::from(step.progressed),step.released_allocation_bytes))}
  Ok((false,0,0))
 }
 /// 🏠️ Hands one genuine empty backing to the explicit pre-admitted parent recipient.
 pub fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<(bool,usize),ValueError>{
  if maximum_items==0||maximum_bytes==0{return Ok((false,0))}self.phase=ScratchPhase::Retiring;
  if !self.indices.is_empty(){self.indices.pop();return Ok((true,0))}
  if self.indices.allocated_bytes()>0{if self.indices.next_release_allocation_bytes().map_err(|error|paged_refusal(error.reason))?>maximum_bytes{return Ok((false,0))}let step=self.indices.return_empty_page(parent,1)?;return Ok((step.progressed,step.returned_allocation_bytes))}
  if !self.symbols.is_empty(){self.symbols.pop();return Ok((true,0))}
  if self.symbols.allocated_bytes()>0{if self.symbols.next_release_allocation_bytes().map_err(|error|paged_refusal(error.reason))?>maximum_bytes{return Ok((false,0))}let step=self.symbols.return_empty_page(parent,1)?;return Ok((step.progressed,step.returned_allocation_bytes))}
  Ok((false,0))
 }
}
