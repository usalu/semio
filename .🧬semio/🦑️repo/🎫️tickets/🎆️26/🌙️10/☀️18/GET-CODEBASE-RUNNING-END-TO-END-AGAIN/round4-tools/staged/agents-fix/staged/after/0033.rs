//! 🧪️ The generic `Mutation<S>` structural edit prepares insert, remove, replace and multi-row removal over documents of any size, restores the base through its ordered inverse rows and conserves custody through every cancellation frontier.
use semio_framework_os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use semio_framework_os_store_one_item_batch::paged_one_item as paged;
use semio_framework_os_spr as protocol;
use paged::mutation_apply_preparation_factory;

#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

use semio_framework_os_spr::{ApplyCapability, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationComposition, MutationDiff, MutationDiffParticipation, MutationInvertibility, MutationLanguageSurface, MutationLeafDescriptor, MutationOutcome, MutationOutcomeClass, OpBinary};
use semio_framework_os_store_one_item::{ArtifactStoreOneItemGrant, ArtifactStoreOneItemPreparationStep};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPreparationRequest};
use semio_framework_os_store_command::HistoryLane;
use semio_framework_value_snapshot_read::{SnapshotRead, SnapshotReadRegistryHandle};
use semio_framework_os_spr::Mutation;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use semio_framework_value::retirement::{OwnedValueRetirementFactory, SharedValueRetirementFactory};
use semio_framework_value::{FromValue, ToValue, ValueError, ValueRefusalKind};
use serde_json::{json, Value};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

thread_local! {
    static ORIGINAL_ISSUER_RETIRES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static APPLY_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static INVERSE_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static DIFF_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[path="🌳️immutable-root/🦀️.rs"]
mod immutable_root;
use immutable_root::*;
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
enum Op { Insert { index: usize, id: String, text: String }, Remove { index: usize }, SetText { index: usize, text: String } }

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
struct Diff { ops: Vec<Op> }

fn refusal(message: &str) -> MutationApplyError { MutationApplyError { code: "mutation.apply.missing-target".into(), message: message.into(), target: Vec::new() } }

impl<D> MutationDiff<D> for Diff {

}
impl ColdMutationDiff<PlainDocument> for Diff {
    fn absorb(&mut self,other:Self){self.ops.extend(other.ops);}

    fn apply(&self,next:&mut PlainDocument,_capability:ApplyCapability)->MutationApplyResult<()>{
        APPLY_CALLS.set(APPLY_CALLS.get()+1);
        let mut value=next.plain();
        let mut length=value.nodes.len();
        for op in &self.ops{
            match op{Op::Insert{index,..}if *index<=length=>length+=1,Op::Remove{index}if *index<length=>length-=1,Op::SetText{index,..}if *index<length=>{},_=>return Err(refusal("node index out of range"))}
        }
        for op in &self.ops{
            match op{Op::Insert{index,id,text}=>value.nodes.insert(*index,PlainNode{id:id.clone(),text:text.clone()}),Op::Remove{index}=>{value.nodes.remove(*index);},Op::SetText{index,text}=>value.nodes[*index].text=text.clone()}
        }
        next.replace_plain(value);Ok(())
    }
}
impl<D> DiffAlgebra<D> for Diff{
    fn is_empty(&self)->bool{self.ops.is_empty()}
}
impl ColdDiffAlgebra<PlainDocument> for Diff{
    fn inverse(&self,base:&PlainDocument)->Self{
        let mut nodes=base.plain().nodes;let mut inverse=Vec::new();
        for op in &self.ops{
            match op{Op::Insert{index,id,text}=>{nodes.insert(*index,PlainNode{id:id.clone(),text:text.clone()});inverse.push(Op::Remove{index:*index});},Op::Remove{index}=>{let node=nodes.remove(*index);inverse.push(Op::Insert{index:*index,id:node.id,text:node.text});},Op::SetText{index,text}=>inverse.push(Op::SetText{index:*index,text:std::mem::replace(&mut nodes[*index].text,text.clone())})}
        }
        inverse.reverse();Self{ops:inverse}
    }
}
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
enum DocumentMutation { Insert { index: usize, id: String, text: String }, Remove { index: usize }, SetText { index: usize, text: String }, RemoveRange { start: usize, count: usize }, Diagnostic { level: String, code: String, message: String } }

const fn descriptor(kind: &'static str, variant: &'static str) -> MutationLeafDescriptor {
    MutationLeafDescriptor {
        schema_version: 1,
        owner: "store/paged-one-item/mutation-apply",
        semantic_kind: kind,
        display_name: kind,
        emoji: "🧬️",
        aggregate_variant: variant,
        payload_schema: "test.document",
        text_opcode: None,
        binary_tag: None,
        invertibility: MutationInvertibility::ExplicitMutation,
        diff_participation: MutationDiffParticipation::Detect,
        outcome_classes: &[MutationOutcomeClass::Applied],
        composition: MutationComposition::Atomic,
        required_language_surfaces: &[MutationLanguageSurface::Rust],
    }
}

fn out_of_range() -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, "node index out of range") }

