use semio_framework_value::{
    OriginalRetirementFailure, RetainedWorkOutcome, RetainedWorkPoll, RetireOwned, ValueError,
    retained_clone::{
        OriginalEffectReply, OriginalEffectReplyComplete, RetainedAllocationEffect,
        RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep,
    },
};
use semio_s_artifact_layout_layout_io::interchange::DrawingBirthPreparation;
use semio_framework_value_testing_admission::{EffectBank, Wallet, demand_grant, denials, driven};
use semio_s_artifact_layout_layout_model::{LayoutDrawingBirth, LayoutDrawingBirthSource};

type Reply<'r> = OriginalEffectReply<'r, RetainedCloneStep, OriginalRetirementFailure>;
type Complete<'r> = OriginalEffectReplyComplete<'r, RetainedCloneStep, OriginalRetirementFailure>;

const BODY: usize = 256;
const TURN_LIMIT: usize = 100_000;

fn bank() -> EffectBank {
    EffectBank::new(Wallet::new(
        RetainedCloneGrant {
            maximum_items: TURN_LIMIT,
            maximum_copy_bytes: 1 << 30,
            maximum_capacity_bytes: 1 << 25,
            maximum_release_bytes: 1 << 25,
            maximum_depth: 256,
        },
        1 << 30,
    ))
}

fn corpus() -> serde_json::Value {
    serde_json::from_str(include_str!("🔣️.json")).unwrap()
}

fn decode<T: semio_framework_value::FromValue>(value: &serde_json::Value) -> T {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

fn birth(row: &serde_json::Value) -> LayoutDrawingBirth<String> {
    LayoutDrawingBirth {
        source: decode::<LayoutDrawingBirthSource>(&row["source"]),
        member: decode(&row["member"]),
        initial: row["initial"].as_str().unwrap().to_string(),
    }
}

fn observe(
    bank: &mut EffectBank,
    grant: RetainedCloneGrant,
    work: u64,
    call: impl for<'r, 'e, 'a> FnMut(&'e mut RetainedAllocationEffect<'a>, Reply<'r>, bool) -> Result<Complete<'r>, ValueError>,
) -> RetainedCloneProgress {
    let ((poll, _), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| driven(bank, grant, work, call));
    let RetainedWorkPoll::Complete(turn) = poll else { panic!("Drawing turn failed to settle"); };
    let receipt = turn.result.unwrap().progress();
    assert!(turn.work <= u128::from(work));
    assert!(!heap.overflowed);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
    bank.post(receipt, grant, (heap.requested_bytes, heap.released_bytes));
    receipt
}

fn close_owned<T: RetireOwned>(bank: &mut EffectBank, mut value: Option<T>) {
    let mut close = RetainedCloneClose::default();
    for _ in 0..TURN_LIMIT {
        if value.is_none() && close.is_empty() { return; }
        let demand = close.next_owner_demands::<T>(value.is_some(), &None, BODY).unwrap();
        let work = close.next_owner_work::<T>(value.is_some(), &None, BODY).unwrap();
        let grant = demand_grant(demand);
        if close.is_empty() {
            observe(bank, grant, work, |effect, reply, _| {
                let RetainedWorkOutcome { result, .. } = close.begin_granted(&mut value, effect);
                Ok(reply.complete(result.map(|step| step.expect("owned close admitted")).map_err(Into::into)))
            });
        } else {
            observe(bank, grant, work, |effect, reply, recover| {
                if recover { close.recover_step_into(BODY, effect, reply) } else { close.step_into(BODY, effect, reply) }
            });
        }
    }
    panic!("owned Drawing close exceeded its turn bound");
}

fn admit(bank: &mut EffectBank, owner: &mut DrawingBirthPreparation<String>, input: &mut Option<LayoutDrawingBirth<String>>, grant: RetainedCloneGrant, work: u64) -> RetainedCloneProgress {
    observe(bank, grant, work, |effect, reply, _| owner.admit_into(input, effect, reply))
}

fn step(bank: &mut EffectBank, owner: &mut DrawingBirthPreparation<String>, grant: RetainedCloneGrant, work: u64) -> RetainedCloneProgress {
    observe(bank, grant, work, |effect, reply, recover| {
        if recover { owner.recover_step_into(BODY, None, effect, reply) } else { owner.step_into(BODY, None, effect, reply) }
    })
}

fn close(bank: &mut EffectBank, owner: &mut DrawingBirthPreparation<String>) {
    observe(bank, demand_grant(owner.begin_close_demand()), 1, |effect, reply, _| owner.begin_close_into(effect, reply));
    for _ in 0..TURN_LIMIT {
        if owner.terminal_is_empty() { return; }
        let demand = owner.demand(BODY).unwrap();
        let work = owner.work_demand(BODY).unwrap();
        let grant = demand_grant(demand);
        for denial in denials(grant, demand) {
            assert_eq!(step(bank, owner, denial, work), Default::default());
            assert_eq!(owner.demand(BODY).unwrap(), demand);
        }
        step(bank, owner, grant, work);
    }
    panic!("Drawing birth close exceeded its turn bound");
}

