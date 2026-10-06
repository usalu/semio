impl PagedChildOwner{
    /// 🏠️ Returns only an already replaced schema; accepted sources and labels remain live.
    pub fn return_committed_scaffold_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        use semio_framework_value::list::PagedListReturnProgress;
        if maximum_items==0||!self.pending_schema_is_retired{return Ok(PagedListReturnProgress::default())}
        let schema=self.pending_schema.as_mut().ok_or_else(||invalid("committed schema scaffold lacks its actual owner"))?;
        if !schema.terminal_is_empty(){return schema.return_one(parent,1)}
        self.pending_schema.take();self.pending_schema_is_retired=false;Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})
    }
}
