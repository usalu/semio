use member_record::OriginalRecordMemberSnapshotOpen;
pub use semio_os_store_member_open_store::{MemberSnapshotOpenOperation, MemberSnapshotOpenProgress, MemberSnapshotOpenStep};

use semio_framework_actor_retained_context::OriginalActorContext;
use semio_framework_actor_retained_turn::{OriginalActorTurnWork, RetainedTurnInput};
use semio_framework_artifact_reference::{ArtifactDialect, ArtifactRef};
use semio_framework_job::{Generation, OperationId, StepBudget, StepContext};
use semio_framework_os_store_member_open::MemberOpenRequest;
use semio_framework_value::{ErasedSnapshotRetirement, RetainedCloneGrant, RetainedCloneProgress};
use semio_os_spr::ActorId;
use semio_os_store_owned_schema_decode::{
    OwnedSchemaDecodeCredits, OwnedSchemaDecodePage, OwnedSchemaDecodePages, OWNED_SCHEMA_DECODE_PAGE_BYTES,
};
use semio_s_artifact_norm_din18599_io::binary::snapshot::decode_din18599_pack;
use semio_s_artifact_norm_din18599_model::{DIN18599_CLIMATE_TABLE_SLOT, genesis_din18599_child_pack};
use semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot;
use pack::snapshot::ArtifactPack;

fn clock() -> Option<u64> { Some(1) }

fn main() {
    let mounted = std::env::args().any(|arg| arg == "mounted");
    let loan_items = std::env::args().find_map(|arg| arg.strip_prefix("loan-items=")?.parse::<usize>().ok());
    let keep_scope_stock = std::env::args().any(|arg| arg == "scope-stock");
    let member_items = std::env::args().find_map(|arg| arg.strip_prefix("member-items=")?.parse::<usize>().ok()).unwrap_or(64);
    let parent = decode_din18599_pack(include_bytes!("din18599-observed.pack")).unwrap();
    let bytes = genesis_din18599_child_pack(&parent, DIN18599_CLIMATE_TABLE_SLOT, &parent.climate_table.child_id).unwrap();
    let expected = SemioTableSnapshot::decode_pack(&bytes).unwrap();
    let mut frame = Vec::new();
    let mut length = bytes.len();
    while length >= 128 {
        frame.push(((length & 127) as u8) | 128);
        length >>= 7;
    }
    frame.push(length as u8);
    frame.extend_from_slice(&bytes);
    frame.push(0);
    let reference = ArtifactRef {
        artifact_id: parent.climate_table.child_id,
        dialect: ArtifactDialect {
            artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "table".into(),
        },
    };
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits {
        maximum_pages: frame.len().div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES), maximum_bytes: frame.len(),
    }).unwrap();
    for page in frame.chunks(OWNED_SCHEMA_DECODE_PAGE_BYTES) {
        pages.admit_page(OwnedSchemaDecodePage::try_from_slice(page).unwrap()).unwrap();
    }
    pages.seal().unwrap();
    let request = MemberOpenRequest::new(OperationId(1), Generation(1), 1000, reference, None, pages, ActorId("author".into()))
        .admit(1).unwrap_or_else(|_| panic!("member request admission"));
    let mut open = OriginalRecordMemberSnapshotOpen::<SemioTableSnapshot>::begin(request)
        .unwrap_or_else(|_| panic!("original record begin"));
    let mut input = RetainedTurnInput {
        operation: 1, generation: 1, epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: 3_600_000_000, fuel: 900_000_000 },
        grant: RetainedCloneGrant {
            maximum_items: 8_388_608, maximum_copy_bytes: 536_870_912, maximum_capacity_bytes: 536_870_912,
            maximum_release_bytes: 536_870_912, maximum_depth: 64,
        },
    };
    let mut actor = OriginalActorContext::new((), input).unwrap();
    for _ in 0..100 {
        let budget = StepBudget::new(input.work.fuel.min(1 << 20), input.work.absolute_deadline_us, input.grant);
        let ready = actor.admit_step(input, budget, clock).unwrap().ready;
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
        if ready { break; }
    }
    actor.take_input().unwrap();
    let mut wire_ready = false;
    let mut last_phase = String::new();
    for turn in 0..100_000 {
        actor.begin_dispatch(input).unwrap();
        let mut observed = RetainedCloneProgress::default();
        let budget = StepBudget::new(input.work.fuel.min(1 << 20), input.work.absolute_deadline_us, input.grant);
        let mut cx = actor.borrow_context(input, budget, &mut observed, clock).unwrap();
        if !wire_ready {
            cx.acknowledge_return_rows_locally();
            wire_ready = cx.admit_original_core_wire_delivery().unwrap() && cx.admit_original_escrow_publisher().unwrap();
        } else {
            let phase = open.probe_phase();
            if phase != last_phase {
                println!("[DEBUG] child record turn {turn} phase {phase}");
                last_phase = phase;
            }
            let stock = cx.retained_grant();
            let mut grant = RetainedCloneGrant {
                maximum_items: member_items,
                maximum_copy_bytes: stock.maximum_copy_bytes.min(256 * 1024), ..stock
            };
            if mounted {
                grant.maximum_depth -= 1;
                grant.maximum_capacity_bytes = grant.maximum_capacity_bytes.min(16 << 20);
                grant.maximum_release_bytes = grant.maximum_release_bytes.min(1 << 20);
            }
            let mut stepped = None;
            let mut advance = |cx: &mut StepContext<'_>| {
                let mut body = |cx: &mut StepContext<'_>| {
                    if turn % 1000 == 0 {
                        println!("[DEBUG] member frontier {turn}: {}; remaining {:?}", open.probe_frontier(cx, grant.maximum_copy_bytes), cx.retained_grant());
                    }
                    stepped = Some(open.step(cx, grant)?);
                    Ok(())
                };
                match loan_items {
                    Some(maximum_items) => cx.with_retained_loan(RetainedCloneGrant { maximum_items, ..if keep_scope_stock {stock} else {grant} }, body),
                    None => body(cx),
                }
            };
            if mounted { cx.with_original_retirement(advance) } else { advance(&mut cx) }
                .unwrap_or_else(|error| panic!("child record turn {turn} phase {last_phase}: {error:?}"));
            let step = stepped.unwrap();
            if let MemberSnapshotOpenStep::Ready = step {
                if let Some((received, request)) = open.take_ready(&mut cx).unwrap() {
                    assert_eq!(received, expected);
                    assert!(open.terminal_is_empty());
                    assert!(request.retained_input_bytes() > 0);
                    let columns = received.columns.iter().map(|column| &column.name).collect::<Vec<_>>();
                    let climate = semio_s_artifact_norm_din18599_model::din18599_climate_data_from_table(&received);
                    println!("[DEBUG] climate columns {columns:?}");
                    println!("[DEBUG] climate temperatures {:?}", climate.theta_e_c);
                    println!("[DEBUG] climate irradiation {:?}", climate.g_h_w_m2);
                    println!("[DEBUG] child record delivered {} rows on turn {turn}", received.rows.len());
                    std::process::exit(0);
                }
            }
        }
        drop(cx);
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
    }
    panic!("child record never reached terminal in {last_phase}");
}
