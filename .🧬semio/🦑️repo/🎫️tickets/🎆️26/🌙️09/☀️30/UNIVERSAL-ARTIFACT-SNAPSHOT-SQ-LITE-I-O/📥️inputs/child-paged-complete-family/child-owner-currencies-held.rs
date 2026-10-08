impl PagedChildOwner{
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(schema)=self.pending_schema.as_ref(){return if schema.terminal_is_empty(){Ok(remove_demand::<Text>())}else{nested_demands(schema,copy)}}
        if let Some(source)=self.partial_source.as_ref().or_else(||self.operations.len().checked_sub(1).and_then(|index|self.operations.get(index))){return if source.terminal_is_empty(){Ok(remove_demand::<OwnedOperationBytes>())}else{nested_demands(source,copy)}}
        if !self.operations.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:self.operations.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()})}
        if let Some(label)=self.partial_label.as_ref().or_else(||self.labels.len().checked_sub(1).and_then(|index|self.labels.get(index))){return if label.terminal_is_empty(){Ok(remove_demand::<ChildLabelOwner>())}else{nested_demands(label,copy)}}
        if !self.labels.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:self.labels.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()})}
        self.metadata.iter().find(|text|!text.terminal_is_empty()).map_or(Ok(Default::default()),|text|nested_demands(text,copy))
    }
}
impl ErasedSnapshotRetirement for PagedChildOwner{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !admit_frontier(self,grant)?{return Ok(RetainedCloneStep::Progress(Default::default()))}
        let nested=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
        if let Some(schema)=self.pending_schema.as_mut(){if !schema.terminal_is_empty(){return schema.close_step(nested)}self.pending_schema.take();self.pending_schema_is_retired=false;return Ok(removed::<Text>())}
        let index=self.operations.len().checked_sub(1);let source=match self.partial_source.as_mut(){Some(source)=>Some(source),None=>index.and_then(|index|self.operations.get_mut(index))};
        if let Some(source)=source{if !source.terminal_is_empty(){return source.close_step(nested)}if self.partial_source.is_some(){self.partial_source.take();}else{self.operations.pop();}return Ok(removed::<OwnedOperationBytes>())}
        if !self.operations.terminal_is_empty(){return Ok(released(self.operations.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?))}
        let index=self.labels.len().checked_sub(1);let label=match self.partial_label.as_mut(){Some(label)=>Some(label),None=>index.and_then(|index|self.labels.get_mut(index))};
        if let Some(label)=label{if !label.terminal_is_empty(){return label.close_step(nested)}if self.partial_label.is_some(){self.partial_label.take();}else{self.labels.pop();}return Ok(removed::<ChildLabelOwner>())}
        if !self.labels.terminal_is_empty(){return Ok(released(self.labels.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?))}
        for text in &mut self.metadata{if !text.terminal_is_empty(){return text.close_step(nested)}}Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn terminal_is_empty(&self)->bool{self.metadata.iter().all(ErasedSnapshotRetirement::terminal_is_empty)&&self.operations.terminal_is_empty()&&self.labels.terminal_is_empty()&&self.partial_source.is_none()&&self.partial_label.is_none()&&self.pending_schema.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
