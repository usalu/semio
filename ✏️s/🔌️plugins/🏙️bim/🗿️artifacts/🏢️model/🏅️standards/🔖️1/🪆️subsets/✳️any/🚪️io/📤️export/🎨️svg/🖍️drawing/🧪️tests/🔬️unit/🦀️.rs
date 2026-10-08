use super::*;
use crate::standards::v1::subsets::any::io::export::svg::codec::{document_text, PathCommand, ViewBox};
use crate::standards::v1::subsets::any::io::export::svg::sheet::layout;
use crate::standards::v1::subsets::any::io::export::svg::style::STYLE_CLASSES;
use crate::standards::v1::subsets::any::io::export::svg::testkit::{house, tags};
use crate::standards::v1::subsets::any::schema::inferences::plan_linework::compute_plan_linework;

fn ground() -> (SvgElement, PlanLinework, Slot) {
    let model = house();
    let plans = compute_plan_linework(&model);
    let slot = layout(&model, &plans).slots.into_iter().find(|slot| slot.storey == "st-ground").expect("the ground slot");
    let plan = plans["st-ground"].clone();
    (storey_group(&slot, &plan), plan, slot)
}

fn parts(element: &SvgElement) -> (&CommonAttrs, &[SvgElement]) {
    match element {
        SvgElement::Group { common, children } => (common, children),
        other => panic!("a group, got {other:?}"),
    }
}

fn layer<'a>(group: &'a SvgElement, class: &str) -> &'a [SvgElement] {
    parts(group).1.iter().find_map(|child| matches!(child, SvgElement::Group { common, .. } if common.class.as_deref() == Some(class)).then(|| parts(child).1)).expect("a layer")
}

fn extra<'a>(common: &'a CommonAttrs, name: &str) -> Option<&'a str> {
    common.extra_attrs.iter().find(|attribute| attribute.name == name).map(|attribute| attribute.value.as_str())
}

fn class_of(element: &SvgElement) -> &str {
    match element {
        SvgElement::Path { common, .. } | SvgElement::Text { common, .. } => common.class.as_deref().expect("a class"),
        other => panic!("a path or text, got {other:?}"),
    }
}

fn document_of(group: SvgElement) -> String {
    let root = SvgElement::Svg { common: CommonAttrs::new(), view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 1.0, height: 1.0 }), width: None, height: None, xmlns: Some("http://www.w3.org/2000/svg".into()), children: vec![group] };
    document_text(&root).expect("the writer accepts the group")
}

#[test]
fn ids_keep_safe_characters_and_replace_the_rest() {
    assert_eq!(xml_id("storey", "st-ground"), "storey-st-ground");
    assert_eq!(xml_id("storey", "a b:c/d"), "storey-a_b_c_d");
}

#[test]
fn the_group_names_its_storey_and_carries_a_title_for_assistive_technology() {
    let (group, _, slot) = ground();
    let (common, children) = parts(&group);
    assert_eq!(common.id.as_deref(), Some("storey-st-ground"));
    assert_eq!((common.class.as_deref(), extra(common, "data-storey"), extra(common, "data-level")), (Some("storey"), Some("st-ground"), Some("0")));
    assert_eq!(extra(common, "aria-label"), Some(slot.name.as_str()));
    assert!(matches!(common.transform.as_deref(), Some([TransformOp::Translate { .. }])));
    assert!(matches!(&children[0], SvgElement::Unknown { name, .. } if name == "title"));
}

#[test]
fn one_path_per_region_and_per_polyline_and_one_text_per_text_anchor() {
    let (group, plan, _) = ground();
    assert_eq!(layer(&group, "layer regions").len(), plan.regions.len());
    assert_eq!(layer(&group, "layer lines").len(), plan.polylines.len());
    assert_eq!(layer(&group, "layer texts").len(), plan.texts.len());
    assert!(!plan.regions.is_empty() && !plan.polylines.is_empty() && !plan.texts.is_empty());
}

#[test]
fn paths_are_painted_hidden_under_projection_under_cut_under_annotation() {
    let (group, _, _) = ground();
    for name in ["layer regions", "layer lines"] {
        let ranks: Vec<usize> = layer(&group, name).iter().map(|path| STYLE_CLASSES.iter().position(|style| class_of(path).split(' ').any(|word| word == *style)).expect("a style class")).collect();
        assert!(ranks.windows(2).all(|pair| pair[0] <= pair[1]), "{name}: {ranks:?}");
    }
}

#[test]
fn regions_are_paths_with_one_closed_subpath_per_ring() {
    let (group, plan, _) = ground();
    let rings = |path: &SvgElement| {
        let SvgElement::Path { d, .. } = path else { panic!("a path") };
        let (moves, closes) = (d.iter().filter(|command| matches!(command, PathCommand::MoveTo { .. })).count(), d.iter().filter(|command| matches!(command, PathCommand::ClosePath)).count());
        assert_eq!(moves, closes, "{d:?}");
        moves
    };
    let mut written: Vec<usize> = layer(&group, "layer regions").iter().map(rings).collect();
    let mut expected: Vec<usize> = plan.regions.iter().map(|region| 1 + region.holes.len()).collect();
    written.sort();
    expected.sort();
    assert_eq!(written, expected);
}

#[test]
fn every_primitive_names_the_model_element_it_depicts() {
    let (group, plan, _) = ground();
    let named = |element: &SvgElement| match element {
        SvgElement::Path { common, .. } | SvgElement::Text { common, .. } => extra(common, "data-element").zip(extra(common, "data-id")).is_some(),
        _ => false,
    };
    assert!(["layer regions", "layer lines", "layer texts"].iter().all(|name| layer(&group, name).iter().all(named)));
    let ids: Vec<&str> = layer(&group, "layer lines").iter().filter_map(|path| if let SvgElement::Path { common, .. } = path { extra(common, "data-id") } else { None }).collect();
    assert!(plan.polylines.iter().all(|line| ids.contains(&line.id.as_str())));
}

#[test]
fn space_tags_stack_number_name_and_area_in_tspans_and_the_group_is_well_formed() {
    let (group, plan, _) = ground();
    let document = document_of(group);
    assert!(document.contains(" m²</tspan>"));
    let stacked: usize = plan.texts.iter().map(|text| 1 + usize::from(!text.detail.is_empty()) + usize::from(text.measure.is_some())).filter(|lines| *lines > 1).sum();
    let found = tags(&document);
    assert_eq!(found.iter().filter(|tag| tag.name == "tspan").count(), stacked);
    assert!(stacked > 0);
    assert_eq!(found.iter().filter(|tag| tag.name == "path").count(), plan.regions.len() + plan.polylines.len());
    assert!(found.iter().filter(|tag| tag.name == "tspan").all(|tag| tag.attribute("x").is_some() && tag.attribute("dy").is_some()));
}
