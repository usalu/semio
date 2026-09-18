//! 📤️ Lowpoly play app commands — shell effects: real mesh export/import round-trips through the host
//! (`exportMesh` → a `DownloadMediaExport`, `loadMeshRequest` → a `RequestFileOpen` the shell answers
//! with `importMeshFile`, which replaces the whole document OUTSIDE undo history via
//! `reset_document_effect`). The `🚪️io` OBJ/PLY/STL serializers and deserializers carry the real
//! geometry (n-gons, multiple objects); nothing here was reachable from the react shell before
//! 2026-09-18 (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS).

use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::io::export::serializers::artifacts::{obj::v3_0::any as obj_export, ply::v1_0::any as ply_export, stl::v_ascii::any as stl_export};
use crate::io::import::deserializers::artifacts::{obj::v3_0::any as obj_import, ply::v1_0::any as ply_import, stl::v_ascii::any as stl_import};
use crate::op::LowpolyMutation;
use crate::LowpolySnapshot;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 📦️ The file-open request id the shell echoes back on `importMeshFile`.
pub const LOWPOLY_MESH_FILE_OPEN_REQUEST: u64 = 1_301;
/// 📁️ What the shell's file picker accepts for a mesh import.
pub const LOWPOLY_MESH_FILE_ACCEPT: &str = ".obj,.ply,.stl";
/// 🧮️ The largest mesh file `importMeshFile` admits on the wire (a low-poly OBJ/PLY/STL is tens to a
/// few hundred KiB; the wire carries it as text or a base64 data url).
pub const LOWPOLY_MESH_FILE_BYTES: usize = 2 * 1024 * 1024;

fn media_fault(code: &str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

/// 📤️ `(filename, mime type, text)` of `snapshot` in `format` (`obj`/`ply`/`stl`).
pub fn export_mesh_text(snapshot: &LowpolySnapshot, format: &str) -> Result<(String, String, String), Fault> {
    let (extension, mime, bytes) = match format.trim().to_ascii_lowercase().as_str() {
        "obj" => ("obj", "model/obj", obj_export::serialize_bytes(snapshot)),
        "ply" => ("ply", "application/x-ply", ply_export::serialize_bytes(snapshot)),
        "stl" => ("stl", "model/stl", stl_export::serialize_bytes(snapshot)),
        other => return Err(media_fault("lowpoly.media.export-format", format!("unsupported mesh export format {other:?} (obj, ply, stl)"))),
    };
    let bytes = bytes.map_err(|error| media_fault("lowpoly.media.export", error.to_string()))?;
    let text = String::from_utf8(bytes).map_err(|error| media_fault("lowpoly.media.export", error.to_string()))?;
    let stem = snapshot.objects.first().map(|object| object.name.trim()).filter(|name| !name.is_empty()).unwrap_or("lowpoly");
    let stem: String = stem.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c.to_ascii_lowercase() } else { '-' }).collect();
    Ok((format!("{stem}.{extension}"), mime.to_string(), text))
}

/// 📥️ The file the shell opened — raw text, or a `data:…;base64,…` url — as bytes.
pub fn opened_file_bytes(payload: &str) -> Result<Vec<u8>, Fault> {
    let Some(rest) = payload.strip_prefix("data:") else { return Ok(payload.as_bytes().to_vec()) };
    let (header, body) = rest.split_once(',').ok_or_else(|| media_fault("lowpoly.media.import", "data url without a payload"))?;
    if header.ends_with(";base64") {
        base64_codec::base64_standard_decode(body.trim().as_bytes()).map_err(|error| media_fault("lowpoly.media.import", error.to_string()))
    } else {
        Ok(body.as_bytes().to_vec())
    }
}

/// 📥️ Decodes `name`'s extension's format from `bytes` into a lowpoly document.
pub fn import_mesh_bytes(name: &str, bytes: &[u8]) -> Result<LowpolySnapshot, Fault> {
    let extension = name.rsplit('.').next().map(str::to_ascii_lowercase).unwrap_or_default();
    let decoded = match extension.as_str() {
        "obj" => obj_import::deserialize_bytes(bytes),
        "ply" => ply_import::deserialize_bytes(bytes),
        "stl" => stl_import::deserialize_bytes(bytes),
        other => return Err(media_fault("lowpoly.media.import-format", format!("unsupported mesh file {name:?} (.{other}; obj, ply, stl)"))),
    };
    decoded.map_err(|error| media_fault("lowpoly.media.import", format!("{name}: {error}")))
}

//#region 🔖️ExportMesh
pub mod export_mesh {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "export-mesh")]
    pub struct ExportMesh {
        pub format: String,
    }

    pub fn handle(payload: &ExportMesh, doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let (filename, mime_type, data) = export_mesh_text(doc.snapshot, &payload.format)?;
        // ⬇️ An absent `encoding` means "`data` IS the file" (the host accepts only that or `base64`).
        Ok(Emit::effect(Effect::DownloadMediaExport { filename, mime_type, data, encoding: None }))
    }
}
//#endregion 🔖️ExportMesh

//#region 🔖️LoadMeshRequest
pub mod load_mesh_request {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "load-mesh-request")]
    pub struct LoadMeshRequest {}

    pub fn handle(_payload: &LoadMeshRequest, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        Ok(Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(LOWPOLY_MESH_FILE_OPEN_REQUEST), accept: LOWPOLY_MESH_FILE_ACCEPT.into(), read_as: Some("dataUrl".into()), import_action: "importMeshFile".into(), multiple: false }))
    }
}
//#endregion 🔖️LoadMeshRequest

//#region 🔖️ImportMeshFile
pub mod import_mesh_file {
    use super::*;

    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(Serialize, Deserialize))]
    #[dsl(keyword = "import-mesh-file")]
    pub struct ImportMeshFile {
        pub name: String,
        pub payload: String,
    }

    /// 📥️ Importing a mesh file replaces the whole document, which has no in-history mutation (see
    /// `📓️taxonomy.md`), so this routes through `reset_document_effect` (a `Effect::LoadDocument`).
    pub fn handle(payload: &ImportMeshFile, _doc: &ArtifactView<'_, LowpolySnapshot>, _cfg: &ConfigView<'_, LowpolyConfig>, _ctx: &mut LowpolyScratch) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault> {
        let bytes = opened_file_bytes(&payload.payload)?;
        let snapshot = import_mesh_bytes(&payload.name, &bytes)?;
        Ok(Emit { effects: vec![crate::editor::lowpoly::reset_document_effect(&snapshot)], ..Default::default() })
    }
}
//#endregion 🔖️ImportMeshFile

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
