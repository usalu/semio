//! 🔚️ Store initialization close of the GIS map document: the bounded close and retirement of the initialization authority and the initialization job constructor. See [`super`].

use super::*;

impl GisMapStoreInitializationAuthority {
    pub(super) fn applied_id(&self, position: usize) -> Option<&str> {
        let envelope = self.envelope.as_ref()?;
        match &envelope.cursor {
            Some(cursor) => cursor.applied_edit_ids.get(position).map(String::as_str),
            None => envelope.vcs.edits.get(position).map(|edit| edit.id.as_str()),
        }
    }

    pub(super) fn redo_id(&self, position: usize) -> Option<&str> {
        self.envelope.as_ref()?.cursor.as_ref()?.redo_edit_ids.get(position).map(String::as_str)
    }

    pub(super) fn fail(&mut self, code: &'static [u8]) {
        self.fault = Some(code.to_vec());
        self.phase = GisMapStoreInitializationPhase::RetireFault;
    }

    pub(super) fn nested(mut demand: RetirementDemand) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "GIS initializer retirement depth overflow"))?;
        Ok(demand)
    }

    fn displaced_factory() -> std::sync::Arc<dyn store::ArtifactOwnedValueRetirementFactory<GisMapSnapshot>> {
        std::sync::Arc::new(GisMapSnapshotRetirementFactory)
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, semio_framework_value::ValueError> {
        if let Some(active) = self.active.as_ref() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if self.pending.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&*self.pending)?);
        }
        if let Some(runtime) = self.runtime.as_ref() {
            return Self::nested(runtime.initialization_retirement_demands(body)?);
        }
        if let Some(clone) = self.clone.as_ref() {
            return clone.close_demands(body);
        }
        if let Some(active) = self.envelope_retirement.as_ref() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if !self.publication.terminal_is_empty() && self.close_core_is_terminal() {
            return Self::nested(self.publication.retirement_demands()?);
        }
        if let Some(envelope) = self.envelope.as_ref() {
            let Some(owners) = self.owners.as_ref() else {
                return Ok(RetirementDemand { copy_bytes: size_of::<Option<store::os_store::document_store_owners::DocumentStoreOwners<GisMapSnapshot, GisMapMutation>>>(), capacity_bytes: store::os_store::document_store_owners::bounded_artifact_store_owners_birth_bytes::<GisMapSnapshot, GisMapMutation>(), depth: 1, ..Default::default() });
            };
            if !owners.constructor_is_complete() {
                return Self::nested(owners.constructor_demands(body)?);
            }
            let mut demand = Self::nested(owners.uninstalled_envelope_retirement_demands(envelope))?;
            demand.copy_bytes = demand.copy_bytes.checked_add(size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "GIS initializer envelope placement overflow"))?;
            return Ok(demand);
        }
        if let Some(owners) = self.owners.as_ref() {
            return if owners.uninstalled_owners_terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: size_of::<Option<store::os_store::document_store_owners::DocumentStoreOwners<GisMapSnapshot, GisMapMutation>>>(), depth: 1, ..Default::default() })
            } else {
                Self::nested(owners.uninstalled_owners_demands(body)?)
            };
        }
        Ok(Default::default())
    }

    pub(super) fn drain_displaced(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending, &mut self.active, child)
    }

    pub(super) fn close_is_terminal(&self) -> bool {
        self.close_core_is_terminal() && self.publication.terminal_is_empty()
    }

    fn close_core_is_terminal(&self) -> bool {
        self.envelope.is_none() && self.runtime.is_none() && self.clone.is_none() && self.active.is_none() && self.pending.is_none() && self.envelope_retirement.is_none() && self.owners.is_none()
    }

    pub(super) fn close_original(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.close_is_terminal() {
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth.max(1) {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "GIS initializer retirement exceeds admitted depth"));
        }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        if self.active.is_some() || self.pending.is_some() {
            return self.drain_displaced(grant);
        }
        if let Some(runtime) = self.runtime.as_mut() {
            let step = runtime.close_step(&Self::displaced_factory(), child)?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                drop(self.runtime.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(clone) = self.clone.as_mut() {
            let step = clone.close_step(child)?;
            if clone.terminal_is_empty() {
                drop(self.clone.take());
            }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(retirement) = self.envelope_retirement.as_mut() {
            if retirement.terminal_is_empty() {
                let released = size_of_val(retirement.as_ref());
                drop(self.envelope_retirement.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: released, ..empty }));
            }
            let step = retirement.close_step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, retirement.terminal_is_empty(), "GIS initializer envelope")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.envelope.is_some() {
            if self.owners.is_none() {
                let placement = demand.copy_bytes;
                let owner_grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - placement, ..grant };
                return match store::os_store::document_store_owners::bounded_artifact_store_owners::<GisMapSnapshot, GisMapMutation>(owner_grant) {
                    Ok((owners, mut receipt)) => {
                        *self.owners = Some(owners);
                        receipt.copied_bytes += placement;
                        Ok(RetainedCloneStep::Progress(receipt))
                    }
                    Err(refused) => {
                        *self.owners = refused.owners;
                        Err(refused.error.with_retained_progress(refused.progress))
                    }
                };
            }
            let owners = self.owners.as_mut().expect("retained initializer owner catalog");
            if !owners.constructor_is_complete() {
                return owners.admit_constructor(child).map(RetainedCloneStep::Progress).map_err(|(error, progress)| error.with_retained_progress(progress));
            }
            let placement = size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>();
            let child = RetainedCloneGrant { maximum_copy_bytes: child.maximum_copy_bytes - placement, ..child };
            let owners = self.owners.take().expect("retained initializer owner catalog");
            let envelope = self.envelope.take().expect("retained initializer envelope");
            return match owners.retire_envelope_uninstalled(envelope, child) {
                Ok((owner, mut receipt)) => {
                    *self.envelope_retirement = Some(owner);
                    receipt.copied_bytes += placement;
                    Ok(RetainedCloneStep::Progress(receipt))
                }
                Err((error, owners, envelope)) => {
                    *self.owners = Some(owners);
                    *self.envelope = Some(envelope);
                    Err(error)
                }
            };
        }
        if let Some(owners) = self.owners.as_mut() {
            if owners.uninstalled_owners_terminal_is_empty() {
                drop(self.owners.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty }));
            }
            let step = owners.close_uninstalled_owners_step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "GIS initializer original catalog")?;
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(empty))
    }

    pub(super) fn terminal_is_empty_inner(&self) -> bool {
        self.terminal_handoff
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.candidate.is_none()
            && self.active.is_none()
            && self.envelope_retirement.is_none()
            && self.owners.is_none()
            && self.pending.is_none()
            && self.publication.terminal_is_empty()
            && self.clone.is_none()
    }
}

impl Drop for GisMapStoreInitializationAuthority {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty_inner(), "GIS store initialization authority reached Drop before exact candidate handoff or retained rejection close");
    }
}

pub fn gis_map_document_store_initialization_job(
    envelope: store::os_store::store_schema::ArtifactEnvelope<GisMapSnapshot, GisMapMutation>,
    operation: semio_framework_job::OperationId,
    generation: semio_framework_job::Generation,
    actor: protocol::ActorId,
) -> semio_framework_plugin::ArtifactStoreInitializationJob<GisMapSnapshot, GisMapMutation> {
    semio_framework_plugin::ArtifactStoreInitializationJob::new(Box::new(GisMapStoreInitializationAuthority::new(envelope, operation, generation, actor)))
}
