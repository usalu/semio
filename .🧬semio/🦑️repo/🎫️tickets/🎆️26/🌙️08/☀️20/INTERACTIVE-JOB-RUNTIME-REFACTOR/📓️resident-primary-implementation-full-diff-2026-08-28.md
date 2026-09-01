# Resident Primary Full Source Diff and Inverse

Source-only review packet. The inverse was checked in memory and has not been applied to disk. Full exact preimages, postimages, hunks and hashes are retained in the adjacent JSON packet. No compiler or native tests ran.

## 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs

### Forward

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
@@ -65,7 +65,7 @@
 //#endregion 📏️Capacity
 
 //#region 📨️AdmissionVocabulary
-use std::{alloc::Layout, any::TypeId, cell::UnsafeCell, marker::PhantomData, mem::size_of, ops::{Deref, DerefMut}, ptr::NonNull, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};
+use std::{alloc::Layout, any::TypeId, cell::UnsafeCell, marker::PhantomData, mem::size_of, num::NonZeroU64, ops::{Deref, DerefMut}, ptr::NonNull, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};
 
 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
 pub enum ResidentPartition { Data, Control }
@@ -147,7 +147,7 @@
 //#region 🏠️OriginalRoot
 pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
 
-enum ResidentReleaseOrigin { Record, PendingAdmission, Admission, PendingConsumer, Consumer }
+enum ResidentReleaseOrigin { Record, PendingAdmission, Admission, PendingConsumer, Consumer, PrimaryPending { registration: NonZeroU64 }, PrimaryConsumer { registration: NonZeroU64 } }
 struct ResidentReleaseAllocation { pointer: NonNull<u8>, layout: Layout }
 enum ResidentReleaseStage {
     Destroy { allocation: ResidentReleaseAllocation, destroy_empty: unsafe fn(NonNull<u8>) },
@@ -174,6 +174,24 @@
     parts.iter().try_fold(0u64, |sum, bytes| sum.checked_add(*bytes as u64).ok_or(ResidentFault::Count))
 }
 
+#[derive(Clone, Copy, PartialEq, Eq)]
+struct ResidentRegistrationStamp { generation: NonZeroU64, type_id: TypeId }
+
+enum ResidentPrimaryBacking { Pending(ConsumerPage), Published(NonNull<ConsumerHeader>), Releasing }
+struct ResidentPrimaryAnchor { stamp: ResidentRegistrationStamp, partition: ResidentPartition, backing: ResidentPrimaryBacking }
+struct ResidentRecoveryPin { pointer: NonNull<ConsumerHeader>, registration: NonZeroU64 }
+
+#[derive(Clone, Copy, Debug, PartialEq, Eq)]
+pub enum ResidentRecoveryMode { Forward, Closing }
+
+struct ResidentRecoveryCursor {
+    stamp: ResidentRegistrationStamp,
+    mode: ResidentRecoveryMode,
+    revoked: bool,
+    next: Option<ResidentRecoveryPin>,
+    found: Option<ResidentRecoveryPin>,
+}
+
 struct LedgerState {
     capacity: ResidentCapacity,
     data: ResidentResources,
@@ -186,6 +204,9 @@
     consumers: Option<ConsumerPage>,
     pending_consumer: Option<ConsumerPage>,
     prepared_consumer: Option<NonNull<ConsumerHeader>>,
+    last_consumer_registration: u64,
+    primary: Option<ResidentPrimaryAnchor>,
+    recovery: Option<ResidentRecoveryCursor>,
     closing: bool,
     closed: bool,
     #[cfg(test)]
@@ -201,7 +222,7 @@
     fn try_lock_close(&self) -> Result<ResidentAccessGuard<'_, LedgerState>, ResidentAccessError> {
         if self.held.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() { return Err(ResidentAccessError::Busy); }
         let guard = ResidentAccessGuard { access: self, thread: PhantomData };
-        if self.poisoned.load(Ordering::Relaxed) && !guard.release.as_ref().is_some_and(|release| release.stage.pointerless()) && !guard.structurally_empty() {
+        if self.poisoned.load(Ordering::Relaxed) && !guard.pointerless_close_allowed() && !guard.structurally_empty() {
             return Err(ResidentAccessError::Poisoned);
         }
         Ok(guard)
@@ -211,13 +232,13 @@
 impl ResidentLedgerRoot {
     pub fn new(capacity: ResidentCapacity) -> Self {
         let zero = ResidentResources { bytes: 0, slots: 0, owners: 0 };
-        Self { state: ResidentAccess::new(LedgerState { capacity, data: zero, control: zero, allocated_bytes: 0, head: None, pending: None, prepared: None, release: None, consumers: None, pending_consumer: None, prepared_consumer: None, closing: false, closed: false, #[cfg(test)] consumer_release_interlock: None }) }
+        Self { state: ResidentAccess::new(LedgerState { capacity, data: zero, control: zero, allocated_bytes: 0, head: None, pending: None, prepared: None, release: None, consumers: None, pending_consumer: None, prepared_consumer: None, last_consumer_registration: 0, primary: None, recovery: None, closing: false, closed: false, #[cfg(test)] consumer_release_interlock: None }) }
     }
 
     pub fn ledger(&self) -> ResidentLedger<'_> { ResidentLedger { root: self } }
 
     pub fn native_layout<C, S>(&self) -> ResidentNativeLayout {
-        ResidentNativeLayout { root_bytes: size_of::<Self>() as u64, admission_page_bytes: size_of::<AdmissionNode>() as u64, consumer_page_bytes: size_of::<ConsumerNode<C>>() as u64, record_page_bytes: size_of::<RecordNode<S>>() as u64, consumer_move_bytes: size_of::<Option<C>>() as u64, shell_move_bytes: size_of::<Option<S>>() as u64, descriptor_move_bytes: size_of::<ErasedRecord>().max(size_of::<Option<AdmissionPage>>()).max(size_of::<ConsumerPage>()).max(size_of::<Option<ResidentRelease>>()) as u64, final_root_bytes: size_of::<Self>() as u64, release_slot_bytes: size_of::<Option<ResidentRelease>>() as u64, pending_consumer_bytes: size_of::<Option<ConsumerPage>>() as u64 }
+        ResidentNativeLayout { root_bytes: size_of::<Self>() as u64, admission_page_bytes: size_of::<AdmissionNode>() as u64, consumer_page_bytes: size_of::<ConsumerNode<C>>() as u64, record_page_bytes: size_of::<RecordNode<S>>() as u64, consumer_move_bytes: size_of::<Option<C>>() as u64, shell_move_bytes: size_of::<Option<S>>() as u64, descriptor_move_bytes: size_of::<ErasedRecord>().max(size_of::<Option<AdmissionPage>>()).max(size_of::<ConsumerPage>()).max(size_of::<Option<ResidentRelease>>()).max(size_of::<Option<ResidentPrimaryAnchor>>()).max(size_of::<Option<ResidentRecoveryCursor>>()) as u64, final_root_bytes: size_of::<Self>() as u64, release_slot_bytes: size_of::<Option<ResidentRelease>>() as u64, pending_consumer_bytes: size_of::<Option<ConsumerPage>>() as u64 }
     }
 
     pub fn allocated_bytes(&self) -> Result<u64, ResidentFault> { self.access()?.map(|state| state.allocated_bytes).ok_or(ResidentFault::Busy) }
@@ -246,6 +267,7 @@
             return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
         }
         if state.release.is_some() { return state.advance_release(grant); }
+        if state.recovery.is_some() { return state.close_recovery(grant); }
         if let Some(pending) = state.pending.as_mut() {
             if let Some(consumer) = pending.consumer.as_ref() {
                 if !consumer.is_empty() { return Ok(ResidentStep::blocked()); }
@@ -297,6 +319,9 @@
             state.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::Admission, partition, charge, stage });
             return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
         }
+        if state.primary.as_ref().is_some_and(|anchor| matches!(&anchor.backing, ResidentPrimaryBacking::Pending(_))) {
+            return state.detach_pending_primary(grant);
+        }
         if let Some(page) = state.pending_consumer.as_ref() {
             if page.initialized && page.pointer.is_none_or(|pointer| !unsafe { (page.empty)(pointer) }) { return Ok(ResidentStep::blocked()); }
             let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>()])?;
@@ -322,15 +347,18 @@
                 interlock.observed.try_send(()).map_err(|_| ResidentFault::Identity)?;
                 interlock.resume.recv_timeout(std::time::Duration::from_secs(1)).map_err(|_| ResidentFault::Identity)?;
             }
-            if header.aliases.load(Ordering::Acquire) != 0 || header.admissions.load(Ordering::Acquire) != 0 { return Ok(ResidentStep::blocked()); }
+            if header.aliases.load(Ordering::Acquire) != 0 || header.admissions.load(Ordering::Acquire) != 0 || header.recovery_pins.load(Ordering::Acquire) != 0 { return Ok(ResidentStep::blocked()); }
             let clear_prepared = state.prepared_consumer == Some(pointer);
-            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>(), if clear_prepared { size_of::<Option<NonNull<ConsumerHeader>>>() } else { 0 }])?;
+            let primary = state.primary_for_page(page)?;
+            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>(), if clear_prepared { size_of::<Option<NonNull<ConsumerHeader>>>() } else { 0 }, if primary.is_some() { size_of::<ResidentPrimaryBacking>() } else { 0 }])?;
             if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+            if primary.is_some() { state.primary.as_mut().unwrap().backing = ResidentPrimaryBacking::Releasing; }
             let page = state.consumers.take().unwrap();
             state.consumers = unsafe { (&mut *header.next.get()).take() };
             if clear_prepared { state.prepared_consumer = None; }
             let stage = ResidentReleaseStage::allocated(pointer.cast(), page.layout, true, page.destroy_empty);
-            state.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::Consumer, partition: page.partition, charge: page.charge, stage });
+            let origin = primary.map_or(ResidentReleaseOrigin::Consumer, |registration| ResidentReleaseOrigin::PrimaryConsumer { registration });
+            state.release = Some(ResidentRelease { origin, partition: page.partition, charge: page.charge, stage });
             return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
         }
         if !state.structurally_empty() { return Err(ResidentFault::Capacity); }
