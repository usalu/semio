//! 🔺️ Converts caller-prepared geometry and editable graph documents through the owning codecs.
//! Geometry evaluation and retention belong to the calling composition.
#![allow(clippy::result_large_err)]

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::neural::Dictionary;
use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout};
use semio_framework_plugin::MeshData;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioUv;
use semio_s_artifact_stdio_semio_mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};

/// 🏷️ Widget ids the import fixture plants, stable so a re-import overwrites rather than stacks.
pub const IMPORT_SOURCE_WIDGET: &str = "imported-source";
pub const IMPORT_GEOMETRY_WIDGET: &str = "imported-geometry";
pub const IMPORT_PREVIEW_WIDGET: &str = "imported-preview";
/// 🏷️ `SemioMesh` id every export carries — the merged preview, not a per-widget mesh.
pub const EXPORT_MESH_ID: &str = "generation3d-preview";

/// 🔌️ Resolves the existing graph connections that own an export's geometry.
pub fn export_source_channels(snapshot: &FlowHostSnapshot, widget_id: Option<&str>) -> Result<Vec<(String, Option<String>)>, semio_framework_diagnostic::TextError> {
    let Some(widget_id) = widget_id else {
        return Ok(snapshot.widgets.iter().filter(|widget| semio_s_artifact_procedural_3d_model::widget_previews(widget)).map(|widget| (semio_s_artifact_procedural_3d_model::widget_id(widget).to_string(), None)).collect());
    };
    if !snapshot.widgets.iter().any(|widget| matches!(widget, Widget::OutputExport { id, .. } if id == widget_id)) { return Err(io_error("choose an export output")); }
    let mut channels = Vec::new();
    for wire in snapshot.synapses.iter().filter(|wire| wire.to == widget_id) {
        if !snapshot.widgets.iter().any(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget) == wire.from) { return Err(io_error("export output has no connected source")); }
        let channel = (wire.from.clone(), Some(wire.from_port.clone()));
        if !channels.contains(&channel) { channels.push(channel); }
    }
    if channels.is_empty() { return Err(io_error("export output has no connected source")); }
    Ok(channels)
}

/// 🪪️ Uses the existing preview chain for visible geometry and connected export outputs.
pub fn retained_geometry_channels(snapshot: &FlowHostSnapshot) -> Vec<(String, Option<String>)> {
    let mut channels = export_source_channels(snapshot, None).unwrap_or_default();
    for widget in &snapshot.widgets {
        if let Widget::OutputExport { id, .. } = widget {
            if let Ok(sources) = export_source_channels(snapshot, Some(id)) {
                for source in sources { if !channels.contains(&source) { channels.push(source); } }
            }
        }
    }
    channels
}

/// 🚨️ Every failure in this artifact's IO is a typed error carrying WHY — an empty document is
/// never a legal answer to bytes that did not decode.
pub fn io_error(message: impl Into<String>) -> semio_framework_diagnostic::TextError {
    semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message.into(), semio_framework_diagnostic::TextSpan::at(1, 1))
}

/// 🔤️ Standard base64 (RFC 4648 §4, `=`-padded) — the exact spelling the brep extension's own
/// `decode_base64` accepts on a `brep.io.import*` node's `data` channel. Reused from the gltf
/// artifact's data-uri codec rather than re-derived here.
pub fn base64_encode(bytes: &[u8]) -> String {
    semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::b64_encode(bytes)
}

/// 🔤️ Inverse of [`base64_encode`], for reading an import fixture's planted payload back.
pub fn base64_decode(text: &str) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::b64_decode(text).map_err(io_error)
}

/// 🔺️ Neutral materialized geometry as this repo's own typed mesh document.
///
/// `MeshData` is the tessellation wire form: flat `f32` triples plus a `u32` index buffer. A
/// mesh WITHOUT indices carries no face connectivity at all (a wire/point preview), so it
/// becomes a `Points` primitive rather than being silently reinterpreted as a triangle soup —
/// the format bridges that cannot represent points then refuse it by name instead of writing
/// invented triangles.
pub fn semio_mesh_from_mesh_data(mesh: &MeshData) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    semio_mesh_from_mesh_data_with_sources(mesh, |_, _, _| Ok(()))
}

