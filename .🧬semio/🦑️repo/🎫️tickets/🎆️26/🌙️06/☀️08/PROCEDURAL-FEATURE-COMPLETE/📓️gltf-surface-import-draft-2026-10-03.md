# Authored glTF Surface Import Implementation

The actual native feature red executed in io-surface-native-current-generation-red.log: missing custom channel, doubly normalized colors, unsupported Face channels. The following code was then applied inside the existing glTF importer and Semio primitive consumer. Native verification is active; no full pass is claimed.

```rust
/// 🎨️ Projects authored sampling into the existing editable material fields.
fn material_surface(document: &GltfDocument, primitive: &GltfPrimitive) -> Result<semio_framework_pack_json::Object, semio_framework_diagnostic::TextError> {
    use semio_framework_pack_json::{Object, Value as Json, array};
    let default_material = GltfMaterial::default();
    let material = match primitive.material { Some(index) => document.materials.get(index).ok_or_else(|| io_error("glTF primitive references an unknown material"))?, None => &default_material };
    let scale = material.normal_texture.as_ref().map_or(1.0, |binding| binding.scale);
    let strength = material.occlusion_texture.as_ref().map_or(1.0, |binding| binding.strength);
    if [scale, strength, material.alpha_cutoff].into_iter().any(|value| !value.is_finite() || !(value as f32).is_finite()) || !(0.0..=1.0).contains(&strength) || !(0.0..=1.0).contains(&material.alpha_cutoff) || material.emissive_factor.iter().any(|value| !value.is_finite() || !(0.0..=1.0).contains(value)) { return Err(io_error("glTF surface coefficients are outside the editable material domain")); }
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

/// 🧩️ Preserves every authored vertex stream in the same imported constructor payload.
fn enrich_surfaces(snapshot: &mut Generation3dSnapshot, from: &GltfSnapshot) -> Result<(), semio_framework_diagnostic::TextError> {
    use crate::standards::v1::subsets::any::io::mesh_bridge::{IMPORT_SOURCE_WIDGET, polygon_import_attribute};
    use semio_framework_pack_json::{Object, Value as Json, array, JsonMemberPolicy};
    let primitives: Vec<_> = from.document.meshes.iter().flat_map(|mesh| &mesh.primitives).collect();
    let mut total = 0usize;
    let mut part = 0usize;
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
        let fields = material_surface(&from.document, primitive)?;
        if let Some(index) = primitive.material {
            let material = payload.get_mut("materials").and_then(Json::as_object_mut).and_then(|materials| materials.get_mut(&format!("mat-{index}"))).and_then(Json::as_object_mut).ok_or_else(|| io_error("glTF constructor is missing its authored material"))?;
            for (key, value) in fields.into_entries() { material.insert(key, value); }
        }
        payload.as_object_mut().ok_or_else(|| io_error("glTF constructor is not an object"))?.insert("attributes", Json::Object(attributes));
        let enriched = payload.to_string();
        total = total.checked_add(enriched.len()).ok_or_else(|| io_error("glTF surface payload capacity overflow"))?;
        if total > 16_000_000 { return Err(io_error("glTF surface exceeds its document capacity")); }
        *text = enriched;
        part += 1;
    }
    if part != primitives.len() { return Err(io_error("glTF primitive source count disagrees")); }
    Ok(())
}
```

```rust
fn decode_primitive(document: &GltfDocument, buffers: &[Vec<u8>], prim: &GltfPrimitive, id: String, material_id: Option<String>) -> Result<SemioPrimitive, String> {
    let topology = gltf_mode_to_topology(prim.mode)?;
    let position = decode_accessor(document, buffers, find_attr(&prim.attributes, "POSITION").ok_or("primitive missing mandatory POSITION attribute")?)?;
    if position.accessor_type.components() != 3 || position.count == 0 || position.count > u32::MAX as usize || position.components.iter().any(|value| !value.is_finite() || !(*value as f32).is_finite()) { return Err("POSITION needs a finite three-component vertex domain".into()); }
    let positions = position.components.chunks_exact(3).map(|values| SemioPoint3 { x: values[0], y: values[1], z: values[2] }).collect();
    let normals = match find_attr(&prim.attributes, "NORMAL") {
        Some(index) => {
            let values = decode_accessor(document, buffers, index)?;
            if values.accessor_type.components() != 3 || values.count != position.count || values.components.iter().any(|value| !value.is_finite()) || values.components.chunks_exact(3).any(|value| value.iter().map(|value| value * value).sum::<f64>() <= 1e-24) { return Err("NORMAL needs one finite nonzero three-component tuple per vertex".into()); }
            values.components.chunks_exact(3).map(|values| SemioPoint3 { x: values[0], y: values[1], z: values[2] }).collect()
        }, None => Vec::new(),
    };
    let uvs = match find_attr(&prim.attributes, "TEXCOORD_0") {
        Some(index) => {
            let values = decode_accessor(document, buffers, index)?;
            if values.accessor_type.components() != 2 || values.count != position.count || values.components.iter().any(|value| !value.is_finite() || !(*value as f32).is_finite()) { return Err("TEXCOORD_0 needs one finite two-component tuple per vertex".into()); }
            values.components.chunks_exact(2).map(|values| SemioUv { u: values[0], v: values[1] }).collect()
        }, None => Vec::new(),
    };
    let colors = match find_attr(&prim.attributes, "COLOR_0") {
        Some(index) => {
            let values = decode_accessor(document, buffers, index)?;
            let width = values.accessor_type.components();
            if !matches!(width, 3 | 4) || values.count != position.count || values.components.iter().any(|value| !value.is_finite() || !(0.0..=1.0).contains(value)) { return Err("COLOR_0 needs one normalized three- or four-component tuple per vertex".into()); }
            values.components.chunks_exact(width).map(|values| SemioRgba { r: values[0] as f32, g: values[1] as f32, b: values[2] as f32, a: if width == 4 { values[3] as f32 } else { 1.0 } }).collect()
        }, None => Vec::new(),
    };
    let indices = match prim.indices {
        Some(index) => {
            let values = decode_accessor(document, buffers, index)?;
            let maximum = match values.component_type { GltfComponentType::UnsignedByte => u8::MAX as f64, GltfComponentType::UnsignedShort => u16::MAX as f64, GltfComponentType::UnsignedInt => u32::MAX as f64, _ => return Err("primitive indices need an unsigned integer component type".into()) };
            if values.accessor_type.components() != 1 || values.normalized || values.components.iter().any(|value| !value.is_finite() || value.fract() != 0.0 || *value < 0.0 || *value >= position.count as f64 || *value == maximum) { return Err("primitive indices need unnormalized unsigned scalar values inside the vertex domain".into()); }
            values.components.iter().map(|value| *value as u32).collect()
        }, None => Vec::new(),
    };
    Ok(SemioPrimitive { id, topology, positions, normals, uvs, colors, indices, material_id })
}
```
