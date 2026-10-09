use super::*;

const KINDS: [PlanKind; 32] = [
    PlanKind::WallCut,
    PlanKind::WallLayer,
    PlanKind::WallOutline,
    PlanKind::CurtainAxis,
    PlanKind::CurtainMullion,
    PlanKind::WindowFrame,
    PlanKind::WindowGlazing,
    PlanKind::WindowSill,
    PlanKind::DoorLeaf,
    PlanKind::DoorSwing,
    PlanKind::ColumnCut,
    PlanKind::ColumnOutline,
    PlanKind::BeamOutline,
    PlanKind::SlabEdge,
    PlanKind::SlabHole,
    PlanKind::RoofOutline,
    PlanKind::StairOutline,
    PlanKind::StairRiser,
    PlanKind::StairCutLine,
    PlanKind::StairArrow,
    PlanKind::StairLanding,
    PlanKind::RailingPath,
    PlanKind::SpaceOutline,
    PlanKind::SpaceTag,
    PlanKind::GridLine,
    PlanKind::GridBubble,
    PlanKind::GridLabel,
    PlanKind::SectionCut,
    PlanKind::Silhouette,
    PlanKind::Edge,
    PlanKind::Datum,
    PlanKind::DatumLabel,
];

#[test]
fn every_kind_has_its_own_kebab_case_class() {
    let mut classes: Vec<&str> = KINDS.iter().map(|kind| kind_class(*kind)).collect();
    classes.sort();
    classes.dedup();
    assert_eq!(classes.len(), KINDS.len());
    assert!(classes.iter().all(|class| class.chars().all(|letter| letter.is_ascii_lowercase() || letter == '-')));
    assert_eq!(kind_class(PlanKind::WallCut), "wall-cut");
}

#[test]
fn the_four_line_styles_have_a_class_a_rank_and_a_stroke_width_that_gets_lighter() {
    let styles = [PlanStyle::Hidden, PlanStyle::Projection, PlanStyle::Cut, PlanStyle::Annotation];
    assert_eq!(styles.map(style_class), STYLE_CLASSES);
    assert_eq!(styles.map(paint_rank), [0, 1, 2, 3]);
    let width = |class: &str| STROKE_WIDTHS.iter().find(|(name, _)| *name == class).expect("a width").1;
    assert!(width("cut") > width("projection") && width("projection") > width("hidden") && width("hidden") > width("annotation"));
}

#[test]
fn the_style_sheet_strokes_each_class_dashes_only_hidden_and_fills_only_cut_and_projection_regions() {
    let sheet = sheet();
    for (class, width) in STROKE_WIDTHS {
        assert!(sheet.contains(&format!(".{class}{{stroke-width:{width}}}")), "{class}");
    }
    assert_eq!(sheet.matches("stroke-dasharray").count(), 1);
    assert!(sheet.contains(".hidden{stroke-dasharray") && sheet.contains("path.region.cut{fill:") && sheet.contains("path.region.projection{fill:") && sheet.contains("fill-rule:evenodd"));
    assert!(!sheet.contains('<') && !sheet.contains('&'));
}

#[test]
fn every_kind_of_view_has_its_own_kebab_case_class_and_the_style_sheet_sets_the_font_on_the_sheet() {
    let kinds = [ViewKind::Plan, ViewKind::CeilingPlan, ViewKind::Section, ViewKind::Elevation, ViewKind::Orthographic, ViewKind::Perspective];
    assert_eq!(kinds.map(view_class), ["plan", "ceiling-plan", "section", "elevation", "orthographic", "perspective"]);
    assert!(sheet().contains(".sheet{font-family:sans-serif}"));
}
