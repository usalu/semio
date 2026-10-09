//! 🧪️ The sheet export as a whole: print order, file stems, which layouts an export takes, and the two writers over the house with its sheet set.

use super::testkit::{house_with_sheets, inferred, labels};
use super::*;

#[test]
fn sheets_print_in_the_order_of_their_numbers_then_ids() {
    let mut model = ModelSnapshot::default();
    for (id, number) in [("sh-b", "A-102"), ("sh-a", "A-102"), ("sh-c", "A-101"), ("sh-d", "B-001")] {
        model.sheets.insert(id.into(), crate::Sheet::standard(number, "x"));
    }
    assert_eq!(ordered(&model), ["sh-c", "sh-a", "sh-b", "sh-d"]);
}

#[test]
fn a_file_stem_keeps_letters_and_digits_and_joins_the_rest_with_one_dash() {
    assert_eq!(file_stem("A-101"), "A-101");
    assert_eq!(file_stem("  A / 101 .."), "A-101");
    assert_eq!(file_stem("Grundriss EG"), "Grundriss-EG");
    assert_eq!(file_stem("///"), "sheet");
}

#[test]
fn an_export_takes_every_layout_in_print_order_or_the_one_it_names() {
    let model = house_with_sheets();
    let inferred = inferred(&model);
    assert_eq!(layouts(&model, &inferred, None).iter().map(|layout| layout.number.as_str()).collect::<Vec<_>>(), ["A-101", "A-201", "A-301"]);
    assert_eq!(layouts(&model, &inferred, Some("sh-sections")).iter().map(|layout| layout.sheet.as_str()).collect::<Vec<_>>(), ["sh-sections"]);
    assert!(layouts(&model, &inferred, Some("sh-gone")).is_empty());
}

#[test]
fn the_svg_of_a_sheet_exists_for_every_sheet_of_the_model_and_for_no_other() {
    let model = house_with_sheets();
    let inferred = inferred(&model);
    for id in ordered(&model) {
        assert!(sheet_svg(&model, &inferred, &id, &labels()).expect("the sheet has a layout").is_ok(), "{id}");
    }
    assert!(sheet_svg(&model, &inferred, "sh-gone", &labels()).is_none());
}

#[test]
fn the_pdf_of_the_set_has_one_page_per_sheet_and_the_pdf_of_one_sheet_one_page() {
    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::decode_pdf;
    let model = house_with_sheets();
    let inferred = inferred(&model);
    let set = decode_pdf(&sheets_pdf(&model, &inferred, None, &labels()).expect("the set writes")).expect("the set reads back");
    assert_eq!(set.pages.len(), 3);
    let one = decode_pdf(&sheets_pdf(&model, &inferred, Some("sh-sections"), &labels()).expect("one sheet writes")).expect("one sheet reads back");
    assert_eq!(one.pages.len(), 1);
}

#[test]
fn the_dialects_are_the_stdio_ones() {
    assert_eq!((PDF_DIALECT.artifact_kind, PDF_DIALECT.standard.0), ("s.stdio.pdf", "1.7"));
}
