//! 🧭️ Fem3d transform gumball — what the live selection resolves to geometrically (the nodes and
//! solids a gesture may move), the pivot the host anchors its gumball at, the whole-record
//! `ReplaceNode`/`ReplaceSolid` mutations one translate/rotate/scale step spells, and the
//! `World3d` selection record that arms the host gumball with live dispatch.

use crate::editor::fem3d::interaction::{
    fem3d_element_endpoints, fem3d_entity_kind, fem3d_load_owner, Fem3dInteractionSnapshot, FEM3D_GRANULARITY_ELEMENT, FEM3D_GRANULARITY_LOAD, FEM3D_GRANULARITY_NODE, FEM3D_GRANULARITY_SOLID, FEM3D_GRANULARITY_SUPPORT,
};
use crate::editor::fem3d::modes::edit::windows::model::config::Fem3dGumballConfig;
use crate::standards::v1::subsets::any::schema::mutations::replace_node;
use crate::standards::v1::subsets::any::schema::mutations::replace_solid;
use crate::standards::v1::subsets::any::schema::mutations::text::Fem3dMutation;
use crate::{element_id, Fem3dSnapshot, FemLoad, FemSolid};
use std::collections::BTreeSet;

//#region 🔖️Targets
/// 🎯️ The geometry a selection moves: node ids and solid ids, each once. Elements, supports and
/// loads resolve to the nodes and solids that carry them; materials, sections, cases and
/// combinations carry no geometry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fem3dTransformTargets {
    pub node_ids: BTreeSet<String>,
    pub solid_ids: BTreeSet<String>,
}

impl Fem3dTransformTargets {
    pub fn is_empty(&self) -> bool {
        self.node_ids.is_empty() && self.solid_ids.is_empty()
    }
}

