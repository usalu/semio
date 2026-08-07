//! ⛰️ GIS terrain artifact — the document entity the 🧊️3d app edits (constitutional: general).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
/// 🗄️ VCS-backed, undoable document for GIS 3D — deliberately minimal for the first pass: the only
/// editable/undoable property is vertical exaggeration (a genuinely useful terrain control).
pub const GIS_3D_TERRAIN_SCHEMA: &str = "gis.terrain";
//#endregion 🔖️Constants

//#region 🔖️Types
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
#[dsl(id = "gis.gisterrain", keyword = "gisterrain")]
pub struct Gis3dTerrainDocument {
    pub exaggeration: f64,
    /// 🔌️ `map:in`'s insertion point (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE
    /// Wave 2 port recipe): the last-imported `2d.map` descriptor JSON (`{positions,routes,regions}`,
    /// same shape `crate::artifacts::gismap::engine::gis_map_descriptor_json` produces), rendered as an
    /// extra pin layer alongside the read-only fixture-text positions (see the 🏔️terrain window's
    /// `instances_json`) — real, undoable document state, not view-only scratch, since importing a map
    /// overlay is a document edit.
    pub imported_features_json: String,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for Gis3dTerrainDocument {
    const EXTENSION: &'static str = "gisterrain";
    fn envelope_id() -> &'static str { "gis.gisterrain" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for Gis3dTerrainDocument {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
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
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs



//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🔌️ `3d.mesh` — the interchange kind `scene:out` produces; canonically declared by `lowpoly`
/// (`mesh_from_mesh_document`'s registration). Re-declared here as an identical-shape duplicate so the
/// 🧊️3d manifest is self-describing on both sides of the edge (the registry dedupes by id).
pub fn mesh_artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.mesh".into(),
        name: "3D Mesh".into(),
        source_format: "mesh.reference".into(),
        component_kind: "mesh".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: "mesh.reference".into(),
        export_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj, OsMediaFormat::Stl],
        import_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_artifact_kind_matches_the_scene_out_interchange_kind() {
        let kind = mesh_artifact_kind();
        assert_eq!(kind.id, "3d.mesh");
        assert_eq!(kind.schema, "mesh.reference");
    }

    #[test]
    fn the_terrain_document_defaults_to_a_flat_unimported_terrain() {
        let document = Gis3dTerrainDocument::default();
        assert_eq!(document.exaggeration, 0.0);
        assert!(document.imported_features_json.is_empty());
    }
}
//#endregion 🧪️Tests
