//! 🧮️ GIS 3D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: panning and selecting never enter the document's undo
//! history, but they still round-trip through the config `ArtifactStore` with a true `backwards`.
//! The terrain's one editable property (exaggeration) is document state and lives in
//! `crate`.

#[cfg(test)]
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🧮️ gis3d's `ArtifactEditor::Config` for the free/live viewport camera.
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "gis3dcfg")]
#[dsl(id = "gis.gis3dcfg")]
#[dsl(layout = "lines")]
pub struct Gis3dConfig {
    /// 🎥️ The free/live world camera (`{position,target,up,fov}` JSON).
    pub camera_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Gis3dConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
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

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for Gis3dConfig {
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

//#endregion 🔖️ArtifactCodec

/// 🎥️ A default overview camera scaled for a real-world DEM tile patch (hundreds of meters to a
/// few kilometers wide) — the generic `world3d_default_camera()` (position `[4,-4,3]`) assumes an
/// object-scale scene and would sit inside the ground here.
fn default_gis3d_camera_json() -> String {
    serde_json::json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string()
}

impl Default for Gis3dConfig {
    fn default() -> Self {
        Self { camera_json: default_gis3d_camera_json(), }
    }
}

store::impl_whole_record_config!(Gis3dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod configuration_diff;
pub use configuration_diff::{Gis3dConfigDelta, Gis3dConfigDiff};

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🔖️OpCodec
impl protocol::OpText for Gis3dConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Gis3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

//#endregion 🔖️OpCodec

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying a `set-camera` mutation to a `Gis3dConfig`, for a
/// language-neutral test adapter — the identical shape and purpose
/// `crate::gis_terrain_mutation_report_json` already establishes for the
/// terrain's own document-level mutations, applied here to gis3d's editor-config artifact (shard
/// G4, this ticket).
///
/// Every field of `Gis3dConfig` is a plain `String` — this bridge never
/// needs `serde_json::from_str::<Gis3dConfig>` at all, and so never needs this struct's own
/// `#[cfg_attr(test, derive(Serialize, Deserialize))]` (unavailable to a `sut`-feature adapter
/// crate, which links this crate as an ordinary dependency, not under `cfg(test)`). Every type in
/// this bridge's own signature is a `str`, matching `gis_terrain_mutation_report_json`'s own doc
/// comment on why that is the whole surface an adapter needs — a generated test host links only
/// `semio-repo-test-host` and, behind its `sut` feature, this crate, so neither `Gis3dConfigMutation`
/// nor `Gis3dConfig` can be named there, and hand-transcribing either into a Rust literal would be a
/// second copy of the committed specification vector, free to drift away from it.
///
/// `Mutation<Gis3dConfig>`/`MutationDiff<Gis3dConfig>` (via `#[derive(dsl::Mutations)]` on
/// `Gis3dConfigMutation` and `#[derive(dsl::MutationLeaf)]` on `SetCamera` are the
/// UNCONDITIONAL mutation-engine traits every leaf's own `diff`/`apply`/`inverse` already exercise
/// in this file's own `#[cfg(test)] mod tests` above — never gated by `cfg(test)`, unlike
/// `Serialize`/`Deserialize`/`ToValue`/`FromValue` — so this bridge reaches the exact same real
/// production behavior those unit tests already assert, through a route this crate's own default
/// build always compiles.
pub fn gis3d_config_mutation_report_json(camera_json: &str, kind: &str, value: &str) -> Result<String, String> {
    use protocol::{Mutation, MutationDiff};
    let base = Gis3dConfig { camera_json: camera_json.to_string() };
    let mutation: Gis3dConfigMutation = match kind {
        "set-camera" => Gis3dConfigMutation::SetCamera(SetCamera { camera_json: value.to_string() }),
        other => return Err(format!("gis3d_config_mutation_report_json: unknown kind {other:?}")),
    };
    let applied = mutation.diff(&base).diff().apply(&base).map_err(|error| error.to_string())?;
    let inverse_steps = mutation.inverse(&base);
    let mut undone = applied.clone();
    for step in &inverse_steps {
        undone = step.diff(&undone).diff().apply(&undone).map_err(|error| error.to_string())?;
    }
    let report = serde_json::json!({
        "base": {"cameraJson": base.camera_json},
        "snapshot": {"cameraJson": applied.camera_json},
        "inverseSnapshot": {"cameraJson": undone.camera_json},
    });
    Ok(report.to_string())
}
//#endregion 🌉️TestBridge

//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧬️direct-leaves/🦀️.rs"]
mod direct_leaf_contracts;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
