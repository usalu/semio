struct PagedTextRetirement<const N:usize>{owner:PagedText<N>}
impl<const N:usize> crate::retirement::RetirementCursor for PagedTextRetirement<N>{
    fn close_step(&mut self,grant:RetainedCloneGrant)->crate::retirement::RetirementStep{match self.owner.close_step(grant){Err(error)=>crate::retirement::RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress))if progress==RetainedCloneProgress::default()=>crate::retirement::RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>crate::retirement::RetirementStep::Progress(progress)}}
    fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
    fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<const N:usize> crate::retirement::RetireOwned for PagedText<N>{
    fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{Box::new(PagedTextRetirement{owner:self})}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<PagedTextRetirement<N>>())}
    fn controlled_retirement_supported()->bool{true}
}
