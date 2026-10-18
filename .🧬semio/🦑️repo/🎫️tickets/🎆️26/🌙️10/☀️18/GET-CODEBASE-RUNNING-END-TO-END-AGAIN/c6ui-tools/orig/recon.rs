//! ♻️ The reconcile authority — the thin [`SurfaceReconciler`] owner that composes a retained shadow with its handback reservation, the credit reservation, the process-wide handback registry, the resumable job, the generation-qualified patch and document owners, the terminal close authority and the surface output pool. The shadow and cursor it drives live in `semio_framework_ui_runtime_reconcile_cursor`; the bounded primitives in `semio_framework_ui_runtime_reconcile_primitives`.

use std::mem::{size_of, take};
use std::sync::{LazyLock, Mutex, MutexGuard, OnceLock};

use semio_framework_ui_runtime_presentation::{ComponentTree, UiNativeResourceLoans, UiNativeResources};
use semio_framework_ui_runtime_reconcile_primitives::{SURFACE_COMPONENT_COPY_WORK_BYTES, SURFACE_RECONCILE_PAGE_BYTES, SurfaceReconcileFault, SurfaceReconcileLimits, SurfaceReconcileUsage, SurfaceTreeRetireCursor, release_surface_reconcile, release_surface_reconcile_one, reserve_surface_reconcile_floor, surface_reconcile_limits_admissible};
use semio_framework_ui_runtime_reconcile_cursor::{SurfaceShadow, SurfaceReconcileCursor, SurfaceReconcileStep};
#[cfg(test)]
use semio_framework_ui_runtime_presentation::TreeNode;

#[path = "../📤️output/🦀️.rs"]
mod output;
pub use output::{SurfaceReconcileOutputReservation, SurfaceReconcileOutputs, SurfaceReconcileOutputTransfer};

//#region 🔖️Reconciler

/// ♻️ Keyed differ for one render surface: the retained [`SurfaceShadow`] it diffs against, composed with the
/// handback reservation it owes the process-wide registry. The shadow owns every retained owner and its
/// retirement; this owner only adds the registry hand-off that outlives it.
#[derive(Debug)]
pub struct SurfaceReconciler {
    shadow: SurfaceShadow,
    handback: Option<SurfaceReconcileHandbackReservation>,
    retirement_armed: bool,
}

impl SurfaceReconciler {
    /// 🌱️ A reconciler for `surface` with no retained state yet — the next reconcile necessarily emits a full
    /// `SetRoot` plus one `Upsert` per node.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new(surface: ui_contract::SurfaceId) -> Self {
        Self::from_shadow(SurfaceShadow::new(surface))
    }

    fn from_shadow(shadow: SurfaceShadow) -> Self {
        Self { shadow, handback: None, retirement_armed: true }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn from_text(surface: impl AsRef<str>) -> Self {
        Self::from_shadow(SurfaceShadow::from_text(surface))
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn reconcile(&mut self, tree: &ComponentTree) -> Option<ui_contract::UiPatch> {
        self.shadow.reconcile(tree)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn snapshot(&self) -> ui_contract::UiSnapshot {
        self.shadow.snapshot()
    }

    /// 🧬️ Returns the retained scalar revision without cloning the retained document.
    pub fn revision(&self) -> ui_contract::UiRevision {
        self.shadow.revision()
    }

    /// 🪪️ Returns the reconciler's surface identity without allocating a second owner.
    pub fn surface(&self) -> &ui_contract::SurfaceId {
        self.shadow.surface()
    }

    /// 📖️ Captures the canonical root without a second payload tree or reservation.
    pub fn capture_document(&self, target: &mut Option<ui_contract::UiDocumentLease>, admitted_bytes: usize) -> Result<bool, ui_contract::UiDocumentLeaseError> {
        self.shadow.capture_document(target, admitted_bytes)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn transaction_reader(&self) -> Self {
        Self::from_shadow(self.shadow.transaction_reader())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn close_transaction_oracle(&mut self) {
        while !self.retire_one() {}
    }

    fn seal_step(&mut self, usage: SurfaceReconcileUsage, output: &mut Option<ui_contract::UiResidentPermit>, has_output: bool) -> Result<bool, ui_contract::UiDocumentAssemblyError> {
        self.shadow.seal_step(size_of::<Self>(), usage, output, has_output)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn mark_rejected(&mut self) {
        self.shadow.mark_rejected();
    }

    pub fn native_resources(&self) -> Option<&UiNativeResources> { self.shadow.native_resources() }

    pub fn native_resource_loans_terminal_is_empty(&self)->bool{self.shadow.native_resource_loans_terminal_is_empty()}

    pub fn with_native_resource_loans_on_original<R>(&mut self,cx:&mut semio_framework_job::StepContext<'_>,receive:impl FnOnce(&mut UiNativeResourceLoans,&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<Option<R>,semio_framework_job::OriginalTurnFailure>{
        self.shadow.with_native_resource_loans_on_original(cx,receive)
    }

    pub fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        self.shadow.close_native_resources_on_original(cx)
    }

    fn retire_one(&mut self) -> bool {
        self.shadow.retire_one(&mut self.handback, release_surface_reconcile_handback)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn install_fixture_record(&mut self, record: ui_contract::UiNodeRecord) {
        self.shadow.install_fixture_record(record);
    }
}

impl Drop for SurfaceReconciler {
    fn drop(&mut self) {
        if !self.retirement_armed {
            return;
        }
        self.retirement_armed = false;
        if self.shadow.terminal_is_empty() {
            if let Some(handback) = self.handback.take() {
                release_surface_reconcile_handback(handback);
            }
            return;
        }
        let generation = self.handback.as_ref().map_or(0, |reservation| reservation.key.generation);
        let owner = SurfaceReconciler { shadow: self.shadow.take_all(), handback: None, retirement_armed: false };
        let state = Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: Some(owner),
            source: None,
            cursor: None,
            candidate: None,
            patch: ui_contract::UiPendingPatch::default(),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: None,
            handback: self.handback.take(),
        });
        handback_surface_reconcile(state);
    }
}

//#endregion 🔖️Reconciler

//#region ⏭️ResumableReconcile

//#region 🎟️RetainedAuthority

pub const SURFACE_RECONCILE_ADMISSION_SLOTS: usize = ui_contract::UI_RESIDENT_SLOTS;
pub const SURFACE_RECONCILE_AGGREGATE_BYTES: usize = ui_contract::UI_RESIDENT_AGGREGATE_BYTES;
pub const SURFACE_RECONCILE_AGGREGATE_ITEMS: usize = ui_contract::UI_RESIDENT_AGGREGATE_ITEMS;

fn reserve_surface_reconcile(limits: SurfaceReconcileLimits) -> Option<ui_contract::UiResidentPermit> {
    if !surface_reconcile_limits_admissible(limits) { return None; }
    if !register_surface_reconcile_backing(SURFACE_RECONCILE_PAGE_BYTES).ok()? { return None; }
    reserve_surface_reconcile_floor(limits)
}

fn register_surface_reconcile_backing(admitted_bytes: usize) -> Result<bool, ui_contract::UiResidentFault> {
    ui_contract::UiResidentPermit::try_register_runtime_backing(size_of::<LazyLock<Mutex<SurfaceReconcileHandbackRegistry>>>() + SurfaceReconcileOutputs::static_backing_bytes(), admitted_bytes)
}

/// 🎫️ Pre-materialization aggregate reservation transferred into exactly one live job.
pub struct SurfaceReconcileReservation {
    generation: u64,
    limits: SurfaceReconcileLimits,
    credit: Option<ui_contract::UiResidentPermit>,
    handback: Option<SurfaceReconcileHandbackReservation>,
    output_handback: Option<SurfaceReconcileHandbackReservation>,
}

/// 🩺️ WHICH of the three process-wide tables refused one render reservation.
///
/// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B56: a refusal here reached the reactor census as the
/// single opaque word `registry-reservation-unavailable`, so a guest livelocked in a refuse/defer cycle
/// (wave B54 §6.4: `more-work` streak 1 008 with every slot clean) could not say whether the aggregate
/// resident CREDIT ledger was exhausted, the handback registry was full, or the generation was invalid —
/// three different defects with three different fixes. Every one of them is named now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileReservationRefusal {
    Generation,
    ResidentCredit,
    HandbackSlots,
    OutputHandbackSlots,
}

impl SurfaceReconcileReservationRefusal {
    pub const fn reason(self) -> &'static str {
        match self {
            Self::Generation => "registry-generation-invalid",
            Self::ResidentCredit => "registry-resident-credit-exhausted",
            Self::HandbackSlots => "registry-handback-slots-exhausted",
            Self::OutputHandbackSlots => "registry-output-handback-slots-exhausted",
        }
    }
}

/// 📊️ What the two process-wide reconcile tables currently hold, for the reactor census.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceReconcileRegistryCensus {
    pub resident_items: usize,
    pub resident_bytes: usize,
    pub resident_slots: usize,
    pub resident_aggregate_bytes: usize,
    pub handback_free: usize,
    pub handback_slots: usize,
}

