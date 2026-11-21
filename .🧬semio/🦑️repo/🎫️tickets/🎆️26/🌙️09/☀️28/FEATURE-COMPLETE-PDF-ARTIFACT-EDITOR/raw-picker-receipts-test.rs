//! 🧪 Exercises raw transport admission and custody with the retained native Original caller.

use super::{STOCK, TURN};
#[path = "🗑️generated/paged-media-receiver/browser-law.rs"]
mod browser;
use browser::{GENERATION_HEX, LIMIT, LOAN, OPERATION_HEX, RAW_KIND, REVISION_HEX, SEQ};
use crate::command_origin::OriginalCommandOrigin;
use crate::raw_picker::{Control, FormatResult, Phase, RawPicker, ResultCode, Transport};
use semio_framework_job::{Generation, OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage};
use semio_framework_value::RetainedCloneGrant;

const PNG: &[u8] = include_bytes!("🗑️generated/paged-media-receiver/raw-picker.png");
const BEGIN: Control<'static> = Control::Begin { name: "raw-picker.png", media_type: "image/png" };
type Picker = RawPicker<[u32; 2], LIMIT>;

fn transport() -> Transport {
    Transport { owner: u64::from(loan().instance_id), seq: SEQ, kind: RAW_KIND, extent: PNG.len() as u32 }
}

fn loan() -> crate::ingress::OriginalMediaLoan<'static> { LOAN }

fn page(index: u32, transport: Transport, body: &[u8]) -> Option<(CommandPageCursor, FixedCommandPage)> {
    Some((CommandPageCursor {
        owner: transport.owner, seq: transport.seq, kind: transport.kind, generation: loan().generation,
        instance: loan().instance_id, command_index: 0, command_count: 1, item_count: 1,
        metadata: transport.extent, page_index: index, page_count: transport.extent.div_ceil(4096),
    }, FixedCommandPage::try_copy_from(body).unwrap()))
}

