// 🗂️ One retained publication owner binds selected ordinals to its complete original child collection.
use super::{paged_owner::PagedChildOwner,paged_encoder::{PagedChildGroups,PagedChildGroup}};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,SnapshotRetirementStep,list::PagedList,retirement::allocation_return::ParentAllocationReturn};
const CAPACITY:usize=isize::MAX as usize;
fn invalid(detail:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,detail)}
pub struct ChildGroupPublicationOwner{children:PagedList<PagedChildOwner,CAPACITY>,indices:PagedList<usize,CAPACITY>,next:usize,complete:bool,closing:bool}
impl ChildGroupPublicationOwner{
    /// 📥️ Takes the complete original collection only after the caller checkpoint accepts the transfer.
    pub fn take(children:&mut Option<PagedList<PagedChildOwner,CAPACITY>>,control:&mut NativeEncodeControl<'_>)->Result<Self,ValueError>{control.checkpoint()?;if children.is_none(){return Err(invalid("child publication requires the original retained collection"))}Ok(Self{children:children.take().unwrap(),indices:PagedList::empty(),next:0,complete:false,closing:false})}
    pub fn allocated_bytes(&self)->usize{self.indices.allocated_bytes()+self.children.allocated_bytes()+self.children.iter().map(PagedChildOwner::allocated_bytes).sum::<usize>()}
    pub fn original_count(&self)->usize{self.children.len()}
    pub fn original(&self,index:usize)->Option<&PagedChildOwner>{self.children.get(index)}
    /// ⛽️ Every selected index is admitted before publication, while empty children remain original owners.
    pub fn step(&mut self,maximum_items:usize,maximum_bytes:usize,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{
        if self.closing{return Err(invalid("retiring publication cannot select new source authority"))}if self.complete{return Ok(true)}if maximum_items==0||maximum_bytes==0{return Ok(false)}
        if self.next==self.children.len(){self.complete=true;return Ok(true)}control.checkpoint()?;let child=self.children.get(self.next).ok_or_else(||invalid("original publication child ordinal changed"))?;child.view()?;
        if !child.operations.is_empty(){if !self.indices.has_reserved_slot(){let required=self.indices.next_allocation_bytes().map_err(ValueError::from)?;if required>maximum_bytes{return Ok(false)}control.charge(required)?;let step=self.indices.reserve_one(maximum_bytes).map_err(|error|ValueError::from(error.refusal()))?;if !step.progressed||step.allocated_bytes!=required{return Err(invalid("publication index differs from exact preadmission"))}return Ok(false)}self.indices.push_reserved(self.next).map_err(|_|invalid("publication index lost its admitted slot"))?;}
        self.next+=1;control.step()?;Ok(false)
    }
    /// 👓️ The only complete source view borrows the same private collection that selection consumed.
    pub fn sources(&self)->Result<SelectedChildGroupSources<'_>,ValueError>{if self.closing||!self.complete||self.next!=self.children.len(){return Err(invalid("publication lacks a complete live selection"))}Ok(SelectedChildGroupSources{owner:self})}
    /// 🏠️ Forwards real empty allocations to the admitted parent without claiming child physical disposal.
    pub fn return_one<const P:usize>(&mut self,parent:&mut ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        use semio_framework_value::list::PagedListReturnProgress;
        if maximum_items==0||maximum_bytes==0{return Ok(PagedListReturnProgress::default())}self.closing=true;
        if self.indices.pop().is_some(){return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}if !self.indices.terminal_is_empty(){return self.indices.return_empty_page(parent,1)}
        if let Some(index)=self.children.len().checked_sub(1){let child=self.children.get_mut(index).unwrap();if !child.terminal_is_empty(){return child.return_one(parent,1,maximum_bytes)}self.children.pop();return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}if !self.children.terminal_is_empty(){return self.children.return_empty_page(parent,1)}Ok(PagedListReturnProgress::default())
    }
}
impl ErasedSnapshotRetirement for ChildGroupPublicationOwner{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
        if items==0||bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0})}self.closing=true;
        if self.indices.pop().is_some(){return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})}if !self.indices.terminal_is_empty(){let step=self.indices.release_empty_page(bytes).map_err(ValueError::from)?;return Ok(SnapshotRetirementStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes})}
        if let Some(index)=self.children.len().checked_sub(1){let child=self.children.get_mut(index).unwrap();if !child.terminal_is_empty(){return child.close_step(1,bytes)}self.children.pop();return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0})}if !self.children.terminal_is_empty(){let step=self.children.release_empty_page(bytes).map_err(ValueError::from)?;return Ok(SnapshotRetirementStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes})}Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self)->bool{self.indices.terminal_is_empty()&&self.children.terminal_is_empty()}
    fn next_close_byte_demand(&self)->usize{if !self.indices.is_empty(){return 1}if !self.indices.terminal_is_empty(){return self.indices.next_release_allocation_bytes().unwrap_or(1)}if let Some(child)=self.children.get(self.children.len().saturating_sub(1)){return child.next_close_byte_demand().max(1)}if !self.children.terminal_is_empty(){return self.children.next_release_allocation_bytes().unwrap_or(1)}0}
}
pub struct SelectedChildGroupSources<'a>{owner:&'a ChildGroupPublicationOwner}
impl PagedChildGroups for SelectedChildGroupSources<'_>{fn len(&self)->usize{self.owner.indices.len()}fn group(&self,index:usize)->Option<PagedChildGroup<'_>>{let ordinal=*self.owner.indices.get(index)?;self.owner.children.get(ordinal)?.view().ok()}}
impl Drop for ChildGroupPublicationOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"publication retains its original children and actual index/outer pages");}}