/// 🎨️ Visits each emitted vertex's source corner inside the same geometry grouping authority.
pub fn semio_mesh_from_mesh_data_with_sources(mesh: &MeshData, mut record: impl FnMut(Option<&str>, usize, bool) -> Result<(), semio_framework_diagnostic::TextError>) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    use semio_framework_plugin::{MeshAttributeDomain, MeshAttributeSemantic};
    if mesh.positions.is_empty() || mesh.positions.len() % 3 != 0 || mesh.positions.len() > 300_000 || mesh.positions.iter().any(|value| !value.is_finite()) {
        return Err(io_error("generation3d export requires 1..100000 finite three-dimensional positions"));
    }
    let topology = if mesh.indices.is_empty() { SemioTopology::Points } else { SemioTopology::Triangles };
    if mesh.indices.len() % 3 != 0 || mesh.indices.len() > 300_000 || mesh.indices.iter().any(|index| *index as usize >= mesh.positions.len() / 3) {
        return Err(io_error("generation3d export has invalid triangle indices"));
    }
    let material_channel = mesh.attributes.values().find(|attribute| attribute.semantic == MeshAttributeSemantic::Material);
    let corner_channels = mesh.attributes.values().any(|attribute| matches!(attribute.domain, MeshAttributeDomain::Corner | MeshAttributeDomain::Face) && attribute.semantic != MeshAttributeSemantic::Material);
    let mut groups = std::collections::BTreeMap::<Option<String>, (SemioPrimitive, std::collections::BTreeMap<usize, u32>)>::new();
    let count = if topology == SemioTopology::Points { mesh.positions.len() / 3 } else { mesh.indices.len() };
    for corner in 0..count {
        let vertex = if topology == SemioTopology::Points { corner } else { mesh.indices[corner] as usize };
        let material = match material_channel {
            Some(attribute) => {
                if topology == SemioTopology::Points || attribute.domain != MeshAttributeDomain::Face { return Err(io_error("generation3d export requires face material assignments")); }
                let id = attribute_value(attribute, corner / 3)?.as_str().ok_or_else(|| io_error("generation3d export material assignment is not an identifier"))?;
                if !mesh.materials.contains_key(id) { return Err(io_error(format!("generation3d export references unknown material '{id}'"))); }
                Some(id.to_string())
            }
            None => None,
        };
        let (primitive, remap) = groups.entry(material.clone()).or_insert_with(|| (SemioPrimitive { topology, material_id: material, ..SemioPrimitive::default() }, std::collections::BTreeMap::new()));
        let key = if corner_channels { corner } else { vertex };
        let created = !remap.contains_key(&key);
        let index = if let Some(index) = remap.get(&key) { *index } else {
            let index = primitive.positions.len() as u32;
            let point = &mesh.positions[vertex * 3..vertex * 3 + 3];
            primitive.positions.push(SemioPoint3 { x: point[0] as f64, y: point[1] as f64, z: point[2] as f64 });
            if let Some(value) = channel_tuple(mesh, MeshAttributeSemantic::Normal, vertex, corner, 3)? { primitive.normals.push(SemioPoint3 { x: value[0], y: value[1], z: value[2] }); }
            if let Some(value) = channel_tuple(mesh, MeshAttributeSemantic::Uv, vertex, corner, 2)? { primitive.uvs.push(SemioUv { u: value[0], v: value[1] }); }
            if let Some(value) = channel_tuple(mesh, MeshAttributeSemantic::Color, vertex, corner, 4)? { primitive.colors.push(SemioRgba { r: value[0] as f32, g: value[1] as f32, b: value[2] as f32, a: value[3] as f32 }); }
            remap.insert(key, index);
            index
        };
        record(primitive.material_id.as_deref(), corner, created)?;
        if topology == SemioTopology::Triangles { primitive.indices.push(index); }
    }
    let primitives = groups.into_values().enumerate().map(|(index, (mut primitive, _))| { primitive.id = format!("{EXPORT_MESH_ID}-prim-{index}"); primitive }).collect();
    let mut materials = Vec::new();
    for (id, data) in &mesh.materials {
        if data.as_object().is_none() { return Err(io_error("generation3d export material must be an owned object")); }
        let defaults = semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfPbrMetallicRoughness::default();
        let color = data.get("baseColor");
        if color.is_some_and(|value| value.as_array().is_none_or(|values| values.len() != 4)) { return Err(io_error("generation3d export material requires an RGBA base color")); }
        let channel = |index: usize| match color { None => Ok(defaults.base_color_factor[index] as f32), Some(color) => color.as_array().expect("validated material color")[index].as_f64().filter(|value| value.is_finite() && (0.0..=1.0).contains(value)).map(|value| value as f32).ok_or_else(|| io_error("generation3d export material color is invalid")) };
        let scalar = |name: &str, default: f64| match data.get(name) { None => Ok(default as f32), Some(value) => value.as_f64().filter(|value| value.is_finite() && (0.0..=1.0).contains(value)).map(|value| value as f32).ok_or_else(|| io_error(format!("generation3d export material '{name}' is invalid"))) };
        let texture = |name: &str| -> Result<Option<String>, semio_framework_diagnostic::TextError> {
            match data.get(name) {
                None | Some(semio_framework_value::DslValue::Null) => Ok(None),
                Some(value) => {
                    let id = value.as_str().filter(|id| mesh.textures.contains_key(*id)).ok_or_else(|| io_error(format!("generation3d export material '{name}' references an unknown texture")))?;
                    Ok(Some(id.to_string()))
                }
            }
        };
        materials.push(SemioMaterial { id: id.clone(), base_color: SemioRgba { r: channel(0)?, g: channel(1)?, b: channel(2)?, a: channel(3)? }, metallic: scalar("metallic",defaults.metallic_factor)?, roughness: scalar("roughness",defaults.roughness_factor)?,
            base_color_texture: texture("baseColorTexture")?, metallic_roughness_texture: texture("metallicRoughnessTexture")?, normal_texture: texture("normalTexture")?, occlusion_texture: texture("occlusionTexture")?, emissive_texture: texture("emissiveTexture")? });
    }
    let textures = mesh.textures.iter().map(|(id, texture)| SemioTexture { id: id.clone(), mime: texture.mime.clone(), bytes: texture.bytes.clone() }).collect();
    Ok(SemioMeshSnapshot { schema: STDIO_SEMIOMESH_DOCUMENT_SCHEMA.into(), meshes: vec![SemioMesh { id: EXPORT_MESH_ID.into(), primitives }], materials, textures })
}

