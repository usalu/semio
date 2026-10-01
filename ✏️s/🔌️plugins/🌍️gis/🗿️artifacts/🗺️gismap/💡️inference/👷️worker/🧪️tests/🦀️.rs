//#region 💡️InferencePortLaws
#[cfg(not(target_arch = "wasm32"))]
use crate::inference_schema::*;
use semio_framework_os_kernel::os_directory::schema::DocumentScope;
use super::*;

#[cfg(not(target_arch = "wasm32"))]
fn inference_test_scope() -> DocumentScope {
    DocumentScope { space_id: "sp-1".into(), document_id: "doc-1".into() }
}

#[cfg(not(target_arch = "wasm32"))]
const INFERENCE_TEST_JOB: &str = "11111111111111111111111111111111";
#[cfg(not(target_arch = "wasm32"))]
const INFERENCE_TEST_HASH: &str = "9071779b724c67e0a45d5e23fddc8dbeb3d9b537936a4a14c293bc373960b130";

#[cfg(not(target_arch = "wasm32"))]
fn inference_test_receipt() -> GisMapInferenceJobReceiptV1 {
    GisMapInferenceJobReceiptV1 {
        schema: "semio.hub.inference-job-receipt/v1".into(),
        job_id: INFERENCE_TEST_JOB.into(),
        state: GisMapInferenceJobStateV1::Accepted,
        proposal_state: GisMapInferenceProposalStateV1::None,
        proposal_hash: None,
        cursor: 0,
        expires_at_ms: 1_700_000_060_000,
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// 🗺️ The exact bounded rectangular preview the hub publishes beside an offered proposal.
#[cfg(not(target_arch = "wasm32"))]
fn inference_test_preview(job_id: &str, proposal_hash: &str) -> GisMapInferencePreviewV1 {
    GisMapInferencePreviewV1 {
        schema: "semio.hub.gis-map-inference-preview/v1".into(),
        job_id: job_id.to_string(),
        proposal_hash: proposal_hash.to_string(),
        region_id: format!("inference-{job_id}"),
        ring: [[1.0, 2.0], [3.0, 2.0], [3.0, 4.0], [1.0, 4.0], [1.0, 2.0]],
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn inference_test_page(state: GisMapInferenceJobStateV1, proposal_state: GisMapInferenceProposalStateV1, proposal_hash: Option<&str>, cancel_requested: bool, stale: bool) -> GisMapInferenceEventPageV1 {
    GisMapInferenceEventPageV1 {
        schema: "semio.hub.inference-job-events/v1".into(),
        job_id: INFERENCE_TEST_JOB.into(),
        state,
        proposal_state,
        cancel_requested,
        stale,
        proposal_hash: proposal_hash.map(str::to_string),
        preview: proposal_hash.map(|hash| inference_test_preview(INFERENCE_TEST_JOB, hash)),
        events: Vec::new(),
        progress: vec![GisMapInferenceProgressV1 { cursor: 1, run_epoch: 1, completed: 1, total: 4, at_ms: 1_700_000_001_000 }],
        next_cursor: 1,
    }
}

/// 🪪️ The lease precondition is the very first thing the driver checks: with no verified
/// execution target it never asks for a single call and reports the localized terminal.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_refuses_to_start_without_a_verified_execution_target_lease() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), false);
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Terminal);
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Failed);
    assert_eq!(driver.status().code, Some(GisMapInferencePortCodeV1::LeaseUnverified));
    assert!(driver.status().job_id.is_none());
    assert_eq!(driver.turn(1_000), GisMapInferenceTurnV1::Terminal, "a terminal port is hard");
}

/// 🔄️ One bounded action at a time: a submit occupies the driver until its exact answer lands,
/// the next turn only polls after the timer deadline, and a Cancel click never fabricates a
/// `cancelled` phase — only the server's own page may.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_runs_one_bounded_action_at_a_time_and_never_optimistically_cancels() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), true);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Idle, "no intent, no work");
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Submit);
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Submitting);
    assert!(matches!(driver.turn(0), GisMapInferenceTurnV1::WaitUntil(_)), "a second action never starts while one is in flight");
    driver.complete(&GisMapInferencePortEventV1::Receipt(inference_test_receipt()));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Running);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Poll { job_id: INFERENCE_TEST_JOB.into(), after: 0 });
    driver.complete(&GisMapInferencePortEventV1::Page(inference_test_page(GisMapInferenceJobStateV1::Running, GisMapInferenceProposalStateV1::None, None, false, false)));
    assert_eq!(driver.status().completed, 1);
    assert_eq!(driver.status().total, 4);
    assert!(matches!(driver.turn(0), GisMapInferenceTurnV1::WaitUntil(_)), "polling is timer-armed, never a busy loop");

    driver.intend(GisMapInferenceIntentV1::Cancel);
    assert!(driver.status().cancel_requested, "a Cancel click is recorded as requested");
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Running, "and never as an optimistic terminal");
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Cancel { job_id: INFERENCE_TEST_JOB.into() });
    driver.complete(&GisMapInferencePortEventV1::Page(inference_test_page(GisMapInferenceJobStateV1::Cancelled, GisMapInferenceProposalStateV1::Cancelled, None, true, false)));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Cancelled);
    assert_eq!(driver.turn(10_000), GisMapInferenceTurnV1::Terminal);
}

