//! 📬️ Flow-owned construction, portable identity, and Store-owned canonical sealing are distinct frontiers.

use super::{recipe::Recipe, SceneHash};
use crate::artifacts::flow::retirement::{MutationRetirementFactory, SnapshotRetirementFactory};
use super::super::{bytes::{edit_id_byte, edit_id_length, TextCopy}, Owner, Retirement};
use super::super::super::{FlowMutation, FlowSnapshot, FlowWorkingScene, FLOW_STORE_MAX_MUTATION_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ArtifactStoreOneItemGrant as Grant, ArtifactStoreOneItemPreparationStep as Advance, SnapshotRetirementStep as Close};

//#region 📬️Factory
pub(in super::super::super) struct PreparationFactory;

impl store::ArtifactStoreOneItemPreparationFactory<FlowSnapshot, FlowMutation> for PreparationFactory {
    fn preflight(&self, mutation: &FlowMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) { return Err("Flow artifact preparation rejected lane or description".into()); }
        if !Recipe::supported(mutation) { return Err("Flow mutation requires its explicit batch-only recipe".into()); }
        let work_items = match mutation { FlowMutation::MoveWidgets(payload) => payload.entries.len(), _ => 1 };
        if work_items == 0 || work_items > FLOW_STORE_MAX_MUTATION_ITEMS { return Err("Flow artifact mutation exceeds admitted semantic items".into()); }
        Ok(store::ArtifactStoreOneItemFootprint { work_items, retained_bytes: FLOW_STORE_MAX_TEXT_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<FlowSnapshot, FlowMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<FlowSnapshot, FlowMutation>>, store::ArtifactStoreOneItemPreparationRequest<FlowSnapshot, FlowMutation>> {
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES || !Recipe::supported(&request.mutation) { return Err(request); }
        Ok(Box::new(Preparation { state: ManuallyDrop::new(PreparationState {
            base: Some(request.base), mutation: Some(Arc::new(request.mutation)), description: request.description, authority: Some(request.authority),
            scene: None, scene_root: None, inverse: None, post: None, hash: None, mutation_reader: None, recipe: None, text: None,
            texts: Default::default(), source_digest: None, digest: None, sealer: None, external_retirement: None,
            retirement: Retirement::default(), phase: 0, phase_bytes: 0, checkpoint: Default::default(), cancelled: false, closing: false, failed: false,
        }) }))
    }
}
//#endregion 📬️Factory

//#region 🧵️Preparation
struct PreparationState {
    base: Option<store::SnapshotRead<FlowSnapshot>>, mutation: Option<Arc<FlowMutation>>, description: Option<String>,
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>, scene: Option<FlowWorkingScene>, scene_root: Option<Arc<FlowWorkingScene>>,
    inverse: Option<Vec<FlowMutation>>, post: Option<FlowSnapshot>, hash: Option<SceneHash>,
    mutation_reader: Option<store::ArtifactCanonicalJsonReader<FlowMutation>>, recipe: Option<Recipe>, text: Option<TextCopy>, texts: [Option<String>; 10],
    source_digest: Option<[u8; 32]>, digest: Option<[u8; 32]>, sealer: Option<store::ArtifactStoreOneItemSealer<FlowSnapshot, FlowMutation>>,
    external_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>, retirement: Retirement,
    phase: u8, phase_bytes: usize, checkpoint: store::ArtifactStoreOneItemCheckpoint, cancelled: bool, closing: bool, failed: bool,
}

struct Preparation { state: ManuallyDrop<PreparationState> }

fn progress(state: &mut PreparationState, bytes: usize) -> Advance {
    state.checkpoint.cursor += 1; state.checkpoint.completed_items += 1; state.checkpoint.completed_bytes += bytes as u64;
    Advance::Progress(state.checkpoint)
}

impl store::ArtifactStoreOneItemPreparation<FlowSnapshot, FlowMutation> for Preparation {
    fn advance(&mut self, grant: Grant) -> Result<Advance, String> {
        let state = &mut *self.state;
        if state.failed { return Err("Flow canonical mutation encoding failed".into()); }
        if !grant.permits_one() || state.closing || state.cancelled { return Ok(Advance::Blocked); }
        if let Some(sealer) = state.sealer.as_mut() {
            let before = sealer.checkpoint().completed_bytes; let step = sealer.advance(grant)?; let bytes = (sealer.checkpoint().completed_bytes - before) as usize;
            let result = progress(state, bytes);
            if matches!(step, Advance::Prepared(_)) { state.checkpoint.digest = state.sealer.as_ref().unwrap().prepared().unwrap().edit_digest(); return Ok(Advance::Prepared(state.checkpoint)); }
            return Ok(if matches!(step, Advance::Blocked) { Advance::Blocked } else { result });
        }
        match state.phase {
            0 => {
                let base = state.base.as_ref().ok_or("Flow preparation lost base")?.get();
                let scene = base.content.local_owner::<FlowWorkingScene>().ok_or("Flow preparation requires exact local scene owner")?;
                state.hash = Some(SceneHash::new(scene)); state.phase = 1;
            }
            1 => {
                let hash = state.hash.as_mut().unwrap(); let bytes = hash.advance(grant)?; state.phase_bytes += bytes;
                if state.phase_bytes > FLOW_STORE_MAX_TEXT_BYTES + crate::artifacts::flow::FLOW_CONTENT_ID_DOMAIN.len() { return Err("Flow artifact base exceeds admitted canonical byte envelope".into()); }
                if hash.complete() { let (root, digest) = hash.take().unwrap(); state.scene_root = Some(root); state.source_digest = Some(digest); hash.begin_close(); state.phase = 2; }
                return Ok(progress(state, bytes));
            }
            2 | 11 => {
                let hash = state.hash.as_mut().unwrap();
                match hash.close_step(grant)? {
                    Close::Complete => {
                        if !hash.terminal_is_empty() { return Err("Flow identity reader closed with owners".into()); }
                        state.hash = None; state.phase = if state.phase == 2 { 3 } else { 100 };
                    }
                    Close::Pending { released_bytes, .. } => return Ok(progress(state, released_bytes)),
                    Close::Blocked => return Ok(Advance::Blocked),
                }
            }
            3 => {
                state.mutation_reader = Some(store::ArtifactCanonicalJsonReader::new(Arc::clone(state.mutation.as_ref().unwrap()), Arc::new(MutationRetirementFactory)));
                state.phase_bytes = 0; state.phase = 4;
            }
            4 => {
                let reader = state.mutation_reader.as_mut().unwrap(); let mut chunk = [0; 256];
                let bytes = match reader.encode_chunk(grant, &mut chunk) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        reader.cancel(); reader.begin_close(); state.failed = true;
                        state.phase_bytes += error.written_bytes;
                        state.retirement.push(Owner::Bytes(error.reason.into_bytes()));
                        return Ok(progress(state, error.written_bytes));
                    }
                };
                state.phase_bytes += bytes;
                if state.phase_bytes > FLOW_STORE_MAX_TEXT_BYTES { return Err("Flow artifact mutation exceeds admitted canonical byte envelope".into()); }
                if reader.is_complete() { reader.begin_close(); state.phase = 5; }
                return Ok(progress(state, bytes));
            }
            5 => {
                let reader = state.mutation_reader.as_mut().unwrap();
                match reader.close_step(grant)? {
                    Close::Complete => { if !reader.terminal_is_empty() { return Err("Flow mutation reader closed with owners".into()); } state.mutation_reader = None; state.phase = 6; }
                    Close::Pending { released_bytes, .. } => return Ok(progress(state, released_bytes)),
                    Close::Blocked => return Ok(Advance::Blocked),
                }
            }
            6 => { state.recipe = Some(Recipe::new(state.scene_root.take().unwrap(), Arc::clone(state.mutation.as_ref().unwrap()))); state.phase = 7; }
            7 => {
                let recipe = state.recipe.as_mut().unwrap();
                if recipe.complete() { let (scene, inverse) = recipe.take().unwrap(); state.scene = Some(scene); state.inverse = Some(inverse); recipe.begin_close(); state.phase = 8; }
                else { return Ok(match recipe.advance(grant)? { Some(bytes) => progress(state, bytes), None => Advance::Blocked }); }
            }
            8 => {
                let recipe = state.recipe.as_mut().unwrap();
                match recipe.close_step(grant)? {
                    Close::Complete => { if !recipe.terminal_is_empty() { return Err("Flow recipe closed with owners".into()); } state.recipe = None; state.phase = 9; }
                    Close::Pending { released_bytes, .. } => return Ok(progress(state, released_bytes)),
                    Close::Blocked => return Ok(Advance::Blocked),
                }
            }
            9 => { state.hash = Some(SceneHash::new(Arc::new(state.scene.take().unwrap()))); state.phase = 10; }
            10 => {
                let hash = state.hash.as_mut().unwrap(); let bytes = hash.advance(grant)?;
                if hash.complete() {
                    let (root, digest) = hash.take().unwrap(); state.scene_root = Some(root); state.digest = Some(digest); hash.begin_close(); state.phase = 11;
                    if matches!(&**state.mutation.as_ref().unwrap(), FlowMutation::ReplaceWidget(_)) && state.source_digest == state.digest { return Err("Flow widget replacement is a no-op".into()); }
                }
                return Ok(progress(state, bytes));
            }
            100..=109 => {
                let index = (state.phase - 100) as usize;
                let copy = state.text.get_or_insert_with(TextCopy::default);
                let bytes = match index {
                    3 | 4 => {
                        const PREFIX: &[u8] = b"flow-content-sha256-";
                        let digest = state.digest.unwrap();
                        copy.advance_ascii(PREFIX.len() + 64, |position| {
                            if position < PREFIX.len() { PREFIX[position] } else { let position = position - PREFIX.len(); let byte = digest[position / 2]; b"0123456789abcdef"[if position % 2 == 0 { (byte >> 4) as usize } else { (byte & 15) as usize }] }
                        }, grant.maximum_bytes)?
                    }
                    8 | 9 => {
                        let sequence = state.authority.as_ref().unwrap().next_sequence_number() as u64;
                        copy.advance_ascii(edit_id_length(sequence, index == 9), |position| edit_id_byte(sequence, position, index == 9), grant.maximum_bytes)?
                    }
                    _ => {
                        let source = match index {
                            0 => state.base.as_ref().unwrap().get().schema.as_str(),
                            1 | 2 => state.authority.as_ref().unwrap().actor(),
                            5 => "s.stdio.semio", 6 => "v1", 7 => "flow", _ => unreachable!(),
                        };
                        copy.advance(source, grant.maximum_bytes)?
                    }
                };
                if copy.complete() { state.texts[index] = copy.take(); state.text = None; state.phase += 1; }
                return Ok(match bytes { Some(bytes) => progress(state, bytes), None => Advance::Blocked });
            }
            110 => {
                let target = store::os_io::ArtifactRef { artifact_id: state.texts[4].take().unwrap(), dialect: store::os_io::ArtifactDialect {
                    artifact_kind: state.texts[5].take().unwrap(), standard: state.texts[6].take().unwrap(), subset: state.texts[7].take().unwrap(),
                } };
                let content = crate::artifacts::flow::FlowContentChild::new(state.texts[3].take().unwrap(), target).with_local_owner(state.scene_root.take().unwrap());
                state.post = Some(FlowSnapshot { schema: state.texts[0].take().unwrap(), camera: state.base.as_ref().unwrap().get().camera.clone(), content }); state.phase = 111;
            }
            111 => {
                let mutation = match Arc::try_unwrap(state.mutation.take().unwrap()) { Ok(mutation) => mutation, Err(root) => { state.mutation = Some(root); return Err("Flow mutation still has an active traversal owner".into()); } };
                let authority = state.authority.as_ref().unwrap();
                let edit = protocol::Edit {
                    id: state.texts[8].take().unwrap(), actor: state.texts[1].take(), forwards: vec![mutation], inverse: state.inverse.take().unwrap(),
                    mutation_meta: vec![protocol::MutationMeta {
                        mutation_id: state.texts[9].take().map(protocol::MutationId), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
                        author_id: state.texts[2].take().map(protocol::ActorId), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                        payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
                    }],
                    description: state.description.take(), coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
                };
                state.sealer = Some(authority.begin_one_item_seal(edit, Arc::new(state.post.take().unwrap()), Arc::new(MutationRetirementFactory), Arc::new(SnapshotRetirementFactory)));
                state.authority = None; state.phase = 112;
            }
            _ => return Err("Flow artifact preparation phase is invalid".into()),
        }
        Ok(progress(state, 0))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.state.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<FlowSnapshot, FlowMutation>> { self.state.sealer.as_ref().and_then(|sealer| sealer.prepared()) }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<FlowSnapshot, FlowMutation>> { self.state.sealer.as_mut().and_then(|sealer| sealer.take_prepared()) }
    fn cancel(&mut self) { self.state.cancelled = true; if let Some(sealer) = self.state.sealer.as_mut() { sealer.cancel(); } }
    fn begin_close(&mut self) {
        self.state.closing = true;
        if let Some(sealer) = self.state.sealer.as_mut() { sealer.begin_close(); }
        if let Some(hash) = self.state.hash.as_mut() { hash.begin_close(); }
        if let Some(reader) = self.state.mutation_reader.as_mut() { reader.begin_close(); }
        if let Some(recipe) = self.state.recipe.as_mut() { recipe.begin_close(); }
    }

    fn close_step(&mut self, grant: Grant) -> Result<Close, String> {
        let state = &mut *self.state;
        if !state.closing || !grant.permits_one() { return Ok(Close::Blocked); }
        if !state.retirement.is_empty() { return store::ErasedSnapshotRetirement::close_step(&mut state.retirement, 1, grant.maximum_bytes); }
        if let Some(owner) = state.external_retirement.as_mut() {
            let step = owner.close_step(1, grant.maximum_bytes)?;
            if step == Close::Complete { if !owner.terminal_is_empty() { return Err("Flow external retirement is nonterminal".into()); } state.external_retirement = None; return Ok(Close::Pending { released_items: 1, released_bytes: 0 }); }
            return Ok(step);
        }
        macro_rules! close_cursor {
            ($field:ident) => { if let Some(owner) = state.$field.as_mut() {
                let step = owner.close_step(grant)?;
                if step == Close::Complete { if !owner.terminal_is_empty() { return Err("Flow preparation cursor closed with live owners".into()); } state.$field = None; return Ok(Close::Pending { released_items: 1, released_bytes: 0 }); }
                return Ok(step);
            } };
        }
        close_cursor!(sealer); close_cursor!(hash); close_cursor!(mutation_reader); close_cursor!(recipe);
        if let Some(text) = state.text.take() { text.retire(&mut state.retirement); }
        else if let Some(text) = state.texts.iter_mut().find_map(Option::take).or_else(|| state.description.take()) { state.retirement.push(Owner::Bytes(text.into_bytes())); }
        else if let Some(inverse) = state.inverse.take() { state.retirement.push(Owner::Mutations(inverse)); }
        else if let Some(post) = state.post.take() { state.external_retirement = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&SnapshotRetirementFactory, post)); }
        else if let Some(scene) = state.scene.take() { state.retirement.push(Owner::Scene(scene)); }
        else if let Some(scene) = state.scene_root.take() { if let Some(scene) = Arc::into_inner(scene) { state.retirement.push(Owner::Scene(scene)); } }
        else if let Some(mutation) = state.mutation.take() { if let Some(mutation) = Arc::into_inner(mutation) { state.retirement.push(Owner::Mutation(mutation)); } }
        else if let Some(base) = state.base.take() { if !base.return_to_registry() { return Err("Flow preparation failed to return exact base root".into()); } }
        else if let Some(authority) = state.authority.take() { state.external_retirement = Some(authority.retire()); }
        else { state.source_digest = None; state.digest = None; return Ok(Close::Complete); }
        Ok(Close::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self) -> bool {
        let state = &*self.state;
        state.closing && state.base.is_none() && state.mutation.is_none() && state.description.is_none() && state.authority.is_none()
            && state.scene.is_none() && state.scene_root.is_none() && state.inverse.is_none() && state.post.is_none() && state.hash.is_none()
            && state.mutation_reader.is_none() && state.recipe.is_none() && state.text.is_none() && state.texts.iter().all(Option::is_none)
            && state.sealer.is_none() && state.external_retirement.is_none() && state.retirement.is_empty() && state.source_digest.is_none() && state.digest.is_none()
    }
}