impl<D> Mutation<D> for DocumentMutation {
    type Diff = Diff;
    const DESCRIPTORS: &'static [MutationLeafDescriptor] = &[descriptor("insert", "Insert"), descriptor("remove", "Remove"), descriptor("set-text", "SetText"), descriptor("remove-range", "RemoveRange"), descriptor("diagnostic", "Diagnostic")];
    fn descriptor(&self) -> &'static MutationLeafDescriptor {
        &<Self as Mutation<D>>::DESCRIPTORS[match self { Self::Insert { .. } => 0, Self::Remove { .. } => 1, Self::SetText { .. } => 2, Self::RemoveRange { .. } => 3, Self::Diagnostic { .. } => 4 }]
    }
    fn inverse_rows(&self) -> usize { match self { Self::RemoveRange { count, .. } => (*count).max(1), _ => 1 } }
}
impl ColdMutation<PlainDocument> for DocumentMutation {
    type ReferenceDiff = Diff;
    fn diff(&self, _base: &PlainDocument) -> MutationOutcome<Diff> {
        DIFF_CALLS.set(DIFF_CALLS.get() + 1);
        let outcome = MutationOutcome::new(Diff {
            ops: match self {
                Self::Insert { index, id, text } => vec![Op::Insert { index: *index, id: id.clone(), text: text.clone() }],
                Self::Remove { index } => vec![Op::Remove { index: *index }],
                Self::SetText { index, text } => vec![Op::SetText { index: *index, text: text.clone() }],
                Self::RemoveRange { start, count } => (0..*count).map(|_| Op::Remove { index: *start }).collect(),
                Self::Diagnostic { .. } => vec![Op::SetText { index: 0, text: "diagnostic applied".into() }],
            },
        });
        match self {
            Self::Diagnostic { level, code, message } => {
                let row = match level.as_str() {
                    "info" => protocol::MutationMessage::info(code.clone(), message.clone()),
                    "warning" => protocol::MutationMessage::warning(code.clone(), message.clone()),
                    "error" => protocol::MutationMessage::error(code.clone(), message.clone()),
                    "fatal" => protocol::MutationMessage::fatal(code.clone(), message.clone()),
                    _ => unreachable!(),
                };
                outcome.absorb_messages([row.at(["a"])])
            }
            _ => outcome,
        }
    }
    fn inverse(&self, base: &PlainDocument) -> Result<Vec<Self>, ValueError> {
        INVERSE_CALLS.set(INVERSE_CALLS.get() + 1);
        let node = |index: usize| base.node_parts(index).ok_or_else(out_of_range);
        Ok(match self {
            Self::Diagnostic { .. } => vec![Self::SetText { index: 0, text: node(0)?.1.clone() }],
            Self::Insert { index, .. } if *index <= base.node_len() => vec![Self::Remove { index: *index }],
            Self::Insert { .. } => return Err(out_of_range()),
            Self::Remove { index } => { let node = node(*index)?; vec![Self::Insert { index: *index, id: node.0.clone(), text: node.1.clone() }] }
            Self::SetText { index, .. } => vec![Self::SetText { index: *index, text: node(*index)?.1.clone() }],
            Self::RemoveRange { start, count } => (0..*count).map(|offset| node(start + offset).map(|node| Self::Insert { index: start + offset, id: node.0.clone(), text: node.1.clone() })).collect::<Result<_, _>>()?,
        })
    }
}

impl OpBinary for DocumentMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> { serde_json::to_vec(self).map_err(|error| semio_framework_os_spr::ProtocolError::Io(error.to_string())) }
    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> { serde_json::from_slice(bytes).map_err(|error| semio_framework_os_spr::ProtocolError::Io(error.to_string())) }
}

fn fixture() -> Value { serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap() }
fn document(json: &Value) -> Document { serde_json::from_value(json.clone()).unwrap() }
fn mutation(json: &Value) -> DocumentMutation { serde_json::from_value(json.clone()).unwrap() }
fn rows(json: &Value) -> Vec<DocumentMutation> { json.as_array().unwrap().iter().map(mutation).collect() }
fn pairs(document:&impl FixtureDocument)->Value{Value::Array((0..document.node_len()).map(|index|{let(id,text)=document.node_parts(index).unwrap();json!([id,text])}).collect())}

fn grant(copy: usize, capacity: usize) -> ArtifactStoreOneItemGrant {
    ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: 1 << 26, maximum_depth: 4096 }
}
const WIDE: usize = 1 << 26;

struct OriginalSource<T: semio_framework_value::retirement::RetireOwned + Sync>(semio_framework_value::retained_clone::RetainedCloneSource<T>);

impl<T: semio_framework_value::retirement::RetireOwned + Sync> OriginalSource<T> {
    fn new(original: T) -> Self {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneSource};
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: RetainedCloneSource::<T>::constructor_copy_bytes(), maximum_capacity_bytes: RetainedCloneSource::<T>::owned_constructor_capacity_bytes::<()>(), maximum_depth: 1, ..Default::default() };
        Self(RetainedCloneSource::admit_owned(original, (), grant).unwrap_or_else(|_| panic!("original test source admission")).0)
    }
    fn borrow(&self) -> semio_framework_value::retained_clone::RetainedCloneRef<'_, T> { self.0.try_borrow().unwrap() }
}

impl<T: semio_framework_value::retirement::RetireOwned + Sync> OriginalSource<T> {
    fn close(&mut self) {
        if std::thread::panicking() { return; }
        for _ in 0..100000 {
            if self.0.terminal_is_empty() { return; }
            let demand = self.0.next_close_demands(4096).unwrap();
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096.max(demand.copy_bytes), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            assert!(self.0.close_step(grant).unwrap().progress().fits(grant));
        }
        panic!("original test source did not physically close");
    }
}

impl<T:semio_framework_value::retirement::RetireOwned+Sync> Drop for OriginalSource<T>{
    fn drop(&mut self){assert!(std::thread::panicking()||self.0.terminal_is_empty(),"test original source requires explicit granted close");}
}
struct Stall;

enum Outcome { Completed { post: Document, inverse: Vec<DocumentMutation>, turns: usize, retained: usize, peak: usize }, Cancelled, Refused(ValueError) }

impl semio_framework_os_store_snapshot_clone::config_apply::ConfigApplyMutation<Document> for DocumentMutation {
    fn exchange(self, post: &mut i64) -> Result<Self, (ValueError, Self)> {
         Ok(Self { value: std::mem::replace( post,self.value) })
        }

}

fn prepare(build: &dyn Fn() -> (Document, DocumentMutation), grant: ArtifactStoreOneItemGrant, cancel_at: Option<usize>) -> Outcome {
    prepare_with(&mutation_apply_preparation_factory::<Document, DocumentMutation>(), build, grant, cancel_at)
}