/// ✅️ Approval is reachable only from an offered proposal with the server's own hash, is asked
/// for exactly once, and `applied` requires a receipt that actually committed.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_approves_only_an_offered_proposal_and_is_terminal_once() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), true);
    driver.intend(GisMapInferenceIntentV1::Approve);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Idle, "approval without an offer asks for nothing");
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Submit);
    driver.complete(&GisMapInferencePortEventV1::Receipt(inference_test_receipt()));
    let mut previewless = inference_test_page(GisMapInferenceJobStateV1::Succeeded, GisMapInferenceProposalStateV1::Offered, Some(INFERENCE_TEST_HASH), false, false);
    previewless.preview = None;
    driver.complete(&GisMapInferencePortEventV1::Page(previewless));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Offered);
    driver.intend(GisMapInferenceIntentV1::Approve);
    assert_eq!(driver.turn(5_000), GisMapInferenceTurnV1::Poll { job_id: INFERENCE_TEST_JOB.into(), after: 1 }, "an offer with no matching preview is never approved");
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Offered);
    driver.complete(&GisMapInferencePortEventV1::Page(inference_test_page(GisMapInferenceJobStateV1::Succeeded, GisMapInferenceProposalStateV1::Offered, Some(INFERENCE_TEST_HASH), false, false)));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Offered);
    driver.intend(GisMapInferenceIntentV1::Approve);
    assert_eq!(driver.turn(10_000), GisMapInferenceTurnV1::Approve { job_id: INFERENCE_TEST_JOB.into(), proposal_hash: INFERENCE_TEST_HASH.into() });
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Approving);
    driver.complete(&GisMapInferencePortEventV1::Approval(GisMapInferenceApprovalReceiptV1 {
        schema: "semio.hub.inference-approval-receipt/v1".into(),
        job_id: INFERENCE_TEST_JOB.into(),
        mutation_id: INFERENCE_TEST_HASH.into(),
        command_hash: INFERENCE_TEST_HASH.into(),
        proposal_hash: INFERENCE_TEST_HASH.into(),
        applied: true,
        undo: GisMapApprovalUndoHandleV1 {
            target_id: "22".repeat(16),
            expected_current: semio_framework_os_kernel::os_directory::EditedArtifactFrontierV1 {
                document_id: "document-map".into(),
                head_edit_ordinal: 2,
                head_edit_id: "approval-edit".into(),
                last_commit_seq: 2,
                chain_sha256: INFERENCE_TEST_HASH.into(),
            },
        },
    }));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Applied);
    assert_eq!(driver.turn(20_000), GisMapInferenceTurnV1::Terminal);
    driver.intend(GisMapInferenceIntentV1::Approve);
    assert_eq!(driver.turn(30_000), GisMapInferenceTurnV1::Terminal, "a committed port never re-approves");
}

