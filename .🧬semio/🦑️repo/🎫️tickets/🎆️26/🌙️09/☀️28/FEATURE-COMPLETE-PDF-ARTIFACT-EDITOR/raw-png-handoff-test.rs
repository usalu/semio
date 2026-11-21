//! 🧪 Drives the exact fixed picker input into real pixel reconstruction under the retained caller.

use crate::{STOCK, TURN, ingress::OriginalMediaLoan, raw_picker::{Control, Phase, RawPicker, ResultCode, Transport},
    raw_png_work::RawPngWork};
use semio_framework_job::{Generation, InteractiveJobCloseStep, OperationId};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_pixels_codecs::png_decoding::PngDecodeLimits;
use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage};
use semio_framework_value::RetainedCloneGrant;
use std::path::Path;

#[path = "🗑️generated/paged-media-receiver/browser-law.rs"]
mod contract;
use contract::{LIMIT, LOAN, RAW_KIND, SEQ};

const SIZES: PngDecodeLimits = PngDecodeLimits { max_pixels: 4096, max_bytes: LIMIT, max_chunks: 1024 };
const TARGET: [u32; 2] = [4, 3];
type Picker = RawPicker<[u32; 2], LIMIT>;
type Work = RawPngWork<[u32; 2], LIMIT>;

fn caller() -> OriginalJobFixtureCaller {
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, || Some(1), 1_000_000, 1 << 24);
    caller.retain_dispatch().unwrap();
    caller
}

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(file!()).parent().unwrap().join("🗑️generated/raw-png").join(name)
}

