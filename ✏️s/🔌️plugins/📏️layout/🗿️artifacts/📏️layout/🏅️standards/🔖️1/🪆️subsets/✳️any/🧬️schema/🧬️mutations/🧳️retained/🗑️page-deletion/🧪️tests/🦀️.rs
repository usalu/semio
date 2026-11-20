use semio_framework_value::{
    RetirementDemand, RetirementPollExit,
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep},
};
use semio_s_artifact_layout_layout_mutations_decisions::{
    delete_page::DeletePage, page_deletion::{DeletePageCoordinates, DeletePagePlan, DeletePageRefusal},
};
use semio_framework_value_clone_source_testing::{FixtureSource, drive::drive};
use semio_s_artifact_layout_layout_model::{
    Frame, Spread, io::text::snapshot::snapshot_wire_codec::default_document, snapshot::LayoutSnapshot,
};
use serde_json::{Value, json};

const BODY: usize = 4096;

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_depth: demand.depth,
        maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes,
    }
}

fn input(case: &Value) -> (DeletePage, LayoutSnapshot) {
    let mut base = default_document();
    let page = base.pages[0].clone();
    let text = page.frames.iter().find(|frame| matches!(frame, Frame::Text { .. })).unwrap();
    let prefix = case["framePrefix"]["character"].as_str()
        .map(|character| character.repeat(case["framePrefix"]["count"].as_u64().unwrap() as usize))
        .unwrap_or_default();
    base.pages = case["pages"].as_array().unwrap().iter().map(|row| {
        let mut page = page.clone();
        page.id = row["id"].as_str().unwrap().into();
        page.spread_id = row["spread"].as_str().unwrap().into();
        page.frames = row["frames"].as_array().unwrap().iter().map(|row| {
            let mut frame = text.clone();
            let Frame::Text { id, thread_next, .. } = &mut frame else { unreachable!() };
            *id = format!("{prefix}{}", row[0].as_str().unwrap());
            *thread_next = row[1].as_str().map(|next| format!("{prefix}{next}"));
            frame
        }).collect();
        page.layers[0].object_ids = page.frames.iter().map(|frame| frame.id().to_owned()).collect();
        page
    }).collect();
    base.spreads = case["spreads"].as_array().unwrap().iter().map(|row| {
        let id = row[0].as_str().unwrap().to_owned();
        let page_ids = row[1].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_owned()).collect();
        Spread { name: id.clone(), id, page_ids }
    }).collect();
    (DeletePage { id: case["delete"].as_str().unwrap().into() }, base)
}

fn output(decision: Result<DeletePageCoordinates, DeletePageRefusal>) -> Value {
    match decision {
        Ok(at) => json!({ "page": at.page, "spread": at.spread, "member": at.member }),
        Err(reason) => match reason {
            DeletePageRefusal::MissingPage => json!({ "error": "missing-page", "code": reason.code() }),
            DeletePageRefusal::MissingSpread { page } => json!({ "error": "missing-spread", "code": reason.code(), "page": page }),
            DeletePageRefusal::SurvivingThread => json!({ "error": "surviving-thread", "code": reason.code() }),
        },
    }
}

fn advance(plan: &mut DeletePagePlan, mutation: RetainedCloneRef<'_, DeletePage>, base: RetainedCloneRef<'_, LayoutSnapshot>, denied: bool) -> RetainedCloneStep {
    let demand = plan.demand(mutation, base).unwrap();
    let mut grant = grant(demand);
    if denied { grant.maximum_copy_bytes = demand.copy_bytes - 1; }
    let work = plan.work_demand(mutation, base).unwrap();
    drive(grant, work, |effect| {
        let step = plan.advance(mutation, base, effect).map_err(Into::into);
        RetirementPollExit::settle(effect, step)
    }).result.unwrap()
}

fn close(plan: &mut DeletePagePlan) {
    plan.begin_close();
    for _ in 0..64 {
        if plan.terminal_is_empty() { return; }
        assert!(!plan.requires_original_birth());
        assert!(!plan.requires_original_control());
        let grant = grant(plan.close_demand(BODY).unwrap());
        let work = plan.close_work_demand(BODY).unwrap();
        let mut recover = false;
        drive(grant, work, |effect| {
            let step = plan.close_step(None, BODY, effect, recover);
            recover = matches!(&step, Err(RetirementPollExit::Pending(_)));
            RetirementPollExit::settle(effect, step)
        }).result.unwrap();
    }
    assert!(plan.terminal_is_empty());
}

#[test]
fn identifies_the_original_page_spread_and_incoming_thread_under_funded_turns() {
    let fixture: Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let (mutation, base) = input(case);
        let before = FixtureSource::from_owner(base.clone());
        let foreign = FixtureSource::from_owner(base);
        let event = FixtureSource::from_owner(mutation.clone());
        let other_event = FixtureSource::from_owner(mutation);
        let mut plan = DeletePagePlan::default();
        for turn in 0..128 {
            if plan.result().is_some() { break; }
            assert_eq!(advance(&mut plan, event.borrow(), before.borrow(), true).progress(), RetainedCloneProgress::default());
            advance(&mut plan, event.borrow(), before.borrow(), false);
            if turn == 0 {
                assert!(plan.demand(event.borrow(), foreign.borrow()).is_err());
                assert!(plan.demand(other_event.borrow(), before.borrow()).is_err());
            }
        }
        assert_eq!(output(plan.result().expect("funded plan completes")), case["expected"], "{}", case["name"]);
        close(&mut plan);
        assert!(plan.result().is_none());
        for cut in fixture["cuts"].as_array().unwrap() {
            let mut plan = DeletePagePlan::default();
            for _ in 0..cut.as_u64().unwrap() {
                if plan.result().is_some() { break; }
                advance(&mut plan, event.borrow(), before.borrow(), false);
            }
            close(&mut plan);
        }
    }
}
