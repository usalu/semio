//! 🧨️ `clash-sets`: the 3D clashes between the solids of the elements two selectors pick, per authored clash set, grouped. Everything here is derived from the element solids, nothing is stored.
//!
//! Definitions. Lengths in metres, building-datum world coordinates (a solid is placed by the building origin, rotation and the datum of its storey).
//! * a clash set tests every element picked by side A against every element picked by side B (never an element against itself; a pair picked from both sides is tested once, in id order);
//! * two solids clash hard when their surfaces interpenetrate (or one lies inside the other) by more than the tolerance of the set, the penetration being the smallest extent of the intersection set (see
//!   `semio_framework_geometry::collision`), and softly when they do not touch but stay closer than the clearance; touching or sharing a face is no clash;
//! * a pair never clashes with its own host: an opening and its wall, a wall sweep and its wall, a railing and the stair, ramp or slab it stands on;
//! * a clash is reported with the world point of the contact, the box of the intersection set (hard) or of the closest points (soft), the storey of the first element and the distance (negative = penetration);
//! * the clashes are grouped by anchor: the element of the clash that takes part in more clashes of the set (ties: the smaller id); groups are listed by size, largest first.
//!
//! The graph has one `Probe` node per solid a clash set picks (the world mesh, its box and its bounding-volume hierarchy: the spatial index, rebuilt only when the solid changes) and one `ClashSet` node per
//! set whose parents are the probes of the elements it picks.
//!
//! Related: buildingSMART BCF clash topics, Navisworks clash detective rules; the pair test is the mesh collision of `semio_framework_geometry::collision`.

use super::super::element_solids::{dep_object, dep_value, ElementSolid, SolidKey};
use crate::{ClashSet, ElementClass, ModelSnapshot, Point3};
use semio_framework_geometry::collision::{self, Aabb, Bvh};
use semio_framework_geometry::mesh::TriMesh;
use semio_framework_value::DslValue;
use std::collections::BTreeMap;

/// 🗺️ The snapshot collections the clash sets read, directly or through the solids of the elements.
pub const READS: &[&str] = &["clash_sets", "walls", "wall_sweeps", "wall_types", "curtain_walls", "curtain_wall_types", "curtain_panel_overrides", "openings", "window_types", "door_types", "columns", "column_types", "beams", "beam_types", "slabs", "slab_types", "ceilings", "ceiling_types", "roofs", "roof_types", "stairs", "ramps", "railings", "storeys", "buildings", "sites"];

//#region 🔖️Values
/// 💥️ How two solids clash: they interpenetrate (hard) or stay closer than the clearance (soft).
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, Default, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
pub enum ClashKind {
    #[default]
    Hard,
    Clearance,
}

/// 💥️ One clash between two elements of a clash set.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Clash {
    pub first: String,
    pub second: String,
    pub kind: ClashKind,
    pub distance: f64,
    pub point: Point3,
    pub min: Point3,
    pub max: Point3,
    pub storey: String,
}

/// 🗂️ The clashes of one anchor element: their indices into the clashes of the result.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ClashGroup {
    pub anchor: String,
    pub members: Vec<u32>,
}

/// 🧨️ What one clash set finds: how many elements each side picked, how many pairs were tested exactly and the clashes with their groups.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct ClashSetResult {
    pub elements_a: u32,
    pub elements_b: u32,
    pub tested: u32,
    pub clashes: Vec<Clash>,
    pub groups: Vec<ClashGroup>,
}

impl ClashSetResult {
    /// 🔢️ How many clashes are hard.
    pub fn hard(&self) -> usize {
        self.clashes.iter().filter(|clash| clash.kind == ClashKind::Hard).count()
    }

    /// 🔢️ How many clashes are soft.
    pub fn soft(&self) -> usize {
        self.clashes.len() - self.hard()
    }
}

/// 📦️ The spatial index of one solid: its class and storey, its world mesh, the box of the mesh and the bounding-volume hierarchy over the triangles.
#[derive(Clone, Debug, PartialEq)]
pub struct SolidProbe {
    pub class: Option<ElementClass>,
    pub storey: String,
    pub bounds: Option<Aabb>,
    pub mesh: TriMesh,
    pub bvh: Bvh,
}