#[test]
fn accepts_the_original_png_once_and_replays_the_last_receipts() {
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, || Some(1), 1_000_000, 1 << 24);
    let mut foreign = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, || Some(1), 1_000_000, 1 << 24);
    caller.retain_dispatch().unwrap();
    foreign.retain_dispatch().unwrap();
    let mut picker = Picker::empty();
    let mut origin = OriginalCommandOrigin::empty();
    let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN };
    let route = transport();
    assert!(caller.turn(TURN, |cx| origin.admit_step(cx)).unwrap());
    assert!(foreign.turn(TURN, |cx| origin.validate(cx)).is_err());
    assert!(crate::raw_picker_pack::picker_args(&picker).is_none());
    assert!(caller.turn(TURN, |cx| picker.issue(loan(), [4, 3], RAW_KIND, LIMIT, cx)).unwrap());
    assert_eq!(picker.issued_loan(), Some(loan()));
    let args = crate::raw_picker_pack::picker_args(&picker).unwrap();
    let request = args.get("originalMedia").unwrap();
    assert_eq!(request.get("kind").unwrap().as_u64(), Some(u64::from(RAW_KIND)));
    assert_eq!(request.get("maximumFileBytes").unwrap().as_u64(), Some(LIMIT as u64));
    let stamp = request.get("loan").unwrap();
    assert_eq!(stamp.get("instanceId").unwrap().as_u64(), Some(u64::from(loan().instance_id)));
    assert_eq!(stamp.get("documentId").unwrap().as_str(), Some(loan().document_id));
    assert_eq!(stamp.get("operationId").unwrap().as_str(), Some(OPERATION_HEX));
    assert_eq!(stamp.get("generation").unwrap().as_str(), Some(GENERATION_HEX));
    assert_eq!(stamp.get("baseRevision").unwrap().as_str(), Some(REVISION_HEX));
    let text = |field| stamp.get(field).unwrap().as_str().unwrap();
    let decoded = crate::raw_picker_pack::DecodedLoan::read(
        text("operationId"), text("generation"), loan().instance_id, text("documentId"), text("baseRevision"),
    ).unwrap();
    assert_eq!(decoded.as_loan(), loan());
    assert!(crate::raw_picker_pack::DecodedLoan::read(
        text("operationId"), text("generation"), loan().instance_id, text("documentId"), &"1A".repeat(32),
    ).is_none());
    let mut changed_revision = *loan().base_revision;
    changed_revision[31] ^= 1;
    let stale = crate::ingress::OriginalMediaLoan { base_revision: &changed_revision, ..loan() };
    assert_eq!(caller.turn(TURN, |cx| picker.control(stale, loan(), route, BEGIN, cx).map(|ack| ack.result)).unwrap(), ResultCode::Stale);
    assert!(foreign.turn(TURN, |cx| picker.control(loan(), loan(), route, BEGIN, cx).map(|ack| ack.result)).is_err());
    assert_eq!(caller.turn(denied, |cx| picker.control(loan(), loan(), route, BEGIN, cx).map(|ack| ack.result)).unwrap(), ResultCode::Retry);
    assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), loan(), route, BEGIN, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
    assert_eq!(caller.turn(denied, |cx| picker.control(loan(), loan(), route, BEGIN, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
    let mut leap = page(1, route, &PNG[4096..]);
    assert_eq!(caller.turn(TURN, |cx| picker.page(loan(), &mut leap, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Refused);
    let mut foreign_page = page(0, Transport { owner: route.owner + 1, ..route }, &PNG[..4096]);
    assert_eq!(caller.turn(TURN, |cx| picker.page(loan(), &mut foreign_page, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Refused);
    let mut routed = page(0, route, &PNG[..4096]);
    assert!(foreign.turn(TURN, |cx| picker.page(loan(), &mut routed, cx).map(|_| ())).is_err());
    assert!(routed.is_some() && leap.is_some() && foreign_page.is_some());
    assert_eq!(caller.turn(denied, |cx| picker.page(loan(), &mut routed, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Retry);
    assert!(routed.is_some());
    for (index, bytes) in PNG.chunks(4096).enumerate() {
        assert_ne!(bytes[0], RAW_KIND);
        let mut offered = page(index as u32, route, bytes);
        assert_eq!(caller.turn(TURN, |cx| picker.page(loan(), &mut offered, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Pending);
        assert!(offered.is_none());
        let short = RetainedCloneGrant { maximum_copy_bytes: 512, ..TURN };
        assert_eq!(caller.turn(short, |cx| picker.poll(loan(), true, cx).map(|ack| ack.map(|ack| ack.result))).unwrap(), None);
        let mut ack = None;
        for _ in 0..64 {
            ack = caller.turn(TURN, |cx| picker.poll(loan(), true, cx).map(|ack| ack.map(|ack| (ack.phase, ack.result)))).unwrap();
            if ack.is_some() { break; }
        }
        assert_eq!(ack, Some((Phase::Page(index as u32), ResultCode::Accepted)));
        let mut replay = page(index as u32, route, bytes);
        assert_eq!(caller.turn(denied, |cx| picker.page(loan(), &mut replay, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Accepted);
        assert!(replay.is_some());
    }
    assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), loan(), route, Control::Complete, cx).map(|ack| ack.result)).unwrap(), ResultCode::Pending);
    assert_eq!(caller.turn(denied, |cx| picker.control(loan(), loan(), route, Control::Complete, cx).map(|ack| ack.result)).unwrap(), ResultCode::Pending);
    let mut format = None;
    assert!(!caller.turn(denied, |cx| picker.take_format_input(loan(), &mut format, cx)).unwrap());
    assert!(caller.turn(TURN, |cx| picker.take_format_input(loan(), &mut format, cx)).unwrap());
    assert!(!caller.turn(TURN, |cx| picker.take_format_input(loan(), &mut format, cx)).unwrap());
    assert!(picker.format_input().is_none());
    assert_eq!(caller.turn(TURN, |cx| picker.poll(stale, false, cx).map(|ack| ack.map(|ack| ack.result))).unwrap(), None);
    assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), stale, route, Control::Complete, cx).map(|ack| ack.result)).unwrap(), ResultCode::Pending);
    let mut format = format.unwrap();
    caller.turn(TURN, |cx| {
        let input = format.current_input(loan(), cx)?.unwrap();
        assert_eq!(input.target, [4, 3]);
        assert!(input.bytes.original().unwrap().iter().eq(PNG.iter().copied()));
        Ok(())
    }).unwrap();
    assert_eq!(caller.turn(TURN, |cx| picker.poll(loan(), true, cx).map(|ack| ack.map(|ack| ack.result))).unwrap(), None);
    let receipt = caller.turn(TURN, |cx| picker.reserve_format_receipt(cx)).unwrap().unwrap();
    let empty = RetainedCloneGrant { maximum_items: 0, ..TURN };
    assert!(caller.turn(empty, |cx| picker.publish_format_receipt(receipt, FormatResult::Published, cx)).unwrap());
    assert_eq!(caller.turn(TURN, |cx| picker.poll(stale, false, cx).map(|ack| ack.map(|ack| ack.result))).unwrap(), None);
    assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), stale, route, Control::Cancel, cx).map(|ack| ack.result)).unwrap(), ResultCode::Stale);
    while !matches!(caller.turn(TURN, |cx| cx.with_original_retirement(|cx| format.close(cx))).unwrap(),
        semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {}
    let mut ack = None;
    for step in 0..1024 {
        ack = caller.turn(TURN, |cx| picker.poll(stale, true, cx).map(|ack| ack.map(|ack| (ack.phase, ack.result)))).unwrap();
        if ack.is_some() { println!("[DEBUG] Original raw Complete paired close finished at turn {}", step + 1); break; }
    }
    assert_eq!(ack, Some((Phase::Complete, ResultCode::Accepted)));
    assert_eq!(caller.turn(denied, |cx| picker.control(loan(), stale, route, Control::Complete, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
    while !caller.turn(TURN, |cx| cx.with_original_retirement(|cx| picker.close(cx))).unwrap() {}
    while !origin.terminal_is_empty() {
        caller.turn(TURN, |cx| cx.with_original_retirement(|cx| origin.return_step(cx))).unwrap();
    }
    caller.retire(TURN);
    foreign.retire(TURN);
}

#[test]
fn paid_picker_crosses_from_command_work_to_the_reactor_source() {
    use crate::guest::OriginalMediaPickerIntent;
    let clock = || Some(1);
    let mut work = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, clock, 1_000_000, 1 << 24);
    let mut reactor = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, clock, 1_000_000, 1 << 24);
    work.retain_dispatch().unwrap();
    reactor.retain_dispatch().unwrap();
    let mut offered = None;
    let mut picker = Picker::empty();
    assert!(work.turn(TURN, |cx| OriginalMediaPickerIntent::issue(loan(), [4, 3], RAW_KIND, LIMIT, &mut offered, cx)).unwrap());
    assert!(crate::raw_picker_pack::picker_args(&picker).is_none());
    let denied = RetainedCloneGrant { maximum_copy_bytes: 0, ..TURN };
    assert!(!reactor.turn(denied, |cx| picker.admit_issued(&mut offered, loan(), cx)).unwrap());
    assert!(offered.is_some());
    assert!(reactor.turn(TURN, |cx| picker.admit_issued(&mut offered, loan(), cx)).unwrap());
    assert!(offered.is_none() && crate::raw_picker_pack::picker_args(&picker).is_some());
    let route = transport();
    assert!(work.turn(TURN, |cx| picker.control(loan(), loan(), route, BEGIN, cx).map(|_| ())).is_err());
    work.retire(TURN);
    assert_eq!(reactor.turn(TURN, |cx| picker.control(loan(), loan(), route, BEGIN, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
    while !reactor.turn(TURN, |cx| cx.with_original_retirement(|cx| picker.close(cx))).unwrap() {}
    reactor.retire(TURN);
}

#[test]
fn complete_can_cancel_while_the_format_owner_is_open() {
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, || Some(1), 1_000_000, 1 << 24);
    caller.retain_dispatch().unwrap();
    let mut picker = Picker::empty();
    let route = transport();
    assert!(caller.turn(TURN, |cx| picker.issue(loan(), [4, 3], RAW_KIND, LIMIT, cx)).unwrap());
    assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), loan(), route, BEGIN, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
    for (index, bytes) in PNG.chunks(4096).enumerate() {
        let mut offered = page(index as u32, route, bytes);
        assert_eq!(caller.turn(TURN, |cx| picker.page(loan(), &mut offered, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Pending);
        let mut ack = None;
        for _ in 0..64 {
            ack = caller.turn(TURN, |cx| picker.poll(loan(), false, cx).map(|ack| ack.map(|ack| ack.result))).unwrap();
            if ack.is_some() { break; }
        }
        assert_eq!(ack, Some(ResultCode::Accepted));
    }
    assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), loan(), route, Control::Complete, cx).map(|ack| ack.result)).unwrap(), ResultCode::Pending);
    assert!(picker.format_input().is_some());
    assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), loan(), route, Control::Cancel, cx).map(|ack| ack.result)).unwrap(), ResultCode::Pending);
    assert!(picker.cancellation_pending() && picker.format_input().is_none());
    assert!(!caller.turn(TURN, |cx| picker.finish_format(FormatResult::Published, cx)).unwrap());
    assert_eq!(caller.turn(TURN, |cx| picker.poll(loan(), false, cx).map(|ack| ack.map(|ack| ack.result))).unwrap(), None);
    let mut ack = None;
    for step in 0..1024 {
        ack = caller.turn(TURN, |cx| picker.poll(loan(), true, cx).map(|ack| ack.map(|ack| (ack.phase, ack.result)))).unwrap();
        if ack.is_some() { println!("[DEBUG] Original raw Cancel paired close finished at turn {}", step + 1); break; }
    }
    assert_eq!(ack, Some((Phase::Cancel, ResultCode::Accepted)));
    while !caller.turn(TURN, |cx| cx.with_original_retirement(|cx| picker.close(cx))).unwrap() {}
    caller.retire(TURN);
}

