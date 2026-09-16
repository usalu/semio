//! 🧭️ Fem2d transform gumball — config, pivot, meta layer, and geometry mutations for selection transforms.

use crate::editor::fem2d::interaction::{fem2d_entity_kind, FEM2D_GRANULARITY_ELEMENT, FEM2D_GRANULARITY_LOAD, FEM2D_GRANULARITY_NODE, FEM2D_GRANULARITY_REGION, FEM2D_GRANULARITY_SUPPORT};
use crate::editor::fem2d::interaction::canvas_gesture::FEM2D_UTILITY_TRANSFORM;
use crate::editor::fem2d::modes::edit::windows::model::{fem2d_element_endpoints, find_node_2d, screen_2d};
use crate::standards::v1::subsets::any::schema::mutations::replace_node;
use crate::standards::v1::subsets::any::schema::mutations::replace_region;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
use crate::{element_id, Fem2dSnapshot, FemLoad, Viewport2d};
use semio_framework_plugin::ViewModel;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

//#region 🔖️Config
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fem2dGumballConfig {
    pub move_axes: bool,
    pub rotate: bool,
    pub scale_axes: bool,
    pub scale_uniform: bool,
}

impl Default for Fem2dGumballConfig {
    fn default() -> Self {
        Self { move_axes: true, rotate: true, scale_axes: true, scale_uniform: true }
    }
}

thread_local! {
    static GUMBALL_CONFIG: RefCell<HashMap<String, Fem2dGumballConfig>> = RefCell::new(HashMap::new());
}

pub fn gumball_config_for_window(window_id: &str) -> Fem2dGumballConfig {
    GUMBALL_CONFIG.with(|store| store.borrow().get(window_id).copied().unwrap_or_default())
}

pub fn set_gumball_flag(window_id: &str, flag: &str, pressed: Option<bool>) {
    GUMBALL_CONFIG.with(|store| {
        let mut map = store.borrow_mut();
        let entry = map.entry(window_id.to_string()).or_default();
        let toggle = |current: &mut bool| *current = pressed.unwrap_or(!*current);
        match flag {
            "move" | "moveAxes" => toggle(&mut entry.move_axes),
            "rotate" => toggle(&mut entry.rotate),
            "scaleAxes" => toggle(&mut entry.scale_axes),
            "scaleUniform" => toggle(&mut entry.scale_uniform),
            _ => {}
        }
    });
}
//#endregion 🔖️Config

//#region 🔖️Targets
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fem2dTransformTargets {
    pub node_ids: HashSet<String>,
    pub region_ids: HashSet<String>,
}

