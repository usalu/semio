//! 🧬️ Original retained-clone traits and standard concrete cursor/field owners.
pub use semio_framework_value_control_contract::*;
pub use semio_framework_value_clone_source::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneBorrowAuthority,RetainedCloneSource,RetainedCloneSourceTake,RetainedOwnedProjection};
use crate::{ErasedSnapshotRetirement,retirement::RetireOwned};
use std::mem::{size_of,ManuallyDrop};
#[path="../🌱️dynamic/🦀️.rs"]
pub mod dynamic;
#[path="../📋️paged-list/🦀️.rs"]
pub mod paged_list;
#[path="../🧺️std-collections/🦀️.rs"]
mod std_collections;
pub use std_collections::{RetainedCloneEntries,RetainedCloneEntriesCursor};
#[path="../🔗️shared/🦀️.rs"]
pub mod shared;
#[path="../📋️field/🦀️.rs"]
mod field;
pub use field::RetainedFieldCursor;
#[path="../🔮️demands/🦀️.rs"]
mod normal_demands;
pub use normal_demands::RetainedCloneDemandCursor;
#[path="../🧳️owner/🦀️.rs"]
pub mod owner;
pub use owner::RetainedCloneOwner;
pub trait RetainedClone: RetireOwned + Send + Sync + Sized + 'static {
    type Cursor: RetainedCloneCursor<Self>;
    fn retained_clone_cursor() -> Self::Cursor;
    /// 🧬️ True for plain scalar values whose arrays clone in one funded bitwise copy turn.
    const BITWISE: bool = false;
    /// 📋️ Copies a whole array bitwise when [`Self::BITWISE`] is set; every other element clones one by one.
    fn bitwise_array<const N: usize>(_source: &[Self; N]) -> Option<[Self; N]> { None }
    /// 🚚️ Appends a contiguous run bitwise into already reserved vector capacity when [`Self::BITWISE`] is set; returns false for every element that must clone one by one.
    fn bitwise_extend(_target: &mut Vec<Self>, _source: &[Self]) -> bool { false }
}

/// 🚚️ The largest payload one bulk vector copy turn moves, equal to the ABI page so a bitwise run is never longer than one page of work.
pub const RETAINED_CLONE_BULK_PAGE_BYTES: usize = 64 * 1024;

/// 📏️ The number of `width`-byte elements one bulk turn may process under `grant`: one item and at most one page of payload, zero when the grant admits neither one item nor one element.
pub fn bulk_run_elements(grant: RetainedCloneGrant, width: usize) -> usize {
    if grant.maximum_items == 0 { return 0; }
    if width == 0 { return usize::MAX; }
    grant.maximum_copy_bytes.min(RETAINED_CLONE_BULK_PAGE_BYTES) / width
}

/// 🧾️ The honest receipt of one bulk turn that processed `elements` elements of `width` bytes: one item and exactly those payload bytes.
pub fn bulk_run_progress(elements: usize, width: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: elements.saturating_mul(width), ..Default::default() }
}

/// 🧷️ Transfers one native owner once; after `take`, callers must close the spent cursor before dropping it.
/// 🎟️ Borrows the actual selected child closure quote without advancing its typed owner.
pub fn clone_cursor_close_demand<T:RetainedClone>(cursor:&impl RetainedCloneCursor<T>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if cursor.terminal_is_empty(){return Ok(crate::RetirementDemand{depth:1,..Default::default()})}Ok(crate::RetirementDemand{copy_bytes:cursor.next_close_copy_byte_demand()?,capacity_bytes:cursor.next_close_capacity_byte_demand(body)?,release_bytes:cursor.next_close_release_byte_demand()?,depth:cursor.next_close_depth_demand()?})}

pub trait RetainedCloneCursor<T: RetainedClone>: Send {
    fn output_ready(&self)->bool{false}
    /// 🎟️ Quotes the actual next immutable-source turn before any work or allocation.
    fn advance_demands(&self, _source: RetainedCloneRef<'_, T>, _maximum_body_bytes: usize) -> Result<crate::RetirementDemand, crate::ValueError> {
        Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor lacks an original-source advance quote"))
    }
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError>;
    fn take(&mut self) -> Option<T>;
    fn begin_close(&mut self) -> bool;
    
    fn terminal_is_empty(&self) -> bool;
    /// 🪜️ Observes the depth of the next controlled close transition without changing ownership.
    fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError>;
    /// 🧮️ Observes the next payload-work grant without allocating or releasing retained owners.
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close work demand"))
    }
    /// 📐️ Observes the next granted close birth; unsupported cursors retain their owners.
    fn next_close_capacity_byte_demand(&self, _maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close birth demand"))
    }
    /// 📐️ Observes the next body or scaffold release independently of payload copying.
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no controlled close release demand"))
    }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        Err(crate::ValueError::new(crate::ValueRefusalKind::UnsupportedOwner, "clone cursor has no capacity-bearing close authority"))
    }
}

