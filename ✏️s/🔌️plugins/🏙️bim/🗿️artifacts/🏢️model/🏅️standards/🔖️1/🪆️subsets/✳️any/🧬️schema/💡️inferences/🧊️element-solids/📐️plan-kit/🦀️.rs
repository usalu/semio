//! 📐️ `plan-kit`: the conversions from authored plan values to the geometry crate and the small plan helpers that the frame, slab, roof, stair and railing solids share.
//!
//! Everything here is pure and total: a malformed input yields an empty or degenerate value that the caller turns into an absent solid.

pub use crate::standards::v1::subsets::any::schema::authored::plan::{mark, point, segment_of as seg};
use crate::standards::v1::subsets::any::schema::authored::profile::{profile_extents, profile_polygon};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidFamily};
use crate::{Layer, ModelSnapshot, Profile, StairFlight, Vertex as PlanVertex};
use semio_framework_geometry::loops::Vertex;
use semio_framework_geometry::{Point, Vec2};

//#region 🔖️Conversion
/// ▭️ The extents `(across, depth)` of a profile: the `x` and `y` extents of its flattened outline, the one definition frames, solids, rooms, bodies and plan share.
pub fn extents_of(profile: &Profile) -> (f64, f64) {
    profile_extents(&profile_polygon(profile))
}

/// ▭️ The depth (thickness through the host) of a profile.
pub fn depth_of(profile: &Profile) -> f64 {
    extents_of(profile).1
}

/// 🔷️ An authored loop of bulged vertices as a geometry loop.
pub fn bulged(vertices: &[PlanVertex]) -> Vec<Vertex> {
    vertices.iter().map(|row| Vertex::new(point(&row.point), row.bulge)).collect()
}

/// 🍰️ The thickness of every layer in order; a negative thickness reads as zero.
pub fn layer_thicknesses(layers: &[Layer]) -> Vec<f64> {
    layers.iter().map(|layer| layer.thickness.max(0.0)).collect()
}

/// 🥞️ Depth of the top and of the bottom of every layer below the top of a build-up whose first layer is the topmost.
pub fn stack(thicknesses: &[f64]) -> Vec<(f64, f64)> {
    thicknesses
        .iter()
        .scan(0.0, |depth, thickness| {
            let top = *depth;
            *depth += thickness;
            Some((top, *depth))
        })
        .collect()
}
//#endregion 🔖️Conversion

//#region 🔖️Placement
/// 🔄️ A loop rotated by `angle` about the origin and then moved to `origin`; bulges are unchanged by a rotation.
pub fn placed(vertices: &[Vertex], origin: Point, angle: f64) -> Vec<Vertex> {
    let (sin, cos) = angle.sin_cos();
    vertices.iter().map(|v| Vertex::new(Point::new(origin.x + v.point.x * cos - v.point.y * sin, origin.y + v.point.x * sin + v.point.y * cos), v.bulge)).collect()
}

/// 🧭️ The unit vector of a plan direction given in radians counter-clockwise from `+X`.
pub fn direction(angle: f64) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}

/// ▭️ The centred rectangle `width × depth` as a counter-clockwise loop.
pub fn rectangle(width: f64, depth: f64) -> Vec<Vertex> {
    let (x, y) = (width / 2.0, depth / 2.0);
    [(-x, -y), (x, -y), (x, y), (-x, y)].iter().map(|&(px, py)| Vertex::corner(px, py)).collect()
}
//#endregion 🔖️Placement

//#region 🔖️Projection
fn bulged_loop(vertices: &[PlanVertex]) -> bool {
    vertices.iter().any(|row| row.bulge.abs() > 1e-12)
}

fn bulged_profile(profile: &Profile) -> bool {
    match profile {
        Profile::Circle { .. } => true,
        Profile::Custom { outline } => bulged_loop(outline),
        _ => false,
    }
}

/// 🌙️ Whether the solid of `id` has tessellated arcs (a round profile, a bulged edge, a spiral), so that its measures hold within the chord tolerance only.
pub fn is_curved(snapshot: &ModelSnapshot, id: &str) -> bool {
    if let Some(column) = snapshot.columns.get(id) {
        return snapshot.column_types.get(&column.column_type).is_some_and(|kind| bulged_profile(&kind.profile));
    }
    if let Some(beam) = snapshot.beams.get(id) {
        return snapshot.beam_types.get(&beam.beam_type).is_some_and(|kind| bulged_profile(&kind.profile));
    }
    if let Some(slab) = snapshot.slabs.get(id) {
        return bulged_loop(&slab.boundary) || slab.holes.iter().any(|hole| bulged_loop(hole));
    }
    if let Some(ceiling) = snapshot.ceilings.get(id) {
        return bulged_loop(&ceiling.boundary) || ceiling.holes.iter().any(|hole| bulged_loop(hole));
    }
    if let Some(roof) = snapshot.roofs.get(id) {
        return bulged_loop(&roof.footprint);
    }
    snapshot.stairs.get(id).is_some_and(|stair| matches!(stair.flight, StairFlight::Spiral { .. }))
}


//#endregion 🔖️Projection

