use super::*;
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::export::svg::testkit::{house, read, svg, tags, HOUSE_DIR};
use crate::standards::v1::subsets::any::io::io;
use semio_framework::io::io_mechanism::IoEntryDirection;

#[test]
fn the_declaration_lists_the_svg_export_between_the_bim_and_the_svg_dialect() {
    let declaration = io();
    let entry = declaration.entries.iter().find(|entry| entry.into == SVG_DIALECT).expect("the SVG entry");
    assert_eq!((entry.from, entry.direction), (crate::BIM_MODEL_DIALECT, IoEntryDirection::Export));
    assert_eq!((SVG_DIALECT.artifact_kind, SVG_DIALECT.standard.0), ("s.stdio.svg", "1.1"));
}

#[test]
fn the_export_entry_turns_a_packed_model_into_svg_text() {
    let model = house();
    let declaration = io();
    let entry = declaration.entries.iter().find(|entry| entry.into == SVG_DIALECT).expect("the SVG entry");
    let produced = (entry.run)(&IoPayload::Binary(pack::encode(&model))).expect("the entry runs");
    assert_eq!(produced.value, IoPayload::Text(svg(&model)));
    assert!(produced.diagnostics.is_empty());
}

#[test]
fn the_document_is_well_formed_svg_1_1_with_a_viewbox_that_fits_the_sheet_in_millimetres() {
    let model = house();
    let text = svg(&model);
    assert!(text.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<svg "));
    let found = tags(&text);
    let root = &found[0];
    assert_eq!((root.name.as_str(), root.attribute("xmlns"), root.attribute("version")), ("svg", Some("http://www.w3.org/2000/svg"), Some("1.1")));
    let view: Vec<f64> = root.attribute("viewBox").unwrap().split(' ').map(|part| part.parse().unwrap()).collect();
    assert_eq!((view[0], view[1]), (0.0, 0.0));
    assert_eq!(root.attribute("width"), Some(format!("{}mm", view[2]).as_str()));
    assert_eq!(root.attribute("height"), Some(format!("{}mm", view[3]).as_str()));
    assert_eq!((root.attribute("data-scale"), root.attribute("data-unit")), (Some("100"), Some("mm")));
    let sheet = sheet::layout(&model, &compute_plan_linework(&model));
    assert!((view[2] - sheet.width).abs() < 1e-3 && (view[3] - sheet.height).abs() < 1e-3);
}

#[test]
fn there_is_one_group_per_storey_with_an_id_a_label_and_the_paths_of_its_plan() {
    let model = house();
    let found = tags(&svg(&model));
    let groups: Vec<_> = found.iter().filter(|tag| tag.name == "g" && tag.has_class("storey")).collect();
    assert_eq!(groups.len(), model.storeys.len());
    for group in &groups {
        let storey = group.attribute("data-storey").unwrap();
        assert!(model.storeys.contains_key(storey));
        assert_eq!(group.attribute("id"), Some(format!("storey-{storey}").as_str()));
        assert_eq!(group.attribute("aria-label"), Some(model.storeys[storey].name.as_str()));
        assert_eq!(group.depth, 1);
    }
    let mut ids: Vec<_> = groups.iter().map(|group| group.attribute("id").unwrap()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), groups.len());
}

#[test]
fn the_path_counts_per_storey_equal_the_regions_and_polylines_of_its_plan() {
    let model = house();
    let plans = compute_plan_linework(&model);
    let found = tags(&svg(&model));
    let mut current = String::new();
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for tag in &found {
        if tag.name == "g" && tag.has_class("storey") {
            current = tag.attribute("data-storey").unwrap().to_string();
        }
        if tag.name == "path" {
            *counts.entry(current.clone()).or_default() += 1;
        }
    }
    for (storey, plan) in &plans {
        assert_eq!(counts[storey], plan.regions.len() + plan.polylines.len(), "{storey}");
    }
}

#[test]
fn every_line_style_class_is_used_by_the_house_and_defined_in_the_style_sheet() {
    let text = svg(&house());
    let found = tags(&text);
    for class in ["cut", "projection", "hidden", "annotation"] {
        assert!(found.iter().any(|tag| tag.name == "path" && tag.has_class(class)), "{class}");
        assert!(text.contains(&format!(".{class}{{stroke-width:")), "{class}");
    }
    assert!(found.iter().any(|tag| tag.name == "path" && tag.has_class("region") && tag.has_class("cut")));
    assert!(found.iter().any(|tag| tag.name == "text" && tag.has_class("space-tag")));
    assert!(found.iter().any(|tag| tag.name == "text" && tag.has_class("grid-label")));
}

#[test]
fn the_export_is_deterministic_and_a_model_without_storeys_is_a_valid_empty_sheet() {
    let model = house();
    assert_eq!(svg(&model), svg(&model));
    let empty = svg(&ModelSnapshot::default());
    let found = tags(&empty);
    assert!(found.iter().all(|tag| !(tag.name == "g" && tag.has_class("storey"))));
    assert_eq!(found[0].attribute("viewBox"), Some("0 0 20 20"));
}

#[test]
fn the_committed_house_file_is_the_current_export() {
    let text = svg(&house());
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(HOUSE_DIR).expect("the fixture directory");
        std::fs::write(format!("{HOUSE_DIR}/🏠️house.svg"), text.as_bytes()).expect("the file is written");
    }
    assert_eq!(String::from_utf8(read("🏠️house.svg")).expect("UTF-8"), text, "the committed export drifted: rewrite it with BIM_BLESS=1");
}