impl SolidProbe {
    /// ⚖️ Bytes the probe accounts for in a cache budget.
    pub fn byte_size(&self) -> usize {
        std::mem::size_of::<Self>() + self.mesh.positions.len() * 24 + self.mesh.indices.len() * 12 + self.bvh.triangle_count() * 48
    }
}
//#endregion 🔖️Values

//#region 🔖️Probe
/// 🧭️ The world mesh of a solid: rotated about `+Z` by its placement, then translated; normals are not needed by the pair test and are left out.
pub fn world_mesh(solid: &ElementSolid) -> TriMesh {
    let placement = &solid.placement;
    let (sin, cos) = placement.rotation.sin_cos();
    let positions = solid.positions.chunks_exact(3).map(|p| [cos * p[0] - sin * p[1] + placement.x, sin * p[0] + cos * p[1] + placement.y, p[2] + placement.z]).collect();
    TriMesh { positions, normals: Vec::new(), indices: solid.indices.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect() }
}

/// 📦️ The probe of the solid of the element `id`.
pub fn probe_of(snapshot: &ModelSnapshot, id: &str, solid: &ElementSolid) -> SolidProbe {
    let mesh = world_mesh(solid);
    SolidProbe { class: crate::class_of(snapshot, id), storey: solid.storey.clone(), bounds: collision::mesh_aabb(&mesh), bvh: Bvh::build(&mesh), mesh }
}
//#endregion 🔖️Probe

//#region 🔖️Selection
/// 🎯️ The ids of the elements side A and side B of `set` pick, in id order: the classified elements of the snapshot the selectors accept.
pub fn picks(snapshot: &ModelSnapshot, set: &ClashSet) -> (Vec<String>, Vec<String>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for (id, class) in crate::classified(snapshot) {
        if set.a.picks(snapshot, &id, class) {
            a.push(id.clone());
        }
        if set.b.picks(snapshot, &id, class) {
            b.push(id);
        }
    }
    (a, b)
}

/// 🔗️ The hosting relations of the model as `(element, host)` rows: an opening and its wall or curtain wall, a wall sweep and its wall, a railing and its stair, ramp or slab. A pair of host and hosted never clashes.
pub fn hosting(snapshot: &ModelSnapshot) -> BTreeMap<String, String> {
    let mut rows: BTreeMap<String, String> = BTreeMap::new();
    rows.extend(snapshot.openings.iter().map(|(id, row)| (id.clone(), row.host.clone())));
    rows.extend(snapshot.wall_sweeps.iter().map(|(id, row)| (id.clone(), row.host.clone())));
    rows.extend(snapshot.railings.iter().filter_map(|(id, row)| row.host.as_ref().map(|host| (id.clone(), host.element.clone()))));
    rows
}

fn hosted(rows: &BTreeMap<String, String>, a: &str, b: &str) -> bool {
    rows.get(a).is_some_and(|host| host == b) || rows.get(b).is_some_and(|host| host == a)
}
//#endregion 🔖️Selection

//#region 🔖️Clashes
/// 📍️ A world point of the contact as a snapshot point.
fn point(p: [f64; 3]) -> Point3 {
    Point3 { x: p[0], y: p[1], z: p[2] }
}

/// 🧱️ Whether the boxes of two probes can clash at all: they overlap once grown by the clearance.
fn near(a: &SolidProbe, b: &SolidProbe, margin: f64) -> bool {
    match (&a.bounds, &b.bounds) {
        (Some(left), Some(right)) => left.overlaps(right, margin),
        _ => false,
    }
}