#[test]
fn parent_reference_copy_preserves_the_exact_original_birth_and_physical_ledger() {
    for row in corpus()["cases"].as_array().unwrap() {
        let mut bank = bank();
        let original = birth(row);
        let payload = original.initial.as_ptr();
        let child = original.member.handle.child_id.as_ptr();
        let mut owner = DrawingBirthPreparation::default();
        let mut input = Some(original);
        admit(&mut bank, &mut owner, &mut input, demand_grant(DrawingBirthPreparation::<String>::admission_demand()), 1);
        assert!(input.is_none());
        for _ in 0..TURN_LIMIT {
            if owner.ready() { break; }
            let demand = owner.demand(BODY).unwrap();
            let work = owner.work_demand(BODY).unwrap();
            let grant = demand_grant(demand);
            for denial in denials(grant, demand) {
                assert_eq!(step(&mut bank, &mut owner, denial, work), Default::default());
                assert_eq!(owner.demand(BODY).unwrap(), demand);
            }
            if work > 0 {
                assert_eq!(step(&mut bank, &mut owner, grant, work - 1), Default::default());
                assert_eq!(owner.demand(BODY).unwrap(), demand);
            }
            let progress = step(&mut bank, &mut owner, grant, work);
            if progress == RetainedCloneProgress::default() && !owner.ready() {
                assert_ne!(
                    (owner.demand(BODY).unwrap(), owner.work_demand(BODY).unwrap()),
                    (demand, work),
                    "the exactly funded Drawing birth turn must advance",
                );
            }
        }
        assert!(
            owner.ready(),
            "Drawing birth did not finish in {TURN_LIMIT} turns: next demand {:?}, next Work {:?}",
            owner.demand(BODY),
            owner.work_demand(BODY),
        );
        let mut output = None;
        let demand = owner.take_demand().unwrap();
        let grant = demand_grant(demand);
        for (denial, work) in denials(grant, demand).map(|denial| (denial, 1)).chain([(grant, 0)]) {
            assert_eq!(observe(&mut bank, denial, work, |effect, reply, _| owner.take_into(&mut output, effect, reply)), Default::default());
            assert!(output.is_none() && owner.ready());
        }
        observe(&mut bank, grant, 1, |effect, reply, _| owner.take_into(&mut output, effect, reply));
        let (parent, birth) = output.unwrap();
        assert_eq!(parent, birth.member);
        assert_eq!(birth.initial.as_ptr(), payload);
        assert_eq!(birth.member.handle.child_id.as_ptr(), child);
        assert_ne!(parent.handle.child_id.as_ptr(), child);
        close(&mut bank, &mut owner);
        close_owned(&mut bank, Some(parent));
        close_owned(&mut bank, Some(birth));
    }
}

#[test]
fn cancellation_closes_original_aliases_prepared_fields_and_unmoved_payloads() {
    let corpus = corpus();
    for row in corpus["cases"].as_array().unwrap() {
        for cut in corpus["cuts"].as_array().unwrap() {
            let mut bank = bank();
            let mut owner = DrawingBirthPreparation::default();
            let mut input = Some(birth(row));
            admit(&mut bank, &mut owner, &mut input, demand_grant(DrawingBirthPreparation::<String>::admission_demand()), 1);
            for _ in 0..cut.as_u64().unwrap() {
                if owner.ready() { break; }
                let grant = demand_grant(owner.demand(BODY).unwrap());
                let work = owner.work_demand(BODY).unwrap();
                step(&mut bank, &mut owner, grant, work);
            }
            close(&mut bank, &mut owner);
        }
    }
}

#[test]
fn birth_admission_quotes_actual_heap_and_preserves_every_undergranted_original() {
    let demand = DrawingBirthPreparation::<String>::admission_demand();
    let grant = demand_grant(demand);
    for row in corpus()["cases"].as_array().unwrap() {
        let mut bank = bank();
        let mut input = Some(birth(row));
        let member = input.as_ref().unwrap().member.handle.child_id.as_ptr();
        let payload = input.as_ref().unwrap().initial.as_ptr();
        let mut owner = DrawingBirthPreparation::default();
        assert!(owner.terminal_is_empty());
        for (denial, work) in denials(grant, demand).map(|denial| (denial, 1)).chain([(grant, 0)]) {
            assert_eq!(admit(&mut bank, &mut owner, &mut input, denial, work), Default::default());
            assert_eq!(input.as_ref().unwrap().member.handle.child_id.as_ptr(), member);
            assert_eq!(input.as_ref().unwrap().initial.as_ptr(), payload);
        }
        admit(&mut bank, &mut owner, &mut input, grant, 1);
        assert!(input.is_none());
        close(&mut bank, &mut owner);
    }
}
