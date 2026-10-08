//! 📜️ Lowpoly artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::LowpolySnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📜️ The reuse example, handcrafted against `COMPONENT_GRAMMAR_SEMIO` — structured half-edge mesh
/// productions (no `mesh-json`). Derive-based `parse_dsl` does not yet consume this shape; the
/// recognizer / handcrafted codec will.
pub const LOWPOLY_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🗣️.dsl.semio");

/// 📖️ Parses `.lowpoly` DSL text into a `LowpolySnapshot`.
pub fn parse_dsl(text: &str) -> Result<LowpolySnapshot, semio_framework_diagnostic::TextError> {
    <LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `LowpolySnapshot` back to `.lowpoly` DSL text.
pub fn print_dsl(document: &LowpolySnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type LowpolySnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{LowpolyObject, LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

/// ✉️ Handcrafted `ArtifactDsl`; `ArtifactPack` over the derived record spec.
impl store::ArtifactDsl for LowpolySnapshot {
    const EXTENSION: &'static str = "lowpoly";
    fn envelope_id() -> &'static str {
        "lowpoly.lowpoly"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

use crate::{LowpolyObject,LowpolyPaintLayer,LowpolyTransform,LOWPOLY_DOCUMENT_SCHEMA};
use crate::schema::LOWPOLY_DEFAULT_EXAMPLE_LABEL;
use semio_framework_3d::mesh::HalfedgeMesh;

/// 🏗️ Builds an object with its authored mesh source and content-addressed child handle.
pub fn snapshot_from_mesh_json(mesh_json: &str, object_id: &str, object_name: &str) -> LowpolySnapshot {
    LowpolySnapshot {
        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        objects: vec![LowpolyObject { mesh_state:None,
            id: object_id.into(),
            name: object_name.into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh: Some(crate::mesh_child_handle(object_id, mesh_json)),
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
            mesh_content: mesh_json.into(),
        }],
    }
}

const CONCRETE_FOREST_LEFT_MESH_JSON: &str = include_str!("../../../📚️examples/🌲️hexagonal-cut-concrete-forest-left/🖼️assets/🧊️.mesh.json");

/// 🧺 One caller-owned default document pair. The parent snapshot owns only the exact
/// `ArtifactChild` handle while the app session owns its matching mesh payload.
pub struct LowpolyOwnedDefaultDocument {
    pub snapshot: crate::LowpolySnapshot,
    pub mesh_workspace: std::collections::HashMap<String, crate::LowpolyMeshState>,
}

/// 🌲️ Hexagonal Cut Concrete Forest Left — the same CAD-derived mesh fixture puzzle 3d and cad shape use.
pub fn concrete_forest_left_owned_document() -> LowpolyOwnedDefaultDocument {
    let mesh=HalfedgeMesh::from_json(CONCRETE_FOREST_LEFT_MESH_JSON).expect("authored default mesh source is valid");
    let state=crate::LowpolyMeshState::from_mesh(mesh);
    let mut snapshot=snapshot_from_mesh_json(CONCRETE_FOREST_LEFT_MESH_JSON,"obj-1",LOWPOLY_DEFAULT_EXAMPLE_LABEL);
    snapshot.objects[0].mesh=Some(crate::managed_mesh_child_handle("obj-1",&state));
    snapshot.objects[0].mesh_state=Some(state.clone());
    let mesh_workspace=std::collections::HashMap::from([("obj-1".to_string(),state)]);
    LowpolyOwnedDefaultDocument { snapshot, mesh_workspace }
}

/// 🧱️ Builds the default play document without UV repacking, so every owner gets a fresh matching
/// handle/payload pair and no process-global child payload cache is required.
pub fn default_owned_document() -> LowpolyOwnedDefaultDocument {
    concrete_forest_left_owned_document()
}

/// 🎞️ Default document projection used by tests and the play app.
pub fn default_snapshot() -> crate::LowpolySnapshot {
    default_owned_document().snapshot
}

/// 🕸️ Fresh app-owned companion payload for `default_snapshot()`'s exact child handle.
pub fn default_mesh_workspace() -> std::collections::HashMap<String, crate::LowpolyMeshState> {
    default_owned_document().mesh_workspace
}

