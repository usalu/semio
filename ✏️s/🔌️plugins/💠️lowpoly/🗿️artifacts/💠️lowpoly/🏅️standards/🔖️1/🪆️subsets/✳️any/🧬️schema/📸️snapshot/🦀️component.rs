//! 🧬️ Lowpoly snapshot schema — persistent fields only.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted lowpoly document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "lowpoly", layout = "lines")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolySnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub objects: Vec<LowpolyObject>,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for LowpolySnapshot {
    const EXTENSION: &'static str = "lowpoly";
    fn envelope_id() -> &'static str { "lowpoly.lowpoly" }
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
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LowpolySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 🏗️ Builds a single-object snapshot from mesh JSON.
pub fn snapshot_from_mesh_json(mesh_json: &str, object_id: &str, object_name: &str) -> LowpolySnapshot {
    LowpolySnapshot {
        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        objects: vec![LowpolyObject {
            id: object_id.into(),
            name: object_name.into(),
            transform: LowpolyTransform::default(),
            smooth_shading: false,
            mesh_json: mesh_json.into(),
            paint_layers: vec![LowpolyPaintLayer::new("Base")],
        }],
    }
}
//#endregion 🔖️Snapshot

impl Default for LowpolySnapshot {
    fn default() -> Self {
        Self { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: Vec::new() }
    }
}
