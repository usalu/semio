    /// 🏡️ Transfers source backing to the actual parent while retaining separate physical disposal responsibility.
    pub fn return_one<const P:usize>(&mut self,parent:&mut crate::value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<OperationByteReturnStep,crate::value::ValueError>{
        if maximum_items==0||maximum_bytes==0{return Ok(OperationByteReturnStep::Pending{returned_items:0,returned_bytes:0});}
        if self.closed{return Ok(OperationByteReturnStep::Complete);}
        self.closing=true;
        if self.bytes.pop().is_some(){return Ok(OperationByteReturnStep::Pending{returned_items:1,returned_bytes:0});}
        if self.bytes.terminal_is_empty(){
            unsafe{ManuallyDrop::drop(&mut self.bytes)};
            self.bytes=ManuallyDrop::new(PagedList::empty());self.closed=true;
            return Ok(OperationByteReturnStep::Complete);
        }
        let progress=self.bytes.return_empty_page(parent,1)?;
        Ok(OperationByteReturnStep::Pending{returned_items:usize::from(progress.progressed),returned_bytes:progress.returned_allocation_bytes})
    }


