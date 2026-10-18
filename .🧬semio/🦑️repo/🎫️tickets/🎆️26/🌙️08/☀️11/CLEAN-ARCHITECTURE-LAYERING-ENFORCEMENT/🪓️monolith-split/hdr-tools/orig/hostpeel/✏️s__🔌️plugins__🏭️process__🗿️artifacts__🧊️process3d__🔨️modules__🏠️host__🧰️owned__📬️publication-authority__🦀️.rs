//! 📬️ process3d publication authority: the fixed publication lease registry, admission, refresh, validation and release of the app and the document publication authority, and the test-only hostile lease hooks.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Process3dPublicationLease {
    pub(super) operation: u64,
    pub(super) generation: u64,
    pub(super) base_revision: u64,
    pub(super) parent_revision: u64,
    pub(super) live_revision: u64,
    pub(super) maximum_items: usize,
    pub(super) maximum_output_pages: usize,
    pub(super) maximum_controls: usize,
    pub(super) closing: bool,
    pub(super) terminal: bool,
}

impl semio_framework_job_registry::FixedOperationOwner for Process3dPublicationLease {
    fn retained_bytes(&self) -> usize {
        size_of::<Self>()
    }

    fn cancel(&mut self) {
        self.closing = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        if !self.closing || grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.terminal {
            return semio_framework_job::InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() };
        }
        if grant.maximum_release_bytes < size_of::<Self>() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() };
        }
        self.terminal = true;
        semio_framework_job::InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes: size_of::<Self>(), ..RetainedCloneProgress::default() } }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.terminal
    }
}

type Process3dPublicationRegistry = semio_framework_job_registry::FixedOperationRegistry<Process3dPublicationLease, PROCESS3D_PUBLICATION_SLOTS>;

fn process3d_publication_leases() -> &'static std::sync::Mutex<Process3dPublicationRegistry> {
    static LEASES: std::sync::OnceLock<std::sync::Mutex<semio_framework_job_registry::FixedOperationRegistry<Process3dPublicationLease, PROCESS3D_PUBLICATION_SLOTS>>> = std::sync::OnceLock::new();
    LEASES.get_or_init(|| std::sync::Mutex::new(Process3dPublicationRegistry::new(PROCESS3D_PUBLICATION_SLOTS * size_of::<Process3dPublicationLease>())))
}

pub fn process3d_admit_publication_authority(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    limits: Process3dPublicationLimits,
) -> Result<(), &'static str> {
    let Process3dPublicationLimits { maximum_items, maximum_output_pages, maximum_controls } = limits;
    if generation.0 != live_revision || base_revision != live_revision || parent_revision != base_revision {
        return Err("process3d-publication.initial-freshness");
    }
    let mut leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("process3d-publication.operation-duplicate");
    }
    if maximum_items == 0 || maximum_items > PROCESS3D_MAXIMUM_DOMAIN_ITEMS || maximum_output_pages != PROCESS3D_MOUNTED_OUTPUT_CHANNELS || maximum_controls != PROCESS3D_MOUNTED_CONTROL_CREDITS {
        return Err("process3d-publication.domain-credits");
    }
    let key = process3d_publication_key(operation, generation);
    leases
        .admit(
            key,
            Process3dPublicationLease { operation: operation.0, generation: generation.0, base_revision, parent_revision, live_revision, maximum_items, maximum_output_pages, maximum_controls, closing: false, terminal: false },
        )
        .map_err(|_| "process3d-publication.saturated")?;
    #[cfg(any(test, feature = "testkit"))]
    process3d_publication_test_admissions().lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(key);
    Ok(())
}

