use super::*;
use crate::standards::v1::subsets::any::io::binary::snapshot as pack;
use crate::standards::v1::subsets::any::io::export::svg::testkit::{house, read, svg, tags, HOUSE_DIR};
use crate::standards::v1::subsets::any::io::io;
use crate::standards::v1::subsets::any::schema::inferences::view_linework::compute_view_linework;
use crate::ViewKind;
use semio_framework_os_kernel::io::io_mechanism::IoEntryDirection;

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
    assert_eq!((root.attribute("data-unit"), root.attribute("class")), (Some("mm"), Some("sheet")));
    let sheet = sheet::layout(&model, &compute_view_linework(&model));
    assert!((view[2] - sheet.width).abs() < 1e-3 && (view[3] - sheet.height).abs() < 1e-3);
}

#[test]
fn there_is_one_group_per_drawn_view_with_an_id_a_label_a_kind_a_scale_and_the_paths_of_its_drawing() {
    let model = house();
    let found = tags(&svg(&model));
    let groups: Vec<_> = found.iter().filter(|tag| tag.name == "g" && tag.has_class("view")).collect();
    let drawn: Vec<_> = model.views.iter().filter(|(_, view)| sheet::is_drawn(view.kind)).collect();
    assert_eq!(groups.len(), drawn.len());
    assert!(drawn.len() > model.storeys.len(), "the sheet carries the sections and elevations besides the plans");
    for group in &groups {
        let id = group.attribute("data-view").unwrap();
        let view = &model.views[id];
        assert_eq!(group.attribute("id"), Some(format!("view-{id}").as_str()));
        assert_eq!(group.attribute("aria-label"), Some(view.name.as_str()));
        assert_eq!((group.attribute("data-scale"), group.attribute("data-building")), (Some(view.scale.to_string().as_str()), Some(view.building.as_str())));
        assert_eq!(group.attribute("data-storey"), view.storey.as_deref());
        assert!(group.has_class(style::view_class(view.kind)));
        assert_eq!(group.depth, 1);
    }
    let mut ids: Vec<_> = groups.iter().map(|group| group.attribute("id").unwrap()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), groups.len());
    assert!(model.views.values().any(|view| view.kind == ViewKind::Perspective), "a camera is kept in the model");
    assert!(groups.iter().all(|group| !group.has_class("perspective")), "and is not drawn");
}

#[test]
fn the_path_counts_per_view_equal_the_regions_and_polylines_of_its_drawing() {
    let model = house();
    let drawings = compute_view_linework(&model);
    let found = tags(&svg(&model));
    let mut current = String::new();
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for tag in &found {
        if tag.name == "g" && tag.has_class("view") {
            current = tag.attribute("data-view").unwrap().to_string();
        }
        if tag.name == "path" {
            *counts.entry(current.clone()).or_default() += 1;
        }
    }
    assert_eq!(counts.len(), model.views.values().filter(|view| sheet::is_drawn(view.kind)).count());
    for (view, count) in &counts {
        let lines = &drawings[view].lines;
        assert_eq!(*count, lines.regions.len() + lines.polylines.len(), "{view}");
    }
}

#[test]
fn a_section_draws_its_cut_as_poche_and_an_elevation_draws_edges_and_datums_but_no_cut() {
    let found = tags(&svg(&house()));
    let mut kind = String::new();
    let mut seen: BTreeMap<(String, &str), usize> = BTreeMap::new();
    for tag in &found {
        if tag.name == "g" && tag.has_class("view") {
            kind = tag.attribute("data-kind").unwrap().to_string();
        }
        for class in ["section-cut", "edge", "datum", "silhouette"] {
            if tag.name == "path" && tag.has_class(class) {
                *seen.entry((kind.clone(), class)).or_default() += 1;
            }
        }
    }
    assert!(seen.contains_key(&("section".to_string(), "section-cut")));
    assert!(!seen.contains_key(&("elevation".to_string(), "section-cut")));
    assert!(seen.contains_key(&("elevation".to_string(), "edge")) && seen.contains_key(&("elevation".to_string(), "silhouette")));
    assert!(seen.contains_key(&("section".to_string(), "datum")) && seen.contains_key(&("elevation".to_string(), "datum")));
    assert!(!seen.contains_key(&("plan".to_string(), "edge")));
}

