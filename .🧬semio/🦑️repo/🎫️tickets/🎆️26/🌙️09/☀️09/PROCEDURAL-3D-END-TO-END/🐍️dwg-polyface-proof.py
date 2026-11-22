"""🔺 Exercises the production DWG face projection against acadrust face conventions."""
from pathlib import Path
import json
import re
import subprocess
import sys

ticket = Path(__file__).parent
out = ticket / "🗑️generated"
root = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any")
mesh = (root / "🧬️schema/🧮️geometry/🦀️.rs").read_text()
projection = mesh[mesh.index("pub fn mesh_to_dwg_drawing"):mesh.index("//#endregion DwgMeshBridge")]
engine = Path("🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs").read_text()
normals = engine[engine.index("    pub fn vertex_count(&self)"):engine.index("    pub fn aabb(&self)")]
fixture = json.loads((root / "🚪️io/📤️export/🧪️tests/🧫️fixtures/polyface-mesh.json").read_text())
acadrust = next(Path(".cache/b/r/m/cargo/home/registry/src").glob("*/acadrust-0.5.5"))
faces = (acadrust / "src/entities/polyface_mesh.rs").read_text()
start = faces.index("    pub fn is_triangle(")
last = faces.index("    pub fn vertex_indices(", start)
reader = faces[start:faces.index("\n    }", last) + len("\n    }")]
check = "Ok(dwg_drawing_to_mesh(drawing))" if "--baseline" in sys.argv else "dwg_drawing_to_mesh(drawing)"
save = "Ok(mesh_to_dwg_drawing(mesh))" if "--baseline" in sys.argv or "--unbatched" in sys.argv else "mesh_to_dwg_drawing(mesh)"
exports = ", ".join(f"({row['triangles']}, {str(row['distinctVertices']).lower()}, "
    + "vec![" + ", ".join(f"({vertices},{faces})" for vertices, faces in row["partSizes"]) + "])"
    for row in fixture["exports"])

