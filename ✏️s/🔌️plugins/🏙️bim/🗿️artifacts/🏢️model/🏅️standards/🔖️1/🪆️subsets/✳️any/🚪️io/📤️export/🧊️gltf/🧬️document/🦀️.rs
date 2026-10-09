//! 🧬️ The BIM scene of the glTF export: PBR materials, triangle meshes (one primitive per material) and a node tree, lowered into the typed `s.stdio.gltf` snapshot (one document plus one binary buffer).
//! Positions and normals are `f32` triples in glTF's Y-up frame; transforms are the node TRS of the spec.
//! 📎 https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html

use super::codec;
use semio_framework_value::DslValue;

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

//#region 🔖️Snapshot
fn json(value: &DslValue) -> codec::Json {
    match value {
        DslValue::Null => codec::Json::Null,
        DslValue::Bool(flag) => codec::Json::Bool(*flag),
        DslValue::Number(number) => codec::Json::Number(number.as_f64()),
        DslValue::String(text) => codec::Json::String(text.clone()),
        DslValue::Bytes(bytes) => codec::Json::Array(bytes.iter().map(|byte| codec::Json::Number(f64::from(*byte))).collect()),
        DslValue::Array(items) => codec::Json::Array(items.iter().map(json).collect()),
        DslValue::Object(rows) => codec::Json::Object(rows.iter().map(|(key, item)| (key.clone(), json(item))).collect()),
    }
}

fn present(value: &DslValue) -> Option<codec::Json> {
    (!value.is_null()).then(|| json(value))
}

fn wide<const N: usize>(values: [f32; N]) -> [f64; N] {
    values.map(f64::from)
}

fn material(material: &GltfMaterial) -> codec::Material {
    codec::Material {
        name: Some(material.name.clone()),
        pbr_metallic_roughness: Some(codec::Pbr { base_color_factor: wide(material.color), metallic_factor: f64::from(material.metallic), roughness_factor: f64::from(material.roughness), ..codec::Pbr::default() }),
        alpha_mode: if material.blend { codec::AlphaMode::Blend } else { codec::AlphaMode::Opaque },
        ..codec::Material::default()
    }
}

fn node(node: &GltfNode) -> codec::Node {
    codec::Node { name: Some(node.name.clone()), translation: node.translation, rotation: node.rotation, mesh: node.mesh, children: node.children.clone(), extras: present(&node.extras), ..codec::Node::default() }
}

#[derive(Default)]
struct Binary {
    bytes: Vec<u8>,
    views: Vec<codec::BufferView>,
    accessors: Vec<codec::Accessor>,
}

impl Binary {
    fn view(&mut self, data: &[u8], target: u64) -> usize {
        let offset = self.bytes.len();
        self.bytes.extend_from_slice(data);
        self.views.push(codec::BufferView { buffer: 0, byte_offset: offset, byte_length: data.len(), byte_stride: None, target: Some(target), name: None, extensions: None, extras: None });
        self.views.len() - 1
    }

    fn accessor(&mut self, view: usize, component_type: codec::ComponentType, count: usize, kind: codec::AccessorType, range: Option<([f32; 3], [f32; 3])>) -> usize {
        let (min, max) = range.map_or((None, None), |(min, max)| (Some(wide(min).to_vec()), Some(wide(max).to_vec())));
        self.accessors.push(codec::Accessor { buffer_view: Some(view), byte_offset: 0, component_type, normalized: false, count, kind, max, min, sparse: None, name: None, extensions: None, extras: None });
        self.accessors.len() - 1
    }

    fn primitive(&mut self, primitive: &GltfPrimitive) -> codec::Primitive {
        let bytes = |values: &[f32]| values.iter().flat_map(|value| value.to_le_bytes()).collect::<Vec<u8>>();
        let corners = primitive.positions.len() / 3;
        let positions = self.view(&bytes(&primitive.positions), ARRAY_BUFFER);
        let positions = self.accessor(positions, codec::ComponentType::Float, corners, codec::AccessorType::Vec3, primitive.bounds());
        let normals = self.view(&bytes(&primitive.normals), ARRAY_BUFFER);
        let normals = self.accessor(normals, codec::ComponentType::Float, corners, codec::AccessorType::Vec3, None);
        let indices = self.view(&primitive.indices.iter().flat_map(|value| value.to_le_bytes()).collect::<Vec<u8>>(), ELEMENT_ARRAY_BUFFER);
        let indices = self.accessor(indices, codec::ComponentType::UnsignedInt, primitive.indices.len(), codec::AccessorType::Scalar, None);
        codec::Primitive { attributes: vec![("POSITION".into(), positions), ("NORMAL".into(), normals)], indices: Some(indices), material: Some(primitive.material), mode: Some(TRIANGLES), ..codec::Primitive::default() }
    }
}

impl GltfModel {
    /// 🧾️ The typed `s.stdio.gltf` snapshot of the scene: one scene, the nodes, meshes and materials, and one buffer holding every accessor's data (none for a scene without meshes).
    pub fn to_snapshot(&self) -> codec::Snapshot {
        let mut binary = Binary::default();
        let meshes = self.meshes.iter().map(|mesh| codec::Mesh { name: Some(mesh.name.clone()), primitives: mesh.primitives.iter().map(|primitive| binary.primitive(primitive)).collect(), ..codec::Mesh::default() }).collect();
        let asset = codec::Asset { generator: Some("semio BIM".into()), extras: present(&self.extras), ..codec::Asset::default() };
        let scene = codec::Scene { nodes: self.roots.clone(), name: Some(self.name.clone()), extensions: None, extras: None };
        let buffers = if binary.bytes.is_empty() { Vec::new() } else { vec![codec::Buffer { byte_length: binary.bytes.len(), uri: None, name: None, extensions: None, extras: None }] };
        let document = codec::Document {
            asset,
            scene: Some(0),
            scenes: vec![scene],
            nodes: self.nodes.iter().map(node).collect(),
            meshes,
            materials: self.materials.iter().map(material).collect(),
            accessors: binary.accessors,
            buffer_views: binary.views,
            buffers,
            ..codec::Document::default()
        };
        codec::Snapshot { document, buffers: if binary.bytes.is_empty() { Vec::new() } else { vec![binary.bytes] }, source_form: codec::SourceForm::Glb, ..codec::Snapshot::default() }
    }

    /// 📦️ The binary glTF (`model/gltf-binary`) bytes of the scene.
    pub fn to_glb(&self) -> Result<Vec<u8>, String> {
        codec::encode(&self.to_snapshot())
    }
}
//#endregion 🔖️Snapshot

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
