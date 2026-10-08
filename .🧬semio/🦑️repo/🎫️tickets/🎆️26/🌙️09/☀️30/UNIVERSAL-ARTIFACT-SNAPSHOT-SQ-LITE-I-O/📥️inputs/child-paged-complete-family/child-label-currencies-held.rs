impl ChildLabelOwner{fn next_cell(&self)->Option<&Text>{self.cells.iter().flatten().find(|cell|!cell.terminal_is_empty())}}
impl ErasedSnapshotRetirement for ChildLabelOwner{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !admit_frontier(self,grant)?{return Ok(RetainedCloneStep::Progress(Default::default()))}for cell in self.cells.iter_mut().flatten(){if !cell.terminal_is_empty(){return cell.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})}}Ok(RetainedCloneStep::Complete(Default::default()))}
    fn terminal_is_empty(&self)->bool{self.next_cell().is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.next_cell().map_or(Ok(0),ErasedSnapshotRetirement::next_copy_byte_demand)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.next_cell().map_or(Ok(0),|cell|cell.next_capacity_byte_demand(copy))}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.next_cell().map_or(Ok(0),ErasedSnapshotRetirement::next_release_byte_demand)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.next_cell().map_or(Ok(0),|cell|Ok(nested_demands(cell,0)?.depth))}
}