/// 📊️ Reads both reconcile registries without reserving anything, so a refusal can be priced
/// against what is actually held rather than guessed at. Answers the zero census when either table is
/// contended — a read that must never itself block a turn.
pub fn surface_reconcile_registry_census() -> SurfaceReconcileRegistryCensus {
    let resident = ui_contract::UiResidentPermit::snapshot().unwrap_or_default();
    let handback_free = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry.free_len,
        Err(std::sync::TryLockError::WouldBlock) => usize::MAX,
        Err(std::sync::TryLockError::Poisoned(poisoned)) => poisoned.into_inner().free_len,
    };
    SurfaceReconcileRegistryCensus {
        resident_items: resident.items,
        resident_bytes: resident.bytes,
        resident_slots: resident.used_slots,
        resident_aggregate_bytes: ui_contract::UI_RESIDENT_AGGREGATE_BYTES,
        handback_free,
        handback_slots: SURFACE_RECONCILE_HANDBACK_SLOTS,
    }
}

impl SurfaceReconcileReservation {
    pub fn try_new(generation: u64) -> Result<Self, SurfaceReconcileReservationRefusal> {
        if generation == 0 {
            return Err(SurfaceReconcileReservationRefusal::Generation);
        }
        let limits = SurfaceReconcileLimits::default();
        let credit = reserve_surface_reconcile(limits).ok_or(SurfaceReconcileReservationRefusal::ResidentCredit)?;
        let Some(handback) = reserve_surface_reconcile_handback(generation) else {
            release_surface_reconcile(credit);
            return Err(SurfaceReconcileReservationRefusal::HandbackSlots);
        };
        let Some(output_handback) = reserve_surface_reconcile_handback(generation) else {
            release_surface_reconcile_handback(handback);
            release_surface_reconcile(credit);
            return Err(SurfaceReconcileReservationRefusal::OutputHandbackSlots);
        };
        Ok(Self { generation, limits, credit: Some(credit), handback: Some(handback), output_handback: Some(output_handback) })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }
}

