//! 🧷️ Identity and recovery slot of the mounted FEM 3D session: the app instance identity, the recovery publication and the recovery slot. See [`super`].

use super::*;

//#region 💼️MountedSession
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Identity {
    pub(super) app_instance_id: u32,
    pub(super) base_revision: RevisionId,
    pub(super) generation: Generation,
    pub(super) canonical_base_revision: [u8; 32],
    pub(super) operation: OperationId,
    pub(super) job: u64,
}

impl Identity {
    pub(super) fn freshness(self, preview_sequence: u64) -> Fem3dVisualFreshness {
        Fem3dVisualFreshness {
            app_instance_id: self.app_instance_id,
            model_revision: self.base_revision.0,
            document_generation: self.generation.0,
            operation: self.operation.0,
            numerical_preview_sequence: preview_sequence,
            surface_generation: self.generation.0,
            renderer_scene_generation: self.generation.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MountedRecoveryPublication {
    Retained,
    Recover,
}

pub(super) struct MountedRecoverySlot {
    pub(super) reserved: Cell<Option<Identity>>,
    pub(super) publication: Cell<Option<(Identity, MountedRecoveryPublication)>>,
    pub(super) owner: Mutex<Option<MountedState>>,
}

impl MountedRecoverySlot {
    pub(super) fn new() -> Self {
        Self { reserved: Cell::new(None), publication: Cell::new(None), owner: Mutex::new(None) }
    }

    pub(super) fn reserve(&self, identity: Identity) -> bool {
        let owner = self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if self.reserved.get().is_some() || self.publication.get().is_some() || owner.is_some() {
            return false;
        }
        self.reserved.set(Some(identity));
        true
    }

    pub(super) fn publish(&self, identity: Identity, publication: MountedRecoveryPublication) -> bool {
        if self.reserved.get() != Some(identity) {
            return false;
        }
        match self.publication.get() {
            None => self.publication.set(Some((identity, publication))),
            Some(current) if current == (identity, publication) => {}
            Some(_) => return false,
        }
        true
    }

    pub(super) fn publish_owner(&self, identity: Identity, state: MountedState) -> Result<(), MountedState> {
        if !self.publish(identity, MountedRecoveryPublication::Recover) {
            return Err(state);
        }
        let mut owner = self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if owner.is_some() {
            return Err(state);
        }
        *owner = Some(state);
        Ok(())
    }

    pub(super) fn take_owner(&self, identity: Identity) -> Option<MountedState> {
        (self.reserved.get() == Some(identity)).then(|| self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take()).flatten()
    }

    pub(super) fn restore_owner(&self, identity: Identity, state: MountedState) -> Result<(), MountedState> {
        if self.reserved.get() != Some(identity) {
            return Err(state);
        }
        let mut owner = self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if owner.is_some() {
            return Err(state);
        }
        *owner = Some(state);
        Ok(())
    }

    pub(super) fn clear_publication(&self, identity: Identity) -> bool {
        if self.publication.get().is_some_and(|current| current.0 == identity) {
            self.publication.set(None);
            return true;
        }
        false
    }

    pub(super) fn release(&self, identity: Identity) -> bool {
        let owner = self.owner.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if self.reserved.get() != Some(identity) || self.publication.get().is_some() || owner.is_some() {
            return false;
        }
        self.reserved.set(None);
        true
    }

    pub(super) fn contains(&self, app_instance_id: u32) -> bool {
        self.reserved.get().is_some_and(|identity| identity.app_instance_id == app_instance_id)
    }
}
//#endregion 💼️MountedSession
