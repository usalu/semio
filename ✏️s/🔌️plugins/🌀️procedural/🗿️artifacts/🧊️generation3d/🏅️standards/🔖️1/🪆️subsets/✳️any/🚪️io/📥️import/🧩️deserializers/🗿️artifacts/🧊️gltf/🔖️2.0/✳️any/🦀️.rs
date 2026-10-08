//! 🧊️ Imports decoded glTF or GLB primitives as editable polygon inputs with surface channels.
use crate::standards::v1::subsets::any::io::mesh_bridge::{import_semio_mesh, io_error};
use crate::Generation3dSnapshot;
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_gltf::GltfSnapshot;
use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfDocument, GltfNode, GltfPrimitive, GltfMaterial, GltfSampler, GltfAlphaMode, GltfJson};
use semio_framework_artifact_flow_flow::{Widget, WidgetLayout, SynapseSpec, OrderedSet};
use semio_framework_artifact_flow_flow::neural::{Dictionary, Value, Atom};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::import::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf;
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

/// 🔧️ The neuron kind glTF geometry re-enters the flow graph through, after normalization.
pub const IMPORT_NEURON_KIND: &str = "brep.mesh.construct";

/// 🪄️ GLB container magic (glTF 2.0 §4.4.1) — the one byte-level cue that picks the container.
const GLB_MAGIC: &[u8; 4] = b"glTF";

pub fn register() {}

pub fn mesh_from_bytes(bytes: &[u8]) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    mesh_from_snapshot(&decode(bytes)?)
}

pub fn mesh_from_snapshot(from: &GltfSnapshot) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    admit_document(&from.document)?;
    ::semio_framework_async::poll::resolve_ready(SemioMeshFromGltf::deserialize(from)).map_err(|error| io_error(format!("generation3d←gltf: {error}")))
}

/// 🚧️ Rejects excessive declarations before the existing decoder allocates primitive domains.
fn admit_document(document: &GltfDocument) -> Result<(), semio_framework_diagnostic::TextError> {
    let (mut primitives, mut vertices, mut triangles, mut decoded) = (0usize, 0usize, 0usize, 0usize);
    let add = |total: &mut usize, amount: usize, maximum: usize| -> Result<(), semio_framework_diagnostic::TextError> { *total = total.checked_add(amount).filter(|total| *total <= maximum).ok_or_else(capacity_error)?; Ok(()) };
    let accessor = |index| document.accessors.get(index).ok_or_else(|| io_error("glTF primitive references an unknown accessor"));
    for mesh in &document.meshes { for primitive in &mesh.primitives {
        add(&mut primitives, 1, 1024)?;
        if primitive.attributes.len() > 64 { return Err(capacity_error()); }
        let position = primitive.attributes.iter().find(|(name, _)| name == "POSITION").ok_or_else(|| io_error("glTF primitive has no position accessor"))?;
        let count = accessor(position.1)?.count;
        add(&mut vertices, count, 100_000)?;
        let indices = primitive.indices.map_or(Ok(count), |index| accessor(index).map(|accessor| accessor.count))?;
        let faces = match primitive.mode.unwrap_or(4) { 0..=3 => 0, 4 => indices / 3, 5 | 6 => indices.saturating_sub(2), _ => return Err(io_error("glTF primitive mode is invalid")) };
        add(&mut triangles, faces, 100_000)?;
        for (_, index) in &primitive.attributes { let value = accessor(*index)?; add(&mut decoded, value.count.checked_mul(value.kind.components()).and_then(|count| count.checked_mul(8)).ok_or_else(capacity_error)?, 16_000_000)?; }
        add(&mut decoded, indices.checked_mul(8).ok_or_else(capacity_error)?, 16_000_000)?;
    }}
    Ok(())
}

fn capacity_error() -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "glTF import exceeds its declared capacity", semio_framework_diagnostic::TextSpan::at(1, 1))
}

fn decode(bytes: &[u8]) -> Result<GltfSnapshot, semio_framework_diagnostic::TextError> {
    if bytes.len() > 16_000_000 { return Err(capacity_error()); }
    let decoded = if bytes.starts_with(GLB_MAGIC) { semio_s_artifact_stdio_gltf::engine::decode_glb(bytes) } else { semio_s_artifact_stdio_gltf::engine::parse_gltf_document(bytes) };
    decoded.map_err(|error| io_error(format!("generation3d←gltf: {error}")))
}