impl Drop for SurfaceReconcileReservation {
    fn drop(&mut self) {
        if let Some(credit) = self.credit.take() {
            release_surface_reconcile(credit);
        }
        if let Some(handback) = self.handback.take() {
            release_surface_reconcile_handback(handback);
        }
        if let Some(handback) = self.output_handback.take() {
            release_surface_reconcile_handback(handback);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SurfaceReconcileJobPhase {
    Drive,
    RetireCursor,
    RetirePrevious,
    SealCandidate,
    Ready,
    Fault,
    Closing,
}

pub const SURFACE_RECONCILE_HANDBACK_SLOTS: usize = SURFACE_RECONCILE_ADMISSION_SLOTS * 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceReconcileHandbackKey {
    slot: usize,
    epoch: u64,
    generation: u64,
}

#[derive(Debug)]
struct SurfaceReconcileHandbackReservation {
    key: SurfaceReconcileHandbackKey,
}

#[derive(Default)]
struct SurfaceReconcileHandbackSlot {
    epoch: u64,
    generation: u64,
    reserved: bool,
    queued: bool,
    state: Option<Box<SurfaceReconcileRetained>>,
}


struct SurfaceReconcileHandbackRegistry {
    slots: [SurfaceReconcileHandbackSlot; SURFACE_RECONCILE_HANDBACK_SLOTS],
    free: [usize; SURFACE_RECONCILE_HANDBACK_SLOTS],
    free_len: usize,
    retirement: [usize; SURFACE_RECONCILE_HANDBACK_SLOTS],
    retirement_head: usize,
    retirement_len: usize,
}

impl Default for SurfaceReconcileHandbackRegistry {
    fn default() -> Self {
        Self {
            slots: std::array::from_fn(|_| SurfaceReconcileHandbackSlot::default()),
            free: std::array::from_fn(|index| SURFACE_RECONCILE_HANDBACK_SLOTS - 1 - index),
            free_len: SURFACE_RECONCILE_HANDBACK_SLOTS,
            retirement: [usize::MAX; SURFACE_RECONCILE_HANDBACK_SLOTS],
            retirement_head: 0,
            retirement_len: 0,
        }
    }
}

static SURFACE_RECONCILE_HANDBACKS: LazyLock<Mutex<SurfaceReconcileHandbackRegistry>> = LazyLock::new(|| Mutex::new(SurfaceReconcileHandbackRegistry::default()));

fn reserve_surface_reconcile_handback(generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    if generation == 0 {
        return None;
    }
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if registry.free_len == 0 {
        return None;
    }
    let slot = registry.free[registry.free_len - 1];
    let epoch = registry.slots[slot].epoch.checked_add(1)?;
    registry.free_len -= 1;
    registry.slots[slot] = SurfaceReconcileHandbackSlot { epoch, generation, reserved: true, queued: false, state: None };
    Some(SurfaceReconcileHandbackReservation { key: SurfaceReconcileHandbackKey { slot, epoch, generation } })
}

fn try_reserve_surface_reconcile_handback(generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    if generation == 0 { return None; }
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return None,
        Err(std::sync::TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
    };
    if registry.free_len == 0 { return None; }
    let slot = registry.free[registry.free_len - 1];
    let epoch = registry.slots[slot].epoch.checked_add(1)?;
    registry.free_len -= 1;
    registry.slots[slot] = SurfaceReconcileHandbackSlot { epoch, generation, reserved: true, queued: false, state: None };
    Some(SurfaceReconcileHandbackReservation { key: SurfaceReconcileHandbackKey { slot, epoch, generation } })
}

#[expect(clippy::needless_pass_by_value, reason = "Releasing a reservation consumes its unique slot authority so callers cannot reuse it.")]
fn release_surface_reconcile_handback_in(registry: &mut SurfaceReconcileHandbackRegistry, reservation: SurfaceReconcileHandbackReservation) {
    let should_free = {
        let Some(slot) = registry.slots.get_mut(reservation.key.slot) else { return };
        if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() {
            return;
        }
        slot.reserved = false;
        slot.generation = 0;
        !slot.queued
    };
    if should_free {
        let index = registry.free_len;
        registry.free[index] = reservation.key.slot;
        registry.free_len += 1;
    }
}

fn release_surface_reconcile_handback(reservation: SurfaceReconcileHandbackReservation) {
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    release_surface_reconcile_handback_in(&mut registry, reservation);
}

fn release_surface_reconcile_handback_one(owner: &mut Option<SurfaceReconcileHandbackReservation>) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
    let Some(reservation) = owner.as_ref() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(Default::default()),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    let slot = registry.slots.get(reservation.key.slot).ok_or("surface handback slot is invalid")?;
    if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() { return Err("surface handback is not the exact retained owner"); }
    let free = !slot.queued;
    if free && registry.free_len >= SURFACE_RECONCILE_HANDBACK_SLOTS { return Err("surface handback free list exhausted"); }
    let slot = &mut registry.slots[reservation.key.slot];
    slot.reserved = false; slot.generation = 0;
    if free { let index = registry.free_len; registry.free[index] = reservation.key.slot; registry.free_len += 1; }
    owner.take();
    Ok(ui_contract::UiValueRetirementStep { complete: true, progressed: true, released_items: 1, released_bytes: 0 })
}

fn rebind_surface_reconcile_handback(mut reservation: SurfaceReconcileHandbackReservation, generation: u64) -> Result<SurfaceReconcileHandbackReservation, SurfaceReconcileHandbackReservation> {
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let Some(slot) = registry.slots.get_mut(reservation.key.slot) else { return Err(reservation) };
    if !slot.reserved || slot.epoch != reservation.key.epoch || slot.generation != reservation.key.generation || slot.state.is_some() || generation == 0 {
        return Err(reservation);
    }
    slot.generation = generation;
    reservation.key.generation = generation;
    Ok(reservation)
}

fn acquire_surface_reconcile_handback(owner: &mut Option<SurfaceReconcileHandbackReservation>, generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    let Some(reservation) = owner.take() else { return reserve_surface_reconcile_handback(generation) };
    match rebind_surface_reconcile_handback(reservation, generation) {
        Ok(reservation) => Some(reservation),
        Err(reservation) => {
            *owner = Some(reservation);
            None
        }
    }
}

fn acquire_reserved_surface_reconcile_handback(owner: &mut Option<SurfaceReconcileHandbackReservation>, reserved: &mut Option<SurfaceReconcileHandbackReservation>, generation: u64) -> Option<SurfaceReconcileHandbackReservation> {
    if owner.is_none() {
        return reserved.take();
    }
    acquire_surface_reconcile_handback(owner, generation)
}

struct SurfaceReconcileRetained {
    generation: u64,
    phase: SurfaceReconcileJobPhase,
    current: Option<SurfaceReconciler>,
    source: Option<ComponentTree>,
    cursor: Option<SurfaceReconcileCursor>,
    candidate: Option<SurfaceReconciler>,
    patch: ui_contract::UiPendingPatch,
    retire_tree: SurfaceTreeRetireCursor,
    fault: Option<SurfaceReconcileFault>,
    usage: SurfaceReconcileUsage,
    credit: Option<ui_contract::UiResidentPermit>,
    handback: Option<SurfaceReconcileHandbackReservation>,
    output_handback: Option<SurfaceReconcileHandbackReservation>,
}

impl SurfaceReconcileRetained {
    fn close_native_resources_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<bool>, semio_framework_job::OriginalTurnFailure> {
        if let Some(owner) = self.candidate.as_mut() { if owner.close_native_resources_on_original(cx)? != Some(true) { return Ok(None); } }
        if let Some(owner) = self.cursor.as_mut() { if owner.close_native_resources_on_original(cx)? != Some(true) { return Ok(None); } }
        if let Some(owner) = self.current.as_mut() { if owner.close_native_resources_on_original(cx)? != Some(true) { return Ok(None); } }
        if let Some(tree) = self.source.as_mut() {
            if let Some(resources) = tree.native_resources.as_mut() {
                if resources.close_step_on_original(cx)? != Some(true) { return Ok(None); }
                tree.native_resources = None;
                return Ok(None);
            }
        }
        self.retire_tree.close_native_resources_on_original(cx)
    }

    fn refuse_publication(&mut self, reason: &'static str) -> Result<bool, &'static str> {
        self.fault = Some(SurfaceReconcileFault::PublicationAuthority(reason));
        self.phase = SurfaceReconcileJobPhase::Fault;
        Err(reason)
    }

    fn close_step(&mut self) -> bool {
        self.close_step_with(None)
    }

    fn close_step_with(&mut self, registry: Option<&mut SurfaceReconcileHandbackRegistry>) -> bool {
        self.close_unit(1, SURFACE_RECONCILE_PAGE_BYTES, registry).0
    }

    /// 🧹️ One retirement unit of this retained surface against the caller's grant, answering
    /// `(complete, consumed)`. A PAGED owner (the pending patch) spends the whole grant in one call
    /// and reports it; every other owner on the ladder is a single retained value and consumes one
    /// item, so [`Self::close_run`] can spend a page grant either way without squaring it.
    fn close_unit(&mut self, items: usize, bytes: usize, mut registry: Option<&mut SurfaceReconcileHandbackRegistry>) -> (bool, usize) {
        self.phase = SurfaceReconcileJobPhase::Closing;
        if self.fault.take().is_some() {
            return (false, 1);
        }
        if !self.patch.terminal_is_empty() {
            let _ = self.patch.close_step(items, bytes);
            return (false, items);
        }
        if let Some(candidate) = self.candidate.as_mut() {
            if !candidate.retire_one() {
                return (false, 1);
            }
            self.candidate = None;
            return (false, 1);
        }
        if let Some(cursor) = self.cursor.as_mut() {
            if !cursor.retire_one() {
                return (false, 1);
            }
            self.cursor = None;
            return (false, 1);
        }
        if let Some(current) = self.current.as_mut() {
            if !current.retire_one() {
                return (false, 1);
            }
            self.current = None;
            return (false, 1);
        }
        if !self.retire_tree.step() {
            return (false, 1);
        }
        if self.retire_tree.try_begin_tree(&mut self.source) {
            return (false, 1);
        }
        if let Some(credit) = self.credit.take() {
            release_surface_reconcile(credit);
            return (false, 1);
        }
        if let Some(handback) = self.handback.take() {
            match registry.as_deref_mut() {
                Some(registry) => release_surface_reconcile_handback_in(registry, handback),
                None => release_surface_reconcile_handback(handback),
            }
            return (false, 1);
        }
        if let Some(handback) = self.output_handback.take() {
            match registry.as_deref_mut() {
                Some(registry) => release_surface_reconcile_handback_in(registry, handback),
                None => release_surface_reconcile_handback(handback),
            }
            return (false, 1);
        }
        (true, 1)
    }

    /// 🧹️ Retires this retained surface for a bounded RUN priced by the caller's page grant instead
    /// of the single owner per call [`Self::close_step`] retires. Every unit the reactor cannot
    /// finish this turn is answered as `MoreWork` and costs the host one round trip, so a
    /// document-scaled retained surface must not be retired one value per turn (ticket 26/09/02,
    /// W-S2 §2.4 measured 1 092 units for one 180-object world publication, W-B2 1 093 turns for the
    /// mixed surface set the same interaction touches).
    fn close_run(&mut self, items: usize, bytes: usize) -> bool {
        let mut remaining = items.max(1);
        while remaining > 0 {
            let (complete, consumed) = self.close_unit(remaining, bytes, None);
            if complete {
                return true;
            }
            remaining = remaining.saturating_sub(consumed.max(1));
        }
        false
    }

    fn close_admitted_step(&mut self) -> bool {
        self.close_admitted_run(1, SURFACE_RECONCILE_PAGE_BYTES)
    }

    fn close_admitted_run(&mut self, items: usize, bytes: usize) -> bool {
        if self.source.is_some() && self.handback.is_none() {
            self.handback = self.output_handback.take().or_else(|| try_reserve_surface_reconcile_handback(self.generation));
            if self.handback.is_none() { return false; }
        }
        self.close_run(items, bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.current.is_none()
            && self.source.is_none()
            && self.cursor.is_none()
            && self.candidate.is_none()
            && self.patch.terminal_is_empty()
            && self.retire_tree.is_empty()
            && self.fault.is_none()
            && self.credit.is_none()
            && self.handback.is_none()
            && self.output_handback.is_none()
    }
}

fn handback_surface_reconcile(mut state: Box<SurfaceReconcileRetained>) {
    if state.handback.is_none() {
        state.handback = state.current.as_mut().and_then(|owner| owner.handback.take()).or_else(|| state.candidate.as_mut().and_then(|owner| owner.handback.take())).or_else(|| state.output_handback.take());
    }
    let Some(reservation) = state.handback.take() else { return };
    let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let enqueue = {
        let slot = &mut registry.slots[reservation.key.slot];
        let enqueue = !slot.queued;
        slot.state = Some(state);
        if enqueue {
            slot.queued = true;
        }
        enqueue
    };
    if enqueue {
        let tail = (registry.retirement_head + registry.retirement_len) % SURFACE_RECONCILE_HANDBACK_SLOTS;
        registry.retirement[tail] = reservation.key.slot;
        registry.retirement_len += 1;
    }
}

/// 🚦️ One admitted reconciliation opportunity result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceReconcileJobStep {
    MoreWork,
    Ready,
    Fault,
}

/// 📄️ Why a retained renderer-document producer stopped before publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceDocumentFault {
    StaleGeneration { expected: u64, actual: u64 },
    StaleRevision { expected: ui_contract::UiRevision, actual: ui_contract::UiRevision },
    Cancelled,
    Build(ui_contract::UiDocumentBuildError),
}

/// ⏭️ One bounded renderer-document production opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceDocumentProducerStep {
    MoreWork,
    Ready,
    Fault,
}

/// 🧵️ Generation-qualified producer which admits one retained node page per opportunity.
pub struct SurfaceDocumentProducer {
    generation: u64,
    revision: ui_contract::UiRevision,
    lease: Option<ui_contract::UiDocumentLease>,
    ready: bool,
    fault: Option<SurfaceDocumentFault>,
}

impl SurfaceDocumentProducer {
    pub fn try_new(current: &SurfaceReconciler, generation: u64) -> Result<Self, ui_contract::UiDocumentLeaseError> {
        if generation == 0 { return Err(ui_contract::UiDocumentLeaseError::StaleHandle); }
        let mut lease = None;
        if !current.capture_document(&mut lease, SURFACE_RECONCILE_PAGE_BYTES)? { return Err(ui_contract::UiDocumentLeaseError::StaleHandle); }
        Ok(Self { generation, revision: current.revision(), lease, ready: false, fault: None })
    }