#[test]
fn acknowledges_cancel_after_retiring_the_matching_partial_source() {
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, || Some(1), 1_000_000, 1 << 24);
    caller.retain_dispatch().unwrap();
    for before_begin in [true, false] {
        let mut picker = Picker::empty();
        let mut route = transport();
        assert!(caller.turn(TURN, |cx| picker.issue(loan(), [4, 3], RAW_KIND, LIMIT, cx)).unwrap());
        let mut changed_revision = *loan().base_revision;
        changed_revision[31] ^= 1;
        let stale = crate::ingress::OriginalMediaLoan { base_revision: &changed_revision, ..loan() };
        if before_begin {
            route.extent = 0;
        } else {
            assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), loan(), route, BEGIN, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
            let mut offered = page(0, route, &PNG[..4096]);
            assert_eq!(caller.turn(TURN, |cx| picker.page(loan(), &mut offered, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Pending);
            let short = RetainedCloneGrant { maximum_copy_bytes: 512, ..TURN };
            assert_eq!(caller.turn(short, |cx| picker.poll(loan(), true, cx).map(|ack| ack.map(|ack| ack.result))).unwrap(), None);
            assert_eq!(caller.turn(TURN, |cx| picker.poll(stale, true, cx).map(|ack| ack.map(|ack| ack.result))).unwrap(), Some(ResultCode::Stale));
            let mut tail = page(1, route, &PNG[4096..]);
            assert_eq!(caller.turn(TURN, |cx| picker.page(loan(), &mut tail, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Stale);
            assert!(tail.is_some());
        }
        assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), stale, route, Control::Cancel, cx).map(|ack| ack.result)).unwrap(), ResultCode::Pending);
        let denied = RetainedCloneGrant { maximum_release_bytes: 0, ..TURN };
        if !before_begin {
            assert_eq!(caller.turn(denied, |cx| picker.poll(stale, true, cx).map(|ack| ack.map(|ack| ack.result))).unwrap(), None);
        }
        let mut ack = None;
        for _ in 0..64 {
            ack = caller.turn(TURN, |cx| picker.poll(stale, true, cx).map(|ack| ack.map(|ack| (ack.phase, ack.result)))).unwrap();
            if ack.is_some() { break; }
        }
        assert_eq!(ack, Some((Phase::Cancel, ResultCode::Accepted)));
        assert!(picker.format_input().is_none());
        assert_eq!(caller.turn(TURN, |cx| picker.control(loan(), stale, route, Control::Cancel, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
        while !caller.turn(TURN, |cx| cx.with_original_retirement(|cx| picker.close(cx))).unwrap() {}
    }
    caller.retire(TURN);
}