pub fn deserialize(from: &GltfSnapshot) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    let mut snapshot = import_semio_mesh(&mesh_from_snapshot(from)?)?;
    if let Err(error) = enrich_surfaces(&mut snapshot, from).and_then(|_| apply_scene_nodes(&mut snapshot, &from.document)) { snapshot.retire_cold(); return Err(error); }
    Ok(snapshot)
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    deserialize(&decode(bytes)?)
}

/// 🎨️ Projects authored sampling into the existing editable material fields.
fn material_surface(document: &GltfDocument, primitive: &GltfPrimitive) -> Result<semio_framework_pack_json::Object, semio_framework_diagnostic::TextError> {
    use semio_framework_pack_json::{Object, Value as Json, array};
    let default_material = GltfMaterial::default();
    let material = match primitive.material { Some(index) => document.materials.get(index).ok_or_else(|| io_error("glTF primitive references an unknown material"))?, None => &default_material };
    let scale = material.normal_texture.as_ref().map_or(1.0, |binding| binding.scale);
    let strength = material.occlusion_texture.as_ref().map_or(1.0, |binding| binding.strength);
    if [scale, strength, material.alpha_cutoff].into_iter().any(|value| !value.is_finite() || !(value as f32).is_finite()) || !(0.0..=1.0).contains(&strength) || material.alpha_cutoff<0.0 || material.emissive_factor.iter().any(|value| !value.is_finite() || !(0.0..=1.0).contains(value)) { return Err(io_error("glTF surface coefficients are outside the editable material domain")); }
    let mut bindings = Vec::new();
    if let Some(pbr) = &material.pbr_metallic_roughness {
        if let Some(binding) = &pbr.base_color_texture { bindings.push(("baseColorTexture", binding.index, binding.tex_coord, binding.extensions.is_some())); }
        if let Some(binding) = &pbr.metallic_roughness_texture { bindings.push(("metallicRoughnessTexture", binding.index, binding.tex_coord, binding.extensions.is_some())); }
    }
    if let Some(binding) = &material.normal_texture { bindings.push(("normalTexture", binding.index, binding.tex_coord, binding.extensions.is_some())); }
    if let Some(binding) = &material.occlusion_texture { bindings.push(("occlusionTexture", binding.index, binding.tex_coord, binding.extensions.is_some())); }
    if let Some(binding) = &material.emissive_texture { bindings.push(("emissiveTexture", binding.index, binding.tex_coord, binding.extensions.is_some())); }
    let mut coordinates = Object::new();
    let mut samplers = Object::new();
    let default_sampler = GltfSampler::default();
    for (role, index, uv, extensions) in bindings {
        let texture = document.textures.get(index).ok_or_else(|| io_error("glTF material references an unknown texture"))?;
        if uv > 63 || !primitive.attributes.iter().any(|(semantic, _)| semantic == &format!("TEXCOORD_{uv}")) { return Err(io_error("glTF material references an unknown UV set")); }
        if extensions { return Err(io_error("glTF texture extensions require supported editable sampling")); }
        let sampler = match texture.sampler { Some(index) => document.samplers.get(index).ok_or_else(|| io_error("glTF texture references an unknown sampler"))?, None => &default_sampler };
        if !matches!(sampler.wrap_s, 33071 | 33648 | 10497) || !matches!(sampler.wrap_t, 33071 | 33648 | 10497) || sampler.mag_filter.is_some_and(|value| !matches!(value, 9728 | 9729)) || sampler.min_filter.is_some_and(|value| !matches!(value, 9728 | 9729 | 9984 | 9985 | 9986 | 9987)) { return Err(io_error("glTF sampler settings are invalid")); }
        let mut fields = Object::new();
        fields.insert("wrapS", sampler.wrap_s.into());
        fields.insert("wrapT", sampler.wrap_t.into());
        if let Some(value) = sampler.mag_filter { fields.insert("magFilter", value.into()); }
        if let Some(value) = sampler.min_filter { fields.insert("minFilter", value.into()); }
        coordinates.insert(role, uv.into());
        samplers.insert(role, Json::Object(fields));
    }
    let tangent = primitive.attributes.iter().any(|(semantic, _)| semantic == "TANGENT");
    let mut fields = Object::new();
    fields.insert("emissive", array(material.emissive_factor.map(Json::from)));
    fields.insert("alphaMode", match material.alpha_mode { GltfAlphaMode::Opaque => "OPAQUE", GltfAlphaMode::Mask => "MASK", GltfAlphaMode::Blend => "BLEND" }.into());
    fields.insert("alphaCutoff", material.alpha_cutoff.into());
    fields.insert("doubleSided", material.double_sided.into());
    fields.insert("normalScale", array([scale, if tangent || material.normal_texture.is_none() { scale } else { -scale }].map(Json::from)));
    fields.insert("occlusionStrength", strength.into());
    fields.insert("textureCoordinates", Json::Object(coordinates));
    fields.insert("textureSamplers", Json::Object(samplers));
    Ok(fields)
}