rows = ", ".join(f"({row['corners']!r}, {row['triangles']!r}.as_slice())" for row in fixture["faces"])
source = out / "dwg-polyface.rs"
source.write_text("""
extern crate self as semio_framework_mesh_engine;

#[derive(Default)]
pub struct MeshData { positions: Vec<f32>, indices: Vec<u32>, normals: Vec<f32> }
impl MeshData {
""" + normals + """
}

""" + mesh.split("//#region DwgTypes", 1)[1].split("//#endregion DwgTypes", 1)[0] + """
struct PolyfaceFace { index1: i16, index2: i16, index3: i16, index4: i16 }
impl PolyfaceFace {
""" + reader + "\n}\n" + projection + """
fn checked(drawing: &DwgDrawing) -> Result<MeshData, String> { CHECK }
fn drawing_with(corners: [i32;4]) -> DwgDrawing {
    DwgDrawing { entities: vec![DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh {
        vertices: vec!VERTICES, faces: vec![corners],
    } }], ..Default::default() }
}

#[test]
fn polyface_markers_and_visibility_preserve_the_intended_triangles() {
    for (corners, expected) in [ROWS] {
        let mesh = checked(&drawing_with(corners)).unwrap();
        let oracle = PolyfaceFace { index1: corners[0] as i16, index2: corners[1] as i16,
            index3: corners[2] as i16, index4: corners[3] as i16 };
        assert_eq!(oracle.vertex_count(), if expected.len() == 3 { 3 } else { 4 });
        assert_eq!(&oracle.vertex_indices().iter().map(|i| *i as u32 - 1).collect::<Vec<_>>()[..3], &expected[..3]);
        assert_eq!(mesh.indices, expected);
    }
}

#[test]
fn polyface_rejects_missing_and_out_of_range_required_indices() {
    for corners in INVALID {
        assert!(checked(&drawing_with(corners)).is_err());
    }
}

#[test]
fn triangle_3dface_leaves_no_unused_duplicate_position() {
    let drawing = DwgDrawing { entities: vec![
        DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [0.0; 3] } },
        DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Face3d {
            corners: [[0.,0.,0.], [1.,0.,0.], [0.,1.,0.], [0.,1.,0.]],
        } },
    ], ..Default::default() };
    let mesh = checked(&drawing).unwrap();
    assert_eq!(mesh.vertex_count(), 3);
    assert_eq!(mesh.indices, [0,1,2]);
}

#[test]
fn exported_polyface_parts_fit_native_index_and_count_widths() {
    for (count, unique, expected) in EXPORTS {
        let vertex_count = if unique { 3 * count } else { 3 };
        let mut mesh = MeshData::default();
        for vertex in 0..vertex_count {
            mesh.positions.extend([vertex as f32, (vertex % 2) as f32, 0.]);
        }
        for face in 0..count {
            let base = if unique { 3 * face } else { 0 };
            mesh.indices.extend([base as u32, base as u32 + 1, base as u32 + 2]);
        }
        let drawing = saved(&mesh).unwrap();
        let mut sizes = Vec::new();
        let mut facet = 0;
        for entity in drawing.entities {
            let DwgGeometry::PolyfaceMesh { vertices, faces } = entity.geometry else { panic!("polyface required"); };
            assert!(vertices.len() <= i16::MAX as usize, "signed polyface vertex index limit");
            assert!(faces.len() <= u16::MAX as usize, "polyface face count limit");
            sizes.push((vertices.len(), faces.len()));
            for [a,b,c,d] in faces {
                assert_eq!(d, 0);
                for (corner, local) in [a,b,c].into_iter().enumerate() {
                    let original = mesh.indices[3 * facet + corner] as usize;
                    assert_eq!(vertices[local as usize - 1].map(|x| x as f32), mesh.positions[3 * original..3 * original + 3]);
                }
                facet += 1;
            }
        }
        assert_eq!(sizes, expected);
        assert_eq!(facet, count);
    }
}

fn saved(mesh: &MeshData) -> Result<DwgDrawing, String> { SAVE }
""".replace("CHECK", check).replace("SAVE", save).replace("EXPORTS", "[" + exports + "]").replace("VERTICES", repr([list(map(float, v)) for v in fixture["vertices"]]))
.replace("ROWS", rows).replace("INVALID", repr(fixture["invalidFaces"])))
if "--projection" in sys.argv:
    cases = json.loads((root / "🚪️io/📤️export/🧪️tests/🧫️fixtures/mesh-projection.json").read_text())
    with source.open("a") as proof:
        proof.write(r"""
#[test]
fn owned_drawing_projects_one_entity_position_face_or_normal_per_step() {
    let entities = vec![
        DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices: vec!POLY_VERTICES, faces: vec!POLY_FACES } },
        DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Point { at: [0.0; 3] } },
        DwgEntity { layer: 0, color: DwgColor::ByLayer, geometry: DwgGeometry::Face3d { corners: FACE_CORNERS } },
    ];
    let cold = dwg_entities_to_mesh(&entities).unwrap();
    let mut read = DwgMeshProjection::new(entities.into_iter());
    assert!(read.take_mesh().is_none());
    let mut counts = [0; 4];
    let mut progress = 0.0;
    loop {
        let step = read.step().unwrap();
        assert!(read.progress() >= progress && read.progress() <= 1.0);
        progress = read.progress();
        match step {
            DwgMeshProjectionStep::Entity => counts[0] += 1,
            DwgMeshProjectionStep::Position => counts[1] += 1,
            DwgMeshProjectionStep::Face => counts[2] += 1,
            DwgMeshProjectionStep::Normal => counts[3] += 1,
            DwgMeshProjectionStep::Complete => break,
        }
        assert!(read.take_mesh().is_none());
    }
    assert_eq!(counts, UNIT_COUNTS);
    let actual = read.take_mesh().unwrap();
    assert_eq!(actual.indices, EXPECTED_INDICES);
    assert_eq!(actual.positions, cold.positions);
    assert_eq!(actual.normals, cold.normals);
    assert!(read.take_mesh().is_none());
    let oracle = include_str!("dwg-polyface-normals.txt").lines().map(|value| value.parse::<f32>().unwrap()).collect::<Vec<_>>();
    assert_eq!(actual.normals.len(), oracle.len());
    for (&actual, expected) in actual.normals.iter().zip(oracle) {
        assert!((actual - expected).abs() < 1e-6, "expected {expected}, saw {actual}");
    }
}

#[test]
fn late_invalid_face_never_releases_a_partial_mesh() {
    let mut drawing = drawing_with([1, 2, 3, 0]);
    let DwgGeometry::PolyfaceMesh { faces, .. } = &mut drawing.entities[0].geometry else { panic!("polyface required") };
    faces.push([1, 2, 3, 5]);
    let mut read = DwgMeshProjection::new(drawing.entities.into_iter());
    let error = loop {
        match read.step() {
            Ok(DwgMeshProjectionStep::Complete) => panic!("invalid face published"),
            Ok(_) => assert!(read.take_mesh().is_none()),
            Err(error) => break error,
        }
    };
    assert_eq!(read.step().err(), Some(error));
    assert!(read.take_mesh().is_none());
}
""".replace("POLY_VERTICES", repr([list(map(float, vertex)) for vertex in cases["polyfaceVertices"]]))
        .replace("POLY_FACES", repr(cases["polyfaceFaces"]))
        .replace("FACE_CORNERS", repr([list(map(float, vertex)) for vertex in cases["face3dCorners"]]))
        .replace("EXPECTED_INDICES", repr(cases["indices"]))
        .replace("UNIT_COUNTS", repr(list(cases["unitCounts"].values()))))