fn prepare_with<P:FixtureDocument+semio_framework_value::retirement::RetireOwned+Sync>(factory: &Arc<dyn ArtifactStoreOneItemPreparationFactory<P, DocumentMutation>>, build: &dyn Fn() -> (P, DocumentMutation), grant: ArtifactStoreOneItemGrant, cancel_at: Option<usize>) -> Outcome {
    let ((document, mutation), _) = observe(build);
    let registry = SnapshotReadRegistryHandle::new();
    let root = Arc::new(document);
    let lease = registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_| panic!("registry admission"));
    let authority = Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), [0; 32], 0, 1, semio_framework_os_spr::HybridLogicalTimestamp::new(1, 0), "actor".into(), None, None, None));
    let mutation_retirement: Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<DocumentMutation>> = Arc::new(OriginalMutationIssuer);
    let snapshot_retirement: Arc<dyn semio_framework_value::SnapshotRetirementFactory<P>> = Arc::new(SharedValueRetirementFactory::<P>::default());
    let request = ArtifactStoreOneItemPreparationRequest { operation: semio_framework_job::OperationId(1), generation: semio_framework_job::Generation(1), base_revision: [0; 32], lane: HistoryLane::Document, authority, base: SnapshotRead::new(Arc::clone(&root), lease), mutation, mutation_retirement: Arc::clone(&mutation_retirement), snapshot_retirement: Arc::clone(&snapshot_retirement) };
    let footprint = factory.preflight(&request.mutation, request.lane);
    let begun = factory.begin(request, grant);
    let mut owner = match begun {
        Ok((owner, receipt)) => { assert!(receipt.fits(grant.retained_grant())); owner }
        Err((error, request)) => { let ArtifactStoreOneItemPreparationRequest{base,mutation,authority,..}=request;drain_fixture(base);drain_fixture(mutation);drain_fixture(authority);drain_fixture(root);return if footprint.is_err() { Outcome::Refused(error) } else { std::panic::panic_any(Stall) }; }
    };
    let mut prepared = None;
    let (mut idle, mut turns, mut refused, mut retained) = (0, 0, None, 0usize);
    let (mut live, mut peak) = (0usize, 0usize);
    for turn in 0..1_000_000 {
        if cancel_at == Some(turn) { owner.cancel(); break; }
        match owner.advance(grant) {
            Ok(ArtifactStoreOneItemPreparationStep::Prepared(_, progress)) => { assert!(progress.fits(grant.retained_grant())); retained += progress.retained_capacity_bytes; live += progress.retained_capacity_bytes; peak = peak.max(live); turns = turn + 1; assert_eq!(owner.prepared().unwrap().outcome_messages(), owner.outcome_messages(), "borrowed ready candidate must expose the same original diagnostics"); prepared = owner.take_prepared(); break; }
            Ok(ArtifactStoreOneItemPreparationStep::Progress(_, progress)) => { assert!(progress.fits(grant.retained_grant()), "receipt {progress:?} exceeds {grant:?}"); retained += progress.retained_capacity_bytes; live += progress.retained_capacity_bytes; peak = peak.max(live); live = live.saturating_sub(progress.released_bytes); idle = 0; }
            Ok(ArtifactStoreOneItemPreparationStep::Blocked) => { idle += 1; if idle >= 64 { owner.cancel(); owner.begin_close(); close(&mut owner); drop(owner); drain_fixture(root);std::panic::panic_any(Stall); } }
            Err(error) => { refused = Some(error); break; }
        }
    }
    let outcome = match (prepared, refused) {
        (Some(prepared), _) => {
            let outcome = Outcome::Completed { post: prepared.post_snapshot().as_ref().plain(), inverse: prepared.edit().inverse.iter().cloned().collect(), turns, retained, peak, messages: prepared.outcome_messages().to_vec() };
            let (mut retirement, progress) = prepared.admit_retirement(mutation_retirement, snapshot_retirement, grant.retained_grant()).map_err(|(error, ..)| error).unwrap();
            assert!(progress.fits(grant.retained_grant()));
            for turn in 0..100_000 {
                if retirement.terminal_is_empty() { break; }
                let copy = retirement.next_copy_byte_demand().unwrap();
                let quoted = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: retirement.next_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: retirement.next_release_byte_demand().unwrap(), maximum_depth: retirement.next_depth_demand().unwrap() };
                assert!(retirement.close_step(quoted).unwrap().progress().fits(quoted));
                assert!(turn < 99_999, "prepared original owners must reach typed terminal closure");
            }
            assert!(retirement.terminal_is_empty());
            outcome
        },
        (None, Some(error)) => Outcome::Refused(error),
        (None, None) => Outcome::Cancelled,
    };
    owner.begin_close();
    close(&mut owner);
    assert_eq!(observe(|| drop(owner)).1.requested_bytes, 0);
    drain_fixture(root);
    outcome
}

fn close<P>(owner: &mut Box<dyn ArtifactStoreOneItemPreparation<P, DocumentMutation>>) {
    for _ in 0..100_000 {
        if owner.terminal_is_empty() { return; }
        let copy = owner.next_close_copy_byte_demand().unwrap();
        let quote = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: owner.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: owner.next_close_release_byte_demand().unwrap(), maximum_depth: owner.next_close_depth_demand().unwrap() };
        let (step, heap) = observe(|| owner.close_step(quote).unwrap());
        assert!(step.progress().fits(quote.retained_grant()), "close receipt {:?} exceeds its quote {quote:?}", step.progress());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes), "close accounting disagrees with its receipt");
    }
    panic!("preparation did not close under its quoted grants");
}

fn completed(outcome: Outcome) -> (PlainDocument, Vec<DocumentMutation>, usize) {
    match outcome { Outcome::Completed { post, inverse, turns, .. } => (post, inverse, turns), Outcome::Cancelled => panic!("cancelled"), Outcome::Refused(error) => panic!("refused: {error}") }
}

fn floor(build: &dyn Fn() -> (Document, DocumentMutation)) -> usize {
    static HOOK: std::sync::Once = std::sync::Once::new();
    HOOK.call_once(|| { let previous = std::panic::take_hook(); std::panic::set_hook(Box::new(move |info| { if !info.payload().is::<Stall>() { previous(info) } })); });
    let mut copy = 1usize;
    loop {
        if matches!(catch_unwind(AssertUnwindSafe(|| prepare(build, grant(copy, WIDE), None))), Ok(Outcome::Completed { .. })) { return copy; }
        copy *= 2;
        assert!(copy <= 1 << 24, "no copy grant completes the case");
    }
}

#[test]
fn mutation_apply_prepares_structural_edits_with_ordered_inverse_rows() {
    let policy = fixture();
    let base = document(&policy["base"]);
    for case in policy["cases"].as_array().unwrap() {
        let build = || (base.clone(), mutation(&case["mutation"]));
        let floor = floor(&build);
        let mut turns = Vec::new();
        for copy in [floor, floor + 7, WIDE] {
            let (post, inverse, count) = completed(prepare(&build, grant(copy, WIDE), None));
            assert_eq!(pairs(&post), case["post"], "post of {} at copy {copy}", case["name"]);
            assert_eq!(inverse, rows(&case["inverse"]), "inverse rows of {} at copy {copy}", case["name"]);
            let restored = inverse.into_iter().fold(post, |current, row| completed(prepare(&|| (current.clone(), row.clone()), grant(copy, WIDE), None)).0);
            assert_eq!(restored, base, "ordered inverse rows restore the base for {}", case["name"]);
            turns.push(count);
        }
        for cancel_at in policy["cancelAt"].as_array().unwrap().iter().map(|turn| turn.as_u64().unwrap() as usize) {
            let outcome = prepare(&build, grant(floor, WIDE), Some(cancel_at));
            assert_eq!(matches!(outcome, Outcome::Completed { .. }), cancel_at >= turns[0], "cancel at {cancel_at} of {} turns for {}", turns[0], case["name"]);
        }
    }
}