@@ -342,16 +370,84 @@
 }
 
 impl LedgerState {
+    fn primary_for_page(&self, page: &ConsumerPage) -> Result<Option<NonZeroU64>, ResidentFault> {
+        let Some(anchor) = self.primary.as_ref() else { return Ok(None); };
+        let ResidentPrimaryBacking::Published(pointer) = &anchor.backing else { return Ok(None); };
+        let same_pointer = page.pointer == Some(*pointer);
+        let same_registration = page.registration == anchor.stamp.generation;
+        if same_pointer != same_registration { return Err(ResidentFault::Identity); }
+        if !same_pointer { return Ok(None); }
+        if page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
+        Ok(Some(page.registration))
+    }
+
+    fn detach_pending_primary(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let anchor = self.primary.as_mut().ok_or(ResidentFault::Identity)?;
+        let ResidentPrimaryBacking::Pending(page) = &anchor.backing else { return Ok(ResidentStep::rejected()); };
+        if page.registration != anchor.stamp.generation || page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
+        let bytes = resident_release_work(&[size_of::<ResidentPrimaryBacking>(), size_of::<Option<ResidentRelease>>()])?;
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        if page.initialized && page.pointer.is_none_or(|pointer| !unsafe { (page.empty)(pointer) }) { return Ok(ResidentStep::blocked()); }
+        let ResidentPrimaryBacking::Pending(page) = std::mem::replace(&mut anchor.backing, ResidentPrimaryBacking::Releasing) else { unreachable!() };
+        let stage = match page.pointer {
+            Some(pointer) => ResidentReleaseStage::allocated(pointer.cast(), page.layout, page.initialized, page.destroy_empty),
+            None => ResidentReleaseStage::Refund { released_layout: None },
+        };
+        self.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::PrimaryPending { registration: page.registration }, partition: page.partition, charge: page.charge, stage });
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
+    }
+
+    fn close_recovery(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let cursor = self.recovery.as_mut().ok_or(ResidentFault::Identity)?;
+        if !cursor.revoked {
+            let bytes = size_of::<bool>() as u64;
+            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+            cursor.revoked = true;
+            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
+        }
+        let slot = if cursor.next.is_some() { &mut cursor.next } else { &mut cursor.found };
+        if let Some(pin) = slot.as_ref() {
+            let bytes = resident_release_work(&[size_of::<Option<ResidentRecoveryPin>>(), size_of::<AtomicUsize>()])?;
+            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+            #[cfg(test)]
+            tests::observe_primary_recovery_pointer_load(pin.registration.get());
+            let header = unsafe { pin.pointer.as_ref() };
+            let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
+            header.recovery_pins.store(remaining, Ordering::Release);
+            *slot = None;
+            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
+        }
+        let bytes = size_of::<Option<ResidentRecoveryCursor>>() as u64;
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        self.recovery = None;
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
+    }
+
+    fn primary_recovery_target<C: Send + 'static>(&self, mode: ResidentRecoveryMode) -> Result<Option<(ResidentRegistrationStamp, NonNull<ConsumerHeader>)>, ResidentFault> {
+        if self.closed || self.closing != (mode == ResidentRecoveryMode::Closing) { return Err(ResidentFault::Closed); }
+        let Some(anchor) = self.primary.as_ref() else { return Ok(None); };
+        if anchor.stamp.type_id != TypeId::of::<C>() { return Err(ResidentFault::Identity); }
+        match &anchor.backing { ResidentPrimaryBacking::Published(pointer) => Ok(Some((anchor.stamp, *pointer))), _ => Ok(None) }
+    }
+
+    fn pointerless_close_allowed(&self) -> bool {
+        self.release.as_ref().is_some_and(|release| release.stage.pointerless()) && self.recovery.is_none() && self.primary.as_ref().is_none_or(|anchor| matches!(&anchor.backing, ResidentPrimaryBacking::Releasing))
+    }
+
     fn structurally_empty(&self) -> bool {
         #[cfg(test)]
         if self.consumer_release_interlock.is_some() { return false; }
         let zero = ResidentResources { bytes: 0, slots: 0, owners: 0 };
-        self.head.is_none() && self.pending.is_none() && self.prepared.is_none() && self.release.is_none() && self.consumers.is_none() && self.pending_consumer.is_none() && self.prepared_consumer.is_none() && self.allocated_bytes == 0 && self.data == zero && self.control == zero
+        self.head.is_none() && self.pending.is_none() && self.prepared.is_none() && self.release.is_none() && self.consumers.is_none() && self.pending_consumer.is_none() && self.prepared_consumer.is_none() && self.primary.is_none() && self.recovery.is_none() && self.allocated_bytes == 0 && self.data == zero && self.control == zero
     }
 
     fn advance_release(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
         let release = self.release.as_mut().ok_or(ResidentFault::Identity)?;
         let slot = size_of::<Option<ResidentRelease>>();
+        let primary = match &release.origin { ResidentReleaseOrigin::PrimaryPending { registration } | ResidentReleaseOrigin::PrimaryConsumer { registration } => Some(*registration), _ => None };
+        if let Some(registration) = primary {
+            if self.primary.as_ref().is_none_or(|anchor| anchor.stamp.generation != registration || anchor.partition != release.partition || !matches!(&anchor.backing, ResidentPrimaryBacking::Releasing)) { return Err(ResidentFault::Identity); }
+        }
         match &release.stage {
             ResidentReleaseStage::Destroy { allocation, destroy_empty } => {
                 let bytes = resident_release_work(&[allocation.layout.size(), slot])?;
@@ -384,8 +480,9 @@
                 Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
             }
             ResidentReleaseStage::Clear { .. } => {
-                let bytes = slot as u64;
+                let bytes = resident_release_work(&[slot, if primary.is_some() { size_of::<Option<ResidentPrimaryAnchor>>() } else { 0 }])?;
                 if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+                if primary.is_some() { self.primary = None; }
                 self.release = None;
                 Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
             }
@@ -413,9 +510,11 @@
 struct ConsumerHeader {
     aliases: AtomicUsize,
     admissions: AtomicUsize,
+    recovery_pins: AtomicUsize,
     closing: UnsafeCell<bool>,
     next: UnsafeCell<Option<ConsumerPage>>,
     type_id: TypeId,
+    registration: NonZeroU64,
 }
 
 #[repr(C)]
@@ -428,17 +527,46 @@
     charge: ResidentResources,
     initialized: bool,
     type_id: TypeId,
-    initialize: unsafe fn(NonNull<ConsumerHeader>),
+    registration: NonZeroU64,
+    initialize: unsafe fn(NonNull<ConsumerHeader>, NonZeroU64),
     empty: unsafe fn(NonNull<ConsumerHeader>) -> bool,
     destroy_empty: unsafe fn(NonNull<u8>),
 }
 
+/// 🧷️ Typed constructors bind Send sources and exact layouts; pointers remain in the original gated root.
 unsafe impl Send for ConsumerPage {}
 
-unsafe fn initialize_consumer<C: Send + 'static>(pointer: NonNull<ConsumerHeader>) {
-    unsafe { pointer.cast::<ConsumerNode<C>>().as_ptr().write(ConsumerNode { header: ConsumerHeader { aliases: AtomicUsize::new(0), admissions: AtomicUsize::new(0), closing: UnsafeCell::new(false), next: UnsafeCell::new(None), type_id: TypeId::of::<C>() }, source: UnsafeCell::new(None) }); }
+impl ConsumerPage {
+    fn reserved<C: Send + 'static>(partition: ResidentPartition, registration: NonZeroU64) -> Result<Self, ResidentFault> {
+        let layout = Layout::new::<ConsumerNode<C>>();
+        let charge = ResidentResources::new(layout.size() as u64, 1, 1)?;
+        Ok(Self { pointer: None, layout, partition, charge, initialized: false, type_id: TypeId::of::<C>(), registration, initialize: initialize_consumer::<C>, empty: empty_consumer::<C>, destroy_empty: destroy_consumer::<C> })
+    }
+
+    fn allocate(&mut self, allocated_bytes: &mut u64, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let bytes = resident_release_work(&[self.layout.size(), size_of::<Option<NonNull<ConsumerHeader>>>(), size_of::<u64>()])?;
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        let allocated = allocated_bytes.checked_add(self.layout.size() as u64).ok_or(ResidentFault::Count)?;
+        let pointer = NonNull::new(unsafe { std::alloc::alloc(self.layout) }.cast::<ConsumerHeader>()).ok_or(ResidentFault::Allocation)?;
+        self.pointer = Some(pointer);
+        *allocated_bytes = allocated;
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
+    }
+
+    fn initialize(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let bytes = resident_release_work(&[self.layout.size(), size_of::<bool>()])?;
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        let pointer = self.pointer.ok_or(ResidentFault::Identity)?;
+        unsafe { (self.initialize)(pointer, self.registration); }
+        self.initialized = true;
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
+    }
 }
 
+unsafe fn initialize_consumer<C: Send + 'static>(pointer: NonNull<ConsumerHeader>, registration: NonZeroU64) {
+    unsafe { pointer.cast::<ConsumerNode<C>>().as_ptr().write(ConsumerNode { header: ConsumerHeader { aliases: AtomicUsize::new(0), admissions: AtomicUsize::new(0), recovery_pins: AtomicUsize::new(0), closing: UnsafeCell::new(false), next: UnsafeCell::new(None), type_id: TypeId::of::<C>(), registration }, source: UnsafeCell::new(None) }); }
+}
+
 unsafe fn empty_consumer<C>(pointer: NonNull<ConsumerHeader>) -> bool { unsafe { (&*pointer.cast::<ConsumerNode<C>>().as_ref().source.get()).is_none() } }
 unsafe fn destroy_consumer<C>(pointer: NonNull<u8>) { unsafe { pointer.cast::<ConsumerNode<C>>().as_ptr().drop_in_place(); } }
 
@@ -511,38 +639,160 @@
 impl ResidentLedgerRoot {
     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
         let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
-        if state.closing { return Ok(ResidentStep::rejected()); }
-        let layout = Layout::new::<ConsumerNode<C>>();
-        let bytes = layout.size() as u64;
-        if !grant.admits(bytes.max(size_of::<ConsumerPage>() as u64)) { return Ok(ResidentStep::blocked()); }
+        let state = &mut *state;
+        if state.closing || state.closed { return Ok(ResidentStep::rejected()); }
         if let Some(page) = state.pending_consumer.as_mut() {
             if page.type_id != TypeId::of::<C>() || page.partition != partition { return Ok(ResidentStep::rejected()); }
-            if page.pointer.is_none() {
-                let allocated = state.allocated_bytes.checked_add(bytes).ok_or(ResidentFault::Count)?;
-                let pointer = NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<ConsumerHeader>()).ok_or(ResidentFault::Allocation)?;
-                state.pending_consumer.as_mut().unwrap().pointer = Some(pointer);
-                state.allocated_bytes = allocated;
-                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
-            }
-            if !page.initialized {
-                unsafe { (page.initialize)(page.pointer.unwrap()); }
-                page.initialized = true;
-                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
-            }
+            if page.pointer.is_none() { return page.allocate(&mut state.allocated_bytes, grant); }
+            if !page.initialized { return page.initialize(grant); }
+            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<NonNull<ConsumerHeader>>>()])?;
+            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
             let page = state.pending_consumer.take().unwrap();
             let pointer = page.pointer.unwrap();
             unsafe { *pointer.as_ref().next.get() = state.consumers.take(); }
             state.prepared_consumer = Some(pointer);
             state.consumers = Some(page);
-            return Ok(ResidentStep::done(ResidentStepKind::Ready, size_of::<ConsumerPage>() as u64));
+            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
         }
-        let charge = ResidentResources { bytes, slots: 1, owners: 1 };
-        if state.reserve(partition, charge).is_err() { return Ok(ResidentStep::blocked()); }
-        state.pending_consumer = Some(ConsumerPage { pointer: None, layout, partition, charge, initialized: false, type_id: TypeId::of::<C>(), initialize: initialize_consumer::<C>, empty: empty_consumer::<C>, destroy_empty: destroy_consumer::<C> });
+        let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<u64>(), size_of::<ResidentResources>(), size_of::<Option<NonNull<ConsumerHeader>>>()])?;
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        let registration = NonZeroU64::new(state.last_consumer_registration.checked_add(1).ok_or(ResidentFault::Count)?).ok_or(ResidentFault::Count)?;
+        let page = ConsumerPage::reserved::<C>(partition, registration)?;
+        if state.reserve(partition, page.charge).is_err() { return Ok(ResidentStep::blocked()); }
+        state.last_consumer_registration = registration.get();
+        state.pending_consumer = Some(page);
         state.prepared_consumer = None;
-        Ok(ResidentStep::done(ResidentStepKind::Pending, size_of::<ConsumerPage>() as u64))
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
     }
 