fn attribute_value(attribute: &semio_framework_plugin::MeshAttribute, domain_index: usize) -> Result<&semio_framework_value::DslValue, semio_framework_diagnostic::TextError> {
    attribute.value_at(domain_index).ok_or_else(|| io_error("generation3d export attribute value is out of range"))
}

fn channel_tuple(mesh: &MeshData, semantic: semio_framework_plugin::MeshAttributeSemantic, vertex: usize, corner: usize, width: usize) -> Result<Option<Vec<f64>>, semio_framework_diagnostic::TextError> {
    use semio_framework_plugin::{MeshAttributeDomain, MeshAttributeSemantic};
    let values = if let Some(attribute) = mesh.attributes.values().find(|attribute| attribute.semantic == semantic && attribute.domain != MeshAttributeDomain::Edge) {
        let index = match attribute.domain { MeshAttributeDomain::Vertex => vertex, MeshAttributeDomain::Corner if !mesh.indices.is_empty() => corner, MeshAttributeDomain::Face if !mesh.indices.is_empty() => corner / 3, _ => return Err(io_error("generation3d export surface channel has an unsupported domain")) };
        let values = attribute_value(attribute, index)?.as_array().ok_or_else(|| io_error("generation3d export surface channel is not a numeric tuple"))?;
        if values.len() != width { return Err(io_error("generation3d export surface tuple width is invalid")); }
        values.iter().map(|value| value.as_f64().ok_or_else(|| io_error("generation3d export surface channel contains a non-number"))).collect::<Result<Vec<_>, _>>()?
    } else {
        let buffer = match semantic { MeshAttributeSemantic::Normal => &mesh.normals, MeshAttributeSemantic::Uv => &mesh.uvs, MeshAttributeSemantic::Color => &mesh.colors, _ => return Ok(None) };
        if buffer.is_empty() { return Ok(None); }
        let stride = if semantic == MeshAttributeSemantic::Color && buffer.len() == mesh.positions.len() { 3 } else { width };
        if buffer.len() != mesh.positions.len() / 3 * stride { return Err(io_error("generation3d export surface buffer does not match its vertices")); }
        let mut values = buffer[vertex * stride..vertex * stride + stride].iter().map(|value| *value as f64).collect::<Vec<_>>();
        if semantic == MeshAttributeSemantic::Color && stride == 3 { values.push(1.0); }
        values
    };
    if values.iter().any(|value| !value.is_finite() || !(*value as f32).is_finite()) { return Err(io_error("generation3d export surface channel contains a non-finite number")); }
    if semantic == MeshAttributeSemantic::Normal && values.iter().map(|value| value * value).sum::<f64>() <= 1e-24 { return Err(io_error("generation3d export surface normal is zero")); }
    if semantic == MeshAttributeSemantic::Color && values.iter().any(|value| !(0.0..=1.0).contains(value)) { return Err(io_error("generation3d export surface color is outside [0,1]")); }
    Ok(Some(values))
}

/// 🧩️ Exports distinct surfaces without flattening seams, channel sets or asset identities.
pub fn semio_mesh_from_meshes(meshes: &[MeshData]) -> Result<SemioMeshSnapshot, semio_framework_diagnostic::TextError> {
    if meshes.is_empty() { return Err(io_error("generation3d export contains no prepared surfaces")); }
    let mut output = SemioMeshSnapshot::default();
    for (index, mesh) in meshes.iter().enumerate() {
        let mut part = semio_mesh_from_mesh_data(mesh)?;
        let prefix = format!("part-{index}-");
        for mesh in &mut part.meshes {
            mesh.id = format!("{prefix}{}", mesh.id);
            for primitive in &mut mesh.primitives {
                primitive.id = format!("{prefix}{}", primitive.id);
                if let Some(id) = &mut primitive.material_id { *id = format!("{prefix}{id}"); }
            }
        }
        for material in &mut part.materials {
            material.id = format!("{prefix}{}", material.id);
            for id in [&mut material.base_color_texture, &mut material.metallic_roughness_texture, &mut material.normal_texture, &mut material.occlusion_texture, &mut material.emissive_texture].into_iter().flatten() { *id = format!("{prefix}{id}"); }
        }
        for texture in &mut part.textures { texture.id = format!("{prefix}{}", texture.id); }
        output.meshes.extend(part.meshes);
        output.materials.extend(part.materials);
        output.textures.extend(part.textures);
    }
    Ok(output)
}

