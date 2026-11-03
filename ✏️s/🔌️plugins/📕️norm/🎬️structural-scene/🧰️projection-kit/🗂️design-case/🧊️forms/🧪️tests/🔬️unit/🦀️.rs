use super::*;
use norm::design_case::{CaseEntry, CaseNumber, InputSpec};
use norm::{ClauseId, Quantity, SubjectRef, UnitId};

struct Toy;

static VOCABULARY: &[InputSpec] = &[
    InputSpec::number("b", "Width b", "Breite b", UnitId::Millimetre),
    InputSpec::number("l", "Length l", "Länge l", UnitId::Metre),
    InputSpec::number("h", "Height h", "Höhe h", UnitId::Metre),
    InputSpec::number("n", "Force N", "Kraft N", UnitId::Newton).example(100_000.0),
    InputSpec::number("ratio", "Ratio", "Verhältnis", UnitId::One).example(1.0),
    InputSpec::number("count", "Count", "Anzahl", UnitId::One).example(3.0),
    InputSpec::number("angle", "Angle", "Winkel", UnitId::Degree).example(30.0),
    InputSpec::number("h_ef", "Effective depth", "Wirksame Höhe", UnitId::Metre),
    InputSpec::number("x", "Notch length", "Ausklinkungslänge", UnitId::Metre),
    InputSpec::number("o", "Grain across", "Faser quer", UnitId::One),
    InputSpec::number("a2", "Spacing across", "Abstand quer", UnitId::Metre),
    InputSpec::number("r", "Radius", "Radius", UnitId::Metre),
    InputSpec::number("t1", "Ply 1", "Lage 1", UnitId::Millimetre),
    InputSpec::number("t2", "Ply 2", "Lage 2", UnitId::Millimetre),
    InputSpec::number("t3", "Ply 3", "Lage 3", UnitId::Millimetre),
    InputSpec::number("o1", "Orientation 1", "Ausrichtung 1", UnitId::One),
    InputSpec::number("o2", "Orientation 2", "Ausrichtung 2", UnitId::One),
    InputSpec::number("o3", "Orientation 3", "Ausrichtung 3", UnitId::One),
    InputSpec::number("e", "End distance", "Endabstand", UnitId::Millimetre),
];

const fn entry(id: &'static str, form: CaseForm, width: &'static str, length: &'static str, height: &'static str, load: &'static str) -> CaseEntry {
    CaseEntry { id, part: "EN TOY-1", clause: "1", en: "Toy", de: "Spielzeug", inputs: &["b", "l", "h", "n", "ratio", "count", "angle"], geometry: Some(CaseGeometry { form, width, length, height, load, aux: &[] }) }
}

const fn aux_entry(id: &'static str, form: CaseForm, width: &'static str, length: &'static str, height: &'static str, aux: &'static [norm::design_case::AuxRow]) -> CaseEntry {
    CaseEntry { id, part: "EN TOY-1", clause: "1", en: "Toy", de: "Spielzeug", inputs: &["b", "l", "h", "n", "ratio", "count", "angle"], geometry: Some(CaseGeometry { form, width, length, height, load: "n", aux }) }
}

static AUX_ENTRIES: [CaseEntry; 20] = [
    aux_entry("FT", CaseForm::Footing, "b", "l", "h", &[("embed", "h"), ("water", "l")]),
    aux_entry("PI", CaseForm::Pile, "b", "", "l", &[("rows", "count"), ("columns", "count"), ("spacing", "h")]),
    aux_entry("RW", CaseForm::RetainingWall, "b", "l", "h", &[("base", "l"), ("base_t", "h"), ("toe", "h"), ("water", "h")]),
    aux_entry("SW", CaseForm::SheetWall, "b", "l", "h", &[("embed", "h"), ("prop", "h"), ("water", "h")]),
    aux_entry("SL", CaseForm::Slope, "b", "l", "h", &[("angle", "angle"), ("ratio", "ratio"), ("soft", "h"), ("level", "h")]),
    aux_entry("SB", CaseForm::SoilBlock, "b", "l", "h", &[("water", "h"), ("rod", "h")]),
    aux_entry("GA", CaseForm::GroundAnchor, "b", "l", "", &[("free", "l"), ("angle", "angle")]),
    aux_entry("SE", CaseForm::Seepage, "b", "l", "h", &[("embed", "h"), ("layer", "h")]),
    aux_entry("WE", CaseForm::Well, "b", "", "h", &[("radius", "h"), ("level", "h")]),
    aux_entry("FA", CaseForm::Fastener, "b", "l", "h", &[("thickness2", "h"), ("diameter", "h"), ("count", "count"), ("spacing", "h")]),
    aux_entry("WD", CaseForm::Weld, "b", "l", "h", &[("throat", "h")]),
    aux_entry("AN", CaseForm::Anchor, "b", "l", "h", &[("embed", "h"), ("diameter", "h")]),
    aux_entry("OP", CaseForm::Opening, "b", "l", "h", &[("open_w", "h"), ("open_l", "h")]),
    aux_entry("SY", CaseForm::Symbol, "", "", "", &[]),
    aux_entry("MN", CaseForm::Member, "b", "l", "h", &[("grain", ""), ("eff_height", "h_ef"), ("notch_length", "x")]),
    aux_entry("MG", CaseForm::Plate, "b", "l", "h", &[("grain", "o")]),
    aux_entry("TP", CaseForm::Member, "b", "l", "h", &[("grain", ""), ("taper", "angle")]),
    aux_entry("AX", CaseForm::Member, "b", "", "h", &[("apex_angle", "angle"), ("apex_radius", "r")]),
    aux_entry("LP", CaseForm::Plate, "b", "l", "", &[("plies", "t1"), ("ply_dir", "o1")]),
    aux_entry("FP", CaseForm::Fastener, "b", "l", "h", &[("thickness2", "h"), ("diameter", "h"), ("count", "count"), ("spacing", "h"), ("rows", "count"), ("spacing2", "a2")]),
];

static DIMENSION_ENTRIES: [CaseEntry; 1] = [aux_entry("DM", CaseForm::Member, "b", "l", "h", &[("dim_x", "e"), ("dim_ye", "a2"), ("dim_ze", "r"), ("dim_angle", "angle")])];

static ENTRIES: [CaseEntry; 4] = [
    entry("M", CaseForm::Member, "b", "l", "h", "n"),
    entry("C", CaseForm::Cylinder, "b", "", "h", ""),
    entry("G", CaseForm::Gauge, "", "", "", "ratio"),
    entry("X", CaseForm::Connection, "b", "l", "h", "n"),
];

impl CaseCatalogue for Toy {
    fn entries() -> Vec<CaseEntry> {
        ENTRIES.iter().chain(AUX_ENTRIES.iter()).chain(DIMENSION_ENTRIES.iter()).copied().collect()
    }

    fn vocabulary() -> &'static [InputSpec] {
        VOCABULARY
    }

    fn evaluate(_: &CaseEntry, _: norm::AnnexChoice, _: &design_case::DesignCase) -> Vec<norm::CheckResult> {
        Vec::new()
    }
}

fn row(id: &str, entry_id: &str, numbers: &[(&str, f64)]) -> CaseRow {
    CaseRow { id: id.into(), label_en: String::new(), label_de: String::new(), entry_id: entry_id.into(), numbers: numbers.iter().map(|(k, v)| CaseNumber { id: (*k).into(), value: *v }).collect(), texts: Vec::new() }
}

fn pose() -> CasePose {
    CasePose { origin: v3(10.0, 20.0, 0.0), yaw: 0.0 }
}

