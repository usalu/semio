#!/usr/bin/env python3
"""W4b: real codecs + IO bridges for obj/stl/ply/dxf/svg/bmp."""
from __future__ import annotations

import importlib.util
import json
import re
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
W4A = TICKET / "generators/w4a_fix_codecs.py"
spec = importlib.util.spec_from_file_location("w4a_fix", W4A)
w4a = importlib.util.module_from_spec(spec)
spec.loader.exec_module(w4a)

TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]

SCHEMA_CONST = {
    "obj": "OBJ",
    "ply": "PLY",
    "stl": "STL",
    "dxf": "DXF",
    "svg": "SVG",
    "bmp": "BMP",
}

MESH_TYPES = r'''
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeshTriangle {
    pub i0: u32,
    pub i1: u32,
    pub i2: u32,
}
'''

MESH_PARSE_HELPERS = r'''
fn parse_face_indices(token: &str) -> Result<Vec<u32>, String> {
    let mut out = Vec::new();
    for part in token.split('/') {
        let idx = part.trim();
        if idx.is_empty() {
            continue;
        }
        let n: i32 = idx.parse().map_err(|e| e.to_string())?;
        let u = if n > 0 { n as u32 - 1 } else { 0 };
        out.push(u);
        break;
    }
    Ok(out)
}

fn triangulate_face(indices: &[u32]) -> Vec<MeshTriangle> {
    if indices.len() < 3 {
        return Vec::new();
    }
    let mut tris = Vec::new();
    for i in 1..indices.len() - 1 {
        tris.push(MeshTriangle { i0: indices[0], i1: indices[i], i2: indices[i + 1] });
    }
    tris
}
'''


def mesh_snapshot(mid: str, name: str, parse_fn: str, write_fn: str) -> str:
    doc = f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"
    return f'''//! 🧬️ {name}Snapshot schema — persistent fields + real codecs.

use crate::artifacts::{mid}::{doc};
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region 🔖️MeshModel
{MESH_TYPES}
//#endregion 🔖️MeshModel

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
    pub vertices: Vec<MeshVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<MeshTriangle>,
}}

impl Default for {name}Snapshot {{
    fn default() -> Self {{
        Self {{
            schema: {doc}.into(),
            vertices: Vec::new(),
            faces: Vec::new(),
        }}
    }}
}}
//#endregion 🔖️Snapshot

//#region 🔖️FormatCodec
{MESH_PARSE_HELPERS}

{parse_fn}

{write_fn}
//#endregion 🔖️FormatCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for {name}Snapshot {{
    const EXTENSION: &'static str = "{mid}";
    fn envelope_id() -> &'static str {{ "stdio.{mid}" }}

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {{
        let body = match store::semio_format::split_text_preamble(text) {{
            Ok((_, rest)) => rest,
            Err(_) => text,
        }};
        let (vertices, faces) = parse_{mid}_text(body).map_err(|e| {{
            store::TextError::new(format!("{mid} parse: {{e}}"), dsl::TextSpan::at(1, 1))
        }})?;
        Ok(Self {{ schema: {doc}.into(), vertices, faces }})
    }}
    fn print_dsl(&self) -> String {{
        let body = write_{mid}_text(&self.vertices, &self.faces);
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
        let raw = write_{mid}_text(&self.vertices, &self.faces).into_bytes();
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
        let (vertices, faces) = parse_{mid}_text(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self {{ schema: {doc}.into(), vertices, faces }})
    }}
}}
//#endregion 🔖️HandcraftedDocumentCodecs
'''