/// 🧩️ Keeps compatible authored channels and scopes material and texture identities across surfaces.
pub fn merge_meshes(meshes: &[MeshData]) -> Result<MeshData, semio_framework_diagnostic::TextError> {
    use semio_framework_plugin::MeshAttributeSemantic;
    let first = meshes.first().ok_or_else(|| io_error("geometry merge contains no surfaces"))?;
    let mut merged = MeshData { paint_texture_base64: first.paint_texture_base64.clone(), ..MeshData::default() };
    let mut signature = None;
    for (part, mesh) in meshes.iter().enumerate() {
        semio_mesh_from_mesh_data(mesh)?;
        if mesh.indices.is_empty() != first.indices.is_empty() || mesh.paint_texture_base64 != first.paint_texture_base64 { return Err(io_error("geometry merge requires compatible surface topology and paint; export separate surfaces")); }
        if merged.positions.len() + mesh.positions.len() > 300_000 || merged.indices.len() + mesh.indices.len() > 300_000 || mesh.edge_positions.len() % 6 != 0 { return Err(io_error("geometry merge exceeds its surface capacity")); }
        let attributes = merge_attributes(mesh)?;
        let current = attributes.iter().map(|(name, attribute)| (name.clone(), attribute.domain, attribute.semantic, attribute.interpolation)).collect::<Vec<_>>();
        if signature.as_ref().is_some_and(|signature| *signature != current) { return Err(io_error("geometry merge requires compatible channel declarations; export separate surfaces")); }
        signature = Some(current);
        if meshes.len() == 1 { return Ok(mesh.clone()); }
        let vertex_offset = merged.positions.len() as u32 / 3;
        merged.positions.extend(&mesh.positions);
        merged.indices.extend(mesh.indices.iter().map(|index| index + vertex_offset));
        for (target, source, reference, width) in [(&mut merged.normals, &mesh.normals, &first.normals, 3), (&mut merged.uvs, &mesh.uvs, &first.uvs, 2), (&mut merged.colors, &mesh.colors, &first.colors, if first.colors.len() == first.positions.len() / 3 * 4 { 4 } else { 3 })] {
            if source.is_empty() != reference.is_empty() || (!source.is_empty() && source.len() != mesh.positions.len() / 3 * width) { return Err(io_error("geometry merge has incompatible vertex buffers; export separate surfaces")); }
            target.extend(source);
        }
        merge_identifiers(&mut merged.vertex_ids, &mesh.vertex_ids, mesh.positions.len() / 3)?;
        merge_identifiers(&mut merged.face_ids, &mesh.face_ids, mesh.indices.len() / 3)?;
        merge_identifiers(&mut merged.edge_ids, &mesh.edge_ids, mesh.edge_positions.len() / 6)?;
        merged.edge_positions.extend(&mesh.edge_positions);
        if mesh.edge_uvs.is_empty() != first.edge_uvs.is_empty() || (!mesh.edge_uvs.is_empty() && mesh.edge_uvs.len() != mesh.edge_positions.len() / 6 * 4) { return Err(io_error("geometry merge has incompatible edge UV buffers; export separate surfaces")); }
        merged.edge_uvs.extend(&mesh.edge_uvs);
        if !mesh.edge_is_seam.is_empty() && mesh.edge_is_seam.len() != mesh.edge_positions.len() / 6 { return Err(io_error("geometry merge has invalid edge seam identities")); }
        merged.edge_is_seam.extend((0..mesh.edge_positions.len() / 6).map(|index| mesh.edge_is_seam.get(index).copied().unwrap_or(0)));
        let prefix = format!("part-{part}-");
        for (name, mut attribute) in attributes {
            if attribute.semantic == MeshAttributeSemantic::Material {
                for value in &mut attribute.values {
                    let semio_framework_value::DslValue::String(id) = value else { return Err(io_error("geometry merge has invalid material references")); };
                    if !mesh.materials.contains_key(id) { return Err(io_error("geometry merge references an unknown material")); }
                    *id = format!("{prefix}{id}");
                }
            }
            if !merged.attributes.contains_key(&name) { merged.attributes.insert(name.clone(), semio_framework_plugin::MeshAttribute { domain: attribute.domain, semantic: attribute.semantic, interpolation: attribute.interpolation, values: Vec::new(), indices: Some(Vec::new()) }); }
            let target = merged.attributes.get_mut(&name).expect("merged channel was just declared");
            let sample_offset = target.values.len() as u32;
            if target.values.len() + attribute.values.len() > 600_000 { return Err(io_error("geometry merge exceeds its channel sample capacity")); }
            let indices = attribute.indices.unwrap_or_else(|| (0..attribute.values.len() as u32).collect());
            target.indices.as_mut().expect("merged channels have sample indices").extend(indices.into_iter().map(|index| index + sample_offset));
            target.values.extend(attribute.values);
        }
        for (id, material) in &mesh.materials {
            let id = format!("{prefix}{id}");
            if id.len() > 128 { return Err(io_error("geometry merge material identifiers exceed their capacity")); }
            let mut material = material.clone();
            let semio_framework_value::DslValue::Object(fields) = &mut material else { return Err(io_error("geometry merge material must be an owned object")); };
            for (field, value) in fields {
                if !field.ends_with("Texture") { continue; }
                let semio_framework_value::DslValue::String(texture) = value else { return Err(io_error("geometry merge material texture must be an identifier")); };
                if !mesh.textures.contains_key(texture) { return Err(io_error("geometry merge material references an unknown texture")); }
                *texture = format!("{prefix}{texture}");
            }
            merged.materials.insert(id, material);
        }
        for (id, texture) in &mesh.textures {
            let id = format!("{prefix}{id}");
            if id.len() > 128 { return Err(io_error("geometry merge texture identifiers exceed their capacity")); }
            merged.textures.insert(id, texture.clone());
        }
        if merged.materials.len() > 10_000 || merged.textures.len() > 256 || merged.textures.values().map(|texture| texture.bytes.len()).sum::<usize>() > 16_000_000 { return Err(io_error("geometry merge exceeds its material or texture capacity")); }
    }
    Ok(merged)
}

