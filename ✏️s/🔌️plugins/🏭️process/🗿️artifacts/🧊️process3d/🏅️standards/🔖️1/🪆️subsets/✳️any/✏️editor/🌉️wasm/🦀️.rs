//! 🌉️ Process3d retained document-load bridge (the mounted operation registry below; the
//! wasm-bindgen `WasmBridge` submodule that used to sit between `🔖️MountedRegistry` and
//! `🧪️MountedLaws` was deleted — nothing ever built it for `wasm32-unknown-unknown`, no engine
//! entry, no `wasm` script target — see
//! `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`).

use crate::artifacts::process3d::op::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type Process3dEnvelope = ArtifactEnvelope<Process3dSnapshot, Process3dMutation>;
pub type Process3dStore = ArtifactStore<Process3dSnapshot, Process3dMutation>;
//#endregion 🔖️Store

//#region 🔖️MountedRegistry
const PROCESS3D_ENVELOPE_MAXIMUM_PAGES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES;
const PROCESS3D_ENVELOPE_MAXIMUM_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES;
const PROCESS3D_ENVELOPE_MAXIMUM_ITEMS: usize = 8_192;
const PROCESS3D_ENVELOPE_OUTPUT_CHANNELS: usize = 4;
const PROCESS3D_ENVELOPE_CONTROL_CREDITS: usize = 1;
const PROCESS3D_ENVELOPE_OPERATION_SLOTS: usize = 4;
const PROCESS3D_OUTPUT_PAGE_BYTES: usize = store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Process3dOutputKind {
    Progress = 0,
    Checkpoint = 1,
    Preview = 2,
    Terminal = 3,
}

impl Process3dOutputKind {
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
struct Process3dOutputPage {
    operation: u64,
    generation: u64,
    sequence: u64,
    kind: Process3dOutputKind,
    len: usize,
    bytes: [u8; PROCESS3D_OUTPUT_PAGE_BYTES],
}

impl Process3dOutputPage {
    fn new(operation: u64, generation: u64, sequence: u64, kind: Process3dOutputKind, status: u8, admitted_pages: usize, admitted_bytes: usize) -> Self {
        let mut bytes = [0; PROCESS3D_OUTPUT_PAGE_BYTES];
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

struct Process3dOutputSlot {
    page: Option<Process3dOutputPage>,
    lease: Option<std::sync::Arc<std::sync::atomic::AtomicU8>>,
    acknowledged: bool,
}

impl Process3dOutputSlot {
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
struct Process3dIngressCredits {
    maximum_pages: usize,
    maximum_bytes: usize,
    maximum_items: usize,
    maximum_output_pages: usize,
    maximum_controls: usize,
}

impl Process3dIngressCredits {
    fn try_new(maximum_pages: usize, maximum_bytes: usize, maximum_items: usize, maximum_output_pages: usize, maximum_controls: usize) -> Result<Self, &'static str> {
        if maximum_pages == 0 || maximum_pages > PROCESS3D_ENVELOPE_MAXIMUM_PAGES {
            return Err("process3d-envelope.page-credits");
        }
        if maximum_bytes == 0 || maximum_bytes > PROCESS3D_ENVELOPE_MAXIMUM_BYTES {
            return Err("process3d-envelope.byte-credits");
        }
        if maximum_items == 0 || maximum_items > PROCESS3D_ENVELOPE_MAXIMUM_ITEMS {
            return Err("process3d-envelope.item-credits");
        }
        if maximum_output_pages != PROCESS3D_ENVELOPE_OUTPUT_CHANNELS {
            return Err("process3d-envelope.output-credits");
        }
        if maximum_controls != PROCESS3D_ENVELOPE_CONTROL_CREDITS {
            return Err("process3d-envelope.control-credits");
        }
        Ok(Self { maximum_pages, maximum_bytes, maximum_items, maximum_output_pages, maximum_controls })
    }
}

struct Process3dMountedOperation {
    operation: u64,
    generation: u64,
    base_revision: u64,
    parent_revision: u64,
    credits: Process3dIngressCredits,
    admitted_pages: usize,
    admitted_bytes: usize,
    sequence: u64,
    outputs: [Process3dOutputSlot; PROCESS3D_ENVELOPE_OUTPUT_CHANNELS],
}

impl Process3dMountedOperation {
    fn matches(&self, operation: u64, generation: u64) -> bool {
        self.operation == operation && self.generation == generation
    }

    fn preflight_page(&self, bytes: usize) -> Result<(), &'static str> {
        let pages = self.admitted_pages.checked_add(1).ok_or("process3d-envelope.page-overflow")?;
        let total_bytes = self.admitted_bytes.checked_add(bytes).ok_or("process3d-envelope.byte-overflow")?;
        if bytes > PROCESS3D_OUTPUT_PAGE_BYTES || pages > self.credits.maximum_pages || total_bytes > self.credits.maximum_bytes {
            return Err("process3d-envelope.page-handback");
        }
        Ok(())
    }

    fn publish(&mut self, kind: Process3dOutputKind, status: u8) {
        let slot = &mut self.outputs[kind as usize];
        slot.reclaim_lost_lease();
        if kind == Process3dOutputKind::Terminal && (slot.page.is_some() || slot.acknowledged) {
            return;
        }
        self.sequence = self.sequence.saturating_add(1);
        slot.page = Some(Process3dOutputPage::new(self.operation, self.generation, self.sequence, kind, status, self.admitted_pages, self.admitted_bytes));
        slot.acknowledged = false;
    }
}

struct Process3dOutputLease {
    page: Process3dOutputPage,
    signal: std::sync::Arc<std::sync::atomic::AtomicU8>,
    consumed: bool,
}

impl Drop for Process3dOutputLease {
    fn drop(&mut self) {
        if !self.consumed {
            self.signal.store(2, std::sync::atomic::Ordering::Release);
        }
    }
}

struct Process3dMountedRegistry {
    operations: [Option<Process3dMountedOperation>; PROCESS3D_ENVELOPE_OPERATION_SLOTS],
}

impl Process3dMountedRegistry {
    fn new() -> Self {
        Self { operations: std::array::from_fn(|_| None) }
    }