pub fn fem2d_transform_targets(doc: &Fem2dSnapshot, selection_ids: &[String]) -> Fem2dTransformTargets {
    let mut out = Fem2dTransformTargets::default();
    for id in selection_ids {
        match fem2d_entity_kind(doc, id) {
            Some(FEM2D_GRANULARITY_NODE) => {
                out.node_ids.insert(id.clone());
            }
            Some(FEM2D_GRANULARITY_REGION) => {
                out.region_ids.insert(id.clone());
            }
            Some(FEM2D_GRANULARITY_ELEMENT) => {
                if let Some(element) = doc.elements.iter().find(|element| element_id(element) == id) {
                    let (start, end) = fem2d_element_endpoints(element);
                    out.node_ids.insert(start.to_string());
                    out.node_ids.insert(end.to_string());
                }
            }
            Some(FEM2D_GRANULARITY_SUPPORT) => {
                if let Some(support) = doc.supports.iter().find(|support| support.id == *id) {
                    out.node_ids.insert(support.node_id.clone());
                }
            }
            Some(FEM2D_GRANULARITY_LOAD) => {
                if let Some((_, load)) = crate::editor::fem2d::interaction::fem2d_load_owner(doc, id) {
                    match load {
                        FemLoad::Nodal { node_id, .. } => {
                            out.node_ids.insert(node_id.clone());
                        }
                        FemLoad::MemberUdl { element_id: target, .. } => {
                            if let Some(element) = doc.elements.iter().find(|element| element_id(element) == target) {
                                let (start, end) = fem2d_element_endpoints(element);
                                out.node_ids.insert(start.to_string());
                                out.node_ids.insert(end.to_string());
                            }
                        }
                        FemLoad::Area { region_id, .. } => {
                            out.region_ids.insert(region_id.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

pub fn fem2d_transform_pivot(doc: &Fem2dSnapshot, targets: &Fem2dTransformTargets) -> Option<(f64, f64)> {
    let mut points = Vec::new();
    for id in &targets.node_ids {
        if let Some(node) = find_node_2d(&doc.nodes, id) {
            points.push((node.x, node.y));
        }
    }
    for id in &targets.region_ids {
        if let Some(region) = doc.regions.iter().find(|region| region.id == *id) {
            for point in &region.outline {
                points.push((point[0], point[1]));
            }
        }
    }
    if points.is_empty() {
        return None;
    }
    let sum = points.iter().fold((0.0, 0.0), |acc, point| (acc.0 + point.0, acc.1 + point.1));
    let count = points.len() as f64;
    Some((sum.0 / count, sum.1 / count))
}
//#endregion 🔖️Targets

//#region 🔖️Geometry
fn translate_model_point(point: (f64, f64), dx: f64, dy: f64) -> (f64, f64) {
    (point.0 + dx, point.1 + dy)
}

fn rotate_model_point(point: (f64, f64), pivot: (f64, f64), angle: f64) -> (f64, f64) {
    let sin = angle.sin();
    let cos = angle.cos();
    let dx = point.0 - pivot.0;
    let dy = point.1 - pivot.1;
    (pivot.0 + dx * cos - dy * sin, pivot.1 + dx * sin + dy * cos)
}

fn scale_model_point(point: (f64, f64), pivot: (f64, f64), sx: f64, sy: f64) -> (f64, f64) {
    (pivot.0 + (point.0 - pivot.0) * sx, pivot.1 + (point.1 - pivot.1) * sy)
}

pub fn fem2d_translate_selection_mutations(doc: &Fem2dSnapshot, selection_ids: &[String], dx: f64, dy: f64) -> Vec<Fem2dMutation> {
    if dx.abs() < 1e-12 && dy.abs() < 1e-12 {
        return Vec::new();
    }
    let targets = fem2d_transform_targets(doc, selection_ids);
    fem2d_apply_model_mutations(doc, &targets, |point| translate_model_point(point, dx, dy))
}

pub fn fem2d_rotate_selection_mutations(doc: &Fem2dSnapshot, selection_ids: &[String], angle: f64) -> Vec<Fem2dMutation> {
    if angle.abs() < 1e-12 {
        return Vec::new();
    }
    let targets = fem2d_transform_targets(doc, selection_ids);
    let pivot = fem2d_transform_pivot(doc, &targets).unwrap_or((0.0, 0.0));
    fem2d_apply_model_mutations(doc, &targets, |point| rotate_model_point(point, pivot, angle))
}

pub fn fem2d_scale_selection_mutations(doc: &Fem2dSnapshot, selection_ids: &[String], sx: f64, sy: f64) -> Vec<Fem2dMutation> {
    if (sx - 1.0).abs() < 1e-12 && (sy - 1.0).abs() < 1e-12 {
        return Vec::new();
    }
    let targets = fem2d_transform_targets(doc, selection_ids);
    let pivot = fem2d_transform_pivot(doc, &targets).unwrap_or((0.0, 0.0));
    fem2d_apply_model_mutations(doc, &targets, |point| scale_model_point(point, pivot, sx, sy))
}

fn fem2d_apply_model_mutations(doc: &Fem2dSnapshot, targets: &Fem2dTransformTargets, map: impl Fn((f64, f64)) -> (f64, f64)) -> Vec<Fem2dMutation> {
    let mut mutations = Vec::new();
    for id in &targets.node_ids {
        let Some(node) = find_node_2d(&doc.nodes, id) else { continue };
        let (x, y) = map((node.x, node.y));
        if (x - node.x).abs() < 1e-12 && (y - node.y).abs() < 1e-12 {
            continue;
        }
        mutations.push(Fem2dMutation::ReplaceNode(replace_node::ReplaceNode { id: id.clone(), new_node: crate::FemNode { id: id.clone(), x, y } }));
    }
    for id in &targets.region_ids {
        let Some(region) = doc.regions.iter().find(|region| region.id == *id) else { continue };
        let outline = region.outline.iter().map(|point| {
            let mapped = map((point[0], point[1]));
            [mapped.0, mapped.1]
        }).collect::<Vec<_>>();
        if outline == region.outline {
            continue;
        }
        let mut new_region = region.clone();
        new_region.outline = outline;
        mutations.push(Fem2dMutation::ReplaceRegion(replace_region::ReplaceRegion { id: id.clone(), new_region }));
    }
    mutations
}
//#endregion 🔖️Geometry

//#region 🔖️MetaLayer
pub fn fem2d_gumball_active(active_utility: &str, selection_ids: &[String]) -> bool {
    active_utility == FEM2D_UTILITY_TRANSFORM && !selection_ids.is_empty()
}

pub fn fem2d_gumball_meta_layer(doc: &Fem2dSnapshot, selection_ids: &[String], camera: &Viewport2d, active_utility: &str, window_id: Option<&str>) -> Option<dsl::json::Value> {
    if !fem2d_gumball_active(active_utility, selection_ids) {
        return None;
    }
    let targets = fem2d_transform_targets(doc, selection_ids);
    let pivot = fem2d_transform_pivot(doc, &targets)?;
    let (layer_x, layer_y) = screen_2d(pivot.0, pivot.1);
    let config = window_id.map(gumball_config_for_window).unwrap_or_default();
    Some(dsl::json!({
        "id": "meta:gumball",
        "role": "meta",
        "gumball": {
            "active": true,
            "pivotLayer": [layer_x, layer_y],
            "pivotModel": [pivot.0, pivot.1],
            "camera": { "x": camera.x, "y": camera.y, "zoom": camera.zoom },
            "selectionIds": selection_ids,
            "config": {
                "moveAxes": config.move_axes,
                "rotate": config.rotate,
                "scaleAxes": config.scale_axes,
                "scaleUniform": config.scale_uniform,
            },
        },
    }))
}

pub fn fem2d_gumball_meta_for_render(doc: &Fem2dSnapshot, interaction: &crate::editor::fem2d::interaction::Fem2dInteractionSnapshot, camera: &Viewport2d, view: &ViewModel, active_utility: &str) -> Option<dsl::json::Value> {
    let window_id = crate::editor::fem2d::interaction::canvas_gesture::fem2d_gesture_window_id_for_render(view, crate::editor::fem2d::modes::edit::windows::model::BODY_KEY)
        .or_else(|| view.window_id.clone());
    fem2d_gumball_meta_layer(doc, &interaction.selected_ids, camera, active_utility, window_id.as_deref())
}
//#endregion 🔖️MetaLayer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