    pub fn generation(&self) -> u64 { self.generation }
    pub fn fault(&self) -> Option<&SurfaceDocumentFault> { self.fault.as_ref() }

    pub fn drive_one(&mut self, current: &SurfaceReconciler, cx: &mut semio_framework_job::StepContext<'_>) -> SurfaceDocumentProducerStep {
        if self.fault.is_some() { return SurfaceDocumentProducerStep::Fault; }
        if cx.generation().0 != self.generation {
            self.fault = Some(SurfaceDocumentFault::StaleGeneration { expected: self.generation, actual: cx.generation().0 });
            return SurfaceDocumentProducerStep::Fault;
        }
        if cx.is_cancelled() { self.fault = Some(SurfaceDocumentFault::Cancelled); return SurfaceDocumentProducerStep::Fault; }
        if cx.should_yield() { return SurfaceDocumentProducerStep::MoreWork; }
        if !current.shadow.document_root_matches(self.lease.as_ref()) {
            self.fault = Some(SurfaceDocumentFault::StaleRevision { expected: self.revision, actual: current.revision() });
            return SurfaceDocumentProducerStep::Fault;
        }
        self.ready = true;
        cx.consume_fuel(1);
        SurfaceDocumentProducerStep::Ready
    }

    pub fn take_ready(mut self) -> Result<SurfaceDocumentOutcome, Self> {
        if !self.ready || self.fault.is_some() { return Err(self); }
        let Some(lease) = self.lease.take() else { return Err(self); };
        Ok(SurfaceDocumentOutcome { generation: self.generation, lease })
    }

    pub fn close_step(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        let Some(lease) = self.lease.as_mut() else { return Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() }); };
        let progress = lease.close_read_step_with_grant(items, bytes)?;
        if progress.complete { self.lease = None; }
        Ok(progress)
    }
}
/// 📬️ Complete generation-qualified fixed document transferred to a renderer consumer.
pub struct SurfaceDocumentOutcome {
    generation: u64,
    lease: ui_contract::UiDocumentLease,
}

impl SurfaceDocumentOutcome {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn header(&self) -> Result<ui_contract::UiDocumentLeaseHeader, ui_contract::UiDocumentLeaseError> {
        self.lease.header()
    }

    pub fn try_read(&self) -> Result<ui_contract::UiDocumentRead<'_>, ui_contract::UiDocumentLeaseError> {
        self.lease.try_read()
    }

    pub fn try_alias(&self) -> Result<Self, ui_contract::UiDocumentLeaseError> {
        let mut alias = None;
        if !self.lease.try_alias_into(&mut alias, SURFACE_RECONCILE_PAGE_BYTES)? { return Err(ui_contract::UiDocumentLeaseError::AliasCapacity); }
        Ok(Self { generation: self.generation, lease: alias.unwrap() })
    }

    pub fn into_lease(self) -> ui_contract::UiDocumentLease {
        self.lease
    }

    pub fn close_step(&mut self) -> bool {
        self.lease.close_read_step_with_grant(1, SURFACE_COMPONENT_COPY_WORK_BYTES).expect("document outcome retains exact retirement authority").complete
    }
}

/// 📨️ Generation-qualified patch owner carrying its share of the live reconciliation credit.
pub struct SurfaceReconcileReadyPatch {
    generation: u64,
    patch: ui_contract::UiPendingPatch,
    credit: Option<ui_contract::UiResidentPermit>,
    handback: Option<SurfaceReconcileHandbackReservation>,
}

fn pending_surface_patch(patch: Option<ui_contract::UiPatch>) -> ui_contract::UiPendingPatch {
    let mut owner = ui_contract::UiPendingPatch::default();
    *owner.source_mut().expect("new pending patch is writable") = patch;
    owner
}

fn close_surface_patch_owner(patch: &mut ui_contract::UiPendingPatch, credit: &mut Option<ui_contract::UiResidentPermit>, handback: &mut Option<SurfaceReconcileHandbackReservation>, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
    if items == 0 || bytes == 0 { return Ok(Default::default()); }
    if !patch.terminal_is_empty() {
        let mut step = patch.close_step(items, bytes)?;
        step.complete = false;
        return Ok(step);
    }
    if credit.is_some() { let mut step = release_surface_reconcile_one(credit)?; step.complete = false; return Ok(step); }
    if handback.is_some() { let mut step = release_surface_reconcile_handback_one(handback)?; step.complete = false; return Ok(step); }
    Ok(ui_contract::UiValueRetirementStep { complete: true, ..Default::default() })
}

#[cfg(test)]
#[path = "../🩹️patch/🧪️tests/🩹️patch/🦀️.rs"]
mod patch_handoff_tests;

impl SurfaceReconcileReadyPatch {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.patch.get().map(|patch| &patch.surface)
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.patch.get().map_or_else(ui_contract::UiRevision::default, |patch| patch.revision)
    }

    pub const fn required_publish_bytes() -> usize { size_of::<SurfaceReconcilePublishedPatch>() + size_of::<ui_contract::UiPatch>() + size_of::<ui_contract::SurfaceId>() }

    /// 📬️ Both destination slots and fixed initialization/moves are admitted before detaching the source.
    pub fn publish_into(&mut self, payload: &mut ui_contract::UiPendingPatch, published: &mut Option<SurfaceReconcilePublishedPatch>, admitted_bytes: usize) -> Result<usize, &'static str> {
        if admitted_bytes < Self::required_publish_bytes() || published.is_some() || !payload.terminal_is_empty() { return Ok(0); }
        if payload.source_mut()?.is_some() { return Ok(0); }
        let Some(source) = self.patch.get() else { return Ok(0); };
        let metadata = pending_surface_patch(Some(ui_contract::UiPatch { surface: source.surface.clone(), base_revision: source.base_revision, revision: source.revision, ops: ui_contract::UiPatchOps::default() }));
        let revision = source.revision;
        *payload.source_mut()? = self.patch.source_mut()?.take();
        *published = Some(SurfaceReconcilePublishedPatch { generation: self.generation, metadata, revision, credit: self.credit.take(), handback: self.handback.take() });
        Ok(Self::required_publish_bytes())
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("ready patch retirement remains exact").complete
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        close_surface_patch_owner(&mut self.patch, &mut self.credit, &mut self.handback, items, bytes)
    }
    pub fn terminal_is_empty(&self) -> bool { self.patch.terminal_is_empty() && self.credit.is_none() && self.handback.is_none() }
}