    fn can_insert(&self) -> bool {
        self.operations.iter().any(Option::is_none)
    }

    fn insert(&mut self, operation: u64, generation: u64, base_revision: u64, parent_revision: u64, credits: Process3dIngressCredits) -> Result<(), &'static str> {
        if self.operations.iter().flatten().any(|entry| entry.operation == operation) {
            return Err("process3d-envelope.operation-duplicate");
        }
        let slot = self.operations.iter_mut().find(|slot| slot.is_none()).ok_or("process3d-envelope.operation-capacity")?;
        *slot = Some(Process3dMountedOperation { operation, generation, base_revision, parent_revision, credits, admitted_pages: 0, admitted_bytes: 0, sequence: 0, outputs: std::array::from_fn(|_| Process3dOutputSlot::empty()) });
        Ok(())
    }

    fn operation_mut(&mut self, operation: u64, generation: u64) -> Result<&mut Process3dMountedOperation, &'static str> {
        self.operations.iter_mut().flatten().find(|entry| entry.matches(operation, generation)).ok_or("process3d-envelope.stale-registry-handle")
    }

    fn operation(&self, operation: u64, generation: u64) -> Result<&Process3dMountedOperation, &'static str> {
        self.operations.iter().flatten().find(|entry| entry.matches(operation, generation)).ok_or("process3d-envelope.stale-registry-handle")
    }

    fn admit_page(&mut self, operation: u64, generation: u64, bytes: usize) -> Result<(), &'static str> {
        let entry = self.operation_mut(operation, generation)?;
        entry.preflight_page(bytes)?;
        entry.admitted_pages += 1;
        entry.admitted_bytes += bytes;
        Ok(())
    }

    fn publish(&mut self, operation: u64, generation: u64, kind: Process3dOutputKind, status: u8) -> Result<(), &'static str> {
        self.operation_mut(operation, generation)?.publish(kind, status);
        Ok(())
    }

    fn take(&mut self, operation: u64, generation: u64, kind: Process3dOutputKind) -> Result<Option<Process3dOutputLease>, &'static str> {
        let slot = &mut self.operation_mut(operation, generation)?.outputs[kind as usize];
        slot.reclaim_lost_lease();
        let Some(page) = slot.page.as_ref() else { return Ok(None) };
        if slot.lease.is_some() {
            return Ok(None);
        }
        let signal = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(0));
        slot.lease = Some(std::sync::Arc::clone(&signal));
        Ok(Some(Process3dOutputLease { page: *page, signal, consumed: false }))
    }

    fn resume(&mut self, lease: &mut Process3dOutputLease) -> Result<(), &'static str> {
        let slot = &mut self.operation_mut(lease.page.operation, lease.page.generation)?.outputs[lease.page.kind as usize];
        if !slot.lease.as_ref().is_some_and(|signal| std::sync::Arc::ptr_eq(signal, &lease.signal)) {
            return Err("process3d-envelope.output-resume-stale");
        }
        lease.signal.store(1, std::sync::atomic::Ordering::Release);
        lease.consumed = true;
        slot.lease = None;
        Ok(())
    }

    fn acknowledge_output(&mut self, lease: &mut Process3dOutputLease) -> Result<(), &'static str> {
        let slot = &mut self.operation_mut(lease.page.operation, lease.page.generation)?.outputs[lease.page.kind as usize];
        let same_page = slot.page.as_ref().is_some_and(|page| page.sequence == lease.page.sequence && page.bytes[..page.len] == lease.page.bytes[..lease.page.len]);
        if !same_page || !slot.lease.as_ref().is_some_and(|signal| std::sync::Arc::ptr_eq(signal, &lease.signal)) {
            return Err("process3d-envelope.output-ack-stale");
        }
        lease.signal.store(3, std::sync::atomic::Ordering::Release);
        lease.consumed = true;
        slot.lease = None;
        slot.page = None;
        slot.acknowledged = true;
        Ok(())
    }

    fn terminal_acknowledged(&mut self, operation: u64, generation: u64) -> Result<bool, &'static str> {
        let slot = &mut self.operation_mut(operation, generation)?.outputs[Process3dOutputKind::Terminal as usize];
        slot.reclaim_lost_lease();
        Ok(slot.acknowledged && slot.page.is_none() && slot.lease.is_none())
    }

    fn prepare_load_acknowledgement(&mut self, operation: u64, generation: u64) -> Result<bool, &'static str> {
        if !self.terminal_acknowledged(operation, generation)? {
            return Ok(false);
        }
        let entry = self.operation_mut(operation, generation)?;
        for output in &mut entry.outputs[..Process3dOutputKind::Terminal as usize] {
            output.reclaim_lost_lease();
            if output.lease.is_some() {
                return Ok(false);
            }
        }
        for output in &mut entry.outputs[..Process3dOutputKind::Terminal as usize] {
            output.page = None;
            output.acknowledged = true;
        }
        Ok(true)
    }

    fn remove(&mut self, operation: u64, generation: u64) -> Result<(), &'static str> {
        let slot = self.operations.iter_mut().find(|slot| slot.as_ref().is_some_and(|entry| entry.matches(operation, generation))).ok_or("process3d-envelope.remove-stale")?;
        if slot.as_ref().is_some_and(|entry| entry.outputs.iter().any(|output| output.page.is_some() || output.lease.is_some())) {
            return Err("process3d-envelope.remove-populated");
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
        let _ = crate::artifacts::process3d::spr::process3d_release_publication_authority(semio_framework_job::OperationId(operation_id), semio_framework_job::Generation(generation));
        let slot = self.operations.iter_mut().find(|slot| slot.as_ref().is_some_and(|entry| entry.matches(operation_id, generation))).expect("Process3d close operation remains retained");
        *slot = None;
        self.operations.iter().all(Option::is_none)
    }

    fn terminal_is_empty(&self) -> bool {
        self.operations.iter().all(Option::is_none)
    }
}