+    pub fn reserve_primary_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
+        if state.closing || state.closed || state.primary.is_some() { return Ok(ResidentStep::rejected()); }
+        let bytes = resident_release_work(&[size_of::<Option<ResidentPrimaryAnchor>>(), size_of::<u64>(), size_of::<ResidentResources>()])?;
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        let registration = NonZeroU64::new(state.last_consumer_registration.checked_add(1).ok_or(ResidentFault::Count)?).ok_or(ResidentFault::Count)?;
+        let page = ConsumerPage::reserved::<C>(partition, registration)?;
+        if state.reserve(partition, page.charge).is_err() { return Ok(ResidentStep::blocked()); }
+        state.last_consumer_registration = registration.get();
+        state.primary = Some(ResidentPrimaryAnchor { stamp: ResidentRegistrationStamp { generation: registration, type_id: TypeId::of::<C>() }, partition, backing: ResidentPrimaryBacking::Pending(page) });
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
+    }
+
+    pub fn prepare_primary_consumer<C: Send + 'static>(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
+        let state = &mut *state;
+        if state.closing || state.closed { return Ok(ResidentStep::rejected()); }
+        let Some(anchor) = state.primary.as_mut() else { return Ok(ResidentStep::rejected()); };
+        if anchor.stamp.type_id != TypeId::of::<C>() { return Err(ResidentFault::Identity); }
+        let ResidentPrimaryBacking::Pending(page) = &mut anchor.backing else { return Ok(ResidentStep::rejected()); };
+        if page.registration != anchor.stamp.generation || page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
+        if page.pointer.is_none() { return page.allocate(&mut state.allocated_bytes, grant); }
+        if !page.initialized { return page.initialize(grant); }
+        let bytes = resident_release_work(&[size_of::<ResidentPrimaryBacking>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>()])?;
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        let pointer = page.pointer.unwrap();
+        let ResidentPrimaryBacking::Pending(page) = std::mem::replace(&mut anchor.backing, ResidentPrimaryBacking::Published(pointer)) else { unreachable!() };
+        unsafe { *pointer.as_ref().next.get() = state.consumers.take(); }
+        state.consumers = Some(page);
+        Ok(ResidentStep::done(ResidentStepKind::Ready, bytes))
+    }
+
+    pub fn begin_primary_recovery<C: Send + 'static>(&self, mode: ResidentRecoveryMode, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
+        let Some((stamp, _)) = state.primary_recovery_target::<C>(mode)? else { return Ok(ResidentStep::rejected()); };
+        if state.recovery.is_some() { return Ok(ResidentStep::rejected()); }
+        let bytes = resident_release_work(&[size_of::<Option<ResidentRecoveryCursor>>(), size_of::<AtomicUsize>()])?;
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        let page = state.consumers.as_ref().ok_or(ResidentFault::Identity)?;
+        if !page.initialized { return Err(ResidentFault::Identity); }
+        let pointer = page.pointer.ok_or(ResidentFault::Identity)?;
+        #[cfg(test)]
+        tests::observe_primary_recovery_pointer_load(page.registration.get());
+        let header = unsafe { pointer.as_ref() };
+        let count = header.recovery_pins.load(Ordering::Acquire).checked_add(1).ok_or(ResidentFault::Count)?;
+        let registration = page.registration;
+        header.recovery_pins.store(count, Ordering::Release);
+        state.recovery = Some(ResidentRecoveryCursor { stamp, mode, revoked: false, next: Some(ResidentRecoveryPin { pointer, registration }), found: None });
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
+    }
+
+    pub fn advance_primary_recovery<C: Send + 'static>(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
+        if state.closed { return Err(ResidentFault::Closed); }
+        let Some(cursor) = state.recovery.as_ref() else { return Ok(ResidentStep::rejected()); };
+        if cursor.revoked { return Err(ResidentFault::Closed); }
+        let Some((stamp, original)) = state.primary_recovery_target::<C>(cursor.mode)? else { return Ok(ResidentStep::rejected()); };
+        if cursor.stamp != stamp || cursor.found.is_some() { return Ok(ResidentStep::rejected()); }
+        let Some(pin) = cursor.next.as_ref() else { return Ok(ResidentStep::rejected()); };
+        let matching = pin.registration == stamp.generation;
+        if matching && pin.pointer != original { return Err(ResidentFault::Identity); }
+        let bytes = if matching {
+            resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<Option<ResidentRecoveryPin>>(), size_of::<Option<ResidentRecoveryPin>>()])?
+        } else {
+            resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<AtomicUsize>(), size_of::<AtomicUsize>(), size_of::<Option<ResidentRecoveryPin>>(), size_of::<Option<ResidentRecoveryPin>>()])?
+        };
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        #[cfg(test)]
+        tests::observe_primary_recovery_pointer_load(pin.registration.get());
+        let header = unsafe { pin.pointer.as_ref() };
+        if header.registration != pin.registration { return Err(ResidentFault::Identity); }
+        let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
+        if matching {
+            if header.type_id != stamp.type_id || (cursor.mode == ResidentRecoveryMode::Forward && unsafe { *header.closing.get() }) { return Err(ResidentFault::Identity); }
+            let cursor = state.recovery.as_mut().unwrap();
+            cursor.found = cursor.next.take();
+            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
+        }
+        let successor = unsafe { (&*header.next.get()).as_ref() }.ok_or(ResidentFault::Identity)?;
+        let pointer = successor.pointer.ok_or(ResidentFault::Identity)?;
+        let registration = successor.registration;
+        if !successor.initialized || pointer == pin.pointer || registration == pin.registration { return Err(ResidentFault::Identity); }
+        #[cfg(test)]
+        tests::observe_primary_recovery_pointer_load(registration.get());
+        let next = unsafe { pointer.as_ref() };
+        let count = next.recovery_pins.load(Ordering::Acquire).checked_add(1).ok_or(ResidentFault::Count)?;
+        next.recovery_pins.store(count, Ordering::Release);
+        header.recovery_pins.store(remaining, Ordering::Release);
+        state.recovery.as_mut().unwrap().next = Some(ResidentRecoveryPin { pointer, registration });
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
+    }
+
+    pub fn capture_primary_consumer<C: Send + 'static>(&self, mode: ResidentRecoveryMode, grant: ResidentGrant) -> Result<(ResidentStep, Option<ResidentConsumer<'_, C>>), ResidentFault> {
+        let Some(mut state) = self.access()? else { return Ok((ResidentStep::blocked(), None)); };
+        let Some((stamp, original)) = state.primary_recovery_target::<C>(mode)? else { return Ok((ResidentStep::rejected(), None)); };
+        let Some(cursor) = state.recovery.as_ref() else { return Ok((ResidentStep::rejected(), None)); };
+        if cursor.revoked { return Err(ResidentFault::Closed); }
+        if cursor.mode != mode || cursor.stamp != stamp || cursor.next.is_some() { return Ok((ResidentStep::rejected(), None)); }
+        let Some(pin) = cursor.found.as_ref() else { return Ok((ResidentStep::rejected(), None)); };
+        if pin.registration != stamp.generation || pin.pointer != original { return Err(ResidentFault::Identity); }
+        let bytes = resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<AtomicUsize>(), size_of::<AtomicUsize>(), size_of::<Option<ResidentRecoveryCursor>>(), size_of::<ResidentConsumer<'_, C>>()])?;
+        if !grant.admits(bytes) { return Ok((ResidentStep::blocked(), None)); }
+        #[cfg(test)]
+        tests::observe_primary_recovery_pointer_load(pin.registration.get());
+        let header = unsafe { pin.pointer.as_ref() };
+        if header.registration != stamp.generation || header.type_id != stamp.type_id { return Err(ResidentFault::Identity); }
+        if mode == ResidentRecoveryMode::Forward && unsafe { *header.closing.get() } { return Err(ResidentFault::Closed); }
+        let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
+        let aliases = header.aliases.load(Ordering::Acquire);
+        let count = aliases.checked_add(1).ok_or(ResidentFault::Count)?;
+        header.aliases.compare_exchange(aliases, count, Ordering::AcqRel, Ordering::Acquire).map_err(|_| ResidentFault::Busy)?;
+        header.recovery_pins.store(remaining, Ordering::Release);
+        state.recovery = None;
+        Ok((ResidentStep::done(ResidentStepKind::Ready, bytes), Some(ResidentConsumer { root: self, pointer: original.cast() })))
+    }
+
+    pub fn begin_primary_consumer_close(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
+        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
+        if state.closed || state.primary.is_none() { return Ok(ResidentStep::rejected()); }
+        let revoke = state.recovery.as_ref().is_some_and(|cursor| !cursor.revoked);
+        let bytes = resident_release_work(&[if state.closing { 0 } else { size_of::<bool>() }, if revoke { size_of::<bool>() } else { 0 }])?;
+        if bytes == 0 { return Ok(ResidentStep { kind: ResidentStepKind::Pending, items: 0, bytes: 0 }); }
+        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
+        if !state.closing { state.closing = true; }
+        if revoke { state.recovery.as_mut().unwrap().revoked = true; }
+        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
+    }
+
     pub fn prepared_consumer<C: Send + 'static>(&self) -> Result<Option<ResidentConsumer<'_, C>>, ResidentFault> {
         self.consumer_access(false)
     }
@@ -597,6 +847,7 @@
 
 struct PendingAdmission { page: Option<AdmissionPage>, consumer: Option<ErasedConsumer>, partition: ResidentPartition, charge: ResidentResources }
 
+/// 🪢️ Send-typed nodes stay in this gated root/list/Release; cursor pointers have counted same-root pins and captures acquire an alias before releasing a pin.
 unsafe impl Send for LedgerState {}
 
 pub struct ResidentAdmission<'root, C> { root: &'root ResidentLedgerRoot, node: NonNull<AdmissionNode>, marker: PhantomData<fn() -> C> }
