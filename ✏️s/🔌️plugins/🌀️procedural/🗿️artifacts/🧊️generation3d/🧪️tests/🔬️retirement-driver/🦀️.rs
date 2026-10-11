//! 🧪️ The grant drivers every retained-clone close and step test of this artifact shares: each currency is granted exactly its quoted demand, so a test that passes proves the quote itself.

#![allow(dead_code)]

use semio_framework_job::{CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, InteractiveStage, JobOutcomeBorrow, OperationId, StepBudget, StepContext};
use semio_framework_value::retained_clone::{admit_retained_clone_close, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};

/// 🎟️ The retained policy one driven step runs under.
pub(crate) const STEP_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🎟️ A one-item grant paying exactly `demand` in every currency.
pub(crate) fn exact_grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

/// 📏️ The next close quote of one erased retirement.
pub(crate) fn erased_demand(owner: &dyn store::ErasedSnapshotRetirement) -> Result<RetirementDemand, ValueError> {
    let copy = owner.next_copy_byte_demand()?;
    Ok(RetirementDemand { copy_bytes: copy, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? })
}

/// 🧹️ Drives one erased retirement to its terminal-empty witness under exactly its quoted grants and answers the released bytes.
pub(crate) fn drive_erased_released(owner: &mut dyn store::ErasedSnapshotRetirement, label: &str) -> usize {
    let mut released = 0usize;
    for _ in 0..1_000_000 {
        if owner.terminal_is_empty() {
            return released;
        }
        let grant = exact_grant(erased_demand(owner).unwrap_or_else(|error| panic!("{label} refused its close quote: {error}")));
        let step = owner.close_step(grant).unwrap_or_else(|error| panic!("{label} retirement faulted: {error}"));
        admit_retained_clone_close(grant, step, owner.terminal_is_empty(), "driven test retirement").unwrap_or_else(|error| panic!("{label} overspent its grant: {error}"));
        released += step.progress().released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(owner.terminal_is_empty(), "{label} reported Complete without its exact terminal-empty witness");
            return released;
        }
    }
    panic!("{label} did not reach its terminal-empty close witness")
}

/// 🎟️ Admits `value` as an owned retirement under a grant that pays exactly its frame birth.
pub(crate) fn retire_owned_for_test<T: semio_framework_value::retirement::RetireOwned>(value: T, label: &str) -> Box<dyn store::ErasedSnapshotRetirement> {
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), maximum_release_bytes: 0, maximum_depth: 2 };
    match semio_framework_value::retirement::admit_owned_retirement(value, birth) {
        Ok((owner, _)) => owner,
        Err((error, _)) => panic!("{label} refused its own birth: {error}"),
    }
}

/// 🔗️ Admits `value` as a shared (`Arc`) retirement under a grant that pays exactly its frame birth.
pub(crate) fn retire_shared_for_test<T: semio_framework_value::retirement::RetireOwned + Sync>(value: std::sync::Arc<T>, label: &str) -> Box<dyn store::ErasedSnapshotRetirement> {
    let factory = semio_framework_value::retirement::SharedValueRetirementFactory::<T>::default();
    let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: store::SnapshotRetirementFactory::retirement_birth_bytes(&factory, &value), maximum_release_bytes: 0, maximum_depth: 2 };
    match store::SnapshotRetirementFactory::retire(&factory, value, birth) {
        Ok((owner, _)) => owner,
        Err((error, _)) => panic!("{label} refused its own birth: {error}"),
    }
}

/// 📏️ The next close grant of one interactive job.
pub(crate) fn job_grant<J: InteractiveJob + ?Sized>(job: &J) -> RetainedCloneGrant {
    let copy = job.next_close_copy_byte_demand().expect("the job quotes its copy demand");
    exact_grant(RetirementDemand { copy_bytes: copy, capacity_bytes: job.next_close_capacity_byte_demand(copy).expect("the job quotes its capacity demand"), release_bytes: job.next_close_release_byte_demand().expect("the job quotes its release demand"), depth: job.next_close_depth_demand().expect("the job quotes its depth demand") })
}

/// 🧹️ Begins the close of one interactive job and drives it to its terminal-empty witness under exactly its quoted grants; answers the released bytes.
pub(crate) fn close_job<J: InteractiveJob + ?Sized>(job: &mut J, label: &str) -> usize {
    job.begin_close();
    let mut released = 0usize;
    for _ in 0..10_000_000 {
        let grant = job_grant(job);
        match job.close_step(grant) {
            InteractiveJobCloseStep::Complete { progress } => {
                assert!(progress.fits(grant), "{label} close receipt exceeds its quoted grant");
                released += progress.released_bytes;
                assert!(job.terminal_is_empty(), "{label} completed without its terminal-empty witness");
                return released;
            }
            InteractiveJobCloseStep::Pending { progress } => {
                assert!(progress.fits(grant), "{label} close receipt exceeds its quoted grant");
                released += progress.released_bytes;
            }
            InteractiveJobCloseStep::Blocked => panic!("{label} close blocked under its exact quoted grant"),
            InteractiveJobCloseStep::Refused { kind, .. } => panic!("{label} close was refused: {kind:?}"),
        }
    }
    panic!("{label} close never completed")
}

