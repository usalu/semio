from pathlib import Path
import json

text = Path("🧰️framework/🔨️modules/🏗️mesh-engine/🚪️io/📝️text/🦀️.rs").read_text()
begin = text.index("/// 🖱️ Reads OBJ")
end = text.index("/// 🔎️ Parses bounded", begin)
index = text.index("fn obj_resolve_index(")
source = text[begin:end] + text[index:text.index("//#endregion Obj", index)]
mesh = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh")
fixture = json.loads((mesh / "🧪️tests/🧫️fixtures/🥽️retained-workbench/🔣️.json").read_text())["objImport"]
print(r'''
use std::collections::BTreeMap;
type HistoryFoldIndex<K, V> = BTreeMap<K, V>;
pub struct PolygonMeshSource {
    vertices: Vec<[f32; 3]>,
    faces: Vec<Vec<u32>>,
    surface: Vec<()>,
    attributes: HistoryFoldIndex<String, ()>,
    materials: HistoryFoldIndex<String, ()>,
    textures: HistoryFoldIndex<String, ()>,
}
''')
for line in source.splitlines():
    if not line.startswith("#[derive(semio_framework_value_derive::RetireOwned)]"):
        print(line)
print(f'const FIXTURE: &str = {json.dumps(fixture["text"], ensure_ascii=False)};')
print(f'const VERTICES: usize = {fixture["vertices"]};')
print(f'const FACES: usize = {fixture["faces"]};')
print(r'''
#[test]
fn parses_polygon_fixture_and_honors_turns() {
    let input = format!("#{}\n{}", "x".repeat(1023), FIXTURE);
    let mut cursor = ObjSourceCursor::new();
    assert!(cursor.step(&input, 0).unwrap().is_none());
    assert_eq!(cursor.progress(&input).0, 0);
    let mut output = None;
    for _ in 0..32 {
        let offset = cursor.progress(&input).0;
        let result = cursor.step(&input, 1).unwrap();
        assert!(cursor.progress(&input).0 <= offset + 256);
        if result.is_some() { output = result; break; }
    }
    let mesh = output.unwrap();
    assert_eq!(mesh.vertices.len(), VERTICES);
    assert_eq!(mesh.faces.len(), FACES);
    assert_eq!(mesh.faces, vec![vec![0, 1, 2, 3]]);
    assert!(mesh.surface.is_empty() && mesh.attributes.is_empty() && mesh.materials.is_empty() && mesh.textures.is_empty());
}

#[test]
fn rejects_large_forward_index_without_truncating() {
    let input = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 4294967299";
    let mut cursor = ObjSourceCursor::new();
    let result = cursor.step(input, 64);
    assert!(matches!(result, Err(message) if message == "obj: face index out of range"));
}
''')

import base64

binary = Path("🧰️framework/🔨️modules/🏗️mesh-engine/🚪️io/💾️binary/🦀️.rs").read_text()
begin = binary.index("/// 📝️ Writes the binary STL header")
end = binary.index("//#endregion Stl", begin)
root = Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any")
rows = json.loads((root / "🚪️io/📐️geometry/📼️mesh-interchange/🧪️tests/🧫️fixtures/🔣️.json").read_text())["cases"]
specimen = next(row for row in rows if row["name"] == "import the cube from binary stl")["inputs"]["data"]
print(r'''
mod stl {
    #[derive(Debug, Default)]
    pub struct MeshData { positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32> }
    impl MeshData {
        fn triangle_count(&self) -> usize { self.indices.len() / 3 }
        fn vertex_count(&self) -> usize { self.positions.len() / 3 }
    }
''')
for line in binary[begin:end].splitlines():
    if not line.startswith("#[derive(semio_framework_value_derive::RetireOwned)]"):
        print(line)
print(f'const FIXTURE: &[u8] = &{list(base64.b64decode(specimen))};')
print(r'''
    #[test]
    fn one_facet_turns_match_independent_cube_file() {
        let mut reader = BinaryStlReader::new(FIXTURE, 12).unwrap();
        let mut mesh = None;
        for turn in 1..=12 {
            mesh = reader.step(FIXTURE, 1).unwrap();
            assert_eq!(mesh.is_some(), turn == 12);
        }
        let mesh = mesh.unwrap();
        assert_eq!(mesh.triangle_count(), 12);
        assert_eq!(mesh.vertex_count(), 36);
        let mut writer = BinaryStlWriter::default();
        let mut bytes = Vec::new();
        for turn in 1..=12 {
            assert_eq!(writer.step(&mesh, &mut bytes, 1), turn == 12);
            assert_eq!(bytes.len(), 84 + 50 * turn);
        }
        assert_eq!(bytes, mesh_to_stl(&mesh));
        let readback = mesh_from_stl(&bytes).unwrap();
        assert_eq!(readback.positions, mesh.positions);
        assert_eq!(readback.normals, mesh.normals);
        assert_eq!(readback.indices, mesh.indices);
        assert!(BinaryStlReader::new(FIXTURE, 11).is_err());
        assert!(BinaryStlReader::new(&FIXTURE[..FIXTURE.len()-1], 12).is_err());
    }
}
''')
