//! 🧪️ Runs the staged native lifetime raw owner against the canonical packet shape and actual Original Source.

use super::{loan, STOCK, TURN};
use crate::command_origin::OriginalCommandOrigin;
use crate::kernel::*;
use crate::original_media::OriginalMediaPickerIntent;
use crate::raw_reactor_owner::OriginalMediaAdmission;
use semio_framework_job::{Generation, OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::{RetainedCloneGrant, ToValue};

const PNG: &[u8] = include_bytes!("🗑️generated/paged-media-receiver/raw-picker.png");

#[test]
fn native_lifetime_owner_dispatches_raw_controls_pages_and_paid_close() {
    let clock = || Some(1);
    let mut source = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, clock, 1_000_000, 1 << 24);
    source.retain_dispatch().unwrap();
    let mut origin = OriginalCommandOrigin::empty();
    assert!(source.turn(TURN, |cx| origin.admit_step(cx)).unwrap());
    let mut admission = OriginalMediaAdmission::empty();
    let mut intent = None;
    assert!(source.turn(TURN, |cx| OriginalMediaPickerIntent::issue(
        loan(), [1, 0, 1, 0, 0, 0, 0, 0], ORIGINAL_MEDIA_COMMAND_KIND, 24_546, &mut intent, cx,
    )).unwrap());
    assert!(source.turn(TURN, |cx| admission.admit_issued(&mut intent, loan(), cx)).unwrap());
    let wire = OriginalMediaWireLoan {
        operation_id: format!("{:016x}", loan().operation_id), generation: format!("{:016x}", loan().generation),
        instance_id: loan().instance_id, document_id: loan().document_id.into(), base_revision: "13".repeat(32),
    };
    let live = (loan().document_id, *loan().base_revision);
    let extent = PNG.len() as u32;
    let mut event = OriginalMediaControlEvent {
        owner: u64::from(loan().instance_id), seq: 1 << 60, kind: ORIGINAL_MEDIA_COMMAND_KIND, extent,
        loan: wire, control: OriginalMediaWireControl::Begin { name: "raw-picker.png".into(), media_type: "image/png".into() },
    };
    let denied = RetainedCloneGrant { maximum_capacity_bytes: 0, ..TURN };
    assert!(source.turn(denied, |cx| admission.control(&event, Some(&live), cx)).unwrap().is_none());
    let ack = source.turn(TURN, |cx| admission.control(&event, Some(&live), cx)).unwrap().unwrap();
    let pack = ack.to_value();
    assert_eq!(pack.get("seq").unwrap().as_u64(), Some(event.seq));
    assert_eq!(pack.get("phase").unwrap().as_str(), Some("begin"));
    assert_eq!(ack.loan, event.loan);
    assert_eq!(ack.result, OriginalMediaWireResult::Accepted);
    let cursor = CommandPageCursor {
        owner: event.owner, generation: loan().generation, command_index: 0, command_count: 1, instance: loan().instance_id,
        seq: event.seq, kind: event.kind, page_index: 0, page_count: 2, item_count: 1, metadata: extent,
    };
    for (index, bytes) in PNG.chunks(4096).enumerate() {
        let mut page = Some((CommandPageCursor { page_index: index as u32, ..cursor }, FixedCommandPage::try_copy_from(bytes).unwrap()));
        assert!(source.turn(denied, |cx| admission.page(Some(&live), &mut page, cx)).unwrap().is_none());
        assert!(page.is_some());
        let ack = source.turn(TURN, |cx| admission.page(Some(&live), &mut page, cx)).unwrap().unwrap();
        assert_eq!((ack.phase, ack.page_index, ack.result), (OriginalMediaWirePhase::Page, Some(index as u32), OriginalMediaWireResult::Pending));
        assert!(page.is_none());
        let mut ack = None;
        for _ in 0..128 {
            ack = source.turn(TURN, |cx| admission.poll(Some(&live), true, cx)).unwrap();
            if ack.is_some() { break; }
        }
        let ack = ack.expect("raw fixed page earns a bounded acknowledgment");
        assert_eq!((ack.phase, ack.page_index, ack.result), (OriginalMediaWirePhase::Page, Some(index as u32), OriginalMediaWireResult::Accepted));
    }
    event.control = OriginalMediaWireControl::Complete;
    let ack = source.turn(TURN, |cx| admission.control(&event, Some(&live), cx)).unwrap().unwrap();
    assert_eq!(ack.result, OriginalMediaWireResult::Pending);
    event.control = OriginalMediaWireControl::Cancel;
    let ack = source.turn(TURN, |cx| admission.control(&event, Some(&live), cx)).unwrap().unwrap();
    assert_eq!(ack.result, OriginalMediaWireResult::Pending);
    assert!(source.turn(TURN, |cx| admission.poll(Some(&live), false, cx)).unwrap().is_none());
    let mut ack = None;
    for _ in 0..1024 {
        ack = source.turn(TURN, |cx| admission.poll(Some(&live), true, cx)).unwrap();
        if ack.is_some() { break; }
    }
    assert_eq!(ack.unwrap().result, OriginalMediaWireResult::Accepted);
    while !source.turn(TURN, |cx| cx.with_original_retirement(|cx| admission.close(cx))).unwrap() {}
    assert!(admission.terminal_is_empty());
    while !origin.terminal_is_empty() {
        source.turn(TURN, |cx| cx.with_original_retirement(|cx| origin.return_step(cx))).unwrap();
    }
    source.retire(TURN);
}
