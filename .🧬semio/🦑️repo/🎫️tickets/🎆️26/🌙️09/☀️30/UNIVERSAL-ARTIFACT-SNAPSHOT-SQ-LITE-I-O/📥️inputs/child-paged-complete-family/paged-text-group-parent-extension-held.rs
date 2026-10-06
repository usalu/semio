impl Text{
    fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        if self.long{return self.paged.return_one(parent,maximum_items)}
        if maximum_items==0||self.length==0{return Ok(semio_framework_value::list::PagedListReturnProgress::default())}
        self.length=0;self.complete=false;Ok(semio_framework_value::list::PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})
    }
}
impl ChildLabelOwner{
    pub fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        for row in &mut self.cells{for text in row{if !text.terminal_is_empty(){return text.return_one(parent,maximum_items)}}}Ok(semio_framework_value::list::PagedListReturnProgress::default())
    }
}
impl PagedChildOwner{
    pub fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        use semio_framework_os_kernel::os_spr::operation_bytes::OperationByteReturnStep;
        use semio_framework_value::list::PagedListReturnProgress;
        if maximum_items==0||maximum_bytes==0{return Ok(PagedListReturnProgress::default())}
        if let Some(schema)=self.pending_schema.as_mut(){if !schema.terminal_is_empty(){return schema.return_one(parent,1)}self.pending_schema.take();return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}
        let index=self.operations.len().checked_sub(1);let source=match self.partial_source.as_mut(){Some(source)=>Some(source),None=>index.and_then(|index|self.operations.get_mut(index))};
        if let Some(source)=source{return Ok(match source.return_one(parent,1,maximum_bytes)?{OperationByteReturnStep::Pending{returned_items,returned_bytes}=>PagedListReturnProgress{progressed:returned_items!=0,returned_allocation_bytes:returned_bytes},OperationByteReturnStep::Complete=>{if self.partial_source.is_some(){self.partial_source.take();}else{self.operations.pop();}PagedListReturnProgress{progressed:true,returned_allocation_bytes:0}}})}
        if !self.operations.terminal_is_empty(){return self.operations.return_empty_page(parent,1)}
        let index=self.labels.len().checked_sub(1);let label=match self.partial_label.as_mut(){Some(label)=>Some(label),None=>index.and_then(|index|self.labels.get_mut(index))};
        if let Some(label)=label{if !label.terminal_is_empty(){return label.return_one(parent,1)}if self.partial_label.is_some(){self.partial_label.take();}else{self.labels.pop();}return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}
        if !self.labels.terminal_is_empty(){return self.labels.return_empty_page(parent,1)}
        for text in &mut self.metadata{if !text.terminal_is_empty(){return text.return_one(parent,1)}}Ok(PagedListReturnProgress::default())
    }
}