```

### Inverse

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
@@ -65,7 +65,7 @@
 //#endregion 📏️Capacity
 
 //#region 📨️AdmissionVocabulary
+use std::{alloc::Layout, any::TypeId, cell::UnsafeCell, marker::PhantomData, mem::size_of, ops::{Deref, DerefMut}, ptr::NonNull, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};
-use std::{alloc::Layout, any::TypeId, cell::UnsafeCell, marker::PhantomData, mem::size_of, num::NonZeroU64, ops::{Deref, DerefMut}, ptr::NonNull, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};
 
 #[derive(Clone, Copy, Debug, PartialEq, Eq)]
 pub enum ResidentPartition { Data, Control }
@@ -147,7 +147,7 @@
 //#region 🏠️OriginalRoot
 pub struct ResidentLedgerRoot { state: ResidentAccess<LedgerState> }
 
+enum ResidentReleaseOrigin { Record, PendingAdmission, Admission, PendingConsumer, Consumer }
-enum ResidentReleaseOrigin { Record, PendingAdmission, Admission, PendingConsumer, Consumer, PrimaryPending { registration: NonZeroU64 }, PrimaryConsumer { registration: NonZeroU64 } }
 struct ResidentReleaseAllocation { pointer: NonNull<u8>, layout: Layout }
 enum ResidentReleaseStage {
     Destroy { allocation: ResidentReleaseAllocation, destroy_empty: unsafe fn(NonNull<u8>) },
@@ -174,24 +174,6 @@
     parts.iter().try_fold(0u64, |sum, bytes| sum.checked_add(*bytes as u64).ok_or(ResidentFault::Count))
 }
 
-#[derive(Clone, Copy, PartialEq, Eq)]
-struct ResidentRegistrationStamp { generation: NonZeroU64, type_id: TypeId }
-
-enum ResidentPrimaryBacking { Pending(ConsumerPage), Published(NonNull<ConsumerHeader>), Releasing }
-struct ResidentPrimaryAnchor { stamp: ResidentRegistrationStamp, partition: ResidentPartition, backing: ResidentPrimaryBacking }
-struct ResidentRecoveryPin { pointer: NonNull<ConsumerHeader>, registration: NonZeroU64 }
-
-#[derive(Clone, Copy, Debug, PartialEq, Eq)]
-pub enum ResidentRecoveryMode { Forward, Closing }
-
-struct ResidentRecoveryCursor {
-    stamp: ResidentRegistrationStamp,
-    mode: ResidentRecoveryMode,
-    revoked: bool,
-    next: Option<ResidentRecoveryPin>,
-    found: Option<ResidentRecoveryPin>,
-}
-
 struct LedgerState {
     capacity: ResidentCapacity,
     data: ResidentResources,
@@ -204,9 +186,6 @@
     consumers: Option<ConsumerPage>,
     pending_consumer: Option<ConsumerPage>,
     prepared_consumer: Option<NonNull<ConsumerHeader>>,
-    last_consumer_registration: u64,
-    primary: Option<ResidentPrimaryAnchor>,
-    recovery: Option<ResidentRecoveryCursor>,
     closing: bool,
     closed: bool,
     #[cfg(test)]
@@ -222,7 +201,7 @@
     fn try_lock_close(&self) -> Result<ResidentAccessGuard<'_, LedgerState>, ResidentAccessError> {
         if self.held.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() { return Err(ResidentAccessError::Busy); }
         let guard = ResidentAccessGuard { access: self, thread: PhantomData };
+        if self.poisoned.load(Ordering::Relaxed) && !guard.release.as_ref().is_some_and(|release| release.stage.pointerless()) && !guard.structurally_empty() {
-        if self.poisoned.load(Ordering::Relaxed) && !guard.pointerless_close_allowed() && !guard.structurally_empty() {
             return Err(ResidentAccessError::Poisoned);
         }
         Ok(guard)
@@ -232,13 +211,13 @@
 impl ResidentLedgerRoot {
     pub fn new(capacity: ResidentCapacity) -> Self {
         let zero = ResidentResources { bytes: 0, slots: 0, owners: 0 };
+        Self { state: ResidentAccess::new(LedgerState { capacity, data: zero, control: zero, allocated_bytes: 0, head: None, pending: None, prepared: None, release: None, consumers: None, pending_consumer: None, prepared_consumer: None, closing: false, closed: false, #[cfg(test)] consumer_release_interlock: None }) }
-        Self { state: ResidentAccess::new(LedgerState { capacity, data: zero, control: zero, allocated_bytes: 0, head: None, pending: None, prepared: None, release: None, consumers: None, pending_consumer: None, prepared_consumer: None, last_consumer_registration: 0, primary: None, recovery: None, closing: false, closed: false, #[cfg(test)] consumer_release_interlock: None }) }
     }
 
     pub fn ledger(&self) -> ResidentLedger<'_> { ResidentLedger { root: self } }
 
     pub fn native_layout<C, S>(&self) -> ResidentNativeLayout {
+        ResidentNativeLayout { root_bytes: size_of::<Self>() as u64, admission_page_bytes: size_of::<AdmissionNode>() as u64, consumer_page_bytes: size_of::<ConsumerNode<C>>() as u64, record_page_bytes: size_of::<RecordNode<S>>() as u64, consumer_move_bytes: size_of::<Option<C>>() as u64, shell_move_bytes: size_of::<Option<S>>() as u64, descriptor_move_bytes: size_of::<ErasedRecord>().max(size_of::<Option<AdmissionPage>>()).max(size_of::<ConsumerPage>()).max(size_of::<Option<ResidentRelease>>()) as u64, final_root_bytes: size_of::<Self>() as u64, release_slot_bytes: size_of::<Option<ResidentRelease>>() as u64, pending_consumer_bytes: size_of::<Option<ConsumerPage>>() as u64 }
-        ResidentNativeLayout { root_bytes: size_of::<Self>() as u64, admission_page_bytes: size_of::<AdmissionNode>() as u64, consumer_page_bytes: size_of::<ConsumerNode<C>>() as u64, record_page_bytes: size_of::<RecordNode<S>>() as u64, consumer_move_bytes: size_of::<Option<C>>() as u64, shell_move_bytes: size_of::<Option<S>>() as u64, descriptor_move_bytes: size_of::<ErasedRecord>().max(size_of::<Option<AdmissionPage>>()).max(size_of::<ConsumerPage>()).max(size_of::<Option<ResidentRelease>>()).max(size_of::<Option<ResidentPrimaryAnchor>>()).max(size_of::<Option<ResidentRecoveryCursor>>()) as u64, final_root_bytes: size_of::<Self>() as u64, release_slot_bytes: size_of::<Option<ResidentRelease>>() as u64, pending_consumer_bytes: size_of::<Option<ConsumerPage>>() as u64 }
     }
 
     pub fn allocated_bytes(&self) -> Result<u64, ResidentFault> { self.access()?.map(|state| state.allocated_bytes).ok_or(ResidentFault::Busy) }
@@ -267,7 +246,6 @@
             return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
         }
         if state.release.is_some() { return state.advance_release(grant); }
-        if state.recovery.is_some() { return state.close_recovery(grant); }
         if let Some(pending) = state.pending.as_mut() {
             if let Some(consumer) = pending.consumer.as_ref() {
                 if !consumer.is_empty() { return Ok(ResidentStep::blocked()); }
@@ -319,9 +297,6 @@
             state.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::Admission, partition, charge, stage });
             return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
         }
-        if state.primary.as_ref().is_some_and(|anchor| matches!(&anchor.backing, ResidentPrimaryBacking::Pending(_))) {
-            return state.detach_pending_primary(grant);
-        }
         if let Some(page) = state.pending_consumer.as_ref() {
             if page.initialized && page.pointer.is_none_or(|pointer| !unsafe { (page.empty)(pointer) }) { return Ok(ResidentStep::blocked()); }
             let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>()])?;
@@ -347,18 +322,15 @@
                 interlock.observed.try_send(()).map_err(|_| ResidentFault::Identity)?;
                 interlock.resume.recv_timeout(std::time::Duration::from_secs(1)).map_err(|_| ResidentFault::Identity)?;
             }
+            if header.aliases.load(Ordering::Acquire) != 0 || header.admissions.load(Ordering::Acquire) != 0 { return Ok(ResidentStep::blocked()); }
-            if header.aliases.load(Ordering::Acquire) != 0 || header.admissions.load(Ordering::Acquire) != 0 || header.recovery_pins.load(Ordering::Acquire) != 0 { return Ok(ResidentStep::blocked()); }
             let clear_prepared = state.prepared_consumer == Some(pointer);
+            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>(), if clear_prepared { size_of::<Option<NonNull<ConsumerHeader>>>() } else { 0 }])?;
-            let primary = state.primary_for_page(page)?;
-            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ResidentRelease>>(), if clear_prepared { size_of::<Option<NonNull<ConsumerHeader>>>() } else { 0 }, if primary.is_some() { size_of::<ResidentPrimaryBacking>() } else { 0 }])?;
             if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-            if primary.is_some() { state.primary.as_mut().unwrap().backing = ResidentPrimaryBacking::Releasing; }
             let page = state.consumers.take().unwrap();
             state.consumers = unsafe { (&mut *header.next.get()).take() };
             if clear_prepared { state.prepared_consumer = None; }
             let stage = ResidentReleaseStage::allocated(pointer.cast(), page.layout, true, page.destroy_empty);
+            state.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::Consumer, partition: page.partition, charge: page.charge, stage });
-            let origin = primary.map_or(ResidentReleaseOrigin::Consumer, |registration| ResidentReleaseOrigin::PrimaryConsumer { registration });
-            state.release = Some(ResidentRelease { origin, partition: page.partition, charge: page.charge, stage });
             return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
         }
         if !state.structurally_empty() { return Err(ResidentFault::Capacity); }
@@ -370,84 +342,16 @@
 }
 
 impl LedgerState {
-    fn primary_for_page(&self, page: &ConsumerPage) -> Result<Option<NonZeroU64>, ResidentFault> {
-        let Some(anchor) = self.primary.as_ref() else { return Ok(None); };
-        let ResidentPrimaryBacking::Published(pointer) = &anchor.backing else { return Ok(None); };
-        let same_pointer = page.pointer == Some(*pointer);
-        let same_registration = page.registration == anchor.stamp.generation;
-        if same_pointer != same_registration { return Err(ResidentFault::Identity); }
-        if !same_pointer { return Ok(None); }
-        if page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
-        Ok(Some(page.registration))
-    }
-
-    fn detach_pending_primary(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let anchor = self.primary.as_mut().ok_or(ResidentFault::Identity)?;
-        let ResidentPrimaryBacking::Pending(page) = &anchor.backing else { return Ok(ResidentStep::rejected()); };
-        if page.registration != anchor.stamp.generation || page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
-        let bytes = resident_release_work(&[size_of::<ResidentPrimaryBacking>(), size_of::<Option<ResidentRelease>>()])?;
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        if page.initialized && page.pointer.is_none_or(|pointer| !unsafe { (page.empty)(pointer) }) { return Ok(ResidentStep::blocked()); }
-        let ResidentPrimaryBacking::Pending(page) = std::mem::replace(&mut anchor.backing, ResidentPrimaryBacking::Releasing) else { unreachable!() };
-        let stage = match page.pointer {
-            Some(pointer) => ResidentReleaseStage::allocated(pointer.cast(), page.layout, page.initialized, page.destroy_empty),
-            None => ResidentReleaseStage::Refund { released_layout: None },
-        };
-        self.release = Some(ResidentRelease { origin: ResidentReleaseOrigin::PrimaryPending { registration: page.registration }, partition: page.partition, charge: page.charge, stage });
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
-    }
-
-    fn close_recovery(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let cursor = self.recovery.as_mut().ok_or(ResidentFault::Identity)?;
-        if !cursor.revoked {
-            let bytes = size_of::<bool>() as u64;
-            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-            cursor.revoked = true;
-            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
-        }
-        let slot = if cursor.next.is_some() { &mut cursor.next } else { &mut cursor.found };
-        if let Some(pin) = slot.as_ref() {
-            let bytes = resident_release_work(&[size_of::<Option<ResidentRecoveryPin>>(), size_of::<AtomicUsize>()])?;
-            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-            #[cfg(test)]
-            tests::observe_primary_recovery_pointer_load(pin.registration.get());
-            let header = unsafe { pin.pointer.as_ref() };
-            let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
-            header.recovery_pins.store(remaining, Ordering::Release);
-            *slot = None;
-            return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
-        }
-        let bytes = size_of::<Option<ResidentRecoveryCursor>>() as u64;
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        self.recovery = None;
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
-    }
-
-    fn primary_recovery_target<C: Send + 'static>(&self, mode: ResidentRecoveryMode) -> Result<Option<(ResidentRegistrationStamp, NonNull<ConsumerHeader>)>, ResidentFault> {
-        if self.closed || self.closing != (mode == ResidentRecoveryMode::Closing) { return Err(ResidentFault::Closed); }
-        let Some(anchor) = self.primary.as_ref() else { return Ok(None); };
-        if anchor.stamp.type_id != TypeId::of::<C>() { return Err(ResidentFault::Identity); }
-        match &anchor.backing { ResidentPrimaryBacking::Published(pointer) => Ok(Some((anchor.stamp, *pointer))), _ => Ok(None) }
-    }
-
-    fn pointerless_close_allowed(&self) -> bool {
-        self.release.as_ref().is_some_and(|release| release.stage.pointerless()) && self.recovery.is_none() && self.primary.as_ref().is_none_or(|anchor| matches!(&anchor.backing, ResidentPrimaryBacking::Releasing))
-    }
-
     fn structurally_empty(&self) -> bool {
         #[cfg(test)]
         if self.consumer_release_interlock.is_some() { return false; }
         let zero = ResidentResources { bytes: 0, slots: 0, owners: 0 };
+        self.head.is_none() && self.pending.is_none() && self.prepared.is_none() && self.release.is_none() && self.consumers.is_none() && self.pending_consumer.is_none() && self.prepared_consumer.is_none() && self.allocated_bytes == 0 && self.data == zero && self.control == zero
-        self.head.is_none() && self.pending.is_none() && self.prepared.is_none() && self.release.is_none() && self.consumers.is_none() && self.pending_consumer.is_none() && self.prepared_consumer.is_none() && self.primary.is_none() && self.recovery.is_none() && self.allocated_bytes == 0 && self.data == zero && self.control == zero
     }
 
     fn advance_release(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
         let release = self.release.as_mut().ok_or(ResidentFault::Identity)?;
         let slot = size_of::<Option<ResidentRelease>>();
-        let primary = match &release.origin { ResidentReleaseOrigin::PrimaryPending { registration } | ResidentReleaseOrigin::PrimaryConsumer { registration } => Some(*registration), _ => None };
-        if let Some(registration) = primary {
-            if self.primary.as_ref().is_none_or(|anchor| anchor.stamp.generation != registration || anchor.partition != release.partition || !matches!(&anchor.backing, ResidentPrimaryBacking::Releasing)) { return Err(ResidentFault::Identity); }
-        }
         match &release.stage {
             ResidentReleaseStage::Destroy { allocation, destroy_empty } => {
                 let bytes = resident_release_work(&[allocation.layout.size(), slot])?;
@@ -480,9 +384,8 @@
                 Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
             }
             ResidentReleaseStage::Clear { .. } => {
+                let bytes = slot as u64;
-                let bytes = resident_release_work(&[slot, if primary.is_some() { size_of::<Option<ResidentPrimaryAnchor>>() } else { 0 }])?;
                 if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-                if primary.is_some() { self.primary = None; }
                 self.release = None;
                 Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
             }
@@ -510,11 +413,9 @@
 struct ConsumerHeader {
     aliases: AtomicUsize,
     admissions: AtomicUsize,
-    recovery_pins: AtomicUsize,
     closing: UnsafeCell<bool>,
     next: UnsafeCell<Option<ConsumerPage>>,
     type_id: TypeId,
-    registration: NonZeroU64,
 }
 
 #[repr(C)]
@@ -527,46 +428,17 @@
     charge: ResidentResources,
     initialized: bool,
     type_id: TypeId,
+    initialize: unsafe fn(NonNull<ConsumerHeader>),
-    registration: NonZeroU64,
-    initialize: unsafe fn(NonNull<ConsumerHeader>, NonZeroU64),
     empty: unsafe fn(NonNull<ConsumerHeader>) -> bool,
     destroy_empty: unsafe fn(NonNull<u8>),
 }
 
-/// 🧷️ Typed constructors bind Send sources and exact layouts; pointers remain in the original gated root.
 unsafe impl Send for ConsumerPage {}
 
+unsafe fn initialize_consumer<C: Send + 'static>(pointer: NonNull<ConsumerHeader>) {
+    unsafe { pointer.cast::<ConsumerNode<C>>().as_ptr().write(ConsumerNode { header: ConsumerHeader { aliases: AtomicUsize::new(0), admissions: AtomicUsize::new(0), closing: UnsafeCell::new(false), next: UnsafeCell::new(None), type_id: TypeId::of::<C>() }, source: UnsafeCell::new(None) }); }
-impl ConsumerPage {
-    fn reserved<C: Send + 'static>(partition: ResidentPartition, registration: NonZeroU64) -> Result<Self, ResidentFault> {
-        let layout = Layout::new::<ConsumerNode<C>>();
-        let charge = ResidentResources::new(layout.size() as u64, 1, 1)?;
-        Ok(Self { pointer: None, layout, partition, charge, initialized: false, type_id: TypeId::of::<C>(), registration, initialize: initialize_consumer::<C>, empty: empty_consumer::<C>, destroy_empty: destroy_consumer::<C> })
-    }
-
-    fn allocate(&mut self, allocated_bytes: &mut u64, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let bytes = resident_release_work(&[self.layout.size(), size_of::<Option<NonNull<ConsumerHeader>>>(), size_of::<u64>()])?;
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        let allocated = allocated_bytes.checked_add(self.layout.size() as u64).ok_or(ResidentFault::Count)?;
-        let pointer = NonNull::new(unsafe { std::alloc::alloc(self.layout) }.cast::<ConsumerHeader>()).ok_or(ResidentFault::Allocation)?;
-        self.pointer = Some(pointer);
-        *allocated_bytes = allocated;
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
-    }
-
-    fn initialize(&mut self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let bytes = resident_release_work(&[self.layout.size(), size_of::<bool>()])?;
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        let pointer = self.pointer.ok_or(ResidentFault::Identity)?;
-        unsafe { (self.initialize)(pointer, self.registration); }
-        self.initialized = true;
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
-    }
 }
 
-unsafe fn initialize_consumer<C: Send + 'static>(pointer: NonNull<ConsumerHeader>, registration: NonZeroU64) {
-    unsafe { pointer.cast::<ConsumerNode<C>>().as_ptr().write(ConsumerNode { header: ConsumerHeader { aliases: AtomicUsize::new(0), admissions: AtomicUsize::new(0), recovery_pins: AtomicUsize::new(0), closing: UnsafeCell::new(false), next: UnsafeCell::new(None), type_id: TypeId::of::<C>(), registration }, source: UnsafeCell::new(None) }); }
-}
-
 unsafe fn empty_consumer<C>(pointer: NonNull<ConsumerHeader>) -> bool { unsafe { (&*pointer.cast::<ConsumerNode<C>>().as_ref().source.get()).is_none() } }
 unsafe fn destroy_consumer<C>(pointer: NonNull<u8>) { unsafe { pointer.cast::<ConsumerNode<C>>().as_ptr().drop_in_place(); } }
 
@@ -639,160 +511,38 @@
 impl ResidentLedgerRoot {
     pub fn prepare_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
         let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
+        if state.closing { return Ok(ResidentStep::rejected()); }
+        let layout = Layout::new::<ConsumerNode<C>>();
+        let bytes = layout.size() as u64;
+        if !grant.admits(bytes.max(size_of::<ConsumerPage>() as u64)) { return Ok(ResidentStep::blocked()); }
-        let state = &mut *state;
-        if state.closing || state.closed { return Ok(ResidentStep::rejected()); }
         if let Some(page) = state.pending_consumer.as_mut() {
             if page.type_id != TypeId::of::<C>() || page.partition != partition { return Ok(ResidentStep::rejected()); }
+            if page.pointer.is_none() {
+                let allocated = state.allocated_bytes.checked_add(bytes).ok_or(ResidentFault::Count)?;
+                let pointer = NonNull::new(unsafe { std::alloc::alloc(layout) }.cast::<ConsumerHeader>()).ok_or(ResidentFault::Allocation)?;
+                state.pending_consumer.as_mut().unwrap().pointer = Some(pointer);
+                state.allocated_bytes = allocated;
+                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
+            }
+            if !page.initialized {
+                unsafe { (page.initialize)(page.pointer.unwrap()); }
+                page.initialized = true;
+                return Ok(ResidentStep::done(ResidentStepKind::Pending, bytes));
+            }
-            if page.pointer.is_none() { return page.allocate(&mut state.allocated_bytes, grant); }
-            if !page.initialized { return page.initialize(grant); }
-            let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<NonNull<ConsumerHeader>>>()])?;
-            if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
             let page = state.pending_consumer.take().unwrap();
             let pointer = page.pointer.unwrap();
             unsafe { *pointer.as_ref().next.get() = state.consumers.take(); }
             state.prepared_consumer = Some(pointer);
             state.consumers = Some(page);
+            return Ok(ResidentStep::done(ResidentStepKind::Ready, size_of::<ConsumerPage>() as u64));
-            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
         }
+        let charge = ResidentResources { bytes, slots: 1, owners: 1 };
+        if state.reserve(partition, charge).is_err() { return Ok(ResidentStep::blocked()); }
+        state.pending_consumer = Some(ConsumerPage { pointer: None, layout, partition, charge, initialized: false, type_id: TypeId::of::<C>(), initialize: initialize_consumer::<C>, empty: empty_consumer::<C>, destroy_empty: destroy_consumer::<C> });
-        let bytes = resident_release_work(&[size_of::<Option<ConsumerPage>>(), size_of::<u64>(), size_of::<ResidentResources>(), size_of::<Option<NonNull<ConsumerHeader>>>()])?;
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        let registration = NonZeroU64::new(state.last_consumer_registration.checked_add(1).ok_or(ResidentFault::Count)?).ok_or(ResidentFault::Count)?;
-        let page = ConsumerPage::reserved::<C>(partition, registration)?;
-        if state.reserve(partition, page.charge).is_err() { return Ok(ResidentStep::blocked()); }
-        state.last_consumer_registration = registration.get();
-        state.pending_consumer = Some(page);
         state.prepared_consumer = None;
+        Ok(ResidentStep::done(ResidentStepKind::Pending, size_of::<ConsumerPage>() as u64))
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
     }
 
-    pub fn reserve_primary_consumer<C: Send + 'static>(&self, partition: ResidentPartition, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
-        if state.closing || state.closed || state.primary.is_some() { return Ok(ResidentStep::rejected()); }
-        let bytes = resident_release_work(&[size_of::<Option<ResidentPrimaryAnchor>>(), size_of::<u64>(), size_of::<ResidentResources>()])?;
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        let registration = NonZeroU64::new(state.last_consumer_registration.checked_add(1).ok_or(ResidentFault::Count)?).ok_or(ResidentFault::Count)?;
-        let page = ConsumerPage::reserved::<C>(partition, registration)?;
-        if state.reserve(partition, page.charge).is_err() { return Ok(ResidentStep::blocked()); }
-        state.last_consumer_registration = registration.get();
-        state.primary = Some(ResidentPrimaryAnchor { stamp: ResidentRegistrationStamp { generation: registration, type_id: TypeId::of::<C>() }, partition, backing: ResidentPrimaryBacking::Pending(page) });
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
-    }
-
-    pub fn prepare_primary_consumer<C: Send + 'static>(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
-        let state = &mut *state;
-        if state.closing || state.closed { return Ok(ResidentStep::rejected()); }
-        let Some(anchor) = state.primary.as_mut() else { return Ok(ResidentStep::rejected()); };
-        if anchor.stamp.type_id != TypeId::of::<C>() { return Err(ResidentFault::Identity); }
-        let ResidentPrimaryBacking::Pending(page) = &mut anchor.backing else { return Ok(ResidentStep::rejected()); };
-        if page.registration != anchor.stamp.generation || page.type_id != anchor.stamp.type_id || page.partition != anchor.partition { return Err(ResidentFault::Identity); }
-        if page.pointer.is_none() { return page.allocate(&mut state.allocated_bytes, grant); }
-        if !page.initialized { return page.initialize(grant); }
-        let bytes = resident_release_work(&[size_of::<ResidentPrimaryBacking>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>(), size_of::<Option<ConsumerPage>>()])?;
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        let pointer = page.pointer.unwrap();
-        let ResidentPrimaryBacking::Pending(page) = std::mem::replace(&mut anchor.backing, ResidentPrimaryBacking::Published(pointer)) else { unreachable!() };
-        unsafe { *pointer.as_ref().next.get() = state.consumers.take(); }
-        state.consumers = Some(page);
-        Ok(ResidentStep::done(ResidentStepKind::Ready, bytes))
-    }
-
-    pub fn begin_primary_recovery<C: Send + 'static>(&self, mode: ResidentRecoveryMode, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
-        let Some((stamp, _)) = state.primary_recovery_target::<C>(mode)? else { return Ok(ResidentStep::rejected()); };
-        if state.recovery.is_some() { return Ok(ResidentStep::rejected()); }
-        let bytes = resident_release_work(&[size_of::<Option<ResidentRecoveryCursor>>(), size_of::<AtomicUsize>()])?;
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        let page = state.consumers.as_ref().ok_or(ResidentFault::Identity)?;
-        if !page.initialized { return Err(ResidentFault::Identity); }
-        let pointer = page.pointer.ok_or(ResidentFault::Identity)?;
-        #[cfg(test)]
-        tests::observe_primary_recovery_pointer_load(page.registration.get());
-        let header = unsafe { pointer.as_ref() };
-        let count = header.recovery_pins.load(Ordering::Acquire).checked_add(1).ok_or(ResidentFault::Count)?;
-        let registration = page.registration;
-        header.recovery_pins.store(count, Ordering::Release);
-        state.recovery = Some(ResidentRecoveryCursor { stamp, mode, revoked: false, next: Some(ResidentRecoveryPin { pointer, registration }), found: None });
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
-    }
-
-    pub fn advance_primary_recovery<C: Send + 'static>(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
-        if state.closed { return Err(ResidentFault::Closed); }
-        let Some(cursor) = state.recovery.as_ref() else { return Ok(ResidentStep::rejected()); };
-        if cursor.revoked { return Err(ResidentFault::Closed); }
-        let Some((stamp, original)) = state.primary_recovery_target::<C>(cursor.mode)? else { return Ok(ResidentStep::rejected()); };
-        if cursor.stamp != stamp || cursor.found.is_some() { return Ok(ResidentStep::rejected()); }
-        let Some(pin) = cursor.next.as_ref() else { return Ok(ResidentStep::rejected()); };
-        let matching = pin.registration == stamp.generation;
-        if matching && pin.pointer != original { return Err(ResidentFault::Identity); }
-        let bytes = if matching {
-            resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<Option<ResidentRecoveryPin>>(), size_of::<Option<ResidentRecoveryPin>>()])?
-        } else {
-            resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<AtomicUsize>(), size_of::<AtomicUsize>(), size_of::<Option<ResidentRecoveryPin>>(), size_of::<Option<ResidentRecoveryPin>>()])?
-        };
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        #[cfg(test)]
-        tests::observe_primary_recovery_pointer_load(pin.registration.get());
-        let header = unsafe { pin.pointer.as_ref() };
-        if header.registration != pin.registration { return Err(ResidentFault::Identity); }
-        let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
-        if matching {
-            if header.type_id != stamp.type_id || (cursor.mode == ResidentRecoveryMode::Forward && unsafe { *header.closing.get() }) { return Err(ResidentFault::Identity); }
-            let cursor = state.recovery.as_mut().unwrap();
-            cursor.found = cursor.next.take();
-            return Ok(ResidentStep::done(ResidentStepKind::Ready, bytes));
-        }
-        let successor = unsafe { (&*header.next.get()).as_ref() }.ok_or(ResidentFault::Identity)?;
-        let pointer = successor.pointer.ok_or(ResidentFault::Identity)?;
-        let registration = successor.registration;
-        if !successor.initialized || pointer == pin.pointer || registration == pin.registration { return Err(ResidentFault::Identity); }
-        #[cfg(test)]
-        tests::observe_primary_recovery_pointer_load(registration.get());
-        let next = unsafe { pointer.as_ref() };
-        let count = next.recovery_pins.load(Ordering::Acquire).checked_add(1).ok_or(ResidentFault::Count)?;
-        next.recovery_pins.store(count, Ordering::Release);
-        header.recovery_pins.store(remaining, Ordering::Release);
-        state.recovery.as_mut().unwrap().next = Some(ResidentRecoveryPin { pointer, registration });
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
-    }
-
-    pub fn capture_primary_consumer<C: Send + 'static>(&self, mode: ResidentRecoveryMode, grant: ResidentGrant) -> Result<(ResidentStep, Option<ResidentConsumer<'_, C>>), ResidentFault> {
-        let Some(mut state) = self.access()? else { return Ok((ResidentStep::blocked(), None)); };
-        let Some((stamp, original)) = state.primary_recovery_target::<C>(mode)? else { return Ok((ResidentStep::rejected(), None)); };
-        let Some(cursor) = state.recovery.as_ref() else { return Ok((ResidentStep::rejected(), None)); };
-        if cursor.revoked { return Err(ResidentFault::Closed); }
-        if cursor.mode != mode || cursor.stamp != stamp || cursor.next.is_some() { return Ok((ResidentStep::rejected(), None)); }
-        let Some(pin) = cursor.found.as_ref() else { return Ok((ResidentStep::rejected(), None)); };
-        if pin.registration != stamp.generation || pin.pointer != original { return Err(ResidentFault::Identity); }
-        let bytes = resident_release_work(&[size_of::<ConsumerHeader>(), size_of::<AtomicUsize>(), size_of::<AtomicUsize>(), size_of::<Option<ResidentRecoveryCursor>>(), size_of::<ResidentConsumer<'_, C>>()])?;
-        if !grant.admits(bytes) { return Ok((ResidentStep::blocked(), None)); }
-        #[cfg(test)]
-        tests::observe_primary_recovery_pointer_load(pin.registration.get());
-        let header = unsafe { pin.pointer.as_ref() };
-        if header.registration != stamp.generation || header.type_id != stamp.type_id { return Err(ResidentFault::Identity); }
-        if mode == ResidentRecoveryMode::Forward && unsafe { *header.closing.get() } { return Err(ResidentFault::Closed); }
-        let remaining = header.recovery_pins.load(Ordering::Acquire).checked_sub(1).ok_or(ResidentFault::Count)?;
-        let aliases = header.aliases.load(Ordering::Acquire);
-        let count = aliases.checked_add(1).ok_or(ResidentFault::Count)?;
-        header.aliases.compare_exchange(aliases, count, Ordering::AcqRel, Ordering::Acquire).map_err(|_| ResidentFault::Busy)?;
-        header.recovery_pins.store(remaining, Ordering::Release);
-        state.recovery = None;
-        Ok((ResidentStep::done(ResidentStepKind::Ready, bytes), Some(ResidentConsumer { root: self, pointer: original.cast() })))
-    }
-
-    pub fn begin_primary_consumer_close(&self, grant: ResidentGrant) -> Result<ResidentStep, ResidentFault> {
-        let Some(mut state) = self.access()? else { return Ok(ResidentStep::blocked()); };
-        if state.closed || state.primary.is_none() { return Ok(ResidentStep::rejected()); }
-        let revoke = state.recovery.as_ref().is_some_and(|cursor| !cursor.revoked);
-        let bytes = resident_release_work(&[if state.closing { 0 } else { size_of::<bool>() }, if revoke { size_of::<bool>() } else { 0 }])?;
-        if bytes == 0 { return Ok(ResidentStep { kind: ResidentStepKind::Pending, items: 0, bytes: 0 }); }
-        if !grant.admits(bytes) { return Ok(ResidentStep::blocked()); }
-        if !state.closing { state.closing = true; }
-        if revoke { state.recovery.as_mut().unwrap().revoked = true; }
-        Ok(ResidentStep::done(ResidentStepKind::Pending, bytes))
-    }
-
     pub fn prepared_consumer<C: Send + 'static>(&self) -> Result<Option<ResidentConsumer<'_, C>>, ResidentFault> {
         self.consumer_access(false)
     }
@@ -847,7 +597,6 @@
 
 struct PendingAdmission { page: Option<AdmissionPage>, consumer: Option<ErasedConsumer>, partition: ResidentPartition, charge: ResidentResources }
 
-/// 🪢️ Send-typed nodes stay in this gated root/list/Release; cursor pointers have counted same-root pins and captures acquire an alias before releasing a pin.
 unsafe impl Send for LedgerState {}
 
 pub struct ResidentAdmission<'root, C> { root: &'root ResidentLedgerRoot, node: NonNull<AdmissionNode>, marker: PhantomData<fn() -> C> }
```