#[derive(Default)]
pub struct RetainedCloneClose {retirement:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>}
impl RetainedCloneClose {
    /// 🪑️ Seats one already admitted original retirement so custody laws can quote and fund its close without a typed owner.
    #[doc(hidden)] pub fn from_admitted_original(retirement:Box<dyn ErasedSnapshotRetirement>)->Self{Self{retirement:ManuallyDrop::new(Some(retirement))}}
    pub fn next_owner_depth_with_binding(&self,present:bool,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if !self.is_empty(){self.next_depth_demand()}else if present{Ok(1)}else{RetainedCloneBinding::depth_demand(binding)}}
    pub fn next_copy_with_binding(&self,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty(){RetainedCloneBinding::copy_demand(binding)}else{self.next_copy_byte_demand()}}
    pub fn next_release_with_binding(&self,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty(){RetainedCloneBinding::release_demand(binding)}else{self.next_release_byte_demand()}}
    pub fn next_owner_capacity_with_binding<T:RetireOwned>(&self,present:bool,body:usize,binding:&Option<RetainedCloneBinding>)->Result<usize,crate::ValueError>{if self.is_empty()&&!present{RetainedCloneBinding::capacity_demand(binding,body)}else{self.next_owner_capacity_byte_demand::<T>(present,body)}}
    pub fn next_copy_byte_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.copy_bytes))}
    pub fn next_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,body)?.capacity_bytes))}
    pub fn next_release_byte_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.release_bytes))}
    pub fn next_depth_demand(&self)->Result<usize,crate::ValueError> {self.retirement.as_ref().map_or(Ok(0),|owner|Ok(crate::factory_ticket_demands(owner,0)?.depth))}
    pub fn next_owner_capacity_byte_demand<T:RetireOwned>(&self,present:bool,body:usize)->Result<usize,crate::ValueError> {
        if !self.is_empty(){return self.next_capacity_byte_demand(body);}
        if present&&!T::controlled_retirement_supported(){return Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"typed owner has no controlled close birth authority"));}
        Ok(if present{crate::owned_retirement_birth_bytes::<T>()}else{0})
    }
    pub fn begin_granted<T:RetireOwned>(&mut self,value:&mut Option<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,crate::ValueError> {
        if !self.is_empty()||value.is_none(){return Ok(None);}
        let bytes=crate::owned_retirement_birth_bytes::<T>();
        if grant.maximum_items==0||bytes>grant.maximum_capacity_bytes{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        match crate::admit_owned_retirement(value.take().unwrap(),grant) {
            Ok((owner,progress))=>{*self.retirement=Some(owner);Ok(Some(RetainedCloneStep::Progress(progress)))},
            Err((error,owner))=>{*value=Some(owner);Err(error)},
        }
    }
    pub fn begin_default_granted<T:RetireOwned+Default>(&mut self,value:&mut T,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,crate::ValueError> {
        if !self.is_empty(){return Ok(None);}
        let bytes=crate::owned_retirement_birth_bytes::<T>();
        if grant.maximum_items==0||bytes>grant.maximum_capacity_bytes{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let mut owner=Some(std::mem::take(value));
        let step=self.begin_granted(&mut owner,grant);
        if let Some(owner)=owner{*value=owner;}
        step
    }
    pub fn step_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError> {
        let step=crate::close_factory_ticket(&mut self.retirement,grant)?;
        if !step.progress().fits(grant){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"clone retirement exceeded admitted full grant"));}
        Ok(step)
    }
    pub fn is_empty(&self)->bool {self.retirement.is_none()}
}

impl Drop for RetainedCloneClose {fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"clone retirement abandoned original ownership");if self.is_empty(){unsafe{ManuallyDrop::drop(&mut self.retirement);}}}}

pub fn close_retained_binding(binding: &mut Option<RetainedCloneBinding>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
    RetainedCloneBinding::close_one(binding, grant)
}