OBJ_PARSE = r'''
pub fn parse_obj_text(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("v") => {
                let x: f32 = parts.next().ok_or("v x")?.parse().map_err(|e| e.to_string())?;
                let y: f32 = parts.next().ok_or("v y")?.parse().map_err(|e| e.to_string())?;
                let z: f32 = parts.next().ok_or("v z")?.parse().map_err(|e| e.to_string())?;
                vertices.push(MeshVertex { x, y, z });
            }
            Some("f") => {
                let tokens: Vec<&str> = parts.collect();
                let mut idxs = Vec::new();
                for t in tokens {
                    let mut got = parse_face_indices(t)?;
                    idxs.append(&mut got);
                }
                faces.extend(triangulate_face(&idxs));
            }
            _ => {}
        }
    }
    Ok((vertices, faces))
}

pub fn write_obj_text(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    let mut out = String::from("# Wavefront OBJ\n");
    for v in vertices {
        out.push_str(&format!("v {} {} {}\n", v.x, v.y, v.z));
    }
    for f in faces {
        out.push_str(&format!("f {} {} {}\n", f.i0 + 1, f.i1 + 1, f.i2 + 1));
    }
    out
}
'''

PLY_PARSE = r'''
pub fn parse_ply_text(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let mut lines = text.lines().peekable();
    if lines.next().map(|l| l.trim()) != Some("ply") {
        return Err("expected ply header".into());
    }
    let mut fmt = String::new();
    let mut vertex_count = 0usize;
    let mut face_count = 0usize;
    loop {
        let line = lines.next().ok_or("unexpected eof in header")?.trim();
        if line == "end_header" {
            break;
        }
        if let Some(rest) = line.strip_prefix("format ") {
            fmt = rest.to_string();
        } else if let Some(rest) = line.strip_prefix("element vertex ") {
            vertex_count = rest.parse().map_err(|e| e.to_string())?;
        } else if let Some(rest) = line.strip_prefix("element face ") {
            face_count = rest.parse().map_err(|e| e.to_string())?;
        }
    }
    if !fmt.starts_with("ascii") {
        return Err("only ascii ply supported".into());
    }
    let mut vertices = Vec::with_capacity(vertex_count);
    for _ in 0..vertex_count {
        let line = lines.next().ok_or("vertex eof")?;
        let mut p = line.split_whitespace();
        let x: f32 = p.next().ok_or("x")?.parse().map_err(|e| e.to_string())?;
        let y: f32 = p.next().ok_or("y")?.parse().map_err(|e| e.to_string())?;
        let z: f32 = p.next().ok_or("z")?.parse().map_err(|e| e.to_string())?;
        vertices.push(MeshVertex { x, y, z });
    }
    let mut faces = Vec::new();
    for _ in 0..face_count {
        let line = lines.next().ok_or("face eof")?;
        let mut p = line.split_whitespace();
        let n: usize = p.next().ok_or("n")?.parse().map_err(|e| e.to_string())?;
        let mut idxs = Vec::with_capacity(n);
        for _ in 0..n {
            idxs.push(p.next().ok_or("idx")?.parse().map_err(|e| e.to_string())?);
        }
        faces.extend(triangulate_face(&idxs));
    }
    Ok((vertices, faces))
}

pub fn write_ply_text(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    let mut out = String::new();
    out.push_str("ply\nformat ascii 1.0\n");
    out.push_str(&format!("element vertex {}\n", vertices.len()));
    out.push_str("property float x\nproperty float y\nproperty float z\n");
    out.push_str(&format!("element face {}\n", faces.len()));
    out.push_str("property list uchar int vertex_indices\nend_header\n");
    for v in vertices {
        out.push_str(&format!("{} {} {}\n", v.x, v.y, v.z));
    }
    for f in faces {
        out.push_str(&format!("3 {} {} {}\n", f.i0, f.i1, f.i2));
    }
    out
}
'''