/// 🔎️ The candidate pairs `(index into a, index into b)` whose boxes overlap once grown by `margin`, in index order: the boxes of side B are sorted along `x` and the sweep walks only the part of the sort that can overlap.
pub fn candidate_pairs(a: &[(&str, &SolidProbe)], b: &[(&str, &SolidProbe)], margin: f64) -> Vec<(usize, usize)> {
    let mut order: Vec<usize> = (0..b.len()).filter(|&at| b[at].1.bounds.is_some()).collect();
    order.sort_by(|&x, &y| {
        let key = |at: usize| b[at].1.bounds.map_or(0.0, |bounds| bounds.min[0]);
        key(x).total_cmp(&key(y)).then(x.cmp(&y))
    });
    let mut reach = Vec::with_capacity(order.len());
    let mut widest = f64::NEG_INFINITY;
    for &at in &order {
        widest = widest.max(b[at].1.bounds.map_or(f64::NEG_INFINITY, |bounds| bounds.max[0]));
        reach.push(widest);
    }
    let mut pairs = Vec::new();
    for (i, (_, left)) in a.iter().enumerate() {
        let Some(bounds) = left.bounds else { continue };
        let upper = order.partition_point(|&at| b[at].1.bounds.is_some_and(|other| other.min[0] <= bounds.max[0] + margin));
        let lower = reach[..upper].partition_point(|&widest| widest < bounds.min[0] - margin);
        let mut row: Vec<usize> = order[lower..upper].iter().copied().filter(|&at| near(left, b[at].1, margin)).collect();
        row.sort_unstable();
        pairs.extend(row.into_iter().map(|j| (i, j)));
    }
    pairs
}

/// 🗂️ The groups of `clashes`: by anchor, largest first.
pub fn grouped(clashes: &[Clash]) -> Vec<ClashGroup> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for clash in clashes {
        *counts.entry(clash.first.as_str()).or_default() += 1;
        *counts.entry(clash.second.as_str()).or_default() += 1;
    }
    let mut groups: BTreeMap<&str, Vec<u32>> = BTreeMap::new();
    for (at, clash) in clashes.iter().enumerate() {
        let (first, second) = (counts[clash.first.as_str()], counts[clash.second.as_str()]);
        let anchor = if second > first { clash.second.as_str() } else { clash.first.as_str() };
        groups.entry(anchor).or_default().push(at as u32);
    }
    let mut rows: Vec<ClashGroup> = groups.into_iter().map(|(anchor, members)| ClashGroup { anchor: anchor.to_string(), members }).collect();
    rows.sort_by(|a, b| b.members.len().cmp(&a.members.len()).then_with(|| a.anchor.cmp(&b.anchor)));
    rows
}

/// 🧨️ The clashes of `set` among the probes side A and side B picked (each a list of `(element id, probe)` in id order); `relations` are the hosting rows, `storey_of` names the storey of an element.
pub fn clashes_of(set: &ClashSet, a: &[(&str, &SolidProbe)], b: &[(&str, &SolidProbe)], relations: &BTreeMap<String, String>, storey_of: &dyn Fn(&str) -> String, cancel: &dyn Fn() -> bool) -> ClashSetResult {
    let mut result = ClashSetResult { elements_a: a.len() as u32, elements_b: b.len() as u32, ..ClashSetResult::default() };
    let mut seen: std::collections::BTreeSet<(&str, &str)> = std::collections::BTreeSet::new();
    for (i, j) in candidate_pairs(a, b, set.clearance.max(0.0)) {
        if cancel() {
            break;
        }
        let ((first_id, first), (second_id, second)) = (a[i], b[j]);
        if first_id == second_id || hosted(relations, first_id, second_id) {
            continue;
        }
        let (low, high) = if first_id <= second_id { (first_id, second_id) } else { (second_id, first_id) };
        if !seen.insert((low, high)) {
            continue;
        }
        result.tested += 1;
        let (low_probe, high_probe) = if first_id <= second_id { (first, second) } else { (second, first) };
        let Some(found) = collision::clash(&low_probe.bvh, &low_probe.mesh, &high_probe.bvh, &high_probe.mesh, set.tolerance, set.clearance, cancel) else { continue };
        result.clashes.push(Clash {
            first: low.to_string(),
            second: high.to_string(),
            kind: if found.kind == collision::ClashKind::Hard { ClashKind::Hard } else { ClashKind::Clearance },
            distance: found.distance,
            point: point(found.point),
            min: point(found.bounds.min),
            max: point(found.bounds.max),
            storey: storey_of(low),
        });
    }
    result.clashes.sort_by(|x, y| (&x.first, &x.second).cmp(&(&y.first, &y.second)));
    result.groups = grouped(&result.clashes);
    result
}
//#endregion 🔖️Clashes

