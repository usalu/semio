//! 🧊️ Encodes retained prepared Generation3d surfaces as glTF 2.0 JSON.
//!
//! 🐛️ Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END this leaf emitted the artifact's own DSL text
//! under a `.gltf` name (see the stl sibling's doc comment for the shared defect).
//!
//! Composition: caller-prepared `SemioMeshSnapshot` → `SemioMeshToGltf` (which packs one
//! little-endian buffer plus tightly-packed accessors/bufferViews) → gltf's own
//! `serialize_gltf_document`, which is what turns the still-`uri`-less geometry buffer into the
//! `data:application/octet-stream;base64,…` uri a `.gltf` file needs. `.glb` is deliberately NOT
//! emitted here: this leaf's registered dialect is the JSON form.
//!
//! 🔖 glTF is the one mesh target that carries every `SemioTopology`, so a wire or point preview
//! exports as a real `mode: 0/1/3` primitive instead of erroring.
use crate::standards::v1::subsets::any::io::mesh_bridge::io_error;
use semio_framework_plugin::ArtifactSerializer;
use semio_s_artifact_stdio_gltf::GltfSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_plugin::{MeshAttribute, MeshAttributeDomain, MeshAttributeSemantic, MeshData};
use semio_framework_value::NativeEncodeControl;
use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfAccessor, GltfAccessorType, GltfAlphaMode, GltfBufferView, GltfComponentType, GltfJson, GltfSampler, GltfTexture};
use std::collections::BTreeMap;

pub fn serialize_mesh(mesh: &SemioMeshSnapshot) -> Result<GltfSnapshot, semio_framework_diagnostic::TextError> {
    ::semio_framework_async::poll::resolve_ready(SemioMeshToGltf::serialize(mesh)).map_err(|error| io_error(format!("generation3d→gltf: {error}")))
}

pub fn serialize_mesh_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    Ok(semio_s_artifact_stdio_gltf::engine::serialize_gltf_document(&serialize_mesh(mesh)?))
}

/// 🎨️ Encodes retained prepared surfaces through the existing geometry and glTF owners.
pub fn serialize_prepared_meshes(meshes: &[semio_framework_plugin::MeshData]) -> Result<GltfSnapshot, semio_framework_diagnostic::TextError> {
    serialize_prepared_meshes_controlled(meshes, &mut NativeEncodeControl::new(64_000_000, &mut |_| true))
}

/// 📤️ Encodes enriched prepared geometry with the owning glTF byte codec.
pub fn serialize_prepared_meshes_bytes(meshes: &[MeshData]) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    serialize_prepared_meshes_bytes_controlled(meshes, &mut NativeEncodeControl::new(64_000_000, &mut |_| true))
}

/// 🧵️ Preserves the caller encoding control through geometry, metadata and final physical bytes.
pub fn serialize_prepared_meshes_bytes_controlled(meshes: &[MeshData], control: &mut NativeEncodeControl<'_>) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    let output = serialize_prepared_meshes_controlled(meshes, control)?;
    semio_s_artifact_stdio_gltf::engine::serialize_gltf_document_owned_controlled(output, control).map_err(control_error)
}