pub struct ScalarCursor<T> {
    value: Option<T>,
    source: Option<RetainedCloneBinding>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T> Default for ScalarCursor<T> {
    fn default() -> Self {
        Self { value: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T> RetainedCloneCursor<T> for ScalarCursor<T>
where
    T: RetainedClone + Copy,
{
    fn output_ready(&self)->bool{self.value.is_some()}
    fn advance_demands(&self, source:RetainedCloneRef<'_,T>, _body:usize)->Result<crate::RetirementDemand,crate::ValueError>{
        if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"scalar clone cannot quote a closing or spent owner"));}
        Ok(if self.source.is_none(){crate::RetirementDemand{copy_bytes:source.binding_copy_bytes().max(size_of::<T>()),depth:1,..Default::default()}}else if self.value.is_none(){crate::RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}else{Default::default()})
    }
    fn advance(&mut self, source: RetainedCloneRef<'_, T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained scalar clone cursor is spent"));
        }
        if self.value.is_none()&&!(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()}).fits(grant){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.value.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<T>(), retained_capacity_bytes: 0, released_bytes: 0 };
        if !progress.fits(grant) {
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        }
        self.value = Some(*source.get());
        Ok(RetainedCloneStep::Complete(progress))
    }

    fn take(&mut self) -> Option<T> {
        let value = self.value.take();
        if value.is_some() {
            self.spent = true;
        }
        value
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.value.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<T>(self.value.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_with_binding(&self.source)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.value.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

macro_rules! retained_clone_scalar {
    ($($type:ty),+ $(,)?) => {$ (
        impl RetainedClone for $type {
            type Cursor = ScalarCursor<Self>;
            fn retained_clone_cursor() -> Self::Cursor { ScalarCursor::default() }
            const BITWISE: bool = true;
            fn bitwise_array<const N: usize>(source: &[Self; N]) -> Option<[Self; N]> { Some(*source) }
            fn bitwise_extend(target: &mut Vec<Self>, source: &[Self]) -> bool { target.extend_from_slice(source); true }
        }
    )+ };
}

retained_clone_scalar!((), bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

/// 🧮️ Clones a fixed array: scalar elements in one bitwise turn, every other element through its own bounded cursor.
pub struct ArrayCursor<T: RetainedClone, const N: usize> {
    slots: [Option<T>; N],
    child: Option<T::Cursor>,
    child_value: Option<T>,
    index: usize,
    source: Option<RetainedCloneBinding>,
    output: Option<[T; N]>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T: RetainedClone, const N: usize> Default for ArrayCursor<T, N> {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), child: None, child_value: None, index: 0, source: None, output: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone, const N: usize> RetainedCloneCursor<[T; N]> for ArrayCursor<T, N> {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,[T;N]>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"array original clone is closing or spent"))}if self.output.is_some(){return Ok(Default::default())}if T::BITWISE||self.index==N{return Ok(crate::RetirementDemand{copy_bytes:size_of::<[T;N]>(),depth:1,..Default::default()})}if self.child_value.is_some(){return clone_cursor_close_demand::<T>(self.child.as_ref().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"array original child absent"))?,body)}let projected=source.project(self.index+1,|array|&array[self.index]);match self.child.as_ref(){Some(child)=>child.advance_demands(projected,body),None=>T::retained_clone_cursor().advance_demands(projected,body)}}

    fn advance(&mut self, source: RetainedCloneRef<'_, [T; N]>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array clone cursor is closing")); }
        if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array clone cursor is spent")); }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.output.is_some() { return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())); }
        if let Some(progress) = source.bind(&mut self.source, grant)? { return Ok(RetainedCloneStep::Progress(progress)); }
        if T::BITWISE {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<[T; N]>(), ..Default::default() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.output = Some(T::bitwise_array(source.get()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "bitwise array element lacks its bitwise copy"))?);
            return Ok(RetainedCloneStep::Complete(progress));
        }
        if self.child_value.is_some() {
            let child = self.child.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array child cursor is missing"))?;
            if !child.terminal_is_empty() {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let step = child.close_step(grant)?;
                return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained array child scaffold close")?.progress()));
            }
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.child = None;
            self.slots[self.index] = self.child_value.take();
            self.index += 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if self.index == N {
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<[T; N]>(), ..Default::default() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            let mut taken = std::array::from_fn::<Option<T>, N, _>(|_| None);
            for (slot, target) in self.slots.iter_mut().zip(taken.iter_mut()) { *target = slot.take(); }
            if taken.iter().any(Option::is_none) {
                for (slot, target) in self.slots.iter_mut().zip(taken.iter_mut()) { if slot.is_none() { *slot = target.take(); } }
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array lost an element owner"));
            }
            self.output = Some(taken.map(|slot| slot.expect("checked retained array element")));
            return Ok(RetainedCloneStep::Complete(progress));
        }
        let index = self.index;
        let child = self.child.get_or_insert_with(T::retained_clone_cursor);
        match child.advance(source.project(index + 1, |array| &array[index]), grant)? {
            RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained array child")?)),
            RetainedCloneStep::Complete(progress) => {
                let progress = admit_retained_clone_progress(grant, progress, "retained array child")?;
                self.child_value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained array child completed without an owner"))?);
                let _ = child.begin_close();
                Ok(RetainedCloneStep::Progress(progress))
            }
        }
    }

    fn take(&mut self) -> Option<[T; N]> {
        let output = self.output.take();
        if output.is_some() { self.spent = true; }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing { return false; }
        self.closing = true;
        true
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "array clone cursor must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained array child close")?.progress())); }
            self.child = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        for slot in self.slots.iter_mut() { if let Some(step) = self.close.begin_granted(slot, grant)? { return Ok(step); } }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return child.next_close_depth_demand(); }
        self.close.next_owner_depth_with_binding(self.child_value.is_some() || self.output.is_some() || self.slots.iter().any(Option::is_some), &self.source)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() || self.slots.iter().any(Option::is_some) { return self.close.next_owner_capacity_with_binding::<T>(true, maximum_release_bytes, &self.source); }
        self.close.next_owner_capacity_with_binding::<[T; N]>(self.output.is_some(), maximum_release_bytes, &self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.slots.iter().all(Option::is_none) && self.close.is_empty() && self.source.is_none()
    }
}