DXF_SNAPSHOT = r'''//! 🧬️ DxfSnapshot schema — persistent fields + real codecs.

use crate::artifacts::dxf::STDIO_DXF_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DrawingModel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DxfLine {
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub x2: f64,
    pub y2: f64,
    pub z2: f64,
}
//#endregion 🔖️DrawingModel

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub lines: Vec<DxfLine>,
}

impl Default for DxfSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️FormatCodec
pub fn parse_dxf_text(text: &str) -> Result<Vec<DxfLine>, String> {
    let mut lines_out = Vec::new();
    let codes: Vec<&str> = text.lines().collect();
    let mut i = 0usize;
    while i + 1 < codes.len() {
        let code = codes[i].trim();
        let val = codes[i + 1].trim();
        i += 2;
        if code == "0" && val == "LINE" {
            let mut line = DxfLine::default();
            while i + 1 < codes.len() {
                let c = codes[i].trim();
                let v = codes[i + 1].trim();
                if c == "0" {
                    break;
                }
                i += 2;
                match c {
                    "10" => line.x1 = v.parse().map_err(|e| e.to_string())?,
                    "20" => line.y1 = v.parse().map_err(|e| e.to_string())?,
                    "30" => line.z1 = v.parse().map_err(|e| e.to_string())?,
                    "11" => line.x2 = v.parse().map_err(|e| e.to_string())?,
                    "21" => line.y2 = v.parse().map_err(|e| e.to_string())?,
                    "31" => line.z2 = v.parse().map_err(|e| e.to_string())?,
                    _ => {}
                }
            }
            lines_out.push(line);
            continue;
        }
    }
    Ok(lines_out)
}

pub fn write_dxf_text(lines: &[DxfLine]) -> String {
    let mut out = String::from("0\nSECTION\n2\nENTITIES\n");
    for ln in lines {
        out.push_str("0\nLINE\n8\n0\n");
        out.push_str(&format!("10\n{}\n20\n{}\n30\n{}\n", ln.x1, ln.y1, ln.z1));
        out.push_str(&format!("11\n{}\n21\n{}\n31\n{}\n", ln.x2, ln.y2, ln.z2));
    }
    out.push_str("0\nENDSEC\n0\nEOF\n");
    out
}
//#endregion 🔖️FormatCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for DxfSnapshot {
    const EXTENSION: &'static str = "dxf";
    fn envelope_id() -> &'static str { "stdio.dxf" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let lines = parse_dxf_text(body).map_err(|e| store::TextError::new(format!("dxf parse: {e}"), dsl::TextSpan::at(1, 1)))?;
        Ok(Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })
    }
    fn print_dsl(&self) -> String {
        let body = write_dxf_text(&self.lines);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for DxfSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = write_dxf_text(&self.lines).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let lines = parse_dxf_text(&text).map_err(|e| store::PackError::Schema(e))?;
        Ok(Self { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
'''

