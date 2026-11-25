//! 📬️ Process3d config and artifact store preparation: retained byte measures, grants and the one-item factories.

use super::*;

//#region 📬️ConfigStorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(super) struct Process3dConfigStorePreparationFactory;

struct Process3dConfigStorePreparation {
    owners: store::OneItemOwners<Process3dConfig, Process3dConfigMutation>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    last_work: u64,
}

fn process3d_config_retained_bytes(config: &Process3dConfig) -> Option<usize> {
    [config.engagement_input.len(), config.sun_color.len(), config.contributions_json.len()].into_iter().try_fold(0usize, usize::checked_add)
}

fn process3d_config_mutation_retained_bytes(mutation: &Process3dConfigMutation) -> usize {
    match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value }) => value.len(),
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ color, .. }) => color.len(),
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json }) => json.len(),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ .. }) | Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ .. }) => 0,
    }
}

pub(crate) fn admit_process3d_config_mutation(mutation: &Process3dConfigMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = process3d_config_mutation_retained_bytes(mutation);
    let envelope = match mutation {
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ .. }) => PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES,
        _ => PROCESS3D_RETAINED_RAW_BYTES,
    };
    if retained_bytes > envelope {
        return Err("Process3d config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

pub(crate) fn prepare_process3d_config(base: &Process3dConfig, mutation: &Process3dConfigMutation) -> Result<(Process3dConfig, Vec<Process3dConfigMutation>), String> {
    admit_process3d_config_mutation(mutation)?;
    if process3d_config_retained_bytes(base).is_none_or(|bytes| bytes > PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES) {
        return Err("Process3d config base exceeds its fixed retained preparation envelope".into());
    }
    let inverse = match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ .. }) => Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: base.engagement_input.clone() }),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ .. }) => Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position: base.camera_position, target: base.camera_target, fov: base.camera_fov }),
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ .. }) => Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: base.sun_enabled, azimuth: base.sun_azimuth, elevation: base.sun_elevation, intensity: base.sun_intensity, color: base.sun_color.clone() }),
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ .. }) => Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: base.contributions_json.clone() }),
        Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ .. }) => Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: base.resolved_up_to }),
    };
    let mut post = base.clone();
    match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value }) => post.engagement_input = value.clone(),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position, target, fov }) => {
            post.camera_position = *position;
            post.camera_target = *target;
            post.camera_fov = *fov;
        }
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled, azimuth, elevation, intensity, color }) => {
            post.sun_enabled = *enabled;
            post.sun_azimuth = *azimuth;
            post.sun_elevation = *elevation;
            post.sun_intensity = *intensity;
            post.sun_color = color.clone();
        }
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json }) => post.contributions_json = json.clone(),
        Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value }) => post.resolved_up_to = *value,
    }
    if process3d_config_retained_bytes(&post).is_none_or(|bytes| bytes > PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES) {
        return Err("Process3d config post-state exceeds its fixed retained preparation envelope".into());
    }
    Ok((post, vec![inverse]))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Process3dConfig, Process3dConfigMutation> for Process3dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Process3dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Process3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Process3dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Process3d config preparation rejected its lane".into());
        }
        admit_process3d_config_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Process3dConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Process3dConfigStorePreparation>(), depth: 1 })
    }

    fn begin_copy_byte_demand(&self, mutation: &Process3dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<usize, semio_framework_value::ValueError> {
        self.begin_demand(mutation, lane)?;
        size_of::<store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation, Process3dConfigMutation>>().checked_add(size_of::<Process3dConfigStorePreparation>()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original preparation header quote overflow"))
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation, Process3dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<
        (Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dConfig, Process3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress),
        (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation, Process3dConfigMutation>),
    > {
        if request.lane != store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Process3dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), last_work: 0, cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dConfig, Process3dConfigMutation> for Process3dConfigStorePreparation {
    fn advance(&mut self, _body: usize, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, _receiving: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::OriginalReceivingPrimary> {
        self.advance_inner(grant).map_err(Into::into)
    }
    fn last_turn_work(&self) -> u64 {
        self.last_work
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn has_pending_receiving_recovery(&self) -> bool {
        false
    }

    fn pending_receiving_demand(&self) -> Option<semio_framework_value::RetirementDemand> {
        None
    }

    fn pending_receiving_recovery(&mut self) -> Option<semio_framework_value::native_decoding::construction::NativeRecoveryLoan<'_>> {
        None
    }

    fn finish_receiving_recovery(&mut self, _: &mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<semio_framework_value::retained_clone::RetainedCloneProgress, semio_framework_value::OriginalReceivingPrimary> {
        semio_framework_value::RetainedWorkOutcome { result: Ok(Default::default()), work: 0 }
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll {
        self.owners.close_step(effect)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

impl Process3dConfigStorePreparation {
    fn advance_inner(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        self.last_work = 0;
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(error) = self.owners.failure.as_ref() {
            return Err(error.clone());
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Process3d config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its mutation owner"))?;
        let (post, inverse) = match prepare_process3d_config(base.get(), &mutation) {
            Ok(candidate) => candidate,
            Err(message) => {
                *self.owners.mutation = Some(mutation);
                *self.owners.failure = Some(ValueError::new(ValueRefusalKind::OwnershipLimit, message.clone()));
                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, message));
            }
        };
        let Some(authority) = self.owners.authority.as_ref() else {
            *self.owners.mutation = Some(mutation);
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its Store authority"));
        };
        let edit = authority.next_edit(mutation, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.last_work = 1;
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
    }
}
//#endregion 📬️ConfigStorePreparation

//#region 📬️ArtifactStorePreparation
pub(crate) use semio_s_artifact_process_process3d_host::{process3d_document_bytes, process3d_document_preparation_factory, process3d_mutation_retained_bytes, PROCESS3D_DOCUMENT_GRANT_BYTES, PROCESS3D_DOCUMENT_MAXIMUM_BYTES, PROCESS3D_DOCUMENT_TEXT_BYTES};

/// 🧮️ Runs the mutation's own semantic `diff`/`inverse` against `base` and applies the resulting diff,
/// so the retained lane and the batch lane can never diverge. An `Error`/`Fatal` outcome (a duplicate
/// step id, a missing target) is a REJECTION here, not a silent no-op: `MutationOutcome::error`/
/// `fatal` force an EMPTY diff, and publishing anyway would write a no-op edit into history.
pub(crate) fn prepare_process3d_document(base: &Process3dSnapshot, mutation: &Process3dMutation) -> Result<(Process3dSnapshot, Vec<Process3dMutation>), String> {
    process3d_mutation_retained_bytes(mutation).map_err(semio_framework_value::ValueError::into_message)?;
    process3d_document_bytes(base).map_err(semio_framework_value::ValueError::into_message)?;
    let outcome = <Process3dMutation as protocol::ColdMutation<Process3dSnapshot>>::diff(mutation, base);
    if let Some(message) = outcome.messages().iter().find(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        return Err(format!("Process3d document mutation was refused by its own vocabulary: {}", message.message));
    }
    let inverse = <Process3dMutation as protocol::ColdMutation<Process3dSnapshot>>::inverse(mutation, base).map_err(semio_framework_value::ValueError::into_message)?;
    let post = protocol::apply_diff_cold(outcome.diff(), base).map_err(|error| format!("Process3d document mutation could not apply onto its exact base: {}", error.message))?;
    process3d_document_bytes(&post).map_err(semio_framework_value::ValueError::into_message)?;
    Ok((post, inverse))
}

//#endregion 📬️ArtifactStorePreparation