/// 🚦️ Admits enriched output through the existing caller encoding control before publication.
pub fn serialize_prepared_meshes_controlled(meshes: &[MeshData], control: &mut NativeEncodeControl<'_>) -> Result<GltfSnapshot, semio_framework_diagnostic::TextError> {
    use crate::standards::v1::subsets::any::io::mesh_bridge;
    if meshes.is_empty() || meshes.len() > 1024 { return Err(io_error("glTF export requires 1..1024 prepared surfaces")); }
    control.checkpoint().map_err(control_error)?;
    let mut parts = control.allocate_vec(meshes.len()).map_err(control_error)?;
    for mesh in meshes {
        let count = mesh.indices.len().max(mesh.positions.len() / 3);
        if count > 300_000 || mesh.attributes.len() > 128 || mesh.materials.len() > 10_000 || mesh.textures.len() > 256 { return Err(io_error("glTF export exceeds prepared surface capacity")); }
        let bytes = count.checked_mul(64 + mesh.attributes.len() * 16).ok_or_else(|| io_error("glTF export surface capacity overflow"))?;
        control.charge(bytes).map_err(control_error)?;
        let mut sources = BTreeMap::<Option<String>, (Vec<usize>, Vec<usize>)>::new();
        for texture in mesh.textures.values() { control.charge(texture.bytes.len().checked_mul(2).ok_or_else(|| io_error("glTF texture capacity overflow"))?).map_err(control_error)?; }
        control.begin_stage(count).map_err(control_error)?;
        let part = mesh_bridge::semio_mesh_from_mesh_data_with_sources(mesh, |material, corner, created| {
            let source = sources.entry(material.map(str::to_owned)).or_default();
            if created { source.0.push(corner); }
            if !mesh.indices.is_empty() && corner % 3 == 0 { source.1.push(corner / 3); }
            control.step().map_err(control_error)
        })?;
        parts.push((part, sources.into_values().collect::<Vec<_>>()));
    }
    let mut owned = SemioMeshSnapshot::default();
    for (index, (part, _)) in parts.iter_mut().enumerate() {
        let prefix = format!("part-{index}-");
        for mesh in &mut part.meshes {
            mesh.id = format!("{prefix}{}", mesh.id);
            for primitive in &mut mesh.primitives { if let Some(id) = &mut primitive.material_id { *id = format!("{prefix}{id}"); } }
        }
        for material in &mut part.materials {
            material.id = format!("{prefix}{}", material.id);
            for id in [&mut material.base_color_texture, &mut material.metallic_roughness_texture, &mut material.normal_texture, &mut material.occlusion_texture, &mut material.emissive_texture].into_iter().flatten() { *id = format!("{prefix}{id}"); }
        }
        for texture in &mut part.textures { texture.id = format!("{prefix}{}", texture.id); }
        owned.meshes.append(&mut part.meshes); owned.materials.append(&mut part.materials); owned.textures.append(&mut part.textures);
    }
    control.checkpoint().map_err(control_error)?;
    let mut output = serialize_mesh(&owned)?;
    for (part_index, mesh) in meshes.iter().enumerate() {
        if !mesh.component_references.is_empty() || !mesh.face_ids.is_empty() || !mesh.edge_ids.is_empty() || !mesh.vertex_ids.is_empty() || !mesh.edge_positions.is_empty() {
            mesh.validate_component_references().map_err(io_error)?;
            let mut references = control.allocate_vec(mesh.component_references.len()).map_err(control_error)?;
            for (domain, values) in &mesh.component_references {
                control.begin_stage(values.len()).map_err(control_error)?;
                let mut labels = control.allocate_vec(values.len()).map_err(control_error)?;
                for value in values { labels.push(GltfJson::String(control.copy_text(value).map_err(control_error)?)); control.step().map_err(control_error)?; }
                references.push((control.copy_text(domain).map_err(control_error)?, GltfJson::Array(labels)));
            }
            let mut metadata = control.allocate_vec(5).map_err(control_error)?;
            metadata.push((control.copy_text("semioComponentReferences").map_err(control_error)?, GltfJson::Object(references)));
            for (name, values) in [("semioFaceIds", &mesh.face_ids), ("semioEdgeIds", &mesh.edge_ids), ("semioVertexIds", &mesh.vertex_ids)] { metadata.push((control.copy_text(name).map_err(control_error)?, component_numbers(values.iter().map(|value| f64::from(*value)), control)?)); }
            metadata.push((control.copy_text("semioEdgePositions").map_err(control_error)?, component_numbers(mesh.edge_positions.iter().map(|value| f64::from(*value)), control)?));
            output.document.meshes[part_index].extras = Some(GltfJson::Object(metadata));
        }
        for (primitive_index, (corners, faces)) in parts[part_index].1.iter().enumerate() {
            let mut extras = control.allocate_vec(mesh.attributes.len()).map_err(control_error)?;
            let mut streams = control.allocate_vec(mesh.attributes.len()).map_err(control_error)?;
            let mut emitted = control.allocate_vec::<String>(mesh.attributes.len()).map_err(control_error)?;
            let mut accessors = control.allocate_vec(mesh.attributes.len()).map_err(control_error)?;
            for (name, attribute) in &mesh.attributes {
                control.begin_stage(0).map_err(control_error)?;
                control.charge(name.len() + 128 + attribute.indices.as_ref().map_or(0, |indices| indices.len().saturating_mul(8))).map_err(control_error)?;
                for value in &attribute.values { admit_metadata(value, control, 0)?; }
                extras.push((control.copy_text(name).map_err(control_error)?, GltfJson::from_value(attribute.to_value_controlled(control).map_err(control_error)?).map_err(|error| io_error(error.to_string()))?));
                if attribute.semantic == MeshAttributeSemantic::Material { continue; }
                let width = attribute_width(attribute);
                if name == "tangent" && (width != Some(4) || attribute.domain == MeshAttributeDomain::Edge) { return Err(io_error("glTF tangent requires four numeric surface components")); }
                if attribute.domain == MeshAttributeDomain::Edge || width.is_none() {
                    if attribute.semantic != MeshAttributeSemantic::Custom && attribute.domain != MeshAttributeDomain::Edge { return Err(io_error("glTF export surface channel is not numeric")); }
                    continue;
                }
                let semantic = attribute_semantic(name, attribute, &emitted)?;
                if emitted.contains(&semantic) { return Err(io_error("glTF export has colliding attribute semantics")); }
                let width = width.expect("numeric channel has width");
                if (semantic == "NORMAL" && width != 3) || (semantic == "TANGENT" && width != 4) || (semantic.starts_with("TEXCOORD_") && width != 2) || (semantic.starts_with("COLOR_") && width != 4) { return Err(io_error("glTF export surface tuple width is invalid")); }
                let accessor = append_attribute(&mut output, mesh, attribute, corners, width, control)?;
                streams.push((control.copy_text(name).map_err(control_error)?, GltfJson::String(control.copy_text(&semantic).map_err(control_error)?)));
                emitted.push(control.copy_text(&semantic).map_err(control_error)?); accessors.push((semantic, accessor));
            }
            let primitive = &mut output.document.meshes[part_index].primitives[primitive_index];
            if mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Uv) { primitive.attributes.retain(|(name, _)| name != "TEXCOORD_0"); }
            if mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Color) { primitive.attributes.retain(|(name, _)| name != "COLOR_0"); }
            for (name, index) in accessors { primitive.attributes.retain(|(existing, _)| existing != &name); primitive.attributes.push((name, index)); }
            if !extras.is_empty() || !mesh.component_references.is_empty() {
                let source_corners = component_numbers(corners.iter().map(|corner| *corner as f64), control)?;
                let source_vertices = component_numbers(corners.iter().map(|corner| if mesh.indices.is_empty() { *corner as f64 } else { f64::from(mesh.indices[*corner]) }), control)?;
                let source_faces = component_numbers(faces.iter().map(|face| *face as f64), control)?;
                primitive.extras = Some(GltfJson::Object(vec![("semioAttributes".into(), GltfJson::Object(extras)), ("semioAttributeStreams".into(), GltfJson::Object(streams)), ("semioSourceCorners".into(), source_corners), ("semioSourceVertices".into(), source_vertices), ("semioSourceFaces".into(), source_faces)]));
            }
        }
        let prefix = format!("part-{part_index}-");
        for (id, material) in &mesh.materials {
            let index = output.document.materials.iter().position(|candidate| candidate.name.as_deref() == Some(&format!("{prefix}{id}"))).ok_or_else(|| io_error("glTF export lost a prepared material"))?;
            enrich_material(&mut output, index, id, part_index, material, control)?;
        }
    }
    output.document.buffers[0].byte_length = output.buffers[0].len();
    control.checkpoint().map_err(control_error)?;
    Ok(output)
}