STL_PARSE = r'''
pub fn parse_stl_ascii(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut tri: [Option<MeshVertex>; 3] = [None, None, None];
    let mut slot = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("vertex ") {
            let coords: Vec<f32> = line
                .trim_start_matches("vertex")
                .split_whitespace()
                .map(|s| s.parse().map_err(|e| e.to_string()))
                .collect::<Result<Vec<_>, _>>()?;
            if coords.len() < 3 {
                return Err("vertex coords".into());
            }
            let v = MeshVertex { x: coords[0], y: coords[1], z: coords[2] };
            tri[slot] = Some(v.clone());
            slot += 1;
            if slot == 3 {
                let i0 = vertices.len() as u32;
                for v in tri.iter().flatten() {
                    vertices.push(v.clone());
                }
                faces.push(MeshTriangle { i0, i1: i0 + 1, i2: i0 + 2 });
                tri = [None, None, None];
                slot = 0;
            }
        }
    }
    Ok((vertices, faces))
}

pub fn write_stl_ascii(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    let mut out = String::from("solid mesh\n");
    for f in faces {
        let a = &vertices[f.i0 as usize];
        let b = &vertices[f.i1 as usize];
        let c = &vertices[f.i2 as usize];
        let ux = b.x - a.x;
        let uy = b.y - a.y;
        let uz = b.z - a.z;
        let vx = c.x - a.x;
        let vy = c.y - a.y;
        let vz = c.z - a.z;
        let nx = uy * vz - uz * vy;
        let ny = uz * vx - ux * vz;
        let nz = ux * vy - uy * vx;
        out.push_str(&format!("  facet normal {} {} {}\n", nx, ny, nz));
        out.push_str("    outer loop\n");
        out.push_str(&format!("      vertex {} {} {}\n", a.x, a.y, a.z));
        out.push_str(&format!("      vertex {} {} {}\n", b.x, b.y, b.z));
        out.push_str(&format!("      vertex {} {} {}\n", c.x, c.y, c.z));
        out.push_str("    endloop\n  endfacet\n");
    }
    out.push_str("endsolid mesh\n");
    out
}

pub fn parse_stl_binary(bytes: &[u8]) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    if bytes.len() < 84 {
        return Err("stl binary too short".into());
    }
    let count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut off = 84usize;
    for _ in 0..count {
        if off + 50 > bytes.len() {
            return Err("stl binary truncated".into());
        }
        off += 12;
        let mut tri_verts = Vec::new();
        for _ in 0..3 {
            let x = f32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
            let y = f32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap());
            let z = f32::from_le_bytes(bytes[off + 8..off + 12].try_into().unwrap());
            off += 12;
            let i = vertices.len() as u32;
            vertices.push(MeshVertex { x, y, z });
            tri_verts.push(i);
        }
        off += 2;
        faces.push(MeshTriangle { i0: tri_verts[0], i1: tri_verts[1], i2: tri_verts[2] });
    }
    Ok((vertices, faces))
}

pub fn write_stl_binary(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> Vec<u8> {
    let mut out = vec![0u8; 84];
    out.extend_from_slice(&(faces.len() as u32).to_le_bytes());
    for f in faces {
        let a = &vertices[f.i0 as usize];
        let b = &vertices[f.i1 as usize];
        let c = &vertices[f.i2 as usize];
        let ux = b.x - a.x;
        let uy = b.y - a.y;
        let uz = b.z - a.z;
        let vx = c.x - a.x;
        let vy = c.y - a.y;
        let vz = c.z - a.z;
        let nx = uy * vz - uz * vy;
        let ny = uz * vx - ux * vz;
        let nz = ux * vy - uy * vx;
        out.extend_from_slice(&nx.to_le_bytes());
        out.extend_from_slice(&ny.to_le_bytes());
        out.extend_from_slice(&nz.to_le_bytes());
        for v in [a, b, c] {
            out.extend_from_slice(&v.x.to_le_bytes());
            out.extend_from_slice(&v.y.to_le_bytes());
            out.extend_from_slice(&v.z.to_le_bytes());
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}
'''

STL_SNAPSHOT_EXTRA = r'''
impl StlSnapshot {
    pub fn parse_stl_text_body(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
        if text.trim_start().starts_with("solid") {
            parse_stl_ascii(text)
        } else {
            parse_stl_binary(text.as_bytes())
        }
    }
}
'''

SVG_SNAPSHOT = r'''//! 🧬️ SvgSnapshot schema — persistent fields + real codecs.

use crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA;
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlDocument, XmlNode};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg")]
pub struct SvgSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub doc: XmlDocument,
}

impl Default for SvgSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc: XmlDocument::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SvgCodec
pub fn parse_svg_xml(text: &str) -> Result<XmlDocument, String> {
    let doc = xml_document_from_text(text)?;
    if let Some(XmlNode::Element { name, .. }) = &doc.root {
        if name != "svg" && !name.ends_with(":svg") {
            return Err("root element must be svg".into());
        }
    } else {
        return Err("svg document requires root element".into());
    }
    Ok(doc)
}

pub fn write_svg_xml(doc: &XmlDocument) -> String {
    xml_document_to_text(doc)
}
//#endregion 🔖️SvgCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for SvgSnapshot {
    const EXTENSION: &'static str = "svg";
    fn envelope_id() -> &'static str { "stdio.svg" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let doc = parse_svg_xml(body).map_err(|e| store::TextError::new(format!("svg parse: {e}"), dsl::TextSpan::at(1, 1)))?;
        Ok(Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })
    }
    fn print_dsl(&self) -> String {
        let body = write_svg_xml(&self.doc);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for SvgSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(&self.doc).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let doc = serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
'''