/// 🧩️ Restores indexed palettes through exact source domains and the canonical polygon validator.
fn restore_authored_attributes(primitive: &GltfPrimitive, polygon: &semio_framework_pack_json::Value, document: &GltfDocument, output: &mut semio_framework_pack_json::Object, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), semio_framework_diagnostic::TextError> {
    use semio_framework_pack_json::{Value as Json, array};
    use semio_framework_value::ToValue;
    let Some(GltfJson::Object(extras)) = &primitive.extras else { return Ok(()); };
    let find = |name: &str| extras.iter().find(|(key, _)| key == name).map(|(_, value)| value);
    let Some(source) = find("semioAttributes") else { return Ok(()); };
    let GltfJson::Object(source) = source else { return Err(io_error("glTF authored attributes must be an object")); };
    if source.len() > 64 || primitive.mode.unwrap_or(4) != 4 { return Err(io_error("glTF authored metadata requires bounded triangle attributes")); }
    let project = |value: &GltfJson, control: &mut semio_framework_value::NativeEncodeControl<'_>| value.to_value_controlled(control).map(|value| semio_framework_pack_json::from_dsl_value(&value)).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)));
    let index = |value: &Json| value.as_f64().filter(|number| number.is_finite() && *number >= 0.0 && *number < 600_000.0 && number.fract() == 0.0).map(|number| number as usize).ok_or_else(|| io_error("glTF authored source index is invalid"));
    let mapping = |name: &str, count: usize, control: &mut semio_framework_value::NativeEncodeControl<'_>| -> Result<Vec<usize>, semio_framework_diagnostic::TextError> {
        let value = project(find(name).ok_or_else(|| io_error("glTF authored source domain is unavailable"))?, control)?;
        let values = value.as_array().filter(|values| values.len() == count).ok_or_else(|| io_error("glTF authored source domain cardinality disagrees"))?;
        values.iter().map(index).collect()
    };
    let vertex_count = polygon.get("vertices").and_then(Json::as_array).ok_or_else(|| io_error("glTF authored source has no vertices"))?.len();
    let polygon_faces = polygon.get("faces").and_then(Json::as_array).ok_or_else(|| io_error("glTF authored source has no faces"))?;
    let (vertices, corners, faces) = (mapping("semioSourceVertices", vertex_count, control)?, mapping("semioSourceCorners", vertex_count, control)?, mapping("semioSourceFaces", polygon_faces.len(), control)?);
    let mut corner_sources = Vec::with_capacity(polygon_faces.len() * 3);
    for face in polygon_faces { let face = face.as_array().filter(|face| face.len() == 3).ok_or_else(|| io_error("glTF authored source requires triangles"))?; for vertex in face { corner_sources.push(*corners.get(index(vertex)?).ok_or_else(|| io_error("glTF authored corner vertex is unknown"))?); } }
    let Some(GltfJson::Object(streams)) = find("semioAttributeStreams") else { return Err(io_error("glTF authored stream bindings are unavailable")); };
    if streams.len() > 64 { return Err(capacity_error()); }
    let mut bound = std::collections::BTreeSet::new();
    for (name, binding) in streams {
        let GltfJson::String(semantic) = binding else { return Err(io_error("glTF authored stream binding is invalid")); };
        if !source.iter().any(|(key, _)| key == name) || !bound.insert(semantic.as_str()) || !primitive.attributes.iter().any(|(key, _)| key == semantic) { return Err(io_error("glTF authored stream binding disagrees")); }
        let alias = if semantic == "NORMAL" { "normal".into() } else if semantic == "TANGENT" { "tangent".into() } else if let Some((prefix, set)) = semantic.split_once('_').filter(|(prefix, _)| matches!(*prefix, "TEXCOORD" | "COLOR")) { let set: u32 = set.parse().map_err(|_| io_error("glTF authored stream set is invalid"))?; if set > 63 { return Err(io_error("glTF authored stream set exceeds its capacity")); } format!("{}{}", if prefix == "COLOR" { "color" } else { "uv" }, if set == 0 { String::new() } else { set.to_string() }) } else if semantic.starts_with('_') && semantic.len() > 1 { semantic.clone() } else { return Err(io_error("glTF authored stream semantic is invalid")); };
        output.remove(&alias);
    }
    for (name, value) in source {
        let mut attribute = project(value, control)?;
        let fields = attribute.as_object_mut().ok_or_else(|| io_error("glTF authored attribute must be an object"))?;
        let values = fields.get("values").and_then(Json::as_array).filter(|values| values.len() <= 600_000).ok_or_else(|| io_error("glTF authored attribute palette is invalid"))?;
        let domain = match fields.get("domain").and_then(Json::as_str) { Some("vertex") => &vertices, Some("face") => &faces, Some("corner") => { if corner_sources.iter().enumerate().any(|(index, corner)| *corner != faces[index / 3] * 3 + index % 3) { return Err(io_error("glTF authored corner source disagrees with its face")); } &corner_sources }, _ => return Err(io_error("glTF authored attribute requires editable surface topology")) };
        let palette = fields.get("indices").map(|value| value.as_array().ok_or_else(|| io_error("glTF authored palette indices must be an array"))).transpose()?;
        if let Some(palette) = palette { for value in palette { if index(value)? >= values.len() { return Err(io_error("glTF authored palette index exceeds its values")); } } }
        let mut indices = Vec::with_capacity(domain.len());
        for source_index in domain { let sample = match palette { Some(palette) => index(palette.get(*source_index).ok_or_else(|| io_error("glTF authored source exceeds its palette indices"))?)?, None => *source_index }; if sample >= values.len() { return Err(io_error("glTF authored source exceeds its palette")); } indices.push(Json::from(sample as u32)); }
        if fields.get("semantic").and_then(Json::as_str) == Some("material") {
            let part = primitive.material.and_then(|index| document.materials.get(index)).and_then(|material| material.extras.as_ref()).and_then(|extras| match extras { GltfJson::Object(fields) => fields.iter().find(|(name, _)| name == "semioMaterialPart").map(|(_, value)| value), _ => None }).ok_or_else(|| io_error("glTF authored material part is unavailable"))?;
            let mut resolved = Vec::with_capacity(values.len());
            for value in values { let id = value.as_str().ok_or_else(|| io_error("glTF authored material palette must contain identifiers"))?; let mut candidates = document.materials.iter().enumerate().filter(|(_, material)| match &material.extras { Some(GltfJson::Object(fields)) => fields.iter().any(|(name, value)| name == "semioMaterialId" && matches!(value, GltfJson::String(value) if value == id)) && fields.iter().any(|(name, value)| name == "semioMaterialPart" && value == part), _ => false }); let (index, _) = candidates.next().ok_or_else(|| io_error("glTF authored material is unavailable"))?; if candidates.next().is_some() { return Err(io_error("glTF authored material identity is ambiguous")); } resolved.push(format!("mat-{index}").into()); }
            fields.insert("values", array(resolved));
        }
        fields.insert("indices", array(indices));
        output.insert(name, attribute);
    }
    Ok(())
}

