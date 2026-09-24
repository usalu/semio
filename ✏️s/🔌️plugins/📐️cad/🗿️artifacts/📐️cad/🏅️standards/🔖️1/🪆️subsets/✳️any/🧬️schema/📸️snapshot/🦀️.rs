//! 🧬️ Cad snapshot schema — artifact-lane fields only.

use crate::{empty_cad_snapshot, CadDrawingChild, CadModelChild, CadNode, CadReferenceList};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted cad document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the four per-pane object/geometry field
/// pairs that used to duplicate `SemioBrepSnapshot`'s topology inline (`CadObject`/`CadGeometry` at
/// `crate::🦀️.rs`) are replaced by four fixed composed
/// `s.stdio.semio.model` CHILD slots — one per `CadPaneId` — plus a forward `drawings` composition
/// slot per the design map's `cad | engineering assembly | model, drawing` row. `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
///
/// 🛡️ `deny_unknown_fields` closes that replacement: a snapshot still carrying the retired inline
/// `objects`/`shapeGeometry`/`activeModelDefinitionId` keys must FAIL to decode, never decode with
/// them silently dropped (`🧫️fixtures/🪪️document-contract`'s `invalidDocuments`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, ArtifactSchema, dsl::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "cad")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shape_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub building_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub energy_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub structure_classic_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default)]
    pub drawings: Vec<CadDrawingChild>,
    #[value(default)]
    #[state(artifact)]
    pub references_by_model_definition_id: BTreeMap<String, CadReferenceList>,
    #[value(default)]
    #[state(artifact)]
    pub nodes: Vec<CadNode>,
}

//#region 🔖️ExactChildren
fn exact_child<S>(child_id: String, target: store::os_io::ArtifactRef, subset: &str) -> Result<store::ArtifactChild<S>, String> {
    if child_id != target.artifact_id {
        return Err("cad child id must equal target artifact id".into());
    }
    if target.dialect.artifact_kind != "s.stdio.semio" || target.dialect.standard != "v1" || target.dialect.subset != subset {
        return Err(format!("cad child must target s.stdio.semio@v1/{subset}"));
    }
    Ok(store::ArtifactChild::new(child_id, target))
}

/// 🛡️ Every composed child handle must name its own target and the exact `s.stdio.semio@v1` subset.
fn require_exact_children(s: &CadSnapshot) -> Result<(), String> {
    for child in [&s.shape_model, &s.building_model, &s.energy_model, &s.structure_classic_model].into_iter().flatten() {
        exact_child::<()>(child.child_id.clone(), child.target.clone(), "model")?;
    }
    for child in &s.drawings {
        exact_child::<()>(child.child_id.clone(), child.target.clone(), "drawing")?;
    }
    Ok(())
}
//#endregion 🔖️ExactChildren

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ `ArtifactDsl` and `ArtifactPack` are the derived spec-driven text and pack of the one
/// `dsl::DslRecord` spec; both re-check every composed child's exact identity on decode.
impl store::ArtifactDsl for CadSnapshot {
    const EXTENSION: &'static str = "cad";
    fn envelope_id() -> &'static str {
        "cad.cad"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        let snapshot = Self::__dsl_from_record(&record)?;
        require_exact_children(&snapshot).map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))?;
        Ok(snapshot)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for CadSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        let snapshot = Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?;
        require_exact_children(&snapshot).map_err(store::PackError::Schema)?;
        Ok(snapshot)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

