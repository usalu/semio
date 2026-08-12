//! 🧬️ Cad snapshot schema — persistent fields only.

use crate::artifacts::cad::{CadGeometry, CadNode, CadObject, CadReferenceList};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted cad document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "cad.cad", layout = "lines")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub id: String,
    #[serde(default)]
    #[dsl(table)]
    #[state(persistent)]
    pub objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    #[state(persistent)]
    pub building_objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    #[state(persistent)]
    pub energy_objects: Vec<CadObject>,
    #[serde(default)]
    #[dsl(table)]
    #[state(persistent)]
    pub structure_classic_objects: Vec<CadObject>,
    #[serde(default)]
    #[state(persistent)]
    pub references_by_model_definition_id: BTreeMap<String, CadReferenceList>,
    #[serde(default)]
    #[dsl(table)]
    #[state(persistent)]
    pub nodes: Vec<CadNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    #[state(persistent)]
    pub shape_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    #[state(persistent)]
    pub building_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    #[state(persistent)]
    pub energy_geometry: Option<CadGeometry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    #[state(persistent)]
    pub structure_classic_geometry: Option<CadGeometry>,
    #[serde(default = "default_model_definition_id")]
    #[state(persistent)]
    pub active_model_definition_id: String,
}

fn default_model_definition_id() -> String {
    "spatial.shape".into()
}

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for CadSnapshot {
    const EXTENSION: &'static str = "cad";
    fn envelope_id() -> &'static str { "cad.cad" }
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

impl store::ArtifactPack for CadSnapshot {
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
//#endregion 🔖️Snapshot