/// 🧩️ Preserves every authored vertex stream in the same imported constructor payload.
fn enrich_surfaces(snapshot: &mut Generation3dSnapshot, from: &GltfSnapshot) -> Result<(), semio_framework_diagnostic::TextError> {
    use crate::standards::v1::subsets::any::io::mesh_bridge::{IMPORT_SOURCE_WIDGET, polygon_import_attribute};
    use semio_framework_pack_json::{Object, Value as Json, array, JsonMemberPolicy};
    let primitives: Vec<_> = from.document.meshes.iter().flat_map(|mesh| &mesh.primitives).collect();
    let mut total = 0usize;
    let mut part = 0usize;
    let mut admitted = |_| true;
    let mut metadata_control = semio_framework_value::NativeEncodeControl::new(64_000_000, &mut admitted);
    for widget in &mut snapshot.host_snapshot.widgets {
        let Widget::InputNote { id, text } = widget else { continue; };
        if id != IMPORT_SOURCE_WIDGET && !id.starts_with(&format!("{IMPORT_SOURCE_WIDGET}-")) { continue; }
        let primitive = primitives.get(part).ok_or_else(|| io_error("glTF primitive source count disagrees"))?;
        let mut payload = semio_framework_pack_json::parse(text, JsonMemberPolicy::Reject).map_err(|error| io_error(error.to_string()))?;
        let vertices = payload.get("vertices").and_then(Json::as_array).ok_or_else(|| io_error("glTF constructor has no vertex domain"))?.len();
        let mut attributes = payload.get("attributes").and_then(Json::as_object).cloned().unwrap_or_else(Object::new);
        for (semantic, index) in &primitive.attributes {
            if matches!(semantic.as_str(), "POSITION" | "NORMAL" | "TEXCOORD_0" | "COLOR_0") { continue; }
            let decoded = semio_s_artifact_stdio_gltf::engine::decode_accessor(&from.document, &from.buffers, *index).map_err(io_error)?;
            let width = decoded.accessor_type.components();
            if decoded.count != vertices || width > 16 || decoded.components.iter().any(|value| !value.is_finite() || !(*value as f32).is_finite()) { return Err(io_error("glTF authored channel disagrees with its finite vertex domain")); }
            let (name, kind) = if let Some(set) = semantic.strip_prefix("TEXCOORD_") {
                let set = set.parse::<u32>().map_err(|_| io_error("glTF UV set identifier is invalid"))?;
                if width != 2 || set > 63 { return Err(io_error("glTF UV channel needs a two-component supported set")); }
                (if set == 0 { "uv".into() } else { format!("uv{set}") }, "uv")
            } else if let Some(set) = semantic.strip_prefix("COLOR_") {
                let set = set.parse::<u32>().map_err(|_| io_error("glTF color set identifier is invalid"))?;
                if !matches!(width, 3 | 4) || set > 63 || decoded.components.iter().any(|value| !(0.0..=1.0).contains(value)) { return Err(io_error("glTF color channel needs a normalized color tuple")); }
                (if set == 0 { "color".into() } else { format!("color{set}") }, "color")
            } else if semantic == "TANGENT" {
                if width != 4 || decoded.components.chunks_exact(4).any(|values| values[..3].iter().map(|value| value * value).sum::<f64>() <= 1e-24 || values[3].abs() != 1.0) { return Err(io_error("glTF tangent needs a nonzero direction and signed handedness")); }
                ("tangent".into(), "custom")
            } else { (semantic.clone(), "custom") };
            if attributes.get(&name).is_some() { return Err(io_error("glTF authored channel identifiers collide")); }
            let values = decoded.components.chunks_exact(width).map(|values| if width == 1 { Json::from(values[0]) } else { let mut values: Vec<_> = values.iter().copied().map(Json::from).collect(); if kind == "color" && width == 3 { values.push(1.0.into()); } array(values) }).collect();
            attributes.insert(name, polygon_import_attribute("vertex", kind, "linear", values, None));
        }
        if attributes.len() > 64 { return Err(io_error("glTF surface exceeds its attribute capacity")); }
        restore_authored_attributes(primitive, &payload, &from.document, &mut attributes, &mut metadata_control)?;
        let fields = material_surface(&from.document, primitive)?;
        if let Some(index) = primitive.material {
            let material = payload.get_mut("materials").and_then(Json::as_object_mut).and_then(|materials| materials.get_mut(&format!("mat-{index}"))).and_then(Json::as_object_mut).ok_or_else(|| io_error("glTF constructor is missing its authored material"))?;
            for (key, value) in fields.into_entries() { material.insert(key, value); }
        }
        payload.as_object_mut().ok_or_else(|| io_error("glTF constructor is not an object"))?.insert("attributes", Json::Object(attributes));
        let enriched = payload.to_string();
        semio_framework::mesh_io::text::parse_polygon_mesh_source(&enriched).map_err(io_error)?;
        total = total.checked_add(enriched.len()).ok_or_else(|| io_error("glTF surface payload capacity overflow"))?;
        if total > 16_000_000 { return Err(io_error("glTF surface exceeds its document capacity")); }
        *text = enriched;
        part += 1;
    }
    if part != primitives.len() { return Err(io_error("glTF primitive source count disagrees")); }
    Ok(())
}

