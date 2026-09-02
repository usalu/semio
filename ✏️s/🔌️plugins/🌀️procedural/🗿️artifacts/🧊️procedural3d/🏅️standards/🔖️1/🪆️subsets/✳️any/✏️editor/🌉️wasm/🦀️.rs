//! 🌉️ Procedural3d retained document-load bridge (the mounted operation registry below; the
//! wasm-bindgen `WasmBridge` submodule that used to sit between `🔖️MountedRegistry` and
//! `🧪️MountedLaws` was deleted, along with the `MountedLaws` test assertions that verified its
//! JS-facing method-name completeness — nothing ever built the bridge for `wasm32-unknown-unknown`,
//! no engine entry, no `wasm` script target — see
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type Procedural3dEnvelope = ArtifactEnvelope<Procedural3dSnapshot, Procedural3dMutation>;
pub type Procedural3dStore = ArtifactStore<Procedural3dSnapshot, Procedural3dMutation>;
//#endregion 🔖️Store

//#region 🔖️MountedRegistry
const PROCEDURAL3D_ENVELOPE_MAXIMUM_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
const PROCEDURAL3D_ENVELOPE_MAXIMUM_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const PROCEDURAL3D_ENVELOPE_MAXIMUM_ITEMS: usize = 8_192;
const PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS: usize = 4;
const PROCEDURAL3D_ENVELOPE_CONTROL_CREDITS: usize = 1;
const PROCEDURAL3D_ENVELOPE_OPERATION_SLOTS: usize = 4;
const PROCEDURAL3D_OUTPUT_PAGE_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Procedural3dOutputKind {
    Progress = 0,
    Checkpoint = 1,
    Preview = 2,
    Terminal = 3,
}

impl Procedural3dOutputKind {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Progress),
            1 => Some(Self::Checkpoint),
            2 => Some(Self::Preview),
            3 => Some(Self::Terminal),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
struct Procedural3dOutputPage {
    operation: u64,
    generation: u64,
    sequence: u64,
    kind: Procedural3dOutputKind,
    len: usize,
    bytes: [u8; PROCEDURAL3D_OUTPUT_PAGE_BYTES],
}

impl Procedural3dOutputPage {
    fn new(operation: u64, generation: u64, sequence: u64, kind: Procedural3dOutputKind, status: u8, admitted_pages: usize, admitted_bytes: usize) -> Self {
        let mut bytes = [0; PROCEDURAL3D_OUTPUT_PAGE_BYTES];
        bytes[..4].copy_from_slice(b"P3DO");
        bytes[4] = 1;
        bytes[5] = kind as u8;
        bytes[6] = status;
        bytes[8..16].copy_from_slice(&operation.to_le_bytes());
        bytes[16..24].copy_from_slice(&generation.to_le_bytes());
        bytes[24..32].copy_from_slice(&sequence.to_le_bytes());
        bytes[32..40].copy_from_slice(&(admitted_pages as u64).to_le_bytes());
        bytes[40..48].copy_from_slice(&(admitted_bytes as u64).to_le_bytes());
        Self { operation, generation, sequence, kind, len: 48, bytes }
    }
}

struct Procedural3dOutputSlot {
    page: Option<Procedural3dOutputPage>,
    lease: Option<std::sync::Arc<std::sync::atomic::AtomicU8>>,
    acknowledged: bool,
}

impl Procedural3dOutputSlot {
    fn empty() -> Self {
        Self { page: None, lease: None, acknowledged: false }
    }