/// 🔐️ The ONE lease the app grants ITSELF for a replacement the host began (`Effect::LoadDocument`
/// → `build_document_store_initialization_job`): base, parent and live revision are all the
/// generation the host started the replacement on — the only publication that commit can accept —
/// with the domain's own credits. A holder that admitted a HOST lease for the same operation first
/// keeps it (`Err`).
///
/// A self-grant is NOT a host publication and does not live in the host's fixed table. That table
/// is four DIRECT-MAPPED slots: a self-grant lying in it refuses an unrelated host publication
/// whose key happens to map to the same slot (`process3d-publication.saturated`) while three slots
/// stay free — which is exactly what a test binary hosting several app instances beside the
/// fixture laws produces. The host drives one replacement per instance at a time, so this
/// authority holds exactly one lease and a new self-grant supersedes the load the host already
/// abandoned; the happy path releases through `process3d_release_app_publication_authority` at
/// validation or initializer retirement.
fn process3d_app_publication_lease() -> &'static std::sync::Mutex<Option<(semio_framework_job_registry::FixedOperationKey, Process3dPublicationLease)>> {
    static LEASE: std::sync::OnceLock<std::sync::Mutex<Option<(semio_framework_job_registry::FixedOperationKey, Process3dPublicationLease)>>> = std::sync::OnceLock::new();
    LEASE.get_or_init(|| std::sync::Mutex::new(None))
}

/// 🔎️ One lease by its exact key, from the host table first and the app self-grant second.
fn process3d_publication_lease_by_key(key: semio_framework_job_registry::FixedOperationKey) -> Result<Option<Process3dPublicationLease>, &'static str> {
    let leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if let Some(lease) = leases.get(key) {
        return Ok(Some(*lease));
    }
    drop(leases);
    let app = process3d_app_publication_lease().try_lock().map_err(|_| "process3d-publication.contended")?;
    Ok(app.as_ref().filter(|(held, _)| *held == key).map(|(_, lease)| *lease))
}

/// 🔎️ One lease by operation, from the host table first and the app self-grant second.
fn process3d_publication_lease_by_operation(operation: semio_framework_job::OperationId) -> Result<Option<Process3dPublicationLease>, &'static str> {
    let leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if let Some((_, lease)) = leases.get_operation(operation) {
        return Ok(Some(*lease));
    }
    drop(leases);
    let app = process3d_app_publication_lease().try_lock().map_err(|_| "process3d-publication.contended")?;
    Ok(app.as_ref().filter(|(_, lease)| lease.operation == operation.0).map(|(_, lease)| *lease))
}

pub fn process3d_admit_app_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("process3d-publication.operation-duplicate");
    }
    drop(leases);
    let mut app = process3d_app_publication_lease().try_lock().map_err(|_| "process3d-publication.contended")?;
    if app.as_ref().is_some_and(|(_, lease)| lease.operation == operation.0) {
        return Err("process3d-publication.operation-duplicate");
    }
    *app = Some((
        process3d_publication_key(operation, generation),
        Process3dPublicationLease {
            operation: operation.0,
            generation: generation.0,
            base_revision: generation.0,
            parent_revision: generation.0,
            live_revision: generation.0,
            maximum_items: PROCESS3D_MAXIMUM_DOMAIN_ITEMS,
            maximum_output_pages: PROCESS3D_MOUNTED_OUTPUT_CHANNELS,
            maximum_controls: PROCESS3D_MOUNTED_CONTROL_CREDITS,
            closing: false,
            terminal: false,
        },
    ));
    Ok(())
}

pub(super) fn process3d_validate_atomic_lease(lease: Process3dPublicationLease, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    if lease.operation != operation.0 {
        return Err("process3d-publication.wrong-operation");
    }
    if lease.generation != generation.0 {
        return Err("process3d-publication.wrong-generation");
    }
    if lease.live_revision != live_generation.0 || lease.base_revision != lease.live_revision {
        return Err("process3d-publication.wrong-base");
    }
    if lease.parent_revision != lease.base_revision {
        return Err("process3d-publication.wrong-parent");
    }
    if lease.maximum_items == 0 || lease.maximum_output_pages != PROCESS3D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != PROCESS3D_MOUNTED_CONTROL_CREDITS {
        return Err("process3d-publication.authority-credits");
    }
    Ok(())
}

