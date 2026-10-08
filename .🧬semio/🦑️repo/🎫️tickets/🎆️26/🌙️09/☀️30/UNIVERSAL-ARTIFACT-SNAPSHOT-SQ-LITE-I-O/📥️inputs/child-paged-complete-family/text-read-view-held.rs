
/// 👓️ Borrows validated original UTF8 storage without flattening its physical ownership.
#[derive(Clone,Copy)]
pub struct TextReadView<'a>{source:TextReadRepresentation<'a>}
#[derive(Clone,Copy)]
enum TextReadRepresentation<'a>{Contiguous(&'a str),Paged(&'a dyn TextReadSource)}
impl<'a> TextReadView<'a>{
    pub const fn from_str(source:&'a str)->Self{Self{source:TextReadRepresentation::Contiguous(source)}}
    pub fn len(&self)->usize{match self.source{TextReadRepresentation::Contiguous(source)=>source.len(),TextReadRepresentation::Paged(source)=>source.byte_len()}}
    pub fn is_empty(&self)->bool{self.len()==0}
    pub fn byte_at(&self,index:usize)->Option<u8>{match self.source{TextReadRepresentation::Contiguous(source)=>source.as_bytes().get(index).copied(),TextReadRepresentation::Paged(source)=>source.byte_at(index)}}
    /// 🪪️ Exposes a contiguous original only when its representation already provides it.
    pub fn contiguous(&self)->Option<&'a str>{match self.source{TextReadRepresentation::Contiguous(source)=>Some(source),TextReadRepresentation::Paged(_)=>None}}
    /// 📥️ Copies at most the supplied output window from the same immutable original storage.
    pub fn copy_bytes(&self,offset:usize,output:&mut[u8])->Result<usize,ValueError>{
        let remaining=self.len().checked_sub(offset).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"text source offset exceeds its original extent"))?;
        let count=remaining.min(output.len());for(index,byte)in output[..count].iter_mut().enumerate(){*byte=self.byte_at(offset+index).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"text source lost an original octet"))?;}Ok(count)
    }
    /// 🔤️ Advances one validated Unicode scalar, including a scalar crossing an original page boundary.
    pub fn character_at(&self,position:&mut usize)->Result<char,ValueError>{character(self,position)}
}
impl TextReadSource for TextReadView<'_>{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.byte_at(index)}}
impl<const N:usize> TextReadSource for PagedText<N>{fn byte_len(&self)->usize{self.byte_len()}fn byte_at(&self,index:usize)->Option<u8>{self.bytes.get(index).copied()}}
impl<const N:usize> PagedText<N>{
    /// 🌱️ Captures the actual validated paged owner directly; partial and retiring owners refuse access.
    pub fn read_view(&self)->Result<TextReadView<'_>,ValueError>{let _=self.borrow()?;Ok(TextReadView{source:TextReadRepresentation::Paged(self)})}
}
