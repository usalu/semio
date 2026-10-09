//! 🧪️ The SVG of a sheet read back by a third-party XML reader (quick-xml): paper size and viewBox, one clipped group per viewport holding the primitives of its view, the title block and the revision table as text.

use super::super::testkit::{house_with_sheets, inferred, labels, texts};
use super::super::*;
use crate::standards::v1::subsets::any::io::export::svg::testkit::{tags, Tag};

fn svg_of(sheet: &str) -> (String, crate::ModelInference) {
    let model = house_with_sheets();
    let inferred = inferred(&model);
    (sheet_svg(&model, &inferred, sheet, &labels()).expect("a layout").expect("a document"), inferred)
}

fn viewports(all: &[Tag]) -> Vec<&Tag> {
    all.iter().filter(|tag| tag.name == "g" && tag.has_class("viewport")).collect()
}

#[test]
fn the_document_is_the_paper_in_millimetres() {
    let (document, _) = svg_of("sh-plans");
    let all = tags(&document);
    let root = &all[0];
    assert_eq!(root.name, "svg");
    assert_eq!((root.attribute("width"), root.attribute("height"), root.attribute("viewBox")), (Some("420mm"), Some("297mm"), Some("0 0 420 297")));
    assert_eq!((root.attribute("data-sheet"), root.attribute("data-number"), root.attribute("data-paper"), root.attribute("data-unit")), (Some("sh-plans"), Some("A-101"), Some("A3"), Some("mm")));
    let portrait = svg_of("sh-sections").0;
    let root = tags(&portrait).remove(0);
    assert_eq!((root.attribute("width"), root.attribute("viewBox")), (Some("297mm"), Some("0 0 297 420")));
}

#[test]
fn every_viewport_is_one_group_clipped_to_a_window_that_exists() {
    let (document, _) = svg_of("sh-plans");
    let all = tags(&document);
    let groups = viewports(&all);
    assert_eq!(groups.len(), 2);
    for group in groups {
        let clip = group.attribute("clip-path").expect("a clip path").trim_start_matches("url(#").trim_end_matches(')').to_string();
        let target = all.iter().find(|tag| tag.name == "clipPath" && tag.attribute("id") == Some(clip.as_str()));
        assert!(target.is_some(), "{clip}");
        assert_eq!(group.attribute("data-scale"), Some("100"));
    }
    let windows: Vec<&str> = viewports(&all).iter().filter_map(|group| group.attribute("data-window")).collect();
    assert_eq!(windows.len(), 2);
}

#[test]
fn a_viewport_holds_the_primitives_of_its_view() {
    let (document, inferred) = svg_of("sh-plans");
    let all = tags(&document);
    let regions = all.iter().filter(|tag| tag.name == "path" && tag.has_class("region")).count();
    let lines = all.iter().filter(|tag| tag.name == "path" && tag.has_class("line")).count();
    let expected = |field: fn(&crate::standards::v1::subsets::any::schema::inferences::view_linework::ViewLinework) -> usize| ["v-plan-st-ground", "v-plan-st-first"].iter().map(|view| field(&inferred.view_linework[*view])).sum::<usize>();
    assert_eq!(regions, expected(|drawing| drawing.lines.regions.len()));
    assert_eq!(lines, expected(|drawing| drawing.lines.polylines.len()));
    assert!(regions > 0 && lines > 0);
}

#[test]
fn the_title_block_prints_the_authored_fields_and_the_headings() {
    let (document, _) = svg_of("sh-plans");
    let printed = texts(&document);
    for expected in ["A-101", "Floor plans", "House on the hill", "UG", "AB", "2026-10-09", "1:100", "Sheet no.", "Drawn by", "Revision", "Plan Ground   1:100", "Plan First   1:100"] {
        assert!(printed.iter().any(|text| text == expected), "{expected}");
    }
}

#[test]
fn the_revision_table_prints_one_row_per_revision_and_only_where_there_are_some() {
    let (plans, _) = svg_of("sh-plans");
    let (sections, _) = svg_of("sh-sections");
    let printed = texts(&plans);
    for expected in ["Issued for permit", "Window sizes", "2026-10-05"] {
        assert!(printed.iter().any(|text| text == expected), "{expected}");
    }
    assert!(plans.contains("class=\"revision-table\""));
    assert!(!sections.contains("revision-table"));
}

#[test]
fn a_viewport_group_names_its_view_and_carries_an_accessible_label() {
    let (document, _) = svg_of("sh-plans");
    let all = tags(&document);
    let ground = viewports(&all).into_iter().find(|group| group.attribute("data-viewport") == Some("vp-ground")).expect("the ground plan");
    assert_eq!((ground.attribute("data-view"), ground.attribute("data-label"), ground.attribute("aria-label"), ground.attribute("data-kind")), (Some("v-plan-st-ground"), Some("Plan Ground"), Some("Plan Ground"), Some("plan")));
}