#[test]
fn mutation_apply_admits_documents_beyond_the_one_item_cap_when_turns_fund_the_snapshot() {
    let text = "x".repeat(1024);
    let large = Document { title: "large".into(), nodes: (0..1400).map(|index| Node { id: format!("n{index}"), text: text.clone() }).collect() };
    let build = || (large.clone(), DocumentMutation::SetText { index: 700, text: "edited".into() });
    let bytes: usize = large.nodes.iter().map(|node| node.id.len() + node.text.len()).sum();
    assert!(bytes > semio_framework_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, "fixture exceeds the one-item cap");
    let factory = mutation_apply_preparation_factory::<Document, DocumentMutation>();
    let footprint = factory.preflight(&build().1, HistoryLane::Document).unwrap();
    assert!(footprint.retained_bytes < 4096 && footprint.is_admissible(), "the footprint counts only the mutation");
    let (post, inverse, _) = completed(prepare(&build, grant(1 << 16, 1 << 26), None));
    assert_eq!(post.nodes[700].text, "edited");
    assert_eq!(post.nodes.len(), 1400);
    assert_eq!(inverse, vec![DocumentMutation::SetText { index: 700, text }]);
    let stalled = catch_unwind(AssertUnwindSafe(|| prepare(&build, grant(1 << 16, 1 << 20), None)));
    assert!(stalled.is_err_and(|payload| payload.is::<Stall>()), "a turn that cannot fund the snapshot-sized apply never completes");
}





#[test]
fn mutation_apply_wire_values_round_trip_through_the_value_contract() {
    let policy = fixture();
    for case in policy["cases"].as_array().unwrap() {
        let original = mutation(&case["mutation"]);
        assert_eq!(DocumentMutation::from_value(original.to_value()).unwrap(), original);
        assert_eq!(<DocumentMutation as OpBinary>::decode_op(&original.encode_op().unwrap()).unwrap(), original);
    }
}

#[derive(semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
struct ScalarConfig{value:i64}
impl semio_framework_os_store_snapshot_clone::config_apply::ConfigApplyMutation<i64> for ScalarConfig{
    fn exchange(self,post:&mut i64)->Result<Self,(ValueError,Self)>{Ok(Self{value:std::mem::replace(post,self.value)})}
}

#[test]
fn config_apply_reaches_prepared_with_independent_scalar_owner(){
    let law:Value=serde_json::from_str(include_str!("./🧫️fixtures/🚦️admission/🔣️.json")).unwrap();
    let base=law["base"].as_i64().unwrap();let expected=base+law["diff"].as_i64().unwrap();
    let root=Arc::new(base);let registry=SnapshotReadRegistryHandle::new();let lease=registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_|panic!("scalar config source lease"));
    let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(1),semio_framework_job::Generation(1),[0;32],0,1,semio_framework_os_spr::HybridLogicalTimestamp::new(1,0),"actor".into(),None,None,None));
    let mutations:Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<ScalarConfig>>=Arc::new(OwnedValueRetirementFactory::<ScalarConfig>::default());
    let snapshots:Arc<dyn semio_framework_value::SnapshotRetirementFactory<i64>>=Arc::new(SharedValueRetirementFactory::<i64>::default());
    let request=ArtifactStoreOneItemPreparationRequest{operation:semio_framework_job::OperationId(1),generation:semio_framework_job::Generation(1),base_revision:[0;32],lane:HistoryLane::Document,authority,base:SnapshotRead::new(Arc::clone(&root),lease),mutation:ScalarConfig{value:expected},mutation_retirement:Arc::clone(&mutations),snapshot_retirement:Arc::clone(&snapshots)};
    let factory=semio_framework_os_store_snapshot_clone::config_apply::config_apply_preparation_factory::<i64,ScalarConfig>();let policy=grant(1<<16,WIDE);
    let(mut owner,receipt)=factory.begin(request,policy).unwrap_or_else(|_|panic!("scalar config birth"));assert!(receipt.fits(policy.retained_grant()));
    let mut prepared=None;let mut turns=0;
    for _ in 0..100000{
        match owner.advance(policy).unwrap(){ArtifactStoreOneItemPreparationStep::Prepared(..)=>{prepared=owner.take_prepared();break;},ArtifactStoreOneItemPreparationStep::Progress(_,receipt)=>assert!(receipt.fits(policy.retained_grant())),ArtifactStoreOneItemPreparationStep::Blocked=>panic!("funded scalar config stalled")}
        turns+=1;
    }
    let prepared=prepared.expect("scalar config prepared");assert_eq!(**prepared.post_snapshot(),expected);assert_eq!(prepared.edit().inverse.get(0).unwrap().value,base);assert_eq!(*root,base);
    let(mut ticket,receipt)=prepared.admit_retirement(mutations,snapshots,policy.retained_grant()).unwrap_or_else(|_|panic!("scalar config candidate retirement"));assert!(receipt.fits(policy.retained_grant()));
    for _ in 0..100000{
        if ticket.terminal_is_empty(){break;}let copy=ticket.next_copy_byte_demand().unwrap();let quote=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:ticket.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:ticket.next_release_byte_demand().unwrap(),maximum_depth:ticket.next_depth_demand().unwrap()};assert!(ticket.close_step(quote).unwrap().progress().fits(quote));
    }
    assert!(ticket.terminal_is_empty());owner.begin_close();
    for _ in 0..100000{
        if owner.terminal_is_empty(){break;}let copy=owner.next_close_copy_byte_demand().unwrap();let quote=ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_close_release_byte_demand().unwrap(),maximum_depth:owner.next_close_depth_demand().unwrap()};assert!(owner.close_step(quote).unwrap().progress().fits(quote.retained_grant()));
    }
    assert!(owner.terminal_is_empty());assert_eq!(observe(||drop(owner)).1.released_bytes,0);drain_fixture(root);
}
/// 🚦️ Diagnostic admission preserves every original row and refuses a valid nonempty hostile diff before writes.



/// ♻️ Decided inverse cancellation uses the supplied issuer without touching the immutable before projection.


