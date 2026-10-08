use super::*;
use crate::standards::v1::subsets::any::io::export::svg::testkit::{house, read, svg, tags};
use crate::standards::v1::subsets::any::io::export::svg::style::STYLE_CLASSES;
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::compute_plan_linework;

#[test]
fn the_report_counts_the_primitives_of_every_storey_plan() {
    let model = house();
    let plans = compute_plan_linework(&model);
    let report = project(&model, &plans);
    assert_eq!(report.storeys.len(), model.storeys.len());
    for (id, row) in &report.storeys {
        let plan = &plans[id];
        assert_eq!((row.regions, row.lines, row.texts), (plan.regions.len(), plan.polylines.len(), plan.texts.len()), "{id}");
        assert_eq!(row.styles.values().sum::<usize>(), plan.regions.len() + plan.polylines.len(), "{id}");
        assert_eq!(row.styles.keys().map(String::as_str).collect::<Vec<_>>(), { let mut sorted = STYLE_CLASSES.to_vec(); sorted.sort(); sorted }, "{id}");
    }
    assert_eq!(report.storeys["st-ground"].level, 0);
}

#[test]
fn the_report_agrees_with_what_a_third_party_xml_reader_counts_in_the_written_file() {
    let model = house();
    let report = projection(&model);
    let found = tags(&svg(&model));
    let groups: Vec<_> = found.iter().filter(|tag| tag.name == "g" && tag.has_class("storey")).collect();
    assert_eq!(groups.len(), report.storeys.len());
    let mut current = String::new();
    let mut counted: std::collections::BTreeMap<String, (usize, usize, usize, usize)> = Default::default();
    for tag in &found {
        if tag.name == "g" && tag.has_class("storey") {
            current = tag.attribute("data-storey").unwrap().to_string();
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
    for (id, row) in &report.storeys {
        assert_eq!(counted[id], (row.regions, row.lines, row.texts, row.arcs), "{id}");
    }
}

#[test]
fn only_straight_geometry_is_measured_and_the_house_has_curved_walls_that_are_only_counted() {
    let model = house();
    let report = projection(&model);
    let arcs: usize = report.storeys.values().map(|row| row.arcs).sum();
    assert!(arcs > 0, "the house has an arc wall");
    let ground = &report.storeys["st-ground"];
    assert!(ground.poche_area > 1.0 && ground.poche_area < 200.0, "{}", ground.poche_area);
    assert!(ground.line_length["cut"] > 0.0);
}

#[test]
fn the_json_form_parses_and_keeps_the_numbers() {
    let report = projection(&house());
    let parsed: serde_json::Value = serde_json::from_str(&report.to_json()).expect("valid JSON");
    assert_eq!(parsed["width"].as_f64(), Some(report.width));
    assert_eq!(parsed["storeys"]["st-ground"]["regions"].as_u64(), Some(report.storeys["st-ground"].regions as u64));
    assert_eq!(parsed["storeys"]["st-ground"]["pocheArea"].as_f64(), Some(report.storeys["st-ground"].poche_area));
    assert_eq!(parsed["storeys"]["st-ground"]["styles"]["cut"].as_u64(), Some(report.storeys["st-ground"].styles["cut"] as u64));
}

#[test]
fn the_subject_report_equals_the_table_the_lxml_and_shapely_oracle_measured_from_the_committed_file() {
    let oracle: serde_json::Value = serde_json::from_slice(&read("🔬️measure/🔣️.json")).expect("the oracle table");
    let ours: serde_json::Value = serde_json::from_str(&projection(&house()).to_json()).unwrap();
    assert_eq!(oracle["storeys"].as_object().unwrap().len(), ours["storeys"].as_object().unwrap().len());
    for (id, row) in ours["storeys"].as_object().unwrap() {
        let measured = &oracle["storeys"][id];
        for key in ["name", "level", "regions", "lines", "texts", "styles", "arcs"] {
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
    let plans = compute_plan_linework(&model);
    for (id, plan) in &plans {
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
    let plans = compute_plan_linework(&model);
    let report = project(&model, &plans);
    for (id, plan) in &plans {
        let row = &report.storeys[id];
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
