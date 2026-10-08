impl ErasedSnapshotRetirement for Text{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if !admit_frontier(self,grant)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !self.inline.terminal_is_empty(){let copied_bytes=self.inline.as_bytes().len();self.inline.close_one(1);self.complete=false;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}))}
        self.paged.close_step(grant)
    }
    fn terminal_is_empty(&self)->bool{self.inline.terminal_is_empty()&&self.paged.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.inline.terminal_is_empty(){Ok(self.inline.as_bytes().len())}else{self.paged.next_copy_byte_demand()}}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.paged.next_capacity_byte_demand(copy)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.inline.terminal_is_empty(){Ok(0)}else{self.paged.next_release_byte_demand()}}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
}
pub(super) fn admit_frontier(owner:&dyn ErasedSnapshotRetirement,grant:RetainedCloneGrant)->Result<bool,ValueError>{
    if grant.maximum_items==0{return Ok(false)}
    if owner.next_depth_demand()?>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"child retirement exceeds admitted depth"))}
    Ok(owner.next_copy_byte_demand()?<=grant.maximum_copy_bytes&&owner.next_capacity_byte_demand(grant.maximum_copy_bytes)?<=grant.maximum_capacity_bytes&&owner.next_release_byte_demand()?<=grant.maximum_release_bytes)
}
pub(super) fn nested_demands(owner:&dyn ErasedSnapshotRetirement,copy:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"child retirement depth overflow"))?})}
pub(super) fn remove_demand<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
pub(super) fn removed<T>()->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()})}
pub(super) fn released(step:semio_framework_value::list::PagedListProgress)->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()})}
