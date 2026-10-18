//! 🧮️ Publication credits and snapshot copy cursor: the mounted channel constants, the publication key, credits and authority release, refresh and validation, and the snapshot copy cursor. See [`super`].

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

pub(super) struct Generation2dSnapshotCopyCursor {
    pub(super) target: std::mem::ManuallyDrop<Option<Generation2dSnapshot>>,
    pub(super) phase: u8,
    pub(super) index: usize,
    pub(super) handed_back: bool,
}

impl Generation2dSnapshotCopyCursor {
    pub(super) fn new(source: &Generation2dSnapshot) -> Result<Self, &'static str> {
        let mut target = Generation2dSnapshot {
            host_snapshot: semio_framework_artifact_flow_flow::FlowHostSnapshot {
                schema: String::new(),
                camera: semio_framework_artifact_flow_flow::CameraJson::default(),
                widgets: Vec::new(),
                synapses: Vec::new(),
                layout: semio_framework_artifact_flow_flow::OrderedMap::new(),
            },
            generation: semio_framework_artifact_playbook_playbook::GenerationPlayRoot::default(),
        };
        target.host_snapshot.widgets.try_reserve_exact(source.host_snapshot.widgets.len()).map_err(|_| "generation2d-initializer.widgets-preflight")?;
        target.host_snapshot.synapses.try_reserve_exact(source.host_snapshot.synapses.len()).map_err(|_| "generation2d-initializer.synapses-preflight")?;
        target.generation.cold_builder_mut()?.generations.try_reserve_exact(source.generation.generations.len()).map_err(|_| "generation2d-initializer.generations-preflight")?;
        Ok(Self { target: std::mem::ManuallyDrop::new(Some(target)), phase: 0, index: 0, handed_back: false })
    }

    pub(super) fn step(&mut self, source: &Generation2dSnapshot) -> Result<bool, &'static str> {
        let target = self.target.as_mut().ok_or("generation2d-initializer.copy-owner")?;
        match self.phase {
            0 => {
                target.host_snapshot.schema = generation2d_copy_string(&source.host_snapshot.schema)?;
                self.phase = 1;
            }
            1 => {
                target.host_snapshot.camera.x = source.host_snapshot.camera.x;
                self.phase = 2;
            }
            2 => {
                target.host_snapshot.camera.y = source.host_snapshot.camera.y;
                self.phase = 3;
            }
            3 => {
                target.host_snapshot.camera.zoom = source.host_snapshot.camera.zoom;
                self.phase = 4;
            }
            4 if self.index < source.host_snapshot.widgets.len() => {
                target.host_snapshot.widgets.push(generation2d_copy_widget(&source.host_snapshot.widgets[self.index])?);
                self.index += 1;
            }
            4 => {
                self.phase = 5;
                self.index = 0;
            }
            5 if self.index < source.host_snapshot.synapses.len() => {
                target.host_snapshot.synapses.push(generation2d_copy_synapse(&source.host_snapshot.synapses[self.index])?);
                self.index += 1;
            }
            5 => {
                self.phase = 6;
                self.index = 0;
            }
            6 if self.index < source.host_snapshot.layout.len() => {
                let (id, layout) = source.host_snapshot.layout.iter().nth(self.index).ok_or("generation2d-initializer.layout-owner")?;
                target.host_snapshot.layout.insert(generation2d_copy_string(id)?, semio_framework_artifact_flow_flow::WidgetLayout { x: layout.x, y: layout.y });
                self.index += 1;
            }
            6 => {
                self.phase = 7;
                self.index = 0;
            }
            7 if self.index < source.generation.generations.len() => {
                target.generation.cold_builder_mut()?.generations.push(generation2d_copy_generation(&source.generation.generations[self.index])?);
                self.index += 1;
            }
            7 => {
                target.generation.cold_builder_mut()?.selected_generation_id = match source.generation.selected_generation_id.as_deref() {
                    Some(value) => Some(generation2d_copy_string(value)?),
                    None => None,
                };
                self.phase = 8;
            }
            8 => {
                target.generation.cold_builder_mut()?.preview_text = match source.generation.preview_text.as_deref() {
                    Some(value) => Some(generation2d_copy_string(value)?),
                    None => None,
                };
                self.phase = 9;
            }
            _ => return Ok(true),
        }
        Ok(self.phase == 9)
    }

    pub(super) fn take(&mut self) -> Option<Generation2dSnapshot> {
        if self.phase != 9 || self.handed_back {
            return None;
        }
        self.handed_back = true;
        self.target.take()
    }

    pub(super) fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if self.target.is_none() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.target)
    }

    pub(super) fn abandon(&mut self, active: &mut Option<Box<dyn store::ErasedSnapshotRetirement>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        if self.target.is_none() {
            self.handed_back = true;
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let step = store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.target, active, grant)?;
        if self.target.is_none() {
            self.handed_back = true;
        }
        Ok(step)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.handed_back && self.target.is_none()
    }
}

impl Drop for Generation2dSnapshotCopyCursor {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "Generation2d snapshot copy cursor reached Drop before handoff or terminal-empty close");
    }
}