fn report(case: &str, demand: f64) -> CheckReport {
    let mut report = CheckReport::default();
    report.push(
        norm::CheckResult::assess("toy", "EN TOY-1", ClauseId::new("EN TOY-1", "§1", "1"), SubjectRef::new(case, format!("cases[id={case}]"), norm::LocalizedCopy::new("Toy", "Spielzeug")), norm::LocalizedCopy::new("Toy", "Spielzeug"))
            .utilization(Quantity::new(QuantityKind::Force, demand), Quantity::new(QuantityKind::Force, 100.0))
            .build(),
    );
    report
}

#[test]
fn the_size_reads_the_declared_inputs_in_metres_and_falls_back_to_the_nominal_form() {
    let set = row("c1", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let geometry = declared::<Toy>(&set).expect("geometry");
    let sized = size::<Toy>(&set, &geometry);
    assert!((sized.dx - 6.0).abs() < 1e-9 && (sized.dy - 0.3).abs() < 1e-9 && (sized.h - 0.5).abs() < 1e-9, "{sized:?}");
    let unset = row("c2", "M", &[]);
    let (w, l, h) = nominal(CaseForm::Member);
    assert_eq!(size::<Toy>(&unset, &geometry), Size { dx: l, dy: w, h });
}

#[test]
fn a_member_draws_body_supports_and_load_with_the_utilisation_colour() {
    let case = row("c1", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5), ("n", 100_000.0)]);
    let drawn = draw::<Toy>(&case, &pose(), &report("c1", 130.0), "cases", "cases", &MaterialRole::Concrete).expect("draw");
    let ids: Vec<&str> = drawn.entities.iter().map(|e| e.entity.id.as_str()).collect();
    assert_eq!(ids, ["c1", "c1:support-a", "c1:support-b", "c1:load"]);
    assert!(drawn.entities.iter().all(|e| e.utilisation.as_ref().is_some_and(|u| (u.ratio - 1.3).abs() < 1e-9)));
    assert_eq!(drawn.entities[0].handles.len(), 6);
    assert!(drawn.points.iter().all(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite()));
}

#[test]
fn the_grips_write_the_inputs_in_their_stored_units_and_bind_the_placement() {
    let case = row("c1", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5), ("n", 100_000.0)]);
    let grips = handles::<Toy>(&case, &pose(), "cases");
    let tags: Vec<&str> = grips.iter().map(|h| h.id.rsplit(':').next().unwrap()).collect();
    assert_eq!(tags, ["width", "length", "height", "load", "move-x", "move-y"]);
    let width = &grips[0].writes[0];
    assert_eq!(width.path, "cases[id=c1].numbers[id=b].value");
    assert!((width.map.base - 300.0).abs() < 1e-12 && (width.map.k - 2000.0).abs() < 1e-9, "{:?}", width.map);
    let load = &grips[3].writes[0];
    assert_eq!(load.path, "cases[id=c1].numbers[id=n].value");
    assert!((load.map.k - 50_000.0).abs() < 1e-9);
    assert_eq!(grips[4].writes[0].path, "placements[id=c1].x");
    assert!(grips.iter().all(|h| !h.writes.is_empty() && h.accessible_name.en.contains("c1") && h.accessible_name.de.contains("c1")));
}

#[test]
fn an_unset_dimension_has_no_grip_but_the_case_stays_drawn_and_movable() {
    let case = row("c3", "M", &[]);
    let grips = handles::<Toy>(&case, &pose(), "cases");
    assert_eq!(grips.iter().map(|h| h.id.as_str()).collect::<Vec<_>>(), ["c3:move-x", "c3:move-y"]);
    assert!(draw::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Neutral).is_some());
}

#[test]
fn the_gauge_pillar_follows_the_load_input_against_its_reference_mark() {
    let case = row("g1", "G", &[("ratio", 1.0)]);
    let geometry = declared::<Toy>(&case).expect("geometry");
    let driver = driver::<Toy>(&case, &geometry).expect("driver");
    assert!((driver.height() - GAUGE_REFERENCE).abs() < 1e-12);
    let drawn = draw::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Neutral).expect("draw");
    assert_eq!(drawn.entities.iter().map(|e| e.entity.id.as_str()).collect::<Vec<_>>(), ["g1", "g1:limit"]);
}

#[test]
fn every_form_projects_finite_solids_without_any_input() {
    for (index, entry) in ENTRIES.iter().enumerate() {
        let case = row(&format!("c{index}"), entry.id, &[]);
        let drawn = draw::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Neutral).expect("draw");
        assert!(!drawn.entities.is_empty() && drawn.anchor.z.is_finite());
        let [x0, x1, y0, y1] = extent::<Toy>(&case).expect("extent");
        assert!(x1 > x0 && y1 > y0);
    }
}

#[test]
fn origins_pack_cases_into_rows_without_overlap() {
    let rows: Vec<CaseRow> = (0..12).map(|i| row(&format!("c{i}"), "M", &[("l", 4.0)])).collect();
    let spots = origins::<Toy>(&rows, 0.0, 0.0);
    assert_eq!(spots.len(), 12);
    assert!(spots[1].1.x > spots[0].1.x && spots[10].1.y > spots[0].1.y && (spots[10].1.x - spots[0].1.x).abs() < 1e-12);
}

fn full(id: &str, entry_id: &str) -> CaseRow {
    row(id, entry_id, &[("b", 300.0), ("l", 6.0), ("h", 0.5), ("n", 100_000.0), ("count", 3.0), ("angle", 30.0), ("h_ef", 0.3), ("x", 0.4), ("o", 1.0), ("a2", 0.1), ("r", 8.0), ("t1", 30.0), ("t2", 40.0), ("t3", 30.0), ("o1", 1.0), ("o2", 0.0), ("o3", 1.0), ("e", 40.0)])
}

#[test]
fn every_ground_and_joint_form_draws_finite_solids_with_and_without_inputs() {
    for entry in &AUX_ENTRIES {
        for case in [full("c1", entry.id), row("c2", entry.id, &[])] {
            let drawn = draw::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Neutral).unwrap_or_else(|| panic!("{} draws nothing", entry.id));
            assert!(drawn.entities.iter().all(|e| matches!(&e.geometry, Geometry::Solid(_))), "{}", entry.id);
            assert_eq!(drawn.entities[0].entity.id, case.id, "{}: the body comes first", entry.id);
            assert!(drawn.points.iter().all(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite()), "{}", entry.id);
            let [x0, x1, y0, y1] = extent::<Toy>(&case).expect("extent");
            assert!(x1 > x0 && y1 > y0, "{}", entry.id);
            assert!(handles::<Toy>(&case, &pose(), "cases").iter().all(|h| h.accessible_name.en.contains(&case.id) && !h.writes.is_empty()), "{}", entry.id);
        }
    }
}

#[test]
fn a_footing_declares_size_embedment_and_water_grips_and_context_soil_without_utilisation() {
    let case = full("c1", "FT");
    let tags: Vec<String> = handles::<Toy>(&case, &pose(), "cases").iter().map(|h| h.id.rsplit(':').next().unwrap().to_string()).collect();
    assert_eq!(tags, ["width", "length", "height", "embed", "water", "load", "move-x", "move-y"]);
    let drawn = draw::<Toy>(&case, &pose(), &report("c1", 130.0), "cases", "cases", &MaterialRole::Concrete).expect("draw");
    let soil = drawn.entities.iter().find(|e| e.entity.id == "c1:soil-0").expect("soil");
    assert!(soil.utilisation.is_none() && soil.appearance.opacity < 1.0 && soil.handles.is_empty());
    assert!(drawn.entities[0].utilisation.is_some() && drawn.entities[0].appearance.opacity == 1.0);
    assert!(drawn.entities.iter().any(|e| e.entity.id == "c1:water"));
}