fn process3d_publication_key(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job_registry::FixedOperationKey {
    semio_framework_job_registry::FixedOperationKey::new(operation, generation)
}

/// 🛣️ The ONE lane every test that admits into the process-global publication registry takes first.
/// The lease table is a four-slot DIRECT-MAPPED registry shared by the whole test binary, so two
/// laws admitting concurrently (the suite runs multi-threaded) collide on a slot and report
/// `process3d-publication.saturated` — a scheduling artifact of the fixture, never the property under
/// test. Poisoning is absorbed: a lane is a scheduling device, and the panic that poisoned it is
/// already the failure being reported.
///
/// 🧹️ Taking the lane also DRAINS whatever the previous lane holder left behind. A law that panics
/// between its `admit` and its `release` (the exact shape of a failing assertion) strands a lease in
/// a four-slot table that nothing can enumerate, so the NEXT laws report
/// `process3d-publication.saturated` and one real defect is reported as three or four. Every host
/// admission in this binary happens under this lane (`🔬️retained-laws`, the editor's production
/// envelope fixture, the mounted registry laws), so draining here can never take a lease a
/// concurrently running law still owns.
#[cfg(any(test, feature = "testkit"))]
pub fn process3d_publication_authority_lane() -> std::sync::MutexGuard<'static, ()> {
    static LANE: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let guard = LANE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    // 🔒️ Leases BEFORE admissions — the same order `process3d_admit_publication_authority` and
    // `process3d_release_publication_authority` take them in, so the drain can never invert a lock pair.
    if let Ok(mut leases) = process3d_publication_leases().try_lock() {
        let mut admitted = process3d_publication_test_admissions().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for key in admitted.drain(..) {
            leases.take(key);
        }
    }
    guard
}

/// 🧾️ Every key this test binary admitted into the process-global host table and has not released —
/// the enumeration `FixedOperationRegistry` does not offer, kept only so the lane above can drain a
/// stranded lease.
#[cfg(any(test, feature = "testkit"))]
fn process3d_publication_test_admissions() -> &'static std::sync::Mutex<Vec<semio_framework_job_registry::FixedOperationKey>> {
    static ADMITTED: std::sync::OnceLock<std::sync::Mutex<Vec<semio_framework_job_registry::FixedOperationKey>>> = std::sync::OnceLock::new();
    ADMITTED.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// 🧮️ Domain item, output page, and control limits admitted for one publication.
#[derive(Clone, Copy)]
pub struct Process3dPublicationLimits {
    pub maximum_items: usize,
    pub maximum_output_pages: usize,
    pub maximum_controls: usize,
}

/// 🔐️ Releases the app-admitted lease of `operation` (a host-admitted one is the host's to release).
pub fn process3d_release_app_publication_authority(operation: semio_framework_job::OperationId) -> bool {
    let Ok(mut app) = process3d_app_publication_lease().try_lock() else { return false };
    if app.as_ref().is_none_or(|(_, lease)| lease.operation != operation.0) {
        return false;
    }
    app.take().is_some()
}

pub fn process3d_refresh_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_revision: u64) -> Result<(), &'static str> {
    let key = process3d_publication_key(operation, generation);
    let mut leases = process3d_publication_leases().try_lock().map_err(|_| "process3d-publication.contended")?;
    if let Some(lease) = leases.get_mut(key) {
        lease.live_revision = live_revision;
        return Ok(());
    }
    drop(leases);
    let mut app = process3d_app_publication_lease().try_lock().map_err(|_| "process3d-publication.contended")?;
    let Some((_, lease)) = app.as_mut().filter(|(held, _)| *held == key) else { return Err("process3d-publication.stale-authority") };
    lease.live_revision = live_revision;
    Ok(())
}

pub fn process3d_validate_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(u64, u64), &'static str> {
    let lease = process3d_publication_lease_by_key(process3d_publication_key(operation, generation))?.ok_or("process3d-publication.stale-authority")?;
    if lease.generation != generation.0 || lease.live_revision != generation.0 || lease.base_revision != lease.live_revision || lease.parent_revision != lease.base_revision {
        return Err("process3d-publication.stale-aba-parent");
    }
    Ok((lease.base_revision, lease.parent_revision))
}

