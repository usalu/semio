/// 🏠️ An authored retained owner can transfer its genuine nested allocations to the actual receiving parent.
pub trait RetainedOwnerParentReturn:ErasedSnapshotRetirement{
    fn return_owner_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>;
}
impl<C:RetainedOwnerParentReturn> RetainedSourceRegistry<C>{
    /// 🪪️ Accepted identities remain borrowed while absent originals hand their real pages to the parent.
    pub fn return_one<const P:usize>(&mut self,parent:&mut semio_framework_value::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize,maximum_bytes:usize)->Result<semio_framework_value::list::PagedListReturnProgress,ValueError>{
        use semio_framework_value::list::PagedListReturnProgress;
        if maximum_items==0||maximum_bytes==0{return Ok(PagedListReturnProgress::default())}
        if self.index<self.cells.len(){
            let cell=self.cells.get_mut(self.index).unwrap();
            if (!cell.retired&&!self.closing)||cell.owner.is_none(){self.index+=1;return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}
            let owner=cell.owner.as_mut().unwrap();if !owner.terminal_is_empty(){return owner.return_owner_one(parent,1,maximum_bytes)}
            cell.owner.take();self.owners-=1;self.index+=1;return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})
        }
        if self.owners!=0{self.index=0;return Ok(PagedListReturnProgress::default())}
        if !self.closing{return Ok(PagedListReturnProgress::default())}
        if self.cells.pop().is_some(){self.index=self.cells.len();return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:0})}
        if !self.cells.terminal_is_empty(){return self.cells.return_empty_page(parent,1)}
        Ok(PagedListReturnProgress::default())
    }
}
