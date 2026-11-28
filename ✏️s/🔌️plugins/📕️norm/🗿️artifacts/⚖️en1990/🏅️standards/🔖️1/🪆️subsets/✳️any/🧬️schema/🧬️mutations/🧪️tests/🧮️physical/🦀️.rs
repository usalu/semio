//! 🧮️ Real EN 1990 public replay preparation pays nested owners under its original caller.
use semio_framework_job::{original_receiving_scope_headroom, Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller};
use semio_framework_os_store_one_item_batch::paged_one_item::{ColdHeapQuote, RetainedColdMutationApplyFactory};
use semio_framework_value::{
    factory::ArcRootRetirement,
    retained_clone::{retained_clone_grant_admits, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep},
    retirement::{controlled::ControlledRetirement, RetireOwned},
    ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, FromValue, NativeDecodeControl, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkPoll, RetirementDemand, ValueError,
};
use semio_framework_value_snapshot_read::{SnapshotRead, SnapshotReadRegistryHandle};
use semio_os_store_replay_operation::{ArtifactMutationSource, ArtifactReplayPreparationContext, ArtifactReplayPreparationFactory, ArtifactReplayPreparationMode, ArtifactReplayPreparationRequest, ArtifactReplayPreparationStep};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use semio_s_artifact_norm_en1990_mutations::{en1990_cold_replay_bounds, En1990Mutation};
use std::sync::Arc;

const WORK: u64 = 1 << 16;
const PAGE: usize = 4096;

fn retained(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
}

fn stock() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 2_000_000, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 4096 }
}

fn settle(poll: OriginalRetirementPoll) -> RetainedCloneStep {
    match poll {
        RetainedWorkPoll::Pending { .. } => RetainedCloneStep::Progress(Default::default()),
        RetainedWorkPoll::Complete(turn) => turn.result.unwrap(),
    }
}

fn effect<T>(control: &mut NativeDecodeControl<'_>, grant: RetainedCloneGrant, run: impl FnOnce(&mut NativeDecodeControl<'_>, &mut RetainedAllocationEffect<'_>) -> T) -> T {
    let available = control.available_work_hint().unwrap_or(WORK);
    let work = available.saturating_sub(semio_framework_value::ORIGINAL_INVOCATION_WORK);
    control.with_effect(grant, work, run).unwrap().expect("funded replay effect opens")
}

#[derive(Debug)]
struct Unpaid {
    turn: usize,
    actual: (usize, usize),
    reported: RetainedCloneProgress,
}

struct Probe {
    caller: OriginalJobFixtureCaller,
    decoder: OriginalJobDecodeOwner,
    turn: usize,
    bound: Option<(usize, usize)>,
    bounded_turns: usize,
    unpaid: Option<Unpaid>,
}

