struct InlineOrder{first:Option<usize>,tail:PagedList<usize,ORDER_CAPACITY>}
impl InlineOrder{
 const fn empty()->Self{Self{first:None,tail:PagedList::empty()}}
 fn len(&self)->usize{usize::from(self.first.is_some())+self.tail.len()}
 fn is_empty(&self)->bool{self.first.is_none()&&self.tail.is_empty()}
 fn has_reserved_slot(&self)->bool{self.first.is_none()||self.tail.has_reserved_slot()}
 fn allocated_bytes(&self)->usize{self.tail.allocated_bytes()}
 fn push_reserved(&mut self,value:usize)->Result<(),usize>{if self.first.is_none(){self.first=Some(value);Ok(())}else{self.tail.push_reserved(value)}}
 fn pop(&mut self)->Option<usize>{self.tail.pop().or_else(||self.first.take())}
 fn swap(&mut self,left:usize,right:usize){let value=self[left];self[left]=self[right];self[right]=value;}
}
impl std::ops::Index<usize> for InlineOrder{type Output=usize;fn index(&self,index:usize)->&usize{if index==0{self.first.as_ref().expect("initialized inline order")}else{&self.tail[index-1]}}}
impl std::ops::IndexMut<usize> for InlineOrder{fn index_mut(&mut self,index:usize)->&mut usize{if index==0{self.first.as_mut().expect("initialized inline order")}else{&mut self.tail[index-1]}}}
#[derive(Clone,Copy,PartialEq,Eq)]
enum InlineSymbolPhase{Discovering,Spilling,Ready,Retiring}
struct InlineSymbols{first:Option<(SourceTextLocator,bool)>,overflow:ProjectedSymbolScratch,spilled:bool,selected:bool,phase:InlineSymbolPhase}
impl InlineSymbols{
 const fn empty()->Self{Self{first:None,overflow:ProjectedSymbolScratch::empty(),spilled:false,selected:false,phase:InlineSymbolPhase::Discovering}}
 fn allocated_bytes(&self)->usize{self.overflow.allocated_bytes()}
 fn note<T:FieldProjectionSource>(&mut self,source:&T,locator:SourceTextLocator,forced:bool,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if self.phase!=InlineSymbolPhase::Discovering{return Err(inline_invalid())}control.checkpoint()?;let _=locator.resolve(source)?;
  if self.spilled{return self.overflow.note(source,locator,forced,control)}
  let Some((first,first_forced))=self.first else{self.first=Some((locator,forced));return Ok(())};
  self.phase=InlineSymbolPhase::Spilling;self.overflow.note(source,first,first_forced,control)?;self.overflow.note(source,locator,forced,control)?;self.first=None;self.spilled=true;self.phase=InlineSymbolPhase::Discovering;Ok(())
 }
 fn finish<T:FieldProjectionSource>(&mut self,source:&T,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{
  if self.phase!=InlineSymbolPhase::Discovering{return Err(inline_invalid())}control.checkpoint()?;control.step()?;
  if self.spilled{self.overflow.finish(source,control)?;}else{self.selected=match self.first{Some((locator,forced))=>forced||locator.resolve(source)?.len()<=128,None=>false};}
  self.phase=InlineSymbolPhase::Ready;Ok(())
 }
 fn len(&self)->Result<usize,ValueError>{if self.phase!=InlineSymbolPhase::Ready{return Err(inline_invalid())}if self.spilled{self.overflow.len()}else{Ok(usize::from(self.selected))}}
 fn locator(&self,index:usize)->Result<SourceTextLocator,ValueError>{if index>=self.len()?{return Err(inline_invalid())}if self.spilled{self.overflow.locator(index)}else{self.first.map(|(locator,_)|locator).ok_or_else(inline_invalid)}}
 fn index<T:FieldProjectionSource>(&self,source:&T,text:&str,control:&mut NativeEncodeControl<'_>)->Result<Option<u64>,ValueError>{
  let length=self.len()?;if self.spilled{return self.overflow.index(source,text,control)}if length==0{return Ok(None)}let original=self.locator(0)?.resolve(source)?;controlled_schema::compare_text(original,text,control).map(|order|(order==Ordering::Equal).then_some(0))
 }
 fn retire_one(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<(bool,usize,usize),ValueError>{
  if maximum_items==0||maximum_bytes==0{return Ok((false,0,0))}self.phase=InlineSymbolPhase::Retiring;if self.first.take().is_some(){return Ok((true,1,0))}self.overflow.retire_one(maximum_items,maximum_bytes)
 }
 fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<(bool,usize),ValueError>{
  if maximum_items==0||maximum_bytes==0{return Ok((false,0))}self.phase=InlineSymbolPhase::Retiring;if self.first.take().is_some(){return Ok((true,0))}self.overflow.return_one(parent,maximum_items,maximum_bytes)
 }
}
fn inline_invalid()->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,"inline Pack scratch has no complete source grant")}
