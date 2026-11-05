//! 🔺️ lowpoly <- gltf
//!
//! Geometry over the real `io::decode_glb` grammar: selected scene mesh instances preserve hierarchy transforms.
//! Triangle primitives become managed Lowpoly geometry; other primitive modes are refused explicitly.
use crate::{mesh_geometry::snapshot_from_parts, mesh_geometry::text_error, mesh_geometry::PolygonPart};
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::decode_accessor;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::decode_glb;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessorType, GltfComponentType, GltfDocument, GltfNode};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;

pub fn register() {}

type ImportResult<T> = Result<T, semio_framework_diagnostic::TextError>;
type Matrix = [f64; 16];
const IDENTITY: Matrix = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];

fn node_matrix(node: &GltfNode) -> ImportResult<Matrix> {
    if let Some(matrix) = node.matrix {
        if node.translation.is_some() || node.rotation.is_some() || node.scale.is_some() {
            return Err(text_error("gltf->lowpoly: node matrix and TRS are mutually exclusive"));
        }
        if matrix.iter().any(|value| !value.is_finite()) || [matrix[3], matrix[7], matrix[11], matrix[15]] != [0.0, 0.0, 0.0, 1.0] {
            return Err(text_error("gltf->lowpoly: node matrix must be finite and affine"));
        }
        return Ok(matrix);
    }
    let [x, y, z, w] = node.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let scale = node.scale.unwrap_or([1.0; 3]);
    let translation = node.translation.unwrap_or([0.0; 3]);
    if [x, y, z, w].iter().chain(scale.iter()).chain(translation.iter()).any(|value| !value.is_finite()) || (x*x + y*y + z*z + w*w - 1.0).abs() > 1e-5 {
        return Err(text_error("gltf->lowpoly: node TRS must be finite with a unit quaternion"));
    }
    Ok([
        (1.0-2.0*(y*y+z*z))*scale[0], 2.0*(x*y+z*w)*scale[0], 2.0*(x*z-y*w)*scale[0], 0.0,
        2.0*(x*y-z*w)*scale[1], (1.0-2.0*(x*x+z*z))*scale[1], 2.0*(y*z+x*w)*scale[1], 0.0,
        2.0*(x*z+y*w)*scale[2], 2.0*(y*z-x*w)*scale[2], (1.0-2.0*(x*x+y*y))*scale[2], 0.0,
        translation[0], translation[1], translation[2], 1.0,
    ])
}

fn decode_mesh(from: &GltfSnapshot, mesh_index: usize) -> ImportResult<PolygonPart> {
    let mesh = from.document.meshes.get(mesh_index).ok_or_else(|| text_error("gltf->lowpoly: mesh index out of range"))?;
    let mut part = PolygonPart { name: mesh.name.clone().unwrap_or_else(|| format!("Mesh {mesh_index}")), ..Default::default() };
    for primitive in &mesh.primitives {
        let mode = primitive.mode.unwrap_or(4);
        if mode != 4 {
            return Err(text_error(format!("gltf->lowpoly: unsupported primitive mode {mode}; TRIANGLES (4) required")));
        }
        let position = primitive.attributes.iter().find_map(|(key, index)| (key == "POSITION").then_some(*index)).ok_or_else(|| text_error("gltf->lowpoly: primitive missing POSITION accessor"))?;
        let decoded = decode_accessor(&from.document, &from.buffers, position).map_err(|error| text_error(format!("gltf->lowpoly: {error}")))?;
        if decoded.accessor_type != GltfAccessorType::Vec3 || decoded.component_type != GltfComponentType::Float || decoded.normalized {
            return Err(text_error("gltf->lowpoly: POSITION accessor must be unnormalized FLOAT VEC3"));
        }
        let base = u32::try_from(part.positions.len()).map_err(|_| text_error("gltf->lowpoly: too many vertices"))?;
        let count = u32::try_from(decoded.count).map_err(|_| text_error("gltf->lowpoly: too many vertices"))?;
        base.checked_add(count).ok_or_else(|| text_error("gltf->lowpoly: too many vertices"))?;
        for point in decoded.components.chunks_exact(3) {
            let point = [point[0] as f32, point[1] as f32, point[2] as f32];
            if point.iter().any(|value| !value.is_finite()) {
                return Err(text_error("gltf->lowpoly: POSITION must be finite and representable"));
            }
            part.positions.push(point);
        }
        let indices = if let Some(index) = primitive.indices {
            let decoded = decode_accessor(&from.document, &from.buffers, index).map_err(|error| text_error(format!("gltf->lowpoly: {error}")))?;
            if decoded.accessor_type != GltfAccessorType::Scalar || decoded.normalized || !matches!(decoded.component_type, GltfComponentType::UnsignedByte | GltfComponentType::UnsignedShort | GltfComponentType::UnsignedInt) {
                return Err(text_error("gltf->lowpoly: indices must be unnormalized unsigned SCALAR"));
            }
            decoded.components.into_iter().map(|value| value as u32).collect::<Vec<_>>()
        } else {
            (0..count).collect()
        };
        if indices.len() % 3 != 0 || indices.iter().any(|index| *index >= count) {
            return Err(text_error("gltf->lowpoly: triangle indices must be complete and in range"));
        }
        part.faces.extend(indices.chunks_exact(3).map(|triangle| triangle.iter().map(|index| base + index).collect()));
    }
    Ok(part)
}