#[test]
fn a_pile_group_draws_rows_times_columns_piles_under_a_cap() {
    let drawn = draw::<Toy>(&full("g", "PI"), &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Concrete).expect("draw");
    let piles = drawn.entities.iter().filter(|e| e.entity.id == "g" || e.entity.id.starts_with("g:pile-")).count();
    assert_eq!(piles, 9);
    assert!(drawn.entities.iter().any(|e| e.entity.id == "g:cap"));
}

#[test]
fn the_declared_auxiliary_roles_are_the_schema_of_their_form() {
    for entry in &AUX_ENTRIES {
        let geometry = entry.geometry.expect("geometry");
        assert!(geometry.aux.iter().all(|(role, _)| is_aux_role(geometry.form, role)), "{}", entry.id);
    }
    assert!(aux_is_scalar(CaseForm::Pile, "rows") && aux_is_scalar(CaseForm::Slope, "angle") && !aux_is_scalar(CaseForm::Footing, "embed"));
}

#[test]
fn the_dimensions_of_every_form_follow_their_driving_inputs() {
    let rows: Vec<CaseRow> = AUX_ENTRIES.iter().chain(ENTRIES.iter()).chain(DIMENSION_ENTRIES.iter()).map(|e| full(&format!("c-{}", e.id), e.id)).collect();
    crate::case_laws::assert_case_dimension_laws::<Toy>(&rows);
}

#[test]
fn the_grid_follows_the_size_of_the_dimension() {
    assert_eq!(grid_for(2.0), GRID);
    assert_eq!(grid_for(0.2), 0.01);
    assert_eq!(grid_for(0.004), 0.001);
}

#[test]
fn the_hydraulic_head_of_a_seepage_path_is_its_own_sheet_of_exactly_the_head() {
    let drawn = draw::<Toy>(&full("s", "SE"), &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Neutral).expect("draw");
    let head = drawn.entities.iter().find(|e| e.entity.id == "s:head").expect("head sheet");
    assert!(matches!(&head.geometry, Geometry::Solid(SolidGeometry { shape: SolidShape::Box { size, .. } }) if (size.z - 0.5).abs() < 1e-12));
}

