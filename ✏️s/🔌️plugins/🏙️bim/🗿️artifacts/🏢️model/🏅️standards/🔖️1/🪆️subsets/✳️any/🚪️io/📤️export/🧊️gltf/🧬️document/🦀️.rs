//! 🧬️ The typed glTF 2.0 document of the BIM export: PBR materials, triangle meshes (one primitive per material) and a node tree, written as one JSON
//! document plus one binary buffer. Positions and normals are `f32` triples in glTF's Y-up frame; transforms are the node TRS of the spec.
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html

use super::container::pack_glb;
use semio_framework_value::DslValue;

const FLOAT: u64 = 5126;
const UNSIGNED_INT: u64 = 5125;
const ARRAY_BUFFER: u64 = 34962;
const ELEMENT_ARRAY_BUFFER: u64 = 34963;
const TRIANGLES: u64 = 4;

//#region 🔖️Values
/// 🎨️ A PBR metallic-roughness material; `blend` makes it transparent through the alpha of `color`.
#[derive(Clone, Debug, PartialEq)]
pub struct GltfMaterial {
    pub name: String,
    pub color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub blend: bool,
}

/// 🔺️ One draw call: indexed triangles of one material.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GltfPrimitive {
    pub material: usize,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
}

impl GltfPrimitive {
    /// 🔢️ The number of triangles.
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// 📦️ The componentwise minimum and maximum of the positions, `None` without a vertex.
    pub fn bounds(&self) -> Option<([f32; 3], [f32; 3])> {
        let mut corners = self.positions.chunks_exact(3);
        let first = corners.next()?;
        let (mut min, mut max) = ([first[0], first[1], first[2]], [first[0], first[1], first[2]]);
        for point in corners {
            for axis in 0..3 {
                min[axis] = min[axis].min(point[axis]);
                max[axis] = max[axis].max(point[axis]);
            }
        }
        Some((min, max))
    }
}

/// 🧊️ A named set of primitives.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GltfMesh {
    pub name: String,
    pub primitives: Vec<GltfPrimitive>,
}

/// 🌳️ A node: an optional translation and rotation quaternion `[x, y, z, w]`, an optional mesh, children and free-form `extras`.
#[derive(Clone, Debug, PartialEq)]
pub struct GltfNode {
    pub name: String,
    pub translation: Option<[f64; 3]>,
    pub rotation: Option<[f64; 4]>,
    pub mesh: Option<usize>,
    pub children: Vec<usize>,
    pub extras: DslValue,
}

/// 🧊️ The whole document: one scene whose roots are `roots`.
#[derive(Clone, Debug, PartialEq)]
pub struct GltfModel {
    pub name: String,
    pub extras: DslValue,
    pub materials: Vec<GltfMaterial>,
    pub meshes: Vec<GltfMesh>,
    pub nodes: Vec<GltfNode>,
    pub roots: Vec<usize>,
}
//#endregion 🔖️Values

//#region 🔖️Json
fn object<const N: usize>(rows: [(&str, DslValue); N]) -> DslValue {
    DslValue::object(rows.into_iter().map(|(key, value)| (key.to_string(), value)))
}

fn index(value: usize) -> DslValue {
    DslValue::uint(value as u64)
}

fn floats(values: &[f64]) -> DslValue {
    DslValue::Array(values.iter().map(|value| DslValue::float(*value)).collect())
}

fn singles(values: &[f32]) -> DslValue {
    DslValue::Array(values.iter().map(|value| DslValue::float(f64::from(*value))).collect())
}

fn text(value: &str) -> DslValue {
    DslValue::String(value.to_string())
}

fn material_json(material: &GltfMaterial) -> DslValue {
    let mut rows = vec![
        ("name".to_string(), text(&material.name)),
        ("pbrMetallicRoughness".to_string(), object([("baseColorFactor", singles(&material.color)), ("metallicFactor", DslValue::float(f64::from(material.metallic))), ("roughnessFactor", DslValue::float(f64::from(material.roughness)))])),
    ];
    if material.blend {
        rows.push(("alphaMode".to_string(), text("BLEND")));
    }
    DslValue::object(rows)
}

fn node_json(node: &GltfNode) -> DslValue {
    let mut rows = vec![("name".to_string(), text(&node.name))];
    if let Some(translation) = node.translation {
        rows.push(("translation".to_string(), floats(&translation)));
    }
    if let Some(rotation) = node.rotation {
        rows.push(("rotation".to_string(), floats(&rotation)));
    }
    if let Some(mesh) = node.mesh {
        rows.push(("mesh".to_string(), index(mesh)));
    }
    if !node.children.is_empty() {
        rows.push(("children".to_string(), DslValue::Array(node.children.iter().map(|child| index(*child)).collect())));
    }
    if !node.extras.is_null() {
        rows.push(("extras".to_string(), node.extras.clone()));
    }
    DslValue::object(rows)
}

