#!/usr/bin/env python3
"""Patch w4b step/ifc/las/gltf with real codecs."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]

BREP_MODEL = r'''
//#region 🔖️BrepModel
/// 📍 B-rep vertex.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔺 B-rep face as polygon vertex indices.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepFace {
    #[serde(default)]
    pub indices: Vec<usize>,
}

/// 📐 Neutral B-rep mesh extracted from CAD text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepMesh {
    #[serde(default)]
    pub vertices: Vec<BrepVertex>,
    #[serde(default)]
    pub faces: Vec<BrepFace>,
}
//#endregion 🔖️BrepModel
'''

MESH_MODEL = r'''
//#region 🔖️MeshModel
/// 📍 Point or mesh vertex.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
//#endregion 🔖️MeshModel
'''

STEP_CODEC = r'''
fn step_collect_entities(text: &str) -> std::collections::HashMap<u64, String> {
    let mut map = std::collections::HashMap::new();
    let mut cur_id: Option<u64> = None;
    let mut cur_body = String::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('#') {
            if let Some((id_s, after)) = rest.split_once('=') {
                if let Ok(id) = id_s.trim().parse::<u64>() {
                    if let Some(prev) = cur_id.take() {
                        map.insert(prev, cur_body.trim().trim_end_matches(';').to_string());
                    }
                    cur_id = Some(id);
                    cur_body = after.trim().to_string();
                    if cur_body.ends_with(';') {
                        map.insert(id, cur_body.trim_end_matches(';').to_string());
                        cur_id = None;
                        cur_body.clear();
                    }
                    continue;
                }
            }
        }
        if cur_id.is_some() {
            cur_body.push(' ');
            cur_body.push_str(line);
            if line.ends_with(';') {
                if let Some(id) = cur_id.take() {
                    map.insert(id, cur_body.trim().trim_end_matches(';').to_string());
                    cur_body.clear();
                }
            }
        }
    }
    if let Some(id) = cur_id {
        map.insert(id, cur_body.trim().trim_end_matches(';').to_string());
    }
    map
}

fn step_parse_cartesian(body: &str) -> Option<BrepVertex> {
    let open = body.find('(')?;
    let inner = &body[open + 1..];
    let close = inner.rfind(')')?;
    let tuple = inner[..close].trim();
    let nums: Vec<f64> = tuple
        .split(',')
        .filter_map(|p| p.trim().parse().ok())
        .collect();
    if nums.len() >= 3 {
        Some(BrepVertex { x: nums[0], y: nums[1], z: nums[2] })
    } else {
        None
    }
}

fn step_parse_poly_loop(body: &str, id_to_idx: &std::collections::HashMap<u64, usize>) -> Vec<usize> {
    let mut out = Vec::new();
    for part in body.split('#') {
        let id_s: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(id) = id_s.parse::<u64>() {
            if let Some(&idx) = id_to_idx.get(&id) {
                out.push(idx);
            }
        }
    }
    out
}

pub fn step_brep_from_text(text: &str) -> Result<BrepMesh, String> {
    let entities = step_collect_entities(text);
    let mut id_to_idx = std::collections::HashMap::new();
    let mut mesh = BrepMesh::default();
    for (id, body) in &entities {
        let upper = body.to_ascii_uppercase();
        if upper.starts_with("CARTESIAN_POINT") {
            if let Some(v) = step_parse_cartesian(body) {
                id_to_idx.insert(*id, mesh.vertices.len());
                mesh.vertices.push(v);
            }
        }
    }
    for (_id, body) in &entities {
        let upper = body.to_ascii_uppercase();
        if upper.starts_with("POLY_LOOP") {
            let idx = step_parse_poly_loop(body, &id_to_idx);
            if idx.len() >= 3 {
                mesh.faces.push(BrepFace { indices: idx });
            }
        }
    }
    Ok(mesh)
}

pub fn step_brep_to_text(mesh: &BrepMesh) -> String {
    let mut out = String::from("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','',(''),(''),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n");
    let mut next_id = 1u64;
    let mut v_ids = Vec::new();
    for v in &mesh.vertices {
        out.push_str(&format!(
            "#{next_id}=CARTESIAN_POINT('',({},{},{}));\n",
            v.x, v.y, v.z
        ));
        v_ids.push(next_id);
        next_id += 1;
    }
    for face in &mesh.faces {
        let refs: Vec<String> = face.indices.iter().filter_map(|&i| v_ids.get(i).map(|id| format!("#{id}"))).collect();
        if refs.len() < 3 {
            continue;
        }
        let loop_id = next_id;
        next_id += 1;
        out.push_str(&format!("#{loop_id}=POLY_LOOP('',({}));\n", refs.join(",")));
        let face_id = next_id;
        next_id += 1;
        out.push_str(&format!("#{face_id}=FACE_OUTER_BOUND('',#{loop_id},.T.);\n"));
        let adv = next_id;
        next_id += 1;
        out.push_str(&format!("#{adv}=ADVANCED_FACE('',(#{face_id}),.F.);\n"));
    }
    out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
    out
}
'''

IFC_CODEC = r'''
fn ifc_collect_entities(text: &str) -> std::collections::HashMap<u64, String> {
    let mut map = std::collections::HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with('#') {
            continue;
        }
        let rest = &line[1..];
        let Some((id_s, after)) = rest.split_once('=') else { continue };
        let Ok(id) = id_s.trim().parse::<u64>() else { continue };
        let body = after.trim().trim_end_matches(';').to_string();
        map.insert(id, body);
    }
    map
}

fn ifc_parse_point(body: &str) -> Option<BrepVertex> {
    let upper = body.to_ascii_uppercase();
    if !upper.starts_with("IFCCARTESIANPOINT") {
        return None;
    }
    let open = body.find('(')?;
    let inner = &body[open..];
    let nums: Vec<f64> = inner
        .split(|c: char| c == '(' || c == ')' || c == ',')
        .filter_map(|p| p.trim().parse().ok())
        .collect();
    if nums.len() >= 3 {
        Some(BrepVertex { x: nums[0], y: nums[1], z: nums[2] })
    } else {
        None
    }
}

fn ifc_refs(body: &str) -> Vec<u64> {
    let mut out = Vec::new();
    for part in body.split('#') {
        let id_s: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
        if let Ok(id) = id_s.parse() {
            out.push(id);
        }
    }
    out
}

pub fn ifc_brep_from_text(text: &str) -> Result<BrepMesh, String> {
    let entities = ifc_collect_entities(text);
    let mut id_to_idx = std::collections::HashMap::new();
    let mut mesh = BrepMesh::default();
    for (id, body) in &entities {
        if let Some(v) = ifc_parse_point(body) {
            id_to_idx.insert(*id, mesh.vertices.len());
            mesh.vertices.push(v);
        }
    }
    for (_id, body) in &entities {
        let upper = body.to_ascii_uppercase();
        if upper.starts_with("IFCPOLYLOOP") {
            let mut idx = Vec::new();
            for rid in ifc_refs(body) {
                if let Some(&i) = id_to_idx.get(&rid) {
                    idx.push(i);
                }
            }
            if idx.len() >= 3 {
                mesh.faces.push(BrepFace { indices: idx });
            }
        }
    }
    for (_id, body) in &entities {
        let upper = body.to_ascii_uppercase();
        if upper.starts_with("IFCFACE") && mesh.faces.is_empty() {
            for rid in ifc_refs(body) {
                if let Some(bound) = entities.get(&rid) {
                    let bu = bound.to_ascii_uppercase();
                    if bu.starts_with("IFCFACEOUTERBOUND") {
                        for br in ifc_refs(bound) {
                            if let Some(loop_body) = entities.get(&br) {
                                if loop_body.to_ascii_uppercase().starts_with("IFCPOLYLOOP") {
                                    let mut idx = Vec::new();
                                    for pr in ifc_refs(loop_body) {
                                        if let Some(&i) = id_to_idx.get(&pr) {
                                            idx.push(i);
                                        }
                                    }
                                    if idx.len() >= 3 {
                                        mesh.faces.push(BrepFace { indices: idx });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(mesh)
}

pub fn ifc_brep_to_text(mesh: &BrepMesh) -> String {
    let mut out = String::from("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');\nFILE_NAME('semio.ifc','',(''),(''),'semio','','');\nFILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n");
    let mut next = 1u64;
    let mut pt_ids = Vec::new();
    for v in &mesh.vertices {
        out.push_str(&format!("#{next}=IFCCARTESIANPOINT(({},{},{}));\n", v.x, v.y, v.z));
        pt_ids.push(next);
        next += 1;
    }
    for face in &mesh.faces {
        let refs: Vec<String> = face.indices.iter().filter_map(|&i| pt_ids.get(i).map(|id| format!("#{id}"))).collect();
        if refs.len() < 3 {
            continue;
        }
        let loop_id = next;
        next += 1;
        out.push_str(&format!("#{loop_id}=IFCPOLYLOOP(({refs}));\n", refs = refs.join(",")));
        let bound_id = next;
        next += 1;
        out.push_str(&format!("#{bound_id}=IFCFACEOUTERBOUND(#{loop_id},.T.);\n"));
        let face_id = next;
        next += 1;
        out.push_str(&format!("#{face_id}=IFCFACE((#{bound_id}));\n"));
    }
    out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
    out
}
'''

LAS_CODEC = r'''
pub fn las_vertices_from_bytes(bytes: &[u8]) -> Result<Vec<MeshVertex>, String> {
    if bytes.len() < 227 {
        return Err("las header too short".into());
    }
    if &bytes[0..4] != b"LASF" {
        return Err("las signature missing".into());
    }
    let point_offset = u32::from_le_bytes(bytes[96..100].try_into().map_err(|_| "offset")?) as usize;
    let point_count = u32::from_le_bytes(bytes[107..111].try_into().map_err(|_| "count")?) as usize;
    let point_format = bytes[104];
    let record_len = u16::from_le_bytes(bytes[105..107].try_into().map_err(|_| "rlen")?) as usize;
    if record_len == 0 {
        return Err("las record length zero".into());
    }
    let x_scale = f64::from_le_bytes(bytes[131..139].try_into().map_err(|_| "xs")?);
    let y_scale = f64::from_le_bytes(bytes[139..147].try_into().map_err(|_| "ys")?);
    let z_scale = f64::from_le_bytes(bytes[147..155].try_into().map_err(|_| "zs")?);
    let x_off = f64::from_le_bytes(bytes[155..163].try_into().map_err(|_| "xo")?);
    let y_off = f64::from_le_bytes(bytes[163..171].try_into().map_err(|_| "yo")?);
    let z_off = f64::from_le_bytes(bytes[171..179].try_into().map_err(|_| "zo")?);
    let data_start = if point_offset >= 227 { point_offset } else { 227 };
    if point_format != 0 {
        return Err(format!("unsupported las point format {point_format}"));
    }
    let mut verts = Vec::with_capacity(point_count.min(1_000_000));
    let mut pos = data_start;
    for _ in 0..point_count {
        if pos + 20 > bytes.len() {
            break;
        }
        let xi = i32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
        let yi = i32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap());
        let zi = i32::from_le_bytes(bytes[pos + 8..pos + 12].try_into().unwrap());
        verts.push(MeshVertex {
            x: xi as f64 * x_scale + x_off,
            y: yi as f64 * y_scale + y_off,
            z: zi as f64 * z_scale + z_off,
        });
        pos += record_len;
    }
    Ok(verts)
}

pub fn las_bytes_from_vertices(verts: &[MeshVertex]) -> Vec<u8> {
    let header_size = 227usize;
    let record_len = 20u16;
    let count = verts.len() as u32;
    let mut out = vec![0u8; header_size + verts.len() * record_len as usize];
    out[0..4].copy_from_slice(b"LASF");
    out[24..26].copy_from_slice(&1u16.to_le_bytes());
    out[104] = 0;
    out[105..107].copy_from_slice(&record_len.to_le_bytes());
    out[107..111].copy_from_slice(&count.to_le_bytes());
    let x_scale = 0.01f64;
    let y_scale = 0.01f64;
    let z_scale = 0.01f64;
    out[131..139].copy_from_slice(&x_scale.to_le_bytes());
    out[139..147].copy_from_slice(&y_scale.to_le_bytes());
    out[147..155].copy_from_slice(&z_scale.to_le_bytes());
    out[96..100].copy_from_slice(&(header_size as u32).to_le_bytes());
    let mut pos = header_size;
    for v in verts {
        let xi = ((v.x) / x_scale).round() as i32;
        let yi = ((v.y) / y_scale).round() as i32;
        let zi = ((v.z) / z_scale).round() as i32;
        out[pos..pos + 4].copy_from_slice(&xi.to_le_bytes());
        out[pos + 4..pos + 8].copy_from_slice(&yi.to_le_bytes());
        out[pos + 8..pos + 12].copy_from_slice(&zi.to_le_bytes());
        pos += record_len as usize;
    }
    out
}
'''

GLTF_CODEC = r'''
fn gltf_decode_buffer_uri(uri: &str) -> Result<Vec<u8>, String> {
    let Some(data) = uri.strip_prefix("data:application/octet-stream;base64,") else {
        return Err("gltf buffer uri must be embedded base64".into());
    };
    use std::io::Read;
    let mut dec = base64::read::DecoderReader::new(data.as_bytes(), base64::engine::general_purpose::STANDARD);
    let mut buf = Vec::new();
    dec.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf)
}

pub fn gltf_vertices_from_value(value: &serde_json::Value) -> Result<Vec<MeshVertex>, String> {
    let accessors = value.get("accessors").and_then(|v| v.as_array()).ok_or("missing accessors")?;
    let buffer_views = value.get("bufferViews").and_then(|v| v.as_array()).ok_or("missing bufferViews")?;
    let buffers = value.get("buffers").and_then(|v| v.as_array()).ok_or("missing buffers")?;
    let meshes = value.get("meshes").and_then(|v| v.as_array()).ok_or("missing meshes")?;
    let mut pos_accessor: Option<usize> = None;
    'outer: for mesh in meshes {
        let prims = mesh.get("primitives").and_then(|v| v.as_array()).ok_or("missing primitives")?;
        for prim in prims {
            if let Some(idx) = prim.get("attributes").and_then(|a| a.get("POSITION")).and_then(|v| v.as_u64()) {
                pos_accessor = Some(idx as usize);
                break 'outer;
            }
        }
    }
    let acc_idx = pos_accessor.ok_or("no POSITION accessor")?;
    let acc = accessors.get(acc_idx).ok_or("accessor index")?;
    if acc.get("type").and_then(|v| v.as_str()) != Some("VEC3") {
        return Err("POSITION must be VEC3".into());
    }
    if acc.get("componentType").and_then(|v| v.as_u64()) != Some(5126) {
        return Err("POSITION must be FLOAT".into());
    }
    let bv_idx = acc.get("bufferView").and_then(|v| v.as_u64()).ok_or("bufferView")? as usize;
    let bv = buffer_views.get(bv_idx).ok_or("bufferView idx")?;
    let buf_idx = bv.get("buffer").and_then(|v| v.as_u64()).ok_or("buffer")? as usize;
    let buf = buffers.get(buf_idx).ok_or("buffer idx")?;
    let uri = buf.get("uri").and_then(|v| v.as_str()).ok_or("buffer uri")?;
    let bytes = gltf_decode_buffer_uri(uri)?;
    let byte_offset = bv.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize
        + acc.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let count = acc.get("count").and_then(|v| v.as_u64()).ok_or("count")? as usize;
    let mut verts = Vec::with_capacity(count);
    let mut pos = byte_offset;
    for _ in 0..count {
        if pos + 12 > bytes.len() {
            break;
        }
        let x = f32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as f64;
        let y = f32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as f64;
        let z = f32::from_le_bytes(bytes[pos + 8..pos + 12].try_into().unwrap()) as f64;
        verts.push(MeshVertex { x, y, z });
        pos += 12;
    }
    Ok(verts)
}

pub fn gltf_value_from_vertices(verts: &[MeshVertex]) -> serde_json::Value {
    let mut bin = Vec::with_capacity(verts.len() * 12);
    for v in verts {
        bin.extend_from_slice(&(v.x as f32).to_le_bytes());
        bin.extend_from_slice(&(v.y as f32).to_le_bytes());
        bin.extend_from_slice(&(v.z as f32).to_le_bytes());
    }
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bin);
    let uri = format!("data:application/octet-stream;base64,{b64}");
    serde_json::json!({
        "asset": { "version": "2.0" },
        "buffers": [{ "byteLength": bin.len(), "uri": uri }],
        "bufferViews": [{ "buffer": 0, "byteOffset": 0, "byteLength": bin.len() }],
        "accessors": [{
            "bufferView": 0,
            "componentType": 5126,
            "count": verts.len(),
            "type": "VEC3",
            "max": [1.0, 1.0, 1.0],
            "min": [0.0, 0.0, 0.0]
        }],
        "meshes": [{ "primitives": [{ "attributes": { "POSITION": 0 } }] }]
    })
}
'''

# base64 - we should NOT use external crate per rules for runtime. Use hand-rolled base64 for gltf.

GLTF_CODEC = r'''
fn b64_decode(data: &str) -> Result<Vec<u8>, String> {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = Vec::new();
    let mut buf = 0u32;
    let mut bits = 0u32;
    for ch in data.bytes().filter(|&b| b != b'=' && !b.is_ascii_whitespace()) {
        let val = TABLE.iter().position(|&t| t == ch).ok_or("invalid base64")? as u32;
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
            buf &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

fn b64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { TABLE[((n >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[(n & 63) as usize] as char } else { '=' });
    }
    out
}

fn gltf_decode_buffer_uri(uri: &str) -> Result<Vec<u8>, String> {
    let Some(data) = uri.strip_prefix("data:application/octet-stream;base64,") else {
        return Err("gltf buffer uri must be embedded base64".into());
    };
    b64_decode(data)
}

pub fn gltf_vertices_from_value(value: &serde_json::Value) -> Result<Vec<MeshVertex>, String> {
    let accessors = value.get("accessors").and_then(|v| v.as_array()).ok_or("missing accessors")?;
    let buffer_views = value.get("bufferViews").and_then(|v| v.as_array()).ok_or("missing bufferViews")?;
    let buffers = value.get("buffers").and_then(|v| v.as_array()).ok_or("missing buffers")?;
    let meshes = value.get("meshes").and_then(|v| v.as_array()).ok_or("missing meshes")?;
    let mut pos_accessor: Option<usize> = None;
    'outer: for mesh in meshes {
        let prims = mesh.get("primitives").and_then(|v| v.as_array()).ok_or("missing primitives")?;
        for prim in prims {
            if let Some(idx) = prim.get("attributes").and_then(|a| a.get("POSITION")).and_then(|v| v.as_u64()) {
                pos_accessor = Some(idx as usize);
                break 'outer;
            }
        }
    }
    let acc_idx = pos_accessor.ok_or("no POSITION accessor")?;
    let acc = accessors.get(acc_idx).ok_or("accessor index")?;
    if acc.get("type").and_then(|v| v.as_str()) != Some("VEC3") {
        return Err("POSITION must be VEC3".into());
    }
    if acc.get("componentType").and_then(|v| v.as_u64()) != Some(5126) {
        return Err("POSITION must be FLOAT".into());
    }
    let bv_idx = acc.get("bufferView").and_then(|v| v.as_u64()).ok_or("bufferView")? as usize;
    let bv = buffer_views.get(bv_idx).ok_or("bufferView idx")?;
    let buf_idx = bv.get("buffer").and_then(|v| v.as_u64()).ok_or("buffer")? as usize;
    let buf = buffers.get(buf_idx).ok_or("buffer idx")?;
    let uri = buf.get("uri").and_then(|v| v.as_str()).ok_or("buffer uri")?;
    let bytes = gltf_decode_buffer_uri(uri)?;
    let byte_offset = bv.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize
        + acc.get("byteOffset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let count = acc.get("count").and_then(|v| v.as_u64()).ok_or("count")? as usize;
    let mut verts = Vec::with_capacity(count);
    let mut pos = byte_offset;
    for _ in 0..count {
        if pos + 12 > bytes.len() {
            break;
        }
        let x = f32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as f64;
        let y = f32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as f64;
        let z = f32::from_le_bytes(bytes[pos + 8..pos + 12].try_into().unwrap()) as f64;
        verts.push(MeshVertex { x, y, z });
        pos += 12;
    }
    Ok(verts)
}

pub fn gltf_value_from_vertices(verts: &[MeshVertex]) -> serde_json::Value {
    let mut bin = Vec::with_capacity(verts.len() * 12);
    for v in verts {
        bin.extend_from_slice(&(v.x as f32).to_le_bytes());
        bin.extend_from_slice(&(v.y as f32).to_le_bytes());
        bin.extend_from_slice(&(v.z as f32).to_le_bytes());
    }
    let b64 = b64_encode(&bin);
    let uri = format!("data:application/octet-stream;base64,{b64}");
    serde_json::json!({
        "asset": { "version": "2.0" },
        "buffers": [{ "byteLength": bin.len(), "uri": uri }],
        "bufferViews": [{ "buffer": 0, "byteOffset": 0, "byteLength": bin.len() }],
        "accessors": [{
            "bufferView": 0,
            "componentType": 5126,
            "count": verts.len(),
            "type": "VEC3",
            "max": [1.0, 1.0, 1.0],
            "min": [0.0, 0.0, 0.0]
        }],
        "meshes": [{ "primitives": [{ "attributes": { "POSITION": 0 } }] }]
    })
}
'''


def brep_snapshot(mid: str, schema_const: str, ext: str, envelope: str, parse_fn: str, print_fn: str, codec: str) -> str:
    name = mid.capitalize() if mid != "ifc" else "Ifc"
    if mid == "step":
        name = "Step"
    return f'''//! 🧬️ {name}Snapshot schema — persistent fields + real codecs.

use crate::artifacts::{mid}::{schema_const};
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};
{BREP_MODEL}
//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.{mid}` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.{mid}")]
pub struct {name}Snapshot {{
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub brep: BrepMesh,
}}

impl Default for {name}Snapshot {{
    fn default() -> Self {{
        Self {{
            schema: {schema_const}.into(),
            brep: BrepMesh::default(),
        }}
    }}
}}
//#endregion 🔖️Snapshot

//#region 🔖️CadTextCodec
{codec}

impl store::DocumentDsl for {name}Snapshot {{
    const EXTENSION: &'static str = "{ext}";
    fn envelope_id() -> &'static str {{ "{envelope}" }}

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {{
        let body = match store::semio_format::split_text_preamble(text) {{
            Ok((_, rest)) => rest,
            Err(_) => text,
        }};
        let brep = {parse_fn}(body).map_err(|e| {{
            store::TextError::new(format!("{mid} parse: {{e}}"), dsl::TextSpan::at(1, 1))
        }})?;
        Ok(Self {{ schema: {schema_const}.into(), brep }})
    }}
    fn print_dsl(&self) -> String {{
        let body = {print_fn}(&self.brep);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }}
}}

impl store::DocumentPack for {name}Snapshot {{
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {{
        let _ = options;
        let raw = {print_fn}(&self.brep).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }}
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {{
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {{
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {{}}, got {{}}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }}
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let brep = {parse_fn}(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self {{ schema: {schema_const}.into(), brep }})
    }}
}}
//#endregion 🔖️CadTextCodec
'''


def las_snapshot() -> str:
    return f'''//! 🧬️ LasSnapshot schema — persistent fields + real codecs.

use crate::artifacts::las::STDIO_LAS_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};
{MESH_MODEL}
//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las")]
pub struct LasSnapshot {{
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<MeshVertex>,
}}

impl Default for LasSnapshot {{
    fn default() -> Self {{
        Self {{
            schema: STDIO_LAS_DOCUMENT_SCHEMA.into(),
            vertices: Vec::new(),
        }}
    }}
}}
//#endregion 🔖️Snapshot

//#region 🔖️LasBinaryCodec
{LAS_CODEC}

impl store::DocumentDsl for LasSnapshot {{
    const EXTENSION: &'static str = "las";
    fn envelope_id() -> &'static str {{ "stdio.las" }}

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {{
        let body = match store::semio_format::split_text_preamble(text) {{
            Ok((_, rest)) => rest,
            Err(_) => text,
        }};
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {{
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }}
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {{
            let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {{
                store::TextError::new(format!("invalid hex: {{e}}"), dsl::TextSpan::at(1, 1))
            }})?;
            bytes.push(byte);
            i += 2;
        }}
        let vertices = las_vertices_from_bytes(&bytes).map_err(|e| {{
            store::TextError::new(e, dsl::TextSpan::at(1, 1))
        }})?;
        Ok(Self {{ schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), vertices }})
    }}
    fn print_dsl(&self) -> String {{
        let body: String = las_bytes_from_vertices(&self.vertices).iter().map(|b| format!("{{b:02x}}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }}
}}

impl store::DocumentPack for LasSnapshot {{
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {{
        let _ = options;
        let raw = las_bytes_from_vertices(&self.vertices);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }}
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {{
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {{
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {{}}, got {{}}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }}
        let _ = options;
        let vertices = las_vertices_from_bytes(&inner).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self {{ schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), vertices }})
    }}
}}
//#endregion 🔖️LasBinaryCodec
'''


def gltf_snapshot() -> str:
    return f'''//! 🧬️ GltfSnapshot schema — persistent fields + real codecs.

use crate::artifacts::gltf::STDIO_GLTF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};
{MESH_MODEL}
//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.gltf")]
pub struct GltfSnapshot {{
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<MeshVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub document: serde_json::Value,
}}

impl Default for GltfSnapshot {{
    fn default() -> Self {{
        Self {{
            schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(),
            vertices: Vec::new(),
            document: serde_json::Value::Null,
        }}
    }}
}}
//#endregion 🔖️Snapshot

//#region 🔖️GltfJsonCodec
{GLTF_CODEC}

impl store::DocumentDsl for GltfSnapshot {{
    const EXTENSION: &'static str = "gltf";
    fn envelope_id() -> &'static str {{ "stdio.gltf" }}

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {{
        let body = match store::semio_format::split_text_preamble(text) {{
            Ok((_, rest)) => rest,
            Err(_) => text,
        }};
        let document = serde_json::from_str(body.trim()).map_err(|e| {{
            store::TextError::new(format!("gltf json: {{e}}"), dsl::TextSpan::at(1, 1))
        }})?;
        let vertices = gltf_vertices_from_value(&document).map_err(|e| {{
            store::TextError::new(e, dsl::TextSpan::at(1, 1))
        }})?;
        Ok(Self {{ schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), vertices, document }})
    }}
    fn print_dsl(&self) -> String {{
        let doc = if self.document.is_null() {{
            gltf_value_from_vertices(&self.vertices)
        }} else {{
            self.document.clone()
        }};
        let body = serde_json::to_string_pretty(&doc).unwrap_or_else(|_| "{{}}".into());
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }}
}}

impl store::DocumentPack for GltfSnapshot {{
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {{
        let _ = options;
        let doc = if self.document.is_null() {{
            gltf_value_from_vertices(&self.vertices)
        }} else {{
            self.document.clone()
        }};
        let raw = serde_json::to_vec(&doc).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }}
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {{
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {{
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {{}}, got {{}}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }}
        let _ = options;
        let document = serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let vertices = gltf_vertices_from_value(&document).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self {{ schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), vertices, document }})
    }}
}}
//#endregion 🔖️GltfJsonCodec
'''


def artifact_schema_rs(mid: str, name: str, fields_rs: str, field_names: list[str]) -> str:
    return f'''//! 🧬️ {name}Artifact schema — full artifact state.

use crate::artifacts::{mid}::{name}Snapshot;
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.{mid}")]
pub struct {name}Artifact {{
    #[state(persistent)]
    pub schema: String,
{fields_rs}
}}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for {name}Artifact {{
    fn default() -> Self {{
        Self::from_snapshot({name}Snapshot::default())
    }}
}}

impl {name}Artifact {{
    pub fn to_snapshot(&self) -> {name}Snapshot {{
        {name}Snapshot {{
            schema: self.schema.clone(),
{"".join(f"            {f}: self.{f}.clone()," for f in field_names)}
        }}
    }}

    pub fn from_snapshot(snapshot: {name}Snapshot) -> Self {{
        Self {{
            schema: snapshot.schema,
{"".join(f"            {f}: snapshot.{f}," for f in field_names)}
        }}
    }}

    pub fn set_snapshot(&mut self, snapshot: {name}Snapshot) {{
        self.schema = snapshot.schema;
{"".join(f"        self.{f} = snapshot.{f};" for f in field_names)}
    }}
}}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn {mid}_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {{
    schema::ArtifactSchemaDescriptor {{
        id: "s.stdio.{mid}",
        artifact: schema::FacetLeaves {{
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        }},
        snapshot: schema::FacetLeaves {{
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        }},
        diff: schema::FacetLeaves {{
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        }},
    }}
}}
//#endregion 🔖️Descriptor
'''


def io_txt_deser(mid: str, name: str, body: str) -> str:
    doc = f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"
    return f'''//! 📥️ Deserialize `stdio.{mid}` from stdio.txt.
use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::{mid}::{{{name}Snapshot, {doc}}};
pub fn register() {{}}
pub fn deserialize(from: &TxtSnapshot) -> Result<{name}Snapshot, store::TextError> {{
{body}
}}
pub fn deserialize_text(text: &str) -> Result<{name}Snapshot, store::TextError> {{
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}}
'''


def io_txt_ser(mid: str, name: str, body: str) -> str:
    return f'''//! 📤️ Serialize `stdio.{mid}` to stdio.txt.
use crate::artifacts::txt::{{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA}};
use crate::artifacts::{mid}::{name}Snapshot;
pub fn register() {{}}
pub fn serialize(from: &{name}Snapshot) -> Result<TxtSnapshot, store::PackError> {{
{body}
}}
pub fn serialize_text(from: &{name}Snapshot) -> Result<String, store::PackError> {{
    Ok(store::DocumentDsl::print_dsl(&serialize(from)?))
}}
'''


def io_bin_deser(mid: str, name: str, body: str) -> str:
    doc = f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"
    return f'''//! 📥️ Deserialize `stdio.{mid}` from stdio.binary.
use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::{mid}::{{{name}Snapshot, {doc}}};
pub fn register() {{}}
pub fn deserialize(from: &BinarySnapshot) -> Result<{name}Snapshot, store::PackError> {{
{body}
}}
'''


def io_bin_ser(mid: str, name: str, body: str) -> str:
    return f'''//! 📤️ Serialize `stdio.{mid}` to stdio.binary.
use crate::artifacts::binary::{{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA}};
use crate::artifacts::{mid}::{name}Snapshot;
pub fn register() {{}}
pub fn serialize(from: &{name}Snapshot) -> Result<BinarySnapshot, store::PackError> {{
{body}
}}
'''


def io_json_deser(mid: str, name: str, body: str) -> str:
    doc = f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"
    return f'''//! 📥️ Deserialize `stdio.{mid}` from stdio.json.
use crate::artifacts::json::JsonSnapshot;
use crate::artifacts::{mid}::{{{name}Snapshot, {doc}}};
pub fn register() {{}}
pub fn deserialize(from: &JsonSnapshot) -> Result<{name}Snapshot, store::TextError> {{
{body}
}}
pub fn deserialize_text(text: &str) -> Result<{name}Snapshot, store::TextError> {{
    deserialize(&<JsonSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}}
'''


def io_json_ser(mid: str, name: str, body: str) -> str:
    return f'''//! 📤️ Serialize `stdio.{mid}` to stdio.json.
use crate::artifacts::json::{{JsonSnapshot, STDIO_JSON_DOCUMENT_SCHEMA}};
use crate::artifacts::{mid}::{name}Snapshot;
pub fn register() {{}}
pub fn serialize(from: &{name}Snapshot) -> Result<JsonSnapshot, store::PackError> {{
{body}
}}
'''


def fix_mutations(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    for bad in ("serde_step::", "serde_ifc::", "serde_las::", "serde_gltf::"):
        text = text.replace(bad, "serde_json::")
    path.write_text(text, encoding="utf-8")


def main() -> None:
    step_base = PLUGIN / "🗿️artifacts" / ROSTER["step"]["dir"]
    ifc_base = PLUGIN / "🗿️artifacts" / ROSTER["ifc"]["dir"]
    las_base = PLUGIN / "🗿️artifacts" / ROSTER["las"]["dir"]
    gltf_base = PLUGIN / "🗿️artifacts" / ROSTER["gltf"]["dir"]

    (step_base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(
        brep_snapshot("step", "STDIO_STEP_DOCUMENT_SCHEMA", "step", "stdio.step", "step_brep_from_text", "step_brep_to_text", STEP_CODEC),
        encoding="utf-8",
    )
    (ifc_base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(
        brep_snapshot("ifc", "STDIO_IFC_DOCUMENT_SCHEMA", "ifc", "stdio.ifc", "ifc_brep_from_text", "ifc_brep_to_text", IFC_CODEC),
        encoding="utf-8",
    )
    (las_base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(las_snapshot(), encoding="utf-8")
    (gltf_base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(gltf_snapshot(), encoding="utf-8")

    for mid, name, base, fields, fnames in [
        ("step", "Step", step_base, "    #[serde(default)]\n    pub brep: crate::artifacts::step::schema::snapshot::BrepMesh,", ["brep"]),
        ("ifc", "Ifc", ifc_base, "    #[serde(default)]\n    pub brep: crate::artifacts::ifc::schema::snapshot::BrepMesh,", ["brep"]),
        ("las", "Las", las_base, "    #[serde(default)]\n    pub vertices: Vec<crate::artifacts::las::schema::snapshot::MeshVertex>,", ["vertices"]),
        ("gltf", "Gltf", gltf_base, "    #[serde(default)]\n    pub vertices: Vec<crate::artifacts::gltf::schema::snapshot::MeshVertex>,\n    #[serde(default)]\n    pub document: serde_json::Value,", ["vertices", "document"]),
    ]:
        (base / "🧬️schema/🦀️component.rs").write_text(
            artifact_schema_rs(mid, name, f"    #[state(persistent)]\n{fields}", fnames),
            encoding="utf-8",
        )
        fix_mutations(base / "🧬️schema/🧬️mutations/🦀️component.rs")

    step_deser = (
        "    let brep = crate::artifacts::step::schema::snapshot::step_brep_from_text(from.text.trim()).map_err(|e| {\n"
        "        store::TextError::new(format!(\"step parse: {e}\"), dsl::TextSpan::at(1, 1))\n"
        "    })?;\n"
        "    Ok(StepSnapshot { schema: STDIO_STEP_DOCUMENT_SCHEMA.into(), brep })"
    )
    step_ser = (
        "    let text = crate::artifacts::step::schema::snapshot::step_brep_to_text(&from.brep);\n"
        "    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })"
    )
    (step_base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
        io_txt_deser("step", "Step", step_deser), encoding="utf-8"
    )
    (step_base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
        io_txt_ser("step", "Step", step_ser), encoding="utf-8"
    )

    ifc_deser = (
        "    let brep = crate::artifacts::ifc::schema::snapshot::ifc_brep_from_text(from.text.trim()).map_err(|e| {\n"
        "        store::TextError::new(format!(\"ifc parse: {e}\"), dsl::TextSpan::at(1, 1))\n"
        "    })?;\n"
        "    Ok(IfcSnapshot { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), brep })"
    )
    ifc_ser = (
        "    let text = crate::artifacts::ifc::schema::snapshot::ifc_brep_to_text(&from.brep);\n"
        "    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })"
    )
    (ifc_base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
        io_txt_deser("ifc", "Ifc", ifc_deser), encoding="utf-8"
    )
    (ifc_base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
        io_txt_ser("ifc", "Ifc", ifc_ser), encoding="utf-8"
    )

    las_deser = (
        "    let vertices = crate::artifacts::las::schema::snapshot::las_vertices_from_bytes(&from.bytes)\n"
        "        .map_err(|e| store::PackError::Schema(e))?;\n"
        "    Ok(LasSnapshot { schema: STDIO_LAS_DOCUMENT_SCHEMA.into(), vertices })"
    )
    las_ser = (
        "    let bytes = crate::artifacts::las::schema::snapshot::las_bytes_from_vertices(&from.vertices);\n"
        "    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })"
    )
    (las_base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🦀️component.rs").write_text(
        io_bin_deser("las", "Las", las_deser), encoding="utf-8"
    )
    (las_base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🦀️component.rs").write_text(
        io_bin_ser("las", "Las", las_ser), encoding="utf-8"
    )

    gltf_deser = (
        "    let vertices = crate::artifacts::gltf::schema::snapshot::gltf_vertices_from_value(&from.value).map_err(|e| {\n"
        "        store::TextError::new(e, dsl::TextSpan::at(1, 1))\n"
        "    })?;\n"
        "    Ok(GltfSnapshot { schema: STDIO_GLTF_DOCUMENT_SCHEMA.into(), vertices, document: from.value.clone() })"
    )
    gltf_ser = (
        "    let value = if from.document.is_null() {\n"
        "        crate::artifacts::gltf::schema::snapshot::gltf_value_from_vertices(&from.vertices)\n"
        "    } else {\n"
        "        from.document.clone()\n"
        "    };\n"
        "    Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })"
    )
    import shutil
    txt_json_imp = gltf_base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt"
    if txt_json_imp.exists():
        shutil.rmtree(txt_json_imp)
    txt_json_exp = gltf_base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt"
    if txt_json_exp.exists():
        shutil.rmtree(txt_json_exp)
    for rel in (
        "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json",
        "🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json",
    ):
        (gltf_base / rel).mkdir(parents=True, exist_ok=True)
    (gltf_base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🦀️component.rs").write_text(
        io_json_deser("gltf", "Gltf", gltf_deser), encoding="utf-8"
    )
    (gltf_base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🦀️component.rs").write_text(
        io_json_ser("gltf", "Gltf", gltf_ser), encoding="utf-8"
    )

    gltf_io = gltf_base / "🚪️io/🦀️component.rs"
    gltf_io.write_text(
        "//! IO stdio.gltf\npub fn register() {\n"
        "    crate::artifacts::gltf::io::import::deserializers::artifacts::json::register();\n"
        "    crate::artifacts::gltf::io::export::serializers::artifacts::json::register();\n"
        "}\n",
        encoding="utf-8",
    )

    step_io = step_base / "🚪️io/🦀️component.rs"
    step_io.write_text(
        "//! IO stdio.step\npub fn register() {\n"
        "    crate::artifacts::step::io::import::deserializers::artifacts::txt::register();\n"
        "    crate::artifacts::step::io::export::serializers::artifacts::txt::register();\n"
        "}\n",
        encoding="utf-8",
    )
    ifc_io = ifc_base / "🚪️io/🦀️component.rs"
    ifc_io.write_text(
        "//! IO stdio.ifc\npub fn register() {\n"
        "    crate::artifacts::ifc::io::import::deserializers::artifacts::txt::register();\n"
        "    crate::artifacts::ifc::io::export::serializers::artifacts::txt::register();\n"
        "}\n",
        encoding="utf-8",
    )

    # Examples
    (step_base / "📚️examples/🎬️demo/🖼️assets/example.step").write_text(
        "ISO-10303-21;\nHEADER;FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));ENDSEC;\nDATA;\n"
        "#1=CARTESIAN_POINT('',(0.0,0.0,0.0));\n#2=CARTESIAN_POINT('',(1.0,0.0,0.0));\n"
        "#3=CARTESIAN_POINT('',(0.0,1.0,0.0));\n#4=POLY_LOOP('',(#1,#2,#3));\nENDSEC;\nEND-ISO-10303-21;\n",
        encoding="utf-8",
    )
    (ifc_base / "📚️examples/🎬️demo/🖼️assets/example.ifc").write_text(
        "ISO-10303-21;\nHEADER;FILE_SCHEMA(('IFC4'));ENDSEC;\nDATA;\n"
        "#1=IFCCARTESIANPOINT((0.,0.,0.));\n#2=IFCCARTESIANPOINT((1.,0.,0.));\n"
        "#3=IFCCARTESIANPOINT((0.,1.,0.));\n#4=IFCPOLYLOOP((#1,#2,#3));\nENDSEC;\nEND-ISO-10303-21;\n",
        encoding="utf-8",
    )
    def write_las_example(path: Path) -> None:
        verts = [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)]
        header_size = 227
        record_len = 20
        out = bytearray(header_size + len(verts) * record_len)
        out[0:4] = b"LASF"
        out[96:100] = (header_size).to_bytes(4, "little")
        out[104] = 0
        out[105:107] = record_len.to_bytes(2, "little")
        out[107:111] = len(verts).to_bytes(4, "little")
        import struct
        x_scale = 0.01
        out[131:139] = struct.pack("<d", x_scale)
        out[139:147] = struct.pack("<d", x_scale)
        out[147:155] = struct.pack("<d", x_scale)
        pos = header_size
        for x, y, z in verts:
            xi = int(round(x / x_scale))
            yi = int(round(y / x_scale))
            zi = int(round(z / x_scale))
            out[pos : pos + 4] = struct.pack("<i", xi)
            out[pos + 4 : pos + 8] = struct.pack("<i", yi)
            out[pos + 8 : pos + 12] = struct.pack("<i", zi)
            pos += record_len
        path.write_bytes(bytes(out))

    write_las_example(las_base / "📚️examples/🎬️demo/🖼️assets/example.las")

    gltf_doc = {
        "asset": {"version": "2.0"},
        "buffers": [{
            "byteLength": 36,
            "uri": "data:application/octet-stream;base64,AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        }],
        "bufferViews": [{"buffer": 0, "byteOffset": 0, "byteLength": 36}],
        "accessors": [{
            "bufferView": 0,
            "componentType": 5126,
            "count": 3,
            "type": "VEC3",
            "min": [0, 0, 0],
            "max": [1, 1, 1],
        }],
        "meshes": [{"primitives": [{"attributes": {"POSITION": 0}}]}],
    }
    import base64, struct, json
    bin_data = b"".join(struct.pack("<fff", *v) for v in [(0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)])
    gltf_doc["buffers"][0]["uri"] = "data:application/octet-stream;base64," + base64.standard_b64encode(bin_data).decode()
    (gltf_base / "📚️examples/🎬️demo/🖼️assets/example.gltf").write_text(
        json.dumps(gltf_doc, indent=2) + "\n", encoding="utf-8"
    )

    print("fixed codecs")


if __name__ == "__main__":
    main()