#[test]
fn an_opening_wider_than_the_slab_keeps_a_rim_and_still_follows_its_input() {
    let wide = row("o", "OP", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let signature = signature::<Toy>(&wide, &MaterialRole::Neutral);
    assert!(signature.iter().all(|v| v.is_finite()) && signature.contains(&0.5));
    crate::case_laws::assert_case_dimension_laws::<Toy>(&[wide]);
}

#[test]
fn a_fastener_with_a_declared_spacing_and_no_count_draws_a_pair() {
    let pair = row("f", "FA", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let drawn = draw::<Toy>(&pair, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Neutral).expect("draw");
    assert!(drawn.entities.iter().any(|e| e.entity.id == "f:fastener-1"));
}

fn solid_box(drawn: &CaseDraw, id: &str) -> (Vec3, Option<Hatch>, f64, bool) {
    let entity = drawn.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id} is drawn"));
    let Geometry::Solid(SolidGeometry { shape: SolidShape::Box { size, .. } }) = &entity.geometry else { panic!("{id} is a box") };
    (size.clone(), entity.appearance.hatch, entity.appearance.opacity, entity.utilisation.is_some())
}

#[test]
fn a_solid_with_a_grain_role_hatches_along_the_length_or_across_by_its_input() {
    let hatch = |numbers: &[(&str, f64)], demand: Option<f64>| {
        let case = row("g", "MG", numbers);
        let checked = demand.map_or_else(CheckReport::default, |d| report("g", d));
        draw::<Toy>(&case, &pose(), &checked, "cases", "cases", &MaterialRole::Timber).expect("draw").entities[0].appearance.hatch
    };
    assert_eq!(hatch(&[("b", 300.0), ("o", 0.0)], None), Some(Hatch::Horizontal), "grain along the length");
    assert_eq!(hatch(&[("b", 300.0), ("o", 1.0)], None), Some(Hatch::Vertical), "grain across the length");
    assert_eq!(hatch(&[("b", 300.0), ("o", 1.0)], Some(130.0)), Some(Hatch::Diagonal), "a failing check wins over the grain");
    let nominal = row("n", "MN", &[]);
    let drawn = draw::<Toy>(&nominal, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw");
    assert_eq!(drawn.entities[0].appearance.hatch, Some(Hatch::Horizontal), "an empty key draws the nominal grain along the length");
    let plain = row("p", "M", &[]);
    assert_eq!(draw::<Toy>(&plain, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw").entities[0].appearance.hatch, None, "a form without the role draws no grain");
}

#[test]
fn a_notched_member_draws_a_translucent_void_of_the_cut_depth_with_grips() {
    let drawn = draw::<Toy>(&full("n", "MN"), &pose(), &report("n", 130.0), "cases", "cases", &MaterialRole::Timber).expect("draw");
    let (size, _, opacity, bound) = solid_box(&drawn, "n:notch");
    assert!((size.z - 0.2).abs() < 1e-12 && (size.x - 0.4).abs() < 1e-12, "{size:?}: depth h - h_ef, length x");
    assert!(opacity < 1.0 && !bound, "the void is translucent context without a utilisation colour");
    let tags: Vec<String> = handles::<Toy>(&full("n", "MN"), &pose(), "cases").iter().map(|h| h.id.rsplit(':').next().unwrap().to_string()).collect();
    assert_eq!(tags, ["width", "length", "height", "effective-height", "notch-length", "load", "move-x", "move-y"]);
    let solid = row("s", "MN", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let plain = draw::<Toy>(&solid, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw");
    assert!(plain.entities.iter().all(|e| e.entity.id != "s:notch"), "no void while the effective depth is unset");
    let uncut = row("u", "MN", &[("b", 300.0), ("l", 6.0), ("h", 0.5), ("h_ef", 0.5)]);
    assert!(draw::<Toy>(&uncut, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw").entities.iter().all(|e| e.entity.id != "u:notch"), "no void when the effective depth equals the depth");
}

#[test]
fn a_fastener_pattern_draws_rows_times_columns_at_both_spacings() {
    let drawn = draw::<Toy>(&full("p", "FP"), &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw");
    let bolts: Vec<&SceneEntity> = drawn.entities.iter().filter(|e| e.entity.id.starts_with("p:fastener-")).collect();
    assert_eq!(bolts.len(), 9, "3 columns times 3 rows");
    let (mut xs, mut ys): (Vec<f64>, Vec<f64>) = bolts.iter().filter_map(|e| match &e.geometry { Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame, .. } }) => Some((frame.origin.x - 10.0, frame.origin.y - 20.0)), _ => None }).unzip();
    xs.sort_by(f64::total_cmp);
    ys.sort_by(f64::total_cmp);
    assert!((xs[8] - xs[0] - 1.0).abs() < 1e-9 && (ys[8] - ys[0] - 0.2).abs() < 1e-9, "spacing 0.5 m along the row, a2 = 0.1 m across: {xs:?} {ys:?}");
    let tags: Vec<String> = handles::<Toy>(&full("p", "FP"), &pose(), "cases").iter().map(|h| h.id.rsplit(':').next().unwrap().to_string()).collect();
    assert!(tags.contains(&"spacing-across".to_string()));
    let single = row("s", "FP", &[("b", 300.0), ("l", 6.0), ("h", 0.5), ("count", 1.0)]);
    let drawn = draw::<Toy>(&single, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw");
    assert_eq!(drawn.entities.iter().filter(|e| e.entity.id.starts_with("s:fastener-")).count(), 1, "one fastener without a pattern");
}

fn body_shape(drawn: &CaseDraw) -> SolidShape {
    let Geometry::Solid(SolidGeometry { shape }) = &drawn.entities[0].geometry else { panic!("the body is a solid") };
    shape.clone()
}

fn area(points: &[Vec2]) -> f64 {
    points.iter().zip(points.iter().cycle().skip(1)).map(|(a, b)| a.x * b.y - b.x * a.y).sum::<f64>().abs() / 2.0
}

fn plain(case: &CaseRow) -> CaseDraw {
    draw::<Toy>(case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw")
}

fn tags(case: &CaseRow) -> Vec<String> {
    handles::<Toy>(case, &pose(), "cases").iter().map(|h| h.id.rsplit(':').next().unwrap().to_string()).collect()
}

#[test]
fn a_tapered_member_draws_the_elevation_as_a_prism_with_an_angle_grip() {
    let case = full("t", "TP");
    let SolidShape::Prism { base, height, frame } = body_shape(&plain(&case)) else { panic!("a profile prism") };
    assert!((height - 0.3).abs() < 1e-12, "extruded across the width b");
    assert_eq!((frame.z.x, frame.z.y, frame.z.z), (0.0, -1.0, 0.0), "towards -Y");
    let top = base.outer.points.iter().map(|p| p.y).fold(f64::MIN, f64::max);
    assert!((top - (0.5 + 6.0 * 30.0_f64.to_radians().tan())).abs() < 1e-9, "the far end rises by l tan(alpha): {top}");
    let (min, max) = base.outer.points.iter().fold((f64::MAX, f64::MIN), |m, p| (m.0.min(p.x), m.1.max(p.x)));
    assert!((max - min - 6.0).abs() < 1e-9 && (max + min).abs() < 1e-9, "the profile is as long as the member and centred");
    let grips = handles::<Toy>(&case, &pose(), "cases");
    let grip = grips.iter().find(|h| h.id.ends_with(":taper-angle")).expect("angle grip");
    assert!(matches!(grip.kind, HandleKind::Angular { .. }));
    assert!((grip.writes[0].map.apply(30.0_f64.to_radians()) - 30.0).abs() < 1e-9, "writes degrees from the zero direction");
    assert!(tags(&case).contains(&"height".to_string()), "the size grips stay");
    let straight = row("s", "TP", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    assert!(matches!(body_shape(&plain(&straight)), SolidShape::Prism { .. }) == false, "no angle, no profile");
}

#[test]
fn a_curved_member_draws_the_arc_prism_an_apex_zone_and_the_angle_and_radius_grips() {
    let case = full("a", "AX");
    let drawn = plain(&case);
    let SolidShape::Prism { base, .. } = body_shape(&drawn) else { panic!("a profile prism") };
    assert!(base.outer.points.len() > 20 && area(&base.outer.points) > 0.0);
    let zone = drawn.entities.iter().find(|e| e.entity.id == "a:apex").expect("apex zone");
    assert!(matches!(&zone.appearance.channel, ColourChannel::Custom { .. }) && zone.appearance.opacity < 1.0);
    let grips = handles::<Toy>(&case, &pose(), "cases");
    let angle = grips.iter().find(|h| h.id.ends_with(":apex-angle")).expect("angle");
    let radial = grips.iter().find(|h| h.id.ends_with(":apex-radius")).expect("radius");
    assert!(matches!(angle.kind, HandleKind::Angular { .. }) && matches!(radial.kind, HandleKind::Radial { .. }));
    assert!((angle.writes[0].map.apply(30.0_f64.to_radians() - std::f64::consts::FRAC_PI_2) - 30.0).abs() < 1e-9, "the angle is measured from the vertical at the apex");
    assert!((radial.writes[0].map.apply(8.0) - 8.0).abs() < 1e-12, "the radius grip writes metres");
}

#[test]
fn a_ply_stack_draws_one_layer_per_ply_grained_by_its_orientation() {
    let case = full("p", "LP");
    let SolidShape::Laminate { layers, footprint, frame } = body_shape(&plain(&case)) else { panic!("a ply stack") };
    assert_eq!(layers.iter().map(|l| l.thickness).collect::<Vec<_>>(), [0.03, 0.04, 0.03]);
    assert!(layers[0].grain.abs() < 1e-12 && (layers[1].grain - std::f64::consts::FRAC_PI_2).abs() < 1e-12 && layers[2].grain.abs() < 1e-12, "the cross ply lies at a right angle");
    assert!((area(&footprint.outer.points) - 0.3 * 6.0).abs() < 1e-9 && (frame.z.z - 1.0).abs() < 1e-12);
    assert!(tags(&case).iter().filter(|t| t.starts_with("ply-")).count() == 2, "a grip on the top of every ply after the first");
    let grips = handles::<Toy>(&case, &pose(), "cases");
    assert!(grips.iter().any(|h| h.id.ends_with(":ply-3") && h.writes[0].path == "cases[id=p].numbers[id=t3].value"));
    let unset = row("u", "LP", &[("b", 300.0), ("l", 6.0)]);
    assert!(matches!(body_shape(&plain(&unset)), SolidShape::Box { .. }), "no declared ply, no stack");
}

fn rect_ring(x0: f64, x1: f64, y0: f64, y1: f64) -> Vec<[f64; 2]> {
    vec![[x0, y0], [x1, y0], [x1, y1], [x0, y1]]
}

fn charred() -> impl Fn(&CaseRow, Size) -> Extras {
    |_, _| {
        let part = |tag: &str, layer: &str, outline: Vec<[f64; 2]>, holes: Vec<Vec<[f64; 2]>>| CharPart { tag: tag.into(), layer: layer.into(), rgb: (60, 40, 30), opacity: 0.5, outline, holes, name: ("Char".into(), "Abbrand".into()), detail: ("char".into(), "Abbrand".into()) };
        let u_band = vec![[-0.15, -0.25], [0.15, -0.25], [0.15, 0.25], [0.13, 0.25], [0.13, -0.23], [-0.13, -0.23], [-0.13, 0.25], [-0.15, 0.25]];
        Extras {
            char: Some(Char {
                parts: vec![
                    part("char@30", "fire-30", u_band, Vec::new()),
                    part("residual", "members", rect_ring(-0.1, 0.1, -0.2, 0.25), Vec::new()),
                    part("ring", "fire-60", rect_ring(-0.15, 0.15, -0.25, 0.25), vec![rect_ring(-0.13, 0.13, -0.23, 0.23)]),
                    part("slab", "fire-60", rect_ring(-0.15, 0.15, -0.25, -0.21), Vec::new()),
                ],
            }),
            ..Extras::default()
        }
    }
}

#[test]
fn a_charred_section_extrudes_the_outlines_it_is_given_along_the_member() {
    let case = row("c", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let drawn = draw_with::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber, &charred()).expect("draw");
    let part = |id: &str| drawn.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id}"));
    let prism = |id: &str| match &part(id).geometry {
        Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { base, height, frame } }) => (base.clone(), *height, frame.clone()),
        other => panic!("{other:?}"),
    };
    let (band, length, frame) = prism("c:char@30");
    assert!((length - 6.0).abs() < 1e-12, "the section runs the length of the member");
    assert_eq!((frame.z.x, frame.z.y, frame.z.z), (1.0, 0.0, 0.0), "along the member");
    assert!((frame.origin.x - (10.0 - 3.0)).abs() < 1e-9 && (frame.origin.z - (SUPPORT_HEIGHT + 0.25)).abs() < 1e-9, "from the start of the member at mid depth");
    assert!((area(&band.outer.points) - (0.3 * 0.5 - 0.26 * 0.48)).abs() < 1e-9);
    assert!((area(&prism("c:residual").0.outer.points) - 0.2 * 0.45).abs() < 1e-9);
    assert_eq!(prism("c:ring").0.holes.len(), 1, "an outline may carry holes");
    assert!((area(&prism("c:slab").0.outer.points) - 0.3 * 0.04).abs() < 1e-9, "a slab is a thin outline from the exposed face");
    assert_eq!(part("c:char@30").layer, "fire-30");
    assert!(drawn.entities.iter().skip(1).all(|e| e.accessible_detail.en.len() > 1));
}

fn moving() -> impl Fn(&CaseRow, Size) -> Extras {
    |_, _| Extras {
        mode: Some(Mode {
            frequency: 5.0,
            amplitude: 0.1,
            shape: vec![(0.0, 0.0), (0.5, 1.0), (1.0, 0.0)],
            grid: Some(ModeGrid { columns: 3, rows: 2, values: vec![0.0, 1.0, 0.0, 0.0, -1.0, 0.0], peak: 2.0, unit: QuantityKind::Acceleration, title: ("Peak".into(), "Spitze".into()), name: ("Mode".into(), "Form".into()) }),
            detail: ("f = 5 Hz".into(), "f = 5 Hz".into()),
        }),
        ..Extras::default()
    }
}

#[test]
fn a_mode_overlay_ghosts_the_body_and_heat_maps_a_plate_from_the_mode_vector() {
    let case = row("m", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let drawn = draw_with::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber, &moving()).expect("draw");
    let ghost = drawn.entities[0].appearance.deformation.as_ref().expect("ghost on the body");
    assert_eq!(ghost.samples.len(), 3);
    assert!((ghost.samples[1].delta.z - 0.1).abs() < 1e-12 && ghost.frequency.as_ref().is_some_and(|q| (q.value - 5.0).abs() < 1e-12));
    let plate = drawn.entities.iter().find(|e| e.entity.id == "m:mode").expect("mode surface");
    assert_eq!(plate.entity.kind, EntityKind::Surface);
    let Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }) = &plate.geometry else { panic!("grid") };
    assert_eq!((grid.columns, grid.rows, grid.points.len()), (3, 2, 6));
    let ColourChannel::Fields { fields, .. } = &plate.appearance.channel else { panic!("field channel") };
    let FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex values") };
    assert_eq!(values, &vec![0.0, 2.0, 0.0, 0.0, 2.0, 0.0], "peak times |value|");
    assert_eq!(plate.appearance.deformation.as_ref().map(|o| o.surface.len()), Some(6));
    assert_eq!(drawn.legends.len(), 1, "one legend for the heat map");
    let bare = plain(&case);
    assert!(bare.legends.is_empty() && bare.entities[0].appearance.deformation.is_none());
}

fn assembled() -> impl Fn(&CaseRow, Size) -> Extras {
    |_, size| {
        let text = |t: &str| (t.to_string(), t.to_string());
        let sub = |tag: &str, at: [f64; 3], shape: SubShape| Sub { tag: tag.into(), name: text("Sub"), detail: text("sub"), role: MaterialRole::Timber, rgb: None, opacity: 1.0, at, axes: None, shape };
        Extras {
            assembly: Some(Assembly {
                half: (size.dx / 2.0, size.dy / 2.0),
                top: size.h,
                subs: vec![
                    sub("a", [0.0, 0.0, 0.0], SubShape::Box { size: [size.dx, size.dy, size.h] }),
                    sub("b", [1.0, 0.0, 0.0], SubShape::Cylinder { radius: 0.1, height: 1.0 }),
                    Sub { axes: Some([[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]]), ..sub("c", [0.0, 0.0, 0.0], SubShape::Box { size: [0.2, 0.1, 0.3] }) },
                    sub("d", [0.0, 0.0, 0.0], SubShape::Laminate { length: 1.0, width: 0.5, plies: vec![(0.02, true), (0.02, false)] }),
                    Sub { rgb: Some((1, 2, 3)), opacity: 0.5, ..sub("e", [0.0, 0.0, 0.5], SubShape::Pins { kind: PinKind::Nail, diameter: 0.01, to: [0.0, 0.0, 0.1], pattern: Pins::Points(vec![[0.0, 0.0, 0.0], [0.0, 0.1, 0.0]]) }) },
                ],
            }),
            grips: vec![Grip { tag: "lift".into(), key: "h".into(), factor: 1.0, kind: GripKind::Scalar, axis: [0.0, 0.0, 1.0], anchor: [0.0, 0.0, size.h] }],
            notes: vec![Note { tag: "note".into(), at: [0.0, 0.0, size.h], text: ("n".into(), "n".into()) }],
            ..Extras::default()
        }
    }
}

#[test]
fn an_assembly_replaces_the_form_by_transformed_sub_forms_with_grips_and_notes() {
    let case = row("s", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let turned = CasePose { origin: v3(10.0, 20.0, 0.0), yaw: std::f64::consts::FRAC_PI_2 };
    let drawn = draw_with::<Toy>(&case, &turned, &CheckReport::default(), "cases", "cases", &MaterialRole::Timber, &assembled()).expect("draw");
    assert_eq!(drawn.entities.iter().map(|e| e.entity.id.as_str()).collect::<Vec<_>>(), ["s", "s:b", "s:c", "s:d", "s:e"], "the first sub-form is the body, the form's own supports are gone");
    let Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame, .. } }) = &drawn.entities[1].geometry else { panic!("cylinder") };
    assert!((frame.origin.x - 10.0).abs() < 1e-9 && (frame.origin.y - 21.0).abs() < 1e-9, "the local x of a sub-form turns with the yaw of the case: {:?}", frame.origin);
    assert!(matches!(&drawn.entities[2].geometry, Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame, .. } }) if (frame.x.z.abs() < 1e-12)), "a transformed sub-form keeps its own axes");
    assert!(matches!(&drawn.entities[3].geometry, Geometry::Solid(SolidGeometry { shape: SolidShape::Laminate { layers, .. } }) if layers.len() == 2));
    assert_eq!(drawn.entities[4].entity.kind, EntityKind::Fastener);
    assert!(drawn.entities[4].appearance.opacity < 1.0 && drawn.entities.iter().all(|e| e.utilisation.is_none()));
    assert_eq!(drawn.notes.len(), 1);
    assert!(handles_with::<Toy>(&case, &pose(), "cases", &assembled()).iter().any(|h| h.id.ends_with(":lift")), "the extra grip comes with the size grips");
    let [x0, x1, y0, y1] = extent_with::<Toy>(&case, &assembled()).expect("extent");
    assert!((x1 - x0 - (6.0 + 2.0 * MARGIN)).abs() < 1e-9 && (y1 - y0 - (0.3 + 2.0 * MARGIN)).abs() < 1e-9, "the plan extent follows the assembly");
    let attached = |_: &CaseRow, _: Size| Extras { attachments: vec![Sub { tag: "extra".into(), name: ("Extra".into(), "Extra".into()), detail: ("e".into(), "e".into()), role: MaterialRole::Neutral, rgb: None, opacity: 1.0, at: [0.0, 0.0, 1.0], axes: None, shape: SubShape::Box { size: [0.1, 0.1, 0.1] } }], ..Extras::default() };
    let kept = draw_with::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber, &attached).expect("draw");
    assert_eq!(kept.entities.iter().map(|e| e.entity.id.as_str()).collect::<Vec<_>>(), ["s", "s:support-a", "s:support-b", "s:extra"], "an attachment is added to the parts of the form");
}