BMP_SNAPSHOT = r'''//! 🧬️ BmpSnapshot schema — persistent fields + real codecs.

use crate::artifacts::bmp::STDIO_BMP_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bmp")]
pub struct BmpSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub width: u32,
    #[state(persistent)]
    pub height: u32,
    #[state(persistent)]
    #[serde(default)]
    pub pixels: Vec<u8>,
}

impl Default for BmpSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), width: 0, height: 0, pixels: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️BmpCodec
pub fn decode_bmp(bytes: &[u8]) -> Result<BmpSnapshot, String> {
    if bytes.len() < 54 || &bytes[0..2] != b"BM" {
        return Err("invalid BMP signature".into());
    }
    let data_off = u32::from_le_bytes(bytes[10..14].try_into().unwrap()) as usize;
    let width = i32::from_le_bytes(bytes[18..22].try_into().unwrap());
    let height = i32::from_le_bytes(bytes[22..26].try_into().unwrap()).unsigned_abs();
    let bpp = u16::from_le_bytes(bytes[28..30].try_into().unwrap());
    if bpp != 24 {
        return Err("only 24-bit BMP supported".into());
    }
    let w = width as u32;
    let h = height as u32;
    let row = ((w * 3 + 3) / 4) * 4;
    let mut pixels = vec![0u8; (w * h * 3) as usize];
    let mut off = data_off;
    for y in 0..h {
        let row_start = off;
        for x in 0..w {
            let i = ((h - 1 - y) * w + x) as usize * 3;
            if row_start + (x as usize) * 3 + 2 >= bytes.len() {
                return Err("bmp pixel overrun".into());
            }
            let b = bytes[row_start + (x as usize) * 3];
            let g = bytes[row_start + (x as usize) * 3 + 1];
            let r = bytes[row_start + (x as usize) * 3 + 2];
            pixels[i] = r;
            pixels[i + 1] = g;
            pixels[i + 2] = b;
        }
        off += row as usize;
    }
    Ok(BmpSnapshot { schema: STDIO_BMP_DOCUMENT_SCHEMA.into(), width: w, height: h, pixels })
}

pub fn encode_bmp(snap: &BmpSnapshot) -> Result<Vec<u8>, String> {
    if snap.width == 0 || snap.height == 0 {
        return Err("empty image".into());
    }
    let w = snap.width;
    let h = snap.height;
    let row = ((w * 3 + 3) / 4) * 4;
    let pixel_bytes = (row * h) as usize;
    let file_size = 54 + pixel_bytes;
    let mut out = Vec::with_capacity(file_size);
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&(file_size as u32).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    out.extend_from_slice(&(-(h as i32)).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(pixel_bytes as u32).to_le_bytes());
    out.extend_from_slice(&[0u8; 16]);
    for y in 0..h {
        let mut row_buf = vec![0u8; row as usize];
        for x in 0..w {
            let i = ((y * w + x) * 3) as usize;
            if i + 2 >= snap.pixels.len() {
                return Err("pixel buffer short".into());
            }
            let o = (x as usize) * 3;
            row_buf[o] = snap.pixels[i + 2];
            row_buf[o + 1] = snap.pixels[i + 1];
            row_buf[o + 2] = snap.pixels[i];
        }
        out.extend_from_slice(&row_buf);
    }
    Ok(out)
}
//#endregion 🔖️BmpCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for BmpSnapshot {
    const EXTENSION: &'static str = "bmp";
    fn envelope_id() -> &'static str { "stdio.bmp" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i + 1 < hex.len() {
            bytes.push(u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| {
                store::TextError::new(format!("hex: {e}"), dsl::TextSpan::at(1, 1))
            })?);
            i += 2;
        }
        decode_bmp(&bytes).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let raw = encode_bmp(self).unwrap_or_default();
        let body: String = raw.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for BmpSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_bmp(self).map_err(|e| store::PackError::Schema(e))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_bmp(&inner).map_err(|e| store::PackError::Schema(e))
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
'''


