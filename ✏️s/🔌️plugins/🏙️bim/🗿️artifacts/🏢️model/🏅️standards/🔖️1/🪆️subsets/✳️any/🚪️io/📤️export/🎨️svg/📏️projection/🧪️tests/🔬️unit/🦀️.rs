use super::*;
use crate::standards::v1::subsets::any::io::export::svg::testkit::{house, read, svg, tags};
use crate::standards::v1::subsets::any::io::export::svg::style::STYLE_CLASSES;
use crate::standards::v1::subsets::any::schema::inferences::view_linework::compute_view_linework;
use crate::standards::v1::subsets::any::io::export::svg::sheet::is_drawn;

#[test]
fn the_report_counts_the_primitives_of_every_drawn_view() {
    let model = house();
    let drawings = compute_view_linework(&model);
    let report = project(&model, &drawings);
    assert_eq!(report.views.len(), model.views.values().filter(|view| is_drawn(view.kind)).count());
    for (id, row) in &report.views {
        let plan = &drawings[id].lines;
        assert_eq!((row.regions, row.lines, row.texts), (plan.regions.len(), plan.polylines.len(), plan.texts.len()), "{id}");
        assert_eq!(row.styles.values().sum::<usize>(), plan.regions.len() + plan.polylines.len(), "{id}");
        assert_eq!(row.styles.keys().map(String::as_str).collect::<Vec<_>>(), { let mut sorted = STYLE_CLASSES.to_vec(); sorted.sort(); sorted }, "{id}");
    }
    let kinds: Vec<_> = report.views.values().map(|row| row.kind.as_str()).collect();
    assert_eq!(kinds.iter().filter(|kind| **kind == "plan").count(), model.storeys.len());
    assert_eq!(kinds.iter().filter(|kind| **kind == "elevation").count(), 4);
    assert_eq!(kinds.iter().filter(|kind| **kind == "section").count(), 2);
    assert!(report.views.values().all(|row| row.scale == 100));
}

#[test]
fn the_report_agrees_with_what_a_third_party_xml_reader_counts_in_the_written_file() {
    let model = house();
    let report = projection(&model);
    let found = tags(&svg(&model));
    let groups: Vec<_> = found.iter().filter(|tag| tag.name == "g" && tag.has_class("view")).collect();
    assert_eq!(groups.len(), report.views.len());
    let mut current = String::new();
    let mut counted: std::collections::BTreeMap<String, (usize, usize, usize, usize)> = Default::default();
    for tag in &found {
        if tag.name == "g" && tag.has_class("view") {
            current = tag.attribute("data-view").unwrap().to_string();
        }
        let row = counted.entry(current.clone()).or_default();
        match tag.name.as_str() {
            "path" if tag.has_class("region") => row.0 += 1,
            "path" if tag.has_class("line") => row.1 += 1,
            "text" if !tag.has_class("title") => row.2 += 1,
            _ => {}
        }
        if tag.name == "path" {
            row.3 += tag.attribute("d").unwrap().matches('A').count();
        }
    }
    for (id, row) in &report.views {
        assert_eq!(counted[id], (row.regions, row.lines, row.texts, row.arcs), "{id}");
    }
}

#[test]
fn only_straight_geometry_is_measured_and_the_house_has_curved_walls_that_are_only_counted() {
    let model = house();
    let report = projection(&model);
    let arcs: usize = report.views.values().map(|row| row.arcs).sum();
    assert!(arcs > 0, "the house has an arc wall");
    let ground = &report.views["v-plan-st-ground"];
    assert!(ground.poche_area > 1.0 && ground.poche_area < 200.0, "{}", ground.poche_area);
    assert!(ground.line_length["cut"] > 0.0);
}

#[test]
fn the_json_form_parses_and_keeps_the_numbers() {
    let report = projection(&house());
    let parsed: serde_json::Value = serde_json::from_str(&report.to_json()).expect("valid JSON");
    assert_eq!(parsed["width"].as_f64(), Some(report.width));
    assert_eq!(parsed["views"]["v-plan-st-ground"]["regions"].as_u64(), Some(report.views["v-plan-st-ground"].regions as u64));
    assert_eq!(parsed["views"]["v-plan-st-ground"]["pocheArea"].as_f64(), Some(report.views["v-plan-st-ground"].poche_area));
    assert_eq!(parsed["views"]["v-plan-st-ground"]["styles"]["cut"].as_u64(), Some(report.views["v-plan-st-ground"].styles["cut"] as u64));
}