## .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs

### Forward

```diff
--- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs
+++ .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs
@@ -679,9 +679,9 @@
     })();
     let mut value = None; let capacity_closed = cleanup::<u64>(&full_root, &mut value);
     assert_eq!(capacity_closed, Ok(true));
-    let (before, after, primary, ordinary, pe, oe) = capacity_observation?;
+    let (before, after, primary, ordinary_refusal, pe, oe) = capacity_observation?;
     assert_eq!(after, before); assert!(before.prepared.is_some());
-    for result in [primary, ordinary] { assert!(matches!(result, Ok(ResidentStep { kind: ResidentStepKind::Blocked, items: 0, bytes: 0 }))); }
+    for result in [primary, ordinary_refusal] { assert!(matches!(result, Ok(ResidentStep { kind: ResidentStepKind::Blocked, items: 0, bytes: 0 }))); }
     assert_eq!((pe.count, oe.count), (0, 0));
     let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).map_err(|_| ResidentFault::Identity)?;
     let maximum = fixture["registrationExhaustion"]["maximum"].as_str().ok_or(ResidentFault::Identity)?.parse::<u64>().map_err(|_| ResidentFault::Count)?;
```

### Inverse

```diff
--- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs
+++ .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs
@@ -679,9 +679,9 @@
     })();
     let mut value = None; let capacity_closed = cleanup::<u64>(&full_root, &mut value);
     assert_eq!(capacity_closed, Ok(true));
+    let (before, after, primary, ordinary, pe, oe) = capacity_observation?;
-    let (before, after, primary, ordinary_refusal, pe, oe) = capacity_observation?;
     assert_eq!(after, before); assert!(before.prepared.is_some());
+    for result in [primary, ordinary] { assert!(matches!(result, Ok(ResidentStep { kind: ResidentStepKind::Blocked, items: 0, bytes: 0 }))); }
-    for result in [primary, ordinary_refusal] { assert!(matches!(result, Ok(ResidentStep { kind: ResidentStepKind::Blocked, items: 0, bytes: 0 }))); }
     assert_eq!((pe.count, oe.count), (0, 0));
     let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).map_err(|_| ResidentFault::Identity)?;
     let maximum = fixture["registrationExhaustion"]["maximum"].as_str().ok_or(ResidentFault::Identity)?.parse::<u64>().map_err(|_| ResidentFault::Count)?;
```