impl Probe {
    fn new() -> Self {
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(199007), Generation(1), stock(), || Some(1), u64::MAX, 1 << 28);
        caller.retain_dispatch().unwrap();
        caller.admit_return_table();
        let decoder = caller.admitted_decoder();
        let mut probe = Self { caller, decoder, turn: 0, bound: None, bounded_turns: 0, unpaid: None };
        probe.run(RetainedCloneGrant::default(), |_| ((), RetainedCloneProgress::default()));
        let grant = retained(semio_framework_value::original_failure_storage_birth_demand());
        probe.run(grant, |control| {
            let poll = control.prepare_failure_storage(grant);
            ((), settle(poll).progress())
        });
        probe
    }

    fn run<T>(&mut self, grant: RetainedCloneGrant, run: impl FnOnce(&mut NativeDecodeControl<'_>) -> (T, RetainedCloneProgress)) -> T {
        let quantum = RetainedCloneGrant { maximum_items: grant.maximum_items.max(1), maximum_depth: grant.maximum_depth.max(1), ..grant };
        let (items, copy, work) = original_receiving_scope_headroom();
        let outer = RetainedCloneGrant { maximum_items: quantum.maximum_items + items, maximum_copy_bytes: quantum.maximum_copy_bytes + copy, ..quantum };
        let mut run = Some(run);
        let mut observed = None;
        for _ in 0..4096 {
            self.caller
                .refreshed_turn_with_decoder(outer, &mut self.decoder, |cx| {
                    cx.with_original_work_loan(WORK + work, u64::MAX, |cx| {
                        let mut outcome = None;
                        cx.with_native_receiving_quoted(&mut outcome, (quantum, WORK), |control, _| {
                            observed = Some(semio_framework_trace::observe_heap_allocations_on_this_thread(|| run.take().expect("original receiving callback runs once")(control)));
                            Ok(())
                        })?;
                        if let Some(outcome) = outcome {
                            outcome.into_result()?;
                        }
                        Ok(())
                    })
                })
                .unwrap();
            if observed.is_some() {
                break;
            }
        }
        self.turn += 1;
        let ((value, progress), heap) = observed.unwrap_or_else(|| panic!("original replay receiving scope did not open on turn {} for {grant:?}", self.turn));
        assert!(progress.fits(grant), "original replay exceeded its physical grant");
        let actual = (heap.requested_bytes, heap.released_bytes);
        self.bounded_turns += usize::from(self.bound.is_some());
        let funded = self.bound.is_some_and(|(capacity, transient)| actual.0 <= capacity.saturating_add(transient) && actual.0.saturating_sub(actual.1) <= capacity);
        if !funded && actual != (progress.retained_capacity_bytes, progress.released_bytes) && self.unpaid.is_none() {
            self.unpaid = Some(Unpaid { turn: self.turn, actual, reported: progress });
        }
        value
    }

    fn retire<T: RetireOwned>(&mut self, value: T) {
        let mut owner = ControlledRetirement::new(value).map_err(|(error, _)| error).unwrap();
        for _ in 0..100_000 {
            if owner.terminal_is_empty() {
                return;
            }
            let demand = owner.next_demand(PAGE).unwrap();
            let grant = RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.max(PAGE), ..retained(demand) };
            self.run(grant, |control| {
                let step = effect(control, grant, |control, effect| {
                    control
                        .with_retirement(grant, |receiving| {
                            let poll = if owner.requires_original_birth() { receiving.with_original_close(|source| Ok::<_, ValueError>(owner.step_original_birth(source, grant)))? } else { owner.step(PAGE, effect) };
                            Ok::<_, ValueError>(poll)
                        })
                        .unwrap()
                });
                ((), settle(step).progress())
            });
        }
        panic!("original EN 1990 fixture custody did not retire");
    }

    fn retire_snapshot(&mut self, snapshot: Arc<En1990Snapshot>) {
        assert_eq!(Arc::weak_count(&snapshot), 0);
        let released_bytes = semio_framework_value::shared_retirement_allocation_bytes::<En1990Snapshot>();
        let grant = retained(RetirementDemand { release_bytes: released_bytes, ..Default::default() });
        let snapshot = self.run(grant, |control| {
            assert!(control.prepare_work(1).unwrap());
            let snapshot = Arc::into_inner(snapshot).expect("replay snapshot is uniquely owned");
            control.commit_work(1).unwrap();
            (snapshot, RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() })
        });
        self.retire(snapshot);
    }

    fn finish(mut self) -> Option<Unpaid> {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: WORK as usize, maximum_capacity_bytes: 0, maximum_release_bytes: semio_framework_value::original_failure_storage_birth_demand().capacity_bytes, maximum_depth: 1 };
        self.run(grant, |control| {
            let step = effect(control, grant, |control, effect| {
                control
                    .with_retirement(grant, |receiving| {
                        let pool = semio_framework_value::OriginalRetirementControl::failure_storage_pool(receiving);
                        let turn = pool.release_step(effect.grant(), effect.maximum_work_units());
                        assert!(effect.performed(u64::try_from(turn.work).unwrap())?);
                        turn.result
                    })
                    .unwrap()
            });
            ((), step.progress())
        });
        if let Ok(credit) = self.caller.turn_with_decoder(stock(), &mut self.decoder, |cx| cx.adopt_original_credit(())).unwrap() {
            let mut credit = std::mem::ManuallyDrop::new(credit);
            let settled = (0..16).any(|_| self.caller.turn(stock(), |cx| cx.settle_original_credit(&mut credit)).unwrap());
            assert!(settled, "original EN 1990 scope credit did not settle");
            std::mem::ManuallyDrop::into_inner(credit);
        }
        for _ in 0..100_000 {
            if self.decoder.terminal_is_empty() {
                self.caller.retire(stock());
                return self.unpaid;
            }
            let demand = self.decoder.retirement_demands(PAGE).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 2, maximum_copy_bytes: demand.copy_bytes.max(PAGE), ..retained(demand) };
            self.caller
                .turn(grant, |cx| {
                    self.decoder.close_on_source(cx)?;
                    Ok(())
                })
                .unwrap();
        }
        panic!("original EN 1990 decoder did not retire");
    }
}
fn texts() -> (&'static str, &'static str, &'static str) {
    (
        semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌉️insert-bridge-sls/✅apply/🔣️.before.json"),
        semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌉️insert-bridge-sls/✅apply/🔣️.mutation.json"),
        semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌉️insert-bridge-sls/✅apply/🔣️.after.json"),
    )
}
fn oracle(before: &str, mutation: &str, after: &str, policy: &serde_json::Value) {
    use std::io::Write;
    let input = serde_json::json!({"before":serde_json::from_str::<serde_json::Value>(before).unwrap(),"mutation":serde_json::from_str::<serde_json::Value>(mutation).unwrap(),"policy":policy,"schema":serde_json::from_str::<serde_json::Value>(include_str!("🧬️schema/🔣️.json")).unwrap()});
    let mut child=std::process::Command::new("repo").args(["tools","exec","bun","--","-e",r#"import{Database}from'bun:sqlite';import Ajv from'ajv/dist/2020.js';const v=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).compile(v.schema)(v.policy))throw Error('physical policy schema');const db=new Database(':memory:');db.exec('create table rows(pos integer primary key,payload text)');v.before.bridgeSls.forEach((row,i)=>db.query('insert into rows values(?,?)').run(i*2,JSON.stringify(row)));const at=v.mutation.index??v.before.bridgeSls.length;db.query('insert into rows values(?,?)').run(at*2-1,JSON.stringify(v.mutation.item));v.before.bridgeSls=db.query('select payload from rows order by pos').all().map(r=>JSON.parse(r.payload));console.log(JSON.stringify(v.before));"#]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(serde_json::from_slice::<En1990Snapshot>(&output.stdout).unwrap(), serde_json::from_str::<En1990Snapshot>(after).unwrap());
}

#[test]
fn en1990_public_factory_pays_nested_projection_and_semantics() {
    let policy: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let (before, mutation, after) = texts();
    oracle(before, mutation, after, &policy);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let bytes = std::hint::black_box(vec![0_u8; 73]);
        std::hint::black_box(&bytes);
        drop(bytes);
    });
    assert_eq!((heap.requested_bytes, heap.released_bytes), (73, 73));
    let cases = policy["heapQuote"]["cases"].as_array().unwrap();
    assert!(cases.iter().all(|case| {
        let quote = ColdHeapQuote::reference(case["snapshotBytes"].as_u64().unwrap() as usize, case["mutationBytes"].as_u64().unwrap() as usize);
        (quote.copy_bytes, quote.capacity_bytes, quote.transient_bytes) == (case["copyBytes"].as_u64().unwrap() as usize, case["capacityBytes"].as_u64().unwrap() as usize, case["transientBytes"].as_u64().unwrap() as usize)
    }));
    assert_eq!((ColdHeapQuote::REFERENCE_FACTOR, ColdHeapQuote::REFERENCE_FIXED_BYTES), (policy["heapQuote"]["factor"].as_u64().unwrap() as usize, policy["heapQuote"]["fixedBytes"].as_u64().unwrap() as usize));
    let bounds = en1990_cold_replay_bounds();
    let maximum = policy["maximumTurns"].as_u64().unwrap() as usize;
    let body = policy["bodyBytes"].as_u64().unwrap() as usize;
    let funded = RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: policy["copyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes: policy["capacityBytes"].as_u64().unwrap() as usize,
        maximum_release_bytes: policy["releaseBytes"].as_u64().unwrap() as usize,
        maximum_depth: policy["depth"].as_u64().unwrap() as usize,
    };
    for index in policy["operationIndices"].as_array().unwrap() {
        let index = index.as_u64().unwrap() as u32;
        for cancel_after in std::iter::once(None).chain(policy["cancelAfter"].as_array().unwrap().iter().map(|n| Some(n.as_u64().unwrap() as usize))) {
            let mut probe = Probe::new();
            let root = Arc::new(serde_json::from_str::<En1990Snapshot>(before).unwrap());
            let expected = serde_json::from_str::<En1990Snapshot>(after).unwrap();
            let parsed = semio_framework_pack_json::parse(mutation, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
            let mut original = Some(En1990Mutation::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap());
            let quote = (bounds.heap_quote)(&root, original.as_ref().unwrap());
            let snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<En1990Snapshot>> = Arc::new(semio_framework_value::OwnedValueRetirementFactory::<En1990Snapshot>::default());
            let mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<En1990Mutation>> = Arc::new(semio_framework_value::OwnedValueRetirementFactory::<En1990Mutation>::default());
            let grant = retained(ArtifactMutationSource::<En1990Mutation>::constructor_demand());
            let mut source = probe.run(grant, |_| {
                let (source, progress) = ArtifactMutationSource::admit(&mut original, &mutations, grant).unwrap().unwrap();
                (source, progress)
            });
            let mut input = None;
            for _ in 0..maximum {
                let grant = retained(source.demands(body).unwrap());
                if source.ready() {
                    probe.run(grant, |_| ((), source.take(&mut input, grant).unwrap().unwrap()));
                    break;
                }
                probe.run(grant, |control| ((), source.advance_chained(control, grant).unwrap().progress()));
            }
            let registry = SnapshotReadRegistryHandle::new();
            let lease = registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_| panic!("original EN 1990 read lease"));
            let context = ArtifactReplayPreparationContext { mode: ArtifactReplayPreparationMode::Prefix, operation_index: index, generation: 1, base_revision: [7; 32] };
            let request = ArtifactReplayPreparationRequest { base: SnapshotRead::new(Arc::clone(&root), lease), input: input.take().unwrap(), context, snapshots: Arc::clone(&snapshots), mutations: Arc::clone(&mutations) };
            let factory = RetainedColdMutationApplyFactory::<En1990Snapshot, En1990Mutation>::bounded(bounds);
            let birth = retained(ArtifactReplayPreparationFactory::preparation_birth_demand(&factory));
            let mut owner = probe.run(birth, |control| {
                assert!(control.prepare_work(1).unwrap());
                control.admit_turn_capacity(birth.maximum_capacity_bytes).unwrap();
                control.charge(birth.maximum_capacity_bytes).unwrap();
                let (owner, progress) = ArtifactReplayPreparationFactory::begin(&factory, request, birth).map_err(|(error, _)| error).unwrap();
                control.commit_work(1).unwrap();
                (owner, progress)
            });
            let mut prepared = None;
            let denied = semio_os_kernel::os_store::one_item::ArtifactStoreOneItemGrant {
                maximum_items: 0,
                maximum_copy_bytes: funded.maximum_copy_bytes,
                maximum_capacity_bytes: funded.maximum_capacity_bytes,
                maximum_release_bytes: funded.maximum_release_bytes,
                maximum_depth: funded.maximum_depth,
            };
            let grant = semio_os_kernel::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, ..denied };
            let mut last_advance = None;
            let mut stagnant_turns = 0;
            for turn in 0..maximum.min(cancel_after.unwrap_or(maximum)) {
                let before = owner.advance_demands(body, WORK).unwrap();
                assert!(retained_clone_grant_admits(funded, before), "operation {index}, cancellation {cancel_after:?}, turn {turn}: replay demand {before:?} exceeds {funded:?}; last advance {last_advance:?}; first heap mismatch {:?}", probe.unpaid);
                let blocked = probe.run(funded, |control| {
                    let step = owner.advance(body, denied, control).unwrap();
                    (step, step.progress())
                });
                assert_eq!(blocked, ArtifactReplayPreparationStep::Blocked, "denial at operation {index}, cancellation {cancel_after:?}, turn {turn}");
                assert_eq!(owner.last_turn_work(), 0, "work on denial at operation {index}, turn {turn}");
                assert_eq!(owner.advance_demands(body, WORK).unwrap(), before, "quote after denial at operation {index}, turn {turn}");
                probe.bound = (before.capacity_bytes == quote.capacity_bytes && before.release_bytes == quote.transient_bytes).then_some((quote.capacity_bytes, quote.transient_bytes));
                let (step, receiving) = probe.run(funded, |control| {
                    let limits = (control.available_work_hint(), control.owned_bytes(), control.maximum_bytes(), control.has_retirement_failure_storage());
                    let step = owner.advance(body, grant, control).unwrap();
                    ((step, limits), step.progress())
                });
                probe.bound = None;
                let work = owner.last_turn_work();
                let after = owner.advance_demands(body, WORK).unwrap();
                last_advance = Some((turn, before, step, work, after, receiving));
                if step == ArtifactReplayPreparationStep::Blocked && work == 0 && after == before {
                    stagnant_turns += 1;
                    assert!(stagnant_turns < 32, "operation {index}, cancellation {cancel_after:?}, turn {turn}: funded replay stayed blocked at {before:?}; receiving (work, owned, maximum, failure storage) {receiving:?}; first heap mismatch {:?}", probe.unpaid);
                } else {
                    stagnant_turns = 0;
                }
                if let ArtifactReplayPreparationStep::Prepared(_) = step {
                    prepared = owner.take_prepared();
                    break;
                }
            }
            if let Some(prepared) = prepared {
                assert!(prepared.apply_refusal.is_none() && prepared.input_refusal.is_none());
                assert_eq!(prepared.next.as_deref(), Some(&expected));
                probe.retire_snapshot(prepared.next.unwrap());
                probe.retire(prepared.inverse.unwrap());
                probe.retire(prepared.messages);
            } else {
                assert!(cancel_after.is_some(), "actual EN 1990 public factory did not complete operation {index}; last advance {last_advance:?}; first heap mismatch {:?}", probe.unpaid);
            }
            owner.cancel();
            owner.begin_close();
            for _ in 0..maximum {
                if owner.terminal_is_empty() {
                    break;
                }
                let grant = retained(owner.close_demands(body).unwrap());
                probe.run(grant, |control| {
                    let poll = effect(control, grant, |control, effect| control.with_retirement(grant, |receiving| Ok::<_, ValueError>(owner.close_step(effect, receiving))).unwrap());
                    ((), settle(poll).progress())
                });
            }
            assert!(owner.terminal_is_empty());
            let released = std::mem::size_of_val(owner.as_ref());
            probe.run(retained(RetirementDemand { release_bytes: released, ..Default::default() }), |control| {
                assert!(control.prepare_work(1).unwrap());
                drop(owner);
                control.commit_work(1).unwrap();
                ((), RetainedCloneProgress { copied_items: 1, released_bytes: released, ..Default::default() })
            });
            source.begin_close();
            for _ in 0..maximum {
                if source.terminal_is_empty() {
                    break;
                }
                let grant = retained(source.close_demand(body).unwrap());
                probe.run(grant, |control| ((), effect(control, grant, |_, effect| source.close_step(effect).unwrap()).progress()));
            }
            assert!(source.terminal_is_empty());
            let mut active: Option<Box<dyn ErasedSnapshotRetirement>> = None;
            for _ in 0..maximum {
                if !registry.has_returned() && active.is_none() {
                    break;
                }
                let demand = if let Some(owner) = active.as_ref() {
                    semio_framework_value_snapshot_read::artifact_retirement_box_demands(owner, body).unwrap()
                } else {
                    registry.returned_admission_demands::<En1990Snapshot>(|snapshot| ArcRootRetirement::original_admission_demands(&Some(Arc::clone(snapshot)))).unwrap()
                };
                let grant = retained(demand);
                probe.run(grant, |control| {
                    let poll = effect(control, grant, |_, effect| semio_framework_value_snapshot_read::advance_returned_snapshot_read(&registry, &mut active, &snapshots, effect));
                    ((), settle(poll).progress())
                });
            }
            assert!(!registry.has_returned() && active.is_none());
            probe.retire_snapshot(root);
            probe.retire(semio_framework_value::FactoryAuthority::new(snapshots));
            probe.retire(semio_framework_value::FactoryAuthority::new(mutations));
            let bounded_turns = probe.bounded_turns;
            assert!(cancel_after.is_some() || bounded_turns == 1, "the cold edit ran {bounded_turns} quoted turns for operation {index}");
            let unpaid = probe.finish();
            assert_eq!(unpaid.is_none(), policy["expected"]["heapMatchesReceipts"].as_bool().unwrap(), "actual EN 1990 nested heap was not physically funded for operation {index}, cancellation {cancel_after:?}: {unpaid:?}");
        }
    }
}
