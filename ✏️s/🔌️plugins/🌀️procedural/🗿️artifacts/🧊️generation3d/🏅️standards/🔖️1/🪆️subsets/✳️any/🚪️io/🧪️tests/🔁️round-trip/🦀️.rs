//! 🔁️ `s.procedural.generation3d@1/*` IO round trips — the gate that would have failed loudly on the
//! seven silently-wrong format leaves ticket 26/09/09/PROCEDURAL-3D-END-TO-END replaced.
//!
//! **What every format is held to.** One committed geometry (`🧫️fixtures/🚪️io/🧊️unit-cube/🔣️.json`:
//! 8 vertices, 12 triangles, bounds `[0,0,0]..[1,1,1]`, enclosed volume 1.0) goes out through this
//! artifact's export leaf as real bytes of the named format, comes back in through its own import
//! leaf, and has to still be that cube. Before this ticket every one of these assertions failed by
//! construction: export returned the artifact's own DSL text under a mesh file's name and import
//! discarded the bytes for an empty document, so "same triangle count" was 12 vs 0 in every lane.
//!
//! **Why the assertions are what they are.** `triangleCount` and the bounding box are the two
//! quantities EVERY mesh format in this surface can carry, so they are asserted everywhere;
//! `vertexCount` is asserted only where the format has a shared vertex pool to preserve (PLY, glTF),
//! because STL/OBJ are legitimately non-indexed and repeat corners. Volume is asserted through a
//! THIRD-PARTY reference — `parry3d`'s `MassProperties::from_trimesh`, the same test-lane oracle the
//! sibling example-geometry harness uses — so a committed expected number is never only this
//! repository's own arithmetic, and a codec that silently reversed a winding or dropped a face would
//! show up as a wrong (or negative) volume rather than a still-plausible triangle count.
//!
//! **Where the per-format cases live.** Beside the leaf each one exercises, not in one pile here —
//! `🚪️io/{📤️export,📥️import}/…/<format>/…/🧪️tests/🔁️round-trip/🦀️.rs`. This file is only the
//! `[[test]]` entry point (emoji file names never compile as an implicit `tests/` target, so the
//! target is declared with an ASCII name in `📦️packages/🦀️rust/Cargo.toml`) plus the fixture, the
//! projection and the oracle every case shares.
//!
//! @see ../../🦀️.rs — `mesh_bridge`, the composition point all the leaves go through.

use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMeshSnapshot, SemioTopology};

//#region 🧊️Fixture
/// 🧊️ The one committed geometry, as authored — see the fixture file's own `$comment`.
pub const UNIT_CUBE_JSON: &str = include_str!("../../../🧫️fixtures/🚪️io/🧊️unit-cube/🔣️.json");

/// 🧊️ The fixture in the renderer's own wire form, i.e. exactly what
/// `crate::editor::generation3d::export_mesh_from_document` hands the mesh bridge at runtime.
pub fn unit_cube_mesh_data() -> semio_framework_plugin::MeshData {
    let parsed: serde_json::Value = serde_json::from_str(UNIT_CUBE_JSON).expect("unit-cube fixture is valid json");
    let positions: Vec<f32> = parsed["positions"].as_array().expect("positions array").iter().map(|value| value.as_f64().expect("position number") as f32).collect();
    let indices: Vec<u32> = parsed["indices"].as_array().expect("indices array").iter().map(|value| value.as_u64().expect("index number") as u32).collect();
    assert_eq!(positions.len(), 24, "the committed unit cube has 8 vertices");
    assert_eq!(indices.len(), 36, "the committed unit cube has 12 triangles");
    semio_framework_plugin::MeshData { positions, indices, ..Default::default() }
}

/// 🔺️ The fixture as this repo's own typed mesh document — the export leaves' actual input.
pub fn unit_cube_semio_mesh() -> SemioMeshSnapshot {
    semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::mesh_bridge::semio_mesh_from_mesh_data(&unit_cube_mesh_data()).expect("the committed unit cube converts to a semio mesh")
}
//#endregion 🧊️Fixture

