impl ChildGroupPublicationOwner{
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.indices.is_empty(){return Ok(remove_demand::<usize>())}if !self.indices.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:self.indices.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()})}
        if let Some(child)=self.children.len().checked_sub(1).and_then(|index|self.children.get(index)){return if child.terminal_is_empty(){Ok(remove_demand::<PagedChildOwner>())}else{nested_demands(child,copy)}}
        if !self.children.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:self.children.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()})}Ok(Default::default())
    }
}
impl ErasedSnapshotRetirement for ChildGroupPublicationOwner{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !admit_frontier(self,grant)?{return Ok(RetainedCloneStep::Progress(Default::default()))}self.closing=true;
        if !self.indices.is_empty(){self.indices.pop();return Ok(removed::<usize>())}if !self.indices.terminal_is_empty(){return Ok(released(self.indices.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?))}
        if let Some(index)=self.children.len().checked_sub(1){let child=self.children.get_mut(index).unwrap();if !child.terminal_is_empty(){return child.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})}self.children.pop();return Ok(removed::<PagedChildOwner>())}
        Ok(released(self.children.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?))
    }
    fn terminal_is_empty(&self)->bool{self.indices.terminal_is_empty()&&self.children.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