/// 🎬️ Prefix, ordinary history and policy merge preserve the same semantic refusal and original row.
#[test]
fn replay_diagnostics_decide_before_inverse_and_application() {
    use semio_framework_os_store_replay::{apply_outcome, fold_operation, EditReplay, ReplayEdits, ReplayMode};
    struct Edits(Vec<protocol::Edit<DocumentMutation>>);
    impl ReplayEdits<DocumentMutation> for Edits {
        fn replay_edit(&self, id: &str) -> Option<&protocol::Edit<DocumentMutation>> { self.0.iter().find(|edit| edit.id == id) }
    }
    let law: Value = serde_json::from_str(include_str!("./🧫️fixtures/🚦️admission/🔣️.json")).unwrap();
    let original = plain_document_json(&fixture()["base"]);
    for case in law["cases"].as_array().unwrap() {
        let mutation = DocumentMutation::Diagnostic { level: case["level"].as_str().unwrap().into(), code: case["code"].as_str().unwrap().into(), message: case["message"].as_str().unwrap().into() };
        let expected = mutation.diff(&original).stamp_op_index(0).messages().to_vec();
        APPLY_CALLS.set(0);
        let (initial, initial_messages) = semio_framework_os_vcs::apply_mutation(&original, &mutation).unwrap();
        assert_eq!(initial_messages, mutation.diff(&original).messages());
        assert_eq!(APPLY_CALLS.get(), case["applyCalls"].as_u64().unwrap() as usize);
        if !case["applied"].as_bool().unwrap() { assert_eq!(initial, original); }
        APPLY_CALLS.set(0);
        let (folded, messages) = fold_operation(&original, &mutation, 0);
        assert_eq!(folded.is_some(), case["applied"].as_bool().unwrap());
        assert_eq!(messages, expected);
        assert_eq!(APPLY_CALLS.get(), case["applyCalls"].as_u64().unwrap() as usize);
        APPLY_CALLS.set(0);
        let (built, outcome) = apply_outcome(&original, mutation.diff(&original).stamp_op_index(0));
        assert_eq!(outcome.messages(), expected);
        assert_eq!(APPLY_CALLS.get(), case["applyCalls"].as_u64().unwrap() as usize);
        if !case["applied"].as_bool().unwrap() { assert_eq!(built, original); assert!(outcome.diff().ops.is_empty()); }
        for (route, mode, from) in [("history", ReplayMode::Report, 0), ("prefix", ReplayMode::Report, 1), ("merge", ReplayMode::Merge(protocol::MergePolicy::Normal), 0)] {
            let count = from + 1;
            let edits = Edits((0..count).map(|index| protocol::Edit { id: format!("e{index}"), actor: None, line: None, forwards: vec![mutation.clone()], inverse: semio_framework_value_persistent::list::PagedList::empty(), mutation_meta: Vec::new(), verb: None, sequence_number: index as i32, started_at: String::new(), finished_at: None }).collect());
            APPLY_CALLS.set(0); INVERSE_CALLS.set(0); DIFF_CALLS.set(0);
            let result = EditReplay::new_after_prefix(mode, 0, Arc::new(original.clone()), edits.0.iter().map(|edit| edit.id.clone()).collect(), from, "test/document", Default::default(), &edits).unwrap()
                .with_retirement_factories(Arc::new(OwnedValueRetirementFactory::<PlainDocument>::default()), Arc::new(OwnedValueRetirementFactory::<DocumentMutation>::default())).run(&edits).unwrap();
            assert_eq!(APPLY_CALLS.get(), count * case["applyCalls"].as_u64().unwrap() as usize, "{route}");
            assert_eq!(INVERSE_CALLS.get(), case["inverseCalls"].as_u64().unwrap() as usize, "{route}");
            assert_eq!(DIFF_CALLS.get(), count, "{route}");
            assert_eq!(result.report.outcomes.len(), 1, "{route}");
            assert_eq!(result.report.outcomes[0].messages, expected, "{route}");
            let post = result.state.as_ref().unwrap();
            if case["applied"].as_bool().unwrap() { assert_eq!(post.nodes[0].text, "diagnostic applied", "{route}"); } else { assert_eq!(post.as_ref(), &original, "{route}"); }
            result.retire_unscheduled().unwrap();
        }
    }
}

/// 🛑️ Cancellation retires a decided delta and diagnostics without executing an inverse or applier.


#[derive(Clone, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct ScalarDiff { delta: i64 }

impl DiffAlgebra<i64> for ScalarDiff {
    fn is_empty(&self) -> bool { self.delta == 0 }
}
impl ColdDiffAlgebra<i64> for ScalarDiff {
    fn inverse(&self, _: &i64) -> Self { INVERSE_CALLS.set(INVERSE_CALLS.get() + 1); Self { delta: -self.delta } }
}

impl MutationDiff<i64> for ScalarDiff {

}
impl ColdMutationDiff<i64> for ScalarDiff {
    fn absorb(&mut self, other: Self) { self.delta += other.delta; }

    fn apply(&self, post: &mut i64, _: ApplyCapability) -> MutationApplyResult<()> { APPLY_CALLS.set(APPLY_CALLS.get() + 1); *post += self.delta; Ok(()) }
}

/// 🧮️ The typed native decision produces the exact portable scalar projection and call counts independently verified by SQLite.
#[test]
fn mutation_apply_admission_scalar_fixture_matches_independent_oracle() {
    let law: Value = serde_json::from_str(include_str!("./🧫️fixtures/🚦️admission/🔣️.json")).unwrap();
    let base = law["base"].as_i64().unwrap();
    for case in law["cases"].as_array().unwrap() {
        let mutation = DocumentMutation::Diagnostic { level: case["level"].as_str().unwrap().into(), code: case["code"].as_str().unwrap().into(), message: case["message"].as_str().unwrap().into() };
        let outcome = mutation.diff(&plain_document_json(&fixture()["base"])).stamp_op_index(0).map(|_| ScalarDiff { delta: law["diff"].as_i64().unwrap() });
        let expected = outcome.messages().to_vec();
        let mut post = base;
        APPLY_CALLS.set(0); INVERSE_CALLS.set(0);
        let messages = match outcome.admit() {
            protocol::MutationAdmission::Admitted(outcome) => {
                let (diff, messages) = outcome.into_parts();
                let _inverse = diff.inverse(&base);
                protocol::apply_diff_cold_into(&diff, &mut post).unwrap();
                messages
            }
            protocol::MutationAdmission::Refused(outcome) => outcome.into_parts().1,
        };
        assert_eq!(messages, expected);
        assert_eq!(post, case["post"].as_i64().unwrap());
        assert_eq!(APPLY_CALLS.get(), case["applyCalls"].as_u64().unwrap() as usize);
        assert_eq!(INVERSE_CALLS.get(), case["inverseCalls"].as_u64().unwrap() as usize);
    }
}