## 🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs

### Forward

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
@@ -552,18 +552,32 @@
     let root = ResidentLedgerRoot::new(admission_capacity()); let grant = full_admission_grant();
     let layouts = [std::alloc::Layout::new::<super::ConsumerNode<AlignedResident>>(), std::alloc::Layout::new::<super::AdmissionNode>(), std::alloc::Layout::new::<super::RecordNode<AlignedResident>>()];
     let envelope = ResidentResources::new(64, 1, 1).unwrap();
+    let reserve_work = [std::mem::size_of::<Option<super::ConsumerPage>>(), std::mem::size_of::<u64>(), std::mem::size_of::<ResidentResources>(), std::mem::size_of::<Option<std::ptr::NonNull<super::ConsumerHeader>>>()].into_iter().try_fold(0u64, |sum, bytes| sum.checked_add(bytes as u64)).unwrap();
+    let allocate_work = [layouts[0].size(), std::mem::size_of::<Option<std::ptr::NonNull<super::ConsumerHeader>>>(), std::mem::size_of::<u64>()].into_iter().try_fold(0u64, |sum, bytes| sum.checked_add(bytes as u64)).unwrap();
+    assert!(reserve_work > 0 && allocate_work > 0 && reserve_work <= grant.max_bytes() && allocate_work <= grant.max_bytes());
     ALLOCATIONS.with(|value| value.set(0)); DEALLOCATIONS.with(|value| value.set(0));
     ALLOCATION_LAYOUTS.with(|value| value.set([(0, 0, 0); 8])); DEALLOCATION_LAYOUTS.with(|value| value.set([(0, 0); 8]));
     COUNT_ALLOCATIONS.with(|value| value.set(true));
