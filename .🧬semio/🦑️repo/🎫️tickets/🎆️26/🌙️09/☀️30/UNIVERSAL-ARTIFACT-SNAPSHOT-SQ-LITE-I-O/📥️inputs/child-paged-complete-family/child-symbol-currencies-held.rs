impl ErasedSnapshotRetirement for ChildSymbolOwner{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"child symbol retirement requires admitted depth"))}
        if !self.entries.is_empty(){if grant.maximum_copy_bytes<size_of::<TextOrdinal>(){return Ok(RetainedCloneStep::Progress(Default::default()))}self.entries.pop();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<TextOrdinal>(),..Default::default()}))}
        let step=self.entries.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes,..Default::default()}))
    }
    fn terminal_is_empty(&self)->bool{self.entries.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.entries.is_empty(){0}else{size_of::<TextOrdinal>()})}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.entries.is_empty(){Ok(0)}else{self.entries.next_release_allocation_bytes().map_err(ValueError::from)}}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal_is_empty()))}
}