/// 📤️ The complete structural preparation transfers admitted original diagnostics into its sealed candidate.
#[test]
fn paged_preparation_preserves_original_admitted_diagnostics_until_typed_retirement() {
    let law: Value = serde_json::from_str(include_str!("./🧫️fixtures/🚦️admission/🔣️.json")).unwrap();
    let original = plain_document_json(&fixture()["base"]);
    for case in law["cases"].as_array().unwrap() {
        let mutation = DocumentMutation::Diagnostic { level: case["level"].as_str().unwrap().into(), code: case["code"].as_str().unwrap().into(), message: case["message"].as_str().unwrap().into() };
        let actual = prepare(&|| (document_from_plain(original.clone()), mutation.clone()), grant(1 << 16, WIDE), None);
        if case["applied"].as_bool().unwrap() {
            let Outcome::Completed { post, messages, .. } = actual else { panic!("admitted original outcome must seal") };
            assert_eq!(post.nodes[0].text, "diagnostic applied");
            assert_eq!(messages.len(), 1, "sealed candidate must preserve original diagnostics");
            let mut expected = case.as_object().unwrap().clone();
            for key in ["applied", "post", "applyCalls", "inverseCalls"] { expected.remove(key); }
            let wire: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&messages[0])).unwrap();
            assert_eq!(wire, Value::Object(expected));
        } else { assert!(matches!(actual, Outcome::Refused(_)), "refused original outcome cannot seal"); }
    }
}

/// ♻️ A provider receipt refusal keeps every returned inverse in its exact installed issuer custody.
#[test]
fn paged_invalid_edit_receipt_retires_returned_inverse_with_original_issuer() {
    use paged::{PagedOneItemEdit,PagedOneItemEditStep,PagedOneItemPreparationFactory};
    use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
    #[derive(Default)]
    struct InvalidReceiptEdit;
    impl PagedOneItemEdit<Document,DocumentMutation> for InvalidReceiptEdit {
        const PREFIX:&'static str="invalid-receipt-test";
        fn recognizes(_:&DocumentMutation)->bool { true }
        fn preflight(_:&DocumentMutation)->Result<usize,String> { Ok(1) }
        fn advance_demands(&self,_before:semio_framework_value::retained_clone::RetainedCloneRef<'_,Document>,_mutation:semio_framework_value::retained_clone::RetainedCloneRef<'_,DocumentMutation>,_post:&Document,_body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}
        fn advance(&mut self,_before:semio_framework_value::retained_clone::RetainedCloneRef<'_,Document>,_mutation:semio_framework_value::retained_clone::RetainedCloneRef<'_,DocumentMutation>,post:&mut Document,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<PagedOneItemEditStep<DocumentMutation>,ValueError> {
            let inverse=vec![DocumentMutation::SetText {index:0,text:node_text(post,0).clone()}];
            Ok(PagedOneItemEditStep::CompleteRows(RetainedCloneProgress{copied_items:1,copied_bytes:grant.maximum_copy_bytes+1,..Default::default()},inverse))
        }
        fn begin_close(&mut self){}
        fn close_demands(&self,_:&dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<DocumentMutation>,_:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{Ok(Default::default())}
        fn close_step(&mut self,_:&dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<DocumentMutation>,_:RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>{Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))}
        fn terminal_is_empty(&self)->bool{true}
    }
    let law:Value=serde_json::from_str(include_str!("./🧫️fixtures/♻️inverse-issuer/🔣️.json")).unwrap();
    let original=plain_document_json(&fixture()["base"]);
    let factory:Arc<dyn ArtifactStoreOneItemPreparationFactory<Document,DocumentMutation>>=Arc::new(PagedOneItemPreparationFactory::<Document,DocumentMutation,InvalidReceiptEdit>::default());
    for row in law["cases"].as_array().unwrap() {
        ORIGINAL_ISSUER_RETIRES.set(0);
        let outcome=prepare_with(&factory,&|| (document_from_plain(original.clone()),DocumentMutation::SetText{index:0,text:"new".into()}),grant(1<<16,WIDE),None);
        assert!(matches!(outcome,Outcome::Refused(_)));
        assert_eq!(ORIGINAL_ISSUER_RETIRES.get(),row["expectedIssuerCalls"].as_u64().unwrap()as usize,"returned inverse must use the actual original issuer on receipt refusal");
    }
}

#[path="../../🧪️tests/🔗️source-custody/🦀️.rs"]
mod source_custody;

#[path="🧳️semantics/🦀️.rs"] mod retained_semantics;

#[derive(Clone,Copy,semio_framework_value::FactoryPayloadRetirement)]
struct ZstEditOriginal;
#[derive(Clone,Copy,semio_framework_value::FactoryPayloadRetirement)]
struct ZstEditOther;
union EditIssuerSlot{original:ZstEditOriginal,other:ZstEditOther}
thread_local!{static OTHER_EDIT_ISSUER_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
impl semio_framework_value::ArtifactOwnedValueRetirementFactory<DocumentMutation> for ZstEditOriginal{
 fn retirement_birth_bytes(&self,_:&DocumentMutation)->usize{semio_framework_value::owned_retirement_birth_bytes::<DocumentMutation>()}
 fn retire_owned(&self,row:DocumentMutation,g:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress),(ValueError,DocumentMutation)>{semio_framework_value::admit_owned_retirement(row,g)}
}
impl semio_framework_value::ArtifactOwnedValueRetirementFactory<DocumentMutation> for ZstEditOther{
 fn retirement_birth_bytes(&self,_:&DocumentMutation)->usize{semio_framework_value::owned_retirement_birth_bytes::<DocumentMutation>()}
 fn retire_owned(&self,row:DocumentMutation,_:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress),(ValueError,DocumentMutation)>{OTHER_EDIT_ISSUER_CALLS.set(OTHER_EDIT_ISSUER_CALLS.get()+1);Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"foreign zero-sized editor issuer must not receive original inverse rows"),row))}
}
/// 🔑️ Generic editor refusal preserves its original inverse vector before a differently typed same-address issuer can act.

