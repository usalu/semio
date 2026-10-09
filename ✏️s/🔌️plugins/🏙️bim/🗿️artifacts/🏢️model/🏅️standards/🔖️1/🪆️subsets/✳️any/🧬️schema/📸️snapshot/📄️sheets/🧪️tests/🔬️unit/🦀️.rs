//! 🧪️ The sheet rules: paper sizes, orientation, dates and why a sheet, a viewport or a revision row cannot be written.

use super::*;
use crate::{View, ViewKind};

fn model() -> ModelSnapshot {
    let mut model = ModelSnapshot::default();
    model.buildings.insert("bldg-1".into(), crate::Building { site: "site-1".into(), name: "House".into(), origin: Point2 { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 0.0 });
    model.storeys.insert("st-ground".into(), crate::Storey { building: "bldg-1".into(), name: "Ground".into(), level: 0, height: 3.0, cut_height: None });
    model.views.insert("v-ground".into(), View::of_storey("bldg-1", "Ground plan", ViewKind::Plan, "st-ground"));
    model.views.insert("v-eye".into(), View { camera: Some(crate::ViewCamera { target: Point2 { x: 0.0, y: 0.0 }, target_height: 1.5, azimuth: 0.0, pitch: 0.2, distance: 10.0 }), ..View::standard("bldg-1", "Eye", ViewKind::Perspective) });
    model.sheets.insert("sh-1".into(), Sheet::standard("A-101", "Ground floor"));
    model
}

#[test]
fn iso_sizes_follow_the_a_series_and_halve_their_area() {
    let areas: Vec<f64> = ISO_SIZES.into_iter().map(|size| size.sides().0 * size.sides().1).collect();
    areas.windows(2).for_each(|pair| assert!((pair[1] / pair[0] - 2.0).abs() < 0.03, "{pair:?}"));
    ISO_SIZES.into_iter().for_each(|size| assert!((size.sides().0 / size.sides().1 - std::f64::consts::SQRT_2).abs() < 0.01, "{size:?}"));
    assert_eq!(IsoSize::A4.sides(), (297.0, 210.0));
    assert_eq!(IsoSize::A0.sides(), (1189.0, 841.0));
}

#[test]
fn orientation_decides_which_side_is_the_width() {
    let mut sheet = Sheet::standard("A-101", "Plan");
    assert_eq!(sheet.size(), (420.0, 297.0));
    sheet.orientation = Orientation::Portrait;
    assert_eq!(sheet.size(), (297.0, 420.0));
    sheet.paper = Paper::Custom { width: 300.0, height: 900.0 };
    assert_eq!(sheet.size(), (300.0, 900.0));
    sheet.orientation = Orientation::Landscape;
    assert_eq!(sheet.size(), (900.0, 300.0));
}

#[test]
fn names_round_trip_through_their_parsers() {
    ISO_SIZES.into_iter().for_each(|size| assert_eq!(IsoSize::parse(size.name()), Some(size)));
    assert_eq!(IsoSize::parse(" a3 "), Some(IsoSize::A3));
    assert_eq!(IsoSize::parse("A5"), None);
    assert_eq!(Orientation::parse("PORTRAIT"), Some(Orientation::Portrait));
    assert_eq!(Orientation::parse("diagonal"), None);
    assert_eq!(Paper::iso(IsoSize::A2).name(), "A2");
    assert_eq!(Paper::Custom { width: 400.0, height: 600.0 }.name(), "600 × 400");
}

#[test]
fn dates_are_empty_or_year_month_day() {
    ["", "2026-10-09", "1999-01-31"].into_iter().for_each(|text| assert!(valid_date(text), "{text}"));
    ["2026-13-01", "2026-00-10", "2026-10-32", "26-10-09", "2026/10/09", "2026-1-09", "20261009", "abcd-ef-gh"].into_iter().for_each(|text| assert!(!valid_date(text), "{text}"));
}

#[test]
fn a_sheet_needs_a_number_a_name_a_printable_paper_and_a_date_that_reads() {
    let base = model();
    let sheet = |change: &dyn Fn(&mut Sheet)| {
        let mut sheet = Sheet::standard("A-102", "First floor");
        change(&mut sheet);
        sheet_problem(&base, "sh-2", &sheet)
    };
    assert_eq!(sheet(&|_| {}), None);
    assert_eq!(sheet(&|sheet| sheet.number = " ".into()).map(|problem| problem.field), Some("number"));
    assert_eq!(sheet(&|sheet| sheet.number = "A-101".into()).map(|problem| problem.field), Some("number"));
    assert_eq!(sheet(&|sheet| sheet.name = String::new()).map(|problem| problem.field), Some("name"));
    assert_eq!(sheet(&|sheet| sheet.paper = Paper::Custom { width: 10.0, height: 400.0 }).map(|problem| problem.field), Some("paper"));
    assert_eq!(sheet(&|sheet| sheet.paper = Paper::Custom { width: 400.0, height: f64::NAN }).map(|problem| problem.field), Some("paper"));
    assert_eq!(sheet(&|sheet| sheet.paper = Paper::Custom { width: 400.0, height: 300.0 }), None);
    assert_eq!(sheet(&|sheet| sheet.date = "2026-02-30x".into()).map(|problem| problem.field), Some("date"));
    assert_eq!(sheet_problem(&base, "sh-1", base.sheets.get("sh-1").unwrap()), None, "a sheet is no duplicate of itself");
}

#[test]
fn a_viewport_needs_a_sheet_a_drawn_view_a_scale_and_a_window_with_a_size() {
    let base = model();
    let at = Point2 { x: 20.0, y: 20.0 };
    let problem = |change: &dyn Fn(&mut Viewport)| {
        let mut viewport = Viewport::standard("sh-1", "v-ground", at);
        change(&mut viewport);
        viewport_problem(&base, "vp-1", &viewport)
    };
    assert_eq!(problem(&|_| {}), None);
    let found = problem(&|viewport| viewport.sheet = "sh-9".into()).unwrap();
    assert_eq!((found.missing, found.field), (true, "sheet"));
    let found = problem(&|viewport| viewport.view = "v-9".into()).unwrap();
    assert_eq!((found.missing, found.field), (true, "view"));
    let found = problem(&|viewport| viewport.view = "v-eye".into()).unwrap();
    assert_eq!((found.missing, found.field), (false, "view"));
    assert_eq!(problem(&|viewport| viewport.scale = 0).map(|problem| problem.field), Some("scale"));
    assert_eq!(problem(&|viewport| viewport.scale = 1001).map(|problem| problem.field), Some("scale"));
    assert_eq!(problem(&|viewport| viewport.scale = 1000), None);
    assert_eq!(problem(&|viewport| viewport.position = Point2 { x: f64::INFINITY, y: 0.0 }).map(|problem| problem.field), Some("position"));
    let empty = ViewCrop { min: Point2 { x: 1.0, y: 1.0 }, max: Point2 { x: 1.0, y: 5.0 } };
    assert_eq!(problem(&|viewport| viewport.crop = Some(empty)).map(|problem| problem.field), Some("crop"));
    assert_eq!(problem(&|viewport| viewport.label = Some("  ".into())).map(|problem| problem.field), Some("label"));
    assert_eq!(problem(&|viewport| viewport.label = Some("Entrance".into())), None);
}

#[test]
fn a_revision_needs_a_sheet_a_mark_unique_on_it_a_date_and_a_description() {
    let mut base = model();
    let row = |mark: &str| SheetRevision { sheet: "sh-1".into(), number: mark.into(), date: "2026-10-01".into(), description: "Issued for permit".into(), author: "UG".into() };
    base.sheet_revisions.insert("rev-1".into(), row("A"));
    assert_eq!(revision_problem(&base, "rev-2", &row("B")), None);
    assert_eq!(revision_problem(&base, "rev-1", &row("A")), None);
    assert_eq!(revision_problem(&base, "rev-2", &row("A")).map(|problem| problem.field), Some("number"));
    assert_eq!(revision_problem(&base, "rev-2", &row(" ")).map(|problem| problem.field), Some("number"));
    assert_eq!(revision_problem(&base, "rev-2", &SheetRevision { sheet: "sh-9".into(), ..row("B") }).map(|problem| (problem.missing, problem.field)), Some((true, "sheet")));
    assert_eq!(revision_problem(&base, "rev-2", &SheetRevision { date: "soon".into(), ..row("B") }).map(|problem| problem.field), Some("date"));
    assert_eq!(revision_problem(&base, "rev-2", &SheetRevision { description: String::new(), ..row("B") }).map(|problem| problem.field), Some("description"));
}

#[test]
fn the_revision_table_and_the_viewports_of_a_sheet_come_in_a_stable_order() {
    let mut base = model();
    let row = |mark: &str| SheetRevision { sheet: "sh-1".into(), number: mark.into(), date: String::new(), description: "x".into(), author: String::new() };
    base.sheet_revisions.insert("r-z".into(), row("B"));
    base.sheet_revisions.insert("r-a".into(), row("C"));
    base.sheet_revisions.insert("r-m".into(), row("A"));
    base.sheet_revisions.insert("r-other".into(), SheetRevision { sheet: "sh-2".into(), ..row("A") });
    assert_eq!(revisions_of(&base, "sh-1").into_iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(), ["r-m", "r-z", "r-a"]);
    base.viewports.insert("vp-b".into(), Viewport::standard("sh-1", "v-ground", Point2 { x: 0.0, y: 0.0 }));
    base.viewports.insert("vp-a".into(), Viewport::standard("sh-1", "v-ground", Point2 { x: 0.0, y: 0.0 }));
    base.viewports.insert("vp-c".into(), Viewport::standard("sh-2", "v-ground", Point2 { x: 0.0, y: 0.0 }));
    assert_eq!(viewports_of(&base, "sh-1").into_iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(), ["vp-a", "vp-b"]);
}