fn admit_metadata(value: &DslValue, control: &mut NativeEncodeControl<'_>, depth: usize) -> Result<(), semio_framework_diagnostic::TextError> {
    if depth >= 64 { return Err(control_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::DepthLimit, "glTF metadata exceeds nesting capacity"))); }
    control.charge(64 + value.as_str().map_or(0, str::len)).map_err(control_error)?;
    if let Some(values) = value.as_array() { for value in values { admit_metadata(value, control, depth + 1)?; } }
    if let Some(fields) = value.as_object() { for (name, value) in fields { control.charge(name.len()).map_err(control_error)?; admit_metadata(value, control, depth + 1)?; } }
    control.step().map_err(control_error)
}

fn attribute_width(attribute: &MeshAttribute) -> Option<usize> {
    match attribute.values.first()? {
        value if value.as_f64().is_some() => Some(1),
        DslValue::Array(values) if (1..=4).contains(&values.len()) && values.iter().all(|value| value.as_f64().is_some()) => Some(values.len()),
        _ => None,
    }
}

fn control_error(error: semio_framework_value::ValueError) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1))
}

fn attribute_semantic(name: &str, attribute: &MeshAttribute, emitted: &[String]) -> Result<String, semio_framework_diagnostic::TextError> {
    let set = |prefix: &str, target: &str| -> Result<String, semio_framework_diagnostic::TextError> {
        let index = match name.strip_prefix(prefix) {
            Some("") => 0,
            Some(value) if value.chars().all(|character| character.is_ascii_digit()) => value.parse::<usize>().map_err(|_| io_error("glTF channel set is invalid"))?,
            _ => (0..64).find(|index| !emitted.contains(&format!("{target}_{index}"))).ok_or_else(|| io_error("glTF export exceeds 64 channel sets"))?,
        };
        if index > 63 { return Err(io_error("glTF channel set exceeds 0..63")); }
        Ok(format!("{target}_{index}"))
    };
    Ok(match attribute.semantic {
        _ if name == "tangent" => "TANGENT".into(),
        MeshAttributeSemantic::Uv => set("uv", "TEXCOORD")?,
        MeshAttributeSemantic::Color => set("color", "COLOR")?,
        MeshAttributeSemantic::Normal if !emitted.iter().any(|name| name == "NORMAL") => "NORMAL".into(),
        _ => if name.starts_with('_') { name.into() } else { format!("_{name}") },
    })
}