fn validate_node_forest(document: &GltfDocument) -> ImportResult<Vec<usize>> {
    let mut parents = vec![false; document.nodes.len()];
    for node in &document.nodes {
        if node.mesh.is_some_and(|index| index >= document.meshes.len()) {
            return Err(text_error("gltf->lowpoly: mesh index out of range"));
        }
        for child in &node.children {
            let parent = parents.get_mut(*child).ok_or_else(|| text_error("gltf->lowpoly: node index out of range"))?;
            if *parent {
                return Err(text_error("gltf->lowpoly: node has multiple parents or duplicate child edges"));
            }
            *parent = true;
        }
    }
    for scene in &document.scenes {
        let mut roots = std::collections::HashSet::new();
        for root in &scene.nodes {
            let parent = parents.get(*root).ok_or_else(|| text_error("gltf->lowpoly: scene root index out of range"))?;
            if *parent || !roots.insert(*root) {
                return Err(text_error("gltf->lowpoly: scene root must be unique and have no parent"));
            }
        }
    }
    let roots: Vec<_> = parents.iter().enumerate().filter_map(|(index, parent)| (!*parent).then_some(index)).collect();
    let mut pending = roots.clone();
    let mut visited = 0;
    while let Some(index) = pending.pop() {
        visited += 1;
        pending.extend(document.nodes[index].children.iter().copied());
    }
    if visited != document.nodes.len() {
        return Err(text_error("gltf->lowpoly: node cycle in document forest"));
    }
    Ok(roots)
}

/// 🌳️ Import the selected scene in world space, or root instances and unreferenced mesh definitions from a scene-less library.
pub fn deserialize(from: &GltfSnapshot) -> ImportResult<LowpolySnapshot> {
    let document = &from.document;
    let forest_roots = validate_node_forest(document)?;
    let roots = if let Some(index) = document.scene.or_else(|| (!document.scenes.is_empty()).then_some(0)) {
        document.scenes.get(index).ok_or_else(|| text_error("gltf->lowpoly: scene index out of range"))?.nodes.clone()
    } else {
        forest_roots
    };
    let mut stack: Vec<_> = roots.into_iter().rev().map(|index| (index, IDENTITY)).collect();
    let mut meshes = std::collections::HashMap::new();
    let mut parts = Vec::new();
    while let Some((index, parent)) = stack.pop() {
        let node = document.nodes.get(index).ok_or_else(|| text_error("gltf->lowpoly: node index out of range"))?;
        let local = node_matrix(node)?;
        let world: Matrix = std::array::from_fn(|index| (0..4).map(|k| parent[k * 4 + index % 4] * local[(index / 4) * 4 + k]).sum());
        stack.extend(node.children.iter().rev().map(|child| (*child, world)));
        let Some(mesh_index) = node.mesh else { continue };
        if !meshes.contains_key(&mesh_index) {
            meshes.insert(mesh_index, decode_mesh(from, mesh_index)?);
        }
        let source = &meshes[&mesh_index];
        let positions = source.positions.iter().map(|point| {
            let point: [f32; 3] = std::array::from_fn(|row| (world[row]*f64::from(point[0]) + world[4+row]*f64::from(point[1]) + world[8+row]*f64::from(point[2]) + world[12+row]) as f32);
            if point.iter().any(|value| !value.is_finite()) { Err(text_error("gltf->lowpoly: transformed POSITION must be finite and representable")) } else { Ok(point) }
        }).collect::<ImportResult<Vec<_>>>()?;
        let determinant = world[0]*(world[5]*world[10]-world[9]*world[6]) - world[4]*(world[1]*world[10]-world[9]*world[2]) + world[8]*(world[1]*world[6]-world[5]*world[2]);
        let mut faces = source.faces.clone();
        if determinant < 0.0 {
            for face in &mut faces { face.reverse(); }
        }
        parts.push(PolygonPart { name: node.name.clone().unwrap_or_else(|| source.name.clone()), positions, faces });
    }
    if document.scenes.is_empty() {
        let referenced: std::collections::HashSet<_> = document.nodes.iter().filter_map(|node| node.mesh).collect();
        for mesh_index in 0..document.meshes.len() {
            if !referenced.contains(&mesh_index) {
                parts.push(decode_mesh(from, mesh_index)?);
            }
        }
    }
    snapshot_from_parts("gltf", parts)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    let snap = decode_glb(bytes).map_err(|e| text_error(format!("gltf->lowpoly: {e}")))?;
    deserialize(&snap)
}