if "--projection" in sys.argv:
    snapshot = root / "🧬️schema/📸️snapshot"
    bodies = (snapshot / "📦️objects/📐️entities/🦀️.rs").read_text().split("//#region 🔖️DrawingModel", 1)[1].split("impl DwgEntityBody", 1)[0]
    bodies = re.sub(r"^\s*#\[(?:value|canonical_json)[^\n]*\]\n", "\n", bodies, flags=re.MULTILINE)
    bodies = re.sub(r"#\[derive\(([^\n]*)\)\]", lambda attr: "#[derive(" + ", ".join(part for part in attr[1].split(", ") if "::" not in part) + ")]", bodies)
    logical = (snapshot / "✏️drawing/🦀️.rs").read_text()
    index = logical[logical.index("#[derive(Default)]\npub struct DwgEntityProjection"):logical.index("/// 🗂️ Handles of the standard objects")]
    with source.open("a") as proof:
        proof.write(bodies + r"""
use std::collections::HashMap;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DwgComplexColor;
pub struct DwgLogicalObject { handle: u64, body: Option<DwgLogicalObjectBody> }
enum DwgLogicalObjectBody { TableRecord(DwgTableRecordBody), Entity(DwgEntityBody) }
enum DwgTableRecordBody { Layer(()) }
""" + index + r"""
#[test]
fn wide_curve_and_text_rows_finish_in_bounded_projection_turns() {
    let make = |handle, body| DwgLogicalObject { handle, body: Some(DwgLogicalObjectBody::Entity(body)) };
    let text = TEXT_PATTERN.repeat(TEXT_REPEATS);
    let records = [
        make(1, DwgEntityBody::LwPolyline(DwgLwPolylineEntity {
            closed: true,
            elevation: -4.0,
            vertices: vec![
                DwgLwPolylineVertex { point: vec![-6., 5.], bulge: 0.75, ..Default::default() },
                DwgLwPolylineVertex { point: vec![3., -9.], bulge: -0.5, ..Default::default() },
                DwgLwPolylineVertex { point: vec![2.], ..Default::default() },
            ],
            ..Default::default()
        })),
        make(2, DwgEntityBody::Spline(DwgSplineEntity {
            degree: 2,
            control_points: vec![-4., 4., 1., 3., -1., 2., 1., 6., 8., 777.],
            knots: vec![0., 0., 0.5, 1., 1.], weights: vec![1., 0.25, 1.],
            ..Default::default()
        })),
        make(3, DwgEntityBody::Text(DwgTextEntity {
            insertion: vec![4.5, 5.5], elevation: 7., height: 0.5, rotation: 0.2, value: text.clone(), ..Default::default()
        })),
    ];
    let expected = [
        DwgGeometry::LwPolyline { closed: true, elevation: -4.0, vertices: vec![[-6.,5.],[3.,-9.],[2.,0.]], bulges: vec![0.75,-0.5,0.] },
        DwgGeometry::Spline { degree: 2, control_points: vec![[-4.,4.,1.],[3.,-1.,2.],[1.,6.,8.]], knots: vec![0.,0.,0.5,1.,1.], weights: vec![1.,0.25,1.] },
        DwgGeometry::Text { at: [4.5,5.5,7.], height: 0.5, rotation: 0.2, content: text },
    ];
    let projection = DwgEntityProjection::default();
    let mut extents = DwgDrawingExtents::default();
    for (row, limit) in ROW_LIMITS.into_iter().enumerate() {
        let expected_turns = ROW_TURNS[row];
        let mut cursor = projection.begin(&records, row).unwrap();
        let mut last = 0;
        let mut pending = 0;
        loop {
            let result = cursor.step(&projection, &records, &mut extents).unwrap();
            let (done, total) = cursor.progress();
            assert!((1..=limit).contains(&(done - last)));
            match result {
                std::task::Poll::Ready(entity) => {
                    assert_eq!(pending + 1, expected_turns);
                    assert_eq!(done, total);
                    assert_eq!(entity.geometry, expected[row]);
                    break;
                }
                std::task::Poll::Pending => {
                    assert!(done < total);
                    last = done;
                    pending += 1;
                }
            }
        }
    }
    let mut drawing = DwgDrawing::default();
    extents.apply(&mut drawing);
    assert_eq!(drawing.extmin, CURVE_TEXT_MIN);
    assert_eq!(drawing.extmax, CURVE_TEXT_MAX);
}

#[test]
fn logical_rows_resolve_original_layer_and_forward_vertex_references() {
    let expected: Vec<[f64; 3]> = vec!POLY_VERTICES;
    let object = |handle, body| DwgLogicalObject { handle, body: Some(DwgLogicalObjectBody::Entity(body)) };
    let layer = |handle| DwgLogicalObject { handle, body: Some(DwgLogicalObjectBody::TableRecord(DwgTableRecordBody::Layer(()))) };
    let vertex = |handle, point: [f64; 3]| object(handle, DwgEntityBody::Vertex(DwgVertexEntity { point: point.to_vec(), ..Default::default() }));
    let face = |handle, indices: [i16; 4]| object(handle, DwgEntityBody::PolyfaceFace(DwgPolyfaceFaceEntity { indices: indices.to_vec(), ..Default::default() }));
    let parent = object(15, DwgEntityBody::PolyfaceMesh(DwgPolyfaceMeshEntity {
        common: DwgEntityCommon { layer_handle: 100, ..Default::default() },
        vertex_handles: vec![20, 21, 80, 22, 9000, 23, 24, 25, 81],
        ..Default::default()
    }));
    let hidden = object(16, DwgEntityBody::Face3d(DwgFace3dEntity {
        common: DwgEntityCommon { color: DwgEntityColor { kind: DwgEntityColorKind::TrueColor, ..Default::default() }, ..Default::default() },
        corners: vec![-100.; 12],
        ..Default::default()
    }));
    let mut rows = vec![layer(100), layer(100), layer(200), parent, hidden, vertex(20, [-999.; 3])];
    rows.extend(expected.iter().enumerate().map(|(index, &point)| vertex(20 + index as u64, point)));
    let faces: [[i16; 4]; 2] = POLY_FACES;
    rows.push(face(80, faces[0]));
    rows.push(face(81, faces[1]));
    let mut projection = DwgEntityProjection::default();
    for (index, entity) in rows.iter().enumerate() { projection.index(index, entity); }
    let entities: Vec<DwgEntity> = (0..rows.len()).filter_map(|row| projection.entity_parts(&rows, row).unwrap())
        .map(|(layer, color, geometry)| dwg_projected_entity(layer, color, geometry)).collect::<Result<_, _>>().unwrap();
    assert_eq!(entities.len(), 1);
    let mut partial = projection.begin(&rows, 3).unwrap();
    let mut stepped_extents = DwgDrawingExtents::default();
    assert_eq!(partial.progress(), (0, POLYFACE_REFERENCES));
    for index in 1..POLYFACE_REFERENCES {
        assert!(matches!(partial.step(&projection, &rows, &mut stepped_extents).unwrap(), std::task::Poll::Pending));
        assert_eq!(partial.progress(), (index, POLYFACE_REFERENCES));
    }
    let std::task::Poll::Ready(projected) = partial.step(&projection, &rows, &mut stepped_extents).unwrap() else { panic!("last handle must finish the entity"); };
    assert_eq!(projected.geometry, entities[0].geometry);
    let mut stepped_drawing = DwgDrawing::default();
    stepped_extents.apply(&mut stepped_drawing);
    assert_eq!(stepped_drawing.extmin, [0.; 3]);
    assert_eq!(stepped_drawing.extmax, [8.; 3]);
    assert!(partial.step(&projection, &rows, &mut stepped_extents).is_err());
    assert_eq!(entities[0].layer, 0);
    assert_eq!(entities[0].geometry, DwgGeometry::PolyfaceMesh { vertices: expected, faces: faces.map(|face| face.map(i32::from)).to_vec() });
    let mut extents = DwgDrawingExtents::default();
    extents.include(&entities[0].geometry);
    let mut drawing = DwgDrawing { entities, ..Default::default() };
    extents.apply(&mut drawing);
    assert_eq!(drawing.extmin, [0.; 3]);
    assert_eq!(drawing.extmax, [8.; 3]);
    let mesh = dwg_drawing_to_mesh(&drawing).unwrap();
    assert_eq!(mesh.indices, EXPECTED_INDICES);
}
""".replace("POLY_VERTICES", repr([list(map(float, vertex)) for vertex in cases["polyfaceVertices"]]))
        .replace("POLY_FACES", repr(cases["polyfaceFaces"]))
        .replace("EXPECTED_INDICES", repr(cases["indices"][:-3]))
        .replace("TEXT_PATTERN", json.dumps(cases["entityProjection"]["textPattern"], ensure_ascii=False))
        .replace("TEXT_REPEATS", str(cases["entityProjection"]["textRepeats"]))
        .replace("ROW_LIMITS", repr(cases["entityProjection"]["rowLimits"]))
        .replace("ROW_TURNS", repr(cases["entityProjection"]["rowTurns"]))
        .replace("CURVE_TEXT_MIN", repr([float(value) for value in cases["entityProjection"]["curveTextMin"]]))
        .replace("CURVE_TEXT_MAX", repr([float(value) for value in cases["entityProjection"]["curveTextMax"]]))
        .replace("POLYFACE_REFERENCES", str(cases["entityProjection"]["polyfaceReferences"])))