#[derive(Default)]
struct Buffer {
    bytes: Vec<u8>,
    views: Vec<DslValue>,
    accessors: Vec<DslValue>,
}

impl Buffer {
    fn view(&mut self, data: &[u8], target: u64) -> usize {
        let offset = self.bytes.len();
        self.bytes.extend_from_slice(data);
        self.views.push(object([("buffer", index(0)), ("byteOffset", index(offset)), ("byteLength", index(data.len())), ("target", DslValue::uint(target))]));
        self.views.len() - 1
    }

    fn accessor(&mut self, view: usize, component: u64, count: usize, kind: &str, range: Option<([f32; 3], [f32; 3])>) -> usize {
        let mut rows = vec![("bufferView", index(view)), ("componentType", DslValue::uint(component)), ("count", index(count)), ("type", text(kind))];
        if let Some((min, max)) = range {
            rows.push(("min", singles(&min)));
            rows.push(("max", singles(&max)));
        }
        self.accessors.push(DslValue::object(rows.into_iter().map(|(key, value)| (key.to_string(), value))));
        self.accessors.len() - 1
    }

    fn primitive(&mut self, primitive: &GltfPrimitive) -> DslValue {
        let bytes = |values: &[f32]| values.iter().flat_map(|value| value.to_le_bytes()).collect::<Vec<u8>>();
        let corners = primitive.positions.len() / 3;
        let positions = self.view(&bytes(&primitive.positions), ARRAY_BUFFER);
        let positions = self.accessor(positions, FLOAT, corners, "VEC3", primitive.bounds());
        let normals = self.view(&bytes(&primitive.normals), ARRAY_BUFFER);
        let normals = self.accessor(normals, FLOAT, corners, "VEC3", None);
        let indices = self.view(&primitive.indices.iter().flat_map(|value| value.to_le_bytes()).collect::<Vec<u8>>(), ELEMENT_ARRAY_BUFFER);
        let indices = self.accessor(indices, UNSIGNED_INT, primitive.indices.len(), "SCALAR", None);
        object([("attributes", object([("POSITION", index(positions)), ("NORMAL", index(normals))])), ("indices", index(indices)), ("material", index(primitive.material)), ("mode", DslValue::uint(TRIANGLES))])
    }
}

impl GltfModel {
    /// 🧾️ The JSON text of the document and the binary buffer its accessors point into.
    pub fn to_json_and_buffer(&self) -> (String, Vec<u8>) {
        let mut buffer = Buffer::default();
        let meshes: Vec<DslValue> = self.meshes.iter().map(|mesh| object([("name", text(&mesh.name)), ("primitives", DslValue::Array(mesh.primitives.iter().map(|primitive| buffer.primitive(primitive)).collect()))])).collect();
        let mut rows = vec![
            ("asset".to_string(), object([("version", text("2.0")), ("generator", text("semio BIM")), ("extras", self.extras.clone())])),
            ("scene".to_string(), index(0)),
            ("scenes".to_string(), DslValue::Array(vec![object([("name", text(&self.name)), ("nodes", DslValue::Array(self.roots.iter().map(|root| index(*root)).collect()))])])),
            ("nodes".to_string(), DslValue::Array(self.nodes.iter().map(node_json).collect())),
            ("meshes".to_string(), DslValue::Array(meshes)),
            ("materials".to_string(), DslValue::Array(self.materials.iter().map(material_json).collect())),
        ];
        if !buffer.bytes.is_empty() {
            rows.push(("accessors".to_string(), DslValue::Array(buffer.accessors)));
            rows.push(("bufferViews".to_string(), DslValue::Array(buffer.views)));
            rows.push(("buffers".to_string(), DslValue::Array(vec![object([("byteLength", index(buffer.bytes.len()))])])));
        }
        (semio_framework_pack_json::to_json_string(&DslValue::object(rows)), buffer.bytes)
    }

    /// 📦️ The binary glTF (`model/gltf-binary`) bytes of the document.
    pub fn to_glb(&self) -> Vec<u8> {
        let (json, buffer) = self.to_json_and_buffer();
        pack_glb(&json, &buffer)
    }
}
//#endregion 🔖️Json

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