#[test]
fn the_dimensions_of_extras_follow_their_driving_inputs() {
    let rows = vec![full("a", "M")];
    crate::case_laws::assert_case_dimension_laws_with::<Toy>(&rows, &assembled());
    crate::case_laws::assert_case_dimension_laws_with::<Toy>(&rows, &moving());
    crate::case_laws::assert_case_dimension_laws_with::<Toy>(&rows, &charred());
}

#[test]
fn the_pack_keeps_cases_clear_of_each_other_and_of_reserved_rectangles_in_shelves() {
    let rows: Vec<CaseRow> = (0..40).map(|i| row(&format!("c{i}"), "M", &[("l", 4.0 + f64::from(i % 5))])).collect();
    let reserved = [[10.0, 30.0, 3.0, 20.0], [-5.0, 100.0, 40.0, 41.0]];
    let spots = origins_with::<Toy>(&rows, 0.0, 4.0, &reserved, &no_extras);
    assert_eq!(spots.len(), 40);
    let rects: Vec<[f64; 4]> = spots.iter().zip(&rows).map(|((_, o), r)| { let e = extent::<Toy>(r).unwrap(); [o.x + e[0], o.x + e[1], o.y + e[2], o.y + e[3]] }).collect();
    for (i, a) in rects.iter().enumerate() {
        assert!(a[0] >= -1e-9 && a[1] <= PACK_WIDTH + 1e-9, "inside the shelf width: {a:?}");
        assert!(reserved.iter().all(|r| !overlaps(a, r)), "{a:?} touches a reserved rectangle");
        assert!(rects[i + 1..].iter().all(|b| !overlaps(a, b)), "{a:?} overlaps another case");
    }
    assert!(rects.iter().map(|r| r[2]).fold(f64::MIN, f64::max) > rects[0][2], "the cases fill more than one shelf");
    assert!(!overlaps(&[0.0, 1.0, 0.0, 1.0], &[1.0, 2.0, 0.0, 1.0]), "touching rectangles do not overlap");
}

