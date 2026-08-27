//! 🎚️ Typed canonical Config mutation traversal, without JSON staging.

use super::super::{FlowConfig, FlowConfigMutation};
use store::{ArtifactCanonicalJson, ArtifactCanonicalJsonNode as Json};
use super::{ConfigCopy, ConfigSource, Owner, Retirement};
use super::bytes::{edit_id_byte, edit_id_length, TextCopy};
use std::sync::Arc;

//#region 📬️Preparation
pub(in super::super) struct PreparationFactory;

impl store::ArtifactStoreOneItemPreparationFactory<FlowConfig, FlowConfigMutation> for PreparationFactory {
    fn preflight(&self, mutation: &FlowConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Flow config preparation rejected its lane or description".into());
        }
        super::super::admit_flow_config_mutation(mutation)
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<FlowConfig, FlowConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<FlowConfig, FlowConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<FlowConfig, FlowConfigMutation>> {
        if matches!(request.mutation, FlowConfigMutation::CancelDuplicateWidget { .. }) {
            return store::ArtifactStoreOneItemPreparationFactory::begin(&super::super::FlowStoreOneItemPreparationFactory::new(store::HistoryLane::Document, super::super::admit_flow_config_mutation, super::super::prepare_flow_config), request);
        }
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation()
            || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
            return Err(request);
        }
        Ok(Box::new(Preparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority),
            phase: 0, admission_item: 0, admission_bytes: 0, copy: None, post: None, inverse: None,
            author: None, meta_author: None, id_copy: None, ids: [None, None], sealer: None, authority_retirement: None, checkpoint: Default::default(), retirement: Retirement::default(), cancelled: false, closing: false,
        }))
    }
}

struct Preparation {
    base: Option<store::SnapshotRead<FlowConfig>>,
    mutation: Option<FlowConfigMutation>,
    description: Option<String>,
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    phase: u8,
    admission_item: usize,
    admission_bytes: usize,
    copy: Option<ConfigCopy>,
    post: Option<FlowConfig>,
    inverse: Option<FlowConfigMutation>,
    author: Option<String>,
    meta_author: Option<String>,
    id_copy: Option<TextCopy>,
    ids: [Option<String>; 2],
    sealer: Option<store::ArtifactStoreOneItemSealer<FlowConfig, FlowConfigMutation>>,
    authority_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retirement: Retirement,
    cancelled: bool,
    closing: bool,
}

impl Preparation {
    fn progress(&mut self, bytes: usize) -> store::ArtifactStoreOneItemPreparationStep {
        self.checkpoint.cursor += 1;
        self.checkpoint.completed_items += 1;
        self.checkpoint.completed_bytes += bytes as u64;
        store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint)
    }
}

