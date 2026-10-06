
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OperationByteReturnStep{Pending{returned_items:usize,returned_bytes:usize},Complete}

/// 🧭️ Reads original operation sources without materializing another payload owner.
pub trait OperationSourceCollection:Send+Sync{
    fn len(&self)->usize;
    fn source_at(&self,index:usize)->Option<crate::codec::ByteSpan<'_>>;
    fn is_empty(&self)->bool{self.len()==0}
}
impl<const N:usize> OperationSourceCollection for crate::value::list::PagedList<OwnedOperationBytes,N>{
    fn len(&self)->usize{crate::value::list::PagedList::len(self)}
    fn source_at(&self,index:usize)->Option<crate::codec::ByteSpan<'_>>{self.get(index).map(crate::codec::ByteSpan::from_source)}
}