fn complete(caller: &mut OriginalJobFixtureCaller, png: &[u8]) -> Picker {
    let mut picker = Picker::empty();
    let route = Transport { owner: LOAN.instance_id.into(), seq: SEQ, kind: RAW_KIND, extent: png.len() as u32 };
    assert!(caller.turn(TURN, |cx| picker.issue(LOAN, TARGET, RAW_KIND, LIMIT, cx)).unwrap());
    let start = Control::Begin { name: "raw.png", media_type: "image/png" };
    assert_eq!(caller.turn(TURN, |cx| picker.control(LOAN, LOAN, route, start, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
    for (index, body) in png.chunks(4096).enumerate() {
        let cursor = CommandPageCursor {
            owner: route.owner, seq: route.seq, kind: route.kind, generation: LOAN.generation,
            instance: LOAN.instance_id, command_index: 0, command_count: 1, item_count: 1,
            metadata: route.extent, page_index: index as u32, page_count: route.extent.div_ceil(4096),
        };
        let mut page = Some((cursor, FixedCommandPage::try_copy_from(body).unwrap()));
        assert_eq!(caller.turn(TURN, |cx| picker.page(LOAN, &mut page, cx).map(|ack| ack.unwrap().result)).unwrap(), ResultCode::Pending);
        let ack = (0..64).find_map(|_| caller.turn(TURN, |cx| picker.poll(LOAN, cx).map(|ack| ack.map(|ack| (ack.phase, ack.result)))).unwrap());
        assert_eq!(ack, Some((Phase::Page(index as u32), ResultCode::Accepted)));
    }
    assert_eq!(caller.turn(TURN, |cx| picker.control(LOAN, LOAN, route, Control::Complete, cx).map(|ack| ack.result)).unwrap(), ResultCode::Accepted);
    picker
}

fn close(caller: &mut OriginalJobFixtureCaller, work: &mut Work, picker: &mut Picker) {
    let released_before = caller.cumulative_receipt().released_bytes;
    let mut done = false;
    for index in 0..4096 {
        done = caller.turn(TURN, |cx| cx.with_original_retirement(|cx| work.close(cx)
            .map(|turn| matches!(turn, InteractiveJobCloseStep::Complete { .. })))).unwrap();
        if done { println!("[DEBUG] drained at turn {index}"); break; }
    }
    assert!(done, "decoder close reached terminal");
    assert!(caller.cumulative_receipt().released_bytes > released_before);
    assert!((0..128).any(|_| caller.turn(TURN, |cx| cx.with_original_retirement(|cx| picker.close(cx))).unwrap()));
}

#[test]
fn exact_fixed_pages_reconstruct_the_pillow_profiles() {
    for name in ["rgba", "palette", "interlacedRgba", "splitFraming"] {
        let source = std::fs::read(fixture(&format!("{name}.png"))).unwrap();
        let oracle = std::fs::read(fixture(&format!("{name}.rgba"))).unwrap();
        let mut caller = caller();
        let mut picker = complete(&mut caller, &source);
        let input = picker.format_input().unwrap();
        let mut work = Work::empty();
        let short = RetainedCloneGrant { maximum_copy_bytes: Work::birth_demand().copy_bytes - 1, ..TURN };
        assert!(caller.turn(short, |cx| work.advance(input, LOAN, SIZES, cx)).unwrap().is_none());
        caller.turn(TURN, |cx| {
            let original = &input.current_input(LOAN, cx)?.unwrap().bytes;
            assert!(original.original().unwrap().iter().eq(source.iter().copied()));
            Ok(())
        }).unwrap();
        let progress = caller.turn(TURN, |cx| work.advance(input, LOAN, SIZES, cx)).unwrap().unwrap();
        assert_eq!(progress.total_bytes, source.len());
        assert_eq!(caller.receipt().retained_capacity_bytes, 0);
        caller.turn(TURN, |cx| {
            assert!(input.current_input(LOAN, cx)?.unwrap().bytes.original().is_none());
            assert!(work.raster(input, LOAN, cx)?.is_none());
            assert!(work.source(input, LOAN, cx)?.is_none());
            Ok(())
        }).unwrap();
        let mut done = false;
        for _ in 0..4096 {
            let progress = caller.turn(TURN, |cx| work.advance(input, LOAN, SIZES, cx)).unwrap().unwrap();
            done = progress.done;
            if done { break; }
        }
        assert!(done, "{name} completed");
        let (width, height, actual) = caller.turn(TURN, |cx| {
            assert!(work.source(input, LOAN, cx)?.unwrap().iter().eq(source.iter().copied()));
            let (target, raster) = work.raster(input, LOAN, cx)?.unwrap();
            assert_eq!(target, TARGET);
            Ok((raster.width, raster.height, raster.pixels.clone()))
        }).unwrap();
        assert_eq!(actual, oracle);
        std::fs::write(fixture(&format!("{name}.native")), &actual).unwrap();
        println!("[DEBUG] {name}: {width}×{height}, {} exact RGBA bytes", actual.len());
        close(&mut caller, &mut work, &mut picker);
        caller.retire(TURN);
    }
}

#[test]
fn same_operation_on_different_caller_and_stale_revision_cannot_publish() {
    let mut rightful = caller();
    let mut foreign = caller();
    let source = std::fs::read(fixture("rgba.png")).unwrap();
    let mut picker = complete(&mut rightful, &source);
    let input = picker.format_input().unwrap();
    let mut work = Work::empty();
    rightful.turn(TURN, |cx| work.advance(input, LOAN, SIZES, cx)).unwrap();
    assert!(foreign.turn(TURN, |cx| work.advance(input, LOAN, SIZES, cx)).is_err());
    let short = RetainedCloneGrant { maximum_items: 1, ..TURN };
    rightful.turn(short, |cx| work.advance(input, LOAN, SIZES, cx)).unwrap();
    let mut revision = *LOAN.base_revision;
    revision[31] ^= 1;
    let stale = OriginalMediaLoan { base_revision: &revision, ..LOAN };
    assert!(rightful.turn(TURN, |cx| work.advance(input, stale, SIZES, cx)).is_err());
    assert!(rightful.turn(TURN, |cx| work.raster(input, LOAN, cx).map(|result| result.is_none())).unwrap());
    close(&mut rightful, &mut work, &mut picker);
    foreign.retire(TURN);
    rightful.retire(TURN);
}

#[test]
fn cancellation_and_crc_refusal_leave_no_raster() {
    for (name, cancel) in [("rgba", true), ("alteredChecksum", false)] {
        let mut caller = caller();
        let source = std::fs::read(fixture(&format!("{name}.png"))).unwrap();
        let mut picker = complete(&mut caller, &source);
        let input = picker.format_input().unwrap();
        let mut work = Work::empty();
        caller.turn(TURN, |cx| work.advance(input, LOAN, SIZES, cx)).unwrap();
        if cancel {
            let pixels_started = (0..1024).any(|_| {
                let progress = caller.turn(TURN, |cx| work.advance(input, LOAN, SIZES, cx)).unwrap().unwrap();
                progress.pixels > 0 && !progress.done
            });
            assert!(pixels_started, "cancel during partially decoded samples");
            caller.cancel().cancel_now();
        }
        let mut refused = false;
        for _ in 0..4096 {
            if caller.turn(TURN, |cx| work.advance(input, LOAN, SIZES, cx)).is_err() {
                refused = true;
                break;
            }
        }
        assert!(refused);
        assert!(caller.turn(TURN, |cx| work.raster(input, LOAN, cx).map(|result| result.is_none())).unwrap());
        close(&mut caller, &mut work, &mut picker);
        caller.retire(TURN);
    }
}
