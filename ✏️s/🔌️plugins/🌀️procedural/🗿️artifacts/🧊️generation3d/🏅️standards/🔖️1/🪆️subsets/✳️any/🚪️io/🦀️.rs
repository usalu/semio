//! 🚪️ IO s.generation3d (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from ⚙️engine::register), not per-leaf register().
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.json", "stdio.obj", "stdio.ply", "stdio.stl", "stdio.txt"]
}
// 🖼️ No "stdio.png"/"stdio.json" here (export-only list): generation2d owns those EXPORT claims, see
// `🚪️IoRegistry` region below. Import is unaffected — both stay in `import_stdio_kinds` above.
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.gltf", "stdio.las", "stdio.obj", "stdio.ply", "stdio.stl", "stdio.txt"]
}
//#region 🔺️MeshBridge
/// 🔺️ Converts caller-prepared geometry and editable graph documents through the owning codecs.
/// Geometry evaluation and retention belong to the calling composition.
pub mod mesh_bridge {
    use crate::Generation3dSnapshot;
    use semio_framework_artifact_flow_flow::neural::Dictionary;
    use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, OrderedMap, OrderedSet, SynapseSpec, Widget, WidgetLayout};
    use semio_framework_plugin::MeshData;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::{SemioMaterial, SemioMesh, SemioMeshSnapshot, SemioPrimitive, SemioTexture, SemioTopology, STDIO_SEMIOMESH_DOCUMENT_SCHEMA};

    /// 🏷️ Widget ids the import fixture plants, stable so a re-import overwrites rather than stacks.
    pub const IMPORT_SOURCE_WIDGET: &str = "imported-source";
    pub const IMPORT_GEOMETRY_WIDGET: &str = "imported-geometry";
    pub const IMPORT_PREVIEW_WIDGET: &str = "imported-preview";
    /// 🏷️ `SemioMesh` id every export carries — the merged preview, not a per-widget mesh.
    pub const EXPORT_MESH_ID: &str = "generation3d-preview";

    /// 🔌️ Resolves the existing graph connections that own an export's geometry.
    pub fn export_source_channels(snapshot: &FlowHostSnapshot, widget_id: Option<&str>) -> Result<Vec<(String, Option<String>)>, semio_framework_diagnostic::TextError> {
        let Some(widget_id) = widget_id else {
            return Ok(snapshot.widgets.iter().filter(|widget| crate::widget_previews(widget)).map(|widget| (crate::widget_id(widget).to_string(), None)).collect());
        };
        if !snapshot.widgets.iter().any(|widget| matches!(widget, Widget::OutputExport { id, .. } if id == widget_id)) { return Err(io_error("choose an export output")); }
        let mut channels = Vec::new();
        for wire in snapshot.synapses.iter().filter(|wire| wire.to == widget_id) {
            if !snapshot.widgets.iter().any(|widget| crate::widget_id(widget) == wire.from) { return Err(io_error("export output has no connected source")); }
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
        semio_s_artifact_stdio_gltf::engine::b64_encode(bytes)
    }

    /// 🔤️ Inverse of [`base64_encode`], for reading an import fixture's planted payload back.
    pub fn base64_decode(text: &str) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
        semio_s_artifact_stdio_gltf::engine::b64_decode(text).map_err(io_error)
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
            let defaults = semio_s_artifact_stdio_gltf::schema::snapshot::GltfPbrMetallicRoughness::default();
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
                let target = merged.attributes.entry(name).or_insert_with(|| semio_framework_plugin::MeshAttribute { domain: attribute.domain, semantic: attribute.semantic, interpolation: attribute.interpolation, values: Vec::new(), indices: Some(Vec::new()) });
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

    fn merge_attributes(mesh: &MeshData) -> Result<std::collections::BTreeMap<String, semio_framework_plugin::MeshAttribute>, semio_framework_diagnostic::TextError> {
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
        let stl = ::semio_framework_async::poll::resolve_ready(<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::export::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl as semio_framework_plugin::ArtifactSerializer>::serialize(mesh))
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
        let mut materials = std::collections::BTreeMap::new();
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
        let mut textures = std::collections::BTreeMap::new();
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
}
pub use mesh_bridge::io_error;
//#endregion 🔺️MeshBridge

//#region 📄️DocumentIo
/// 📄️ The USER-facing half of this artifact's IO — the roster the editor's and the viewer's
/// `exportDocument`/`importDocument` commands pick from, the filename and MIME every download
/// carries, and the two byte-level entry points those commands call.
///
/// 🐛️ Until ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane the nine leaves below
/// `📤️export`/`📥️import` were round-trip tested and **unreachable**: no command, menu item, button
/// or keybinding in `✏️editor` or `👁️viewer` named them, so a user could not import or export
/// anything (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1). This module is the composition
/// point that closed that gap; it re-implements no leaf and spells no file grammar.
///
/// 🏷️ A row's `id` is what the user picks (the `format` `ActionArgDef::select` option); its
/// `kind_id` is the owning `s.stdio.<format>` artifact's OWN representation id, and the extension,
/// MIME and binary-ness come from THAT artifact's `formats()` — this module never restates a
/// format's file facts, so a format that renames its extension cannot drift from the download.
///
/// @see ../../../../../../🗄️stdio/🗿️artifacts — every row's owning artifact and its `formats()`.
/// @see ../✏️editor/🎮️commands/📤️export-document, ../✏️editor/🎮️commands/📥️import-document.
pub mod document_io {
    use super::mesh_bridge::io_error;
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export_leaves;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import_leaves;
    use crate::Generation3dSnapshot;
    use semio_framework_plugin::io::FormatDescriptor;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

    /// 🏷️ One row of the user-facing format roster: the id the user picks, the two labels the picker
    /// shows, and the owning stdio artifact's representation this artifact's leaf writes or reads.
    ///
    /// 🗣️ English first, German second, with NO default: `LocalizedLabel::native` takes both and a
    /// locale resolves one, so a row cannot exist in one language only.
    #[derive(Debug)]
    pub struct Generation3dFormatRow {
        pub id: &'static str,
        pub label_en: &'static str,
        pub label_de: &'static str,
        pub kind_id: &'static str,
        pub descriptors: fn() -> Result<Vec<FormatDescriptor>, semio_framework_plugin::ArtifactDefinitionError>,
    }

    /// 📤️ Every format `exportDocument` offers, in menu order. Exactly [`super::export_stdio_kinds`]
    /// — `json`/`png` are absent because generation2d owns those EXPORT claims (see `🚪️IoRegistry`),
    /// and offering a user a format this artifact does not claim would be a lie in the picker.
    pub const EXPORT_FORMATS: [Generation3dFormatRow; 7] = [
        Generation3dFormatRow { id: "stl", label_en: "STL Mesh", label_de: "STL-Netz", kind_id: "s.stdio.stl.standard.ascii.representation.document", descriptors: semio_s_artifact_stdio_stl::formats },
        Generation3dFormatRow { id: "obj", label_en: "OBJ Mesh", label_de: "OBJ-Netz", kind_id: "s.stdio.obj.standard.3-0.representation.document", descriptors: semio_s_artifact_stdio_obj::formats },
        Generation3dFormatRow { id: "ply", label_en: "PLY Mesh", label_de: "PLY-Netz", kind_id: "s.stdio.ply.standard.1-0.representation.document", descriptors: semio_s_artifact_stdio_ply::formats },
        Generation3dFormatRow { id: "gltf", label_en: "glTF Mesh", label_de: "glTF-Netz", kind_id: "s.stdio.gltf.standard.2-0.representation.document", descriptors: semio_s_artifact_stdio_gltf::formats },
        Generation3dFormatRow { id: "las", label_en: "LAS Point Cloud", label_de: "LAS-Punktwolke", kind_id: "s.stdio.las.standard.1-0.representation.document", descriptors: semio_s_artifact_stdio_las::formats },
        Generation3dFormatRow { id: "dwg", label_en: "DWG Drawing", label_de: "DWG-Zeichnung", kind_id: "s.stdio.dwg.standard.ac1018.representation.document", descriptors: semio_s_artifact_stdio_dwg::formats },
        Generation3dFormatRow { id: "txt", label_en: "Semio Text (whole artifact)", label_de: "Semio-Text (ganzes Artefakt)", kind_id: "s.stdio.txt.standard.utf-8.representation.document", descriptors: semio_s_artifact_stdio_txt::formats },
    ];

    /// 📥️ Every format `importDocument` accepts, in picker order — the SEVEN of
    /// [`super::import_stdio_kinds`]'s nine whose deserializer really reconstructs a document.
    /// `las` and `png` are deliberately absent: their import leaves are honest `Err`s (LAS is
    /// export-only, PNG carries no recoverable graph), and an `accept` filter that offered them
    /// would put files in the picker that can only fail. `no_import_formats_can_be_deserialized`
    /// pins that, so a leaf that later gains a real decoder cannot stay silently out of the picker.
    pub const IMPORT_FORMATS: [Generation3dFormatRow; 7] = [
        Generation3dFormatRow { id: "stl", label_en: "STL Mesh", label_de: "STL-Netz", kind_id: "s.stdio.stl.standard.ascii.representation.document", descriptors: semio_s_artifact_stdio_stl::formats },
        Generation3dFormatRow { id: "obj", label_en: "OBJ Mesh", label_de: "OBJ-Netz", kind_id: "s.stdio.obj.standard.3-0.representation.document", descriptors: semio_s_artifact_stdio_obj::formats },
        Generation3dFormatRow { id: "ply", label_en: "PLY Mesh", label_de: "PLY-Netz", kind_id: "s.stdio.ply.standard.1-0.representation.document", descriptors: semio_s_artifact_stdio_ply::formats },
        Generation3dFormatRow { id: "gltf", label_en: "glTF Mesh", label_de: "glTF-Netz", kind_id: "s.stdio.gltf.standard.2-0.representation.document", descriptors: semio_s_artifact_stdio_gltf::formats },
        Generation3dFormatRow { id: "dwg", label_en: "DWG Drawing", label_de: "DWG-Zeichnung", kind_id: "s.stdio.dwg.standard.ac1018.representation.document", descriptors: semio_s_artifact_stdio_dwg::formats },
        Generation3dFormatRow { id: "json", label_en: "Generation JSON", label_de: "Generation-JSON", kind_id: "s.stdio.json.standard.rfc8259.representation.document", descriptors: semio_s_artifact_stdio_json::formats },
        Generation3dFormatRow { id: "txt", label_en: "Semio Text (whole artifact)", label_de: "Semio-Text (ganzes Artefakt)", kind_id: "s.stdio.txt.standard.utf-8.representation.document", descriptors: semio_s_artifact_stdio_txt::formats },
    ];

    /// 🏷️ The owning artifact's own descriptor for one row — the single source of extension, MIME
    /// and binary-ness.
    pub fn descriptor_of(row: &Generation3dFormatRow) -> Result<FormatDescriptor, semio_framework_diagnostic::TextError> {
        let descriptors = (row.descriptors)().map_err(|error| io_error(format!("generation3d io: format `{}` has no readable descriptor set ({error})", row.id)))?;
        descriptors
            .into_iter()
            .find(|descriptor| descriptor.kind_id == row.kind_id)
            .ok_or_else(|| io_error(format!("generation3d io: format `{}` declares representation `{}`, which its owning artifact does not publish", row.id, row.kind_id)))
    }

    fn row_of(rows: &'static [Generation3dFormatRow], id: &str) -> Result<&'static Generation3dFormatRow, semio_framework_diagnostic::TextError> {
        rows.iter()
            .find(|row| row.id == id)
            .ok_or_else(|| io_error(format!("generation3d io: `{id}` is not one of this artifact's formats ({})", rows.iter().map(|row| row.id).collect::<Vec<_>>().join(", "))))
    }

    /// 🗣️ One `ActionArgOption` per [`EXPORT_FORMATS`] row, in table order — the editor's picker and
    /// the viewer's are THE SAME list, built once here rather than spelled twice, so a format this
    /// artifact stops claiming disappears from both surfaces at once.
    pub fn export_format_options() -> Vec<semio_framework_plugin::ActionArgOption> {
        EXPORT_FORMATS.iter().map(|row| semio_framework_plugin::ActionArgOption::new(row.id, semio_framework_ui_locale::LocalizedLabel::native(row.label_en, row.label_de))).collect()
    }

    /// 🗂️ The file picker's `accept` filter — every importable extension, comma-joined, exactly the
    /// shape `Effect::RequestFileOpen` carries.
    pub fn import_accept_filter() -> Result<String, semio_framework_diagnostic::TextError> {
        let mut extensions = Vec::new();
        for row in IMPORT_FORMATS.iter() {
            extensions.extend(descriptor_of(row)?.extensions);
        }
        Ok(extensions.join(","))
    }

    /// 📤️ One finished export, ready to become an `Effect::DownloadMediaExport`.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Generation3dDocumentExport {
        pub filename: String,
        pub data: String,
        pub mime_type: String,
        pub encoding: Option<String>,
    }
    semio_framework_value::artifact_retire_struct!(Generation3dDocumentExport { filename, data, mime_type, encoding });

    /// 🗣️ Reports a real format limitation through the app's localized inspector.
    pub struct Generation3dIoDiagnostic {
        pub code: &'static str,
        pub label: semio_framework_ui_locale::LocalizedLabel,
    }

    /// 📤️ Describes authored surface data the selected interchange codec cannot retain.
    pub fn format_diagnostics(meshes: &[semio_framework_plugin::MeshData], format: &str) -> Vec<Generation3dIoDiagnostic> {
        use semio_framework_plugin::MeshAttributeSemantic;
        if format == "txt" { return Vec::new(); }
        let supported: &[&str] = match format {
            "gltf" => &["normal", "uv", "color", "material", "texture", "custom"],
            "obj" => &["normal", "uv"],
            "ply" => &["normal", "uv", "color"],
            "las" => &["color"],
            _ => &[],
        };
        let present = [
            ("normal", "Authored normals", "Erstellte Normalen", meshes.iter().any(|mesh| !mesh.normals.is_empty() || mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Normal))),
            ("uv", "UV coordinates", "UV-Koordinaten", meshes.iter().any(|mesh| !mesh.uvs.is_empty() || mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Uv))),
            ("color", "Vertex colors", "Knotenfarben", meshes.iter().any(|mesh| !mesh.colors.is_empty() || mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Color))),
            ("material", "Material assignments and properties", "Materialzuweisungen und Eigenschaften", meshes.iter().any(|mesh| !mesh.materials.is_empty() || mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Material))),
            ("texture", "Texture images and bindings", "Texturbilder und Verknüpfungen", meshes.iter().any(|mesh| !mesh.textures.is_empty())),
            ("custom", "Custom attributes", "Benutzerdefinierte Attribute", meshes.iter().any(|mesh| mesh.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Custom))),
        ];
        let mut diagnostics: Vec<_> = present.into_iter().filter(|(code, _, _, present)| *present && !supported.contains(code)).map(|(code, en, de, _)| Generation3dIoDiagnostic {
            code,
            label: semio_framework_ui_locale::LocalizedLabel::native(
                &format!("{en} will not be retained by {}. The Semio Text document retains the editable data.", format.to_uppercase()),
                &format!("{de} bleiben in {} nicht erhalten. Das Semio-Text-Dokument erhält die bearbeitbaren Daten.", format.to_uppercase()),
            ),
        }).collect();
        let mut add = |code: &'static str, en: String, de: String| diagnostics.push(Generation3dIoDiagnostic { code, label: semio_framework_ui_locale::LocalizedLabel::native(&en, &de) });
        for (name, semantic, code) in [("normal",MeshAttributeSemantic::Normal,"multiple-normal"),("uv",MeshAttributeSemantic::Uv,"multiple-uv"),("color",MeshAttributeSemantic::Color,"multiple-color"),("material",MeshAttributeSemantic::Material,"multiple-material")] {
            if format != "gltf" && supported.contains(&name) && meshes.iter().any(|mesh| mesh.attributes.values().filter(|attribute| attribute.semantic == semantic).count() > 1) { add(code, format!("{} retains one {name} channel. Additional sets remain in the Semio Text document.",format.to_uppercase()), format!("{} erhält einen {name}-Kanal. Zusätzliche Sätze bleiben im Semio-Text-Dokument.",format.to_uppercase())); }
        }
        if matches!(format,"stl"|"ply"|"las"|"dwg") && meshes.len() > 1 { add("part-grouping","This format combines surface parts and their identities. The editable graph retains each part.".into(),"Dieses Format verbindet Oberflächenteile und ihre Kennungen. Der bearbeitbare Graph erhält jedes Teil.".into()); }
        if format == "las" && meshes.iter().any(|mesh| !mesh.indices.is_empty()) { add("topology","LAS exports points and cannot retain face connectivity.".into(),"LAS exportiert Punkte und kann die Flächenverbindungen nicht erhalten.".into()); }
        let levels = match format { "ply" => 255.0, "las" => 65535.0, _ => 0.0 };
        let mut color_precision = false;
        let mut color_alpha = false;
        for mesh in meshes {
            let mut check = |values: &[f64]| {
                color_precision |= levels > 0.0 && values.iter().take(if format == "las" {3}else{4}).any(|value| (value-(value*levels).round()/levels).abs() > 1e-8);
                color_alpha |= values.len() == 4 && values[3] != 1.0;
            };
            if let Some(attribute) = mesh.attributes.values().find(|attribute| attribute.semantic == MeshAttributeSemantic::Color) {
                for value in &attribute.values { if let Some(values) = value.as_array() { check(&values.iter().filter_map(semio_framework_value::DslValue::as_f64).collect::<Vec<_>>()); } }
            } else {
                let width = if mesh.colors.len() == mesh.positions.len()/3*4 {4}else{3};
                for color in mesh.colors.chunks(width) { check(&color.iter().map(|value| *value as f64).collect::<Vec<_>>()); }
            }
        }
        if color_precision { let bits = if format == "ply" {8}else{16}; add("color-precision",format!("{} rounds color channels to {bits} bits.",format.to_uppercase()),format!("{} rundet Farbkanäle auf {bits} Bit.",format.to_uppercase())); }
        if format == "las" && color_alpha { add("color-alpha","LAS cannot retain color transparency.".into(),"LAS kann die Farbtransparenz nicht erhalten.".into()); }
        if format == "las" && meshes.iter().any(|mesh| mesh.positions.iter().any(|value| {let value=*value as f64;(value-(value/0.0001).round()*0.0001).abs()>1e-8})) { add("coordinate-precision","LAS rounds positions to the codec's 0.0001-unit grid.".into(),"LAS rundet Positionen auf das Raster des Codecs mit 0.0001 Einheiten.".into()); }
        diagnostics
    }

    /// 📤️ The MESH half of every geometry export, isolated exactly as each leaf's own
    /// `serialize_mesh_bytes` is: it needs no flow evaluator, so the format table, the filename and
    /// the wire envelope are all provable natively against the committed unit-cube host_snapshot.
    pub fn export_mesh_bytes(mesh: &SemioMeshSnapshot, format: &str) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
        match format {
            "stl" => export_leaves::stl::v_ascii::any::serialize_mesh_bytes(mesh),
            "obj" => export_leaves::obj::v3_0::any::serialize_mesh_bytes(mesh),
            "ply" => export_leaves::ply::v1_0::any::serialize_mesh_bytes(mesh),
            "gltf" => export_leaves::gltf::v2_0::any::serialize_mesh_bytes(mesh),
            "las" => export_leaves::las::v1_0::any::serialize_mesh_bytes(mesh),
            "dwg" => export_leaves::dwg::v_ac1018::any::serialize_mesh_bytes(mesh),
            "txt" => Err(io_error("generation3d export: `txt` is the whole document's own text, not a mesh — it has no mesh-only half")),
            other => Err(io_error(format!("generation3d export: `{other}` is not one of this artifact's formats"))),
        }
    }

    /// 📄️ Encodes the reversible graph document without geometry evaluation.
    pub fn export_document_bytes(snapshot: &Generation3dSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
        export_leaves::txt::v_utf_8::any::serialize_bytes(snapshot)
    }

    /// 🧵️ Retains the physical output until its exact textual or binary envelope can publish.
    pub struct Generation3dDocumentEnvelope {
        row: &'static Generation3dFormatRow, bytes: Option<Vec<u8>>, position: usize, binary: Option<bool>,
        filename: Option<String>, mime_type: Option<String>, data: Option<String>, complete: bool,
    }

    impl Generation3dDocumentEnvelope {
        /// 🌱️ Takes existing physical output without copying or interpreting its bytes.
        pub fn new(row: &'static Generation3dFormatRow, bytes: Vec<u8>) -> Self { Self { row, bytes: Some(bytes), position: 0, binary: None, filename: None, mime_type: None, data: None, complete: false } }
        /// 📍️ Returns the processed physical byte count.
        pub fn position(&self) -> usize { self.position }
        /// ⏱️ Encodes one base64 triple or validates one UTF-8 scalar per supplied unit.
        pub fn step(&mut self, maximum_units: usize, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Option<Generation3dDocumentExport>, semio_framework_diagnostic::TextError> {
            let controlled = |error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1));
            for _ in 0..maximum_units {
                control.checkpoint().map_err(controlled)?;
                if self.complete { return Ok(None); }
                if self.binary.is_none() {
                    let descriptor = descriptor_of(self.row)?;
                    let extension = descriptor.extensions.first().ok_or_else(|| io_error("generation3d export format claims no extension"))?;
                    let mime = descriptor.mimes.first().ok_or_else(|| io_error("generation3d export format claims no MIME type"))?;
                    self.filename = Some(control.copy_text(&format!("generation3d{extension}")).map_err(controlled)?);
                    self.mime_type = Some(control.copy_text(mime).map_err(controlled)?);
                    if descriptor.is_binary {
                        let length = self.bytes.as_ref().unwrap().len().div_ceil(3).checked_mul(4).ok_or_else(|| io_error("generation3d export envelope size overflow"))?;
                        control.charge(length).map_err(controlled)?;
                        let mut output = String::new(); output.try_reserve_exact(length).map_err(|_| io_error("generation3d export envelope allocation failed"))?;
                        self.data = Some(output);
                    }
                    self.binary = Some(descriptor.is_binary);
                } else {
                    let bytes = self.bytes.as_ref().expect("physical source is retained");
                    if self.position == bytes.len() {
                        control.checkpoint().map_err(controlled)?;
                        if self.binary == Some(false) { self.data = Some(unsafe { String::from_utf8_unchecked(self.bytes.take().unwrap()) }); }
                        let encoding = if self.binary == Some(true) { Some(control.copy_text("base64").map_err(controlled)?) } else { None };
                        self.complete = true;
                        return Ok(Some(Generation3dDocumentExport { filename: self.filename.take().unwrap(), mime_type: self.mime_type.take().unwrap(), data: self.data.take().unwrap(), encoding }));
                    }
                    if self.binary == Some(true) {
                        let end = (self.position + 3).min(bytes.len());
                        control.charge(4).map_err(controlled)?;
                        self.data.as_mut().unwrap().push_str(&super::mesh_bridge::base64_encode(&bytes[self.position..end]));
                        control.advance(end - self.position).map_err(controlled)?; self.position = end;
                    } else {
                        let first = bytes[self.position];
                        let width = if first < 0x80 { 1 } else if first < 0xe0 { 2 } else if first < 0xf0 { 3 } else { 4 };
                        let end = (self.position + width).min(bytes.len());
                        std::str::from_utf8(&bytes[self.position..end]).map_err(|error| io_error(format!("generation3d textual export has invalid UTF-8 ({error})")))?;
                        control.advance(end - self.position).map_err(controlled)?; self.position = end;
                    }
                }
                control.step().map_err(controlled)?;
            }
            Ok(None)
        }
    }

    impl semio_framework_value::retirement::RetireOwned for Generation3dDocumentEnvelope {
        fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { use semio_framework_value::retirement::{sequence,deferred};sequence(vec![deferred(self.bytes),deferred(self.filename),deferred(self.mime_type),deferred(self.data)]) }
        fn retirement_birth_bytes(&self) -> Option<usize> { use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.bytes),deferred_birth_bytes_for(&self.filename),deferred_birth_bytes_for(&self.mime_type),deferred_birth_bytes_for(&self.data)]) }
        fn controlled_retirement_supported() -> bool { true }
    }

    /// 📦️ Wraps raw export bytes in the wire envelope `Effect::DownloadMediaExport` expects: the
    /// owning artifact's extension and MIME, and base64 exactly when that artifact calls itself
    /// binary. A row that claims to be text and hands back bytes that are not UTF-8 is a typed
    /// error, never a lossy `from_utf8_lossy` download.
    pub fn document_export_envelope(row: &Generation3dFormatRow, bytes: Vec<u8>) -> Result<Generation3dDocumentExport, semio_framework_diagnostic::TextError> {
        let descriptor = descriptor_of(row)?;
        let extension = descriptor.extensions.first().cloned().ok_or_else(|| io_error(format!("generation3d export: format `{}` claims no extension", row.id)))?;
        let mime_type = descriptor.mimes.first().cloned().ok_or_else(|| io_error(format!("generation3d export: format `{}` claims no MIME type", row.id)))?;
        let filename = format!("generation3d{extension}");
        if descriptor.is_binary {
            return Ok(Generation3dDocumentExport { filename, data: super::mesh_bridge::base64_encode(&bytes), mime_type, encoding: Some("base64".into()) });
        }
        let data = String::from_utf8(bytes).map_err(|error| io_error(format!("generation3d export: format `{}` is declared textual but its bytes are not UTF-8 ({error})", row.id)))?;
        Ok(Generation3dDocumentExport { filename, data, mime_type, encoding: None })
    }

    /// 📤️ Encodes caller-prepared geometry in its declared format.
    pub fn export_geometry(mesh: &SemioMeshSnapshot, format: &str) -> Result<Generation3dDocumentExport, semio_framework_diagnostic::TextError> {
        let row = row_of(&EXPORT_FORMATS, format)?;
        document_export_envelope(row, export_mesh_bytes(mesh, row.id)?)
    }

    /// 🎨️ Keeps rich prepared surfaces until their declared serializer has emitted the interchange.
    pub fn export_prepared_geometry(meshes: &[semio_framework_plugin::MeshData], format: &str) -> Result<Generation3dDocumentExport, semio_framework_diagnostic::TextError> {
        let row = row_of(&EXPORT_FORMATS, format)?;
        let bytes = if format == "gltf" { export_leaves::gltf::v2_0::any::serialize_prepared_meshes_bytes(meshes)? } else { export_mesh_bytes(&super::mesh_bridge::semio_mesh_from_meshes(meshes)?, format)? };
        document_export_envelope(row, bytes)
    }

    /// 📄️ Exports the reversible graph document as its own text.
    pub fn export_document(snapshot: &Generation3dSnapshot) -> Result<Generation3dDocumentExport, semio_framework_diagnostic::TextError> {
        let row = row_of(&EXPORT_FORMATS, "txt")?;
        document_export_envelope(row, export_document_bytes(snapshot)?)
    }

    /// 📦️ Decodes what `Effect::RequestFileOpen { read_as: Some("dataUrl") }` hands back. A shell
    /// that answers with the raw text instead (the `read_as: None` case, and every test harness) is
    /// read as its own UTF-8 bytes.
    pub fn import_payload_bytes(payload: &str) -> Result<std::borrow::Cow<'_, [u8]>, semio_framework_diagnostic::TextError> {
        if let Some((header, encoded)) = payload.split_once(',') {
            if header.starts_with("data:") {
                return if header.ends_with(";base64") { super::mesh_bridge::base64_decode(encoded).map(std::borrow::Cow::Owned) } else { Ok(std::borrow::Cow::Borrowed(encoded.as_bytes())) };
            }
        }
        Ok(std::borrow::Cow::Borrowed(payload.as_bytes()))
    }

    /// 🏷️ The roster row a picked file's name resolves to, by the owning artifact's OWN extension
    /// claim — never a hand-written extension table.
    pub fn import_row_for_name(name: &str) -> Result<&'static Generation3dFormatRow, semio_framework_diagnostic::TextError> {
        let lowered = name.to_ascii_lowercase();
        for row in IMPORT_FORMATS.iter() {
            for extension in descriptor_of(row)?.extensions {
                if lowered.ends_with(&extension.to_ascii_lowercase()) {
                    return Ok(row);
                }
            }
        }
        Err(io_error(format!("generation3d import: `{name}` is not one of this artifact's importable formats ({})", IMPORT_FORMATS.iter().map(|row| row.id).collect::<Vec<_>>().join(", "))))
    }

    /// 📥️ The bytes of one picked file as a real document, through that format's own leaf.
    pub fn import_document_bytes(format: &str, bytes: &[u8]) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
        match format {
            "stl" => import_leaves::stl::v_ascii::any::deserialize_bytes(bytes),
            "obj" => import_leaves::obj::v3_0::any::deserialize_bytes(bytes),
            "ply" => import_leaves::ply::v1_0::any::deserialize_bytes(bytes),
            "gltf" => import_leaves::gltf::v2_0::any::deserialize_bytes(bytes),
            "dwg" => import_leaves::dwg::v_ac1018::any::deserialize_bytes(bytes),
            "json" => import_leaves::json::v_rfc8259::any::deserialize_bytes(bytes),
            "txt" => import_leaves::txt::v_utf_8::any::deserialize_bytes(bytes),
            other => Err(io_error(format!("generation3d import: `{other}` is not one of this artifact's importable formats"))),
        }
    }

    /// 📥️ The whole import, from a picked file to a document.
    pub fn import_document(name: &str, payload: &str) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
        let row = import_row_for_name(name)?;
        import_document_bytes(row.id, &import_payload_bytes(payload)?)
    }
}
pub use document_io::{export_document, import_document as import_picked_document, Generation3dDocumentExport};
//#endregion 📄️DocumentIo

