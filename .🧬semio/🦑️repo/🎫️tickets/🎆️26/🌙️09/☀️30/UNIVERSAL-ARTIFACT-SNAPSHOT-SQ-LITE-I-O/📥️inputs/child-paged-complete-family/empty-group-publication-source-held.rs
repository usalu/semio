// 🗂️ Empty semantic groups disappear from publication while their original owners remain retained.
use super::{ChildEmit,PluginCloseStep};
use super::source_codec::{ChildGroupSources,ChildGroupSource};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,list::PagedList};

/// 🔢️ A bounded ordinal plan borrows every published group from the complete retained child owner.
pub struct ChildGroupPublicationSelection{indices:PagedList<usize,{isize::MAX as usize}>,next:usize,complete:bool}
impl ChildGroupPublicationSelection{
    pub fn new()->Self{Self{indices:PagedList::empty(),next:0,complete:false}}
    /// ⛽️ Selects one actual nonempty semantic group without disposing an empty child or copying its source.
    pub fn step<const N:usize>(&mut self,children:&PagedList<ChildEmit,N>,maximum_items:usize,maximum_bytes:usize,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
        if self.complete{return Ok(true)}if maximum_items==0||maximum_bytes==0{return Ok(false)}
        if self.next==children.len(){self.complete=true;return Ok(true)}
        control.checkpoint()?;let child=&children[self.next];
        if !child.ops.is_empty(){
            if !self.indices.has_reserved_slot(){let required=self.indices.next_allocation_bytes().map_err(ValueError::from)?;if required>maximum_bytes{return Ok(false)}control.charge(required)?;let step=self.indices.reserve_one(maximum_bytes).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"child publication selection admitted no physical backing"))}return Ok(false)}
            self.indices.push_reserved(self.next).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"child publication selection lost its admitted slot"))?;
        }
        self.next+=1;control.step()?;Ok(false)
    }
    /// 🫳️ The complete selection keeps the full caller-owned collection borrowed through publication.
    pub fn sources<'a,const N:usize>(&'a self,children:&'a PagedList<ChildEmit,N>)->Result<SelectedChildGroupSources<'a,N>,ValueError>{if !self.complete||self.next!=children.len(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"child publication selection does not cover the complete retained owner"))}Ok(SelectedChildGroupSources{selection:self,children})}
    pub fn close_one(&mut self,maximum_items:usize,maximum_bytes:usize)->Result<PluginCloseStep,ValueError>{if maximum_items==0||maximum_bytes==0{return Ok(PluginCloseStep::Pending{released_items:0,released_bytes:0})}if self.indices.pop().is_some(){return Ok(PluginCloseStep::Pending{released_items:1,released_bytes:0})}if !self.indices.terminal_is_empty(){let step=self.indices.release_empty_page(maximum_bytes).map_err(ValueError::from)?;return Ok(PluginCloseStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes})}Ok(PluginCloseStep::Complete)}
    pub fn terminal_is_empty(&self)->bool{self.indices.terminal_is_empty()}
}
pub struct SelectedChildGroupSources<'a,const N:usize>{selection:&'a ChildGroupPublicationSelection,children:&'a PagedList<ChildEmit,N>}
impl<const N:usize> ChildGroupSources for SelectedChildGroupSources<'_,N>{
    fn len(&self)->usize{self.selection.indices.len()}
    fn group_at(&self,index:usize)->Option<ChildGroupSource<'_>>{let index=*self.selection.indices.get(index)?;let child=self.children.get(index)?;Some(ChildGroupSource{owner:&child.owner,slot:&child.slot,child_id:&child.child_id,schema:&child.op_schema.0,operations:&child.ops,labels:&child.labels})}
}
impl Drop for ChildGroupPublicationSelection{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"child publication selection requires explicit close of its original metadata pages");}}