fn merge_attributes(mesh: &MeshData) -> Result<semio_framework_mesh_engine::HistoryFoldIndex<String, semio_framework_plugin::MeshAttribute>, semio_framework_diagnostic::TextError> {
    use semio_framework_plugin::{MeshAttribute, MeshAttributeDomain, MeshAttributeSemantic, MeshAttributeInterpolation};
    let mut attributes = mesh.attributes.clone();
    for (buffer, semantic, name, width) in [(&mesh.normals, MeshAttributeSemantic::Normal, "normal", 3), (&mesh.uvs, MeshAttributeSemantic::Uv, "uv", 2), (&mesh.colors, MeshAttributeSemantic::Color, "color", if mesh.colors.len() == mesh.positions.len() / 3 * 4 { 4 } else { 3 })] {
        if buffer.is_empty() { continue; }
        if buffer.len() != mesh.positions.len() / 3 * width || buffer.iter().any(|value| !value.is_finite()) { return Err(io_error("geometry merge has invalid vertex buffers")); }
        if attributes.values().any(|attribute| attribute.semantic == semantic) { continue; }
        if attributes.contains_key(name) { return Err(io_error("geometry merge channel name is occupied")); }
        let values = buffer.chunks_exact(width).map(|tuple| {
            let mut tuple = tuple.iter().map(|value| semio_framework_value::DslValue::float(*value as f64)).collect::<Vec<_>>();
            if semantic == MeshAttributeSemantic::Color && width == 3 { tuple.push(semio_framework_value::DslValue::float(1.0)); }
            semio_framework_value::DslValue::Array(tuple)
        }).collect();
        attributes.insert(name.into(), MeshAttribute { domain: MeshAttributeDomain::Vertex, semantic, interpolation: MeshAttributeInterpolation::Linear, values, indices: None });
    }
    if attributes.len() > 64 { return Err(io_error("geometry merge exceeds its channel capacity")); }
    for attribute in attributes.values() {
        let count = match attribute.domain { MeshAttributeDomain::Vertex => mesh.positions.len() / 3, MeshAttributeDomain::Corner => mesh.indices.len(), MeshAttributeDomain::Face => mesh.indices.len() / 3, MeshAttributeDomain::Edge => mesh.edge_positions.len() / 6 };
        if attribute.domain_len() != count || attribute.indices.as_ref().is_some_and(|indices| indices.iter().any(|index| *index as usize >= attribute.values.len())) { return Err(io_error("geometry merge attribute does not match its domain")); }
    }
    Ok(attributes)
}

fn merge_identifiers(target: &mut Vec<u32>, source: &[u32], count: usize) -> Result<(), semio_framework_diagnostic::TextError> {
    if !source.is_empty() && source.len() != count { return Err(io_error("geometry merge has invalid component identities")); }
    let offset = target.iter().max().copied().map_or(Ok(0), |maximum| maximum.checked_add(1).ok_or_else(|| io_error("geometry merge exceeds its component identity capacity")))?;
    for index in 0..count { target.push(source.get(index).copied().unwrap_or(index as u32).checked_add(offset).ok_or_else(|| io_error("geometry merge exceeds its component identity capacity"))?); }
    Ok(())
}

/// 📥️ Converts validated geometry into this artifact's editable graph document.
pub fn generation3d_document_from_mesh(mesh: &MeshData) -> Result<semio_framework_pack_json::Value, String> {
    let snapshot = import_mesh_data(mesh).map_err(|error| error.to_string())?;
    let value = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&snapshot));
    snapshot.retire_cold();
    Ok(value)
}

/// 📤️ Validates the declared document and geometry prepared by its retained evaluation owner.
pub fn generation3d_mesh_from_document(doc: &semio_framework_value::DslValue, prepared: &MeshData) -> Result<MeshData, String> {
    let snapshot = <Generation3dSnapshot as semio_framework_value::FromValue>::from_value(doc.clone()).map_err(|error| error.to_string())?;
    snapshot.retire_cold();
    semio_mesh_from_mesh_data(prepared).map_err(|error| error.to_string())?;
    Ok(prepared.clone())
}

/// 📦️ Encodes the supplied mesh through the canonical STL grammar.
pub fn stl_ascii_bytes(mesh: &SemioMeshSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    let stl = ::semio_framework_async::poll::resolve_ready(<semio_s_artifact_stdio_semio_mesh_export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl as semio_framework_plugin::ArtifactSerializer>::serialize(mesh))
        .map_err(|error| io_error(error.to_string()))?;
    Ok(semio_s_artifact_stdio_stl::engine::encode_stl_ascii(&stl).into_bytes())
}

/// 📥️ The three-widget import fixture: a note holding the payload, a `brep.io.import*` neuron
/// that turns it into real BRep geometry, and a preview sink. `preview: true` on the neuron is
/// what puts the imported mesh in the 3D window (`widget_previews`).
pub fn import_document(neuron_kind: &str, data_text: String) -> Generation3dSnapshot {
    import_graph(neuron_kind, data_text, "geometry")
}