/// 🎬️ Keeps hierarchy and mesh instances in the existing editable transform inference graph.
pub fn apply_scene_nodes(snapshot: &mut Generation3dSnapshot, document: &GltfDocument) -> Result<(), semio_framework_diagnostic::TextError> {
    if !document.animations.is_empty() || !document.skins.is_empty() || document.nodes.iter().any(|node| node.skin.is_some() || !node.weights.is_empty()) || document.meshes.iter().any(|mesh| !mesh.weights.is_empty() || mesh.primitives.iter().any(|primitive| !primitive.targets.is_empty())) { return Err(io_error("glTF animated, skinned or morphed geometry needs a static surface source")); }
    if document.nodes.len() > 1024 { return Err(io_error("glTF scene exceeds 1024 nodes")); }
    if document.nodes.is_empty() {
        if document.scene.is_some() || document.scenes.iter().any(|scene| !scene.nodes.is_empty()) { return Err(io_error("glTF scene references unavailable nodes")); }
        return Ok(());
    }
    let matrices = document.nodes.iter().map(node_matrix).collect::<Result<Vec<_>, _>>()?;
    let mut parents = vec![None; document.nodes.len()];
    for (parent, node) in document.nodes.iter().enumerate() {
        if node.mesh.is_some_and(|mesh| mesh >= document.meshes.len()) { return Err(io_error("glTF node references an unknown mesh")); }
        for &child in &node.children {
            if child >= parents.len() || parents[child].is_some() { return Err(io_error("glTF nodes must form a tree with known children")); }
            parents[child] = Some(parent);
        }
    }
    let all_roots: Vec<usize> = parents.iter().enumerate().filter_map(|(index, parent)| parent.is_none().then_some(index)).collect();
    let mut queue = all_roots.clone();
    let mut cursor = 0;
    while cursor < queue.len() { let index = queue[cursor]; queue.extend(&document.nodes[index].children); cursor += 1; }
    if queue.len() != document.nodes.len() { return Err(io_error("glTF node hierarchy contains a cycle")); }
    let roots = match document.scenes.get(document.scene.unwrap_or(0)) {
        Some(scene) => scene.nodes.clone(),
        None if document.scenes.is_empty() && document.scene.is_none() => all_roots,
        None => return Err(io_error("glTF default scene is unavailable")),
    };
    if roots.iter().collect::<std::collections::BTreeSet<_>>().len() != roots.len() || roots.iter().any(|root| *root >= parents.len() || parents[*root].is_some()) { return Err(io_error("glTF scene roots are invalid")); }
    let mut offsets = Vec::with_capacity(document.meshes.len());
    let mut offset = 0usize;
    for mesh in &document.meshes { offsets.push(offset); offset += mesh.primitives.len(); }
    let mut pending: Vec<Vec<usize>> = roots.into_iter().rev().map(|root| vec![root]).collect();
    let mut instances = Vec::new();
    let mut transform_count = 0usize;
    while let Some(path) = pending.pop() {
        if path.len() > 128 { return Err(io_error("glTF node hierarchy exceeds 128 levels")); }
        let node_index = *path.last().expect("a scene path contains its root");
        let node = &document.nodes[node_index];
        for child in node.children.iter().rev() { let mut child_path = path.clone(); child_path.push(*child); pending.push(child_path); }
        let Some(mesh) = node.mesh else { continue; };
        for part in 0..document.meshes[mesh].primitives.len() {
            transform_count += path.len();
            if instances.len() >= 1024 || transform_count > 4096 { return Err(io_error("glTF scene exceeds its surface or transform capacity")); }
            instances.push((node_index, part, offsets[mesh] + part, path.clone()));
        }
    }
    if instances.is_empty() { return Err(io_error("glTF selected scene contains no surfaces")); }
    let source_id = |part: usize| if part == 0 { "imported-geometry".to_string() } else { format!("imported-geometry-{part}") };
    let ids: std::collections::BTreeSet<_> = snapshot.host_snapshot.widgets.iter().map(crate::widget_id).collect();
    if instances.iter().any(|(_, _, part, _)| !snapshot.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == &source_id(*part) && neuron_kind == "brep.mesh.construct"))) { return Err(io_error("glTF primitive has no editable constructor")); }
    for (node, part, _, path) in &instances {
        if ids.contains(format!("imported-scene-preview-{node}-{part}").as_str()) || path.iter().any(|ancestor| ids.contains(format!("imported-transform-{node}-{part}-{ancestor}").as_str())) { return Err(io_error("glTF transform identity is occupied")); }
    }
    let host = &mut snapshot.host_snapshot;
    let removed: std::collections::BTreeSet<_> = host.widgets.iter().filter_map(|widget| match widget { Widget::OutputPreview { id, .. } => Some(id.clone()), _ => None }).collect();
    host.widgets.retain(|widget| !removed.contains(crate::widget_id(widget)));
    host.synapses.retain(|wire| !removed.contains(&wire.from) && !removed.contains(&wire.to));
    for id in removed { host.layout.remove(&id); }
    for widget in &mut host.widgets { if let Widget::Neuron { neuron_kind, preview, .. } = widget { if neuron_kind == "brep.mesh.construct" { *preview = false; } } }
    for (instance, (node, part, source_part, path)) in instances.into_iter().enumerate() {
        let mut source = source_id(source_part);
        for (step, ancestor) in path.iter().rev().enumerate() {
            let id = format!("imported-transform-{node}-{part}-{ancestor}");
            let matrix = matrices[*ancestor].into_iter().enumerate().fold(Dictionary::with_schema("list"), |matrix, (index, value)| matrix.insert(index.to_string(), Value::Dictionary(Dictionary::with_schema("number").insert("value", Value::Atom(Atom::Decimal(value))))));
            host.widgets.push(Widget::Neuron { id: id.clone(), neuron_kind: "brep.mesh.transform".into(), params: Dictionary::new().insert("matrix", Value::Dictionary(matrix)), input_ports: Vec::new(), output_ports: Vec::new(), preview: false });
            host.synapses.push(SynapseSpec { id: format!("{id}-mesh"), from: source, to: id.clone(), from_port: "meshOut".into(), to_port: "mesh".into() });
            host.layout.insert(id.clone(), WidgetLayout { x: 520.0 + step as f64 * 260.0, y: instance as f64 * 220.0 });
            source = id;
        }
        let id = format!("imported-scene-preview-{node}-{part}");
        host.widgets.push(Widget::OutputPreview { id: id.clone(), preview: Dictionary::new(), expanded: OrderedSet::new() });
        host.synapses.push(SynapseSpec { id: format!("{id}-mesh"), from: source, to: id.clone(), from_port: "meshOut".into(), to_port: String::new() });
        host.layout.insert(id, WidgetLayout { x: 520.0 + path.len() as f64 * 260.0, y: instance as f64 * 220.0 });
    }
    Ok(())
}