fn append_attribute(output: &mut GltfSnapshot, mesh: &MeshData, attribute: &MeshAttribute, corners: &[usize], width: usize, control: &mut NativeEncodeControl<'_>) -> Result<usize, semio_framework_diagnostic::TextError> {
    let bytes = corners.len().checked_mul(width * 4).ok_or_else(|| io_error("glTF attribute capacity overflow"))?;
    control.charge(bytes).map_err(control_error)?; control.begin_stage(corners.len()).map_err(control_error)?;
    output.buffers[0].try_reserve_exact(bytes).map_err(|_| io_error("glTF attribute allocation failed"))?;
    let offset = output.buffers[0].len();
    for &corner in corners {
        let vertex = if mesh.indices.is_empty() { corner } else { mesh.indices[corner] as usize };
        let domain = match attribute.domain { MeshAttributeDomain::Vertex => vertex, MeshAttributeDomain::Corner if !mesh.indices.is_empty() => corner, MeshAttributeDomain::Face if !mesh.indices.is_empty() => corner / 3, _ => return Err(io_error("glTF export surface channel domain is unavailable")) };
        let value = attribute.value_at(domain).ok_or_else(|| io_error("glTF export attribute sample is missing"))?;
        let array = value.as_array();
        if array.map_or(1, |values| values.len()) != width { return Err(io_error("glTF attribute samples have inconsistent widths")); }
        for component in 0..width {
            let value = array.map_or(value, |values| &values[component]);
            let value = value.as_f64().filter(|value| value.is_finite() && (*value as f32).is_finite()).ok_or_else(|| io_error("glTF attribute sample is not a finite number"))?;
            if attribute.semantic == MeshAttributeSemantic::Color && !(0.0..=1.0).contains(&value) { return Err(io_error("glTF color sample is outside [0,1]")); }
            output.buffers[0].extend_from_slice(&(value as f32).to_le_bytes());
        }
        control.step().map_err(control_error)?;
    }
    let view = output.document.buffer_views.len();
    output.document.buffer_views.push(GltfBufferView { buffer: 0, byte_offset: offset, byte_length: bytes, byte_stride: None, target: Some(34962), name: None, extensions: None, extras: None });
    let index = output.document.accessors.len();
    output.document.accessors.push(GltfAccessor { buffer_view: Some(view), byte_offset: 0, component_type: GltfComponentType::Float, normalized: false, count: corners.len(), kind: match width { 1 => GltfAccessorType::Scalar, 2 => GltfAccessorType::Vec2, 3 => GltfAccessorType::Vec3, _ => GltfAccessorType::Vec4 }, max: None, min: None, sparse: None, name: None, extensions: None, extras: None });
    Ok(index)
}