/// 🥽️ Preserves an indexed triangle mesh as editable polygon data in the procedural graph.
pub fn import_mesh_data(mesh: &MeshData) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    Ok(import_graph("brep.mesh.construct", polygon_mesh_from_mesh_data(mesh)?.to_string(), "meshOut"))
}

/// 🎨️ Keeps primitive seams and surface assets on editable polygon inputs.
pub fn import_semio_mesh(snapshot: &SemioMeshSnapshot) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    use semio_framework_plugin::{MeshAttribute, MeshAttributeDomain, MeshAttributeInterpolation, MeshAttributeSemantic, MeshTexture};
    let mut materials = semio_framework_mesh_engine::HistoryFoldIndex::new();
    for material in &snapshot.materials {
        let color = material.base_color;
        let mut data = semio_framework_pack_json::object([
            ("baseColor".into(), semio_framework_pack_json::array([color.r, color.g, color.b, color.a].map(|value| semio_framework_pack_json::Value::from(value as f64)))),
            ("metallic".into(), (material.metallic as f64).into()),
            ("roughness".into(), (material.roughness as f64).into()),
        ]);
        for (name, texture) in [
            ("baseColorTexture", &material.base_color_texture), ("metallicRoughnessTexture", &material.metallic_roughness_texture),
            ("normalTexture", &material.normal_texture), ("occlusionTexture", &material.occlusion_texture), ("emissiveTexture", &material.emissive_texture),
        ] {
            if let Some(texture) = texture {
                if !snapshot.textures.iter().any(|entry| entry.id == *texture) { return Err(io_error(format!("mesh import references unknown texture '{texture}'"))); }
                data.as_object_mut().expect("material is an object").insert(name, texture.clone().into());
            }
        }
        if materials.insert(material.id.clone(), semio_framework_pack_json::to_dsl_value(&data)).is_some() { return Err(io_error("mesh import has duplicate material identifiers")); }
    }
    let mut textures = semio_framework_mesh_engine::HistoryFoldIndex::new();
    for texture in &snapshot.textures {
        if textures.insert(texture.id.clone(), MeshTexture { mime: texture.mime.clone(), bytes: texture.bytes.clone() }).is_some() { return Err(io_error("mesh import has duplicate texture identifiers")); }
    }
    let mut payloads = Vec::new();
    let mut vertices = 0usize;
    let mut faces = 0usize;
    for source in &snapshot.meshes {
        for primitive in &source.primitives {
            let sequential;
            let indices = if primitive.indices.is_empty() {
                sequential = (0..primitive.positions.len()).map(|index| index as u32).collect::<Vec<_>>();
                &sequential
            } else { &primitive.indices };
            let triangles = match primitive.topology {
                SemioTopology::Triangles => indices.clone(),
                SemioTopology::TriangleStrip => indices.windows(3).enumerate().flat_map(|(index, triangle)| if index % 2 == 0 { [triangle[0], triangle[1], triangle[2]] } else { [triangle[1], triangle[0], triangle[2]] }).collect(),
                SemioTopology::TriangleFan => (1..indices.len().saturating_sub(1)).flat_map(|index| [indices[0], indices[index], indices[index + 1]]).collect(),
                _ => return Err(io_error(format!("mesh import primitive '{}' needs a surface topology", primitive.id))),
            };
            vertices = vertices.saturating_add(primitive.positions.len());
            faces = faces.saturating_add(triangles.len() / 3);
            if vertices > 100_000 || faces > 100_000 { return Err(io_error("mesh import exceeds 100000 vertices or faces")); }
            let mut mesh = MeshData {
                positions: primitive.positions.iter().flat_map(|point| [point.x as f32, point.y as f32, point.z as f32]).collect(),
                normals: primitive.normals.iter().flat_map(|point| [point.x as f32, point.y as f32, point.z as f32]).collect(),
                uvs: primitive.uvs.iter().flat_map(|point| [point.u as f32, point.v as f32]).collect(),
                colors: primitive.colors.iter().flat_map(|color| [color.r, color.g, color.b, color.a]).collect(),
                indices: triangles,
                materials: materials.clone(),
                textures: textures.clone(),
                ..MeshData::default()
            };
            if let Some(material) = &primitive.material_id {
                if !materials.contains_key(material) { return Err(io_error(format!("mesh import references unknown material '{material}'"))); }
                mesh.attributes.insert("material".into(), MeshAttribute { indices:None, domain: MeshAttributeDomain::Face, semantic: MeshAttributeSemantic::Material, interpolation: MeshAttributeInterpolation::Constant, values: vec![semio_framework_value::DslValue::String(material.clone()); mesh.indices.len() / 3] });
            }
            payloads.push(polygon_mesh_from_mesh_data(&mesh)?.to_string());
        }
    }
    import_polygon_meshes(payloads)
}