impl<T: RetainedClone, const N: usize> RetainedClone for [T; N] {
    type Cursor = ArrayCursor<T, N>;
    fn retained_clone_cursor() -> Self::Cursor { ArrayCursor::default() }
    const BITWISE: bool = T::BITWISE;
    fn bitwise_array<const M: usize>(source: &[Self; M]) -> Option<[Self; M]> {
        if !T::BITWISE { return None; }
        Some(std::array::from_fn(|index| T::bitwise_array(&source[index]).expect("bitwise array rows copy bitwise")))
    }
    fn bitwise_extend(target: &mut Vec<Self>, source: &[Self]) -> bool {
        if !T::BITWISE { return false; }
        target.extend(source.iter().map(|row| T::bitwise_array(row).expect("bitwise array rows copy bitwise")));
        true
    }
}

#[derive(Default)]
pub struct StringCursor {
    output: Option<String>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
}

impl RetainedCloneCursor<String> for StringCursor {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,String>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.phase==3{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"string original clone is closing or spent"))}let original=source.get();let(copy,capacity)=match self.phase{0=>(0,original.len()),1=>{let start=self.output.as_ref().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"string original output absent"))?.len();if start>original.len()||!original.is_char_boundary(start){return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"string original offset changed"))}let mut end=start+(original.len()-start).min(body.min(RETAINED_CLONE_BULK_PAGE_BYTES));while end>start&&!original.is_char_boundary(end){end-=1;}if end==start&&start<original.len(){end=start+original[start..].chars().next().unwrap().len_utf8();}(end-start,0)},_=>(0,0)};Ok(crate::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()})}

    fn advance(&mut self, source: RetainedCloneRef<'_, String>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone cursor is closing"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        let source = source.get();
        match self.phase {
            0 => {
                let planned = source.len();
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let mut output = String::new();
                output.try_reserve_exact(planned).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained string clone capacity allocation failed"))?;
                let actual = output.capacity();
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone allocator exceeded its admitted capacity"));
                }
                self.output = Some(output);
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            1 => {
                let output = self.output.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone output owner is missing"))?;
                let start = output.len();
                if !source.is_char_boundary(start) {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone source content changed across a UTF-8 boundary"));
                }
                let remaining = source.len().checked_sub(start).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone source changed"))?;
                if remaining == 0 {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.phase = 2;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                let maximum_end = start + remaining.min(grant.maximum_copy_bytes);
                let mut end = maximum_end;
                while end > start && !source.is_char_boundary(end) {
                    end -= 1;
                }
                if end == start {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                output.push_str(&source[start..end]);
                let progress = RetainedCloneProgress { copied_items: 0, copied_bytes: end - start, retained_capacity_bytes: 0, released_bytes: 0 };
                Ok(RetainedCloneStep::Progress(progress))
            }
            2 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            3 => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone cursor is spent")),
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained string clone state is invalid")),
        }
    }

    fn take(&mut self) -> Option<String> {
        if self.phase != 2 {
            return None;
        }
        self.phase = 3;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "clone cursor must begin close before granted retirement")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} self.close.next_owner_depth_with_binding(self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_owner_capacity_with_binding::<String>(self.output.is_some(), maximum_release_bytes,&self.source)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        self.close.next_release_with_binding(&self.source)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl RetainedClone for String {
    type Cursor = StringCursor;
    fn retained_clone_cursor() -> Self::Cursor {
        StringCursor::default()
    }
}

pub struct VecCursor<T: RetainedClone> {
    values: Vec<T>,
    child: Option<T::Cursor>,
    child_value: Option<T>,
    index: usize,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    output: Option<Vec<T>>,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for VecCursor<T> {
    fn default() -> Self {
        Self { values: Vec::new(), child: None, child_value: None, index: 0, source: None, phase: 0, output: None, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> VecCursor<T> {
    /// 🚚️ Copies one bounded run of a bitwise vector per turn into the capacity reserved up front, so the clone costs one item and at most one page of payload per turn instead of a cursor lifecycle per element.
    fn advance_bitwise(&mut self, source: &[T], grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.index != self.values.len() || self.index > source.len() {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained bitwise vector clone progress disagrees with its source"));
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let width = size_of::<T>();
        let remaining = source.len() - self.index;
        let count = remaining.min(bulk_run_elements(grant, width));
        if remaining > 0 && count == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let end = self.index + count;
        if !T::bitwise_extend(&mut self.values, &source[self.index..end]) {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "bitwise vector element lacks its bitwise copy"));
        }
        self.index = end;
        let progress = bulk_run_progress(count, width);
        if end < source.len() { return Ok(RetainedCloneStep::Progress(progress)); }
        self.output = Some(std::mem::take(&mut self.values));
        self.phase = 2;
        Ok(RetainedCloneStep::Complete(progress))
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Vec<T>> for VecCursor<T> {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Vec<T>>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.phase==3{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"vector original clone is closing or spent"))}let original=source.get();if self.phase==0{return Ok(crate::RetirementDemand{capacity_bytes:original.len().checked_mul(size_of::<T>()).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"vector original reserve overflow"))?,depth:1,..Default::default()})}if self.phase==2||self.index==original.len(){return Ok(crate::RetirementDemand{depth:1,..Default::default()})}if T::BITWISE{let width=size_of::<T>();let count=(original.len()-self.index).min(if width==0{usize::MAX}else{body.min(RETAINED_CLONE_BULK_PAGE_BYTES)/width}).max(1);return Ok(crate::RetirementDemand{copy_bytes:count.saturating_mul(width),depth:1,..Default::default()})}if self.child_value.is_some(){return clone_cursor_close_demand::<T>(self.child.as_ref().ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"vector original child absent"))?,body)}let projected=source.project(self.index+1,|values|&values[self.index]);match self.child.as_ref(){Some(child)=>child.advance_demands(projected,body),None=>T::retained_clone_cursor().advance_demands(projected,body)}}

    fn advance(&mut self, source: RetainedCloneRef<'_, Vec<T>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone cursor is closing"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        let source_value = source.get();
        if self.phase == 0 {
            let planned = source_value.len().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone capacity overflow"))?;
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            self.values.try_reserve_exact(source_value.len()).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained vector clone capacity allocation failed"))?;
            let actual = self.values.capacity().checked_mul(size_of::<T>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained vector clone actual capacity overflow"))?;
            if actual > grant.maximum_capacity_bytes {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone allocator exceeded its admitted capacity"));
            }
            self.phase = 1;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }));
        }
        if self.phase == 2 {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if self.phase == 3 {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector clone cursor is spent"));
        }
        if T::BITWISE { return self.advance_bitwise(source_value, grant); }
        let mut used = RetainedCloneProgress::default();
        loop {
            let remaining = RetainedCloneGrant {
                maximum_items: grant.maximum_items.saturating_sub(used.copied_items),
                maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(used.copied_bytes),
                maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(used.retained_capacity_bytes),
                maximum_depth: grant.maximum_depth, maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(used.released_bytes) };
            if let Some(child) = self.child.as_mut() {
                if self.child_value.is_some() {
                    if !child.terminal_is_empty() {
                        if remaining.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(used));
                        }
                        let step = child.close_step(remaining)?;
                        let progress = admit_retained_clone_close(remaining, step, child.terminal_is_empty(), "retained vector child scaffold close")?.progress();
                        if progress == RetainedCloneProgress::default() { return Ok(RetainedCloneStep::Progress(used)); }
                        used = used.checked_add(progress)?;
                        continue;
                    }
                    if remaining.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    self.child = None;
                    self.values.push(self.child_value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child owner is missing"))?);
                    self.index += 1;
                    used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    if used.copied_items == grant.maximum_items {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    continue;
                }
            }
            if self.index == source_value.len() {
                if remaining.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(used));
                }
                self.output = Some(std::mem::take(&mut self.values));
                self.phase = 2;
                used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                return Ok(RetainedCloneStep::Complete(used));
            }
            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 && remaining.maximum_release_bytes == 0 {
                return Ok(RetainedCloneStep::Progress(used));
            }
            let child = self.child.get_or_insert_with(T::retained_clone_cursor);
            match child.advance(source.project(self.index + 1, |values| &values[self.index]), remaining)? {
                RetainedCloneStep::Progress(progress) => {
                    let progress = admit_retained_clone_progress(remaining, progress, "retained vector child")?;
                    if progress == RetainedCloneProgress::default() {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    used = used.checked_add(progress)?;
                }
                RetainedCloneStep::Complete(progress) => {
                    let progress = admit_retained_clone_progress(remaining, progress, "retained vector child")?;
                    used = used.checked_add(progress)?;
                    if used.copied_items == grant.maximum_items {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    self.child_value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained vector child completed without an owner"))?);
                    let _ = child.begin_close();
                    used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                    continue;
                }
            }
        }
    }

    fn take(&mut self) -> Option<Vec<T>> {
        if self.phase != 2 {
            return None;
        }
        self.phase = 3;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            let progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
            self.child = None;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        if !self.values.is_empty() || (size_of::<T>() != 0 && self.values.capacity() > 0) { if let Some(step) = self.close.begin_default_granted(&mut self.values, grant)? { return Ok(step); } }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if let Some(child)=self.child.as_ref(){return child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()>0),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Vec<T>>(self.output.is_some()||!self.values.is_empty()||(size_of::<T>()!=0&&self.values.capacity()!=0),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.values.is_empty() && (size_of::<T>() == 0 || self.values.capacity() == 0) && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Vec<T> {
    type Cursor = VecCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        VecCursor::default()
    }
}

pub struct OptionCursor<T: RetainedClone> {
    child: T::Cursor,
    child_value: Option<T>,
    output: Option<Option<T>>,
    source: Option<RetainedCloneBinding>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for OptionCursor<T> {
    fn default() -> Self {
        Self { child: T::retained_clone_cursor(), child_value: None, output: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Option<T>> for OptionCursor<T> {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Option<T>>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"optional original clone is closing or spent"))}if self.output.is_some()||source.get().is_none(){return Ok(crate::RetirementDemand{depth:1,..Default::default()})}if self.child_value.is_some(){let mut demand=clone_cursor_close_demand::<T>(&self.child,body)?;demand.depth=demand.depth.max(1);return Ok(demand)}if self.child.output_ready(){return Ok(crate::RetirementDemand{depth:1,..Default::default()})}self.child.advance_demands(source.project(1,|value|value.as_ref().unwrap()),body)}

    fn advance(&mut self, source: RetainedCloneRef<'_, Option<T>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional clone cursor is spent"));
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        let source_value = source.get();
        match source_value {
            None => {
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.output = Some(None);
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            Some(_) => {
                if self.child_value.is_some() {
                    if !self.child.terminal_is_empty() {
                        if grant.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                        }
                        let step = self.child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained optional child scaffold close")?.progress()));
                    }
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.output = Some(Some(self.child_value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child owner is missing"))?));
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                match self.child.advance(source.project(1, |value| value.as_ref().expect("validated retained optional child")), grant)? {
                    RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained optional child")?)),
                    RetainedCloneStep::Complete(progress) => {
                        let progress = admit_retained_clone_progress(grant, progress, "retained optional child")?;
                        if progress.copied_items >= grant.maximum_items {
                            return Ok(RetainedCloneStep::Progress(progress));
                        }
                        self.child_value = Some(self.child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained optional child completed without an owner"))?);
                        let _ = self.child.begin_close();
                        Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
                    }
                }
            }
        }
    }

    fn take(&mut self) -> Option<Option<T>> {
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if !self.child.terminal_is_empty() {
            if self.child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.child.terminal_is_empty(), "retained child close")?.progress()));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.child_value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if !self.child.terminal_is_empty(){return self.child.next_close_depth_demand();} self.close.next_owner_depth_with_binding(self.child_value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_capacity_byte_demand(maximum_release_bytes); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Option<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.child.terminal_is_empty() { return self.child.next_close_release_byte_demand(); }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.terminal_is_empty() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Option<T> {
    type Cursor = OptionCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        OptionCursor::default()
    }
}

pub struct BoxCursor<T: RetainedClone> {
    child: Option<Box<T::Cursor>>,
    value: Option<T>,
    output: Option<Box<T>>,
    source: Option<RetainedCloneBinding>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<T: RetainedClone> Default for BoxCursor<T> {
    fn default() -> Self {
        Self { child: None, value: None, output: None, source: None, spent: false, closing: false, close: RetainedCloneClose::default() }
    }
}

impl<T: RetainedClone> RetainedCloneCursor<Box<T>> for BoxCursor<T> {
    fn output_ready(&self)->bool{self.output.is_some()}
    fn advance_demands(&self,source:RetainedCloneRef<'_,Box<T>>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"boxed original clone is closing or spent"))}if self.output.is_some(){return Ok(Default::default())}if self.value.is_some(){if let Some(child)=self.child.as_ref(){if !child.terminal_is_empty(){return clone_cursor_close_demand::<T>(&**child,body)}return Ok(crate::RetirementDemand{release_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}return Ok(crate::RetirementDemand{capacity_bytes:size_of::<T>(),depth:1,..Default::default()})}if let Some(child)=self.child.as_ref(){let mut demand=child.advance_demands(source.project(1,|boxed|boxed.as_ref()),body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"boxed original clone quote depth overflow"))?;Ok(demand)}else{Ok(crate::RetirementDemand{capacity_bytes:size_of::<T::Cursor>(),depth:1,..Default::default()})}}

    fn advance(&mut self, source: RetainedCloneRef<'_, Box<T>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box clone cursor is spent"));
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.value.is_some() {
            if let Some(child) = self.child.as_mut() {
                if !child.terminal_is_empty() {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    let step = child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained box child scaffold close")?.progress()));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let bytes = size_of::<T::Cursor>();
                if bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.child = None;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: bytes }));
            }
            let capacity = size_of::<T>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            let value = self.value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child owner is missing"))?;
            self.output = Some(Box::new(value));
            return Ok(RetainedCloneStep::Complete(progress));
        }
        if self.child.is_none() {
            if grant.maximum_depth == 0 {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::DepthLimit, "retained box clone structural depth limit exceeded"));
            }
            let capacity = size_of::<T::Cursor>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: capacity, released_bytes: 0 };
            if !progress.fits(grant) {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            self.child = Some(Box::new(T::retained_clone_cursor()));
            return Ok(RetainedCloneStep::Progress(progress));
        }
        let child = self.child.as_mut().expect("initialized retained box child");
        let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        match child.advance(source.project(1, |boxed| boxed.as_ref()), child_grant)? {
            RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(child_grant, progress, "retained box child")?)),
            RetainedCloneStep::Complete(progress) => {
                let progress = admit_retained_clone_progress(child_grant, progress, "retained box child")?;
                if progress.copied_items >= grant.maximum_items {
                    return Ok(RetainedCloneStep::Progress(progress));
                }
                self.value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained box child completed without an owner"))?);
                let _ = child.begin_close();
                Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
            }
        }
    }

    fn take(&mut self) -> Option<Box<T>> {
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "native owner must begin close before granted retirement")); }
        if let Some(child) = self.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            if grant.maximum_depth==0 { return Err(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"boxed clone scaffold release requires depth")); }
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: size_of::<T::Cursor>() };
            if !progress.fits(grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }
            self.child = None;
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(1)}else{child.next_close_depth_demand()};} self.close.next_owner_depth_with_binding(self.value.is_some()||self.output.is_some(),&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.value.is_some() { return self.close.next_owner_capacity_with_binding::<T>(true,maximum_release_bytes,&self.source); }
        self.close.next_owner_capacity_with_binding::<Box<T>>(self.output.is_some(),maximum_release_bytes,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(size_of::<T::Cursor>()) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_with_binding(&self.source)
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.child.is_none() && self.value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() }
}

