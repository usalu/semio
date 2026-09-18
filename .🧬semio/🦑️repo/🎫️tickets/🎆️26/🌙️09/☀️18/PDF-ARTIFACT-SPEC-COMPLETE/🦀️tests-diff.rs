use super::*;
use crate::standards::v1_7::subsets::base::io::{text_document, PdfTextLayout};
use protocol::DiffCodec;

fn base() -> PdfSnapshot {
    let mut snapshot = text_document(&[(200.0, 300.0, "one"), (200.0, 300.0, "two")]);
    snapshot.images.push(PdfImage::gray8("Im1", 1, 1, vec![7]));
    snapshot.outlines.push(PdfOutlineItem::to_page("Start", 0));
    snapshot.language = Some("en".into());
    snapshot
}

fn edited() -> PdfSnapshot {
    let mut snapshot = base();
    snapshot.pages[0].content.push(PdfOp::Rectangle { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
    snapshot.pages[0].content.push(PdfOp::Fill);
    snapshot.pages[0].rotate = 90;
    snapshot.pages[0].crop_box = Some([1.0, 1.0, 2.0, 2.0]);
    snapshot.pages.remove(1);
    snapshot.pages.push(PdfPage::new(50.0, 50.0));
    snapshot.fonts[0].to_unicode = Some(PdfToUnicode { byte_width: 1, mappings: vec![PdfToUnicodeMapping::Char { code: 65, text: "A".into() }] });
    snapshot.images.push(PdfImage::gray8("Im2", 1, 1, vec![9]));
    snapshot.images.remove(0);
    snapshot.outlines[0].title = "Begin".into();
    snapshot.language = None;
    snapshot.page_mode = Some(PdfPageMode::UseOutlines);
    snapshot.info.title = Some("Edited".into());
    snapshot.catalog_extra.push(PdfDictEntry::new("Marker", PdfObject::Int(1)));
    snapshot
}

#[test]
fn between_apply_and_inverse_are_consistent() {
    let (a, b) = (base(), edited());
    let diff = PdfDiff::between(&a, &b);
    assert!(!diff.is_empty());
    assert_eq!(diff.apply(&a).unwrap(), b);
    let inverse = diff.inverse(&a);
    assert_eq!(inverse.apply(&b).unwrap(), a);
    assert!(PdfDiff::between(&a, &a).is_empty());
    let content = diff.pages.as_ref().unwrap().modified[0].diff.content.as_ref().unwrap();
    assert_eq!(content.added.len(), 2, "content is patched by operator, not re-sent: {content:?}");
}

#[test]
fn absorb_composes_sequential_diffs() {
    let a = base();
    let b = edited();
    let mut c = b.clone();
    c.pages[0].content.insert(0, PdfOp::Save);
    c.pages[0].content.push(PdfOp::Restore);
    c.pages.insert(0, PdfPage::new(20.0, 20.0));
    c.images.push(PdfImage::gray8("Im3", 1, 1, vec![1]));
    c.images[0].interpolate = true;
    c.outlines.push(PdfOutlineItem::to_page("End", 1));
    c.language = Some("de".into());
    let mut first = PdfDiff::between(&a, &b);
    let second = PdfDiff::between(&b, &c);
    first.absorb(second);
    assert_eq!(first.apply(&a).unwrap(), c);
}

#[test]
fn codecs_round_trip_every_lane() {
    let diff = PdfDiff::between(&base(), &edited());
    let line = diff.print_diff();
    assert!(!line.trim_end().contains('\n'), "one record line");
    assert_eq!(PdfDiff::parse_diff(&line).unwrap(), diff);
    let bytes = diff.encode_diff().unwrap();
    assert_eq!(bytes[0], store::pack_rt::OP_BINARY_FORMAT);
    assert_eq!(PdfDiff::decode_diff(&bytes).unwrap(), diff);
    assert_eq!(diff.encode_diff().unwrap(), bytes, "binary encoding is deterministic");
    let empty = PdfDiff::default();
    assert_eq!(PdfDiff::parse_diff(&empty.print_diff()).unwrap(), empty);
}

#[test]
fn builders_produce_sparse_patches() {
    let a = base();
    let layout = PdfTextLayout::new(&a.fonts[0], 10.0);
    let ops = layout.show_lines(&["more".to_string()], 10.0, 100.0);
    let diff = diff_append_page_content(&a, 0, &ops);
    let next = diff.apply(&a).unwrap();
    assert_eq!(next.pages[0].content.len(), a.pages[0].content.len() + ops.len());
    let removed = diff_remove_content(0, 0, 2).apply(&next).unwrap();
    assert_eq!(removed.pages[0].content.len(), next.pages[0].content.len() - 2);
    let font = diff_set_font(&a, PdfFont::standard("F2", "Courier")).apply(&a).unwrap();
    assert_eq!(font.fonts.len(), 2);
    assert!(diff_set_font(&a, a.fonts[0].clone()).is_empty());
    let without = diff_remove_font(&font, "F2").apply(&font).unwrap();
    assert_eq!(without.fonts, a.fonts);
    let boxed = diff_set_page_box(0, PdfPageBox::Trim, Some([0.0, 0.0, 5.0, 5.0])).apply(&a).unwrap();
    assert_eq!(boxed.pages[0].trim_box, Some([0.0, 0.0, 5.0, 5.0]));
    let annotated = diff_insert_annotation(0, 0, PdfAnnotation::link([0.0, 0.0, 1.0, 1.0], "x")).apply(&a).unwrap();
    assert_eq!(annotated.pages[0].annotations.len(), 1);
    let mode = diff_set_page_mode(&a, Some(PdfPageMode::FullScreen)).apply(&a).unwrap();
    assert_eq!(mode.page_mode, Some(PdfPageMode::FullScreen));
    assert!(diff_set_page_mode(&mode, Some(PdfPageMode::FullScreen)).is_empty());
    let cleared = diff_set_language(&a, None).apply(&a).unwrap();
    assert_eq!(cleared.language, None);
}

#[test]
fn validation_rejects_out_of_range_targets() {
    let a = base();
    assert!(diff_remove_page(7).apply(&a).is_err());
    assert!(diff_remove_content(0, 99, 1).apply(&a).is_err());
    assert!(diff_remove_font(&a, "nope").is_empty());
    let mut duplicate = PdfDiff::default();
    duplicate.fonts = Some(PdfKeyedDiff { added: vec![PdfIndexedItem { index: 0, value: a.fonts[0].clone() }], ..Default::default() });
    assert!(duplicate.apply(&a).is_err(), "adding an existing key is rejected");
}