#[test]
fn the_subject_report_equals_the_table_the_lxml_and_shapely_oracle_measured_from_the_committed_file() {
    let oracle: serde_json::Value = serde_json::from_slice(&read("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&house()).to_json()).unwrap();
    assert_eq!(oracle["views"].as_object().unwrap().len(), ours["views"].as_object().unwrap().len());
    for (id, row) in ours["views"].as_object().unwrap() {
        let measured = &oracle["views"][id];
        for key in ["name", "kind", "scale", "regions", "lines", "texts", "styles", "arcs"] {
            assert_eq!(measured[key], row[key], "{id}.{key}");
        }
        assert!((measured["pocheArea"].as_f64().unwrap() - row["pocheArea"].as_f64().unwrap()).abs() < 1e-9, "{id}: poché area");
        for class in STYLE_CLASSES {
            assert!((measured["lineLength"][class].as_f64().unwrap() - row["lineLength"][class].as_f64().unwrap()).abs() < 1e-9, "{id}: {class} length");
        }
    }
    for key in ["width", "height"] {
        assert!((oracle[key].as_f64().unwrap() - ours[key].as_f64().unwrap()).abs() < 1e-9, "{key}");
    }
}

#[test]
fn the_sampled_poche_area_of_curved_regions_agrees_with_the_closed_form_of_the_bulges() {
    let oracle: serde_json::Value = serde_json::from_slice(&read("🔬️measure/🔣️.json")).expect("the oracle table");
    let model = house();
    let drawings = compute_view_linework(&model);
    for (id, plan) in drawings.iter().map(|(id, drawing)| (id, &drawing.lines)) {
        let exact: f64 = plan
            .regions
            .iter()
            .filter(|region| region.style == PlanStyle::Cut)
            .map(|region| {
                let area = |ring: &[crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanVertex]| semio_framework_geometry::loops::area(&ring.iter().map(|v| semio_framework_geometry::loops::Vertex::new(semio_framework_geometry::Point::new(v.x, v.y), v.bulge)).collect::<Vec<_>>());
                area(&region.outer) - region.holes.iter().map(|hole| area(hole)).sum::<f64>()
            })
            .sum();
        let sampled = oracle["audit"]["sampledPocheArea"][id].as_f64().expect("a sampled area");
        assert!((sampled - exact).abs() <= 1e-4 * exact.abs().max(1.0), "{id}: sampled {sampled}, closed form {exact}");
    }
}

fn shoelace(vertices: &[crate::standards::v1::subsets::any::schema::inferences::plan_linework::PlanVertex]) -> f64 {
    let at = |index: usize| vertices[index % vertices.len()];
    (0..vertices.len()).map(|index| at(index).x * at(index + 1).y - at(index + 1).x * at(index).y).sum::<f64>().abs() / 2.0
}

#[test]
fn the_written_geometry_measures_what_the_plan_measures_in_metres_at_the_drawing_scale() {
    let model = house();
    let drawings = compute_view_linework(&model);
    let report = project(&model, &drawings);
    for (id, plan) in drawings.iter().map(|(id, drawing)| (id, &drawing.lines)).filter(|(id, _)| report.views.contains_key(*id)) {
        let row = &report.views[id];
        let area: f64 = plan.regions.iter().filter(|region| region.style == PlanStyle::Cut && !std::iter::once(&region.outer).chain(region.holes.iter()).any(|ring| ring.iter().any(|vertex| vertex.bulge != 0.0))).map(|region| shoelace(&region.outer) - region.holes.iter().map(|hole| shoelace(hole)).sum::<f64>()).sum();
        assert!((row.poche_area - area).abs() < 1e-3, "{id}: written {} m2, plan {area} m2", row.poche_area);
        for class in STYLE_CLASSES {
            let length: f64 = plan
                .polylines
                .iter()
                .filter(|line| style_class(line.style) == class && line.vertices.iter().all(|vertex| vertex.bulge == 0.0))
                .map(|line| {
                    let spans = if line.closed { line.vertices.len() } else { line.vertices.len() - 1 };
                    (0..spans).map(|index| (line.vertices[(index + 1) % line.vertices.len()].x - line.vertices[index].x).hypot(line.vertices[(index + 1) % line.vertices.len()].y - line.vertices[index].y)).sum::<f64>()
                })
                .sum();
            assert!((row.line_length[class] - length).abs() < 1e-3 * (1.0 + plan.polylines.len() as f64), "{id} {class}: written {} m, plan {length} m", row.line_length[class]);
        }
    }
}

#[test]
fn the_notation_of_the_notated_room_equals_the_table_the_lxml_and_shapely_oracle_measured_from_its_file() {
    use crate::standards::v1::subsets::any::io::export::svg::testkit::{notated, read_notated};
    let oracle: serde_json::Value = serde_json::from_slice(&read_notated("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&notated()).to_json()).unwrap();
    assert_eq!(oracle["views"].as_object().unwrap().len(), ours["views"].as_object().unwrap().len());
    for (id, row) in ours["views"].as_object().unwrap() {
        let measured = &oracle["views"][id];
        for key in ["name", "kind", "scale", "regions", "lines", "texts", "styles", "arcs", "printed"] {
            assert_eq!(measured[key], row[key], "{id}.{key}");
        }
        assert_eq!(measured["notation"].as_object().unwrap().keys().collect::<Vec<_>>(), row["notation"].as_object().unwrap().keys().collect::<Vec<_>>(), "{id}: the kinds");
        for (kind, found) in row["notation"].as_object().unwrap() {
            assert_eq!(measured["notation"][kind]["count"], found["count"], "{id}.{kind}: count");
            assert!((measured["notation"][kind]["length"].as_f64().unwrap() - found["length"].as_f64().unwrap()).abs() < 1e-9, "{id}.{kind}: length");
        }
    }
    let ground = &ours["views"]["v-plan-st-ground"];
    assert_eq!(ground["notation"]["dimension-line"]["count"], 7);
    assert!((ground["notation"]["dimension-line"]["length"].as_f64().unwrap() - (8.0 + 8.0 + 5.7 + 0.3 + 3.0 + 3.0 + 3.0)).abs() < 1e-2, "the lines span the measured distances up to the snapping of the sheet");
    assert!(ground["printed"].as_array().unwrap().iter().any(|text| text == "8.00") && ground["printed"].as_array().unwrap().iter().any(|text| text == "Verify on site"));
}

#[test]
fn the_house_has_no_notation() {
    let report = projection(&house());
    assert!(report.views.values().all(|row| row.notation.is_empty() && row.printed.is_empty()));
}

#[test]
fn a_section_cuts_poche_a_building_elevation_cuts_nothing_and_both_draw_edges() {
    let report = projection(&house());
    let (section, elevation) = (&report.views["v-section-a"], &report.views["v-elevation-south"]);
    assert!(section.poche_area > 1.0 && section.line_length["projection"] > 1.0, "{section:?}");
    assert_eq!((elevation.poche_area, elevation.kind.as_str()), (0.0, "elevation"));
    assert!(elevation.line_length["projection"] > 1.0 && elevation.regions > 0);
}

#[test]
fn a_view_is_drawn_at_its_own_scale_so_halving_the_scale_doubles_every_written_length() {
    let mut model = house();
    let base = projection(&model);
    model.views.get_mut("v-elevation-south").expect("the south elevation").scale = 50;
    let finer = projection(&model);
    let (before, after) = (&base.views["v-elevation-south"], &finer.views["v-elevation-south"]);
    assert_eq!(after.scale, 50);
    assert_eq!((before.regions, before.lines), (after.regions, after.lines));
    assert!((after.line_length["projection"] - before.line_length["projection"]).abs() < 1e-3, "lengths are reported in model metres at any scale");
    assert!(finer.height > base.height);
}