fn node_matrix(node: &GltfNode) -> Result<[f64; 16], semio_framework_diagnostic::TextError> {
    if node.matrix.is_some() && (node.translation.is_some() || node.rotation.is_some() || node.scale.is_some()) { return Err(io_error("glTF matrix and TRS are mutually exclusive")); }
    let [tx, ty, tz] = node.translation.unwrap_or([0.0; 3]);
    let [sx, sy, sz] = node.scale.unwrap_or([1.0; 3]);
    let [x, y, z, w] = node.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    if ((x*x+y*y+z*z+w*w).sqrt()-1.0).abs() > 1e-6 { return Err(io_error("glTF quaternion must be a unit rotation")); }
    let matrix = node.matrix.unwrap_or([(1.0-2.0*(y*y+z*z))*sx,2.0*(x*y+z*w)*sx,2.0*(x*z-y*w)*sx,0.0,2.0*(x*y-z*w)*sy,(1.0-2.0*(x*x+z*z))*sy,2.0*(y*z+x*w)*sy,0.0,2.0*(x*z+y*w)*sz,2.0*(y*z-x*w)*sz,(1.0-2.0*(x*x+y*y))*sz,0.0,tx,ty,tz,1.0]);
    if matrix.iter().any(|value| !(*value as f32).is_finite()) || [matrix[3],matrix[7],matrix[11],matrix[15]] != [0.0,0.0,0.0,1.0] { return Err(io_error("glTF placement requires a finite affine matrix")); }
    let determinant = matrix[0]*(matrix[5]*matrix[10]-matrix[9]*matrix[6])-matrix[4]*(matrix[1]*matrix[10]-matrix[9]*matrix[2])+matrix[8]*(matrix[1]*matrix[6]-matrix[5]*matrix[2]);
    if !determinant.is_finite() || determinant == 0.0 { return Err(io_error("glTF placement matrix is singular")); }
    Ok(matrix)
}