def bulk_fix_rs(base: Path, mid: str) -> None:
    doc = f"STDIO_{SCHEMA_CONST[mid]}_DOCUMENT_SCHEMA"
    for p in base.rglob("*.rs"):
        t = p.read_text(encoding="utf-8")
        orig = t
        t = t.replace("OpBmp", "OpBinary")
        t = t.replace("OpObj", "OpBinary")
        t = t.replace("OpStl", "OpBinary")
        t = t.replace("OpPly", "OpBinary")
        t = t.replace("OpDxf", "OpBinary")
        t = t.replace("OpSvg", "OpBinary")
        t = t.replace("from_bmp", "from_binary")
        t = t.replace("from_obj", "from_binary")
        t = t.replace("from_stl", "from_binary")
        t = t.replace("wrap_bmp", "wrap_binary")
        t = t.replace("unwrap_bmp", "unwrap_binary")
        t = t.replace("DecomposeSource::Bmp", "DecomposeSource::Binary")
        t = t.replace("schema::snapshot::bmp::", "schema::snapshot::binary::")
        t = re.sub(r"serde_\w+::", "serde_json::", t)
        t = t.replace("STDIO_JSON_DOCUMENT_SCHEMA", doc)
        if t != orig:
            p.write_text(t, encoding="utf-8")


def fix_mesh_artifact(mid: str, name: str, emoji: str, parse_write: str) -> None:
    base = PLUGIN / "🗿️artifacts" / emoji
    bulk_fix_rs(base, mid)
    snap_path = base / "🧬️schema/📸️snapshot/🦀️component.rs"
    parts = parse_write.split("\n\n", 1)
    parse_fn = parts[0]
    write_fn = parts[1] if len(parts) > 1 else ""
    snap_path.write_text(mesh_snapshot(mid, name, parse_fn, write_fn), encoding="utf-8")
    fields = (
        "    #[state(persistent)]\n    #[serde(default)]\n    pub vertices: Vec<crate::artifacts::"
        f"{mid}::schema::snapshot::MeshVertex>,\n"
        "    #[state(persistent)]\n    #[serde(default)]\n    pub faces: Vec<crate::artifacts::"
        f"{mid}::schema::snapshot::MeshTriangle>,"
    )
    field_names = ["vertices", "faces"]
    (base / "🧬️schema/🦀️component.rs").write_text(
        w4a.artifact_schema_rs(mid, name, fields, field_names), encoding="utf-8"
    )
    parse_body = (
        f"    let (vertices, faces) = crate::artifacts::{mid}::schema::snapshot::parse_{mid}_text(from.text.as_str())\n"
        f"        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;\n"
        f"    Ok({name}Snapshot {{ schema: STDIO_{mid.upper()}_DOCUMENT_SCHEMA.into(), vertices, faces }})"
    )
    ser_body = (
        f"    let text = crate::artifacts::{mid}::schema::snapshot::write_{mid}_text(&from.vertices, &from.faces);\n"
        f"    Ok(TxtSnapshot {{ schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text }})"
    )
    (base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
        w4a.io_deser_rs(mid, name, parse_body), encoding="utf-8"
    )
    (base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
        w4a.io_ser_rs(mid, name, ser_body), encoding="utf-8"
    )
    w4a.fix_mutations(base / "🧬️schema/🧬️mutations/🦀️component.rs")
    w4a.fix_graphql_ts(base, "vertices: String!", "vertices: unknown[]; faces: unknown[];")


