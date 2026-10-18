//! 📬️ Config store preparation: the config preparation factory and byte counter, the config edit bytes and fault and the one item publication implementations of the config store. See [`super`].

use super::*;

//#region 📬️ConfigStorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct AnimatePresentationConfigPreparationFactory;

pub struct AnimatePresentationConfigPreparation {
    owners: store::OneItemOwners<PresentationConfig, PresentationConfigMutation>,
    pub(super) serialized_bytes: Option<usize>,
    pub(super) checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    pub(super) cancelled: bool,
}

pub struct AnimatePresentationConfigByteCounter {
    pub(super) bytes: usize,
}

impl std::io::Write for AnimatePresentationConfigByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.bytes.saturating_add(bytes.len()) > ANIMATE_PRESENTATION_CONFIG_STEP_BYTES {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
        }
        self.bytes += bytes.len();
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn animate_presentation_config_edit_bytes(edit: &protocol::Edit<PresentationConfigMutation>) -> Result<usize, String> {
    let mut counter = AnimatePresentationConfigByteCounter { bytes: 0 };
    use std::io::Write as _;
    counter.write_all(semio_framework_pack_json::to_json_string(edit).as_bytes()).map_err(|_| "Animate Presentation config edit exceeds its serialized byte envelope".to_string())?;
    Ok(counter.bytes)
}

fn animate_presentation_config_fault(message: &'static str) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, message)
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<PresentationConfig, PresentationConfigMutation> for AnimatePresentationConfigPreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<PresentationConfigMutation>>>, grant: RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<PresentationConfigMutation>>, RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &PresentationConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        let mutation_bytes = match mutation {
            PresentationConfigMutation::SetEngagementInput(payload) => payload.value.len(),
        };
        if lane != store::os_store::store_schema::HistoryLane::Document || mutation_bytes > ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES {
            return Err("Animate Presentation config preparation rejected its lane or byte envelope".into());
        }
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, ANIMATE_PRESENTATION_CONFIG_STEP_BYTES))
    }

    fn begin_demand(&self, mutation: &PresentationConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        self.preflight(mutation, lane).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Animate Presentation config preparation has no admissible lane or byte envelope"))?;
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<AnimatePresentationConfigPreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<PresentationConfig, PresentationConfigMutation, PresentationConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<(Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<PresentationConfig, PresentationConfigMutation>>, RetainedCloneProgress), (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<PresentationConfig, PresentationConfigMutation, PresentationConfigMutation>)> {
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        if request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((animate_presentation_config_fault("Animate Presentation config preparation original request refused"), request));
        }
        Ok((Box::new(AnimatePresentationConfigPreparation {
            owners: store::OneItemOwners::from_request(request),
            serialized_bytes: None,
            checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<PresentationConfig, PresentationConfigMutation> for AnimatePresentationConfigPreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        if !grant.permits_one() || grant.maximum_copy_bytes < ANIMATE_PRESENTATION_CONFIG_STEP_BYTES || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.refused.is_some() || self.owners.failure.is_some() {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        if self.owners.candidate.is_none() && self.owners.sealed.is_none() {
            let base = self.owners.base.as_ref().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its exact base root"))?.get();
            let base_bytes = base.engagement_input.len();
            if base_bytes > ANIMATE_PRESENTATION_CONFIG_BASE_BYTES {
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Animate Presentation config base exceeds retained byte capacity"));
            }
            let mutation = self.owners.mutation.take().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its mutation owner"))?;
            let mut post = base.clone();
            let inverse = match &mutation {
                PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value }) => {
                    post.engagement_input = value.clone();
                    PresentationConfigMutation::SetEngagementInput(crate::editor::animate::config::SetEngagementInput { value: base.engagement_input.clone() })
                }
            };
            *self.owners.candidate = Some((post, vec![inverse], mutation));
            self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: base_bytes as u64, digest: [0; 32] };
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress { copied_items: 1, copied_bytes: base_bytes, ..Default::default() }));
        }
        if self.owners.sealed.is_none() {
            let (post, inverse, forward) = self.owners.candidate.take().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its candidate"))?;
            let Some(authority) = self.owners.authority.as_ref() else {
                *self.owners.candidate = Some((post, inverse, forward));
                return Err(animate_presentation_config_fault("Animate Presentation config preparation lost its Store authority"));
            };
            *self.owners.sealed = Some((post, authority.next_edit(forward, inverse)));
        }
        if self.serialized_bytes.is_none() {
            let (post, edit) = self.owners.sealed.as_ref().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its semantic edit"))?;
            let bytes = animate_presentation_config_edit_bytes(edit).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Animate Presentation config edit exceeds its serialized byte envelope"))?;
            if bytes.saturating_add(post.engagement_input.len()).saturating_add(512) > ANIMATE_PRESENTATION_CONFIG_STEP_BYTES {
                return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Animate Presentation config publication exceeds the 4096-byte complete envelope"));
            }
            self.serialized_bytes = Some(bytes);
            self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.checkpoint.completed_bytes.saturating_add(bytes as u64), digest: [0; 32] };
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
        }
        let (post, edit) = self.owners.sealed.take().ok_or_else(|| animate_presentation_config_fault("Animate Presentation config preparation lost its validated edit"))?;
        let Some(authority) = self.owners.authority.as_ref() else {
            *self.owners.sealed = Some((post, edit));
            return Err(animate_presentation_config_fault("Animate Presentation config preparation lost its Store authority"));
        };
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 3, completed_items: 3, completed_bytes: self.checkpoint.completed_bytes.saturating_add(self.serialized_bytes.unwrap_or(0) as u64), digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<PresentationConfig, PresentationConfigMutation>> {
        self.owners.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<PresentationConfig, PresentationConfigMutation>> {
        self.owners.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.owners.begin_close();
    }
    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️ConfigStorePreparation