/// 🧩️ Gives each surface its own existing construct neuron and preview.
pub fn import_polygon_meshes(payloads: Vec<String>) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    if payloads.is_empty() { return Err(io_error("mesh import contains no surfaces")); }
    if payloads.len() > 1024 || payloads.iter().map(String::len).sum::<usize>() > 16_000_000 { return Err(io_error("mesh import exceeds its document capacity")); }
    let mut snapshot = import_graph("brep.mesh.construct", payloads[0].clone(), "meshOut");
    for (index, payload) in payloads.into_iter().enumerate().skip(1) {
        let mut part = import_host("brep.mesh.construct", payload, "meshOut");
        let suffix = format!("-{index}");
        snapshot.host_snapshot.widgets.extend(std::mem::take(&mut part.widgets).into_iter().map(|widget| match widget {
            Widget::InputNote { id, text } => Widget::InputNote { id: format!("{id}{suffix}"), text },
            Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } => Widget::Neuron { id: format!("{id}{suffix}"), neuron_kind, params, input_ports, output_ports, preview },
            Widget::OutputPreview { id, preview, expanded } => Widget::OutputPreview { id: format!("{id}{suffix}"), preview, expanded },
            _ => unreachable!("import graph contains only source, construct and preview widgets"),
        }));
        snapshot.host_snapshot.synapses.extend(std::mem::take(&mut part.synapses).into_iter().map(|mut synapse| {
            synapse.id.push_str(&suffix);
            synapse.from.push_str(&suffix);
            synapse.to.push_str(&suffix);
            synapse
        }));
        for (id, source_layout) in &part.layout {
            let mut layout = source_layout.clone();
            layout.y = index as f64 * 220.0;
            snapshot.host_snapshot.layout.insert(format!("{id}{suffix}"), layout);
        }
        part.retire_cold();
    }
    Ok(snapshot)
}

/// 🧩️ Writes original polygon faces and their authored channels to the existing constructor.
pub fn polygon_import_payload(vertices: Vec<[f64; 3]>, faces: Vec<Vec<u32>>, attributes: semio_framework_pack_json::Object) -> Result<String, semio_framework_diagnostic::TextError> {
    if vertices.len() < 3 || vertices.len() > 100_000 || faces.is_empty() || faces.len() > 100_000 || vertices.iter().flatten().any(|value| !(*value as f32).is_finite()) { return Err(io_error("polygon import requires finite surface geometry within capacity")); }
    if faces.iter().map(Vec::len).sum::<usize>() > 600_000 || faces.iter().any(|face| face.len() < 3 || face.iter().any(|index| *index as usize >= vertices.len()) || face.iter().collect::<std::collections::BTreeSet<_>>().len() != face.len()) { return Err(io_error("polygon import has invalid face indices")); }
    if attributes.len() > 64 { return Err(io_error("polygon import exceeds its attribute capacity")); }
    let mut data = semio_framework_pack_json::Object::new();
    data.insert("vertices", semio_framework_pack_json::array(vertices.into_iter().map(|point| semio_framework_pack_json::array(point.map(semio_framework_pack_json::Value::from)))));
    data.insert("faces", semio_framework_pack_json::array(faces.into_iter().map(|face| semio_framework_pack_json::array(face.into_iter().map(semio_framework_pack_json::Value::from)))));
    if !attributes.is_empty() { data.insert("attributes", semio_framework_pack_json::Value::Object(attributes)); }
    let payload = semio_framework_pack_json::Value::Object(data).to_string();
    if payload.len() > 16_000_000 { return Err(io_error("polygon import exceeds 16 MB")); }
    Ok(payload)
}

/// 🎨️ Uses canonical channel fields and indexed samples for source corner seams.
pub fn polygon_import_attribute(domain: &str, semantic: &str, interpolation: &str, values: Vec<semio_framework_pack_json::Value>, indices: Option<Vec<u32>>) -> semio_framework_pack_json::Value {
    let mut attribute = semio_framework_pack_json::Object::new();
    attribute.insert("domain", domain.into());
    attribute.insert("semantic", semantic.into());
    attribute.insert("interpolation", interpolation.into());
    attribute.insert("values", semio_framework_pack_json::array(values));
    if let Some(indices) = indices { attribute.insert("indices", semio_framework_pack_json::array(indices.into_iter().map(semio_framework_pack_json::Value::from))); }
    semio_framework_pack_json::Value::Object(attribute)
}