fn enrich_material(output: &mut GltfSnapshot, index: usize, id: &str, part: usize, source: &DslValue, control: &mut NativeEncodeControl<'_>) -> Result<(), semio_framework_diagnostic::TextError> {
    control.begin_stage(5).map_err(control_error)?;
    for field in ["textureCoordinates", "textureSamplers"] { if source.get(field).is_some_and(|value| value.as_object().is_none()) { return Err(io_error(format!("glTF material {field} must be an owned object"))); } }
    let coefficient = |name: &str, default: f64| -> Result<f64, semio_framework_diagnostic::TextError> {
        source.get(name).map_or(Ok(default), |value| value.as_f64().filter(|value| value.is_finite() && (*value as f32).is_finite()).ok_or_else(|| io_error(format!("glTF material {name} is invalid"))))
    };
    control.begin_stage(0).map_err(control_error)?;
    admit_metadata(source, control, 0)?;
    let extras = GltfJson::from_value(source.to_value_controlled(control).map_err(control_error)?).map_err(|error| io_error(error.to_string()))?;
    let material = &mut output.document.materials[index];
    let mut metadata = control.allocate_vec(3).map_err(control_error)?;
    metadata.push((control.copy_text("semioMaterial").map_err(control_error)?, extras));
    metadata.push((control.copy_text("semioMaterialId").map_err(control_error)?, GltfJson::String(control.copy_text(id).map_err(control_error)?)));
    metadata.push((control.copy_text("semioMaterialPart").map_err(control_error)?, GltfJson::Number(part as f64)));
    material.extras = Some(GltfJson::Object(metadata));
    control.begin_stage(5).map_err(control_error)?;
    material.alpha_mode = match source.get("alphaMode").map_or(Ok("OPAQUE"), |value| value.as_str().ok_or_else(|| io_error("glTF alpha mode is not a string")))? { "OPAQUE" => GltfAlphaMode::Opaque, "MASK" => GltfAlphaMode::Mask, "BLEND" => GltfAlphaMode::Blend, _ => return Err(io_error("glTF alpha mode is invalid")) };
    material.alpha_cutoff = coefficient("alphaCutoff", 0.5)?;
    if material.alpha_cutoff < 0.0 { return Err(io_error("glTF alpha cutoff must be nonnegative")); }
    material.double_sided = match source.get("doubleSided") { None => false, Some(DslValue::Bool(value)) => *value, _ => return Err(io_error("glTF double-sided material flag is invalid")) };
    if let Some(value) = source.get("emissive") {
        let values = value.as_array().filter(|values| values.len() == 3).ok_or_else(|| io_error("glTF emissive factor requires RGB"))?;
        for (axis, value) in values.iter().enumerate() { material.emissive_factor[axis] = value.as_f64().filter(|value| value.is_finite() && (0.0..=1.0).contains(value)).ok_or_else(|| io_error("glTF emissive factor is outside [0,1]"))?; }
    }
    if let Some(normal) = &mut material.normal_texture {
        normal.scale = match source.get("normalScale") {
            None => 1.0,
            Some(value) if value.as_f64().is_some() => coefficient("normalScale", 1.0)?,
            Some(value) => {
                let values = value.as_array().filter(|values| values.len() == 2).ok_or_else(|| io_error("glTF normal scale requires a scalar or pair"))?;
                let x = values[0].as_f64().filter(|value| value.is_finite()).ok_or_else(|| io_error("glTF normal scale is invalid"))?;
                let y = values[1].as_f64().filter(|value| value.is_finite()).ok_or_else(|| io_error("glTF normal scale is invalid"))?;
                let authored = output.document.meshes.iter().flat_map(|mesh| &mesh.primitives).filter(|primitive| primitive.material == Some(index)).all(|primitive| primitive.attributes.iter().any(|(name, _)| name == "TANGENT"));
                if !((authored && x == y) || (!authored && x == -y)) { return Err(io_error("glTF cannot represent the authored anisotropic normal scale")); }
                x
            }
        };
        if !(normal.scale as f32).is_finite() { return Err(io_error("glTF normal scale is outside finite f32")); }
    }
    if let Some(occlusion) = &mut material.occlusion_texture { occlusion.strength = coefficient("occlusionStrength", 1.0)?; if !(0.0..=1.0).contains(&occlusion.strength) { return Err(io_error("glTF occlusion strength is outside [0,1]")); } }
    for role in ["baseColorTexture", "metallicRoughnessTexture", "normalTexture", "occlusionTexture", "emissiveTexture"] {
        let binding = match role {
            "baseColorTexture" => material.pbr_metallic_roughness.as_ref().and_then(|pbr| pbr.base_color_texture.as_ref()).map(|binding| binding.index),
            "metallicRoughnessTexture" => material.pbr_metallic_roughness.as_ref().and_then(|pbr| pbr.metallic_roughness_texture.as_ref()).map(|binding| binding.index),
            "normalTexture" => material.normal_texture.as_ref().map(|binding| binding.index),
            "occlusionTexture" => material.occlusion_texture.as_ref().map(|binding| binding.index),
            _ => material.emissive_texture.as_ref().map(|binding| binding.index),
        };
        let Some(binding) = binding else { continue; };
        let coordinates = source.get("textureCoordinates").and_then(|coordinates| coordinates.get(role));
        let set = coordinates.map_or(Ok(0), |value| value.as_f64().filter(|value| (0.0..=63.0).contains(value) && value.fract() == 0.0).map(|value| value as u64).ok_or_else(|| io_error("glTF texture UV selection is outside 0..63")))?;
        if output.document.meshes.iter().flat_map(|mesh| &mesh.primitives).filter(|primitive| primitive.material == Some(index)).any(|primitive| !primitive.attributes.iter().any(|(name, _)| name == &format!("TEXCOORD_{set}"))) { return Err(io_error("glTF texture selects a missing UV channel")); }
        let fields = source.get("textureSamplers").and_then(|samplers| samplers.get(role));
        if fields.is_some_and(|value| value.as_object().is_none()) { return Err(io_error("glTF texture sampler must be an owned object")); }
        let integer = |name: &str, domain: &[u64], default: Option<u64>| -> Result<Option<u64>, semio_framework_diagnostic::TextError> {
            fields.and_then(|fields| fields.get(name)).map_or(Ok(default), |value| value.as_f64().filter(|value| value.fract() == 0.0 && domain.contains(&(*value as u64))).map(|value| Some(value as u64)).ok_or_else(|| io_error("glTF texture sampler is invalid")))
        };
        let sampler = GltfSampler { wrap_s: integer("wrapS", &[33071,33648,10497], Some(10497))?.unwrap(), wrap_t: integer("wrapT", &[33071,33648,10497], Some(10497))?.unwrap(), mag_filter: integer("magFilter", &[9728,9729], None)?, min_filter: integer("minFilter", &[9728,9729,9984,9985,9986,9987], None)?, ..GltfSampler::default() };
        let sampler_index = output.document.samplers.iter().position(|existing| existing == &sampler).unwrap_or_else(|| { output.document.samplers.push(sampler); output.document.samplers.len() - 1 });
        let image = output.document.textures[binding].source;
        let texture_index = output.document.textures.iter().position(|texture| texture.source == image && texture.sampler == Some(sampler_index)).unwrap_or_else(|| { output.document.textures.push(GltfTexture { source: image, sampler: Some(sampler_index), ..GltfTexture::default() }); output.document.textures.len() - 1 });
        match role {
            "baseColorTexture" => { let binding = material.pbr_metallic_roughness.as_mut().unwrap().base_color_texture.as_mut().unwrap(); binding.index = texture_index; binding.tex_coord = set; },
            "metallicRoughnessTexture" => { let binding = material.pbr_metallic_roughness.as_mut().unwrap().metallic_roughness_texture.as_mut().unwrap(); binding.index = texture_index; binding.tex_coord = set; },
            "normalTexture" => { let binding = material.normal_texture.as_mut().unwrap(); binding.index = texture_index; binding.tex_coord = set; },
            "occlusionTexture" => { let binding = material.occlusion_texture.as_mut().unwrap(); binding.index = texture_index; binding.tex_coord = set; },
            _ => { let binding = material.emissive_texture.as_mut().unwrap(); binding.index = texture_index; binding.tex_coord = set; },
        }
        control.step().map_err(control_error)?;
    }
    Ok(())
}

/// 🎯️ Projects original picking buffers and segment coordinates under the same output budget.
fn component_numbers(values: impl ExactSizeIterator<Item=f64>, control: &mut NativeEncodeControl<'_>) -> Result<GltfJson, semio_framework_diagnostic::TextError> {
    control.begin_stage(values.len()).map_err(control_error)?;
    let mut output = control.allocate_vec(values.len()).map_err(control_error)?;
    for value in values { if !value.is_finite() { return Err(io_error("glTF component coordinates must be finite")); } output.push(GltfJson::Number(value)); control.step().map_err(control_error)?; }
    Ok(GltfJson::Array(output))
}