//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::io::Generation3dAnalyzer;
    use crate::Generation3dSnapshot;
    use {semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GLTF: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_OBJ: Dialect = Dialect { artifact_kind: "s.stdio.obj", standard: StandardId("3.0"), subset: SubsetId("*") };
    const DEP_PLY: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };
    const DEP_STL: Dialect = Dialect { artifact_kind: "s.stdio.stl", standard: StandardId("ascii"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    pub struct Generation3dComposerComposition;

    impl ArtifactComposition for Generation3dComposerComposition {
        type Snapshot = Generation3dSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_DWG, DEP_GLTF, DEP_JSON, DEP_OBJ, DEP_PLY, DEP_STL, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = Generation3dAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GLTF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::gltf::v2_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_OBJ {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::obj::v3_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PLY {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::ply::v1_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_STL {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::stl::v_ascii::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TXT {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::io::Confidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "Generation3dComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️IoRegistry
/// 🚪️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// the composer/export-entry registry lives with the rest of `🚪️io`, not behind an engine facade.
pub mod io_registry {
    use crate::standards::v1::subsets::any::io::Generation3dBuilder as Generation3dAnyBuilder;
    use crate::standards::v1::subsets::any::io::Generation3dComposer as Generation3dAnyComposer;
    use {semio_framework_plugin::composer_entry_of,semio_framework_plugin::ArtifactBuilder,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposedArtifact,semio_framework_plugin::ComposerEntry,semio_framework_artifact_reference::Dialect,semio_framework_plugin::ErasedComposeSource,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Graph compositions preserve document text; prepared geometry uses the owning mesh codecs.
    const GENERATION3D_DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };
    const GENERATION3D_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    async fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::Generation3dSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == GENERATION3D_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => Generation3dAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => Generation3dAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "Generation3dComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == GENERATION3D_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "Generation3dComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
    fn compose_export_txt(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources).await?;
            let encoded = crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::serialize_bytes(&snapshot);
            snapshot.retire_cold();
            let bytes = encoded.map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_TXT_DIALECT, payload: IoPayload::Text(String::from_utf8(bytes).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?), diagnostics: Vec::new(), confidence: semio_framework_plugin::io::Confidence::High })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<Generation3dAnyComposer>(),
                    ComposerEntry { writes: EXPORT_TXT_DIALECT, reads: &[GENERATION3D_DIALECT], compose: compose_export_txt },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️IoRegistry

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🪶️sqlite/🦀️.rs"]
pub mod sqlite;

pub mod derived_construction {
    use crate::{Generation3dDiff, Generation3dMutation, Generation3dSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct Generation3dBuilderConstruction {
        snapshot: Generation3dSnapshot,
        diagnostics: Vec<semio_framework_diagnostic::Diagnostic>,
    }

    impl ArtifactBuilder for Generation3dBuilderConstruction {
        type Snapshot = Generation3dSnapshot;
        type Mutation = Generation3dMutation;
        type Diff = Generation3dDiff;
        fn empty() -> Self {
            Self { snapshot: Generation3dSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<Generation3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
            match protocol::apply_diff(outcome.diff(), &self.snapshot) {
                Ok(snapshot) => self.snapshot = snapshot,
                Err(error) => self.diagnostics.push(semio_framework_diagnostic::Diagnostic::error("build.apply", semio_framework_diagnostic::TextSpan::at(1, 1), error.to_string())),
            }
            (self, outcome)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            let snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            self.snapshot = snapshot;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<semio_framework_diagnostic::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::Generation3dSnapshot;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct Generation3dParts {
        pub snapshot: Option<Generation3dSnapshot>,
    }

    pub struct Generation3dAnalyzerAnalysis;

    impl ArtifactAnalysis for Generation3dAnalyzerAnalysis {
        type Parts = Generation3dParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.procedural.generation3d", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            semio_framework_plugin::io::Confidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = Generation3dParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = semio_framework_plugin::io::Confidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.text", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <Generation3dSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = semio_framework_plugin::io::Confidence::Low;
                            diagnostics.push(semio_framework_diagnostic::Diagnostic::error("analyze.binary", semio_framework_diagnostic::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec Generation3dBuilderFacets {
        construction: Generation3dBuilderConstruction,
        analysis: Generation3dAnalyzerAnalysis,
        composition: crate::standards::v1::subsets::any::io::derived_composition::Generation3dComposerComposition,
    }
    builder: Generation3dBuilder,
    analyzer: Generation3dAnalyzer,
    composer: Generation3dComposer,
);

#[path="📐️geometry/🦀️.rs"]
pub mod geometry;

#[path = "📝️text/🔣️value/🦀️.rs"]
mod value_codec;