impl Drop for SurfaceReconcileReadyPatch {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let generation = self.generation;
        handback_surface_reconcile(Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: None,
            source: None,
            cursor: None,
            candidate: None,
            patch: take(&mut self.patch),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: self.credit.take(),
            handback: self.handback.take(),
        }));
    }
}

/// 📮️ Credit witness retained by the reactor until the published revision is acknowledged or closed.
pub struct SurfaceReconcilePublishedPatch {
    generation: u64,
    metadata: ui_contract::UiPendingPatch,
    revision: ui_contract::UiRevision,
    credit: Option<ui_contract::UiResidentPermit>,
    handback: Option<SurfaceReconcileHandbackReservation>,
}

impl SurfaceReconcilePublishedPatch {
    /// ♻️ Releases one exact published owner without handing its terminal into a global queue.
    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("published patch retirement remains exact").complete
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> {
        close_surface_patch_owner(&mut self.metadata, &mut self.credit, &mut self.handback, items, bytes)
    }

    /// 🧾️ Verifies this published handle owns neither payload nor admission/handback authority.
    pub fn terminal_is_empty(&self) -> bool {
        self.metadata.terminal_is_empty() && self.credit.is_none() && self.handback.is_none()
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn matches(&self, surface: &str, revision: u64) -> bool {
        self.surface().is_some_and(|exact| exact.0.as_str() == surface) && self.revision.0 == revision
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.metadata.get().map(|patch| &patch.surface)
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.revision
    }

    pub const fn required_acknowledge_bytes() -> usize { size_of::<Self>() + size_of::<SurfaceReconcilePublishedAck>() }

    /// ✅️ A mismatched receipt, occupied target, or insufficient grant leaves both slots untouched.
    pub fn acknowledge_into(source: &mut Option<Self>, target: &mut Option<SurfaceReconcilePublishedAck>, surface: &str, revision: u64, admitted_bytes: usize) -> Result<bool, &'static str> {
        if admitted_bytes < Self::required_acknowledge_bytes() || target.is_some() || !source.as_ref().is_some_and(|owner| owner.matches(surface, revision)) { return Ok(false); }
        *target = Some(SurfaceReconcilePublishedAck { owner: source.take().ok_or("published source lost after receipt validation")? });
        Ok(true)
    }
}

/// ✅️ Unforgeable ACK authority produced only by consuming the exact published patch owner.
pub struct SurfaceReconcilePublishedAck {
    owner: SurfaceReconcilePublishedPatch,
}

impl SurfaceReconcilePublishedAck {
    pub fn generation(&self) -> u64 {
        self.owner.generation()
    }

    pub fn surface(&self) -> Option<&ui_contract::SurfaceId> {
        self.owner.surface()
    }

    pub fn revision(&self) -> ui_contract::UiRevision {
        self.owner.revision()
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<ui_contract::UiValueRetirementStep, &'static str> { self.owner.close_step_with_grant(items, bytes) }
    pub fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

impl Drop for SurfaceReconcilePublishedPatch {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        handback_surface_reconcile(Box::new(SurfaceReconcileRetained {
            output_handback: None,
            generation: self.generation,
            phase: SurfaceReconcileJobPhase::Closing,
            current: None,
            source: None,
            cursor: None,
            candidate: None,
            patch: take(&mut self.metadata),
            retire_tree: SurfaceTreeRetireCursor::default(),
            fault: None,
            usage: SurfaceReconcileUsage::default(),
            credit: self.credit.take(),
            handback: self.handback.take(),
        }));
    }
}

/// 🧵️ Generation-keyed by-value reconciliation job advanced once per worker grant.
pub struct SurfaceReconcileJob {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl std::fmt::Debug for SurfaceReconcileJob {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SurfaceReconcileJob")
            .field("phase", &self.state.as_ref().map(|state| state.phase))
            .field("usage", &self.state.as_ref().map(|state| state.usage))
            .field("credit", &self.state.as_ref().and_then(|state| state.credit.as_ref()))
            .field("fault", &self.fault())
            .finish()
    }
}

impl SurfaceReconcileJob {
    pub fn try_new(current: SurfaceReconciler, tree: ComponentTree, generation: u64) -> Result<Self, SurfaceReconcileRejected> {
        Self::try_new_with_limits(current, tree, generation, SurfaceReconcileLimits::default())
    }