fn prism_of(drawn: &CaseDraw, id: &str) -> (Vec<Vec2>, f64) {
    match &drawn.entities.iter().find(|e| e.entity.id == id).unwrap_or_else(|| panic!("{id}")).geometry {
        Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { base, height, .. } }) => (base.outer.points.clone(), *height),
        other => panic!("{other:?}"),
    }
}

fn spread(values: impl Iterator<Item = f64>) -> f64 {
    let all: Vec<f64> = values.collect();
    all.iter().copied().fold(f64::MIN, f64::max) - all.iter().copied().fold(f64::MAX, f64::min)
}

fn origin_of(entity: &SceneEntity) -> [f64; 3] {
    match &entity.geometry {
        Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame, .. } | SolidShape::Cylinder { frame, .. } | SolidShape::Prism { frame, .. } | SolidShape::Laminate { frame, .. } }) => [frame.origin.x, frame.origin.y, frame.origin.z],
        Geometry::Fastener(pins) => [pins.axis.from.x, pins.axis.from.y, pins.axis.from.z],
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_profile_forms_equal_the_independent_python_oracle() {
    let taper = plain(&full("t", "TP"));
    let (ring, depth) = prism_of(&taper, "t");
    let apex = plain(&full("a", "AX"));
    let (zone, zone_depth) = prism_of(&apex, "a:apex");
    let stack = plain(&full("p", "LP"));
    let SolidShape::Laminate { layers, footprint, .. } = body_shape(&stack) else { panic!("a ply stack") };
    let cut = row("c", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let burnt = draw_with::<Toy>(&cut, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber, &charred()).expect("draw");
    let (band, length) = prism_of(&burnt, "c:char@30");
    let (residual, _) = prism_of(&burnt, "c:residual");
    let slab = match &burnt.entities.iter().find(|e| e.entity.id == "c:slab").expect("slab").geometry {
        Geometry::Solid(SolidGeometry { shape: SolidShape::Box { size, .. } }) => size.z,
        other => panic!("{other:?}"),
    };
    let shaken = draw_with::<Toy>(&row("m", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]), &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber, &moving()).expect("draw");
    let ghost = shaken.entities[0].appearance.deformation.as_ref().expect("ghost");
    let plate = shaken.entities.iter().find(|e| e.entity.id == "m:mode").expect("plate");
    let Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }) = &plate.geometry else { panic!("grid") };
    let ColourChannel::Fields { fields, .. } = &plate.appearance.channel else { panic!("fields") };
    let FieldSamples::Vertex { values } = &fields[0].samples else { panic!("vertex") };
    let turned = CasePose { origin: v3(10.0, 20.0, 0.0), yaw: 0.7 };
    let placed = draw_with::<Toy>(&row("s", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]), &turned, &CheckReport::default(), "cases", "cases", &MaterialRole::Timber, &assembled()).expect("draw");
    let subs = [("a", [0.0, 0.0, 0.0]), ("b", [1.0, 0.0, 0.0]), ("c", [0.0, 0.0, 0.0]), ("d", [0.0, 0.0, 0.0]), ("e", [0.0, 0.0, 0.5])];
    let notes = dimension_annotations::<Toy>(&full("d", "DM"), &pose(), Tone::Info);
    let (line_from, line_to, _, _, line_binding) = dimension_of(&notes, "d:dim-x");
    let (arc_from, arc_to, _, arc_value, _) = dimension_of(&notes, "d:dim-angle");
    let at = |p: &Vec3| [p.x, p.y, p.z];
    let cases = serde_json::json!([
        { "form": "dimension", "inputs": { "value": 40.0, "per_metre": 1000.0, "origin": [10.0, 20.0], "dx": 6.0, "top": 0.8 }, "measured": { "from": at(&line_from), "to": at(&line_to), "k": line_binding.expect("binding").map.k } },
        { "form": "angle", "inputs": { "degrees": 30.0, "ray": 1.8, "origin": [10.0, 20.0], "top": 0.8 }, "measured": { "from": at(&arc_from), "to": at(&arc_to), "radians": arc_value.value } },
        { "form": "taper", "inputs": { "b": 0.3, "l": 6.0, "h": 0.5, "angle_deg": 30.0 }, "measured": { "area": area(&ring), "top": ring.iter().map(|p| p.y).fold(f64::MIN, f64::max), "length": spread(ring.iter().map(|p| p.x)), "depth": depth } },
        { "form": "apex", "inputs": { "b": 0.3, "h": 0.5, "r": 8.0, "angle_deg": 30.0, "steps": 12 }, "measured": { "zone_area": area(&zone), "zone_depth": zone_depth } },
        { "form": "plies", "inputs": { "b": 0.3, "l": 6.0, "t_mm": [30.0, 40.0, 30.0], "o": [1.0, 0.0, 1.0] }, "measured": { "thicknesses": layers.iter().map(|l| l.thickness).collect::<Vec<_>>(), "grains": layers.iter().map(|l| l.grain).collect::<Vec<_>>(), "footprint_area": area(&footprint.outer.points) } },
        { "form": "char", "inputs": { "b": 0.3, "h": 0.5, "l": 6.0, "faces": [1.0, 1.0, 0.0, 1.0], "outer": 0.0, "inner": 0.02, "residual": 0.05, "slab": 0.04 }, "measured": { "band_area": area(&band), "residual_area": area(&residual), "slab_depth": slab, "length": length } },
        { "form": "mode", "inputs": { "amplitude": 0.1, "shape": [[0.0, 0.0], [0.5, 1.0], [1.0, 0.0]], "peak": 2.0, "values": [0.0, 1.0, 0.0, 0.0, -1.0, 0.0], "columns": 3, "rows": 2, "l": 6.0, "b": 0.3 }, "measured": { "ghost_max": ghost.samples.iter().map(|s| s.delta.z.abs()).fold(0.0, f64::max), "heat": values, "nodes": grid.points.len(), "extent": [spread(grid.points.iter().map(|p| p.x)), spread(grid.points.iter().map(|p| p.y))] } },
        { "form": "assembly", "inputs": { "origin": [10.0, 20.0], "yaw": 0.7, "subs": subs.iter().map(|(tag, at)| serde_json::json!({ "tag": tag, "at": at })).collect::<Vec<_>>() }, "measured": { "count": placed.entities.len(), "origins": placed.entities.iter().map(origin_of).collect::<Vec<_>>() } },
    ]);
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧰️projection-kit/🗂️design-case/🧊️forms/🔮️oracle/🐍️.py");
    let output = norm::python_oracle::run_ok(script, &[], Some(cases.to_string().as_bytes()));
    let verdict = String::from_utf8_lossy(&output.stdout);
    assert!(verdict.contains("\"ok\": true"), "oracle verdict: {verdict}");
}