//#region 🔖️Testing
/// 🧪️ Shared test support of the families: fixture cases written by the oracle and the comparison of a solid with its expected row.
#[cfg(test)]
pub mod testing {
    use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidFamily};
    use crate::standards::v1::subsets::any::schema::mutations::{apply_model_mutation, set_storey_height::SetStoreyHeight};
    use crate::{ModelMutation, ModelSnapshot};
    use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
    use std::collections::BTreeMap;

    /// 🧫️ An authored snapshot and the table the third-party oracle wrote for it.
    pub struct Case {
        pub snapshot: ModelSnapshot,
        pub expected: serde_json::Value,
    }

    /// 🧫️ Decodes a fixture file: its `snapshot` member through the artifact decoder and its `expected` member as JSON.
    pub fn case(text: &str) -> Case {
        let document: serde_json::Value = serde_json::from_str(text).expect("the fixture is JSON");
        let snapshot = from_json_str(&document["snapshot"].to_string(), JsonMemberPolicy::Reject).expect("the fixture snapshot decodes");
        Case { snapshot, expected: document["expected"].clone() }
    }

    /// 🪜️ The snapshot with the height of `storey` changed by `delta` through the real mutation.
    pub fn raised(snapshot: &ModelSnapshot, storey: &str, delta: f64) -> ModelSnapshot {
        let height = snapshot.storeys[storey].height + delta;
        apply_model_mutation(snapshot, &ModelMutation::SetStoreyHeight(SetStoreyHeight { id: storey.into(), height })).expect("the height edit applies")
    }

    /// ⚖️ `true` when `got` is within `tolerance` of `want`.
    pub fn close(want: f64, got: f64, tolerance: f64) -> bool {
        (want - got).abs() <= tolerance
    }

    fn number(row: &serde_json::Value, key: &str) -> f64 {
        row[key].as_f64().unwrap_or_else(|| panic!("expected row has no number `{key}`"))
    }

    fn vector(row: &serde_json::Value, key: &str) -> [f64; 3] {
        let items = row[key].as_array().unwrap_or_else(|| panic!("expected row has no vector `{key}`"));
        [items[0].as_f64().unwrap_or(f64::NAN), items[1].as_f64().unwrap_or(f64::NAN), items[2].as_f64().unwrap_or(f64::NAN)]
    }

    /// 🥞️ The lowest and highest `z` and the volume of the triangles of one group of a solid.
    pub fn group_extent(solid: &ElementSolid, group: u32) -> (f64, f64, f64) {
        let mesh = solid.mesh();
        let (mut low, mut high, mut volume) = (f64::INFINITY, f64::NEG_INFINITY, 0.0);
        for index in (0..solid.face_groups.len()).filter(|&index| solid.face_groups[index] == group) {
            let [a, b, c] = mesh.triangle(index);
            volume += (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0;
            for vertex in [a, b, c] {
                low = low.min(vertex[2]);
                high = high.max(vertex[2]);
            }
        }
        (low, high, volume)
    }

    /// 🕸️ `true` when the mesh has no boundary: every directed edge (quantised at 1 nm) is balanced by as many reverse edges, which also holds for several closed shells that touch.
    pub fn is_closed(solid: &ElementSolid) -> bool {
        let mesh = solid.mesh();
        let key = |p: [f64; 3]| [(p[0] / 1e-9).round() as i64, (p[1] / 1e-9).round() as i64, (p[2] / 1e-9).round() as i64];
        let mut balance: BTreeMap<([i64; 3], [i64; 3]), i64> = BTreeMap::new();
        for triangle in &mesh.indices {
            let corners = triangle.map(|index| key(mesh.positions[index as usize]));
            for edge in 0..3 {
                let (a, b) = (corners[edge], corners[(edge + 1) % 3]);
                if a != b {
                    *balance.entry((a.min(b), a.max(b))).or_insert(0) += if a < b { 1 } else { -1 };
                }
            }
        }
        !balance.is_empty() && balance.values().all(|count| *count == 0)
    }

    /// 🔮️ Asserts that every expected row is met by the solid of the same id (family, volume, bounds, closed surface) and that every `null` row has no solid.
    pub fn assert_matches_oracle(case: &Case, solids: &BTreeMap<String, ElementSolid>, family: SolidFamily) {
        let table = case.expected.as_object().expect("expected is a table");
        assert!(!table.is_empty(), "the oracle table is empty");
        for (id, row) in table {
            if row.is_null() {
                assert!(!solids.contains_key(id), "{id} must have no solid");
                continue;
            }
            let solid = solids.get(id).unwrap_or_else(|| panic!("{id} must have a solid"));
            assert_eq!(format!("{:?}", solid.family), row["family"].as_str().unwrap_or(""), "{id}.family");
            assert_eq!(solid.family, family, "{id} is a {family:?}");
            let (volume, tolerance) = (number(row, "volume"), number(row, "volume_tolerance"));
            assert!(close(volume, solid.volume, tolerance), "{id}.volume: oracle {volume}, subject {} (tolerance {tolerance})", solid.volume);
            let tolerance = number(row, "bounds_tolerance");
            let (min, max) = (vector(row, "min"), vector(row, "max"));
            let (got_min, got_max) = ([solid.bounds.min.x, solid.bounds.min.y, solid.bounds.min.z], [solid.bounds.max.x, solid.bounds.max.y, solid.bounds.max.z]);
            for axis in 0..3 {
                assert!(close(min[axis], got_min[axis], tolerance), "{id}.min[{axis}]: oracle {}, subject {} (tolerance {tolerance})", min[axis], got_min[axis]);
                assert!(close(max[axis], got_max[axis], tolerance), "{id}.max[{axis}]: oracle {}, subject {} (tolerance {tolerance})", max[axis], got_max[axis]);
            }
            assert!(is_closed(solid), "{id} has a boundary: some edge is not balanced by its reverse");
            assert!(close(solid.mesh().signed_volume(), solid.volume, 1e-9), "{id}: volume is the signed volume of the mesh");
            assert!(solid.area > 0.0 && solid.triangle_count() >= 12, "{id} has a closed surface");
        }
    }
}
//#endregion 🔖️Testing

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
