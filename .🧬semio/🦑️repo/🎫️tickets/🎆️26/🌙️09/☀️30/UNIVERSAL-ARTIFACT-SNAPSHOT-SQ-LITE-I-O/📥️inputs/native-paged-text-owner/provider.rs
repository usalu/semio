//! 📝️ First-party UTF8 text owns separately admitted metadata and physical payload pages.
use crate::{NativeDecodeControl,ValueError,ValueRefusalKind,ErasedSnapshotRetirement,SnapshotRetirementStep,list::{PagedList,PagedListError}};

/// 🪟️ Immutable UTF8 input exposes original octets without requiring contiguous storage.
pub trait TextReadSource{fn byte_len(&self)->usize;fn byte_at(&self,index:usize)->Option<u8>;}
impl TextReadSource for [u8]{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}}
impl TextReadSource for str{fn byte_len(&self)->usize{self.len()}fn byte_at(&self,index:usize)->Option<u8>{self.as_bytes().get(index).copied()}}
impl<T:TextReadSource+?Sized> TextReadSource for &T{fn byte_len(&self)->usize{(**self).byte_len()}fn byte_at(&self,index:usize)->Option<u8>{(**self).byte_at(index)}}
fn error(error:PagedListError)->ValueError{ValueError::new(match error.kind{crate::list::PagedListRefusalKind::OwnershipLimit=>ValueRefusalKind::OwnershipLimit,crate::list::PagedListRefusalKind::AllocationFailed=>ValueRefusalKind::AllocationFailed,crate::list::PagedListRefusalKind::InvariantViolated=>ValueRefusalKind::InvariantViolated},error.reason)}
fn character(source:&impl TextReadSource,position:&mut usize)->Result<char,ValueError>{
    let first=source.byte_at(*position).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"truncated paged UTF8"))?;
    let count=match first{0..=127=>1,194..=223=>2,224..=239=>3,240..=244=>4,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid paged UTF8"))};
    let mut bytes=[0;4];for(index,byte)in bytes[..count].iter_mut().enumerate(){*byte=source.byte_at(*position+index).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"truncated paged UTF8"))?;}
    let value=std::str::from_utf8(&bytes[..count]).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid paged UTF8"))?.chars().next().unwrap();*position+=count;Ok(value)
}
/// 📚️ Semantic UTF8 backing retains all admitted allocations on canceled or refused construction.
pub struct PagedText<const N:usize>{bytes:PagedList<u8,N>,complete:bool,closing:bool}
impl<const N:usize> PagedText<N>{
    /// 🈳️ Creates a stack-resident text owner with no physical allocation.
    pub const fn empty()->Self{Self{bytes:PagedList::empty(),complete:false,closing:false}}
    pub fn byte_len(&self)->usize{self.bytes.len()}
    pub fn allocated_bytes(&self)->usize{self.bytes.allocated_bytes()}
    /// 👓️ Exposes text only after complete UTF8 validation and before retirement starts.
    pub fn borrow(&self)->Result<TextReadSpan<'_,N>,ValueError>{if !self.complete||self.closing{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"paged text lacks a complete live UTF8 capability"));}Ok(TextReadSpan{owner:self})}
    /// 🛬️ Copies original semantic octets after each exact allocation admission, retaining partial pages on every failure.
    pub fn read_from_source(&mut self,source:&impl TextReadSource,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
        if self.bytes.allocated_bytes()!=0||self.complete||self.closing{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"paged text source construction requires an empty owner"));}
        let length=source.byte_len();if length>N{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"paged text exceeds declared logical capacity"));}
        control.scoped_stage(|control|{control.begin_stage(length)?;let mut position=0;while position<length{let previous=position;character(source,&mut position)?;control.advance(position-previous)?;}Ok::<_,ValueError>(())})?;
        control.scoped_stage(|control|{
            control.begin_stage(length)?;let mut position=0;
            while position<length{
                let value=character(source,&mut position)?;let mut octets=[0;4];
                for byte in value.encode_utf8(&mut octets).as_bytes(){
                    while !self.bytes.has_reserved_slot(){
                        let required=self.bytes.next_allocation_bytes().map_err(error)?;control.charge(required)?;
                        let step=self.bytes.reserve_one(required).map_err(|failure|error(failure.refusal()))?;
                        if !step.progressed||step.allocated_bytes!=required{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"paged text allocation differs from its exact admission"));}
                    }
                    self.bytes.push_reserved(*byte).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"paged text rejected its funded byte"))?;control.advance(1)?;
                }
            }
            Ok::<_,ValueError>(())
        })?;self.complete=true;Ok(())
    }
}
impl<const N:usize> ErasedSnapshotRetirement for PagedText<N>{
    fn close_step(&mut self,items:usize,bytes:usize)->Result<SnapshotRetirementStep,ValueError>{
        if items==0||bytes==0{return Ok(SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});}
        if self.terminal_is_empty(){return Ok(SnapshotRetirementStep::Complete);}self.closing=true;
        if self.bytes.pop().is_some(){return Ok(SnapshotRetirementStep::Pending{released_items:1,released_bytes:0});}
        let step=self.bytes.release_empty_page(bytes).map_err(error)?;
        Ok(SnapshotRetirementStep::Pending{released_items:usize::from(step.progressed),released_bytes:step.released_allocation_bytes})
    }
    fn terminal_is_empty(&self)->bool{self.bytes.terminal_is_empty()}
    fn next_close_byte_demand(&self)->usize{if self.bytes.len()!=0{1}else{self.bytes.next_release_allocation_bytes().unwrap_or(0)}}
}
impl<const N:usize> Drop for PagedText<N>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"paged semantic text retains actual physical allocations");}}
/// 🔤️ A complete text borrow streams validated bytes and Unicode scalars without flattening.
pub struct TextReadSpan<'a,const N:usize>{owner:&'a PagedText<N>}
impl<const N:usize> TextReadSpan<'_,N>{
    pub fn byte_len(&self)->usize{self.owner.byte_len()}
    pub fn byte_at(&self,index:usize)->Option<u8>{self.owner.bytes.get(index).copied()}
    pub fn bytes(&self)->impl Iterator<Item=u8>+'_ {self.owner.bytes.iter().copied()}
    pub fn chars(&self)->TextChars<'_,N>{TextChars{source:self,position:0}}
}
impl<const N:usize> TextReadSource for TextReadSpan<'_,N>{fn byte_len(&self)->usize{self.byte_len()}fn byte_at(&self,index:usize)->Option<u8>{self.byte_at(index)}}
pub struct TextChars<'a,const N:usize>{source:&'a TextReadSpan<'a,N>,position:usize}
impl<const N:usize> Iterator for TextChars<'_,N>{type Item=char;fn next(&mut self)->Option<char>{if self.position==self.source.byte_len(){None}else{Some(character(&self.source,&mut self.position).expect("validated original paged UTF8"))}}}

