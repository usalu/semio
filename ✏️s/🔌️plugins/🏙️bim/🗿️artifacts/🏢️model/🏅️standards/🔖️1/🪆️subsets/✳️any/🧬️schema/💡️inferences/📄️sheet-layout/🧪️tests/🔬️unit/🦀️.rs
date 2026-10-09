//! 🧪️ The layout of a sheet: where the frame, the windows, the title block and the revision table go, which findings it raises and what its dependency reads.

use super::*;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::{PlanBounds, PlanKind, PlanLinework, PlanPolyline, PlanStyle, PlanVertex};
use crate::{DetailLevel, IsoSize, Orientation, Paper, Point2, Sheet, SheetRevision, View, Viewport};

fn drawing(view: &str, bounds: (f64, f64, f64, f64)) -> ViewLinework {
    let stroke = PlanPolyline { id: "l-1".into(), element: "w-1".into(), kind: PlanKind::WallOutline, style: PlanStyle::Cut, closed: false, vertices: vec![PlanVertex { x: bounds.0, y: bounds.1, bulge: 0.0 }, PlanVertex { x: bounds.2, y: bounds.3, bulge: 0.0 }] };
    let lines = PlanLinework { storey: "st-ground".into(), cut_height: 1.2, cut_elevation: 1.2, regions: Vec::new(), polylines: vec![stroke], texts: Vec::new(), bounds: PlanBounds { min_x: bounds.0, min_y: bounds.1, max_x: bounds.2, max_y: bounds.3 } };
    ViewLinework { view: view.into(), kind: ViewKind::Plan, scale: 100, detail: DetailLevel::Medium, lines }
}

fn model() -> ModelSnapshot {
    let mut model = ModelSnapshot::default();
    model.views.insert("v-ground".into(), View::of_storey("bldg-1", "Ground plan", ViewKind::Plan, "st-ground"));
    model.views.insert("v-first".into(), View::of_storey("bldg-1", "First plan", ViewKind::Plan, "st-first"));
    model.sheets.insert("sh-1".into(), Sheet::standard("A-101", "Ground floor"));
    model.sheets.insert("sh-2".into(), Sheet::standard("A-102", "First floor"));
    model.viewports.insert("vp-1".into(), Viewport::standard("sh-1", "v-ground", Point2 { x: 30.0, y: 40.0 }));
    model
}

fn drawings(model: &ModelSnapshot) -> BTreeMap<String, ViewLinework> {
    model.views.keys().map(|view| (view.clone(), drawing(view, (0.0, 0.0, 8.0, 6.0)))).collect()
}

fn layout(model: &ModelSnapshot, id: &str) -> SheetLayout {
    let all = drawings(model);
    layout_of(model, id, &all.iter().map(|(view, drawing)| (view.as_str(), drawing)).collect())
}

fn issues(layout: &SheetLayout) -> Vec<(SheetIssue, Vec<&str>)> {
    layout.findings.iter().map(|finding| (finding.issue, finding.viewports.iter().map(String::as_str).collect())).collect()
}

#[test]
fn a_sheet_is_laid_out_on_its_paper_with_the_frame_and_the_name_of_its_paper() {
    let layout = layout(&model(), "sh-1");
    assert_eq!((layout.sheet.as_str(), layout.number.as_str(), layout.name.as_str(), layout.paper.as_str()), ("sh-1", "A-101", "Ground floor", "A3"));
    assert_eq!((layout.width, layout.height), (420.0, 297.0));
    assert_eq!(layout.frame, PaperRect { x: 20.0, y: 10.0, width: 390.0, height: 277.0 });
    assert_eq!(layout.title_block.rect, PaperRect { x: 230.0, y: 239.0, width: 180.0, height: 48.0 });
    assert_eq!(layout.revisions.rect.height, 0.0);
}

#[test]
fn the_paper_follows_the_size_and_the_orientation_of_the_sheet() {
    let mut model = model();
    let sheet = model.sheets.get_mut("sh-1").unwrap();
    sheet.paper = Paper::iso(IsoSize::A4);
    sheet.orientation = Orientation::Portrait;
    let layout = layout(&model, "sh-1");
    assert_eq!((layout.width, layout.height, layout.paper.as_str()), (210.0, 297.0, "A4"));
    assert_eq!(layout.frame, PaperRect { x: 20.0, y: 10.0, width: 180.0, height: 277.0 });
    assert_eq!(layout.title_block.rect.width, 180.0);
}