-    let mut refused = [ResidentStepKind::Pending; 4];
-    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[0].size() as u64 - 1)].into_iter().enumerate() { refused[index] = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, short).unwrap().kind; }
-    let short_consumer_allocations = ALLOCATIONS.with(std::cell::Cell::get);
-    for _ in 0..4 { root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap(); }
+    let mut refused = [ResidentStepKind::Pending; 8];
+    let mut short_consumer_allocations = 0;
+    let mut consumer_unchanged = true;
+    let mut exact_consumer_work = true;
+    for (phase, bytes) in [reserve_work, allocate_work].into_iter().enumerate() {
+        let before = (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
+        for (index, short) in [ResidentGrant::new(0, bytes).unwrap(), ResidentGrant::new(1, 0).unwrap(), admission_grant(bytes.checked_sub(1).unwrap())].into_iter().enumerate() {
+            refused[phase * 3 + index] = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, short).unwrap().kind;
+            consumer_unchanged &= before == (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
+            short_consumer_allocations = ALLOCATIONS.with(std::cell::Cell::get);
+        }
+        let exact = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, admission_grant(bytes)).unwrap();
+        exact_consumer_work &= bytes <= grant.max_bytes() && exact.items == 1 && exact.bytes == bytes && exact.kind == ResidentStepKind::Pending;
+    }
+    for _ in 0..2 { root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap(); }
     let consumer = root.prepared_consumer::<AlignedResident>().unwrap().unwrap(); let ledger = root.ledger();
     for _ in 0..3 { ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(); }
     let cell = ledger.prepared_admission(&consumer).unwrap().unwrap(); ledger.claim_admission(&consumer, &cell, grant).unwrap();
     let before_record = ALLOCATIONS.with(std::cell::Cell::get);
-    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[2].size() as u64 - 1)].into_iter().enumerate() { refused[index + 2] = ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, short).unwrap().kind; }
+    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[2].size() as u64 - 1)].into_iter().enumerate() { refused[index + 6] = ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, short).unwrap().kind; }
     let after_short_record = ALLOCATIONS.with(std::cell::Cell::get);
     for _ in 0..2 { ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, grant).unwrap(); }
     let record = cell.record::<AlignedResident>().unwrap().unwrap();
@@ -575,7 +589,7 @@
     let allocations = ALLOCATIONS.with(std::cell::Cell::get); let deallocations = DEALLOCATIONS.with(std::cell::Cell::get);
     let allocation_layouts = ALLOCATION_LAYOUTS.with(std::cell::Cell::get); let deallocation_layouts = DEALLOCATION_LAYOUTS.with(std::cell::Cell::get);
     close_admission_root(&root);
-    assert_eq!(refused, [ResidentStepKind::Blocked; 4]); assert_eq!(short_consumer_allocations, 0); assert_eq!(before_record, after_short_record);
+    assert_eq!(refused, [ResidentStepKind::Blocked; 8]); assert_eq!(short_consumer_allocations, 0); assert!(consumer_unchanged && exact_consumer_work); assert_eq!(before_record, after_short_record);
     assert!(record_aligned); assert_eq!((allocations, deallocations), (3, 3));
     for index in 0..3 { assert_eq!((allocation_layouts[index].1, allocation_layouts[index].2), (layouts[index].size(), layouts[index].align())); }
     for (index, layout) in [layouts[2], layouts[1], layouts[0]].into_iter().enumerate() { assert_eq!(deallocation_layouts[index], (layout.size(), layout.align())); }
```

### Inverse

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
@@ -552,32 +552,18 @@
     let root = ResidentLedgerRoot::new(admission_capacity()); let grant = full_admission_grant();
     let layouts = [std::alloc::Layout::new::<super::ConsumerNode<AlignedResident>>(), std::alloc::Layout::new::<super::AdmissionNode>(), std::alloc::Layout::new::<super::RecordNode<AlignedResident>>()];
     let envelope = ResidentResources::new(64, 1, 1).unwrap();
-    let reserve_work = [std::mem::size_of::<Option<super::ConsumerPage>>(), std::mem::size_of::<u64>(), std::mem::size_of::<ResidentResources>(), std::mem::size_of::<Option<std::ptr::NonNull<super::ConsumerHeader>>>()].into_iter().try_fold(0u64, |sum, bytes| sum.checked_add(bytes as u64)).unwrap();
-    let allocate_work = [layouts[0].size(), std::mem::size_of::<Option<std::ptr::NonNull<super::ConsumerHeader>>>(), std::mem::size_of::<u64>()].into_iter().try_fold(0u64, |sum, bytes| sum.checked_add(bytes as u64)).unwrap();
-    assert!(reserve_work > 0 && allocate_work > 0 && reserve_work <= grant.max_bytes() && allocate_work <= grant.max_bytes());
     ALLOCATIONS.with(|value| value.set(0)); DEALLOCATIONS.with(|value| value.set(0));
     ALLOCATION_LAYOUTS.with(|value| value.set([(0, 0, 0); 8])); DEALLOCATION_LAYOUTS.with(|value| value.set([(0, 0); 8]));
     COUNT_ALLOCATIONS.with(|value| value.set(true));
+    let mut refused = [ResidentStepKind::Pending; 4];
+    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[0].size() as u64 - 1)].into_iter().enumerate() { refused[index] = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, short).unwrap().kind; }
+    let short_consumer_allocations = ALLOCATIONS.with(std::cell::Cell::get);
+    for _ in 0..4 { root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap(); }
-    let mut refused = [ResidentStepKind::Pending; 8];
-    let mut short_consumer_allocations = 0;
-    let mut consumer_unchanged = true;
-    let mut exact_consumer_work = true;
-    for (phase, bytes) in [reserve_work, allocate_work].into_iter().enumerate() {
-        let before = (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
-        for (index, short) in [ResidentGrant::new(0, bytes).unwrap(), ResidentGrant::new(1, 0).unwrap(), admission_grant(bytes.checked_sub(1).unwrap())].into_iter().enumerate() {
-            refused[phase * 3 + index] = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, short).unwrap().kind;
-            consumer_unchanged &= before == (root.usage(ResidentPartition::Data).unwrap(), root.allocated_bytes().unwrap());
-            short_consumer_allocations = ALLOCATIONS.with(std::cell::Cell::get);
-        }
-        let exact = root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, admission_grant(bytes)).unwrap();
-        exact_consumer_work &= bytes <= grant.max_bytes() && exact.items == 1 && exact.bytes == bytes && exact.kind == ResidentStepKind::Pending;
-    }
-    for _ in 0..2 { root.prepare_consumer::<AlignedResident>(ResidentPartition::Data, grant).unwrap(); }
     let consumer = root.prepared_consumer::<AlignedResident>().unwrap().unwrap(); let ledger = root.ledger();
     for _ in 0..3 { ledger.prepare_admission(&consumer, ResidentPartition::Data, grant).unwrap(); }
     let cell = ledger.prepared_admission(&consumer).unwrap().unwrap(); ledger.claim_admission(&consumer, &cell, grant).unwrap();
     let before_record = ALLOCATIONS.with(std::cell::Cell::get);
+    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[2].size() as u64 - 1)].into_iter().enumerate() { refused[index + 2] = ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, short).unwrap().kind; }
-    for (index, short) in [ResidentGrant::new(0, 4096).unwrap(), admission_grant(layouts[2].size() as u64 - 1)].into_iter().enumerate() { refused[index + 6] = ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, short).unwrap().kind; }
     let after_short_record = ALLOCATIONS.with(std::cell::Cell::get);
     for _ in 0..2 { ledger.reserve_record::<AlignedResident, AlignedResident>(&cell, envelope, grant).unwrap(); }
     let record = cell.record::<AlignedResident>().unwrap().unwrap();
@@ -589,7 +575,7 @@
     let allocations = ALLOCATIONS.with(std::cell::Cell::get); let deallocations = DEALLOCATIONS.with(std::cell::Cell::get);
     let allocation_layouts = ALLOCATION_LAYOUTS.with(std::cell::Cell::get); let deallocation_layouts = DEALLOCATION_LAYOUTS.with(std::cell::Cell::get);
     close_admission_root(&root);
+    assert_eq!(refused, [ResidentStepKind::Blocked; 4]); assert_eq!(short_consumer_allocations, 0); assert_eq!(before_record, after_short_record);
-    assert_eq!(refused, [ResidentStepKind::Blocked; 8]); assert_eq!(short_consumer_allocations, 0); assert!(consumer_unchanged && exact_consumer_work); assert_eq!(before_record, after_short_record);
     assert!(record_aligned); assert_eq!((allocations, deallocations), (3, 3));
     for index in 0..3 { assert_eq!((allocation_layouts[index].1, allocation_layouts[index].2), (layouts[index].size(), layouts[index].align())); }
     for (index, layout) in [layouts[2], layouts[1], layouts[0]].into_iter().enumerate() { assert_eq!(deallocation_layouts[index], (layout.size(), layout.align())); }
```

## 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json

### Forward

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
@@ -17,6 +17,8 @@
     ],
     "exactAllocation": { "alignment": 64, "failure": "injected-null", "consumerFailure": "retain-reservation", "admissionFailure": "retain-reservation", "recordFailure": "release-uninstalled-reservation", "cancelFrontiers": ["reserved", "allocated", "initialized", "published"], "cancelAllocations": [0, 1, 1, 1], "cancelDeallocations": [0, 1, 1, 1], "deallocation": "same-requested-layout-once" },
     "grantCases": ["zero-items", "required-minus-one", "required"],
