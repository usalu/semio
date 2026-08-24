//! 🧬️ Fem2d snapshot schema — artifact-lane fields only.

use crate::artifacts::fem2d::{FemAnalysisSettings, FemCombination, FemElement, FemLoadCase, FemMaterial, FemNode, FemRegion, FemSection, FemSupport};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted fem2d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "fem.fem2d", layout = "lines")]
#[artifact_schema(id = "s.fem.fem2d")]
pub struct Fem2dSnapshot {
    #[dsl(table)]
    #[state(artifact)]
    pub nodes: Vec<FemNode>,
    #[dsl(statements, block)]
    #[state(artifact)]
    pub elements: Vec<FemElement>,
    #[dsl(table)]
    #[state(artifact)]
    pub regions: Vec<FemRegion>,
    #[dsl(table)]
    #[state(artifact)]
    pub materials: Vec<FemMaterial>,
    #[dsl(table)]
    #[state(artifact)]
    pub sections: Vec<FemSection>,
    #[dsl(table)]
    #[state(artifact)]
    pub supports: Vec<FemSupport>,
    #[dsl(table)]
    #[state(artifact)]
    pub load_cases: Vec<FemLoadCase>,
    #[dsl(table)]
    #[state(artifact)]
    pub combinations: Vec<FemCombination>,
    #[dsl(block)]
    #[state(artifact)]
    pub analysis: FemAnalysisSettings,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for Fem2dSnapshot {
    const EXTENSION: &'static str = "fem2d";
    fn envelope_id() -> &'static str {
        "fem.fem2d"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Fem2dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🌉️IdentityBridge
/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `fem2d_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn fem2d_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <Fem2dSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <Fem2dSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <Fem2dSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <Fem2dSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <Fem2dSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = serde_json::json!({
        "parsed": serde_json::to_value(&parsed).map_err(|error| error.to_string())?,
        "reparsed": serde_json::to_value(&reparsed).map_err(|error| error.to_string())?,
        "packDecoded": serde_json::to_value(&unpacked).map_err(|error| error.to_string())?,
        "canonicalText": canonical,
        "canonicalTextAgain": canonical_again,
    });
    Ok(report.to_string())
}
//#endregion 🌉️IdentityBridge