/// ⏳️ The poll budget is finite: an unanswered job retires as indeterminate instead of spinning.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_retires_after_its_bounded_poll_budget() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), true);
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Submit);
    driver.complete(&GisMapInferencePortEventV1::Receipt(inference_test_receipt()));
    let mut now_ms = 0u64;
    for _ in 0..GisMapInferenceDriverV1::MAX_POLL_TURNS {
        assert!(matches!(driver.turn(now_ms), GisMapInferenceTurnV1::Poll { .. }));
        driver.complete(&GisMapInferencePortEventV1::Page(inference_test_page(GisMapInferenceJobStateV1::Running, GisMapInferenceProposalStateV1::None, None, false, false)));
        now_ms = now_ms.saturating_add(GisMapInferenceDriverV1::POLL_INTERVAL_MS);
    }
    assert_eq!(driver.turn(now_ms), GisMapInferenceTurnV1::Terminal);
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Failed);
    assert_eq!(driver.status().code, Some(GisMapInferencePortCodeV1::Transport));
}

/// 🧊️ A page for a different job id can never move this port.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_ignores_an_answer_for_another_job() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), true);
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Submit);
    driver.complete(&GisMapInferencePortEventV1::Receipt(inference_test_receipt()));
    let mut foreign = inference_test_page(GisMapInferenceJobStateV1::Succeeded, GisMapInferenceProposalStateV1::Offered, Some(INFERENCE_TEST_HASH), false, false);
    foreign.job_id = "22222222222222222222222222222222".into();
    driver.complete(&GisMapInferencePortEventV1::Page(foreign));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Running);
    assert!(driver.status().proposal_hash.is_none());
}
//#endregion 💡️InferencePortLaws

fn recovery_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }

fn recovery_value(value: &serde_json::Value) -> DslValue {
    let text=serde_json::to_string(value).unwrap();
    let own=semio_framework_os_kernel::os_pack::json::from_json_str::<DslValue>(&text).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_os_kernel::os_pack::json::to_json_string(&own)).unwrap(),*value);
    own
}

fn recovery_driver(fixture:&serde_json::Value) -> NativeGisMapServiceV1 {
    NativeGisMapServiceV1 {driver:GisMapInferenceDriverV1::new(inference_test_scope(),true),request_id:fixture["originalRequestId"].as_str().unwrap().into(),reconcile_required:false,recovery_turns:0,next_recovery_at_ms:0,recovery_in_flight:false,exhausted:false}
}

fn recovery_call(driver:&mut dyn InstalledServiceDriverV1,time:u64,expected:&str)->DslValue {
    let InstalledServiceTurnV1::Call {action,payload}=driver.turn(time) else {panic!("expected owner call {expected}")};
    assert_eq!(action,expected);
    payload
}

#[test]
fn native_service_retains_original_request_and_cancels_only_after_recovery() {
    let fixture=recovery_fixture();
    let mut driver=recovery_driver(&fixture);
    driver.intend("propose",DslValue::Object(vec![])).unwrap();
    let submit=recovery_call(&mut driver,0,"submit");
    driver.complete("submit",&submit,Err(DocumentHttpPortCodeV1::Transport));
    assert_eq!(driver.driver.status.phase,GisMapInferencePortPhaseV1::Indeterminate);
    driver.intend(fixture["intentBeforeReceipt"].as_str().unwrap(),DslValue::Object(vec![])).unwrap();
    assert!(!driver.terminal());
    let reconcile=recovery_call(&mut driver,0,"reconcile");
    let request:serde_json::Value=serde_json::from_str(&semio_framework_os_kernel::os_pack::json::to_json_string(&reconcile)).unwrap();
    assert_eq!(request["requestId"],fixture["originalRequestId"]);
    driver.complete("reconcile",&reconcile,Ok(recovery_value(&fixture["recovered"])));
    assert!(driver.driver.status.cancel_requested);
    let cancel=recovery_call(&mut driver,1,"cancel");
    driver.complete("cancel",&cancel,Ok(recovery_value(&fixture["cancelled"])));
    assert_eq!(driver.driver.status.phase,GisMapInferencePortPhaseV1::Cancelled);
    assert!(driver.terminal());
}