+    "preparationGrantFrontiers": [{"phase":"reserve","required":"Option<ConsumerPage>+u64+ResidentResources+Option<NonNull<ConsumerHeader>>","before":{"reserved":false,"allocations":0},"afterRefusal":{"reserved":false,"allocations":0},"afterExact":{"reserved":true,"allocations":0}},{"phase":"allocate","required":"ConsumerNode<C>.layout+Option<NonNull<ConsumerHeader>>+u64","before":{"reserved":true,"allocations":0},"afterRefusal":{"reserved":true,"allocations":0},"afterExact":{"reserved":true,"allocations":1}}],
+    "preparationGrantCases": ["zero-items", "zero-bytes", "required-minus-one", "required"],
     "releaseTrace": [
       { "phase": "prepared", "consumerDrops": 0, "shellDrops": 0, "terminal": false },
       { "phase": "caller-lost", "consumerDrops": 0, "shellDrops": 0, "terminal": false },
```

### Inverse

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️fixture.json
@@ -17,8 +17,6 @@
     ],
     "exactAllocation": { "alignment": 64, "failure": "injected-null", "consumerFailure": "retain-reservation", "admissionFailure": "retain-reservation", "recordFailure": "release-uninstalled-reservation", "cancelFrontiers": ["reserved", "allocated", "initialized", "published"], "cancelAllocations": [0, 1, 1, 1], "cancelDeallocations": [0, 1, 1, 1], "deallocation": "same-requested-layout-once" },
     "grantCases": ["zero-items", "required-minus-one", "required"],
-    "preparationGrantFrontiers": [{"phase":"reserve","required":"Option<ConsumerPage>+u64+ResidentResources+Option<NonNull<ConsumerHeader>>","before":{"reserved":false,"allocations":0},"afterRefusal":{"reserved":false,"allocations":0},"afterExact":{"reserved":true,"allocations":0}},{"phase":"allocate","required":"ConsumerNode<C>.layout+Option<NonNull<ConsumerHeader>>+u64","before":{"reserved":true,"allocations":0},"afterRefusal":{"reserved":true,"allocations":0},"afterExact":{"reserved":true,"allocations":1}}],
-    "preparationGrantCases": ["zero-items", "zero-bytes", "required-minus-one", "required"],
     "releaseTrace": [
       { "phase": "prepared", "consumerDrops": 0, "shellDrops": 0, "terminal": false },
       { "phase": "caller-lost", "consumerDrops": 0, "shellDrops": 0, "terminal": false },
```

## 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json

### Forward

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json
@@ -15,13 +15,15 @@
     "unrelatedByte": { "const": 73 },
     "nativeOwnership": {
       "type": "object", "additionalProperties": false,
-      "required": ["firstAccessAllocations", "foreignRepopulation", "phaseAccess", "exactAllocation", "grantCases", "releaseTrace", "refusals", "layout", "unknownFaultFinalDisposal"],
+      "required": ["firstAccessAllocations", "foreignRepopulation", "phaseAccess", "exactAllocation", "grantCases", "preparationGrantFrontiers", "preparationGrantCases", "releaseTrace", "refusals", "layout", "unknownFaultFinalDisposal"],
       "properties": {
         "firstAccessAllocations": { "const": [0, 0, 0] },
         "foreignRepopulation": { "type": "object", "additionalProperties": false, "required": ["afterEmptyObservation", "accepted", "consumerDropsDuringRelease"], "properties": { "afterEmptyObservation": { "const": true }, "accepted": { "const": false }, "consumerDropsDuringRelease": { "const": 0 } } },
         "phaseAccess": { "const": [{ "phase": "open", "forward": true, "recovery": false }, { "phase": "consumer-closing", "forward": false, "recovery": true }, { "phase": "root-closing", "forward": false, "recovery": true }] },
         "exactAllocation": { "const": { "alignment": 64, "failure": "injected-null", "consumerFailure": "retain-reservation", "admissionFailure": "retain-reservation", "recordFailure": "release-uninstalled-reservation", "cancelFrontiers": ["reserved", "allocated", "initialized", "published"], "cancelAllocations": [0, 1, 1, 1], "cancelDeallocations": [0, 1, 1, 1], "deallocation": "same-requested-layout-once" } },
         "grantCases": { "const": ["zero-items", "required-minus-one", "required"] },
+        "preparationGrantFrontiers": { "const": [{"phase":"reserve","required":"Option<ConsumerPage>+u64+ResidentResources+Option<NonNull<ConsumerHeader>>","before":{"reserved":false,"allocations":0},"afterRefusal":{"reserved":false,"allocations":0},"afterExact":{"reserved":true,"allocations":0}},{"phase":"allocate","required":"ConsumerNode<C>.layout+Option<NonNull<ConsumerHeader>>+u64","before":{"reserved":true,"allocations":0},"afterRefusal":{"reserved":true,"allocations":0},"afterExact":{"reserved":true,"allocations":1}}] },
+        "preparationGrantCases": { "const": ["zero-items", "zero-bytes", "required-minus-one", "required"] },
         "releaseTrace": {
           "type": "array", "minItems": 7, "maxItems": 7,
           "items": {
```

### Inverse

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/📨️admission/🧪️schema.json
@@ -15,15 +15,13 @@
     "unrelatedByte": { "const": 73 },
     "nativeOwnership": {
       "type": "object", "additionalProperties": false,
+      "required": ["firstAccessAllocations", "foreignRepopulation", "phaseAccess", "exactAllocation", "grantCases", "releaseTrace", "refusals", "layout", "unknownFaultFinalDisposal"],
-      "required": ["firstAccessAllocations", "foreignRepopulation", "phaseAccess", "exactAllocation", "grantCases", "preparationGrantFrontiers", "preparationGrantCases", "releaseTrace", "refusals", "layout", "unknownFaultFinalDisposal"],
       "properties": {
         "firstAccessAllocations": { "const": [0, 0, 0] },
         "foreignRepopulation": { "type": "object", "additionalProperties": false, "required": ["afterEmptyObservation", "accepted", "consumerDropsDuringRelease"], "properties": { "afterEmptyObservation": { "const": true }, "accepted": { "const": false }, "consumerDropsDuringRelease": { "const": 0 } } },
         "phaseAccess": { "const": [{ "phase": "open", "forward": true, "recovery": false }, { "phase": "consumer-closing", "forward": false, "recovery": true }, { "phase": "root-closing", "forward": false, "recovery": true }] },
         "exactAllocation": { "const": { "alignment": 64, "failure": "injected-null", "consumerFailure": "retain-reservation", "admissionFailure": "retain-reservation", "recordFailure": "release-uninstalled-reservation", "cancelFrontiers": ["reserved", "allocated", "initialized", "published"], "cancelAllocations": [0, 1, 1, 1], "cancelDeallocations": [0, 1, 1, 1], "deallocation": "same-requested-layout-once" } },
         "grantCases": { "const": ["zero-items", "required-minus-one", "required"] },
-        "preparationGrantFrontiers": { "const": [{"phase":"reserve","required":"Option<ConsumerPage>+u64+ResidentResources+Option<NonNull<ConsumerHeader>>","before":{"reserved":false,"allocations":0},"afterRefusal":{"reserved":false,"allocations":0},"afterExact":{"reserved":true,"allocations":0}},{"phase":"allocate","required":"ConsumerNode<C>.layout+Option<NonNull<ConsumerHeader>>+u64","before":{"reserved":true,"allocations":0},"afterRefusal":{"reserved":true,"allocations":0},"afterExact":{"reserved":true,"allocations":1}}] },
-        "preparationGrantCases": { "const": ["zero-items", "zero-bytes", "required-minus-one", "required"] },
         "releaseTrace": {
           "type": "array", "minItems": 7, "maxItems": 7,
           "items": {
```

## 🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts

### Forward

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts
@@ -55,6 +55,21 @@
     });
     assert.deepEqual(cancellationModels.map(state => state.allocations), admissionFixture.nativeOwnership.exactAllocation.cancelAllocations);
     assert.deepEqual(cancellationModels.map(state => state.deallocations), admissionFixture.nativeOwnership.exactAllocation.cancelDeallocations);
+    let preparationState = { reserved: false, allocations: 0 }; let preparationRefusals = 0;
+    for (const frontier of admissionFixture.nativeOwnership.preparationGrantFrontiers) {
+      assert.deepEqual(preparationState, frontier.before);
+      for (const refusal of admissionFixture.nativeOwnership.preparationGrantCases.slice(0, -1)) {
+        const unchanged = produce(preparationState, () => {});
+        assert.equal(unchanged, preparationState, refusal); assert.deepEqual(unchanged, frontier.afterRefusal); preparationRefusals++;
+      }
+      preparationState = produce(preparationState, draft => {
+        if (frontier.phase === "reserve") { assert.equal(draft.reserved, false); draft.reserved = true; }
+        else { assert.equal(frontier.phase, "allocate"); assert(draft.reserved); assert.equal(draft.allocations, 0); draft.allocations++; }
+      });
+      assert.deepEqual(preparationState, frontier.afterExact);
+    }
+    assert.throws(() => produce({ reserved: false, allocations: 0 }, draft => { assert(draft.reserved); draft.allocations++; }));
+    console.log(`[DEBUG] Native preparation reference phases=${admissionFixture.nativeOwnership.preparationGrantFrontiers.length} refusalStates=${preparationRefusals} allocationBeforeReservationRejected=true nativeGrantArithmeticExecuted=false oracle=Immer`);
     assert.equal(admissionFixture.nativeOwnership.unknownFaultFinalDisposal, false);
     console.log(`[DEBUG] Native ownership neutralTrace=${nativeTrace.length} phaseAccess=${phaseAccess.length} cancellationFrontiers=${cancellationModels.length} sealedReplacementRefused=true oracle=Ajv+Immer actualNativeExecution=false unknownFaultFinalDisposal=false`);
     const bootstrap = new OwnedResidentLedger(fixture.capacity); const bootstrapConsumer = Object.freeze({ name: "original" }); const foreignConsumer = Object.freeze({ name: "foreign" }); const bootstrapGrant = admissionFixture.grants[2]!;
```

### Inverse

```diff
--- 🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts
+++ 🧰️framework/🔨️modules/🌱️value/💾️resident/📜️script.ts
@@ -55,21 +55,6 @@
     });
     assert.deepEqual(cancellationModels.map(state => state.allocations), admissionFixture.nativeOwnership.exactAllocation.cancelAllocations);
     assert.deepEqual(cancellationModels.map(state => state.deallocations), admissionFixture.nativeOwnership.exactAllocation.cancelDeallocations);
-    let preparationState = { reserved: false, allocations: 0 }; let preparationRefusals = 0;
-    for (const frontier of admissionFixture.nativeOwnership.preparationGrantFrontiers) {
-      assert.deepEqual(preparationState, frontier.before);
-      for (const refusal of admissionFixture.nativeOwnership.preparationGrantCases.slice(0, -1)) {
-        const unchanged = produce(preparationState, () => {});
-        assert.equal(unchanged, preparationState, refusal); assert.deepEqual(unchanged, frontier.afterRefusal); preparationRefusals++;
-      }
-      preparationState = produce(preparationState, draft => {
-        if (frontier.phase === "reserve") { assert.equal(draft.reserved, false); draft.reserved = true; }
-        else { assert.equal(frontier.phase, "allocate"); assert(draft.reserved); assert.equal(draft.allocations, 0); draft.allocations++; }
-      });
-      assert.deepEqual(preparationState, frontier.afterExact);
-    }
-    assert.throws(() => produce({ reserved: false, allocations: 0 }, draft => { assert(draft.reserved); draft.allocations++; }));
-    console.log(`[DEBUG] Native preparation reference phases=${admissionFixture.nativeOwnership.preparationGrantFrontiers.length} refusalStates=${preparationRefusals} allocationBeforeReservationRejected=true nativeGrantArithmeticExecuted=false oracle=Immer`);
     assert.equal(admissionFixture.nativeOwnership.unknownFaultFinalDisposal, false);
     console.log(`[DEBUG] Native ownership neutralTrace=${nativeTrace.length} phaseAccess=${phaseAccess.length} cancellationFrontiers=${cancellationModels.length} sealedReplacementRefused=true oracle=Ajv+Immer actualNativeExecution=false unknownFaultFinalDisposal=false`);
     const bootstrap = new OwnedResidentLedger(fixture.capacity); const bootstrapConsumer = Object.freeze({ name: "original" }); const foreignConsumer = Object.freeze({ name: "foreign" }); const bootstrapGrant = admissionFixture.grants[2]!;
```

