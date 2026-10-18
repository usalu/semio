//! 🪪️ Publication leases: the fixed operation publication lease and its owner registration, the keyed lease lookups and the atomic lease validation. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Generation2dPublicationLease {
    pub(super) operation: u64,
    pub(super) generation: u64,
    pub(super) base_revision: u64,
    pub(super) parent_revision: u64,
    pub(super) live_revision: u64,
    pub(super) maximum_items: usize,
    pub(super) maximum_output_pages: usize,
    pub(super) maximum_controls: usize,
    closing: bool,
    pub(super) terminal: bool,
}

impl semio_framework_job_registry::FixedOperationOwner for Generation2dPublicationLease {
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
        use semio_framework_job::InteractiveJobCloseStep;
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if self.terminal {
            return InteractiveJobCloseStep::Complete { progress: Default::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_release_bytes < size_of::<Self>() {
            return InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        self.terminal = true;
        InteractiveJobCloseStep::Complete { progress: semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, released_bytes: size_of::<Self>(), ..Default::default() } }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.terminal
    }
}

type Generation2dPublicationRegistry = semio_framework_job_registry::FixedOperationRegistry<Generation2dPublicationLease, GENERATION2D_PUBLICATION_SLOTS>;

pub(super) fn generation2d_publication_leases() -> &'static std::sync::Mutex<Generation2dPublicationRegistry> {
    static LEASES: std::sync::OnceLock<std::sync::Mutex<semio_framework_job_registry::FixedOperationRegistry<Generation2dPublicationLease, GENERATION2D_PUBLICATION_SLOTS>>> = std::sync::OnceLock::new();
    LEASES.get_or_init(|| std::sync::Mutex::new(Generation2dPublicationRegistry::new(GENERATION2D_PUBLICATION_SLOTS * size_of::<Generation2dPublicationLease>())))
}

pub fn generation2d_admit_publication_authority(
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    base_revision: u64,
    parent_revision: u64,
    live_revision: u64,
    credits: Generation2dPublicationCredits,
) -> Result<(), &'static str> {
    let Generation2dPublicationCredits { maximum_items, maximum_output_pages, maximum_controls } = credits;
    if generation.0 != live_revision || base_revision != live_revision || parent_revision != base_revision {
        return Err("generation2d-publication.initial-freshness");
    }
    let mut leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("generation2d-publication.operation-duplicate");
    }
    if maximum_items == 0 || maximum_items > GENERATION2D_MAXIMUM_DOMAIN_ITEMS || maximum_output_pages != GENERATION2D_MOUNTED_OUTPUT_CHANNELS || maximum_controls != GENERATION2D_MOUNTED_CONTROL_CREDITS {
        return Err("generation2d-publication.domain-credits");
    }
    leases
        .admit(
            generation2d_publication_key(operation, generation),
            Generation2dPublicationLease { operation: operation.0, generation: generation.0, base_revision, parent_revision, live_revision, maximum_items, maximum_output_pages, maximum_controls, closing: false, terminal: false },
        )
        .map_err(|_| "generation2d-publication.saturated")
}

/// 🔐️ The ONE lease the app grants ITSELF for a replacement the host began (`loadDocumentArchive`, a whole-document load
/// → `build_document_store_initialization_job`): base, parent and live revision are all the generation the host started
/// the replacement on — the only publication that commit can accept — with the domain's own credits. A holder that
/// admitted a HOST lease for the same operation first keeps it (`Err`). The twin of `process3d_app_publication_lease`.
///
/// A self-grant is NOT a host publication and does not live in the host's fixed direct-mapped table, where it would refuse
/// an unrelated host publication mapping to the same slot. The host drives one replacement per instance at a time, so this
/// authority holds exactly one lease and a new self-grant supersedes the load the host already abandoned; it is released
/// through `generation2d_release_app_publication_authority` at validation, cancellation, fault and initializer close.
pub(super) fn generation2d_app_publication_lease() -> &'static std::sync::Mutex<Option<(semio_framework_job_registry::FixedOperationKey, Generation2dPublicationLease)>> {
    static LEASE: std::sync::OnceLock<std::sync::Mutex<Option<(semio_framework_job_registry::FixedOperationKey, Generation2dPublicationLease)>>> = std::sync::OnceLock::new();
    LEASE.get_or_init(|| std::sync::Mutex::new(None))
}

/// 🔎️ One lease by its exact key, from the host table first and the app self-grant second.
pub(super) fn generation2d_publication_lease_by_key(key: semio_framework_job_registry::FixedOperationKey) -> Result<Option<Generation2dPublicationLease>, &'static str> {
    let leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    if let Some(lease) = leases.get(key) {
        return Ok(Some(*lease));
    }
    drop(leases);
    let app = generation2d_app_publication_lease().try_lock().map_err(|_| "generation2d-publication.contended")?;
    Ok(app.as_ref().filter(|(held, _)| *held == key).map(|(_, lease)| *lease))
}

/// 🔎️ One lease by operation, from the host table first and the app self-grant second.
pub(super) fn generation2d_publication_lease_by_operation(operation: semio_framework_job::OperationId) -> Result<Option<Generation2dPublicationLease>, &'static str> {
    let leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    if let Some((_, lease)) = leases.get_operation(operation) {
        return Ok(Some(*lease));
    }
    drop(leases);
    let app = generation2d_app_publication_lease().try_lock().map_err(|_| "generation2d-publication.contended")?;
    Ok(app.as_ref().filter(|(_, lease)| lease.operation == operation.0).map(|(_, lease)| *lease))
}

/// 🔐️ Grants the app's own lease for `operation` — see [`generation2d_app_publication_lease`].
pub fn generation2d_admit_app_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    if leases.get_operation(operation).is_some() {
        return Err("generation2d-publication.operation-duplicate");
    }
    drop(leases);
    let mut app = generation2d_app_publication_lease().try_lock().map_err(|_| "generation2d-publication.contended")?;
    if app.as_ref().is_some_and(|(_, lease)| lease.operation == operation.0) {
        return Err("generation2d-publication.operation-duplicate");
    }
    *app = Some((
        generation2d_publication_key(operation, generation),
        Generation2dPublicationLease {
            operation: operation.0,
            generation: generation.0,
            base_revision: generation.0,
            parent_revision: generation.0,
            live_revision: generation.0,
            maximum_items: GENERATION2D_MAXIMUM_DOMAIN_ITEMS,
            maximum_output_pages: GENERATION2D_MOUNTED_OUTPUT_CHANNELS,
            maximum_controls: GENERATION2D_MOUNTED_CONTROL_CREDITS,
            closing: false,
            terminal: false,
        },
    ));
    Ok(())
}

pub(super) fn generation2d_validate_atomic_lease(lease: Generation2dPublicationLease, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    if lease.operation != operation.0 {
        return Err("generation2d-publication.wrong-operation");
    }
    if lease.generation != generation.0 {
        return Err("generation2d-publication.wrong-generation");
    }
    if lease.live_revision != live_generation.0 || lease.base_revision != lease.live_revision {
        return Err("generation2d-publication.wrong-base");
    }
    if lease.parent_revision != lease.base_revision {
        return Err("generation2d-publication.wrong-parent");
    }
    if lease.maximum_items == 0 || lease.maximum_output_pages != GENERATION2D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != GENERATION2D_MOUNTED_CONTROL_CREDITS {
        return Err("generation2d-publication.authority-credits");
    }
    Ok(())
}