if "--semio" in sys.argv:
    semio = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs").read_text()
    bridge = semio[semio.index("fn append_geometry"):semio.index("//#endregion", semio.index("fn append_geometry"))]
    with source.open("a") as proof:
        proof.write(r"""
#[derive(PartialEq, Debug)]
struct SemioPoint3 { x: f64, y: f64, z: f64 }
#[derive(Default)]
enum SemioTopology { #[default] Triangles }
#[derive(Default)]
struct SemioPrimitive { id: String, topology: SemioTopology, positions: Vec<SemioPoint3>, indices: Vec<u32> }
struct SemioMesh { id: String, primitives: Vec<SemioPrimitive> }
""" + bridge + r"""
#[test]
fn semio_and_native_use_the_same_acadrust_face_corners() {
    for (corners, expected) in [ROWS] {
        let mut drawing = drawing_with(corners);
        drawing.ensure_layer("source");
        let more = drawing.ensure_layer("next");
        drawing.ensure_layer("empty");
        drawing.entities.push(DwgEntity {
            layer: more, color: DwgColor::ByLayer, geometry: DwgGeometry::Face3d {
                corners: [[0.,0.,0.],[1.,0.,0.],[0.,1.,0.],[0.,1.,0.]],
            },
        });
        let meshes = semio_meshes_from_drawing(&drawing).unwrap();
        assert_eq!(meshes.iter().map(|mesh| mesh.id.as_str()).collect::<Vec<_>>(), ["source", "next"]);
        let source = &meshes[0].primitives[0];
        assert_eq!(source.id, "source-mesh");
        assert!(matches!(source.topology, SemioTopology::Triangles));
        assert_eq!(source.indices, expected);
        assert_eq!(source.positions.len(), 4);
        assert_eq!(meshes[1].primitives[0].positions.len(), 3);
        assert_eq!(meshes[1].primitives[0].indices, [0,1,2]);
    }
    let mut drawing = drawing_with([1, 2, 99, 0]);
    drawing.ensure_layer("source");
    assert!(semio_meshes_from_drawing(&drawing).is_err());
}
""".replace("ROWS", rows))

binary = out / "dwgpoly"
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
                "rustc", "--edition=2024", "--test", str(source), "-o", str(binary)], check=True)
subprocess.run(["repo", "tools", "exec", "--light", "python", "--", "-c",
                "import os,sys; os.execv(sys.argv[1], sys.argv[1:])", str(binary)], check=True)
