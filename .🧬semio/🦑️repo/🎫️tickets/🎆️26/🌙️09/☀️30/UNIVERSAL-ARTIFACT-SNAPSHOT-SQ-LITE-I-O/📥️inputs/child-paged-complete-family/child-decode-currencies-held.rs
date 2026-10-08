impl PagedChildDecodeOwner{
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.symbols.terminal_is_empty(){return nested_demands(&self.symbols,copy)}
        if let Some(child)=self.child.as_ref().or_else(||self.groups.len().checked_sub(1).and_then(|index|self.groups.get(index))){return if child.terminal_is_empty(){Ok(remove_demand::<PagedChildOwner>())}else{nested_demands(child,copy)}}
        if !self.groups.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:self.groups.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()})}Ok(Default::default())
    }
}
impl ErasedSnapshotRetirement for PagedChildDecodeOwner{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !admit_frontier(self,grant)?{return Ok(RetainedCloneStep::Progress(Default::default()))}let nested=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if !self.symbols.terminal_is_empty(){return self.symbols.close_step(nested)}let index=self.groups.len().checked_sub(1);let child=match self.child.as_mut(){Some(child)=>Some(child),None=>index.and_then(|index|self.groups.get_mut(index))};
        if let Some(child)=child{if !child.terminal_is_empty(){return child.close_step(nested)}if self.child.is_some(){self.child.take();}else{self.groups.pop();}return Ok(removed::<PagedChildOwner>())}
        Ok(released(self.groups.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?))
    }
    fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.groups.terminal_is_empty()&&self.symbols.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