#[test]
fn a_viewport_window_is_the_drawing_at_the_scale_of_the_viewport_inside_the_frame() {
    let layout = layout(&model(), "sh-1");
    let placed = &layout.viewports[0];
    assert_eq!(placed.window, PaperRect { x: 30.0, y: 40.0, width: 90.0, height: 70.0 });
    assert_eq!((placed.view.as_str(), placed.label.as_str(), placed.scale, placed.map.mm), ("v-ground", "Ground plan", 100, 10.0));
    assert!(layout.findings.is_empty(), "{:?}", layout.findings);
}

#[test]
fn the_title_block_prints_the_scales_of_the_viewports_and_the_last_revision() {
    let mut model = model();
    model.viewports.insert("vp-2".into(), Viewport { scale: 50, ..Viewport::standard("sh-1", "v-first", Point2 { x: 130.0, y: 40.0 }) });
    model.sheet_revisions.insert("r-1".into(), SheetRevision { sheet: "sh-1".into(), number: "A".into(), date: "2026-10-01".into(), description: "Permit".into(), author: "UG".into() });
    model.sheet_revisions.insert("r-2".into(), SheetRevision { sheet: "sh-1".into(), number: "B".into(), date: "2026-10-05".into(), description: "Windows".into(), author: "AB".into() });
    let layout = layout(&model, "sh-1");
    let value = |field| layout.title_block.cells.iter().find(|cell| cell.field == field).map(|cell| cell.value.clone()).unwrap_or_default();
    assert_eq!((value(TitleField::Scale).as_str(), value(TitleField::Revision).as_str()), ("1:50, 1:100", "B"));
    assert_eq!(layout.revisions.rows.iter().map(|row| row.mark.as_str()).collect::<Vec<_>>(), ["A", "B"]);
    assert_eq!(layout.revisions.rect.height, 18.0);
    assert_eq!(layout.revisions.rect.bottom(), layout.title_block.rect.y);
}

#[test]
fn a_window_beyond_the_frame_over_the_title_block_or_on_another_window_is_a_finding() {
    let mut model = model();
    model.viewports.insert("vp-out".into(), Viewport::standard("sh-1", "v-first", Point2 { x: 350.0, y: 20.0 }));
    model.viewports.insert("vp-title".into(), Viewport::standard("sh-1", "v-first", Point2 { x: 300.0, y: 200.0 }));
    model.viewports.insert("vp-over".into(), Viewport::standard("sh-1", "v-first", Point2 { x: 100.0, y: 60.0 }));
    let found = issues(&layout(&model, "sh-1"));
    assert!(found.contains(&(SheetIssue::ViewportOutside, vec!["vp-out"])), "{found:?}");
    assert!(found.contains(&(SheetIssue::ViewportOverTitleBlock, vec!["vp-title"])), "{found:?}");
    assert!(found.contains(&(SheetIssue::ViewportsOverlap, vec!["vp-1", "vp-over"])), "{found:?}");
    assert!(!found.iter().any(|(issue, ids)| *issue == SheetIssue::ViewportOutside && ids == &vec!["vp-1"]), "{found:?}");
}

#[test]
fn the_revision_table_is_covered_too_and_touching_windows_are_fine() {
    let mut model = model();
    model.sheet_revisions.insert("r-1".into(), SheetRevision { sheet: "sh-1".into(), number: "A".into(), date: String::new(), description: "x".into(), author: String::new() });
    model.viewports.insert("vp-table".into(), Viewport::standard("sh-1", "v-first", Point2 { x: 300.0, y: 165.0 }));
    model.viewports.insert("vp-next".into(), Viewport::standard("sh-1", "v-first", Point2 { x: 120.0, y: 40.0 }));
    let found = issues(&layout(&model, "sh-1"));
    assert!(found.contains(&(SheetIssue::ViewportOverTitleBlock, vec!["vp-table"])), "{found:?}");
    assert!(!found.iter().any(|(issue, _)| *issue == SheetIssue::ViewportsOverlap), "{found:?}");
}