/// 🧹️ Begins the close of one command work and drives it to its terminal-empty witness under exactly its quoted grants.
pub(crate) fn close_work<A: semio_framework_plugin::ArtifactApp, W: semio_framework_plugin::retained_command::ArtifactCommandWork<A> + ?Sized>(work: &mut W, label: &str) {
    work.begin_close();
    for _ in 0..10_000_000 {
        let copy = work.next_close_copy_byte_demand().unwrap_or_else(|error| panic!("{label} copy quote refused: {error}"));
        let grant = exact_grant(RetirementDemand {
            copy_bytes: copy,
            capacity_bytes: work.next_close_capacity_byte_demand(copy).unwrap_or_else(|error| panic!("{label} capacity quote refused: {error}")),
            release_bytes: work.next_close_release_byte_demand().unwrap_or_else(|error| panic!("{label} release quote refused: {error}")),
            depth: work.next_close_depth_demand().unwrap_or_else(|error| panic!("{label} depth quote refused: {error}")),
        });
        match work.close_step(grant) {
            InteractiveJobCloseStep::Complete { progress } => {
                assert!(progress.fits(grant), "{label} close receipt exceeds its quoted grant");
                assert!(work.terminal_is_empty(), "{label} completed without its terminal-empty witness");
                return;
            }
            InteractiveJobCloseStep::Pending { progress } => assert!(progress.fits(grant), "{label} close receipt exceeds its quoted grant"),
            InteractiveJobCloseStep::Blocked => panic!("{label} close blocked under its exact quoted grant"),
            InteractiveJobCloseStep::Refused { kind, .. } => panic!("{label} close was refused: {kind:?}"),
        }
    }
    panic!("{label} close never completed")
}

/// 🧹️ Spends one exact-grant close turn of the instance owner `O` behind `handle`; answers whether it reached its terminal-empty witness.
pub(crate) fn close_instance_owner_turn<O: semio_framework_plugin::ArtifactInstanceOperationOwner + 'static>(handle: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> bool {
    handle
        .with_mut::<O, _>(|owner| {
            let demand = owner.retirement_demands(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| semio_framework_plugin::Fault::from(error.to_string()))?;
            let step = owner.close_step(exact_grant(demand))?;
            Ok(matches!(step, semio_framework_plugin::PluginLifecycleStep::Complete(_)))
        })
        .expect("the owner lends itself to its close ladder")
}

/// 🧹️ Closes the instance owner `O` behind `handle` to its terminal-empty witness.
pub(crate) fn close_instance_owner<O: semio_framework_plugin::ArtifactInstanceOperationOwner + 'static>(handle: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, label: &str) {
    for _ in 0..1_000_000 {
        if close_instance_owner_turn::<O>(handle) {
            return;
        }
    }
    panic!("{label} did not reach terminal-empty")
}

/// 🧭️ What one driven call of a job lent to its caller.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Stepped {
    Yield,
    Preview(Vec<u8>),
    Checkpoint,
    Complete,
    Fault,
    Cancelled,
}

/// 🦶️ Drives one step of `job` under `fuel` and `deadline_us` with the shared test grant and answers what it lent.
pub(crate) fn step_once<J: InteractiveJob + ?Sized>(job: &mut J, fuel: u64, deadline_us: u64, cancel: &CancelToken, operation: OperationId, generation: Generation, sequence: &mut u64) -> Stepped {
    let mut receipt = RetainedCloneProgress::default();
    let mut cx = StepContext::new(operation, generation, StepBudget::new(fuel, deadline_us, STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, sequence, &mut receipt);
    let mut verdict = None;
    match semio_framework_job::drive_step(job, &mut cx, "procedural.test", InteractiveStage::InteractiveStep, &mut verdict).expect("job step admission") {
        None | Some(JobOutcomeBorrow::Yield { .. }) => Stepped::Yield,
        Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => Stepped::Preview((0..payload.page_count()).flat_map(|index| payload.page(index).expect("preview page").to_vec()).collect()),
        Some(JobOutcomeBorrow::CheckpointReady { .. }) => Stepped::Checkpoint,
        Some(JobOutcomeBorrow::Complete { .. }) => Stepped::Complete,
        Some(JobOutcomeBorrow::Fault { .. }) => Stepped::Fault,
        Some(JobOutcomeBorrow::Cancelled { .. }) => Stepped::Cancelled,
    }
}

/// 🦶️ Steps `job` with a fresh operation until it leaves `Yield` and answers the terminal (or preview) it lent.
pub(crate) fn step_until_settled<J: InteractiveJob + ?Sized>(job: &mut J, fuel: u64, limit: usize) -> Stepped {
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut sequence = 0;
    for _ in 0..limit {
        let now = semio_framework_job::default_now_us().expect("clock");
        match step_once(job, fuel, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4, &cancel, operation, generation, &mut sequence) {
            Stepped::Yield => {}
            settled => return settled,
        }
    }
    panic!("the job never left Yield")
}