    fn reclaim_lost_lease(&mut self) {
        if self.lease.as_ref().is_some_and(|lease| lease.load(std::sync::atomic::Ordering::Acquire) == 2) {
            self.lease = None;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Procedural3dIngressCredits {
    maximum_pages: usize,
    maximum_bytes: usize,
    maximum_items: usize,
    maximum_output_pages: usize,
    maximum_controls: usize,
}

impl Procedural3dIngressCredits {
    fn try_new(maximum_pages: usize, maximum_bytes: usize, maximum_items: usize, maximum_output_pages: usize, maximum_controls: usize) -> Result<Self, &'static str> {
        if maximum_pages == 0 || maximum_pages > PROCEDURAL3D_ENVELOPE_MAXIMUM_PAGES {
            return Err("procedural3d-envelope.page-credits");
        }
        if maximum_bytes == 0 || maximum_bytes > PROCEDURAL3D_ENVELOPE_MAXIMUM_BYTES {
            return Err("procedural3d-envelope.byte-credits");
        }
        if maximum_items == 0 || maximum_items > PROCEDURAL3D_ENVELOPE_MAXIMUM_ITEMS {
            return Err("procedural3d-envelope.item-credits");
        }
        if maximum_output_pages != PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS {
            return Err("procedural3d-envelope.output-credits");
        }
        if maximum_controls != PROCEDURAL3D_ENVELOPE_CONTROL_CREDITS {
            return Err("procedural3d-envelope.control-credits");
        }
        Ok(Self { maximum_pages, maximum_bytes, maximum_items, maximum_output_pages, maximum_controls })
    }
}

struct Procedural3dMountedOperation {
    operation: u64,
    generation: u64,
    base_revision: u64,
    parent_revision: u64,
    credits: Procedural3dIngressCredits,
    admitted_pages: usize,
    admitted_bytes: usize,
    sequence: u64,
    outputs: [Procedural3dOutputSlot; PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS],
}

impl Procedural3dMountedOperation {
    fn matches(&self, operation: u64, generation: u64) -> bool {
        self.operation == operation && self.generation == generation
    }

    fn preflight_page(&self, bytes: usize) -> Result<(), &'static str> {
        let pages = self.admitted_pages.checked_add(1).ok_or("procedural3d-envelope.page-overflow")?;
        let total_bytes = self.admitted_bytes.checked_add(bytes).ok_or("procedural3d-envelope.byte-overflow")?;
        if bytes > PROCEDURAL3D_OUTPUT_PAGE_BYTES || pages > self.credits.maximum_pages || total_bytes > self.credits.maximum_bytes {
            return Err("procedural3d-envelope.page-handback");
        }
        Ok(())
    }

    fn publish(&mut self, kind: Procedural3dOutputKind, status: u8) {
        let slot = &mut self.outputs[kind as usize];
        slot.reclaim_lost_lease();
        if kind == Procedural3dOutputKind::Terminal && (slot.page.is_some() || slot.acknowledged) {
            return;
        }
        self.sequence = self.sequence.saturating_add(1);
        slot.page = Some(Procedural3dOutputPage::new(self.operation, self.generation, self.sequence, kind, status, self.admitted_pages, self.admitted_bytes));
        slot.acknowledged = false;
    }
}

struct Procedural3dOutputLease {
    page: Procedural3dOutputPage,
    signal: std::sync::Arc<std::sync::atomic::AtomicU8>,
    consumed: bool,
}

impl Drop for Procedural3dOutputLease {
    fn drop(&mut self) {
        if !self.consumed {
            self.signal.store(2, std::sync::atomic::Ordering::Release);
        }
    }
}

struct Procedural3dMountedRegistry {
    operations: [Option<Procedural3dMountedOperation>; PROCEDURAL3D_ENVELOPE_OPERATION_SLOTS],
}

impl Procedural3dMountedRegistry {
    fn new() -> Self {
        Self { operations: std::array::from_fn(|_| None) }
    }

    fn can_insert(&self) -> bool {
        self.operations.iter().any(Option::is_none)
    }

    fn insert(&mut self, operation: u64, generation: u64, base_revision: u64, parent_revision: u64, credits: Procedural3dIngressCredits) -> Result<(), &'static str> {
        if self.operations.iter().flatten().any(|entry| entry.operation == operation) {
            return Err("procedural3d-envelope.operation-duplicate");
        }
        let slot = self.operations.iter_mut().find(|slot| slot.is_none()).ok_or("procedural3d-envelope.operation-capacity")?;
        *slot = Some(Procedural3dMountedOperation { operation, generation, base_revision, parent_revision, credits, admitted_pages: 0, admitted_bytes: 0, sequence: 0, outputs: std::array::from_fn(|_| Procedural3dOutputSlot::empty()) });
        Ok(())
    }

