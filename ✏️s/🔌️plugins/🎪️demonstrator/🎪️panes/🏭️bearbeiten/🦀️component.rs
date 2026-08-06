//! 🏭️ `bearbeiten` pane — the demonstrator's entwerfen-mit-bestand fabrication surface, served by
//! 🏭️process's `process3d-play` app. This is the one pane that owns real bridging logic: process's
//! engine tessellates a typed `Process3dDocument` rather than raw JSON, so the untyped
//! `MeshData`-codec signature the OS registries expect is adapted here.
//!
//! See <https://github.com/usalu/semio/issues/2510> for the bundle rationale.

use semio_framework_plugin::{GlbExporter, MeshData, ObjExporter, PluginBundle, StlExporter};
use serde_json::Value;

use process::apps::process3d::{create_process3d_app, Process3dPlayApp};
use process::artifacts::process3d::engine::processed_mesh;
use process::artifacts::process3d::{Process3dDocument, PROCESS_3D_SCHEMA};

const PROCESS_3D_KIND: &str = "3d.process";
const PROCESS_3D_FORMAT: &str = "process";

/// 🔺️ Replays a process document's fabrication steps and returns the resulting mesh — the JSON-typed
/// adapter the OS mesh registries require around `process`'s typed engine entry point.
fn mesh_from_document(doc: &Value) -> Result<MeshData, String> {
    let document: Process3dDocument = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    processed_mesh(&document).ok_or_else(|| "process3d: kernel replay failed".to_string())
}

/// 🚫️ Process documents are a fabrication program, not a mesh — there is no meaningful inverse, so the
/// importer slot is filled with an explicit refusal rather than left unregistered.
fn document_from_mesh(_mesh: &MeshData) -> Result<Value, String> {
    Err("process3d: mesh import not supported".into())
}

/// 🔌️ Binds process's mesh codecs into the OS export registries and the app's document codec into the
/// plugin runtime.
pub fn register_exports() {
    semio_framework_os::register_mesh_exporter(PROCESS_3D_KIND, PROCESS_3D_FORMAT, mesh_from_document, Box::new(ObjExporter));
    semio_framework_os::register_mesh_exporter(PROCESS_3D_KIND, PROCESS_3D_FORMAT, mesh_from_document, Box::new(GlbExporter));
    semio_framework_os::register_mesh_exporter(PROCESS_3D_KIND, PROCESS_3D_FORMAT, mesh_from_document, Box::new(StlExporter));
    semio_framework_os::register_mesh_dwg_export_handler(PROCESS_3D_KIND, PROCESS_3D_FORMAT, mesh_from_document);
    semio_framework_os::register_mesh_dwg_import_handler(PROCESS_3D_KIND, document_from_mesh);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Process3dPlayApp>(PROCESS_3D_SCHEMA);
}

/// 🎪️ Adds the pane's app to the shared demonstrator bundle.
pub fn register_app(bundle: PluginBundle) -> PluginBundle {
    bundle.register_document_app(create_process3d_app(), || Process3dPlayApp)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ The JSON adapter must surface a deserialization failure as an error string, rather than
    /// panic — the OS mesh registry calls it with whatever the host hands over.
    #[test]
    fn mesh_from_document_rejects_a_non_document_payload() {
        assert!(mesh_from_document(&serde_json::json!("not a process document")).is_err());
    }

    /// 🧪️ An empty-but-well-formed process document still replays to a mesh (every field defaults), so
    /// the adapter's success path stays reachable for the host's export handlers.
    #[test]
    fn mesh_from_document_replays_a_default_document() {
        assert!(mesh_from_document(&serde_json::json!({})).is_ok());
    }

    /// 🧪️ Mesh import stays explicitly unsupported for process documents.
    #[test]
    fn document_from_mesh_always_refuses() {
        assert_eq!(document_from_mesh(&MeshData::default()).unwrap_err(), "process3d: mesh import not supported");
    }
}
//#endregion 🧪️Tests