//#region 📐️Projection
/// 📐️ The shape every producer and reader in this lane is compared through — deliberately the
/// format-independent subset, so one expectation serves six grammars.
#[derive(Debug, Clone, PartialEq)]
pub struct MeshProjection {
    pub triangle_count: usize,
    pub vertex_count: usize,
    pub min: [f64; 3],
    pub max: [f64; 3],
}

/// 🔺️ Every triangle of a mesh as three resolved corner positions, indices already applied.
pub fn triangles_of(mesh: &SemioMeshSnapshot) -> Vec<[[f64; 3]; 3]> {
    let mut out = Vec::new();
    for entry in &mesh.meshes {
        for primitive in &entry.primitives {
            if primitive.topology != SemioTopology::Triangles {
                continue;
            }
            let corners: Vec<u32> = if primitive.indices.is_empty() { (0..primitive.positions.len() as u32).collect() } else { primitive.indices.clone() };
            for face in corners.chunks_exact(3) {
                let point = |index: u32| -> [f64; 3] {
                    let p: SemioPoint3 = primitive.positions[index as usize];
                    [p.x, p.y, p.z]
                };
                out.push([point(face[0]), point(face[1]), point(face[2])]);
            }
        }
    }
    out
}

/// 📍️ Every position of a mesh, whether or not it participates in a triangle — this is what LAS's
/// point-cloud export has to preserve, and what PLY/glTF's shared vertex pool has to keep distinct.
pub fn positions_of(mesh: &SemioMeshSnapshot) -> Vec<[f64; 3]> {
    mesh.meshes.iter().flat_map(|entry| entry.primitives.iter()).flat_map(|primitive| primitive.positions.iter().map(|p| [p.x, p.y, p.z])).collect()
}

pub fn project(mesh: &SemioMeshSnapshot) -> MeshProjection {
    let positions = positions_of(mesh);
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for position in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(position[axis]);
            max[axis] = max[axis].max(position[axis]);
        }
    }
    MeshProjection { triangle_count: triangles_of(mesh).len(), vertex_count: positions.len(), min, max }
}
//#endregion 📐️Projection

//#region ⚖️Assertions
/// 🎚️ Tolerance every lane compares within. LAS is the widest real quantizer in this surface
/// (`SemioMeshToLas` writes a `0.0001`-unit scale), and `f32` accessor storage in glTF/PLY costs
/// about `1e-7` on unit coordinates; `1e-3` clears both while still catching a genuinely wrong
/// coordinate.
pub const TOLERANCE: f64 = 1e-3;

pub fn assert_close(label: &str, actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= TOLERANCE, "{label}: expected {expected}, got {actual} (tolerance {TOLERANCE})");
}

/// 🧊️ The committed cube's own numbers, asserted on whatever came back out of a format.
pub fn assert_is_unit_cube(format: &str, projection: &MeshProjection) {
    assert_eq!(projection.triangle_count, 12, "{format}: triangle count survived the round trip");
    for axis in 0..3 {
        assert_close(&format!("{format}: min[{axis}]"), projection.min[axis], 0.0);
        assert_close(&format!("{format}: max[{axis}]"), projection.max[axis], 1.0);
    }
}
/// 🧹️ Disposes of a document the way this repository requires.
///
/// `OrderedMap`/`OrderedSet`/`Dictionary` are cold-tracked roots: a NON-EMPTY one panics on drop
/// unless its owner retires it explicitly (`ordered-map root must be explicitly retired before
/// drop`). That is not something the import fixture introduces — every real document carries a
/// non-empty `layout` (see any bundled example's own `🗣️.dsl.semio`, which writes a `layout={…}`
/// block), so `parse_dsl`'s callers carry the same obligation; the example-geometry lane discharges
/// it by moving the fixture into a `FlowHost` and retiring the generation half. A test that only
/// looks at a document has to do it by hand.
pub fn retire_document(document: semio_s_artifact_procedural_generation3d::Generation3dSnapshot) {
    let semio_s_artifact_procedural_generation3d::Generation3dSnapshot { fixture, generation } = document;
    generation.retire_cold();
    fixture.retire_cold();
}
//#endregion ⚖️Assertions