    fn operation_mut(&mut self, operation: u64, generation: u64) -> Result<&mut Procedural3dMountedOperation, &'static str> {
        self.operations.iter_mut().flatten().find(|entry| entry.matches(operation, generation)).ok_or("procedural3d-envelope.stale-registry-handle")
    }

    fn operation(&self, operation: u64, generation: u64) -> Result<&Procedural3dMountedOperation, &'static str> {
        self.operations.iter().flatten().find(|entry| entry.matches(operation, generation)).ok_or("procedural3d-envelope.stale-registry-handle")
    }

    fn admit_page(&mut self, operation: u64, generation: u64, bytes: usize) -> Result<(), &'static str> {
        let entry = self.operation_mut(operation, generation)?;
        entry.preflight_page(bytes)?;
        entry.admitted_pages += 1;
        entry.admitted_bytes += bytes;
        Ok(())
    }

    fn publish(&mut self, operation: u64, generation: u64, kind: Procedural3dOutputKind, status: u8) -> Result<(), &'static str> {
        self.operation_mut(operation, generation)?.publish(kind, status);
        Ok(())
    }

    fn take(&mut self, operation: u64, generation: u64, kind: Procedural3dOutputKind) -> Result<Option<Procedural3dOutputLease>, &'static str> {
        let slot = &mut self.operation_mut(operation, generation)?.outputs[kind as usize];
        slot.reclaim_lost_lease();
        let Some(page) = slot.page.as_ref() else { return Ok(None) };
        if slot.lease.is_some() {
            return Ok(None);
        }
        let signal = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(0));
        slot.lease = Some(std::sync::Arc::clone(&signal));
        Ok(Some(Procedural3dOutputLease { page: *page, signal, consumed: false }))
    }

    fn resume(&mut self, lease: &mut Procedural3dOutputLease) -> Result<(), &'static str> {
        let slot = &mut self.operation_mut(lease.page.operation, lease.page.generation)?.outputs[lease.page.kind as usize];
        if !slot.lease.as_ref().is_some_and(|signal| std::sync::Arc::ptr_eq(signal, &lease.signal)) {
            return Err("procedural3d-envelope.output-resume-stale");
        }
        lease.signal.store(1, std::sync::atomic::Ordering::Release);
        lease.consumed = true;
        slot.lease = None;
        Ok(())
    }

    fn acknowledge_output(&mut self, lease: &mut Procedural3dOutputLease) -> Result<(), &'static str> {
        let slot = &mut self.operation_mut(lease.page.operation, lease.page.generation)?.outputs[lease.page.kind as usize];
        let same_page = slot.page.as_ref().is_some_and(|page| page.sequence == lease.page.sequence && page.bytes[..page.len] == lease.page.bytes[..lease.page.len]);
        if !same_page || !slot.lease.as_ref().is_some_and(|signal| std::sync::Arc::ptr_eq(signal, &lease.signal)) {
            return Err("procedural3d-envelope.output-ack-stale");
        }
        lease.signal.store(3, std::sync::atomic::Ordering::Release);
        lease.consumed = true;
        slot.lease = None;
        slot.page = None;
        slot.acknowledged = true;
        Ok(())
    }

    fn terminal_acknowledged(&mut self, operation: u64, generation: u64) -> Result<bool, &'static str> {
        let slot = &mut self.operation_mut(operation, generation)?.outputs[Procedural3dOutputKind::Terminal as usize];
        slot.reclaim_lost_lease();
        Ok(slot.acknowledged && slot.page.is_none() && slot.lease.is_none())
    }

    fn prepare_load_acknowledgement(&mut self, operation: u64, generation: u64) -> Result<bool, &'static str> {
        if !self.terminal_acknowledged(operation, generation)? {
            return Ok(false);
        }
        let entry = self.operation_mut(operation, generation)?;
        for output in &mut entry.outputs[..Procedural3dOutputKind::Terminal as usize] {
            output.reclaim_lost_lease();
            if output.lease.is_some() {
                return Ok(false);
            }
        }
        for output in &mut entry.outputs[..Procedural3dOutputKind::Terminal as usize] {
            output.page = None;
            output.acknowledged = true;
        }
        Ok(true)
    }

    fn remove(&mut self, operation: u64, generation: u64) -> Result<(), &'static str> {
        let slot = self.operations.iter_mut().find(|slot| slot.as_ref().is_some_and(|entry| entry.matches(operation, generation))).ok_or("procedural3d-envelope.remove-stale")?;
        if slot.as_ref().is_some_and(|entry| entry.outputs.iter().any(|output| output.page.is_some() || output.lease.is_some())) {
            return Err("procedural3d-envelope.remove-populated");
        }
        *slot = None;
        Ok(())
    }

    fn close_step(&mut self) -> bool {
        let Some(operation) = self.operations.iter_mut().flatten().next() else { return true };
        for output in &mut operation.outputs {
            output.reclaim_lost_lease();
            if output.lease.is_some() {
                return false;
            }
            if output.page.take().is_some() {
                output.acknowledged = true;
                return false;
            }
        }
        let operation_id = operation.operation;
        let generation = operation.generation;
        let _ = crate::artifacts::procedural3d::spr::procedural3d_release_publication_authority(semio_framework_job::OperationId(operation_id), semio_framework_job::Generation(generation));
        let slot = self.operations.iter_mut().find(|slot| slot.as_ref().is_some_and(|entry| entry.matches(operation_id, generation))).expect("Procedural3d close operation remains retained");
        *slot = None;
        self.operations.iter().all(Option::is_none)
    }

    fn terminal_is_empty(&self) -> bool {
        self.operations.iter().all(Option::is_none)
    }
}