#[test]
fn a_view_without_a_drawing_is_an_empty_viewport() {
    let model = model();
    let none: BTreeMap<&str, &ViewLinework> = BTreeMap::new();
    let layout = layout_of(&model, "sh-1", &none);
    assert!(layout.viewports[0].empty);
    assert_eq!(issues(&layout), [(SheetIssue::ViewportEmpty, vec!["vp-1"])]);
}

#[test]
fn a_viewport_of_a_missing_view_is_not_placed_and_a_missing_sheet_has_no_layout() {
    let mut model = model();
    model.viewports.insert("vp-lost".into(), Viewport::standard("sh-1", "v-gone", Point2 { x: 130.0, y: 40.0 }));
    assert_eq!(layout(&model, "sh-1").viewports.len(), 1);
    assert_eq!(layout(&model, "sh-9"), SheetLayout::default());
}

#[test]
fn only_the_viewports_of_the_sheet_are_placed_in_id_order() {
    let mut model = model();
    model.viewports.insert("vp-0".into(), Viewport::standard("sh-1", "v-first", Point2 { x: 200.0, y: 40.0 }));
    model.viewports.insert("vp-x".into(), Viewport::standard("sh-2", "v-first", Point2 { x: 30.0, y: 40.0 }));
    assert_eq!(layout(&model, "sh-1").viewports.iter().map(|placed| placed.viewport.as_str()).collect::<Vec<_>>(), ["vp-0", "vp-1"]);
    assert_eq!(layout(&model, "sh-2").viewports.iter().map(|placed| placed.viewport.as_str()).collect::<Vec<_>>(), ["vp-x"]);
}

#[test]
fn the_findings_read_in_english_and_german() {
    let finding = SheetFinding { issue: SheetIssue::ViewportsOverlap, viewports: vec!["vp-1".into(), "vp-2".into()] };
    assert_eq!(finding.text("en").as_deref(), Some("Viewports vp-1 and vp-2 overlap."));
    assert_eq!(finding.text("de").as_deref(), Some("Ansichtsfenster vp-1 und vp-2 überlappen sich."));
    assert_eq!(finding.text("fr"), None);
    [SheetIssue::ViewportOutside, SheetIssue::ViewportOverTitleBlock, SheetIssue::ViewportEmpty].into_iter().for_each(|issue| assert!(["en", "de"].into_iter().all(|locale| issue.message(locale, &["vp-1".to_string()]).is_some_and(|text| text.contains("vp-1")))));
}

#[test]
fn the_dependency_reads_the_sheet_its_viewports_its_revisions_and_the_names_of_its_views_only() {
    let base = model();
    let before = dependency(&base, "sh-1");
    let changed = |change: &dyn Fn(&mut ModelSnapshot)| {
        let mut next = base.clone();
        change(&mut next);
        dependency(&next, "sh-1") != before
    };
    assert!(changed(&|next| next.sheets.get_mut("sh-1").unwrap().drawn_by = "UG".into()));
    assert!(changed(&|next| next.viewports.get_mut("vp-1").unwrap().label = Some("Entrance".into())));
    assert!(changed(&|next| next.views.get_mut("v-ground").unwrap().name = "Renamed".into()));
    assert!(changed(&|next| {
        next.sheet_revisions.insert("r-1".into(), SheetRevision { sheet: "sh-1".into(), number: "A".into(), date: String::new(), description: "x".into(), author: String::new() });
    }));
    assert!(!changed(&|next| next.sheets.get_mut("sh-2").unwrap().drawn_by = "UG".into()));
    assert!(!changed(&|next| next.views.get_mut("v-first").unwrap().name = "Renamed".into()));
    assert!(!changed(&|next| next.views.get_mut("v-ground").unwrap().scale = 50));
    assert!(!changed(&|next| {
        next.viewports.insert("vp-x".into(), Viewport::standard("sh-2", "v-ground", Point2 { x: 0.0, y: 0.0 }));
    }));
    assert_eq!(dependency(&base, "sh-9"), DslValue::Null);
}