impl Drop for Preparation {
    fn drop(&mut self) {
        if store::ArtifactStoreOneItemPreparation::terminal_is_empty(self) { unsafe { ManuallyDrop::drop(&mut self.state); } }
        else if !std::thread::panicking() { panic!("Flow artifact preparation must close before drop"); }
    }
}
//#endregion 🧵️Preparation

//#region 🧪️StorePublication
#[cfg(test)]
mod tests {
    use super::*;
    use store::SpaceMember;

    #[semio_framework_async_macros::async_test]
    async fn semantic_artifact_prepare_publish_retry_cancel_and_close_use_production_and_one_byte_grants() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️fixtures/🧬️artifact-recipes.json")).unwrap();
        let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
        for bytes in fixture["grants"].as_array().unwrap() {
            let grant = Grant { maximum_items: 1, maximum_bytes: bytes.as_u64().unwrap() as usize };
            for row in fixture["cases"].as_array().unwrap() {
                for cancel in [None, Some(0), Some(131), Some(5001)] {
                    let scene = super::super::recipe::tests::source(&label);
                    let content = crate::artifacts::flow::flow_content_child_handle_and_cache(scene.widgets, scene.synapses, scene.layout);
                    let initial = FlowSnapshot { schema: "flow".into(), camera: flow::CameraJson::default(), content };
                    let initial_scene = initial.content.local_owner::<FlowWorkingScene>().unwrap(); let baseline = serde_json::Value::from(dsl::ToValue::to_value(&*initial_scene)); drop(initial_scene);
                    let envelope = store::create_document_envelope::<FlowSnapshot, FlowMutation>("flow.flow", "retained-recipe", initial, None);
                    let mut store = store::ArtifactStore::new(envelope).await.unwrap();
                    store.install_member_store_owners_exact(crate::artifacts::flow::retirement::store_owners());
                    let generation = store.generation_now();
                    let mutation = dsl::FromValue::from_value(dsl::DslValue::from(row["mutation"].clone())).unwrap();
                    let mut publication = store.begin_apply_one(semio_framework_job::OperationId(1), generation, store.content_revision_now(), "flow-test".into(), mutation, None, store::HistoryLane::Document, Some(&PreparationFactory)).unwrap();
                    let mut finished = false; let mut published = false; let mut previous = 0;
                    for step in 0..500_000 {
                        if cancel == Some(step) { publication.begin_close(); finished = true; break; }
                        let outcome = store.advance_apply_one(&mut publication, grant).unwrap(); let bytes = publication.progress().completed_bytes;
                        assert!(bytes >= previous && bytes - previous <= grant.maximum_bytes as u64); previous = bytes;
                        if matches!(outcome, store::ArtifactStoreOneItemAdvance::Published(_)) {
                            assert!(publication.retry()); assert_eq!(store.generation_now(), generation + 1); assert!(publication.acknowledge()); finished = true; published = true; break;
                        }
                    }
                    assert!(finished);
                    {
                        let snapshot = store.snapshot().unwrap(); let scene = snapshot.content.local_owner::<FlowWorkingScene>().unwrap(); let json = serde_json::Value::from(dsl::ToValue::to_value(&*scene));
                        if !published { assert_eq!(json, baseline); }
                        else {
                            assert_eq!(json["widgets"].as_array().unwrap().iter().map(|widget| widget["id"].clone()).collect::<Vec<_>>(), *row["widgets"].as_array().unwrap());
                            assert_eq!(json["synapses"].as_array().unwrap().iter().map(|edge| edge["id"].clone()).collect::<Vec<_>>(), *row["synapses"].as_array().unwrap());
                            if row["id"] == "replace-widget" { assert_eq!(json["widgets"][1]["label"], "changed"); }
                            if row["id"] == "move-widget" { assert_eq!(json["layout"]["b"], serde_json::json!({ "x": 5.0, "y": 7.0 })); }
                        }
                    }
                    for _ in 0..500_000 {
                        let step = publication.close_step(grant).unwrap();
                        if let Close::Pending { released_items, released_bytes } = step { assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes); }
                        if step == Close::Complete { break; }
                    }
                    assert!(publication.terminal_is_empty());
                    for _ in 0..500_000 {
                        let step = store.close_owned_step(1, grant.maximum_bytes).unwrap();
                        if let Close::Pending { released_items, released_bytes } = step { assert!(released_items <= 1 && released_bytes <= grant.maximum_bytes); }
                        if step == Close::Complete { break; }
                    }
                    assert!(store.close_owned_terminal_is_empty());
                    eprintln!("[DEBUG] Flow artifact recipe={} grant={} cancel={cancel:?} published={published} terminal=true", row["id"], grant.maximum_bytes);
                }
            }
        }
    }
}
//#endregion 🧪️StorePublication
