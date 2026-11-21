use member_receiving::MemberHistoryReceiving;
use semio_framework_actor_retained_context::OriginalActorContext;
use semio_framework_actor_retained_turn::{OriginalActorTurnWork, RetainedTurnInput};
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_async::poll::resolve_ready;
use semio_framework_job::StepBudget;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_os_kernel::os_store::text_format::{empty_document_spr, stamp_document_spr_identity};
use semio_s_artifact_norm_din18599_io::binary::snapshot::decode_din18599_pack;

fn clock() -> Option<u64> { Some(1) }

fn main() {
    let parent = decode_din18599_pack(include_bytes!("din18599-observed.pack")).unwrap();
    let id = &parent.climate_table.target.artifact_id;
    let dialect = &parent.climate_table.target.dialect;
    let owner = semio_os_kernel::os_store::OwnerRef {
        parent: ArtifactRef { artifact_id: "din18599-captured".into(), dialect: ArtifactDialect {
            artifact_kind: "s.norm.din18599".into(), standard: "1".into(), subset: "*".into(),
        }},
        slot: "climateTable".into(), child_id: parent.climate_table.child_id.clone(),
    };
    let schema = "stdio.semio.table";
    let empty = resolve_ready(empty_document_spr(id, schema));
    let bytes = resolve_ready(stamp_document_spr_identity(&empty, id, schema, dialect, Some(&owner))).unwrap();
    let expected = resolve_ready(semio_os_spr::decode_history(&bytes, &semio_os_spr::DecodeOptions::default())).unwrap();
    let fixture: serde_json::Value = serde_json::from_str(include_str!(env!("MEMBER_HISTORY_READER_LAW"))).unwrap();
    let limits = serde_json::from_value(fixture["limits"].clone()).unwrap();
    let mut receiving = MemberHistoryReceiving::new((0, bytes.len()), limits, 8 << 20);
    let mut output = None;
    let mut input = RetainedTurnInput {
        operation: 1, generation: 1, epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: 3_600_000_000, fuel: 60_000_000 },
        grant: RetainedCloneGrant {
            maximum_items: 8_388_608, maximum_copy_bytes: 536_870_912, maximum_capacity_bytes: 536_870_912,
            maximum_release_bytes: 536_870_912, maximum_depth: 64,
        },
    };
    let mut actor = OriginalActorContext::new((), input).unwrap();
    for _ in 0..100 {
        let budget = StepBudget::new(1 << 20, input.work.absolute_deadline_us, input.grant);
        let ready = actor.admit_step(input, budget, clock).unwrap().ready;
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
        if ready { break; }
    }
    actor.take_input().unwrap();
    let mut wire_ready = false;
    for turn in 0..200_000 {
        actor.begin_dispatch(input).unwrap();
        let mut observed = RetainedCloneProgress::default();
        let budget = StepBudget::new(1 << 20, input.work.absolute_deadline_us, input.grant);
        let mut cx = actor.borrow_context(input, budget, &mut observed, clock).unwrap();
        if !wire_ready {
            cx.acknowledge_return_rows_locally();
            wire_ready = cx.admit_original_core_wire_delivery().unwrap() && cx.admit_original_escrow_publisher().unwrap();
        } else {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 256 * 1024, ..cx.retained_grant() };
            let step = receiving.step(&bytes, &mut cx, grant, &mut output)
                .unwrap_or_else(|error| panic!("child history turn {turn}: {error:?}"));
            if turn % 20_000 == 0 || matches!(step, semio_os_spr::history::RetainedHistorySourceStep::Fault) {
                println!("[DEBUG] Child history turn {turn} {step:?} {} stock {:?}", receiving.probe_state(), cx.retained_grant());
            }
            assert!(!matches!(step, semio_os_spr::history::RetainedHistorySourceStep::Fault));
            if receiving.terminal_is_empty() {
                let log = output.take().expect("received original history");
                assert_eq!((log.doc_id, log.schema, log.composition), (expected.doc_id, expected.schema, expected.composition));
                assert_eq!(log.edits.len(), expected.edits.len());
                println!("[DEBUG] Child history completed at turn {turn}");
                std::process::exit(0);
            }
        }
        drop(cx);
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
    }
    panic!("child history did not complete")
}