/// 🎨️ Keeps authored domain channels and surface assets in the existing polygon payload.
pub fn polygon_mesh_from_mesh_data(mesh: &MeshData) -> Result<semio_framework_pack_json::Value, semio_framework_diagnostic::TextError> {
    use semio_framework_value::ToValue;
    if mesh.positions.len() < 9 || mesh.positions.len() > 300_000 || mesh.positions.len() % 3 != 0 || mesh.positions.iter().any(|value| !value.is_finite()) {
        return Err(io_error("mesh import requires 3..100000 finite three-dimensional vertices"));
    }
    if mesh.indices.is_empty() || mesh.indices.len() > 300_000 || mesh.indices.len() % 3 != 0 {
        return Err(io_error("mesh import requires 1..100000 indexed triangles"));
    }
    if mesh.indices.iter().any(|id| *id as usize >= mesh.positions.len() / 3) || mesh.indices.chunks_exact(3).any(|triangle| triangle[0] == triangle[1] || triangle[1] == triangle[2] || triangle[0] == triangle[2]) {
        return Err(io_error("mesh import contains an invalid triangle index"));
    }
    let vertices = semio_framework_pack_json::array(mesh.positions.chunks_exact(3).map(|point| semio_framework_pack_json::array(point.iter().map(|coordinate| semio_framework_pack_json::Value::from(*coordinate as f64)))));
    let faces = semio_framework_pack_json::array(mesh.indices.chunks_exact(3).map(|triangle| semio_framework_pack_json::array(triangle.iter().map(|id| semio_framework_pack_json::Value::from(*id)))));
    let count = mesh.positions.len() / 3;
    let mut attributes = match semio_framework_pack_json::from_dsl_value(&mesh.attributes.to_value()) {
        semio_framework_pack_json::Value::Object(attributes) => attributes,
        _ => return Err(io_error("mesh attributes must be an owned object")),
    };
    for (buffer, semantic, width) in [(&mesh.normals, "normal", 3), (&mesh.uvs, "uv", 2), (&mesh.colors, "color", if mesh.colors.len() == count * 4 { 4 } else { 3 })] {
        if buffer.is_empty() { continue; }
        if buffer.len() != count * width || buffer.iter().any(|value| !value.is_finite()) { return Err(io_error(format!("mesh import {semantic} buffer does not match its vertices"))); }
        if attributes.iter().any(|(_, attribute)| attribute.get("semantic").and_then(semio_framework_pack_json::Value::as_str) == Some(semantic)) { continue; }
        if attributes.contains_key(semantic) { return Err(io_error(format!("mesh import {semantic} channel name is occupied"))); }
        let values = semio_framework_pack_json::array(buffer.chunks_exact(width).map(|tuple| {
            let mut values = tuple.iter().map(|value| semio_framework_pack_json::Value::from(*value as f64)).collect::<Vec<_>>();
            if semantic == "color" && width == 3 { values.push(semio_framework_pack_json::Value::from(1.0)); }
            semio_framework_pack_json::array(values)
        }));
        attributes.insert(semantic, semio_framework_pack_json::object([("domain".into(), "vertex".into()), ("semantic".into(), semantic.into()), ("interpolation".into(), "linear".into()), ("values".into(), values)]));
    }
    let mut data = semio_framework_pack_json::Object::new();
    data.insert("vertices", vertices);
    data.insert("faces", faces);
    if !attributes.is_empty() { data.insert("attributes", semio_framework_pack_json::Value::Object(attributes)); }
    if !mesh.materials.is_empty() { data.insert("materials", semio_framework_pack_json::from_dsl_value(&mesh.materials.to_value())); }
    if !mesh.textures.is_empty() { data.insert("textures", semio_framework_pack_json::from_dsl_value(&mesh.textures.to_value())); }
    let data = semio_framework_pack_json::Value::Object(data);
    if data.to_string().len() > 16_000_000 { return Err(io_error("mesh input exceeds 16 MB")); }
    Ok(data)
}

fn import_graph(neuron_kind: &str, data_text: String, output_port: &str) -> Generation3dSnapshot {
    Generation3dSnapshot { host_snapshot: import_host(neuron_kind, data_text, output_port), ..Generation3dSnapshot::default() }
}

fn import_host(neuron_kind: &str, data_text: String, output_port: &str) -> FlowHostSnapshot {
    let mut layout = OrderedMap::new();
    layout.insert(IMPORT_SOURCE_WIDGET.to_string(), WidgetLayout { x: 0.0, y: 0.0 });
    layout.insert(IMPORT_GEOMETRY_WIDGET.to_string(), WidgetLayout { x: 260.0, y: 0.0 });
    layout.insert(IMPORT_PREVIEW_WIDGET.to_string(), WidgetLayout { x: 520.0, y: 0.0 });
    FlowHostSnapshot {
        schema: "flow.host_snapshot".into(),
        camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
        widgets: vec![
            Widget::InputNote { id: IMPORT_SOURCE_WIDGET.into(), text: data_text },
            Widget::Neuron { id: IMPORT_GEOMETRY_WIDGET.into(), neuron_kind: neuron_kind.into(), params: Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: true },
            Widget::OutputPreview { id: IMPORT_PREVIEW_WIDGET.into(), preview: Dictionary::new(), expanded: OrderedSet::new() },
        ],
        synapses: vec![
            SynapseSpec { id: "imported-data".into(), from: IMPORT_SOURCE_WIDGET.into(), to: IMPORT_GEOMETRY_WIDGET.into(), from_port: "text".into(), to_port: "data".into() },
            SynapseSpec { id: "imported-geometry".into(), from: IMPORT_GEOMETRY_WIDGET.into(), to: IMPORT_PREVIEW_WIDGET.into(), from_port: output_port.into(), to_port: String::new() },
        ],
        layout,
    }
}

/// 🔎️ The payload an import fixture planted, and the neuron kind that consumes it.
pub fn imported_source(snapshot: &Generation3dSnapshot) -> Option<(&str, &str)> {
    let text = snapshot.host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::InputNote { id, text } if id == IMPORT_SOURCE_WIDGET => Some(text.as_str()),
        _ => None,
    })?;
    let kind = snapshot.host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::Neuron { id, neuron_kind, .. } if id == IMPORT_GEOMETRY_WIDGET => Some(neuron_kind.as_str()),
        _ => None,
    })?;
    Some((kind, text))
}