//#region 🔖️Dependency
/// 🔑️ What the `ClashSet` node of `id` reads of the snapshot besides the probes of its parents: the set, the ids each side picks (membership follows the class, storey and phase of every element) and the hosting relations.
pub fn dependency(snapshot: &ModelSnapshot, id: &str) -> DslValue {
    let Some(set) = snapshot.clash_sets.get(id) else { return DslValue::Null };
    let (a, b) = picks(snapshot, set);
    let storeys = DslValue::object(a.iter().chain(b.iter()).map(|element| (element.clone(), dep_value(&crate::storey_of(snapshot, element).cloned()))));
    dep_object([("set", dep_value(set)), ("a", dep_value(&a)), ("b", dep_value(&b)), ("storeys", storeys), ("hosting", dep_value(&hosting(snapshot)))])
}

/// 🔑️ The solid keys a clash set needs probes of, among the `solids` the graph has: the elements either side picks.
pub fn needed<'a>(snapshot: &ModelSnapshot, set: &ClashSet, solids: &'a [SolidKey]) -> Vec<&'a SolidKey> {
    let (a, b) = picks(snapshot, set);
    solids.iter().filter(|key| a.binary_search(&key.id).is_ok() || b.binary_search(&key.id).is_ok()).collect()
}
//#endregion 🔖️Dependency

//#region 🔖️Tables
fn number(value: f64) -> DslValue {
    DslValue::float(value)
}

/// ⚖️ The table the third-party oracle recomputes, `{ <set id>: { elements_a, elements_b, tested, clashes: [ { first, second, kind, distance, point } ] } }`: the contact point of a soft clash is one of a continuum of
/// closest points and is left out (null).
pub fn table_json(results: &BTreeMap<String, ClashSetResult>) -> String {
    let sets = results.iter().map(|(id, result)| {
        let clashes = result.clashes.iter().map(|clash| {
            let point = if clash.kind == ClashKind::Hard { DslValue::Array(vec![number(clash.point.x), number(clash.point.y), number(clash.point.z)]) } else { DslValue::Null };
            DslValue::object([
                ("first".to_string(), DslValue::String(clash.first.clone())),
                ("second".to_string(), DslValue::String(clash.second.clone())),
                ("kind".to_string(), DslValue::String(format!("{:?}", clash.kind))),
                ("distance".to_string(), number(clash.distance)),
                ("point".to_string(), point),
            ])
        });
        let row = DslValue::object([
            ("elements_a".to_string(), DslValue::int(i64::from(result.elements_a))),
            ("elements_b".to_string(), DslValue::int(i64::from(result.elements_b))),
            ("tested".to_string(), DslValue::int(i64::from(result.tested))),
            ("clashes".to_string(), DslValue::Array(clashes.collect())),
        ]);
        (id.clone(), row)
    });
    semio_framework_pack_json::to_json_string(&DslValue::object(sets))
}

/// 🧊️ The world meshes of the classified elements that have a solid, `{ <element id>: { class, storey, positions, indices } }`: the committed input of the clash oracle, written by the blessing test from the inferred solids.
pub fn meshes_json(snapshot: &ModelSnapshot, solids: &BTreeMap<String, ElementSolid>) -> String {
    let rows = crate::classified(snapshot).into_iter().filter_map(|(id, class)| {
        let solid = solids.get(&id).filter(|solid| !solid.is_empty())?;
        let mesh = world_mesh(solid);
        let positions = DslValue::Array(mesh.positions.iter().map(|p| DslValue::Array(p.iter().map(|value| number(*value)).collect())).collect());
        let indices = DslValue::Array(mesh.indices.iter().map(|t| DslValue::Array(t.iter().map(|index| DslValue::int(i64::from(*index))).collect())).collect());
        let row = DslValue::object([("class".to_string(), DslValue::String(class.name().to_string())), ("storey".to_string(), DslValue::String(solid.storey.clone())), ("positions".to_string(), positions), ("indices".to_string(), indices)]);
        Some((id, row))
    });
    semio_framework_pack_json::to_json_string(&DslValue::object(rows))
}
//#endregion 🔖️Tables

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