    pub fn try_new_with_limits(mut current: SurfaceReconciler, tree: ComponentTree, generation: u64, limits: SurfaceReconcileLimits) -> Result<Self, SurfaceReconcileRejected> {
        let surface_bytes = current.shadow.surface_identifier_bytes();
        let handback = acquire_surface_reconcile_handback(&mut current.handback, generation);
        let credit = if surface_bytes <= limits.max_identifier_bytes { reserve_surface_reconcile(limits) } else { None };
        let output_handback = reserve_surface_reconcile_handback(generation);
        let (credit, handback, output_handback) = match (credit, handback, output_handback) {
            (Some(credit), Some(handback), Some(output_handback)) => (credit, handback, output_handback),
            (credit, handback, output_handback) => {
                return Err(SurfaceReconcileRejected {
                    state: Some(Box::new(SurfaceReconcileRetained {
                        output_handback,
                        generation,
                        phase: SurfaceReconcileJobPhase::Fault,
                        current: Some(current),
                        source: Some(tree),
                        cursor: None,
                        candidate: None,
                        patch: ui_contract::UiPendingPatch::default(),
                        retire_tree: SurfaceTreeRetireCursor::default(),
                        fault: Some(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: surface_bytes }, limits }),
                        usage: SurfaceReconcileUsage::default(),
                        credit,
                        handback,
                    })),
                })
            }
        };
        let cursor = SurfaceReconcileCursor::new_admitted(tree, &current.shadow, limits, generation, Some(credit));
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: Some(output_handback),
                generation,
                phase: SurfaceReconcileJobPhase::Drive,
                current: Some(current),
                source: None,
                cursor: Some(cursor),
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit: None,
                handback: Some(handback),
            })),
        })
    }

    pub fn try_new_reserved(mut current: SurfaceReconciler, tree: ComponentTree, mut reservation: SurfaceReconcileReservation) -> Result<Self, SurfaceReconcileRejected> {
        let generation = reservation.generation;
        let limits = reservation.limits;
        let surface_bytes = current.shadow.surface_identifier_bytes();
        let handback = acquire_reserved_surface_reconcile_handback(&mut current.handback, &mut reservation.handback, generation);
        if surface_bytes > limits.max_identifier_bytes {
            return Err(SurfaceReconcileRejected {
                state: Some(Box::new(SurfaceReconcileRetained {
                    output_handback: reservation.output_handback.take(),
                    generation,
                    phase: SurfaceReconcileJobPhase::Fault,
                    current: Some(current),
                    source: Some(tree),
                    cursor: None,
                    candidate: None,
                    patch: ui_contract::UiPendingPatch::default(),
                    retire_tree: SurfaceTreeRetireCursor::default(),
                    fault: Some(SurfaceReconcileFault::IdentifierBytes { actual: surface_bytes, max: limits.max_identifier_bytes }),
                    usage: SurfaceReconcileUsage::default(),
                    credit: reservation.credit.take(),
                    handback,
                })),
            });
        }
        let Some(handback) = handback else {
            return Err(SurfaceReconcileRejected {
                state: Some(Box::new(SurfaceReconcileRetained {
                    output_handback: reservation.output_handback.take(),
                    generation,
                    phase: SurfaceReconcileJobPhase::Fault,
                    current: Some(current),
                    source: Some(tree),
                    cursor: None,
                    candidate: None,
                    patch: ui_contract::UiPendingPatch::default(),
                    retire_tree: SurfaceTreeRetireCursor::default(),
                    fault: Some(SurfaceReconcileFault::Credits { usage: SurfaceReconcileUsage { nodes: 0, items: 1, bytes: surface_bytes }, limits }),
                    usage: SurfaceReconcileUsage::default(),
                    credit: reservation.credit.take(),
                    handback: None,
                })),
            });
        };
        let cursor = SurfaceReconcileCursor::new_admitted(tree, &current.shadow, limits, generation, reservation.credit.take());
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: reservation.output_handback.take(),
                generation,
                phase: SurfaceReconcileJobPhase::Drive,
                current: Some(current),
                source: None,
                cursor: Some(cursor),
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit: reservation.credit.take(),
                handback: Some(handback),
            })),
        })
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn base_revision(&self) -> ui_contract::UiRevision {
        self.state.as_ref().and_then(|state| state.current.as_ref()).map_or(ui_contract::UiRevision::default(), SurfaceReconciler::revision)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn transaction_usage(&self) -> SurfaceReconcileUsage {
        self.state.as_ref().map_or_else(SurfaceReconcileUsage::default, |state| state.cursor.as_ref().map_or(state.usage, SurfaceReconcileCursor::usage))
    }

    pub fn drive_one(&mut self, generation: u64, cx: &mut semio_framework_job::StepContext<'_>) -> Result<SurfaceReconcileJobStep, semio_framework_job::OriginalTurnFailure> {
        let Some(state) = self.state.as_mut() else { return Ok(SurfaceReconcileJobStep::Fault) };
        if state.phase == SurfaceReconcileJobPhase::Fault || state.phase == SurfaceReconcileJobPhase::Closing {
            return Ok(SurfaceReconcileJobStep::Fault);
        }
        if generation != state.generation {
            state.fault = Some(SurfaceReconcileFault::StaleGeneration { expected: state.generation, actual: generation });
            state.phase = SurfaceReconcileJobPhase::Fault;
            return Ok(SurfaceReconcileJobStep::Fault);
        }
        if cx.is_cancelled() {
            state.fault = Some(SurfaceReconcileFault::Cancelled);
            state.phase = SurfaceReconcileJobPhase::Fault;
            return Ok(SurfaceReconcileJobStep::Fault);
        }
        if cx.should_yield() {
            return Ok(SurfaceReconcileJobStep::MoreWork);
        }
        if state.phase == SurfaceReconcileJobPhase::Ready {
            return Ok(SurfaceReconcileJobStep::Ready);
        }
        let outcome = match state.phase {
            SurfaceReconcileJobPhase::Drive => {
                let Some(current) = state.current.as_ref() else {
                    state.phase = SurfaceReconcileJobPhase::Fault;
                    return Ok(SurfaceReconcileJobStep::Fault);
                };
                let Some(cursor) = state.cursor.as_mut() else {
                    state.phase = SurfaceReconcileJobPhase::Fault;
                    return Ok(SurfaceReconcileJobStep::Fault);
                };
                match cursor.step(&current.shadow) {
                    SurfaceReconcileStep::Yield { .. } => SurfaceReconcileJobStep::MoreWork,
                    SurfaceReconcileStep::Complete { shadow, patch } => {
                        state.usage = cursor.usage();
                        state.candidate = Some(SurfaceReconciler::from_shadow(shadow));
                        *state.patch.source_mut().expect("ready job retains writable patch owner") = patch;
                        state.phase = SurfaceReconcileJobPhase::RetireCursor;
                        SurfaceReconcileJobStep::MoreWork
                    }
                    SurfaceReconcileStep::Fault(fault) => {
                        state.fault = Some(fault);
                        state.phase = SurfaceReconcileJobPhase::Fault;
                        SurfaceReconcileJobStep::Fault
                    }
                }
            }
            SurfaceReconcileJobPhase::RetireCursor => {
                if let Some(cursor) = state.cursor.as_mut() { if cursor.close_native_resources_on_original(cx)? != Some(true) { return Ok(SurfaceReconcileJobStep::MoreWork); } }
                if state.cursor.as_mut().is_some_and(|cursor| cursor.retire_one()) {
                    state.cursor = None;
                    state.phase = SurfaceReconcileJobPhase::RetirePrevious;
                }
                SurfaceReconcileJobStep::MoreWork
            }
            SurfaceReconcileJobPhase::RetirePrevious => {
                if let Some(current) = state.current.as_mut() { if current.close_native_resources_on_original(cx)? != Some(true) { return Ok(SurfaceReconcileJobStep::MoreWork); } }
                if state.current.as_mut().is_some_and(|current| current.retire_one()) {
                    state.current = None;
                    state.phase = SurfaceReconcileJobPhase::SealCandidate;
                    SurfaceReconcileJobStep::MoreWork
                } else {
                    SurfaceReconcileJobStep::MoreWork
                }
            }
            SurfaceReconcileJobPhase::SealCandidate => {
                let candidate = state.candidate.as_mut().expect("completed cursor retains candidate assembly");
                match candidate.seal_step(state.usage, &mut state.credit, state.patch.get().is_some()) {
                    Ok(true) => { state.phase = SurfaceReconcileJobPhase::Ready; SurfaceReconcileJobStep::Ready }
                    Ok(false) => SurfaceReconcileJobStep::MoreWork,
                    Err(error) if error.kind == ui_contract::UiDocumentAssemblyErrorKind::Contended => SurfaceReconcileJobStep::MoreWork,
                    Err(_) => { state.fault = Some(SurfaceReconcileFault::AliasCapacity); state.phase = SurfaceReconcileJobPhase::Fault; SurfaceReconcileJobStep::Fault }
                }
            }
            SurfaceReconcileJobPhase::Ready => SurfaceReconcileJobStep::Ready,
            SurfaceReconcileJobPhase::Fault | SurfaceReconcileJobPhase::Closing => SurfaceReconcileJobStep::Fault,
        };
        cx.consume_fuel(1);
        if cx.is_cancelled() {
            state.fault = Some(SurfaceReconcileFault::Cancelled);
            state.phase = SurfaceReconcileJobPhase::Fault;
            return Ok(SurfaceReconcileJobStep::Fault);
        }
        Ok(outcome)
    }

    pub fn fault(&self) -> Option<&SurfaceReconcileFault> {
        self.state.as_ref().and_then(|state| state.fault.as_ref())
    }

    pub fn is_ready(&self) -> bool { self.state.as_ref().is_some_and(|state| state.phase == SurfaceReconcileJobPhase::Ready) }

    pub const fn required_ready_transfer_bytes() -> usize {
        2 * size_of::<SurfaceReconciler>() + size_of::<SurfaceReconcileReadyPatch>() + 3 * size_of::<ui_contract::UiPendingPatch>() + 2 * size_of::<ui_contract::UiResidentPermit>() + 4 * size_of::<SurfaceReconcileHandbackReservation>()
    }

    /// 📬️ Preadmitted receivers take the exact roots while the original job shell remains retained.
    pub fn take_ready_into(&mut self, current: &mut Option<SurfaceReconciler>, ready: &mut Option<SurfaceReconcileReadyPatch>, admitted_bytes: usize) -> Result<bool, &'static str> {
        if admitted_bytes < Self::required_ready_transfer_bytes() || current.is_some() || ready.is_some() { return Ok(false); }
        let Some(state) = self.state.as_mut() else { return Ok(false); };
        if state.phase != SurfaceReconcileJobPhase::Ready { return Ok(false); }
        let Some(candidate) = state.candidate.as_ref() else { return state.refuse_publication("ready job has no canonical candidate"); };
        if !candidate.shadow.has_document() || candidate.handback.is_some() || state.handback.is_none() { return state.refuse_publication("ready job canonical owner authority is incomplete"); }
        if state.current.is_some() || state.cursor.is_some() || state.source.is_some() || !state.retire_tree.is_empty() { return state.refuse_publication("ready job still owns unfinished source retirement"); }
        let has_patch = state.patch.get().is_some();
        if has_patch && (state.credit.is_none() || state.output_handback.is_none()) { return state.refuse_publication("ready patch is missing its paired publication authority"); }
        if has_patch {
            *ready = Some(SurfaceReconcileReadyPatch { generation: state.generation, patch: ui_contract::UiPendingPatch::default(), credit: None, handback: None });
            let target = ready.as_mut().expect("preadmitted structural output receiver");
            std::mem::swap(&mut target.patch, &mut state.patch);
            target.credit = state.credit.take();
            target.handback = state.output_handback.take();
        }
        state.candidate.as_mut().expect("preflight canonical candidate").handback = state.handback.take();
        *current = state.candidate.take();
        state.phase = SurfaceReconcileJobPhase::Closing;
        Ok(true)
    }

    #[cfg(any(test, feature = "testkit"))]
    fn take_ready(mut self) -> Result<(SurfaceReconciler, Option<SurfaceReconcileReadyPatch>), Self> {
        let mut current = None;
        let mut ready = None;
        if self.take_ready_into(&mut current, &mut ready, Self::required_ready_transfer_bytes()) != Ok(true) { return Err(self); }
        let mut terminal = self.into_terminal();
        while !terminal.close_step() {}
        Ok((current.expect("granted exact root transfer"), ready))
    }

    pub fn into_terminal(mut self) -> SurfaceReconcileTerminal {
        SurfaceReconcileTerminal { state: self.state.take() }
    }
}

