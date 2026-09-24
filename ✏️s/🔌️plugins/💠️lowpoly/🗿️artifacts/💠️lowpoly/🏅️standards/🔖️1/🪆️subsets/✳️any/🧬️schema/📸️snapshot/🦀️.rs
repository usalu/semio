//! 🧬️ Lowpoly snapshot schema — artifact-lane fields only.
//!
//! `ArtifactDsl` and `ArtifactPack` are the derived spec-driven text and pack of the one
//! `dsl::DslRecord` spec, which carries each object's composed `mesh` child handle.

use crate::{LowpolyObject, LowpolyPaintLayer, LowpolyTransform, LOWPOLY_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted lowpoly document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "lowpoly")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolySnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub objects: Vec<LowpolyObject>,
}

impl Default for LowpolySnapshot {
    fn default() -> Self {
        Self { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), objects: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Handcrafted `ArtifactDsl`; `ArtifactPack` over the derived record spec.
impl store::ArtifactDsl for LowpolySnapshot {
    const EXTENSION: &'static str = "lowpoly";
    fn envelope_id() -> &'static str {
        "lowpoly.lowpoly"
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

impl store::ArtifactPack for LowpolySnapshot {
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
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️DocumentHelpers
/// 🏗️ Builds a single-object snapshot from mesh JSON — only the real persisted `mesh` handle
/// (content-addressed off `mesh_json` via `mesh_child_handle`, identical geometry always resolving
/// to the identical handle). The caller is responsible for seeding its OWN session-local
/// `mesh_workspace` cache (`✏️editor/🖌️session::LowpolyScratch`) with `mesh_json` under
/// `object_id` — this function no longer does it implicitly (round 2 of this ticket's round-trip
/// law fix: `LowpolyObject` carries no live mesh content at all any more, see that struct's own doc
/// comment).
pub fn snapshot_from_mesh_json(mesh_json: &str, object_id: &str, object_name: &str) -> LowpolySnapshot {
    LowpolySnapshot {
        schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        objects: vec![LowpolyObject {
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
//#endregion 🔖️DocumentHelpers