#[test]
fn a_model_without_views_exports_an_empty_sheet_and_a_view_at_another_scale_gets_a_bigger_slot() {
    let mut model = house();
    let at_100 = sheet::layout(&model, &compute_view_linework(&model));
    model.views.get_mut("v-plan-st-ground").expect("the ground plan").scale = 50;
    let at_50 = sheet::layout(&model, &compute_view_linework(&model));
    let slot = |layout: &sheet::Layout| layout.slots.iter().find(|slot| slot.view == "v-plan-st-ground").cloned().expect("the slot");
    assert!(slot(&at_50).width > slot(&at_100).width && slot(&at_50).height > slot(&at_100).height);
    assert_eq!(slot(&at_50).frame.mm, 20.0);
    assert!(tags(&svg(&model)).iter().any(|tag| tag.attribute("data-view") == Some("v-plan-st-ground") && tag.attribute("data-scale") == Some("50")));
    model.views.clear();
    let found = tags(&svg(&model));
    assert!(found.iter().all(|tag| !(tag.name == "g" && tag.has_class("view"))));
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
fn the_export_is_deterministic_and_a_model_without_views_is_a_valid_empty_sheet() {
    let model = house();
    assert_eq!(svg(&model), svg(&model));
    let empty = svg(&ModelSnapshot::default());
    let found = tags(&empty);
    assert!(found.iter().all(|tag| !(tag.name == "g" && tag.has_class("view"))));
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

#[test]
fn the_committed_notated_file_is_the_current_export() {
    use crate::standards::v1::subsets::any::io::export::svg::testkit::{notated, read_notated, NOTATED_DIR};
    let text = svg(&notated());
    if std::env::var("BIM_BLESS").is_ok() {
        std::fs::create_dir_all(NOTATED_DIR).expect("the fixture directory");
        std::fs::write(format!("{NOTATED_DIR}/🪧️notated.svg"), text.as_bytes()).expect("the file is written");
    }
    assert_eq!(String::from_utf8(read_notated("🪧️notated.svg")).expect("UTF-8"), text, "the committed export drifted: rewrite it with BIM_BLESS=1");
}

#[test]
fn a_plan_with_annotations_gets_an_annotation_layer_and_a_plan_without_gets_none() {
    use crate::standards::v1::subsets::any::io::export::svg::testkit::notated;
    assert!(tags(&svg(&house())).iter().all(|tag| !tag.has_class("annotations")), "the house has no annotations and no annotation layer");
    let found = tags(&svg(&notated()));
    let at = found.iter().position(|tag| tag.name == "g" && tag.has_class("annotations")).expect("the annotated storey has the layer");
    let depth = found[at].depth;
    let inside: Vec<_> = found[at + 1..].iter().take_while(|tag| tag.depth > depth).collect();
    let count = |name: &str, kind: &str| inside.iter().filter(|tag| tag.name == name && tag.has_class(kind)).count();
    assert_eq!((count("path", "dimension-line"), count("text", "dimension-text"), count("text", "tag-text"), count("text", "note-text")), (7, 8, 4, 1));
    assert_eq!((count("path", "leader-line"), count("text", "leader-text")), (1, 1));
    assert!(count("path", "dimension-extension") > 0 && count("path", "dimension-mark") > 0);
    assert!(inside.iter().all(|tag| tag.has_class("annotation")), "every primitive of the layer is drawn in the annotation line style");
    let outside = found.iter().enumerate().filter(|(index, tag)| (*index <= at || *index > at + inside.len()) && (tag.name == "path" || tag.name == "text"));
    assert!(outside.into_iter().all(|(_, tag)| !["dimension-line", "dimension-text", "tag-text", "note-text", "leader-line"].iter().any(|kind| tag.has_class(kind))), "no annotation primitive outside its layer");
    assert!(inside.iter().filter(|tag| tag.name == "text").all(|tag| tag.attribute("text-anchor") == Some("middle") && tag.attribute("font-size").is_some()), "texts sit by the middle of their baseline and carry their own size");
}