impl Drop for SurfaceReconcileJob {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

/// 🔄️ Exact pre-admission owner returned without snapshot/tree cloning.
pub struct SurfaceReconcileRejected {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl std::fmt::Debug for SurfaceReconcileRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("SurfaceReconcileRejected").field("generation", &self.generation()).finish()
    }
}

impl SurfaceReconcileRejected {
    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn retry(mut self, limits: SurfaceReconcileLimits) -> Result<SurfaceReconcileJob, Self> {
        let Some(mut state) = self.state.take() else { return Err(self) };
        let (Some(mut current), Some(tree)) = (state.current.take(), state.source.take()) else {
            self.state = Some(state);
            return Err(self);
        };
        if let Some(credit) = state.credit.take() { release_surface_reconcile(credit); }
        if let Some(handback) = state.output_handback.take() { release_surface_reconcile_handback(handback); }
        current.handback = state.handback.take();
        match SurfaceReconcileJob::try_new_with_limits(current, tree, state.generation, limits) {
            Ok(job) => Ok(job),
            Err(mut rejected) => {
                if let Some(next) = rejected.state.as_mut() {
                    next.fault = state.fault.take();
                }
                Err(rejected)
            }
        }
    }

    pub fn take_sources(&mut self) -> Option<(SurfaceReconciler, ComponentTree)> {
        let state = self.state.as_mut()?;
        let mut current = state.current.take()?;
        let Some(tree) = state.source.take() else {
            state.current = Some(current);
            return None;
        };
        state.fault = None;
        if let Some(credit) = state.credit.take() { release_surface_reconcile(credit); }
        current.handback = state.handback.take();
        Some((current, tree))
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn close_step(&mut self) -> bool {
        self.state.as_mut().is_none_or(|state| state.close_admitted_step())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.as_ref().is_none_or(|state| state.terminal_is_empty())
    }

    pub fn into_terminal(mut self) -> SurfaceReconcileTerminal {
        SurfaceReconcileTerminal { state: self.state.take() }
    }
}

impl Drop for SurfaceReconcileRejected {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

/// 🧹️ Public fault/cancel/close authority; each close grant retires one retained owner.
pub struct SurfaceReconcileTerminal {
    state: Option<Box<SurfaceReconcileRetained>>,
}

impl SurfaceReconcileTerminal {
    #[expect(clippy::result_large_err, reason = "Rejected admission returns the exact reconciler, tree and reservation without a fallible allocation.")]
    pub fn try_from_reserved_sources(mut current: SurfaceReconciler, tree: ComponentTree, mut reservation: SurfaceReconcileReservation) -> Result<Self, (SurfaceReconciler, ComponentTree, SurfaceReconcileReservation)> {
        let generation = reservation.generation;
        let Some(handback) = acquire_reserved_surface_reconcile_handback(&mut current.handback, &mut reservation.handback, generation) else {
            return Err((current, tree, reservation));
        };
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: reservation.output_handback.take(),
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: Some(current),
                source: Some(tree),
                cursor: None,
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit: reservation.credit.take(),
                handback: Some(handback),
            })),
        })
    }

    #[expect(clippy::result_large_err, reason = "Rejected admission preserves the caller's exact reconciler without allocating on failure.")]
    pub fn try_from_reconciler(mut reconciler: SurfaceReconciler, generation: u64) -> Result<Self, SurfaceReconciler> {
        let Some(handback) = acquire_surface_reconcile_handback(&mut reconciler.handback, generation) else {
            return Err(reconciler);
        };
        let credit = None;
        Ok(Self {
            state: Some(Box::new(SurfaceReconcileRetained {
                output_handback: None,
                generation,
                phase: SurfaceReconcileJobPhase::Closing,
                current: Some(reconciler),
                source: None,
                cursor: None,
                candidate: None,
                patch: ui_contract::UiPendingPatch::default(),
                retire_tree: SurfaceTreeRetireCursor::default(),
                fault: None,
                usage: SurfaceReconcileUsage::default(),
                credit,
                handback: Some(handback),
            })),
        })
    }

    pub fn handback_key(&self) -> Option<SurfaceReconcileHandbackKey> {
        self.state.as_ref()?.handback.as_ref().map(|handback| handback.key)
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn fault(&self) -> Option<&SurfaceReconcileFault> {
        self.state.as_ref().and_then(|state| state.fault.as_ref())
    }

    pub fn resume(mut self, generation: u64) -> Result<SurfaceReconcileJob, Self> {
        let Some(state) = self.state.as_mut() else { return Err(self) };
        if state.generation != generation || state.cursor.is_none() || state.current.is_none() {
            return Err(self);
        }
        state.fault = None;
        state.cursor.as_mut().map(SurfaceReconcileCursor::clear_fault);
        state.phase = SurfaceReconcileJobPhase::Drive;
        Ok(SurfaceReconcileJob { state: self.state.take() })
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, SURFACE_RECONCILE_PAGE_BYTES)
    }

    /// 🧹️ Retires this terminal against the caller's PAGE grant — the same pricing
    /// [`SurfaceReconcileReadyPatch::close_step_with_grant`] and the pending-patch authority already
    /// take. A terminal holding a document-scaled retained surface keeps the reactor turn in
    /// `MoreWork`, and the host answers every `MoreWork` with one more round trip, so one owner per
    /// call is one host round trip per retained value (ticket 26/09/02, W-B2).
    pub fn close_step_on_original(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let Some(state) = self.state.as_mut() else { return Ok(true); };
        if state.close_native_resources_on_original(cx)? != Some(true) { return Ok(false); }
        if cx.should_yield() { return Ok(false); }
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(false); }
        let complete = state.close_admitted_run(1, grant.maximum_release_bytes.min(grant.maximum_copy_bytes));
        cx.consume_fuel(1);
        Ok(complete)
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> bool {
        self.state.as_mut().is_none_or(|state| state.close_admitted_run(items, bytes))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.as_ref().is_none_or(|state| state.terminal_is_empty())
    }
}

impl Drop for SurfaceReconcileTerminal {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            handback_surface_reconcile(state);
        }
    }
}

