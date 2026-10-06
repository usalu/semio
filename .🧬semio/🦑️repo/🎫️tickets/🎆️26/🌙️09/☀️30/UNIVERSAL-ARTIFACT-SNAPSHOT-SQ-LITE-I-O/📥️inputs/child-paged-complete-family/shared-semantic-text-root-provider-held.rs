//! 📝️ Controlled aliases retain one actual semantic UTF8 owner until an explicitly admitted final recipient accepts it.
use std::{alloc::Layout,cell::UnsafeCell,mem::ManuallyDrop,ptr::NonNull,sync::atomic::{AtomicUsize,Ordering}};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind,paged_text::{PagedText,TextReadSpan}};
struct Root<const N:usize>{references:AtomicUsize,payload:UnsafeCell<ManuallyDrop<PagedText<N>>>}
/// 🪪️ The handle contains scalar authority; it never contains a copied contiguous or paged text payload.
pub struct SharedSemanticText<const N:usize>{root:Option<NonNull<Root<N>>>}
/// 📥️ The caller reserves an intrinsic unique payload slot before any reference can retire.
pub struct SemanticTextFinalRecipient<const N:usize>{payload:Option<PagedText<N>>}
/// ♻️ A nonlast alias performs logical work; only the unique last alias disposes the declared control allocation.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct SemanticTextAliasRelease{pub released_items:usize,pub released_bytes:usize,pub final_payload_transferred:bool}
unsafe impl<const N:usize> Send for SharedSemanticText<N>{}
unsafe impl<const N:usize> Sync for SharedSemanticText<N>{}
fn invalid(detail:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,detail)}
impl<const N:usize> SemanticTextFinalRecipient<N>{
    pub const fn empty()->Self{Self{payload:None}}
    pub fn has_payload(&self)->bool{self.payload.is_some()}
    pub fn take_payload(&mut self)->Option<PagedText<N>>{self.payload.take()}
}
impl<const N:usize> Drop for SemanticTextFinalRecipient<N>{fn drop(&mut self){assert!(std::thread::panicking()||self.payload.is_none(),"semantic final recipient retains the real paged owner for caller retirement");}}
impl<const N:usize> SharedSemanticText<N>{
    pub fn root_layout()->Layout{Layout::new::<Root<N>>()}
    /// 🏠️ Exact root admission and allocation precede consuming the caller's complete original text.
    pub fn take_original(incoming:&mut Option<PagedText<N>>,control:&mut NativeEncodeControl<'_>)->Result<Self,ValueError>{
        incoming.as_ref().ok_or_else(||invalid("semantic root requires the original caller owner"))?.borrow()?;
        let layout=Self::root_layout();if layout.size()>4096{return Err(invalid("semantic root exceeds physical page authority"))}control.checkpoint()?;control.charge(layout.size())?;
        let root=NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<Root<N>>()).ok_or_else(||ValueError::new(ValueRefusalKind::AllocationFailed,"semantic root allocation refused"))?;
        unsafe{root.as_ptr().write(Root{references:AtomicUsize::new(1),payload:UnsafeCell::new(ManuallyDrop::new(incoming.take().unwrap()))})};Ok(Self{root:Some(root)})
    }
    /// 👓️ Every live handle borrows the same completed immutable original, including its real page pointers.
    pub fn borrow(&self)->Result<TextReadSpan<'_,N>,ValueError>{let root=self.root.ok_or_else(||invalid("semantic alias has retired"))?;unsafe{(&*root.as_ref().payload.get()).borrow()}}
    /// 🧮️ Controlled scalar retention has no payload allocation and refuses cancellation or overflow before publishing an alias.
    pub fn retain(&self,control:&mut NativeEncodeControl<'_>)->Result<Self,ValueError>{
        let pointer=self.root.ok_or_else(||invalid("retired semantic alias cannot retain"))?;let root=unsafe{pointer.as_ref()};let mut count=root.references.load(Ordering::Relaxed);
        loop{control.checkpoint()?;control.step()?;let next=count.checked_add(1).filter(|next|*next<=isize::MAX as usize).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"semantic alias count exceeds native authority"))?;if count==0{return Err(invalid("semantic root lacks live authority"))}match root.references.compare_exchange_weak(count,next,Ordering::Relaxed,Ordering::Relaxed){Ok(_)=>return Ok(Self{root:Some(pointer)}),Err(current)=>count=current}}
    }
    /// 📤️ An empty caller slot is mandatory on every branch; no last-reference prediction precedes atomic transfer.
    pub fn retire_into(&mut self,recipient:&mut SemanticTextFinalRecipient<N>,maximum_items:usize,maximum_bytes:usize)->Result<SemanticTextAliasRelease,ValueError>{
        if recipient.payload.is_some(){return Err(invalid("semantic final recipient is occupied"))}if maximum_items==0||maximum_bytes<Self::root_layout().size(){return Ok(SemanticTextAliasRelease{released_items:0,released_bytes:0,final_payload_transferred:false})}
        let Some(pointer)=self.root.take()else{return Ok(SemanticTextAliasRelease{released_items:0,released_bytes:0,final_payload_transferred:false})};let root=unsafe{pointer.as_ref()};let previous=root.references.fetch_sub(1,Ordering::AcqRel);assert!(previous>0,"semantic reference authority underflow");
        if previous!=1{return Ok(SemanticTextAliasRelease{released_items:1,released_bytes:0,final_payload_transferred:false})}
        recipient.payload=Some(unsafe{ManuallyDrop::into_inner(std::ptr::read(root.payload.get()))});unsafe{std::alloc::dealloc(pointer.as_ptr().cast(),Self::root_layout())};Ok(SemanticTextAliasRelease{released_items:1,released_bytes:Self::root_layout().size(),final_payload_transferred:true})
    }
    pub fn terminal_is_empty(&self)->bool{self.root.is_none()}
}
impl<const N:usize> Drop for SharedSemanticText<N>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"semantic alias requires consuming explicit retirement");}}