//#region 🔮️Oracle
/// 🔮️ The THIRD-PARTY check. `parry3d` recomputes the enclosed volume and the axis-aligned bounds of
/// whatever an independent reader recovered from our bytes, so the expected `1.0` is confirmed by a
/// library that shares no code with this repository's mesh path. A dropped face, a flipped winding
/// or a collapsed axis all move this number; none of them necessarily moves a triangle count.
/// @see https://github.com/dimforge/parry
pub fn oracle_volume_and_bounds(triangles: &[[[f64; 3]; 3]]) -> (f64, [f64; 3], [f64; 3]) {
    let mut points: Vec<parry3d::math::Point<parry3d::math::Real>> = Vec::with_capacity(triangles.len() * 3);
    let mut indices: Vec<[u32; 3]> = Vec::with_capacity(triangles.len());
    for triangle in triangles {
        let base = points.len() as u32;
        for corner in triangle {
            points.push(parry3d::math::Point::new(corner[0] as parry3d::math::Real, corner[1] as parry3d::math::Real, corner[2] as parry3d::math::Real));
        }
        indices.push([base, base + 1, base + 2]);
    }
    let properties = parry3d::mass_properties::MassProperties::from_trimesh(1.0, &points, &indices);
    let aabb = parry3d::bounding_volume::Aabb::from_points(points.iter());
    (properties.mass() as f64, [aabb.mins.x as f64, aabb.mins.y as f64, aabb.mins.z as f64], [aabb.maxs.x as f64, aabb.maxs.y as f64, aabb.maxs.z as f64])
}

/// 🔮️ The full third-party verdict on a recovered mesh: volume 1.0 on the unit cube, and bounds the
/// independent library agrees with.
pub fn assert_oracle_agrees_on_unit_cube(format: &str, mesh: &SemioMeshSnapshot) {
    let triangles = triangles_of(mesh);
    assert_eq!(triangles.len(), 12, "{format}: oracle input has the cube's 12 triangles");
    let (volume, min, max) = oracle_volume_and_bounds(&triangles);
    assert_close(&format!("{format}: parry3d enclosed volume"), volume, 1.0);
    for axis in 0..3 {
        assert_close(&format!("{format}: parry3d min[{axis}]"), min[axis], 0.0);
        assert_close(&format!("{format}: parry3d max[{axis}]"), max[axis], 1.0);
    }
}
/// 🔤️ The cube as ASCII STL bytes — the txt case needs a realistic document to round-trip, and an
/// IMPORTED one (long base64 note, real neuron kind, two synapses, three layout entries) exercises
/// far more of the grammar than the default fixture does.
pub fn stl_bytes_for_txt_case() -> Vec<u8> {
    semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::export::serializers::artifacts::stl::v_ascii::any::serialize_mesh_bytes(&unit_cube_semio_mesh()).expect("the unit cube exports as stl")
}
//#endregion 🔮️Oracle

//#region 🔖️Cases
#[path = "../../📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🧪️tests/🔁️round-trip/🦀️.rs"]
mod stl;
#[path = "../../📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🧪️tests/🔁️round-trip/🦀️.rs"]
mod obj;
#[path = "../../📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🧪️tests/🔁️round-trip/🦀️.rs"]
mod ply;
#[path = "../../📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🧪️tests/🔁️round-trip/🦀️.rs"]
mod gltf;
#[path = "../../📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🧪️tests/🔁️round-trip/🦀️.rs"]
mod las;
#[path = "../../📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🧪️tests/🔁️round-trip/🦀️.rs"]
mod dwg;
#[path = "../../📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🧪️tests/🔁️round-trip/🦀️.rs"]
mod txt;
#[path = "../../📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🧪️tests/🔁️round-trip/🦀️.rs"]
mod png;
// 📄️ The user-facing surface those nine leaves reach the user THROUGH — the roster, the download
// envelope, the accept filter and the picked-file path. It shares this lane's fixture, projection
// and third-party oracle rather than standing up a second copy of them.
#[path = "../📄️document-surface/🦀️.rs"]
mod document_surface;
//#endregion 🔖️Cases