impl store::ArtifactStoreOneItemPreparation<FlowConfig, FlowConfigMutation> for Preparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use store::ArtifactStoreOneItemPreparationStep as Step;
        if !grant.permits_one() || self.cancelled || self.closing { return Ok(Step::Blocked); }
        if let Some(sealer) = self.sealer.as_mut() {
            let before = sealer.checkpoint().completed_bytes;
            let result = sealer.advance(grant)?;
            let delta = (sealer.checkpoint().completed_bytes - before) as usize;
            let progress = self.progress(delta);
            if matches!(result, Step::Prepared(_)) {
                self.checkpoint.digest = self.sealer.as_ref().unwrap().prepared().unwrap().edit_digest();
                return Ok(Step::Prepared(self.checkpoint));
            }
            return Ok(progress);
        }
        let base = self.base.as_ref().ok_or_else(|| "Flow config preparation lost its base".to_owned())?.get();
        let mutation = self.mutation.as_ref().ok_or_else(|| "Flow config preparation lost its mutation".to_owned())?;
        let mut bytes = 0;
        match self.phase {
            0 => {
                if base.preview_off_node_ids.len() > super::super::FLOW_STORE_MAX_SCENE_ITEMS { return Err("Flow config base exceeds its item envelope".into()); }
                let source = ConfigSource::base(base);
                let length = if self.admission_item < source.preview.len() { source.preview[self.admission_item].len() }
                    else if let Some(text) = source.text.get(self.admission_item - source.preview.len()) { text.len() }
                    else { self.phase = 1; 0 };
                self.admission_bytes = self.admission_bytes.checked_add(length).ok_or_else(|| "Flow config byte count overflow".to_owned())?;
                if self.admission_bytes > super::super::FLOW_STORE_MAX_TEXT_BYTES { return Err("Flow config base exceeds its text envelope".into()); }
                self.admission_item += 1;
            }
            1 => {
                if matches!(mutation, FlowConfigMutation::SetContributions { .. }) { return Err("Flow contribution publication requires post-ACK host synchronization".into()); }
                self.copy = Some(ConfigCopy::new(&ConfigSource::post(base, mutation, false), None));
                self.phase = 2;
            }
            2 => {
                let copy = self.copy.as_mut().unwrap();
                bytes = copy.step(&ConfigSource::post(base, mutation, false), grant.maximum_bytes)?;
                if copy.complete() { self.post = copy.take(); self.copy = None; self.phase = 3; }
            }
            3 => { self.copy = Some(ConfigCopy::new(&ConfigSource::base(base), super::inverse_field(mutation))); self.phase = 4; }
            4 => {
                let copy = self.copy.as_mut().unwrap();
                bytes = copy.step(&ConfigSource::base(base), grant.maximum_bytes)?;
                if copy.complete() {
                    self.inverse = Some(super::inverse(mutation, copy.take().unwrap()));
                    self.copy = None; self.phase = 5;
                }
            }
            5 | 7 => {
                let mut source = ConfigSource::base(base);
                source.text = ["", "", "", "", "", "", self.authority.as_ref().unwrap().actor()];
                self.copy = Some(ConfigCopy::new(&source, Some(7)));
                self.phase += 1;
            }
            6 | 8 => {
                let mut source = ConfigSource::base(base);
                source.text = ["", "", "", "", "", "", self.authority.as_ref().unwrap().actor()];
                let copy = self.copy.as_mut().unwrap();
                bytes = copy.step(&source, grant.maximum_bytes)?;
                if copy.complete() {
                    let value = copy.take().unwrap().locale;
                    if self.phase == 6 { self.author = Some(value); } else { self.meta_author = Some(value); }
                    self.copy = None; self.phase += 1;
                }
            }
            9 | 10 => {
                let sequence = self.authority.as_ref().unwrap().next_sequence_number();
                let metadata = self.phase == 10;
                let copy = self.id_copy.get_or_insert_with(TextCopy::default);
                bytes = copy.advance_ascii(edit_id_length(sequence, metadata), |index| edit_id_byte(sequence, index, metadata), grant.maximum_bytes)?.unwrap_or(0);
                if copy.complete() { self.ids[usize::from(metadata)] = copy.take(); self.id_copy = None; self.phase += 1; }
            }
            11 => {
                let authority = self.authority.as_ref().unwrap();
                let edit = protocol::Edit {
                    mutation_meta: vec![protocol::MutationMeta {
                        mutation_id: self.ids[1].take().map(protocol::MutationId), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
                        author_id: self.meta_author.take().map(protocol::ActorId), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                        payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
                    }],
                    id: self.ids[0].take().unwrap(), actor: self.author.take(), forwards: vec![self.mutation.take().unwrap()], inverse: vec![self.inverse.take().unwrap()],
                    description: self.description.take(), coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
                };
                self.sealer = Some(authority.begin_one_item_seal(edit, Arc::new(self.post.take().unwrap()), Arc::new(RetirementFactory), Arc::new(RetirementFactory)));
                self.authority = None;
                self.phase = 12;
            }
            _ => return Err("Flow config preparation has invalid phase".into()),
        }
        Ok(self.progress(bytes))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<FlowConfig, FlowConfigMutation>> { self.sealer.as_ref().and_then(|owner| owner.prepared()) }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<FlowConfig, FlowConfigMutation>> { self.sealer.as_mut().and_then(|owner| owner.take_prepared()) }
    fn cancel(&mut self) { self.cancelled = true; if let Some(sealer) = self.sealer.as_mut() { sealer.cancel(); } }
    fn begin_close(&mut self) { self.closing = true; if let Some(sealer) = self.sealer.as_mut() { sealer.begin_close(); } }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        use store::SnapshotRetirementStep as Step;
        if !self.closing || !grant.permits_one() { return Ok(Step::Blocked); }
        if !self.retirement.is_empty() { return store::ErasedSnapshotRetirement::close_step(&mut self.retirement, grant.maximum_items, grant.maximum_bytes); }
        if let Some(owner) = self.authority_retirement.as_mut() {
            let step = owner.close_step(grant.maximum_items, grant.maximum_bytes)?;
            if step == Step::Complete {
                if !owner.terminal_is_empty() { return Err("Flow authority retirement closed with live owners".into()); }
                self.authority_retirement = None;
                return Ok(Step::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(sealer) = self.sealer.as_mut() {
            let step = sealer.close_step(grant)?;
            if matches!(step, Step::Complete) {
                if !sealer.terminal_is_empty() { return Err("Flow config sealer closed with retained owners".into()); }
                self.sealer = None;
            }
            return Ok(if matches!(step, Step::Complete) { Step::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(copy) = self.id_copy.take() { copy.retire(&mut self.retirement); }
        else if let Some(value) = self.ids.iter_mut().find_map(Option::take) { self.retirement.push(Owner::Bytes(value.into_bytes())); }
        else if let Some(copy) = self.copy.take() { copy.retire(&mut self.retirement); }
        else if let Some(value) = self.post.take() { self.retirement.push(Owner::Config(value)); }
        else if let Some(value) = self.mutation.take().or_else(|| self.inverse.take()) { self.retirement.push(Owner::ConfigMutation(value)); }
        else if let Some(value) = self.description.take().or_else(|| self.author.take()).or_else(|| self.meta_author.take()) { self.retirement.push(Owner::Bytes(value.into_bytes())); }
        else if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Flow config preparation could not return its exact base".into()); }
        }
        else if let Some(authority) = self.authority.take() { self.authority_retirement = Some(authority.retire()); }
        else { return Ok(Step::Complete); }
        Ok(Step::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.retirement.is_empty() && self.sealer.is_none() && self.copy.is_none() && self.id_copy.is_none() && self.ids.iter().all(Option::is_none) && self.post.is_none()
            && self.mutation.is_none() && self.inverse.is_none() && self.description.is_none() && self.author.is_none() && self.meta_author.is_none()
            && self.base.is_none() && self.authority.is_none() && self.authority_retirement.is_none()
    }
}

struct RetirementFactory;

impl store::ArtifactOwnedValueRetirementFactory<FlowConfig> for RetirementFactory {
    fn retire_owned(&self, value: FlowConfig) -> Box<dyn store::ErasedSnapshotRetirement> {
        let mut retirement = Retirement::default();
        retirement.push(Owner::Config(value));
        Box::new(retirement)
    }
}

impl store::ArtifactOwnedValueRetirementFactory<FlowConfigMutation> for RetirementFactory {
    fn retire_owned(&self, value: FlowConfigMutation) -> Box<dyn store::ErasedSnapshotRetirement> {
        let mut retirement = Retirement::default();
        retirement.push(Owner::ConfigMutation(value));
        Box::new(retirement)
    }
}

impl store::SnapshotRetirementFactory<FlowConfig> for RetirementFactory {
    fn retire(&self, snapshot: Arc<FlowConfig>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(SnapshotRetirement { snapshot: Some(snapshot), retirement: Retirement::default() })
    }
}

struct SnapshotRetirement {
    snapshot: Option<Arc<FlowConfig>>,
    retirement: Retirement,
}

impl store::ErasedSnapshotRetirement for SnapshotRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes == 0 { return Ok(store::SnapshotRetirementStep::Blocked); }
        if let Some(snapshot) = self.snapshot.take() {
            if let Some(value) = Arc::into_inner(snapshot) { self.retirement.push(Owner::Config(value)); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        store::ErasedSnapshotRetirement::close_step(&mut self.retirement, maximum_items, maximum_bytes)
    }
    fn terminal_is_empty(&self) -> bool { self.snapshot.is_none() && self.retirement.is_empty() }
}
//#endregion 📬️Preparation

//#region 🔣️CanonicalTraversal
enum Node<'a> {
    Scalar(Json<'a>),
    Config(&'a FlowConfig),
    Camera(&'a flow::CameraJson),
    Strings(&'a [String]),
    Mutation(&'a FlowConfigMutation),
    Payload(&'a FlowConfigMutation),
}

impl<'a> Node<'a> {
    fn resolve(mut self, path: &[usize]) -> Result<Self, String> {
        for &index in path { self = self.child(index)?; }
        Ok(self)
    }

    fn json(self) -> Json<'a> {
        match self {
            Self::Scalar(value) => value,
            Self::Config(_) => Json::Object(13),
            Self::Camera(_) => Json::Object(3),
            Self::Strings(values) => Json::Array(values.len()),
            Self::Mutation(_) | Self::Payload(_) => Json::Object(1),
        }
    }

    fn child(self, index: usize) -> Result<Self, String> {
        Ok(match self {
            Self::Config(value) => match index {
                0 => Self::Strings(&value.preview_off_node_ids),
                1 => Self::Camera(&value.camera),
                2 => Self::Scalar(Json::String(&value.lod_mode)),
                3 => Self::Scalar(Json::F64(value.proximity_distance)),
                4 => Self::Scalar(Json::Bool(value.grid_visible)),
                5 => Self::Scalar(Json::Bool(value.grid_snap_enabled)),
                6 => Self::Scalar(Json::F64(value.grid_factor)),
                7 => Self::Scalar(Json::String(&value.catalogue_sections_json)),
                8 => Self::Scalar(Json::String(&value.automation_enabled_json)),
                9 => Self::Scalar(Json::String(&value.contributions_json)),
                10 => Self::Scalar(Json::String(&value.generation_json)),
                11 => Self::Scalar(Json::String(&value.duplicate_widget_progress_json)),
                12 => Self::Scalar(Json::String(&value.locale)),
                _ => return Err("Flow canonical config index outside fields".into()),
            },
            Self::Camera(value) => Self::Scalar(Json::F64(match index {
                0 => value.x, 1 => value.y, 2 => value.zoom,
                _ => return Err("Flow canonical camera index outside fields".into()),
            })),
            Self::Strings(values) => Self::Scalar(Json::String(values.get(index).ok_or_else(|| "Flow canonical preview index outside values".to_owned())?)),
            Self::Mutation(value) if index == 0 => Self::Payload(value),
            Self::Payload(value) if index == 0 => match value {
                FlowConfigMutation::Snapshot { config } => Self::Config(config),
                FlowConfigMutation::SetPreviewOff { node_ids } => Self::Strings(node_ids),
                FlowConfigMutation::SetCamera { camera } => Self::Camera(camera),
                FlowConfigMutation::SetLodMode { value } | FlowConfigMutation::SetLocale { value } => Self::Scalar(Json::String(value)),
                FlowConfigMutation::SetProximityDistance { value } | FlowConfigMutation::SetGridFactor { value } => Self::Scalar(Json::F64(*value)),
                FlowConfigMutation::SetGridVisible { value } | FlowConfigMutation::SetGridSnapEnabled { value } => Self::Scalar(Json::Bool(*value)),
                FlowConfigMutation::SetContributions { json } | FlowConfigMutation::SetAutomationEnabled { json }
                | FlowConfigMutation::SetGeneration { json } | FlowConfigMutation::SetDuplicateWidgetProgress { json }
                | FlowConfigMutation::SetCatalogueSections { sections_json: json } => Self::Scalar(Json::String(json)),
                FlowConfigMutation::CancelDuplicateWidget { generation } => Self::Scalar(Json::U64(*generation)),
            },
            _ => return Err("Flow canonical path is not a container child".into()),
        })
    }

    fn key(self, index: usize) -> Result<&'static str, String> {
        let key = match self {
            Self::Config(_) => ["previewOffNodeIds", "camera", "lodMode", "proximityDistance", "gridVisible", "gridSnapEnabled", "gridFactor", "catalogueSectionsJson", "automationEnabledJson", "contributionsJson", "generationJson", "duplicateWidgetProgressJson", "locale"].get(index).copied(),
            Self::Camera(_) => ["x", "y", "zoom"].get(index).copied(),
            Self::Mutation(value) if index == 0 => Some(match value {
                FlowConfigMutation::SetContributions { .. } => "SetContributions",
                FlowConfigMutation::Snapshot { .. } => "Snapshot",
                FlowConfigMutation::SetPreviewOff { .. } => "SetPreviewOff",
                FlowConfigMutation::SetCamera { .. } => "SetCamera",
                FlowConfigMutation::SetLodMode { .. } => "SetLodMode",
                FlowConfigMutation::SetProximityDistance { .. } => "SetProximityDistance",
                FlowConfigMutation::SetGridVisible { .. } => "SetGridVisible",
                FlowConfigMutation::SetGridSnapEnabled { .. } => "SetGridSnapEnabled",
                FlowConfigMutation::SetGridFactor { .. } => "SetGridFactor",
                FlowConfigMutation::SetCatalogueSections { .. } => "SetCatalogueSections",
                FlowConfigMutation::SetAutomationEnabled { .. } => "SetAutomationEnabled",
                FlowConfigMutation::SetGeneration { .. } => "SetGeneration",
                FlowConfigMutation::SetDuplicateWidgetProgress { .. } => "SetDuplicateWidgetProgress",
                FlowConfigMutation::CancelDuplicateWidget { .. } => "CancelDuplicateWidget",
                FlowConfigMutation::SetLocale { .. } => "SetLocale",
            }),
            Self::Payload(value) if index == 0 => Some(match value {
                FlowConfigMutation::Snapshot { .. } => "config",
                FlowConfigMutation::SetPreviewOff { .. } => "node_ids",
                FlowConfigMutation::SetCamera { .. } => "camera",
                FlowConfigMutation::SetCatalogueSections { .. } => "sections_json",
                FlowConfigMutation::CancelDuplicateWidget { .. } => "generation",
                FlowConfigMutation::SetContributions { .. } | FlowConfigMutation::SetAutomationEnabled { .. }
                | FlowConfigMutation::SetGeneration { .. } | FlowConfigMutation::SetDuplicateWidgetProgress { .. } => "json",
                _ => "value",
            }),
            _ => None,
        };
        key.ok_or_else(|| "Flow canonical key outside object fields".to_owned())
    }
}

impl ArtifactCanonicalJson for FlowConfigMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<Json<'_>, String> {
        Ok(Node::Mutation(self).resolve(path)?.json())
    }

    fn canonical_json_key(&self, object_path: &[usize], index: usize) -> Result<&str, String> {
        Node::Mutation(self).resolve(object_path)?.key(index)
    }
}
//#endregion 🔣️CanonicalTraversal

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::SpaceMember;

    #[semio_framework_async_macros::async_test]
    async fn max_semantic_config_publication_cancel_retry_and_close_use_real_grants() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🎯️grant-frontier.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            for grant_value in fixture["preparationGrantBytes"].as_array().unwrap() {
                let maximum_bytes = grant_value.as_u64().unwrap() as usize;
                for cancel in [None, Some(row["cancelAt"].as_u64().unwrap() as usize)] {
                    let text = row["unit"].as_str().unwrap().repeat(row["repetitions"].as_u64().unwrap() as usize);
                    let initial = FlowConfig::default();
                    let expected = if cancel.is_some() { initial.catalogue_sections_json.clone() } else { text.clone() };
                    let envelope = store::create_document_envelope::<FlowConfig, FlowConfigMutation>("flow.config", "grant-frontier", initial, None);
                    let mut store = store::ArtifactStore::new(envelope).await.unwrap();
                    store.install_member_store_owners_exact(store::MemberStoreOwners::new(
                        Arc::new(RetirementFactory), Arc::new(RetirementFactory), Arc::new(RetirementFactory),
                        Box::new(store::ArtifactStoreCursorDisposer::<FlowConfig, FlowConfigMutation>::new()),
                    ));
                    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes };
                    let generation = store.generation_now();
                    let mut publication = store.begin_apply_one(
                        semio_framework_job::OperationId(1), generation, store.content_revision_now(), "flow-test".into(),
                        FlowConfigMutation::SetCatalogueSections { sections_json: text }, None, store::HistoryLane::Document, Some(&PreparationFactory),
                    ).unwrap();
                    let mut last_bytes = 0;
                    let mut finished = false;
                    for step in 0..500_000 {
                        if cancel == Some(step) { publication.begin_close(); finished = true; break; }
                        let result = store.advance_apply_one(&mut publication, grant).unwrap();
                        let bytes = publication.progress().completed_bytes;
                        assert!(bytes >= last_bytes && bytes - last_bytes <= maximum_bytes as u64);
                        last_bytes = bytes;
                        if matches!(result, store::ArtifactStoreOneItemAdvance::Published(_)) {
                            assert!(cancel.is_none());
                            assert!(publication.retry());
                            assert_eq!(store.generation_now(), generation + 1);
                            assert!(publication.acknowledge());
                            finished = true;
                            break;
                        }
                    }
                    assert!(finished, "preparation reached its live frontier");
                    assert_eq!(store.snapshot().unwrap().catalogue_sections_json, expected);
                    for _ in 0..500_000 {
                        let step = publication.close_step(grant).unwrap();
                        if let store::SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
                            assert!(released_items <= 1 && released_bytes <= maximum_bytes);
                        }
                        if step == store::SnapshotRetirementStep::Complete { break; }
                    }
                    assert!(publication.terminal_is_empty());
                    for _ in 0..500_000 {
                        if store.close_owned_step(1, maximum_bytes).unwrap() == store::SnapshotRetirementStep::Complete { break; }
                    }
                    assert!(store.close_owned_terminal_is_empty());
                    eprintln!("[DEBUG] Flow semantic config case={} grant={} cancel={cancel:?} closed=true", row["id"], maximum_bytes);
                }
            }
        }
    }

    fn oracle(value: &impl ArtifactCanonicalJson, path: &mut Vec<usize>) -> serde_json::Value {
        match value.canonical_json_node(path).unwrap() {
            Json::Null => serde_json::Value::Null,
            Json::Bool(value) => value.into(),
            Json::U64(value) => value.into(),
            Json::F64(value) => serde_json::json!(value),
            Json::String(value) => value.into(),
            Json::Array(length) => serde_json::Value::Array((0..length).map(|index| {
                path.push(index); let item = oracle(value, path); path.pop(); item
            }).collect()),
            Json::Object(length) => {
                let mut object = serde_json::Map::new();
                for index in 0..length {
                    let key = value.canonical_json_key(path, index).unwrap().to_owned();
                    path.push(index); let item = oracle(value, path); path.pop();
                    object.insert(key, item);
                }
                serde_json::Value::Object(object)
            }
            other => panic!("unexpected Flow scalar: {other:?}"),
        }
    }

    #[test]
    fn typed_config_canonical_tree_matches_serde_for_every_variant() {
        let text = "\0🌊\"\\".repeat(1024);
        let variants = [
            FlowConfigMutation::Snapshot { config: FlowConfig::default() },
            FlowConfigMutation::SetContributions { json: text.clone() },
            FlowConfigMutation::SetPreviewOff { node_ids: vec![text.clone(), "next".into()] },
            FlowConfigMutation::SetCamera { camera: flow::CameraJson { x: 1.0, y: -0.0, zoom: 2.0 } },
            FlowConfigMutation::SetLodMode { value: text.clone() },
            FlowConfigMutation::SetProximityDistance { value: 3.0 },
            FlowConfigMutation::SetGridVisible { value: true },
            FlowConfigMutation::SetGridSnapEnabled { value: false },
            FlowConfigMutation::SetGridFactor { value: 4.0 },
            FlowConfigMutation::SetCatalogueSections { sections_json: text.clone() },
            FlowConfigMutation::SetAutomationEnabled { json: text.clone() },
            FlowConfigMutation::SetGeneration { json: text.clone() },
            FlowConfigMutation::SetDuplicateWidgetProgress { json: text.clone() },
            FlowConfigMutation::CancelDuplicateWidget { generation: u64::MAX },
            FlowConfigMutation::SetLocale { value: text },
        ];
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🎯️grant-frontier.json")).unwrap();
        assert_eq!(variants.len(), fixture["canonicalVariants"].as_array().unwrap().len());
        for (index, value) in variants.into_iter().enumerate() {
            assert_eq!(value.canonical_json_key(&[], 0).unwrap(), fixture["canonicalVariants"][index].as_str().unwrap());
            assert_eq!(oracle(&value, &mut Vec::new()), serde_json::to_value(&value).unwrap());
            assert!(value.canonical_json_node(&[9]).is_err());
            assert!(value.canonical_json_key(&[], 1).is_err());
        }
    }
}
//#endregion 🧪️Tests
