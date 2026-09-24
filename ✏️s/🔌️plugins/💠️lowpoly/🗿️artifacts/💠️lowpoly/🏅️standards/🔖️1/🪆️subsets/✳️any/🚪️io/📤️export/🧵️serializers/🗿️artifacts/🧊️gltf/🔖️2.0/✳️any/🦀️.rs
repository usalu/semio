//! lowpoly -> gltf
//!
//! Real glTF 2.0 GLB export through `io::encode_glb`: every object with non-empty `mesh_content`
//! becomes one node/mesh pair (world-space positions, fan-triangulated n-gons).
//!
//! 🔖 `IoFidelity::Lossy`: objects and names survive, n-gons come back as triangles, paint does not.
use crate::io::mesh_geometry::world_parts;
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_gltf::io::{encode_glb, GltfAccessorType, GltfComponentType};
use semio_s_artifact_stdio_gltf::schema::snapshot::{
    GltfAccessor, GltfAsset, GltfBuffer, GltfBufferView, GltfDocument, GltfMesh, GltfNode, GltfPrimitive, GltfScene, GltfSourceForm,
    GltfSnapshot,
};
use semio_s_artifact_stdio_gltf::STDIO_GLTF_DOCUMENT_SCHEMA;

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<GltfSnapshot, store::TextError> {
    let parts = world_parts("gltf", snapshot)?;
    let mut bin = Vec::new();
    let mut accessors = Vec::new();
    let mut buffer_views = Vec::new();
    let mut meshes = Vec::new();
    let mut nodes = Vec::new();
    for part in &parts {
        if part.positions.is_empty() || part.faces.is_empty() {
            continue;
        }
        let pos_offset = bin.len();
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for p in &part.positions {
            let v = [p[0] as f32, p[1] as f32, p[2] as f32];
            for i in 0..3 {
                min[i] = min[i].min(v[i]);
                max[i] = max[i].max(v[i]);
                bin.extend_from_slice(&v[i].to_le_bytes());
            }
        }
        let pos_len = bin.len() - pos_offset;
        let pos_view = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: pos_offset,
            byte_length: pos_len,
            byte_stride: None,
            target: Some(34962),
            name: None,
            extensions: None,
            extras: None,
        });
        let pos_accessor = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: Some(pos_view),
            byte_offset: 0,
            component_type: GltfComponentType::Float,
            normalized: false,
            count: part.positions.len(),
            kind: GltfAccessorType::Vec3,
            max: Some(max.iter().map(|v| f64::from(*v)).collect()),
            min: Some(min.iter().map(|v| f64::from(*v)).collect()),
            sparse: None,
            name: None,
            extensions: None,
            extras: None,
        });

        let mut indices: Vec<u32> = Vec::new();
        for face in &part.faces {
            if face.len() < 3 {
                continue;
            }
            for i in 1..face.len() - 1 {
                indices.extend_from_slice(&[face[0], face[i], face[i + 1]]);
            }
        }
        if indices.is_empty() {
            continue;
        }
        while bin.len() % 4 != 0 {
            bin.push(0);
        }
        let idx_offset = bin.len();
        for index in &indices {
            bin.extend_from_slice(&index.to_le_bytes());
        }
        let idx_len = bin.len() - idx_offset;
        let idx_view = buffer_views.len();
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset: idx_offset,
            byte_length: idx_len,
            byte_stride: None,
            target: Some(34963),
            name: None,
            extensions: None,
            extras: None,
        });
        let idx_accessor = accessors.len();
        accessors.push(GltfAccessor {
            buffer_view: Some(idx_view),
            byte_offset: 0,
            component_type: GltfComponentType::UnsignedInt,
            normalized: false,
            count: indices.len(),
            kind: GltfAccessorType::Scalar,
            max: None,
            min: None,
            sparse: None,
            name: None,
            extensions: None,
            extras: None,
        });

        let mesh_index = meshes.len();
        meshes.push(GltfMesh {
            primitives: vec![GltfPrimitive {
                attributes: vec![("POSITION".into(), pos_accessor)],
                indices: Some(idx_accessor),
                material: None,
                mode: Some(4),
                targets: Vec::new(),
                extensions: None,
                extras: None,
            }],
            weights: Vec::new(),
            name: Some(part.name.clone()),
            extensions: None,
            extras: None,
        });
        nodes.push(GltfNode {
            children: Vec::new(),
            mesh: Some(mesh_index),
            camera: None,
            skin: None,
            matrix: None,
            translation: None,
            rotation: None,
            scale: None,
            weights: Vec::new(),
            name: Some(part.name.clone()),
            extensions: None,
            extras: None,
        });
    }

    let node_indices: Vec<usize> = (0..nodes.len()).collect();
    let document = GltfDocument {
        asset: GltfAsset {
            version: "2.0".into(),
            generator: Some("semio.lowpoly".into()),
            copyright: None,
            min_version: None,
            extensions: None,
            extras: None,
        },
        scene: Some(0),
        scenes: vec![GltfScene { nodes: node_indices, name: None, extensions: None, extras: None }],
        nodes,
        meshes,
        accessors,
        buffer_views,
        buffers: vec![GltfBuffer { byte_length: bin.len(), uri: None, name: None, extensions: None, extras: None }],
        materials: Vec::new(),
        textures: Vec::new(),
        images: Vec::new(),
        samplers: Vec::new(),
        skins: Vec::new(),
        animations: Vec::new(),
        cameras: Vec::new(),
        extensions_used: Vec::new(),
        extensions_required: Vec::new(),
        extensions: None,
        extras: None,
    };
    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), document, buffers: vec![bin], source_form: GltfSourceForm::Glb })
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_glb(&serialize(snapshot)?).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
