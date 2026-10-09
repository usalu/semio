//! 🧪️ The title block and the revision table: cells tile the block without gaps, empty fields fall back to what the sheet shows, the revision table grows upward from the block.

use super::*;
use crate::SheetRevision;

fn frame() -> PaperRect {
    PaperRect { x: 20.0, y: 10.0, width: 390.0, height: 277.0 }
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a} != {b}");
}

fn revision(mark: &str) -> SheetRevision {
    SheetRevision { sheet: "sh-1".into(), number: mark.into(), date: "2026-10-01".into(), description: "Issued".into(), author: "UG".into() }
}

#[test]
fn the_title_block_sits_in_the_bottom_right_corner_of_the_frame() {
    let block = title_block(&frame(), &Sheet::standard("A-101", "Ground floor"), &[100], None);
    near(block.rect.right(), frame().right());
    near(block.rect.bottom(), frame().bottom());
    near(block.rect.width, TITLE_BLOCK_WIDTH);
    near(block.rect.height, 3.0 * ROW_HEIGHT);
}

#[test]
fn the_cells_tile_the_block_without_gaps_or_overlap() {
    let block = title_block(&frame(), &Sheet::standard("A-101", "Ground floor"), &[100], None);
    assert_eq!(block.cells.iter().map(|cell| cell.field).collect::<Vec<_>>(), [TitleField::Project, TitleField::Number, TitleField::Title, TitleField::Scale, TitleField::DrawnBy, TitleField::CheckedBy, TitleField::Date, TitleField::Revision]);
    near(block.cells.iter().map(|cell| cell.rect.width * cell.rect.height).sum::<f64>(), block.rect.width * block.rect.height);
    for (index, first) in block.cells.iter().enumerate() {
        assert!(block.rect.contains(&first.rect));
        for second in &block.cells[index + 1..] {
            assert!(!first.rect.overlaps(&second.rect), "{:?} / {:?}", first.field, second.field);
        }
    }
}

#[test]
fn a_narrow_frame_shrinks_the_block_to_its_width_and_a_low_one_to_its_height() {
    let narrow = PaperRect { x: 20.0, y: 10.0, width: 100.0, height: 30.0 };
    let block = title_block(&narrow, &Sheet::standard("A-101", "Plan"), &[], None);
    near(block.rect.width, 100.0);
    near(block.rect.height, 30.0);
    assert!(narrow.contains(&block.rect));
}

#[test]
fn the_cells_print_the_authored_fields_and_fall_back_to_what_the_sheet_shows() {
    let mut sheet = Sheet::standard("A-101", "Ground floor");
    sheet.project = "House".into();
    sheet.drawn_by = "UG".into();
    sheet.checked_by = "AB".into();
    sheet.date = "2026-10-09".into();
    let value = |block: &TitleBlock, field: TitleField| block.cells.iter().find(|cell| cell.field == field).map(|cell| cell.value.clone()).unwrap_or_default();
    let derived = title_block(&frame(), &sheet, &[100, 50, 100], Some("B"));
    assert_eq!(
        [TitleField::Project, TitleField::Number, TitleField::Title, TitleField::Scale, TitleField::DrawnBy, TitleField::CheckedBy, TitleField::Date, TitleField::Revision].map(|field| value(&derived, field)),
        ["House", "A-101", "Ground floor", "1:50, 1:100", "UG", "AB", "2026-10-09", "B"]
    );
    sheet.scale_label = "As indicated".into();
    sheet.revision = "C".into();
    let authored = title_block(&frame(), &sheet, &[100], Some("B"));
    assert_eq!((value(&authored, TitleField::Scale).as_str(), value(&authored, TitleField::Revision).as_str()), ("As indicated", "C"));
    assert_eq!(value(&title_block(&frame(), &Sheet::standard("A-1", "x"), &[], None), TitleField::Scale), "");
}

#[test]
fn scales_are_listed_once_from_the_largest_scale_to_the_smallest() {
    assert_eq!(scale_text(&[100, 50, 100, 200]), "1:50, 1:100, 1:200");
    assert_eq!(scale_text(&[]), "");
}

#[test]
fn the_revision_table_grows_upward_from_the_title_block_one_row_per_revision() {
    let block = title_block(&frame(), &Sheet::standard("A-101", "Ground floor"), &[], None);
    let (first, second) = (revision("A"), revision("B"));
    let (a, b) = ("rev-1".to_string(), "rev-2".to_string());
    let table = revision_table(&block.rect, &[(&a, &first), (&b, &second)]);
    near(table.rect.height, 3.0 * REVISION_ROW_HEIGHT);
    near(table.rect.bottom(), block.rect.y);
    near(table.rect.width, block.rect.width);
    assert_eq!(table.rows.iter().map(|row| (row.revision.as_str(), row.mark.as_str())).collect::<Vec<_>>(), [("rev-1", "A"), ("rev-2", "B")]);
    near(table.rows[0].rect.y, table.rect.y + REVISION_ROW_HEIGHT);
    near(table.rows[1].rect.bottom(), table.rect.bottom());
    assert!(!table.rect.overlaps(&block.rect));
}

#[test]
fn a_sheet_without_revisions_has_an_empty_table_of_no_height() {
    let block = title_block(&frame(), &Sheet::standard("A-101", "Ground floor"), &[], None);
    let table = revision_table(&block.rect, &[]);
    assert_eq!((table.rect.height, table.rows.len()), (0.0, 0));
}

#[test]
fn the_column_edges_split_the_table_in_the_shares_of_its_columns() {
    let edges = column_edges(180.0);
    assert_eq!(edges, [0.0, 14.0, 40.0, 150.0, 180.0]);
    let half = column_edges(90.0);
    near(half[2], 20.0);
    near(half[4], 90.0);
}