fn grip_of(case: &CaseRow, tag: &str) -> Handle {
    handles::<Toy>(case, &pose(), "cases").into_iter().find(|h| h.id.ends_with(&format!(":{tag}"))).unwrap_or_else(|| panic!("grip {tag} of {}", case.id))
}

#[test]
fn the_fastener_count_and_row_steppers_write_the_integer_count_of_their_input() {
    let count = grip_of(&full("f", "FP"), "count");
    assert!(matches!(count.kind, HandleKind::CountStepper) && count.writes[0].path == "cases[id=f].numbers[id=count].value");
    assert_eq!((count.writes[0].map.base, count.writes[0].map.k, &count.writes[0].quantise), (0.0, 1.0, &Some(Quantise::Int)));
    assert_eq!((count.limits.as_ref().map(|l| l.min), count.limits.as_ref().map(|l| l.max)), (Some(1.0), Some(12.0)));
    assert!(grip_of(&full("f", "FP"), "rows").writes[0].path.ends_with("[id=count].value"));
    assert!(!tags(&full("f", "FA")).contains(&"rows".to_string()), "no rows role, no rows stepper");
    assert!(tags(&row("u", "FP", &[("b", 300.0)])).iter().all(|t| t != "count"), "an unset count has no stepper");
}

#[test]
fn the_pile_group_steppers_write_rows_and_columns() {
    let case = full("g", "PI");
    for tag in ["rows", "columns"] {
        let stepper = grip_of(&case, tag);
        assert!(matches!(stepper.kind, HandleKind::CountStepper) && stepper.writes[0].quantise == Some(Quantise::Int), "{tag}");
    }
}

#[test]
fn the_slope_and_anchor_angle_grips_write_degrees_about_their_zero_direction() {
    let slope = grip_of(&full("s", "SL"), "slope-angle");
    assert!(matches!(slope.kind, HandleKind::Angular { .. }) && slope.writes[0].path == "cases[id=s].numbers[id=angle].value");
    assert!((slope.writes[0].map.apply(40.0_f64.to_radians() - std::f64::consts::PI) - 40.0).abs() < 1e-9, "the slope angle is measured from the horizontal towards the toe");
    let s = slope.kind.angular_s(&slope.anchor).expect("anchor off the axis");
    assert!((slope.writes[0].map.apply(s) - 30.0).abs() < 1e-6, "the anchor stands at the current angle: {s}");
    let anchor = grip_of(&full("a", "GA"), "anchor-angle");
    let s = anchor.kind.angular_s(&anchor.anchor).expect("anchor off the axis");
    assert!((anchor.writes[0].map.apply(s) - 30.0).abs() < 1e-6, "the tendon grip stands at the current inclination: {s}");
}

fn quantities() -> impl Fn(&CaseRow, Size) -> Extras {
    |_, size| Extras {
        grips: vec![
            Grip { tag: "stepper".into(), key: "count".into(), factor: 1.0, kind: GripKind::Count { min: 2.0, max: 9.0 }, axis: [0.0, 0.0, 1.0], anchor: [0.0, 0.0, size.h] },
            Grip { tag: "gap".into(), key: "l".into(), factor: 2.0, kind: GripKind::Spacing, axis: [1.0, 0.0, 0.0], anchor: [1.0, 0.0, size.h] },
            Grip { tag: "gauge".into(), key: "n".into(), factor: 1.0, kind: GripKind::Gauge { k: 20_000.0, step: 500.0, limits: (-1.0, 2.0), follow: vec![Follow { key: "ratio".into(), base: 4.0, k: 10.0, step: 0.5 }] }, axis: [0.0, 0.0, 1.0], anchor: [0.0, 0.0, size.h + 1.0] },
        ],
        ..Extras::default()
    }
}

#[test]
fn extra_count_spacing_and_gauge_grips_bind_their_input_and_followers() {
    let case = row("q", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5), ("n", 100_000.0), ("count", 3.0), ("ratio", 1.0)]);
    let grips = handles_with::<Toy>(&case, &pose(), "cases", &quantities());
    let find = |tag: &str| grips.iter().find(|h| h.id == format!("q:{tag}")).unwrap_or_else(|| panic!("grip {tag}"));
    let stepper = find("stepper");
    assert!(matches!(stepper.kind, HandleKind::CountStepper) && stepper.writes.len() == 1 && stepper.writes[0].map == Affine::IDENTITY && stepper.writes[0].quantise == Some(Quantise::Int));
    let gap = find("gap");
    assert!(matches!(gap.kind, HandleKind::ScalarAlong { .. }) && gap.writes[0].path == "cases[id=q].numbers[id=l].value");
    assert!((gap.writes[0].map.base - 6.0).abs() < 1e-12 && (gap.writes[0].map.k - 2.0).abs() < 1e-12, "stored change per metre = factor × per_metre: {:?}", gap.writes[0].map);
    let gauge = find("gauge");
    let paths: Vec<&str> = gauge.writes.iter().map(|w| w.path.as_str()).collect();
    assert_eq!(paths, ["cases[id=q].numbers[id=n].value", "cases[id=q].numbers[id=ratio].value"], "one drag writes the input and its follower");
    assert!((gauge.writes[0].map.apply(0.5) - 110_000.0).abs() < 1e-9 && (gauge.writes[1].map.apply(0.5) - 9.0).abs() < 1e-12);
    let unset = row("u", "M", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    let bare: Vec<String> = handles_with::<Toy>(&unset, &pose(), "cases", &quantities()).iter().map(|h| h.id.clone()).collect();
    assert!(bare.iter().all(|id| !id.ends_with(":stepper") && !id.ends_with(":gauge")), "an input without a leaf has no grip: {bare:?}");
    let drawn = draw_with::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber, &quantities()).expect("draw");
    assert!(["q:stepper", "q:gap", "q:gauge"].iter().all(|id| drawn.entities[0].handles.contains(&(*id).to_string())), "the body carries the grips");
}

#[test]
fn the_slope_ratio_grip_moves_the_toe_by_the_height_times_the_ratio() {
    let case = row("s", "SL", &[("b", 2.0), ("l", 6.0), ("h", 3.0), ("ratio", 2.0)]);
    let ratio = grip_of(&case, "slope-ratio");
    assert!(matches!(ratio.kind, HandleKind::ScalarAlong { .. }) && ratio.writes[0].path == "cases[id=s].numbers[id=ratio].value");
    assert!((ratio.writes[0].map.apply(0.0) - 2.0).abs() < 1e-12 && (ratio.writes[0].map.apply(3.0) - 3.0).abs() < 1e-12, "one more metre of run is 1/h more ratio");
    assert!((ratio.anchor.x - (10.0 + 6.0)).abs() < 1e-9, "the grip stands at the toe, h x ratio = 6 m from the crest: {:?}", ratio.anchor);
    assert!(!tags(&full("s2", "SL")).contains(&"slope-ratio".to_string()), "without a ratio leaf the angle is the input");
}