pub fn take_surface_reconcile_terminal(key: SurfaceReconcileHandbackKey) -> Result<Option<SurfaceReconcileTerminal>, &'static str> {
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(None),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    let Some(slot) = registry.slots.get_mut(key.slot) else { return Ok(None); };
    if !slot.reserved || slot.epoch != key.epoch || slot.generation != key.generation { return Ok(None); }
    if slot.state.as_ref().is_some_and(|state| state.handback.is_some()) { return Err("queued surface owner retains an external registry reservation"); }
    let Some(mut state) = slot.state.take() else { return Ok(None); };
    state.handback = Some(SurfaceReconcileHandbackReservation { key });
    Ok(Some(SurfaceReconcileTerminal { state: Some(state) }))
}

pub fn close_surface_reconcile_handback_one() -> Result<bool, &'static str> {
    static RESIDENT_TURN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if RESIDENT_TURN.fetch_xor(true, std::sync::atomic::Ordering::Relaxed) && ui_contract::UiResidentPermit::has_pending_returns() {
        ui_contract::UiResidentPermit::drain_one().map_err(|error| error.reason())?;
        return Ok(false);
    }
    let mut registry = match SURFACE_RECONCILE_HANDBACKS.try_lock() {
        Ok(registry) => registry,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(false),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("surface handback registry is poisoned"),
    };
    if registry.retirement_len == 0 { return Ok(!ui_contract::UiResidentPermit::has_pending_returns()); }
    let head = registry.retirement_head;
    let index = registry.retirement[head];
    {
        let slot = registry.slots.get_mut(index).ok_or("surface retirement queue contains an invalid slot")?;
        if !slot.queued { return Err("surface retirement queue lost its exact owner"); }
    }
    let mut owned = registry.slots[index].state.take();
    let complete = if let Some(state) = owned.as_mut() {
        if let Some(handback) = state.handback.take() {
            release_surface_reconcile_handback_in(&mut registry, handback);
        }
        if let Some(handback) = state.current.as_mut().and_then(|owner| owner.handback.take()) {
            release_surface_reconcile_handback_in(&mut registry, handback);
        }
        if let Some(handback) = state.candidate.as_mut().and_then(|owner| owner.handback.take()) {
            release_surface_reconcile_handback_in(&mut registry, handback);
        }
        if state.fault.is_none() && !state.patch.terminal_is_empty() {
            state.phase = SurfaceReconcileJobPhase::Closing;
            state.patch.close_step(1, SURFACE_RECONCILE_PAGE_BYTES)?;
            false
        } else { state.close_step_with(Some(&mut registry)) && state.terminal_is_empty() }
    } else { true };
    registry.slots[index].state = owned;
    if complete && (registry.slots[index].state.is_some() || !registry.slots[index].reserved) && registry.free_len >= SURFACE_RECONCILE_HANDBACK_SLOTS { return Err("surface handback free list exhausted"); }
    registry.retirement[head] = usize::MAX;
    registry.retirement_head = (head + 1) % SURFACE_RECONCILE_HANDBACK_SLOTS;
    registry.retirement_len -= 1;
    if complete {
        let slot = &mut registry.slots[index];
        slot.queued = false;
        if slot.state.is_some() {
            slot.state = None;
            slot.reserved = false;
            slot.generation = 0;
        }
        if !slot.reserved {
            let free = registry.free_len;
            registry.free[free] = index;
            registry.free_len += 1;
        }
    } else {
        let tail = (registry.retirement_head + registry.retirement_len) % SURFACE_RECONCILE_HANDBACK_SLOTS;
        registry.retirement[tail] = index;
        registry.retirement_len += 1;
    }
    Ok(registry.retirement_len == 0 && !ui_contract::UiResidentPermit::has_pending_returns())
}


/// 🔒️ Serializes laws that admit into the process-wide surface output, resident, and handback registries.
pub fn surface_reconcile_registry_test_guard() -> SurfaceReconcileRegistryTestGuard {
    SurfaceReconcileRegistryTestGuard::acquire()
}

pub struct SurfaceReconcileRegistryTestGuard {
    _lock: MutexGuard<'static, ()>,
}

impl SurfaceReconcileRegistryTestGuard {
    fn acquire() -> Self {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        let lock = GUARD.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        isolate_surface_reconcile_registries();
        Self { _lock: lock }
    }
}

impl Drop for SurfaceReconcileRegistryTestGuard {
    fn drop(&mut self) {
        isolate_surface_reconcile_registries();
    }
}

fn isolate_surface_reconcile_registries() {
    drain_surface_reconcile_registry_until_idle();
    reclaim_orphaned_handback_slots();
    drain_surface_reconcile_registry_until_idle();
}

/// ♻️ Rebuilds the handback free list from the slots that are genuinely unowned, and returns every
/// retained state it reclaimed so the drop happens outside the registry lock.
///
/// A RESERVED slot is never reclaimed. The registry is process-wide while
/// [`SurfaceReconcileRegistryTestGuard`]'s mutex only serialises the laws that take the guard, so a
/// blanket reset frees slots that a law running beside this one still holds: that owner's own release
/// then finds the free list already full and reports `surface handback free list exhausted` — an
/// accounting overflow, reported as if the pool had run dry, in whichever law happened to release
/// next. Reclaiming by state instead of by position keeps one owner's isolation from corrupting
/// another's, and the retirement ring is rebuilt from the entries whose slot survived rather than
/// cleared, so a live owner's queued retirement is not silently dropped either.
fn reclaim_orphaned_handback_slots() {
    let mut orphaned = Vec::new();
    {
        let mut registry = SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut reclaimed = [false; SURFACE_RECONCILE_HANDBACK_SLOTS];
        let mut free_len = 0;
        for index in (0..SURFACE_RECONCILE_HANDBACK_SLOTS).rev() {
            if registry.slots[index].reserved {
                continue;
            }
            if let Some(state) = registry.slots[index].state.take() {
                orphaned.push(state);
            }
            let epoch = registry.slots[index].epoch;
            registry.slots[index] = SurfaceReconcileHandbackSlot { epoch, ..SurfaceReconcileHandbackSlot::default() };
            registry.free[free_len] = index;
            reclaimed[index] = true;
            free_len += 1;
        }
        registry.free_len = free_len;
        let mut retirement = [usize::MAX; SURFACE_RECONCILE_HANDBACK_SLOTS];
        let mut retirement_len = 0;
        for offset in 0..registry.retirement_len {
            let slot = registry.retirement[(registry.retirement_head + offset) % SURFACE_RECONCILE_HANDBACK_SLOTS];
            if slot >= SURFACE_RECONCILE_HANDBACK_SLOTS || reclaimed[slot] {
                continue;
            }
            retirement[retirement_len] = slot;
            retirement_len += 1;
        }
        registry.retirement = retirement;
        registry.retirement_head = 0;
        registry.retirement_len = retirement_len;
    }
    drop(orphaned);
}

fn drain_surface_reconcile_registry_until_idle() {
    output::recover_output_registry_poison();
    drop(SURFACE_RECONCILE_HANDBACKS.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
    for _ in 0..(64 * 4 + SURFACE_RECONCILE_HANDBACK_SLOTS * 8) {
        let output_pending = output::output_registry_has_pending_returns();
        if output_pending {
            let _ = SurfaceReconcileOutputs::drain_one(1, SURFACE_RECONCILE_PAGE_BYTES);
        }
        let resident_pending = ui_contract::UiResidentPermit::has_pending_returns();
        if resident_pending {
            let _ = ui_contract::UiResidentPermit::drain_one();
        }
        let handback_idle = close_surface_reconcile_handback_one().unwrap_or(false);
        let value_idle = ui_contract::close_ui_value_page_one();
        let built_idle = ui_contract::close_built_node_page_one();
        if !output_pending && !resident_pending && handback_idle && value_idle && built_idle {
            break;
        }
    }
}

#[cfg(test)]
#[path = "../🚪️handback/🧪️tests/🚪️handback/🦀️.rs"]
mod handback_entry_tests;

//#endregion 🎟️RetainedAuthority

//#endregion ⏭️ResumableReconcile

//#region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️authority-unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