impl Drop for Procedural3dMountedRegistry {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Procedural3d mounted registry reached Drop before terminal-empty close");
    }
}
//#endregion 🔖️MountedRegistry

//#region 🧪️MountedLaws
#[cfg(test)]
mod mounted_laws {
    use std::cell::Cell;

    use super::*;

    fn exact_credits(maximum_pages: usize, maximum_bytes: usize) -> Procedural3dIngressCredits {
        Procedural3dIngressCredits::try_new(maximum_pages, maximum_bytes, PROCEDURAL3D_ENVELOPE_MAXIMUM_ITEMS, PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS, PROCEDURAL3D_ENVELOPE_CONTROL_CREDITS).expect("exact Procedural3d mounted credits")
    }

    fn insert(registry: &mut Procedural3dMountedRegistry, operation: u64, generation: u64, credits: Procedural3dIngressCredits) {
        registry.insert(operation, generation, generation, generation, credits).expect("Procedural3d mounted operation admission");
    }

    #[test]
    fn every_credit_rejects_zero_and_maximum_plus_one_before_operation_construction() {
        let valid = (PROCEDURAL3D_ENVELOPE_MAXIMUM_PAGES, PROCEDURAL3D_ENVELOPE_MAXIMUM_BYTES, PROCEDURAL3D_ENVELOPE_MAXIMUM_ITEMS, PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS, PROCEDURAL3D_ENVELOPE_CONTROL_CREDITS);
        assert!(Procedural3dIngressCredits::try_new(0, valid.1, valid.2, valid.3, valid.4).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0 + 1, valid.1, valid.2, valid.3, valid.4).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, 0, valid.2, valid.3, valid.4).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1 + 1, valid.2, valid.3, valid.4).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, 0, valid.3, valid.4).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, valid.2 + 1, valid.3, valid.4).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, valid.2, 0, valid.4).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3 + 1, valid.4).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3, 0).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3, valid.4 + 1).is_err());
        assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3, valid.4).is_ok());
    }

    #[test]
    fn repeated_rejected_controls_do_not_consume_operation_credits() {
        let valid = (PROCEDURAL3D_ENVELOPE_MAXIMUM_PAGES, PROCEDURAL3D_ENVELOPE_MAXIMUM_BYTES, PROCEDURAL3D_ENVELOPE_MAXIMUM_ITEMS, PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS, PROCEDURAL3D_ENVELOPE_CONTROL_CREDITS);
        for _ in 0..64 {
            assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3, 0).is_err());
            assert!(Procedural3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3, valid.4 + 1).is_err());
        }
        let mut registry = Procedural3dMountedRegistry::new();
        let credits = exact_credits(1, PROCEDURAL3D_OUTPUT_PAGE_BYTES);
        for operation in 1..=PROCEDURAL3D_ENVELOPE_OPERATION_SLOTS as u64 {
            insert(&mut registry, operation, 70, credits);
        }
        for operation in 1..=PROCEDURAL3D_ENVELOPE_OPERATION_SLOTS as u64 {
            registry.remove(operation, 70).expect("rejected controls preserved every operation credit");
        }
        assert!(registry.terminal_is_empty());
    }

    #[test]
    fn operation_and_page_maximum_plus_one_leave_producers_unconstructed() {
        let mut registry = Procedural3dMountedRegistry::new();
        let credits = exact_credits(1, PROCEDURAL3D_OUTPUT_PAGE_BYTES);
        for operation in 1..=PROCEDURAL3D_ENVELOPE_OPERATION_SLOTS as u64 {
            insert(&mut registry, operation, 7, credits);
        }
        assert_eq!(registry.insert(99, 7, 7, 7, credits), Err("procedural3d-envelope.operation-capacity"));
        assert!(registry.operation(1, 8).is_err(), "stale generation handle rejected");
        assert!(registry.operation(99, 7).is_err(), "wrong operation handle rejected");
        assert_eq!(registry.insert(1, 8, 8, 8, credits), Err("procedural3d-envelope.operation-duplicate"));

        let producer_calls = Cell::new(0);
        registry.operation(1, 7).expect("first operation").preflight_page(PROCEDURAL3D_OUTPUT_PAGE_BYTES).expect("exact page");
        producer_calls.set(producer_calls.get() + 1);
        registry.admit_page(1, 7, PROCEDURAL3D_OUTPUT_PAGE_BYTES).expect("exact page handoff");
        assert_eq!(registry.operation(1, 7).expect("first operation").preflight_page(1), Err("procedural3d-envelope.page-handback"));
        assert_eq!(producer_calls.get(), 1, "maximum-plus-one page never enters the producer");

        for operation in 1..=PROCEDURAL3D_ENVELOPE_OPERATION_SLOTS as u64 {
            registry.remove(operation, 7).expect("empty operation retirement");
        }
    }

    #[test]
    fn outputs_are_bounded_latest_wins_with_lossless_terminal_take_resume_and_ack() {
        let mut registry = Procedural3dMountedRegistry::new();
        insert(&mut registry, 21, 8, exact_credits(1, PROCEDURAL3D_OUTPUT_PAGE_BYTES));

        registry.publish(21, 8, Procedural3dOutputKind::Progress, 0).expect("initial progress");
        let first = registry.operation(21, 8).expect("operation").outputs[Procedural3dOutputKind::Progress as usize].page.as_ref().expect("progress").sequence;
        registry.publish(21, 8, Procedural3dOutputKind::Progress, 1).expect("latest progress");
        let latest = registry.operation(21, 8).expect("operation").outputs[Procedural3dOutputKind::Progress as usize].page.as_ref().expect("progress").sequence;
        assert!(latest > first);

        let mut progress = registry.take(21, 8, Procedural3dOutputKind::Progress).expect("take").expect("progress lease");
        let leased_progress = progress.page.sequence;
        registry.publish(21, 8, Procedural3dOutputKind::Progress, 2).expect("latest progress while leased");
        registry.resume(&mut progress).expect("resume");
        drop(progress);
        let mut progress = registry.take(21, 8, Procedural3dOutputKind::Progress).expect("retake").expect("progress lease");
        assert!(progress.page.sequence > leased_progress);
        registry.acknowledge_output(&mut progress).expect("progress ACK");
        drop(progress);

        registry.publish(21, 8, Procedural3dOutputKind::Checkpoint, 1).expect("checkpoint");
        let mut checkpoint = registry.take(21, 8, Procedural3dOutputKind::Checkpoint).expect("take").expect("checkpoint lease");
        let checkpoint_sequence = checkpoint.page.sequence;
        registry.resume(&mut checkpoint).expect("checkpoint resume");
        drop(checkpoint);
        let mut checkpoint = registry.take(21, 8, Procedural3dOutputKind::Checkpoint).expect("retake").expect("checkpoint lease");
        assert_eq!(checkpoint.page.sequence, checkpoint_sequence);
        registry.acknowledge_output(&mut checkpoint).expect("checkpoint ACK");
        drop(checkpoint);

        registry.publish(21, 8, Procedural3dOutputKind::Terminal, 2).expect("terminal");
        let terminal_sequence = registry.operation(21, 8).expect("operation").outputs[Procedural3dOutputKind::Terminal as usize].page.as_ref().expect("terminal").sequence;
        registry.publish(21, 8, Procedural3dOutputKind::Terminal, 4).expect("lossless terminal retry");
        assert_eq!(registry.operation(21, 8).expect("operation").outputs[Procedural3dOutputKind::Terminal as usize].page.as_ref().expect("terminal").sequence, terminal_sequence,);
        drop(registry.take(21, 8, Procedural3dOutputKind::Terminal).expect("take").expect("lost terminal lease"));
        let mut terminal = registry.take(21, 8, Procedural3dOutputKind::Terminal).expect("retake").expect("terminal retained after handle loss");
        assert!(!registry.terminal_acknowledged(21, 8).expect("terminal state"));
        registry.acknowledge_output(&mut terminal).expect("terminal ACK");
        drop(terminal);
        registry.publish(21, 8, Procedural3dOutputKind::Terminal, 4).expect("terminal remains acknowledged");
        assert!(registry.operation(21, 8).expect("operation").outputs[Procedural3dOutputKind::Terminal as usize].page.is_none());
        assert!(registry.prepare_load_acknowledgement(21, 8).expect("load ACK preflight"));
        registry.remove(21, 8).expect("terminal-empty removal");
    }

    #[test]
    fn complete_before_ack_and_interrupted_close_retain_owners() {
        let credits = exact_credits(1, PROCEDURAL3D_OUTPUT_PAGE_BYTES);
        let mut registry = Procedural3dMountedRegistry::new();
        insert(&mut registry, 31, 9, credits);
        registry.publish(31, 9, Procedural3dOutputKind::Terminal, 2).expect("terminal");
        assert_eq!(registry.remove(31, 9), Err("procedural3d-envelope.remove-populated"));
        assert!(!registry.prepare_load_acknowledgement(31, 9).expect("complete remains unacknowledged"));
        let mut terminal = registry.take(31, 9, Procedural3dOutputKind::Terminal).expect("take").expect("terminal");
        registry.acknowledge_output(&mut terminal).expect("terminal ACK");
        drop(terminal);
        assert!(registry.prepare_load_acknowledgement(31, 9).expect("load ACK ready"));
        registry.remove(31, 9).expect("complete operation removal");

        insert(&mut registry, 32, 9, credits);
        for kind in [Procedural3dOutputKind::Progress, Procedural3dOutputKind::Checkpoint, Procedural3dOutputKind::Preview, Procedural3dOutputKind::Terminal] {
            registry.publish(32, 9, kind, 3).expect("close fixture output");
        }
        assert!(!registry.close_step(), "close releases at most one retained owner per call");
        for _ in 0..PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS + 2 {
            if registry.close_step() {
                break;
            }
        }
        assert!(registry.terminal_is_empty());
        assert!(registry.close_step(), "terminal-empty close is idempotent");
        assert!(registry.close_step(), "repeated terminal-empty close is idempotent");
    }

    #[test]
    fn populated_ordinary_drop_panics_until_hostile_close_drains() {
        let dropped = std::panic::catch_unwind(|| {
            let mut registry = Procedural3dMountedRegistry::new();
            insert(&mut registry, 33, 10, exact_credits(1, PROCEDURAL3D_OUTPUT_PAGE_BYTES));
            registry.publish(33, 10, Procedural3dOutputKind::Terminal, 4).expect("fault terminal");
        });
        assert!(dropped.is_err(), "populated ordinary drop must remain fail-loud");

        let mut registry = Procedural3dMountedRegistry::new();
        insert(&mut registry, 34, 10, exact_credits(1, PROCEDURAL3D_OUTPUT_PAGE_BYTES));
        registry.publish(34, 10, Procedural3dOutputKind::Terminal, 3).expect("cancel terminal");
        for _ in 0..PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS + 2 {
            if registry.close_step() {
                break;
            }
        }
        assert!(registry.terminal_is_empty());
        assert!(registry.close_step());
    }

    #[test]
    fn authoritative_publication_rejects_stale_generation_aba_and_parent() {
        use semio_framework_job::{Generation, OperationId};

        let operation = OperationId(u64::MAX - 71);
        assert_eq!(
            crate::artifacts::procedural3d::spr::procedural3d_admit_publication_authority(operation, Generation(41), 41, 40, 41, PROCEDURAL3D_ENVELOPE_MAXIMUM_ITEMS, PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS, PROCEDURAL3D_ENVELOPE_CONTROL_CREDITS),
            Err("procedural3d-publication.initial-freshness")
        );
        assert!(crate::artifacts::procedural3d::spr::procedural3d_admit_publication_authority(operation, Generation(41), 41, 41, 41, PROCEDURAL3D_ENVELOPE_MAXIMUM_ITEMS, PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS, PROCEDURAL3D_ENVELOPE_CONTROL_CREDITS,)
            .is_ok());
        assert_eq!(crate::artifacts::procedural3d::spr::procedural3d_validate_publication_authority(operation, Generation(41)), Ok((41, 41)));
        assert_eq!(crate::artifacts::procedural3d::spr::procedural3d_validate_atomic_publication_authority(OperationId(operation.0 + 1), Generation(41), Generation(41)), Err("procedural3d-publication.wrong-operation"));
        assert_eq!(crate::artifacts::procedural3d::spr::procedural3d_validate_atomic_publication_authority(operation, Generation(42), Generation(41)), Err("procedural3d-publication.wrong-generation"));
        crate::artifacts::procedural3d::spr::procedural3d_refresh_publication_authority(operation, Generation(41), 42).expect("authoritative live revision refresh");
        assert_eq!(crate::artifacts::procedural3d::spr::procedural3d_validate_atomic_publication_authority(operation, Generation(41), Generation(42)), Err("procedural3d-publication.wrong-base"));
        assert!(crate::artifacts::procedural3d::spr::procedural3d_release_publication_authority(operation, Generation(41)));

        assert!(crate::artifacts::procedural3d::spr::procedural3d_admit_publication_authority(operation, Generation(42), 42, 42, 42, PROCEDURAL3D_ENVELOPE_MAXIMUM_ITEMS, PROCEDURAL3D_ENVELOPE_OUTPUT_CHANNELS, PROCEDURAL3D_ENVELOPE_CONTROL_CREDITS,)
            .is_ok());
        assert!(crate::artifacts::procedural3d::spr::procedural3d_validate_publication_authority(operation, Generation(41)).is_err());
        assert_eq!(crate::artifacts::procedural3d::spr::procedural3d_validate_publication_authority(operation, Generation(42)), Ok((42, 42)));
        assert_eq!(crate::artifacts::procedural3d::spr::procedural3d_validate_atomic_publication_authority(operation, Generation(42), Generation(42)), Ok(()));
        assert!(crate::artifacts::procedural3d::spr::procedural3d_release_publication_authority(operation, Generation(42)));
    }

    #[test]
    fn domain_local_static_verifier_rejects_raw_routes_and_proves_three_dimensional_coverage() {
        let owner_source = include_str!("../../🧬️schema/🧬️mutations/💾️binary/🦀️.rs");
        let snapshot_source = include_str!("../../🧬️schema/📸️snapshot/💾️binary/🦀️.rs");
        let lifecycle_fixture = include_str!("../../🧪️tests/🔣️p8yz-b-retained-mounted-laws.json");
        let owner_fixture = include_str!("../../🧪️tests/🔣️p8yz-b-owner-catalog-laws.json");
        let oracle_fixture = include_str!("../../🧪️tests/🔣️p8yz-b-third-party-oracle-laws.json");

        assert!(owner_source.contains("mutation.delete-widget-position.3d-only"));
        assert!(owner_source.contains("PROCEDURAL3D_RETAINED_SCHEMA_DISCRIMINATOR"));
        let mounted_snapshot = snapshot_source.split_once("//#region 🔖️MountedCanonicalPackSession").expect("P3 mounted snapshot region").1.split_once("#[cfg(test)]\nmod retained_mounted_laws").expect("P3 mounted production boundary").0;
        let forbidden_whole_routes = [
            "OwnedSchemaHexAuthority",
            "hex::decode",
            "decode_hex",
            "from_hex",
            "ArtifactPack",
            "Vec<u8>",
            "collect::<Vec<u8>>",
            "decode_pack",
            "decode_document",
            "RecordValue",
            "serde_json::from_slice",
            "serde_json::from_str",
            "serde_json::from_value",
            "ArtifactStore::new",
            ".diff(",
            "::diff(",
            ".apply(",
            "::apply(",
            ".clone(",
        ];
        let mounted_field = owner_source.split_once("struct Procedural3dPackSnapshotAuthority").expect("P3 mounted envelope snapshot authority").1.split_once("enum Procedural3dMutationDecodeState").expect("P3 mounted snapshot authority boundary").0;
        let mounted_mutation =
            owner_source.split_once("struct Procedural3dMutationDecodeAuthority").expect("P3 mounted mutation authority").1.split_once("struct Procedural3dRejectedConflictAuthority").expect("P3 mounted mutation authority boundary").0;
        for forbidden in forbidden_whole_routes {
            assert!(!mounted_snapshot.contains(forbidden), "mounted P3 typed snapshot route regained a whole decoder edge: {forbidden}");
            assert!(!mounted_field.contains(forbidden), "mounted P3 envelope authority regained a whole decode edge: {forbidden}");
            assert!(!mounted_mutation.contains(forbidden), "mounted P3 mutation authority regained a whole decode edge: {forbidden}");
        }
        for required in ["RetainedPackSourceCursor", "RetainedPackAnchorCursor", "RetainedPackSegmentCursor", "RetainedPackCatalogCursor", "RetainedValueCursor"] {
            assert!(mounted_snapshot.contains(required), "mounted P3 route lost retained canonical layer: {required}");
        }
        assert!(snapshot_source.contains("one scalar byte opportunity"));
        assert!(owner_source.contains("PROCEDURAL3D_RETAINED_COMBINED_DEPTH: usize = 12"));
        assert!(snapshot_source.contains("*b\"P3D3\""));
        assert!(owner_source.contains("*b\"P2D2\""));
        assert!(owner_source.contains("cx.should_yield()"));
        assert!(owner_source.contains("cx.fuel_remaining() == 0"));
        assert!(lifecycle_fixture.contains("complete-before-ack"));
        assert!(lifecycle_fixture.contains("repeatedRejectedControlsDoNotConsumeCredits"));
        assert!(lifecycle_fixture.contains("terminal-empty-idempotent"));
        assert!(owner_fixture.contains("change-generation-value"));
        assert!(owner_fixture.contains("\"combinedDepth\": 12"));
        assert!(owner_fixture.contains("generic-clone"));
        assert!(oracle_fixture.contains("serde_json"));
        assert!(oracle_fixture.contains("Procedural3dSemanticOracle"));
        assert!(oracle_fixture.contains("\"runtimeDependency\": false"));
        assert!(crate::artifacts::procedural3d::spr::procedural3d_retained_catalog_is_complete());
    }
}
//#endregion 🧪️MountedLaws