#[test]
fn native_service_missing_admission_has_finite_recovery_and_no_replacement_submit() {
    let fixture=recovery_fixture();let mut driver=recovery_driver(&fixture);
    driver.intend("propose",DslValue::Object(vec![])).unwrap();
    let submit=recovery_call(&mut driver,0,"submit");driver.complete("submit",&submit,Err(DocumentHttpPortCodeV1::Transport));
    for turn in 0..fixture["maxRecoveryTurns"].as_u64().unwrap() {
        let request=recovery_call(&mut driver,turn*GisMapInferenceDriverV1::POLL_INTERVAL_MS,"reconcile");
        driver.complete("reconcile",&request,Ok(recovery_value(&fixture["missing"])));
    }
    assert!(matches!(driver.turn(240*GisMapInferenceDriverV1::POLL_INTERVAL_MS),InstalledServiceTurnV1::Terminal));
    assert_eq!(driver.driver.status.phase,GisMapInferencePortPhaseV1::Indeterminate);
    assert!(driver.terminal());
}

#[test]
fn native_service_refuses_recovery_for_a_different_original_request() {
    let fixture=recovery_fixture();let mut driver=recovery_driver(&fixture);
    driver.intend("propose",DslValue::Object(vec![])).unwrap();
    let submit=recovery_call(&mut driver,0,"submit");driver.complete("submit",&submit,Err(DocumentHttpPortCodeV1::Transport));
    let request=recovery_call(&mut driver,0,"reconcile");let mut substituted=fixture["recovered"].clone();substituted["requestId"]="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into();
    driver.complete("reconcile",&request,Ok(recovery_value(&substituted)));
    assert_eq!(driver.driver.status.code,Some(GisMapInferencePortCodeV1::Invalid));
    assert!(driver.terminal());
}

#[test]
fn production_owner_contribution_injects_live_native_recovery_driver() {
    let mut fixture=recovery_fixture();let contribution=gis_map_service_contribution_v1();
    assert_eq!(contribution.owner,"gis");assert_eq!(contribution.service_id,GIS_MAP_INFERENCE_SERVICE_ID);
    let mut driver=(contribution.create)(inference_test_scope(),true);
    driver.intend("propose",DslValue::Object(vec![])).unwrap();
    let submit=recovery_call(driver.as_mut(),0,"submit");
    let request=GisMapInferenceJobRequestV1::from_value(submit.clone()).unwrap();
    fixture["recovered"]["requestId"]=request.request_id.into();
    driver.complete("submit",&submit,Err(DocumentHttpPortCodeV1::Transport));
    driver.intend("close",DslValue::Object(vec![])).unwrap();
    let reconcile=recovery_call(driver.as_mut(),0,"reconcile");
    driver.complete("reconcile",&reconcile,Ok(recovery_value(&fixture["recovered"])));
    let cancel=recovery_call(driver.as_mut(),1,"cancel");
    driver.complete("cancel",&cancel,Ok(recovery_value(&fixture["cancelled"])));
    assert!(driver.terminal());
    assert_eq!(GisMapInferencePortStatusV1::from_value(driver.status()).unwrap().phase,GisMapInferencePortPhaseV1::Cancelled);
}