fn dimension_of(annotations: &[Annotation], id: &str) -> (Vec3, Vec3, Vec3, Quantity, Option<FieldBinding>) {
    annotations.iter().find_map(|a| match a { Annotation::Dimension { id: found, from, to, offset, value, binding } if found == id => Some((from.clone(), to.clone(), offset.clone(), value.clone(), binding.clone())), _ => None }).unwrap_or_else(|| panic!("dimension {id}"))
}

#[test]
fn a_linear_dimension_is_a_line_a_label_a_marker_and_a_grip_bound_to_its_input() {
    let case = full("d", "DM");
    let notes = dimension_annotations::<Toy>(&case, &pose(), Tone::Info);
    let (from, to, offset, value, binding) = dimension_of(&notes, "d:dim-x");
    assert!((from.x - 7.0).abs() < 1e-9 && (to.x - 7.04).abs() < 1e-9 && (to.z - 0.8).abs() < 1e-9 && (from.y - 20.0).abs() < 1e-9, "40 mm from the start face of the 6 m member: {from:?} {to:?}");
    assert!(offset.y < -0.29 && value == Quantity::new(QuantityKind::Length, 0.04));
    let binding = binding.expect("binding");
    assert_eq!(binding.path, "cases[id=d].numbers[id=e].value");
    assert!((binding.map.base - 40.0).abs() < 1e-12 && (binding.map.k - 1000.0).abs() < 1e-9);
    assert!(notes.iter().any(|a| matches!(a, Annotation::Label { id, text, .. } if id == "d:dim-x:label" && text.en.contains("End distance 40") && text.de.contains("Endabstand 40"))));
    let grip = grip_of(&case, "dim-x");
    assert!(matches!(grip.kind, HandleKind::ScalarAlong { .. }) && grip.writes[0].path == "cases[id=d].numbers[id=e].value");
    assert!((grip.writes[0].map.apply(0.01) - 50.0).abs() < 1e-9, "10 mm more drag is 10 mm more distance");
    assert!((grip.anchor.x - 7.04).abs() < 1e-9 && (grip.anchor.z - 0.8).abs() < 1e-9, "the grip stands at the marker: {:?}", grip.anchor);
    let drawn = draw::<Toy>(&case, &pose(), &report("d", 50.0), "cases", "cases", &MaterialRole::Timber).expect("draw");
    let marker = drawn.entities.iter().find(|e| e.entity.id == "d:dim-x-mark").expect("marker");
    assert!(marker.utilisation.is_none() && marker.appearance.opacity < 1.0 && marker.handles.is_empty());
    assert!(drawn.entities[0].handles.contains(&"d:dim-x".to_string()) && drawn.notes.iter().any(|a| a.id() == "d:dim-x"));
}

#[test]
fn a_dimension_from_the_far_end_or_the_top_runs_against_its_axis() {
    let case = full("d", "DM");
    let notes = dimension_annotations::<Toy>(&case, &pose(), Tone::Info);
    let (from, to, ..) = dimension_of(&notes, "d:dim-ye");
    assert!((from.y - 20.15).abs() < 1e-9 && (to.y - 20.05).abs() < 1e-9, "0.1 m from the far side towards the middle: {from:?} {to:?}");
    let (from, to, ..) = dimension_of(&notes, "d:dim-ze");
    assert!((from.z - 0.8).abs() < 1e-9 && (to.z - 0.8 + 8.0).abs() < 1e-9, "8 m down from the top face: {from:?} {to:?}");
    let far = grip_of(&case, "dim-ye");
    assert!(matches!(far.kind, HandleKind::ScalarAlong { ref axis } if axis.y < -0.99), "{:?}", far.kind);
}

#[test]
fn an_angular_dimension_is_a_sector_between_the_grain_and_the_load_direction_with_an_angular_grip() {
    let case = full("d", "DM");
    let notes = dimension_annotations::<Toy>(&case, &pose(), Tone::Info);
    let (from, to, _, value, binding) = dimension_of(&notes, "d:dim-angle");
    let chord = 2.0 * ((to.z - from.z).atan2(to.x - from.x) - std::f64::consts::FRAC_PI_2);
    assert!((chord - 30.0_f64.to_radians()).abs() < 1e-9 && value == Quantity::new(QuantityKind::Angle, 30.0_f64.to_radians()), "{chord}");
    let binding = binding.expect("binding");
    assert!((binding.map.base - 30.0).abs() < 1e-12 && (binding.map.k - 180.0 / std::f64::consts::PI).abs() < 1e-9);
    let grip = grip_of(&case, "dim-angle");
    assert!(matches!(grip.kind, HandleKind::Angular { .. }) && grip.writes[0].path == "cases[id=d].numbers[id=angle].value");
    assert!((grip.writes[0].map.apply(45.0_f64.to_radians()) - 45.0).abs() < 1e-9, "the grip writes the absolute angle in degrees");
    assert!((grip.anchor.x - (10.0 + 1.8 * 30.0_f64.to_radians().cos())).abs() < 1e-9 && (grip.anchor.z - (0.8 + 0.9)).abs() < 1e-9, "the grip stands on the load ray: {:?}", grip.anchor);
    let drawn = draw::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw");
    let sector = drawn.entities.iter().find(|e| e.entity.id == "d:dim-angle-arc").expect("sector");
    assert!(matches!(&sector.geometry, Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { .. } })) && sector.handles.is_empty());
}

#[test]
fn an_annotation_role_with_an_unset_input_draws_and_grips_nothing_and_every_form_accepts_the_roles() {
    let case = row("u", "DM", &[("b", 300.0), ("l", 6.0), ("h", 0.5)]);
    assert!(dimension_annotations::<Toy>(&case, &pose(), Tone::Info).is_empty());
    assert!(!tags(&case).iter().any(|tag| tag.starts_with("dim-")));
    let drawn = draw::<Toy>(&case, &pose(), &CheckReport::default(), "cases", "cases", &MaterialRole::Timber).expect("draw");
    assert!(drawn.entities.iter().all(|e| !e.entity.id.contains("dim-")) && drawn.notes.is_empty());
    assert_eq!((dimension_role("dim_x2"), dimension_role("dim_ze"), dimension_role("dim_angle1"), dimension_role("grain")), (Some(Stem::FromStart), Some(Stem::FromTop), Some(Stem::Angle), None));
    assert_eq!(dimension_tag("dim_x2"), "dim-x2");
    assert!(is_aux_role(CaseForm::Footing, "dim_z3") && is_aux_role(CaseForm::Member, "grain") && !is_aux_role(CaseForm::Footing, "grain"));
    let rows: Vec<CaseRow> = [1.0, 2.0].iter().map(|k| row(&format!("k{k}"), "DM", &[("b", 300.0), ("l", 6.0), ("h", 0.5), ("e", 40.0 * k), ("angle", 30.0 * k), ("a2", 0.1), ("r", 8.0)])).collect();
    let metres = |case: &CaseRow| { let (from, to, ..) = dimension_of(&dimension_annotations::<Toy>(case, &pose(), Tone::Info), &format!("{}:dim-x", case.id)); ((to.x - from.x).powi(2) + (to.y - from.y).powi(2)).sqrt() };
    assert!((metres(&rows[1]) / metres(&rows[0]) - 2.0).abs() < 1e-9, "a doubled input doubles the dimension line");
}
