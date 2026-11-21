//! 🧪 Exercises the exact paid Store pages with the authored large PNG and a cross-word resource namespace.

use crate::authoring::edit::PngGraphEdit;
use semio_s_artifact_stdio_pdf_editor_document::ImagePlacement;
use crate::standards::v1_7::subsets::base::schema::{
    mutations::PdfMutation,
    snapshot::{ObjRef, PdfImage, PdfIndirectObject, PdfObject, PdfOp, PdfPage, PdfSnapshot},
};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::{RetainedCloneGrant, paged::ByteReadSource, retirement::controlled::ControlledRetirement};
use semio_plugin::__job::{Generation, OperationId, close_original_effect_step};

const REVISION: [u8; 32] = [51; 32];
const INPUT: &[u8] = include_bytes!("input.png");
const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 8, maximum_copy_bytes: 64 << 10, maximum_capacity_bytes: 64 << 10,
    maximum_release_bytes: 64 << 10, maximum_depth: 64,
};
const STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 24,
    maximum_release_bytes: 1 << 24, ..TURN
};

struct Input;
impl ByteReadSource for Input {
    fn byte_len(&self) -> usize { INPUT.len() }
    fn byte_at(&self, at: usize) -> Option<u8> { INPUT.get(at).copied() }
    fn read_bytes(&self, at: usize, output: &mut [u8]) -> usize {
        let Some(rest) = INPUT.get(at..) else { return 0 };
        let count = output.len().min(rest.len());
        output[..count].copy_from_slice(&rest[..count]);
        count
    }
}

#[test]
fn paid_graph_and_placement_close_after_donating_every_exact_row() {
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(3), Generation(1), STOCK, || Some(1), 1_000_000, 1 << 27);
    caller.retain_dispatch().unwrap();
    let mut base = PdfSnapshot::default();
    base.pages.push(PdfPage::new(150., 100.));
    base.objects.push(PdfIndirectObject { id: ObjRef { num: 90, gen: 2 }, value: PdfObject::Null });
    for name in (1..66).map(|index| format!("SemioImage{index}")).chain(["SemioImage+66".into(), "SemioImage066".into()]) {
        base.images.push(PdfImage::rgb8(name, 1, 1, vec![0; 3]));
    }
    let input = Input;
    let placement = ImagePlacement::Insert { page: 0, matrix: [40., 0., 0., 40., 20., 10.] };
    let mut slot = None;
    let without_capacity = RetainedCloneGrant { maximum_capacity_bytes: 0, ..TURN };
    assert!(!caller.bare_turn(without_capacity, |cx| PngGraphEdit::admit(&input, REVISION, placement, (521, 257), true, &mut slot, cx)).unwrap());
    assert!(slot.is_none());
    assert!(caller.bare_turn(TURN, |cx| PngGraphEdit::admit(&input, REVISION, placement, (521, 257), true, &mut slot, cx)).unwrap());
    let work = slot.as_mut().unwrap();
    assert!(caller.bare_turn(TURN, |cx| work.advance(&input, &base, &[50; 32], cx)).is_err());
    assert!((0..2000).any(|_| caller.bare_turn(TURN, |cx| work.advance(&input, &base, &REVISION, cx)).unwrap()));
    let mut rows = None;
    assert!(!caller.bare_turn(TURN, |cx| work.take_pages(&[50; 32], &mut rows, cx)).unwrap());
    assert!(caller.bare_turn(TURN, |cx| work.take_pages(&REVISION, &mut rows, cx)).unwrap());
    let rows = rows.unwrap();
    assert_eq!(rows.len(), 18);
    let mut objects = Vec::new();
    for (index, row) in rows.iter().take(15).enumerate() {
        let PdfMutation::InsertObject(row) = row else { panic!("graph row {index}"); };
        assert_eq!((row.id.num, row.id.gen), (91 + index as u32, 0));
        objects.push(PdfIndirectObject { id: row.id, value: row.value.clone() });
    }
    let mut hash = semio_framework_hash::Sha256::new();
    hash.update(INPUT);
    let reference = crate::native::png_source::PngPlane::Alpha.reference(hash.finalize());
    let (_, original) = crate::native::png_source::PngGraphSource::resolve(&objects, &reference).unwrap();
    let mut actual = vec![0; INPUT.len()];
    assert_eq!(original.read_bytes(0, &mut actual), INPUT.len());
    assert_eq!(actual, INPUT);
    let mut names = rows.iter().skip(15);
    let (Some(PdfMutation::SetImage(mask)), Some(PdfMutation::SetImage(color)), Some(PdfMutation::AppendPageContent(paint))) =
        (names.next(), names.next(), names.next()) else { panic!("mask, color and paint are in the original edit"); };
    assert_eq!(mask.image.id, "SemioImage67");
    assert_eq!(color.image.id, "SemioImage66");
    assert_eq!(color.image.soft_mask.as_deref(), Some("SemioImage67"));
    assert!(matches!(&paint.content[2], PdfOp::PaintXObject { name } if name == "SemioImage66"));
    drop(names);
    let mut retired = ControlledRetirement::new(Some(rows)).ok().unwrap();
    assert!((0..2_000).any(|_| {
        caller.bare_turn(TURN, |cx| cx.with_original_retirement(|cx| {
            if !work.terminal_is_empty() { return work.close(cx); }
            let body = 64 << 10;
            close_original_effect_step(cx, retired.next_demand(body)?, |effect| retired.step(body, effect))?;
            Ok(())
        })).unwrap();
        work.terminal_is_empty() && retired.terminal_is_empty()
    }));
    base.objects.extend(objects);
    let next_revision = [52; 32];
    let mut reuse = None;
    assert!(caller.bare_turn(TURN, |cx| {
        PngGraphEdit::admit(&input, next_revision, placement, (521, 257), true, &mut reuse, cx)
    }).unwrap());
    let reuse = reuse.as_mut().unwrap();
    assert!((0..2_000).any(|_| caller.bare_turn(TURN, |cx| reuse.advance(&input, &base, &next_revision, cx)).unwrap()));
    let mut placed = None;
    assert!(caller.bare_turn(TURN, |cx| reuse.take_pages(&next_revision, &mut placed, cx)).unwrap());
    let placed = placed.unwrap();
    assert_eq!(placed.len(), 3);
    let mut rows = placed.iter();
    let (Some(PdfMutation::SetImage(mask)), Some(PdfMutation::SetImage(_)), Some(PdfMutation::AppendPageContent(_))) =
        (rows.next(), rows.next(), rows.next()) else { panic!("the reused source adds only mask, color and paint"); };
    assert!(matches!(&mask.image.body, crate::standards::v1_7::subsets::base::schema::snapshot::PdfImageBody::Artifact { reference: actual } if actual == &reference));
    drop(rows);
    let mut placed = ControlledRetirement::new(Some(placed)).ok().unwrap();
    assert!((0..2_000).any(|_| {
        caller.bare_turn(TURN, |cx| cx.with_original_retirement(|cx| {
            if !reuse.terminal_is_empty() { return reuse.close(cx); }
            close_original_effect_step(cx, placed.next_demand(64 << 10)?, |effect| placed.step(64 << 10, effect))?;
            Ok(())
        })).unwrap();
        reuse.terminal_is_empty() && placed.terminal_is_empty()
    }));
    let _ = caller.retire(TURN);
}