impl<T: RetainedClone> RetainedClone for Box<T> {
    type Cursor = BoxCursor<T>;
    fn retained_clone_cursor() -> Self::Cursor {
        BoxCursor::default()
    }
}

macro_rules! retained_clone_tuple {
    ($cursor:ident, $count:expr, $(($index:tt, $type:ident, $child:ident, $value:ident)),+ $(,)?) => {
        pub struct $cursor<$($type: RetainedClone),+> {
            $($child: $type::Cursor, $value: Option<$type>,)+
            phase: usize,
            source: Option<RetainedCloneBinding>,
            output: Option<($($type,)+)>,
            spent: bool,
            draining: bool,
            closing: bool,
            close: RetainedCloneClose,
        }

        impl<$($type: RetainedClone),+> Default for $cursor<$($type),+> {
            fn default() -> Self {
                Self {
                    $($child: $type::retained_clone_cursor(), $value: None,)+
                    phase: 0,
                    source: None,
                    output: None,
                    spent: false,
                    draining: false,
                    closing: false,
                    close: RetainedCloneClose::default(),
                }
            }
        }

        impl<$($type: RetainedClone),+> RetainedCloneCursor<($($type,)+)> for $cursor<$($type),+> {
    fn output_ready(&self)->bool{self.output.is_some()}
            fn advance_demands(&self,source:RetainedCloneRef<'_,($($type,)+)>,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}if self.closing||self.spent{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"tuple original clone is closing or spent"))}if self.output.is_some(){return Ok(Default::default())}match self.phase{$($index=>if self.draining{clone_cursor_close_demand::<$type>(&self.$child,body)}else{self.$child.advance_demands(source.project($index+1,|value|&value.$index),body)},)+_=>Ok(crate::RetirementDemand{depth:1,..Default::default()})}}

            fn advance(&mut self, source: RetainedCloneRef<'_, ($($type,)+)>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
                if self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is closing")); }
                if self.spent { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone cursor is spent")); }
                if self.output.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
                if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(RetainedCloneStep::Progress(progress));}
                if self.draining {
                    match self.phase {
                        $($index => {
                            if !self.$child.terminal_is_empty() {
                                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                                let step = self.$child.close_step(grant)?;
                    return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained tuple child scaffold close")?.progress()));
                            }
                            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                            self.draining = false;
                            self.phase += 1;
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                        },)+
                        _ => return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple drain state is invalid")),
                    }
                }
                match self.phase {
                    $($index => match self.$child.advance(source.project($index + 1, |value| &value.$index), grant)? {
                        RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained tuple child")?)),
                        RetainedCloneStep::Complete(progress) => {
                            let progress = admit_retained_clone_progress(grant, progress, "retained tuple child")?;
                            if progress.copied_items >= grant.maximum_items {
                                return Ok(RetainedCloneStep::Progress(progress));
                            }
                            self.$value = Some(self.$child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple child completed without an owner"))?);
                            let _ = self.$child.begin_close();
                            self.draining = true;
                            Ok(RetainedCloneStep::Progress(progress.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?))
                        }
                    },)+
                    $count => {
                        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                        self.output = Some(($(self.$value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple owner is missing"))?,)+));
                        self.phase += 1;
                        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
                    }
                    _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained tuple clone state is invalid")),
                }
            }

            fn take(&mut self) -> Option<($($type,)+)> {
                let output = self.output.take();
                if output.is_some() { self.spent = true; }
                output
            }

            fn begin_close(&mut self) -> bool {
                if self.closing { return false; }
                self.closing = true;
                true
            }

            

            fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "tuple must begin close before granted retirement")); }
                $(if !self.$child.terminal_is_empty() {
                    if self.$child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
                    let step = self.$child.close_step(grant)?; return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.$child.terminal_is_empty(), "retained child close")?.progress()));
                })+
                if !self.close.is_empty() { return self.close.step_granted(grant); }
                $(if let Some(step) = self.close.begin_granted(&mut self.$value, grant)? { return Ok(step); })+
                if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
                close_retained_binding(&mut self.source, grant)
            }

            fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if !self.closing {return Ok(0);} $(if !self.$child.terminal_is_empty(){return self.$child.next_close_depth_demand();})+ self.close.next_owner_depth_with_binding(self.output.is_some()$(||self.$value.is_some())+,&self.source)}
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_copy_byte_demand(); })+
                self.close.next_copy_with_binding(&self.source)
            }
            fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_capacity_byte_demand(maximum_release_bytes); })+
                if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
                $(if self.$value.is_some() { return self.close.next_owner_capacity_with_binding::<$type>(true,maximum_release_bytes,&self.source); })+
                self.close.next_owner_capacity_with_binding::<($($type,)+)>(self.output.is_some(),maximum_release_bytes,&self.source)
            }
            fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
                if !self.closing { return Ok(0); }
                $(if !self.$child.terminal_is_empty() { return self.$child.next_close_release_byte_demand(); })+
                self.close.next_release_with_binding(&self.source)
            }
            fn terminal_is_empty(&self) -> bool {
                self.closing && self.output.is_none() && self.close.is_empty() && self.source.is_none() $(&& self.$value.is_none() && self.$child.terminal_is_empty())+
            }
        }

        impl<$($type: RetainedClone),+> RetainedClone for ($($type,)+) {
            type Cursor = $cursor<$($type),+>;
            fn retained_clone_cursor() -> Self::Cursor { Default::default() }
        }
    };
}

retained_clone_tuple!(Tuple2Cursor, 2, (0, A, first_cursor, first_value), (1, B, second_cursor, second_value));
retained_clone_tuple!(Tuple3Cursor, 3, (0, A, first_cursor, first_value), (1, B, second_cursor, second_value), (2, C, third_cursor, third_value));


#[cfg(test)]
#[path="../🧮️scalar/🧪️tests/🦀️.rs"]
mod scalar_authority_tests;
#[cfg(test)]
#[path="../🚚️bulk-vec/🧪️tests/🦀️.rs"]
mod bulk_vec_tests;
#[cfg(test)]
pub(crate) use semio_framework_value_clone_source_testing::{FixtureSource,FixtureSourceBirth};

#[cfg(test)]
#[path="🧪️tests/♻️close/🦀️.rs"]
mod original_close_frame_tests;