def fix_stl() -> None:
    mid, name, emoji = "stl", "Stl", ROSTER["stl"]["dir"]
    stl_write = r'''
pub fn parse_stl_text(text: &str) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    parse_stl_ascii(text)
}

pub fn write_stl_text(vertices: &[MeshVertex], faces: &[MeshTriangle]) -> String {
    write_stl_ascii(vertices, faces)
}
'''
    fix_mesh_artifact(mid, name, emoji, STL_PARSE + stl_write)
    base = PLUGIN / "🗿️artifacts" / emoji
    snap = (base / "🧬️schema/📸️snapshot/🦀️component.rs").read_text(encoding="utf-8")
    snap = snap.replace(
        "parse_stl_text(body).map_err",
        "parse_stl_text(body).map_err",
    )
    extra = r'''

pub fn parse_stl_bytes(bytes: &[u8]) -> Result<(Vec<MeshVertex>, Vec<MeshTriangle>), String> {
    if bytes.len() >= 5 && &bytes[0..5] == b"solid".as_ref() {
        parse_stl_ascii(std::str::from_utf8(bytes).map_err(|e| e.to_string())?)
    } else {
        parse_stl_binary(bytes)
    }
}
'''
    snap = snap.replace("//#endregion 🔖️FormatCodec", extra + "\n//#endregion 🔖️FormatCodec")
    (base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(snap, encoding="utf-8")
    bin_parse = (
        "    let (vertices, faces) = crate::artifacts::stl::schema::snapshot::parse_stl_bytes(&from.bytes)\n"
        "        .map_err(|e| store::PackError::Schema(e))?;\n"
        "    Ok(StlSnapshot { schema: STDIO_STL_DOCUMENT_SCHEMA.into(), vertices, faces })"
    )
    bin_ser = (
        "    let bytes = crate::artifacts::stl::schema::snapshot::write_stl_binary(&from.vertices, &from.faces);\n"
        "    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })"
    )
    (base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🦀️component.rs").write_text(
        f'''//! 📥️ Deserialize `stdio.stl` from stdio.binary.

use crate::artifacts::binary::{{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA}};
use crate::artifacts::stl::{{StlSnapshot, STDIO_STL_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &BinarySnapshot) -> Result<StlSnapshot, store::PackError> {{
{bin_parse}
}}
''',
        encoding="utf-8",
    )
    (base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🦀️component.rs").write_text(
        f'''//! 📤️ Serialize `stdio.stl` to stdio.binary.

use crate::artifacts::binary::{{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA}};
use crate::artifacts::stl::StlSnapshot;

pub fn register() {{}}

pub fn serialize(from: &StlSnapshot) -> Result<BinarySnapshot, store::PackError> {{
{bin_ser}
}}
''',
        encoding="utf-8",
    )


def fix_dxf() -> None:
    mid, name, emoji = "dxf", "Dxf", ROSTER["dxf"]["dir"]
    base = PLUGIN / "🗿️artifacts" / emoji
    bulk_fix_rs(base, mid)
    (base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(DXF_SNAPSHOT, encoding="utf-8")
    fields = "    #[state(persistent)]\n    #[serde(default)]\n    pub lines: Vec<crate::artifacts::dxf::schema::snapshot::DxfLine>,"
    (base / "🧬️schema/🦀️component.rs").write_text(
        w4a.artifact_schema_rs(mid, name, fields, ["lines"]), encoding="utf-8"
    )
    parse_body = (
        "    let lines = crate::artifacts::dxf::schema::snapshot::parse_dxf_text(from.text.as_str())\n"
        "        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;\n"
        "    Ok(DxfSnapshot { schema: STDIO_DXF_DOCUMENT_SCHEMA.into(), lines })"
    )
    ser_body = (
        "    let text = crate::artifacts::dxf::schema::snapshot::write_dxf_text(&from.lines);\n"
        "    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })"
    )
    (base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
        w4a.io_deser_rs(mid, name, parse_body), encoding="utf-8"
    )
    (base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
        w4a.io_ser_rs(mid, name, ser_body), encoding="utf-8"
    )
    w4a.fix_mutations(base / "🧬️schema/🧬️mutations/🦀️component.rs")
    w4a.fix_graphql_ts(base, "lines: String!", "lines: unknown[];")


def fix_svg() -> None:
    mid, name, emoji = "svg", "Svg", ROSTER["svg"]["dir"]
    base = PLUGIN / "🗿️artifacts" / emoji
    bulk_fix_rs(base, mid)
    (base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(SVG_SNAPSHOT, encoding="utf-8")
    fields = (
        "    #[state(persistent)]\n    #[serde(default)]\n"
        "    pub doc: crate::artifacts::xml::schema::snapshot::XmlDocument,"
    )
    (base / "🧬️schema/🦀️component.rs").write_text(
        w4a.artifact_schema_rs(mid, name, fields, ["doc"]), encoding="utf-8"
    )
    parse_body = (
        "    let text = crate::artifacts::xml::schema::snapshot::xml_document_to_text(&from.doc);\n"
        "    let doc = crate::artifacts::svg::schema::snapshot::parse_svg_xml(&text)\n"
        "        .map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))?;\n"
        "    Ok(SvgSnapshot { schema: STDIO_SVG_DOCUMENT_SCHEMA.into(), doc })"
    )
    ser_body = (
        "    let text = crate::artifacts::svg::schema::snapshot::write_svg_xml(&from.doc);\n"
        "    let doc = crate::artifacts::xml::schema::snapshot::xml_document_from_text(&text)\n"
        "        .map_err(|e| store::PackError::Schema(e))?;\n"
        "    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })"
    )
    (base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📰xml/🦀️component.rs").write_text(
        f'''//! 📥️ Deserialize `stdio.svg` from stdio.xml.

use crate::artifacts::xml::{{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA}};
use crate::artifacts::svg::{{SvgSnapshot, STDIO_SVG_DOCUMENT_SCHEMA}};

pub fn register() {{}}

pub fn deserialize(from: &XmlSnapshot) -> Result<SvgSnapshot, store::TextError> {{
{parse_body}
}}
''',
        encoding="utf-8",
    )
    (base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📰xml/🦀️component.rs").write_text(
        f'''//! 📤️ Serialize `stdio.svg` to stdio.xml.

use crate::artifacts::xml::{{XmlSnapshot, STDIO_XML_DOCUMENT_SCHEMA}};
use crate::artifacts::svg::SvgSnapshot;

pub fn register() {{}}

pub fn serialize(from: &SvgSnapshot) -> Result<XmlSnapshot, store::PackError> {{
{ser_body}
}}
''',
        encoding="utf-8",
    )
    w4a.fix_mutations(base / "🧬️schema/🧬️mutations/🦀️component.rs")
    w4a.fix_graphql_ts(base, "doc: String!", "doc: unknown;")


def fix_bmp() -> None:
    mid, name, emoji = "bmp", "Bmp", ROSTER["bmp"]["dir"]
    base = PLUGIN / "🗿️artifacts" / emoji
    bulk_fix_rs(base, mid)
    (base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(BMP_SNAPSHOT, encoding="utf-8")
    fields = (
        "    #[state(persistent)]\n    pub width: u32,\n"
        "    #[state(persistent)]\n    pub height: u32,\n"
        "    #[state(persistent)]\n    #[serde(default)]\n    pub pixels: Vec<u8>,"
    )
    (base / "🧬️schema/🦀️component.rs").write_text(
        w4a.artifact_schema_rs(mid, name, fields, ["width", "height", "pixels"]), encoding="utf-8"
    )
    (base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/💾️binary/🦀️component.rs").write_text(
        '''//! 📥️ Deserialize `stdio.bmp` from stdio.binary.

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::bmp::{BmpSnapshot, STDIO_BMP_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &BinarySnapshot) -> Result<BmpSnapshot, store::PackError> {
    crate::artifacts::bmp::schema::snapshot::decode_bmp(&from.bytes)
        .map_err(|e| store::PackError::Schema(e))
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<BmpSnapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::DocumentPack>::decode_pack(bytes)?)
}
''',
        encoding="utf-8",
    )
    (base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/💾️binary/🦀️component.rs").write_text(
        '''//! 📤️ Serialize `stdio.bmp` to stdio.binary.

use crate::artifacts::binary::{BinarySnapshot, STDIO_BINARY_DOCUMENT_SCHEMA};
use crate::artifacts::bmp::BmpSnapshot;

pub fn register() {}

pub fn serialize(from: &BmpSnapshot) -> Result<BinarySnapshot, store::PackError> {
    let bytes = crate::artifacts::bmp::schema::snapshot::encode_bmp(from)
        .map_err(|e| store::PackError::Schema(e))?;
    Ok(BinarySnapshot { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes })
}
''',
        encoding="utf-8",
    )
    w4a.fix_mutations(base / "🧬️schema/🧬️mutations/🦀️component.rs")
    w4a.fix_graphql_ts(base, "width: Int!", "width: number; height: number; pixels: number[];")


fix_mesh_artifact("obj", "Obj", ROSTER["obj"]["dir"], OBJ_PARSE)
fix_mesh_artifact("ply", "Ply", ROSTER["ply"]["dir"], PLY_PARSE)
fix_stl()
fix_dxf()
fix_svg()
fix_bmp()

print("w4b codecs applied")