pub fn fem3d_transform_targets(doc: &Fem3dSnapshot, selection_ids: &[String]) -> Fem3dTransformTargets {
    let mut out = Fem3dTransformTargets::default();
    let member_nodes = |target: &str, out: &mut Fem3dTransformTargets| {
        if let Some(element) = doc.elements.iter().find(|element| element_id(element) == target) {
            let (start, end) = fem3d_element_endpoints(element);
            out.node_ids.insert(start.to_string());
            out.node_ids.insert(end.to_string());
        }
    };
    for id in selection_ids {
        match fem3d_entity_kind(doc, id) {
            Some(FEM3D_GRANULARITY_NODE) => {
                out.node_ids.insert(id.clone());
            }
            Some(FEM3D_GRANULARITY_SOLID) => {
                out.solid_ids.insert(id.clone());
            }
            Some(FEM3D_GRANULARITY_ELEMENT) => member_nodes(id, &mut out),
            Some(FEM3D_GRANULARITY_SUPPORT) => {
                if let Some(support) = doc.supports.iter().find(|support| support.id == *id) {
                    out.node_ids.insert(support.node_id.clone());
                }
            }
            Some(FEM3D_GRANULARITY_LOAD) => {
                if let Some((_, load)) = fem3d_load_owner(doc, id) {
                    match load {
                        FemLoad::Nodal { node_id, .. } => {
                            out.node_ids.insert(node_id.clone());
                        }
                        FemLoad::MemberUdl { element_id: target, .. } => member_nodes(target, &mut out),
                        FemLoad::Area { solid_id, .. } => {
                            out.solid_ids.insert(solid_id.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// 🎯️ The gumball pivot: the centroid of every moved node and every moved solid's own centroid.
pub fn fem3d_transform_pivot(doc: &Fem3dSnapshot, targets: &Fem3dTransformTargets) -> Option<[f64; 3]> {
    let mut points = Vec::new();
    for id in &targets.node_ids {
        if let Some(node) = doc.nodes.iter().find(|node| &node.id == id) {
            points.push([node.x, node.y, node.z]);
        }
    }
    for id in &targets.solid_ids {
        if let Some(point) = doc.solids.iter().find(|solid| &solid.id == id).and_then(crate::standards::v1::subsets::any::scene::fem3d_solid_centroid) {
            points.push(point);
        }
    }
    if points.is_empty() {
        return None;
    }
    let count = points.len() as f64;
    let sum = points.iter().fold([0.0; 3], |sum, point| [sum[0] + point[0], sum[1] + point[1], sum[2] + point[2]]);
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}
//#endregion 🔖️Targets

//#region 🔖️Geometry
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn normalized(v: [f64; 3]) -> Option<[f64; 3]> {
    let length = dot(v, v).sqrt();
    (length > 1e-12).then(|| [v[0] / length, v[1] / length, v[2] / length])
}

/// 🔄️ Rodrigues' rotation of `point` about the unit `axis` through `pivot` by `angle` radians.
fn rotate_point(point: [f64; 3], pivot: [f64; 3], axis: [f64; 3], angle: f64) -> [f64; 3] {
    let p = sub(point, pivot);
    let (sin, cos) = angle.sin_cos();
    let term_1 = [p[0] * cos, p[1] * cos, p[2] * cos];
    let k = cross(axis, p);
    let term_2 = [k[0] * sin, k[1] * sin, k[2] * sin];
    let along = dot(axis, p) * (1.0 - cos);
    let term_3 = [axis[0] * along, axis[1] * along, axis[2] * along];
    add(pivot, add(add(term_1, term_2), term_3))
}

fn scale_point(point: [f64; 3], pivot: [f64; 3], factors: [f64; 3]) -> [f64; 3] {
    let p = sub(point, pivot);
    add(pivot, [p[0] * factors[0], p[1] * factors[1], p[2] * factors[2]])
}

/// 🧊️ One solid's footprint, re-drawn through a world-space point map: every outline and hole
/// vertex is lifted into the world at the solid's base, mapped, and projected back; the extrusion
/// offset and length follow the base and top centres. `None` when the map does not keep the
/// footprint plane — a rotation about an axis that is not the solid's extrusion axis, which no
/// extruded record can spell.
fn map_solid(solid: &FemSolid, map: impl Fn([f64; 3]) -> [f64; 3]) -> Option<FemSolid> {
    let lift = |point: &[f64; 2], w: f64| solid.axis.to_world(point[0], point[1], w);
    let base_w = solid.base_z;
    let top_w = solid.base_z + solid.height;
    let mut mapped_base_w: Option<f64> = None;
    let mut project = |point: &[f64; 2]| -> Option<[f64; 2]> {
        let (uv, w) = solid.axis.from_world(map(lift(point, base_w)));
        match mapped_base_w {
            Some(known) if (known - w).abs() > 1e-9 => return None,
            None => mapped_base_w = Some(w),
            _ => {}
        }
        Some(uv)
    };
    let outline = solid.outline.iter().map(&mut project).collect::<Option<Vec<_>>>()?;
    let holes = solid.holes.iter().map(|hole| hole.iter().map(&mut project).collect::<Option<Vec<_>>>()).collect::<Option<Vec<_>>>()?;
    let new_base = mapped_base_w?;
    let reference = solid.outline.first()?;
    let (_, mapped_top) = solid.axis.from_world(map(lift(reference, top_w)));
    let height = mapped_top - new_base;
    if !(height > 0.0) {
        return None;
    }
    Some(FemSolid { outline, holes, base_z: new_base, height, ..solid.clone() })
}

fn apply(doc: &Fem3dSnapshot, targets: &Fem3dTransformTargets, map: impl Fn([f64; 3]) -> [f64; 3]) -> Vec<Fem3dMutation> {
    let mut mutations = Vec::new();
    for id in &targets.node_ids {
        let Some(node) = doc.nodes.iter().find(|node| &node.id == id) else { continue };
        let [x, y, z] = map([node.x, node.y, node.z]);
        if (x - node.x).abs() < 1e-12 && (y - node.y).abs() < 1e-12 && (z - node.z).abs() < 1e-12 {
            continue;
        }
        mutations.push(Fem3dMutation::ReplaceNode(replace_node::ReplaceNode { id: id.clone(), new_node: crate::FemNode { id: id.clone(), x, y, z } }));
    }
    for id in &targets.solid_ids {
        let Some(solid) = doc.solids.iter().find(|solid| &solid.id == id) else { continue };
        let Some(new_solid) = map_solid(solid, &map) else { continue };
        if new_solid == *solid {
            continue;
        }
        mutations.push(Fem3dMutation::ReplaceSolid(replace_solid::ReplaceSolid { id: id.clone(), new_solid }));
    }
    mutations
}

/// ➡️ The mutations one translation step spells.
pub fn fem3d_translate_selection_mutations(doc: &Fem3dSnapshot, selection_ids: &[String], delta: [f64; 3]) -> Vec<Fem3dMutation> {
    if delta.iter().all(|component| component.abs() < 1e-12) {
        return Vec::new();
    }
    let targets = fem3d_transform_targets(doc, selection_ids);
    apply(doc, &targets, |point| add(point, delta))
}

/// 🔄️ The mutations one rotation step spells: `axis` need not be unit length; a zero axis or a
/// zero angle spells nothing. Solids follow only about their own extrusion axis.
pub fn fem3d_rotate_selection_mutations(doc: &Fem3dSnapshot, selection_ids: &[String], axis: [f64; 3], angle: f64) -> Vec<Fem3dMutation> {
    let Some(axis) = normalized(axis) else { return Vec::new() };
    if angle.abs() < 1e-12 {
        return Vec::new();
    }
    let targets = fem3d_transform_targets(doc, selection_ids);
    let Some(pivot) = fem3d_transform_pivot(doc, &targets) else { return Vec::new() };
    apply(doc, &targets, |point| rotate_point(point, pivot, axis, angle))
}

/// 📐️ The mutations one scale step spells, about the selection's pivot.
pub fn fem3d_scale_selection_mutations(doc: &Fem3dSnapshot, selection_ids: &[String], factors: [f64; 3]) -> Vec<Fem3dMutation> {
    if factors.iter().all(|factor| (factor - 1.0).abs() < 1e-12) || factors.iter().any(|factor| !factor.is_finite() || factor.abs() < 1e-9) {
        return Vec::new();
    }
    let targets = fem3d_transform_targets(doc, selection_ids);
    let Some(pivot) = fem3d_transform_pivot(doc, &targets) else { return Vec::new() };
    apply(doc, &targets, |point| scale_point(point, pivot, factors))
}
//#endregion 🔖️Geometry

//#region 🔖️SelectionRecord
/// 🕹️ Whether the world gumball should render: the transform utility is armed, at least one handle
/// flag is on (an all-off gumball would draw nothing to grab), and the live selection moves something.
pub fn fem3d_gumball_active(doc: &Fem3dSnapshot, interaction: &Fem3dInteractionSnapshot, transform_armed: bool, config: &Fem3dGumballConfig) -> bool {
    transform_armed && config.any() && !fem3d_transform_targets(doc, &interaction.selected_ids).is_empty()
}

/// 🕹️ The host's `WorldSelectionRecord` for a fem3d window: the framework-owned `fem3d` selection
/// and hover projected onto the field names `World3dHost`'s `parseSelection` reads, plus — while the
/// transform utility is armed — the gumball descriptor. `gumballLiveDispatch` asks the host to
/// dispatch every drag step as an incremental `translateSelection`/`rotateSelection`/
/// `scaleSelection` instead of previewing locally: the document IS the preview here, because the
/// results window re-solves it on every step.
pub fn fem3d_selection_json(doc: &Fem3dSnapshot, interaction: &Fem3dInteractionSnapshot, transform_armed: bool, config: &Fem3dGumballConfig) -> String {
    let hovered = interaction.hovered_ids.first().map(String::as_str);
    let mut value: dsl::json::Value = dsl::json::parse(&semio_framework_plugin::world3d_selection_json_with_granularity("rectangle", &interaction.selected_ids, hovered, Some(FEM3D_GRANULARITY_NODE))).unwrap_or_else(|_| dsl::json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("selectionMode", dsl::json!("object"));
        object.insert("targets", dsl::json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        if let Some(id) = interaction.selected_ids.first() {
            object.insert("activeObjectId", dsl::json!(id));
        }
        let active = fem3d_gumball_active(doc, interaction, transform_armed, config);
        object.insert("gumballActive", dsl::json!(active));
        if transform_armed {
            object.insert("transformMode", dsl::json!("transform"));
            object.insert(
                "gumballConfig",
                dsl::json!({
                    "moveAxes": config.move_axes,
                    "movePlanes": config.move_planes,
                    "rotate": config.rotate,
                    "scaleAxes": config.scale_axes,
                    "scalePlanes": false,
                    "scaleUniform": config.scale_uniform,
                }),
            );
            object.insert("gumballLiveDispatch", dsl::json!(true));
            if active {
                if let Some(pivot) = fem3d_transform_pivot(doc, &fem3d_transform_targets(doc, &interaction.selected_ids)) {
                    object.insert("gumballTarget", dsl::json!(pivot));
                }
            }
        }
    }
    dsl::json::to_string(&value)
}
//#endregion 🔖️SelectionRecord

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