impl Drop for Process3dMountedRegistry {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Process3d mounted registry reached Drop before terminal-empty close");
    }
}
//#endregion 🔖️MountedRegistry

//#region 🧪️MountedLaws
#[cfg(test)]
mod mounted_laws {
    use std::cell::Cell;

    use super::*;

    fn exact_credits(maximum_pages: usize, maximum_bytes: usize) -> Process3dIngressCredits {
        Process3dIngressCredits::try_new(maximum_pages, maximum_bytes, PROCESS3D_ENVELOPE_MAXIMUM_ITEMS, PROCESS3D_ENVELOPE_OUTPUT_CHANNELS, PROCESS3D_ENVELOPE_CONTROL_CREDITS).expect("exact Process3d mounted credits")
    }

    fn insert(registry: &mut Process3dMountedRegistry, operation: u64, generation: u64, credits: Process3dIngressCredits) {
        registry.insert(operation, generation, generation, generation, credits).expect("Process3d mounted operation admission");
    }

    #[test]
    fn every_credit_rejects_zero_and_maximum_plus_one_before_operation_construction() {
        let valid = (PROCESS3D_ENVELOPE_MAXIMUM_PAGES, PROCESS3D_ENVELOPE_MAXIMUM_BYTES, PROCESS3D_ENVELOPE_MAXIMUM_ITEMS, PROCESS3D_ENVELOPE_OUTPUT_CHANNELS, PROCESS3D_ENVELOPE_CONTROL_CREDITS);
        assert!(Process3dIngressCredits::try_new(0, valid.1, valid.2, valid.3, valid.4).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0 + 1, valid.1, valid.2, valid.3, valid.4).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, 0, valid.2, valid.3, valid.4).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, valid.1 + 1, valid.2, valid.3, valid.4).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, valid.1, 0, valid.3, valid.4).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, valid.1, valid.2 + 1, valid.3, valid.4).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, valid.1, valid.2, 0, valid.4).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3 + 1, valid.4).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3, 0).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3, valid.4 + 1).is_err());
        assert!(Process3dIngressCredits::try_new(valid.0, valid.1, valid.2, valid.3, valid.4).is_ok());
    }

    #[test]
    fn operation_and_page_maximum_plus_one_leave_producers_unconstructed() {
        let mut registry = Process3dMountedRegistry::new();
        let credits = exact_credits(1, PROCESS3D_OUTPUT_PAGE_BYTES);
        for operation in 1..=PROCESS3D_ENVELOPE_OPERATION_SLOTS as u64 {
            insert(&mut registry, operation, 7, credits);
        }
        assert_eq!(registry.insert(99, 7, 7, 7, credits), Err("process3d-envelope.operation-capacity"));

        let producer_calls = Cell::new(0);
        registry.operation(1, 7).expect("first operation").preflight_page(PROCESS3D_OUTPUT_PAGE_BYTES).expect("exact page");
        producer_calls.set(producer_calls.get() + 1);
        registry.admit_page(1, 7, PROCESS3D_OUTPUT_PAGE_BYTES).expect("exact page handoff");
        assert_eq!(registry.operation(1, 7).expect("first operation").preflight_page(1), Err("process3d-envelope.page-handback"));
        assert_eq!(producer_calls.get(), 1, "maximum-plus-one page never enters the producer");

        for operation in 1..=PROCESS3D_ENVELOPE_OPERATION_SLOTS as u64 {
            registry.remove(operation, 7).expect("empty operation retirement");
        }
    }

    #[test]
    fn outputs_are_bounded_latest_wins_with_lossless_terminal_take_resume_and_ack() {
        let mut registry = Process3dMountedRegistry::new();
        insert(&mut registry, 21, 8, exact_credits(1, PROCESS3D_OUTPUT_PAGE_BYTES));

        registry.publish(21, 8, Process3dOutputKind::Progress, 0).expect("initial progress");
        let first = registry.operation(21, 8).expect("operation").outputs[Process3dOutputKind::Progress as usize].page.as_ref().expect("progress").sequence;
        registry.publish(21, 8, Process3dOutputKind::Progress, 1).expect("latest progress");
        let latest = registry.operation(21, 8).expect("operation").outputs[Process3dOutputKind::Progress as usize].page.as_ref().expect("progress").sequence;
        assert!(latest > first);

        let mut progress = registry.take(21, 8, Process3dOutputKind::Progress).expect("take").expect("progress lease");
        let leased_progress = progress.page.sequence;
        registry.publish(21, 8, Process3dOutputKind::Progress, 2).expect("latest progress while leased");
        registry.resume(&mut progress).expect("resume");
        drop(progress);
        let mut progress = registry.take(21, 8, Process3dOutputKind::Progress).expect("retake").expect("progress lease");
        assert!(progress.page.sequence > leased_progress);
        registry.acknowledge_output(&mut progress).expect("progress ACK");
        drop(progress);

        registry.publish(21, 8, Process3dOutputKind::Checkpoint, 1).expect("checkpoint");
        let mut checkpoint = registry.take(21, 8, Process3dOutputKind::Checkpoint).expect("take").expect("checkpoint lease");
        let checkpoint_sequence = checkpoint.page.sequence;
        registry.resume(&mut checkpoint).expect("checkpoint resume");
        drop(checkpoint);
        let mut checkpoint = registry.take(21, 8, Process3dOutputKind::Checkpoint).expect("retake").expect("checkpoint lease");
        assert_eq!(checkpoint.page.sequence, checkpoint_sequence);
        registry.acknowledge_output(&mut checkpoint).expect("checkpoint ACK");
        drop(checkpoint);

        registry.publish(21, 8, Process3dOutputKind::Terminal, 2).expect("terminal");
        let terminal_sequence = registry.operation(21, 8).expect("operation").outputs[Process3dOutputKind::Terminal as usize].page.as_ref().expect("terminal").sequence;
        registry.publish(21, 8, Process3dOutputKind::Terminal, 4).expect("lossless terminal retry");
        assert_eq!(registry.operation(21, 8).expect("operation").outputs[Process3dOutputKind::Terminal as usize].page.as_ref().expect("terminal").sequence, terminal_sequence,);
        drop(registry.take(21, 8, Process3dOutputKind::Terminal).expect("take").expect("lost terminal lease"));
        let mut terminal = registry.take(21, 8, Process3dOutputKind::Terminal).expect("retake").expect("terminal retained after handle loss");
        assert!(!registry.terminal_acknowledged(21, 8).expect("terminal state"));
        registry.acknowledge_output(&mut terminal).expect("terminal ACK");
        drop(terminal);
        registry.publish(21, 8, Process3dOutputKind::Terminal, 4).expect("terminal remains acknowledged");
        assert!(registry.operation(21, 8).expect("operation").outputs[Process3dOutputKind::Terminal as usize].page.is_none());
        assert!(registry.prepare_load_acknowledgement(21, 8).expect("load ACK preflight"));
        registry.remove(21, 8).expect("terminal-empty removal");
    }

    #[test]
    fn complete_before_ack_and_interrupted_close_retain_owners() {
        let credits = exact_credits(1, PROCESS3D_OUTPUT_PAGE_BYTES);
        let mut registry = Process3dMountedRegistry::new();
        insert(&mut registry, 31, 9, credits);
        registry.publish(31, 9, Process3dOutputKind::Terminal, 2).expect("terminal");
        assert_eq!(registry.remove(31, 9), Err("process3d-envelope.remove-populated"));
        assert!(!registry.prepare_load_acknowledgement(31, 9).expect("complete remains unacknowledged"));
        let mut terminal = registry.take(31, 9, Process3dOutputKind::Terminal).expect("take").expect("terminal");
        registry.acknowledge_output(&mut terminal).expect("terminal ACK");
        drop(terminal);
        assert!(registry.prepare_load_acknowledgement(31, 9).expect("load ACK ready"));
        registry.remove(31, 9).expect("complete operation removal");

        insert(&mut registry, 32, 9, credits);
        for kind in [Process3dOutputKind::Progress, Process3dOutputKind::Checkpoint, Process3dOutputKind::Preview, Process3dOutputKind::Terminal] {
            registry.publish(32, 9, kind, 3).expect("close fixture output");
        }
        assert!(!registry.close_step(), "close releases at most one retained owner per call");
        for _ in 0..PROCESS3D_ENVELOPE_OUTPUT_CHANNELS + 2 {
            if registry.close_step() {
                break;
            }
        }
        assert!(registry.terminal_is_empty());
    }

    #[test]
    fn authoritative_publication_rejects_stale_generation_aba_and_parent() {
        use semio_framework_job::{Generation, OperationId};

        let operation = OperationId(u64::MAX - 71);
        assert_eq!(
            crate::artifacts::process3d::spr::process3d_admit_publication_authority(operation, Generation(41), 41, 40, 41, PROCESS3D_ENVELOPE_MAXIMUM_ITEMS, PROCESS3D_ENVELOPE_OUTPUT_CHANNELS, PROCESS3D_ENVELOPE_CONTROL_CREDITS),
            Err("process3d-publication.initial-freshness")
        );
        assert!(crate::artifacts::process3d::spr::process3d_admit_publication_authority(operation, Generation(41), 41, 41, 41, PROCESS3D_ENVELOPE_MAXIMUM_ITEMS, PROCESS3D_ENVELOPE_OUTPUT_CHANNELS, PROCESS3D_ENVELOPE_CONTROL_CREDITS,).is_ok());
        assert_eq!(crate::artifacts::process3d::spr::process3d_validate_publication_authority(operation, Generation(41)), Ok((41, 41)));
        assert_eq!(crate::artifacts::process3d::spr::process3d_validate_atomic_publication_authority(OperationId(operation.0 + 1), Generation(41), Generation(41)), Err("process3d-publication.wrong-operation"));
        assert_eq!(crate::artifacts::process3d::spr::process3d_validate_atomic_publication_authority(operation, Generation(42), Generation(41)), Err("process3d-publication.wrong-generation"));
        crate::artifacts::process3d::spr::process3d_refresh_publication_authority(operation, Generation(41), 42).expect("authoritative live revision refresh");
        assert_eq!(crate::artifacts::process3d::spr::process3d_validate_atomic_publication_authority(operation, Generation(41), Generation(42)), Err("process3d-publication.wrong-base"));
        assert!(crate::artifacts::process3d::spr::process3d_release_publication_authority(operation, Generation(41)));

        assert!(crate::artifacts::process3d::spr::process3d_admit_publication_authority(operation, Generation(42), 42, 42, 42, PROCESS3D_ENVELOPE_MAXIMUM_ITEMS, PROCESS3D_ENVELOPE_OUTPUT_CHANNELS, PROCESS3D_ENVELOPE_CONTROL_CREDITS,).is_ok());
        assert!(crate::artifacts::process3d::spr::process3d_validate_publication_authority(operation, Generation(41)).is_err());
        assert_eq!(crate::artifacts::process3d::spr::process3d_validate_publication_authority(operation, Generation(42)), Ok((42, 42)));
        assert_eq!(crate::artifacts::process3d::spr::process3d_validate_atomic_publication_authority(operation, Generation(42), Generation(42)), Ok(()));
        assert!(crate::artifacts::process3d::spr::process3d_release_publication_authority(operation, Generation(42)));
    }
}
//#endregion 🧪️MountedLaws

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::process3d::mutations::change_cursor::ChangeCursor;
    use crate::artifacts::process3d::mutations::change_step_enabled::ChangeStepEnabled;
    use crate::artifacts::process3d::mutations::change_step_origin::ChangeStepOrigin;
    use crate::artifacts::process3d::mutations::change_stock_label::ChangeStockLabel;
    use crate::artifacts::process3d::mutations::create_step::CreateStep;
    use crate::artifacts::process3d::mutations::delete_step::DeleteStep;
    use crate::artifacts::process3d::mutations::replace_stock_solid::ReplaceStockSolid;
    use crate::artifacts::process3d::op::Process3dMutation;
    use crate::artifacts::process3d::{brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot, Pose, ProcessMeasure, ProcessStep, StepOrigin, WorkingSolid, PROCESS_3D_SCHEMA};
    use store::{create_document_envelope, Author, ArtifactCommand};

    fn cut_step(id: &str) -> ProcessStep {
        ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
    }

    fn drill_step(id: &str) -> ProcessStep {
        ProcessStep {
            id: id.into(),
            label: "Drill".into(),
            enabled: true,
            origin: Some(StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() }),
            measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() },
        }
    }

    async fn new_store() -> Process3dStore {
        Process3dStore::new(create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_snapshot(), None)).await.expect("new store")
    }

    /// ↩️ Ticket `26/09/01/PROCESS-END-TO-END`: `step_payloads` is the durable, inline timeline
    /// record now — `CreateStep`/`ChangeStepEnabled`/`ChangeStepOrigin`/`DeleteStep` are real
    /// mutations against it, so this dispatches each through the wasm-facing store and asserts the
    /// observed effect, then confirms undo restores the full pre-delete step (not merely its id).
    #[semio_framework_async_macros::async_test]
    async fn step_mutations_dispatch_real_effects() {
        let mut store = new_store().await;
        let empty = store.snapshot().expect("snapshot");

        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") })], description: None }).await.expect("dispatch create");
        let after_create = store.snapshot().expect("snapshot");
        assert_ne!(after_create, empty, "CreateStep must change the persisted document");
        assert!(after_create.step_payloads.iter().any(|step| step.id == "cut-1"));

        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "cut-1".into(), new_enabled: false })], description: None }).await.expect("dispatch enabled change");
        assert!(!store.snapshot().expect("snapshot").step_payloads.iter().find(|step| step.id == "cut-1").expect("cut-1 present").enabled);

        let origin = StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() };
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "cut-1".into(), new_origin: Some(origin.clone()) })], description: None }).await.expect("dispatch origin change");
        assert_eq!(store.snapshot().expect("snapshot").step_payloads.iter().find(|step| step.id == "cut-1").expect("cut-1 present").origin, Some(origin.clone()));

        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::DeleteStep(DeleteStep { id: "cut-1".into() })], description: None }).await.expect("dispatch delete");
        assert_eq!(store.snapshot().expect("snapshot"), empty, "DeleteStep must restore the pre-create document");

        store.dispatch(ArtifactCommand::Undo).await.expect("undo");
        let restored = store.snapshot().expect("snapshot");
        let restored_step = restored.step_payloads.iter().find(|step| step.id == "cut-1").expect("undo of DeleteStep must restore cut-1");
        assert!(!restored_step.enabled, "undo must restore the disabled flag, not just the step's presence");
        assert_eq!(restored_step.origin, Some(origin), "undo must restore the full pre-delete step, including origin");
    }

    #[semio_framework_async_macros::async_test]
    async fn moves_cursor_and_undo_restores_it() {
        let mut store = new_store().await;
        store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(2) })], description: None }).await.expect("move cursor");
        assert_eq!(store.snapshot().expect("snapshot").resolved_up_to, Some(2));

        store.dispatch(ArtifactCommand::Undo).await.expect("undo");
        assert_eq!(store.snapshot().expect("snapshot").resolved_up_to, None);
    }

    /// 🧬️ `Stock`'s `id` has no semantic mutation of its own (it is a fixed singleton-facet key, never
    /// a user-addressed identity field) — only `solid`/`label`/`pose` each carry their own mutation
    /// now (`ReplaceStockSolid`/`ChangeStockLabel`/`MoveStock`), so these two tests compose the fields
    /// that actually change instead of replacing the whole `Stock` record. `ReplaceStockSolid` stays a
    /// real, fully-working mutation (a handle SWAP, never needing to read prior child content).
    #[semio_framework_async_macros::async_test]
    async fn sets_stock_and_backwards_restores() {
        let mut store = new_store().await;
        let original_solid = store.snapshot().expect("snapshot").stock_solid;
        let new_handle = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::Cylinder { radius: 0.2, height: 2.0 }));
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: new_handle.clone() }), Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() })],
                description: None,
            })
            .await
            .expect("set stock");
        let updated = store.snapshot().expect("snapshot");
        assert_eq!(updated.stock_solid, new_handle);
        assert_eq!(updated.stock_label, "Beam");

        store.dispatch(ArtifactCommand::Undo).await.expect("undo");
        assert_eq!(store.snapshot().expect("snapshot").stock_solid, original_solid);
    }

    #[semio_framework_async_macros::async_test]
    async fn sets_stock_to_imported_solid_and_backwards_restores() {
        let mut store = new_store().await;
        let original_solid = store.snapshot().expect("snapshot").stock_solid;
        let imported_handle = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::ImportedSolid { solid_handle: "solid-7".into() }));
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: imported_handle.clone() }), Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Imported STEP".into() })],
                description: None,
            })
            .await
            .expect("set imported stock");
        let updated = store.snapshot().expect("snapshot");
        assert_eq!(updated.stock_solid, imported_handle);
        assert_eq!(updated.stock_label, "Imported STEP");

        store.dispatch(ArtifactCommand::Undo).await.expect("undo");
        assert_eq!(store.snapshot().expect("snapshot").stock_solid, original_solid);
    }

    //#region 🔖️DocumentTextTests
    #[semio_framework_async_macros::async_test]
    async fn process3d_document_text_round_trips_after_apply_and_checkpoint() {
        let envelope = create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_snapshot(), None);
        let mut store = Process3dStore::new(envelope).await.expect("new store");
        store
            .dispatch(ArtifactCommand::Apply {
                mutations: vec![
                    Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::Box { width: 2.4, depth: 0.12, height: 0.24 })) }),
                    Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Timber Beam".into() }),
                    Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") }),
                    Process3dMutation::CreateStep(CreateStep { index: 1, step: drill_step("drill-1") }),
                    Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(1) }),
                ],
                description: Some("build timeline".into()),
            })
            .await
            .expect("apply");
        store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).await.expect("commit");
        store::os_store::test_support::assert_document_text_round_trip(&store).await;
        store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    }
    //#endregion 🔖️DocumentTextTests
}
//#endregion 🧪️Tests