//#region 🧪️FixtureLaws
/// 🧊️ The fixture itself is the cube it claims to be, checked by the third-party oracle BEFORE any
/// codec touches it — otherwise a wrong fixture would make every format lane agree on the wrong
/// answer, which is exactly the failure mode a committed expectation is supposed to rule out.
#[test]
fn committed_unit_cube_fixture_is_a_closed_unit_cube() {
    let mesh = unit_cube_semio_mesh();
    let projection = project(&mesh);
    assert_eq!(projection.vertex_count, 8, "the fixture keeps a shared 8-vertex pool");
    assert_is_unit_cube("fixture", &projection);
    assert_oracle_agrees_on_unit_cube("fixture", &mesh);
}

/// 🚫️ A mesh with no positions is a typed error, never an empty file that looks like a successful
/// export — the whole point of the ticket.
#[test]
fn empty_preview_mesh_is_a_typed_error_not_an_empty_export() {
    let error = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::mesh_bridge::semio_mesh_from_mesh_data(&semio_framework_plugin::MeshData::default()).expect_err("an empty preview must not convert");
    assert!(error.to_string().contains("no preview geometry"), "the error names the cause, got {error}");
}
//#endregion 🧪️FixtureLaws

//#region 🌍️CrossLanguage
/// 🌍️ The three committed files under `🧫️fixtures/🚪️io/🧊️unit-cube/*second-implementation.*` were written
/// by `🧪️tests/🚪️io-procedural-3d-1/🐍️.py` — an independent Python implementation of the ASCII STL,
/// Wavefront OBJ and ASCII PLY grammars, written from the format specifications rather than from
/// this repository's Rust. Feeding them through this artifact's real import leaves is the half of
/// the differential our own round trips cannot supply: bytes a DIFFERENT implementation produced,
/// in a different language, that our reader has to agree with.
///
/// The Python side asserts the mirror law on the same bytes (`self_check`, runnable directly), and
/// the `🥒️.feature` beside it registers both directions with the repository test platform. Each
/// case below reads a file THIS repository did not write and requires the cube back out of it.
fn assert_second_implementation_file(label: &str, recovered: SemioMeshSnapshot) {
    assert_is_unit_cube(label, &project(&recovered));
    assert_oracle_agrees_on_unit_cube(label, &recovered);
}

#[test]
fn stl_written_by_the_python_second_implementation_reads_back_as_the_unit_cube() {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::stl::v_ascii::any as leaf;
    let bytes = include_bytes!("../../../🧫️fixtures/🚪️io/🧊️unit-cube/🔺️second-implementation.stl");
    assert_second_implementation_file("stl (second implementation)", leaf::mesh_from_bytes(bytes).expect("the second implementation's stl imports"));
}

#[test]
fn obj_written_by_the_python_second_implementation_reads_back_as_the_unit_cube() {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::obj::v3_0::any as leaf;
    let bytes = include_bytes!("../../../🧫️fixtures/🚪️io/🧊️unit-cube/🗿️second-implementation.obj");
    assert_second_implementation_file("obj (second implementation)", leaf::mesh_from_bytes(bytes).expect("the second implementation's obj imports"));
}

#[test]
fn ply_written_by_the_python_second_implementation_reads_back_as_the_unit_cube() {
    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::import::deserializers::artifacts::ply::v1_0::any as leaf;
    let bytes = include_bytes!("../../../🧫️fixtures/🚪️io/🧊️unit-cube/🧱️second-implementation.ply");
    assert_second_implementation_file("ply (second implementation)", leaf::mesh_from_bytes(bytes).expect("the second implementation's ply imports"));
}

//#endregion 🌍️CrossLanguage