/// 🔐️ Fail-closed Process3d authority used by the shared atomic replacement branch.
pub fn process3d_validate_atomic_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let lease = process3d_publication_lease_by_operation(operation)?.ok_or("process3d-publication.authority-missing")?;
    #[cfg(any(test, feature = "testkit"))]
    let lease = {
        let mut lease = lease;
        let mut hostiles = process3d_publication_hostiles().try_lock().map_err(|_| "process3d-publication.hostile-contended")?;
        if let Some(hostile) = hostiles.iter_mut().flatten().find(|value| value.operation == operation.0) {
            hostile.observed = Some(match hostile.hostile {
                Process3dPublicationHostile::Missing => "process3d-publication.authority-missing",
                Process3dPublicationHostile::WrongOperation => "process3d-publication.wrong-operation",
                Process3dPublicationHostile::WrongGeneration => "process3d-publication.wrong-generation",
                Process3dPublicationHostile::WrongBase => "process3d-publication.wrong-base",
                Process3dPublicationHostile::WrongParent => "process3d-publication.wrong-parent",
            });
            match hostile.hostile {
                Process3dPublicationHostile::Missing => return Err("process3d-publication.authority-missing"),
                Process3dPublicationHostile::WrongOperation => lease.operation = lease.operation.wrapping_add(1),
                Process3dPublicationHostile::WrongGeneration => lease.generation = lease.generation.wrapping_add(1),
                Process3dPublicationHostile::WrongBase => lease.base_revision = lease.base_revision.wrapping_add(1),
                Process3dPublicationHostile::WrongParent => lease.parent_revision = lease.parent_revision.wrapping_add(1),
            }
        }
        lease
    };
    process3d_validate_atomic_lease(lease, operation, generation, live_generation)
}

pub fn process3d_publication_item_credit(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<usize, &'static str> {
    let lease = process3d_publication_lease_by_key(process3d_publication_key(operation, generation))?.ok_or("process3d-publication.stale-authority")?;
    if lease.maximum_output_pages != PROCESS3D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != PROCESS3D_MOUNTED_CONTROL_CREDITS {
        return Err("process3d-publication.domain-credits-lost");
    }
    Ok(lease.maximum_items)
}

pub fn process3d_release_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> bool {
    let Ok(mut leases) = process3d_publication_leases().try_lock() else { return false };
    let key = process3d_publication_key(operation, generation);
    #[cfg(any(test, feature = "testkit"))]
    process3d_publication_test_admissions().lock().unwrap_or_else(std::sync::PoisonError::into_inner).retain(|held| *held != key);
    leases.take(key).is_some()
}

#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Process3dPublicationHostile {
    Missing,
    WrongOperation,
    WrongGeneration,
    WrongBase,
    WrongParent,
}

#[cfg(any(test, feature = "testkit"))]
fn process3d_publication_hostiles() -> &'static std::sync::Mutex<[Option<Process3dPublicationHostileLease>; PROCESS3D_PUBLICATION_SLOTS]> {
    static HOSTILES: std::sync::OnceLock<std::sync::Mutex<[Option<Process3dPublicationHostileLease>; PROCESS3D_PUBLICATION_SLOTS]>> = std::sync::OnceLock::new();
    HOSTILES.get_or_init(|| std::sync::Mutex::new([None; PROCESS3D_PUBLICATION_SLOTS]))
}

#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Copy)]
struct Process3dPublicationHostileLease {
    pub(super) operation: u64,
    hostile: Process3dPublicationHostile,
    observed: Option<&'static str>,
}

#[cfg(any(test, feature = "testkit"))]
pub fn process3d_arm_publication_hostile(operation: semio_framework_job::OperationId, hostile: Process3dPublicationHostile) {
    let mut hostiles = process3d_publication_hostiles().try_lock().expect("Process3d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_none()).expect("Process3d hostile publication authority has a fixed slot");
    *slot = Some(Process3dPublicationHostileLease { operation: operation.0, hostile, observed: None });
}

#[cfg(any(test, feature = "testkit"))]
pub fn process3d_take_publication_hostile_observed(operation: semio_framework_job::OperationId) -> Option<&'static str> {
    let mut hostiles = process3d_publication_hostiles().try_lock().expect("Process3d hostile publication authority is uncontended");
    let slot = hostiles.iter_mut().find(|slot| slot.is_some_and(|value| value.operation == operation.0))?;
    slot.take()?.observed
}
