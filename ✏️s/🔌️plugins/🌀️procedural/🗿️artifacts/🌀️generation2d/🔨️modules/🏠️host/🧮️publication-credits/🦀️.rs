//! 🧮️ Publication credits: the mounted channel constants, the publication key, credits and authority release, refresh and validation. See [`super`].

use super::*;

pub const GENERATION2D_MOUNTED_OUTPUT_CHANNELS: usize = 4;

pub const GENERATION2D_MOUNTED_CONTROL_CREDITS: usize = 1;

pub(super) const GENERATION2D_PUBLICATION_SLOTS: usize = 4;

pub(super) fn generation2d_publication_key(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> semio_framework_job_registry::FixedOperationKey {
    semio_framework_job_registry::FixedOperationKey::new(operation, generation)
}

/// 🧮️ Domain credits admitted together for one publication authority.
#[derive(Clone, Copy, Debug)]
pub struct Generation2dPublicationCredits {
    pub maximum_items: usize,
    pub maximum_output_pages: usize,
    pub maximum_controls: usize,
}

/// 🔐️ Releases the app-admitted lease of `operation` (a host-admitted one is the host's to release).
pub fn generation2d_release_app_publication_authority(operation: semio_framework_job::OperationId) -> bool {
    let Ok(mut app) = generation2d_app_publication_lease().try_lock() else { return false };
    if app.as_ref().is_none_or(|(_, lease)| lease.operation != operation.0) {
        return false;
    }
    app.take().is_some()
}

pub fn generation2d_refresh_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_revision: u64) -> Result<(), &'static str> {
    let key = generation2d_publication_key(operation, generation);
    let mut leases = generation2d_publication_leases().try_lock().map_err(|_| "generation2d-publication.contended")?;
    if let Some(lease) = leases.get_mut(key) {
        lease.live_revision = live_revision;
        return Ok(());
    }
    drop(leases);
    let mut app = generation2d_app_publication_lease().try_lock().map_err(|_| "generation2d-publication.contended")?;
    let Some((_, lease)) = app.as_mut().filter(|(held, _)| *held == key) else { return Err("generation2d-publication.stale-authority") };
    lease.live_revision = live_revision;
    Ok(())
}

pub fn generation2d_validate_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<(u64, u64), &'static str> {
    let lease = generation2d_publication_lease_by_key(generation2d_publication_key(operation, generation))?.ok_or("generation2d-publication.stale-authority")?;
    if lease.generation != generation.0 || lease.live_revision != generation.0 || lease.base_revision != lease.live_revision || lease.parent_revision != lease.base_revision {
        return Err("generation2d-publication.stale-aba-parent");
    }
    Ok((lease.base_revision, lease.parent_revision))
}

/// 🔐️ Fail-closed Generation2d authority used by the shared atomic replacement branch.
pub fn generation2d_validate_atomic_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), &'static str> {
    let lease = generation2d_publication_lease_by_operation(operation)?.ok_or("generation2d-publication.authority-missing")?;
    #[cfg(any(test, feature = "testkit"))]
    let lease = {
        let mut lease = lease;
        let mut hostiles = generation2d_publication_hostiles().try_lock().map_err(|_| "generation2d-publication.hostile-contended")?;
        if let Some(hostile) = hostiles.iter_mut().flatten().find(|value| value.operation == operation.0) {
            hostile.observed = Some(match hostile.hostile {
                Generation2dPublicationHostile::Missing => "generation2d-publication.authority-missing",
                Generation2dPublicationHostile::WrongOperation => "generation2d-publication.wrong-operation",
                Generation2dPublicationHostile::WrongGeneration => "generation2d-publication.wrong-generation",
                Generation2dPublicationHostile::WrongBase => "generation2d-publication.wrong-base",
                Generation2dPublicationHostile::WrongParent => "generation2d-publication.wrong-parent",
            });
            match hostile.hostile {
                Generation2dPublicationHostile::Missing => return Err("generation2d-publication.authority-missing"),
                Generation2dPublicationHostile::WrongOperation => lease.operation = lease.operation.wrapping_add(1),
                Generation2dPublicationHostile::WrongGeneration => lease.generation = lease.generation.wrapping_add(1),
                Generation2dPublicationHostile::WrongBase => lease.base_revision = lease.base_revision.wrapping_add(1),
                Generation2dPublicationHostile::WrongParent => lease.parent_revision = lease.parent_revision.wrapping_add(1),
            }
        }
        lease
    };
    generation2d_validate_atomic_lease(lease, operation, generation, live_generation)
}

pub fn generation2d_publication_item_credit(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> Result<usize, &'static str> {
    let lease = generation2d_publication_lease_by_key(generation2d_publication_key(operation, generation))?.ok_or("generation2d-publication.stale-authority")?;
    if lease.maximum_output_pages != GENERATION2D_MOUNTED_OUTPUT_CHANNELS || lease.maximum_controls != GENERATION2D_MOUNTED_CONTROL_CREDITS {
        return Err("generation2d-publication.domain-credits-lost");
    }
    Ok(lease.maximum_items)
}

pub fn generation2d_release_publication_authority(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation) -> bool {
    let Ok(mut leases) = generation2d_publication_leases().try_lock() else { return false };
    leases.take(generation2d_publication_key(operation, generation)).is_some()
}