mod quick {
use super::*;
#[test]
fn mutation_apply_prepares_structural_edits_with_ordered_inverse_rows() {
    let policy = fixture();
    let base = document(&policy["base"]);
    let factory = semio_framework_os_store_snapshot_clone::config_apply::config_apply_preparation_factory::<Document, DocumentMutation>();
    let build = || (base.clone(), DocumentMutation::SetText { index: 1, text: "changed".into() });
    let Outcome::Completed { post, inverse, turns, retained, peak } = prepare_with(&factory, &build, grant(1 << 16, WIDE), None) else { panic!("config apply must complete") };
    let tiny = Document { title: String::new(), nodes: vec![Node { id: "a".into(), text: "b".into() }] };
    let Outcome::Completed { retained: tiny_retained, peak: tiny_peak, .. } = prepare_with(&factory, &|| (tiny.clone(), DocumentMutation::SetText { index: 0, text: "c".into() }), grant(1 << 16, WIDE), None) else { panic!("tiny must complete") };
    assert_eq!(post.nodes[1].text, "changed");
    assert_eq!(inverse, vec![DocumentMutation::SetText { index: 1, text: "beta é".into() }]);
}
#[test]
fn mutation_apply_refuses_semantic_errors_and_closes() {
    let policy = fixture();
    let base = plain_document_json(&policy["base"]);
    for bad in [DocumentMutation::Remove { index: 9 }, DocumentMutation::SetText { index: 9, text: "x".into() }, DocumentMutation::RemoveRange { start: 3, count: 2 }, DocumentMutation::Insert { index: 9, id: "x".into(), text: "x".into() }] {
        let outcome = prepare(&|| (document_from_plain(base.clone()), bad.clone()), grant(1 << 16, WIDE), None);
        assert!(matches!(outcome, Outcome::Refused(_)), "{bad:?} must be refused");
    }
}
#[test]
fn mutation_apply_diagnostics_gate_atomic_publication() {
    use paged::PagedOneItemEdit;
    use paged::PagedOneItemEditStep;
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    let law: Value = serde_json::from_str(include_str!("./🧫️fixtures/🚦️admission/🔣️.json")).unwrap();
    let original = plain_document_json(&fixture()["base"]);
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: WIDE, maximum_capacity_bytes: WIDE, maximum_release_bytes: WIDE, maximum_depth: 4096 };
    for case in law["cases"].as_array().unwrap() {
        let mutation = DocumentMutation::Diagnostic { level: case["level"].as_str().unwrap().into(), code: case["code"].as_str().unwrap().into(), message: case["message"].as_str().unwrap().into() };
        let mut owner = paged::MutationApplyEdit::<Document, DocumentMutation>::default();
        let mut before_source=OriginalSource::new(document_from_plain(original.clone()));
        let mut mutation_source=OriginalSource::new(mutation.clone());
        let issuer=OwnedValueRetirementFactory::<DocumentMutation>::default();
        let mut post = document_from_plain(original.clone());
        APPLY_CALLS.set(0); INVERSE_CALLS.set(0); DIFF_CALLS.set(0);
        let (applied, messages) = loop {
            match owner.advance(before_source.borrow(),mutation_source.borrow(),&mut post,grant).unwrap() {
                PagedOneItemEditStep::CompleteOutcome(_, rows, messages) => { assert_eq!(rows, vec![DocumentMutation::SetText { index: 0, text: original.nodes[0].text.clone() }]); break (true, messages) },
                PagedOneItemEditStep::RefusedOutcome(_, messages) => break (false, messages),
                PagedOneItemEditStep::Progress(_) => {},
                _ => panic!("diagnostic outcome must retain original messages"),
            }
        };
        assert_eq!(APPLY_CALLS.get(), case["applyCalls"].as_u64().unwrap() as usize);
        assert_eq!(INVERSE_CALLS.get(), case["inverseCalls"].as_u64().unwrap() as usize);
        assert_eq!(DIFF_CALLS.get(), 1);
        assert_eq!(applied, case["applied"].as_bool().unwrap());
        assert_eq!(messages.len(), 1);
        let actual: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&messages[0])).unwrap();
        let mut expected = case.as_object().unwrap().clone();
        for key in ["applied", "post", "applyCalls", "inverseCalls"] { expected.remove(key); }
        assert_eq!(actual, Value::Object(expected));
        if applied { assert_eq!(node_text(&post,0), "diagnostic applied"); } else { assert_eq!(plain_document(&post), original); }
        let mut ledger = protocol::MutationMessageLedgerRetirement::new(String::new(), messages);
        while !matches!(ledger.close_step(grant).unwrap(), RetainedCloneStep::Complete(_)) {}
        owner.begin_close();
        while !matches!(owner.close_step(&issuer,grant).unwrap(), RetainedCloneStep::Complete(_)) {}
        assert!(owner.terminal_is_empty());
        drain_fixture(post);before_source.close();mutation_source.close();
    }
}
#[test]
fn mutation_apply_inverse_cancellation_uses_original_factory() {
    use paged::{PagedOneItemEdit, PagedOneItemEditStep};
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    let law: Value = serde_json::from_str(include_str!("./🧫️fixtures/♻️inverse-cancellation/🔣️.json")).unwrap();
    for case in law["cases"].as_array().unwrap() {
        let original = plain_document_json(&law["base"]);
        let mutation = mutation(&law["mutation"]);
        let mut before_source = OriginalSource::new(document_from_plain(original.clone()));
        let mut mutation_source = OriginalSource::new(mutation);
        let issuer = OriginalMutationIssuer;
        let mut post = document_from_plain(original.clone());
        let mut owner = paged::MutationApplyEdit::<Document, DocumentMutation>::default();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: WIDE, maximum_capacity_bytes: WIDE, maximum_release_bytes: WIDE, maximum_depth: 4096 };
        ORIGINAL_ISSUER_RETIRES.set(0); APPLY_CALLS.set(0); INVERSE_CALLS.set(0);
                let boundary=match case["boundary"].as_str().unwrap(){"decision"=>paged::test_support::Stage::Source,"inverse"=>paged::test_support::Stage::Commit,_=>unreachable!()};
        for turn in 0..10000 {
            if owner.stage==boundary{break;}
            assert!(turn<9999,"retained semantic boundary was not reached");
            assert!(matches!(owner.advance(before_source.borrow(), mutation_source.borrow(), &mut post, grant).unwrap(), PagedOneItemEditStep::Progress(_)));
        }
        assert_eq!(owner.inverse.as_ref().map_or(0, Vec::len), case["inverseRows"].as_u64().unwrap() as usize);
        owner.begin_close();
        while !owner.terminal_is_empty() {
            let demand = owner.close_demands(&issuer, WIDE).unwrap();
            let quote = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let step = owner.close_step(&issuer, quote).unwrap();
            assert!(step.progress().fits(quote));
            if matches!(step, RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); }
        }
        assert_eq!(plain_document(&post), original);
        assert_eq!(node_text(&post,0), case["postText"].as_str().unwrap());
        assert_eq!(plain_document(before_source.borrow().get()), original);
        assert_eq!(APPLY_CALLS.get(), 0);
        assert_eq!(ORIGINAL_ISSUER_RETIRES.get(), case["issuerCalls"].as_u64().unwrap() as usize);
        drain_fixture(post);before_source.close();mutation_source.close();
    }
}
#[test]
fn mutation_apply_cancellation_after_decision_preserves_base() {
    use paged::{PagedOneItemEdit, PagedOneItemEditStep};
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    let mut post = document(&fixture()["base"]);
    let original = plain_document(&post);
    let mutation = DocumentMutation::Diagnostic { level: "warning".into(), code: "mutation.partial".into(), message: "retained warning".into() };
    let mut before_source=OriginalSource::new(document_from_plain(original.clone()));
    let mut mutation_source=OriginalSource::new(mutation.clone());
    let issuer=OwnedValueRetirementFactory::<DocumentMutation>::default();
    let mut owner = paged::MutationApplyEdit::<Document, DocumentMutation>::default();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: WIDE, maximum_capacity_bytes: WIDE, maximum_release_bytes: WIDE, maximum_depth: 4096 };
    APPLY_CALLS.set(0); INVERSE_CALLS.set(0); DIFF_CALLS.set(0);
    for turn in 0..10000 {
        if owner.stage == paged::test_support::Stage::Source { break; }
        assert!(turn < 9999, "original typed decision boundary was not reached");
        assert!(matches!(owner.advance(before_source.borrow(),mutation_source.borrow(),&mut post,grant).unwrap(), PagedOneItemEditStep::Progress(_)));
    }
    owner.begin_close();
    while !owner.terminal_is_empty() {
        let demand = owner.close_demands(&issuer,WIDE).unwrap();
        let quote = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let step = owner.close_step(&issuer,quote).unwrap();
        assert!(step.progress().fits(quote));
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(owner.terminal_is_empty()); }
    }
    assert_eq!(plain_document(&post), original);
    assert_eq!((DIFF_CALLS.get(), INVERSE_CALLS.get(), APPLY_CALLS.get()), (1, 0, 0));
    drain_fixture(post);before_source.close();mutation_source.close();
}
#[test]
fn mutation_apply_rejects_same_data_different_method_table_inverse_issuer(){
 use paged::{PagedOneItemEdit,PagedOneItemEditStep};use semio_framework_value::retained_clone::RetainedCloneGrant;
 let manifest=std::path::Path::new(env!("CARGO_MANIFEST_DIR"));let root=manifest.ancestors().find(|p|p.join(".🧬semio").is_dir()).expect("native fixture repository root");let law:Value=serde_json::from_str(&std::fs::read_to_string(root.join("🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📋️vector/🧫️fixtures/🔣️.json")).unwrap()).unwrap();let law=&law["issuerAlias"];let index=law["pendingId"].as_u64().unwrap()as usize;
 let slot=EditIssuerSlot{original:ZstEditOriginal};let original_issuer=unsafe{&slot.original};let foreign=unsafe{&slot.other};assert_eq!(std::ptr::from_ref(original_issuer).cast::<()>(),std::ptr::from_ref(foreign).cast::<()>());OTHER_EDIT_ISSUER_CALLS.set(0);
 let original=plain_document_json(&fixture()["base"]);let mut before=OriginalSource::new(document_from_plain(original.clone()));let mut input=OriginalSource::new(DocumentMutation::SetText{index,text:"changed".into()});let mut post=document_from_plain(original.clone());let mut owner=paged::MutationApplyEdit::<Document,DocumentMutation>::default();let wide=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:WIDE,maximum_capacity_bytes:WIDE,maximum_release_bytes:WIDE,maximum_depth:4096};
 for turn in 0..10000{if owner.stage==paged::test_support::Stage::Commit{break;}assert!(matches!(owner.advance(before.borrow(),input.borrow(),&mut post,wide).unwrap(),PagedOneItemEditStep::Progress(_)));assert!(turn<9999);}
 let inverse=owner.inverse.as_ref().unwrap();let backing=inverse.as_ptr();let text=match &inverse[0]{DocumentMutation::SetText{index:actual,text}=>{assert_eq!(*actual,index);text.as_ptr()},_=>panic!("original inverse shape")};owner.begin_close();let d=owner.close_demands(original_issuer,WIDE).unwrap();let q=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};owner.close_step(original_issuer,q).unwrap();
 let(result,heap)=observe(||owner.close_step(foreign,q));assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(OTHER_EDIT_ISSUER_CALLS.get(),law["providerCalls"].as_u64().unwrap()as usize);let inverse=owner.inverse.as_ref().unwrap();assert_eq!(inverse.as_ptr(),backing);match &inverse[0]{DocumentMutation::SetText{text:actual,..}=>assert_eq!(actual.as_ptr(),text),_=>panic!("original inverse row changed")};assert_eq!(plain_document(&post),original);
 for turn in 0..100000{if owner.terminal_is_empty(){break;}let d=owner.close_demands(original_issuer,WIDE).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let s=owner.close_step(original_issuer,g).unwrap();assert!(s.progress().fits(g));assert!(turn<99999);}
 assert!(owner.terminal_is_empty());drain_fixture(post);before.close();input.close();
}

}
mod long {
use super::*;
#[test]
fn mutation_apply_admits_large_immutable_documents_with_bounded_changed_paths() {
    let text = "x".repeat(1024);
    let large = PlainDocument { title: "large".into(), nodes: (0..1400).map(|index| PlainNode { id: format!("n{index}"), text: text.clone() }).collect() };
    let build = || (document_from_plain(large.clone()), DocumentMutation::SetText { index: 700, text: "edited".into() });
    let bytes: usize = large.nodes.iter().map(|node| node.id.len() + node.text.len()).sum();
    assert!(bytes > semio_framework_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, "fixture exceeds the one-item cap");
    let factory = mutation_apply_preparation_factory::<Document, DocumentMutation>();
    let probe=DocumentMutation::SetText{index:700,text:"edited".into()};
    let footprint = factory.preflight(&probe, HistoryLane::Document).unwrap();
    assert!(footprint.retained_bytes < 4096 && footprint.is_admissible(), "the footprint counts only the mutation");
    let (post, inverse, _) = completed(prepare(&build, grant(1 << 16, 1 << 26), None));
    assert_eq!(post.nodes[700].text, "edited");
    assert_eq!(post.nodes.len(), 1400);
    assert_eq!(inverse, vec![DocumentMutation::SetText { index: 700, text }]);
    let (bounded,bounded_inverse,_)=completed(prepare(&build,grant(1<<16,1<<20),None));
    assert_eq!(bounded.nodes[700].text,"edited");assert_eq!(bounded.nodes.len(),1400);assert_eq!(bounded_inverse,inverse);
}
}
