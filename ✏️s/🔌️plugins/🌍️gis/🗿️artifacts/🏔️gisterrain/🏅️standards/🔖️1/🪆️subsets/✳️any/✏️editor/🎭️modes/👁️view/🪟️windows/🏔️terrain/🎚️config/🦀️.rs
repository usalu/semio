//! 🧮️ Persisted-local camera and brush configuration owned by one exact GIS Terrain window.
//!
//! Each registered `gis3d-main` window stores and reloads its own camera and brush settings.

use crate::schema::terrain_descriptor::TerrainDescriptorJson;
use crate::standards::v1::subsets::any::io::text::snapshot::{artifact_terrain_camera_frame, build_artifact_terrain_camera_json, fixture_descriptor};
use crate::GisTerrainSnapshot;
use semio_framework_pack_json::{from_dsl_value, parse as parse_json, to_dsl_value, JsonMemberPolicy};
use semio_framework_value::{FromValue as _, NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ToValue as _, ValueError};
use semio_framework_value_derive::{FromValue, ToValue};
use store::os_store::codec_registry::native_snapshot::{OriginalArtifactHistoryReadLimits, OriginalArtifactPackReceiving, OriginalArtifactPairFailure, OriginalSnapshotRecordReceiving};
#[cfg(test)]
use serde::{Deserialize, Serialize};

#[path = "🖌️brush/🦀️.rs"]
mod brush;
pub use brush::TerrainBrushSettings;

//#region 🔖️Config
/// 🧮️ Free/live viewport camera for one concrete terrain window.
#[derive(semio_framework_dsl_record_derive::DslRecord, semio_framework_dsl_record_derive::DslReceiving, Clone, Debug, Default, PartialEq, semio_os_kernel::DslArtifact, ToValue, FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact(extension = "gisterrainwindowcfg")]
#[artifact(id = "gis.gisterrainwindowcfg")]
#[dsl(layout = "lines")]
pub struct GisTerrainWindowConfig {
    /// 🎥️ The free/live world camera JSON. An empty value frames the terrain automatically.
    pub camera_json: String,
    #[dsl(block)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub brush: Option<TerrainBrushSettings>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl pack::snapshot::ArtifactDsl for GisTerrainWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        let config = Self::__dsl_from_record(&record)?;
        config.validate_brush().map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(config)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl pack::snapshot::ArtifactPack for GisTerrainWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        let config = Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)?;
        config.validate_brush().map_err(store::PackError::from)?;
        Ok(config)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for GisTerrainWindowConfig {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}

impl OriginalArtifactPackReceiving for GisTerrainWindowConfig {
    type Receiving = OriginalSnapshotRecordReceiving<Self>;

    fn original_pack_birth_demand() -> RetirementDemand {
        RetirementDemand { copy_bytes: std::mem::size_of::<Self::Receiving>(), depth: 1, ..Default::default() }
    }

    fn admit_original_pack(
        source: &[u8],
        limits: protocol::PackLimits,
        native: &NativeDecodeControl<'_>,
        grant: RetainedCloneGrant,
    ) -> Result<Option<(Self::Receiving, RetainedCloneProgress)>, ValueError> {
        Self::Receiving::admit(source, Self::__DSL_ENVELOPE_ID, limits, native, grant)
    }
}

//#endregion 🔖️ArtifactCodec

impl store::ConfigRecord for GisTerrainWindowConfig {}

impl GisTerrainWindowConfig {
    fn validate_brush(&self) -> Result<(), semio_framework_value::ValueError> {
        self.brush.map(TerrainBrushSettings::validate).transpose().map(|_| ())
    }

    /// 🧭️ Restores a saved camera if its local coordinates belong to the current terrain.
    pub fn scene_camera(self, descriptor: &TerrainDescriptorJson, document: &GisTerrainSnapshot) -> String {
        let georeferenced = document.imported_map.is_some() || document.elevation_source.is_some();
        if let Ok(mut saved) = parse_json(&self.camera_json, JsonMemberPolicy::Reject) {
            if let Some(fields) = saved.as_object_mut() {
                let frame = fields.remove("terrainFrame");
                if !georeferenced && frame.is_none() {
                    return self.camera_json;
                }
                let same_frame = frame.and_then(|value| <[f64; 3]>::from_value(to_dsl_value(&value)).ok())
                    .is_some_and(|saved| saved.into_iter().zip(artifact_terrain_camera_frame(document, descriptor))
                        .all(|(actual, expected)| (actual - expected).abs() <= expected.abs().max(1.0) * 1e-12));
                if georeferenced && same_frame {
                    return saved.to_string();
                }
            }
        }
        build_artifact_terrain_camera_json(document, descriptor)
    }
}

/// 🎥️ Saves the window's camera in the geographic frame active when it was captured.
pub fn addressed_camera(view: &semio_plugin::ViewModel, document: &GisTerrainSnapshot, camera_json: &str) -> Result<semio_plugin::WindowConfigMutation, semio_plugin::Fault> {
    let mut camera_json = camera_json.to_owned();
    if document.imported_map.is_some() || document.elevation_source.is_some() {
        let mut camera = parse_json(&camera_json, JsonMemberPolicy::Reject).map_err(|_| semio_plugin::Fault::from("gis-terrain-camera-json-invalid"))?;
        let fields = camera.as_object_mut().ok_or_else(|| semio_plugin::Fault::from("gis-terrain-camera-json-invalid"))?;
        fields.insert("terrainFrame", from_dsl_value(&artifact_terrain_camera_frame(document, &fixture_descriptor(document)).to_value()));
        camera_json = camera.to_string();
    }
    addressed(view, GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json }))
}

pub struct GisTerrainWindowConfigOwner;

impl semio_plugin::WindowConfigOwner for GisTerrainWindowConfigOwner {
    fn initial_state_demands()->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:2*std::mem::size_of::<Self::State>(),depth:1,..Default::default()})}
    fn initial_state(grant:semio_framework_value::RetainedCloneGrant)->Result<(Self::State,semio_framework_value::RetainedCloneProgress),semio_framework_value::ValueError>{let d=Self::initial_state_demands()?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original window config birth is not funded"))}Ok((Self::State::default(),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    fn initial_state_pack_source(state:&Self::State)->Result<(&dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,semio_framework_dsl_record::BorrowedRecordSpec),semio_framework_value::ValueError>{Ok((state,<Self::State as semio_framework_dsl_record::BorrowedDslRecord>::RECORD))}
    const WINDOW_KIND_ID: &'static str = super::GIS3D_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "gis.gisterrainwindowcfg";
    const MAXIMUM_PUBLICATION_BYTES: usize = 8_192;
    type State = GisTerrainWindowConfig;
    type Mutation = GisTerrainWindowConfigMutation;
    type Edit = store::snapshot_clone_preparation::ConfigApplyEdit<GisTerrainWindowConfig, GisTerrainWindowConfigMutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize = 64;

    fn native_pair_demands() -> Result<RetirementDemand, ValueError> {
        Ok(semio_plugin::window_config_native_pair_demands::<Self::State>())
    }

    fn admit_native_pair(
        pack: &[u8],
        spr: &[u8],
        pack_limits: protocol::PackLimits,
        history_limits: OriginalArtifactHistoryReadLimits,
        native: &mut NativeDecodeControl<'_>,
        grant: RetainedCloneGrant,
        output: &mut Option<Box<dyn semio_plugin::WindowConfigNativePairReceiving<Self::State>>>,
    ) -> Result<RetainedCloneProgress, OriginalArtifactPairFailure> {
        semio_plugin::admit_window_config_native_pair::<Self::State>(pack, spr, pack_limits, history_limits, native, grant, output)
    }

    fn native_operation_wire() -> store::os_store::ArtifactOperationWire<Self::Mutation> {
        store::os_store::ArtifactOperationWire::canonical().with_original_binary_receiving()
    }

    fn build_retained_edit() -> std::sync::Arc<Self::Edit> {
        std::sync::Arc::new(store::snapshot_clone_preparation::ConfigApplyEdit::new())
    }

    fn preparation_source_birth() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand { semio_plugin::bounded_window_config_preparation_source_birth::<Self>() }
    fn build_store_owners(grant: semio_framework_value::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::State, Self::Mutation>> { semio_plugin::bounded_window_config_store_owners::<Self>(grant) }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn store_disposer_birth() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_plugin::bounded_window_config_store_disposer_birth::<Self>()
    }

    fn build_store_disposer() -> Box<dyn semio_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

pub fn current<C>(view: &semio_plugin::ConfigView<'_, C>) -> GisTerrainWindowConfig {
    view.window::<GisTerrainWindowConfigOwner>().cloned().unwrap_or_default()
}

#[doc(hidden)]
pub const WINDOW_CONTEXT_REQUIRED: &str = "gis.terrain.window.context-required";

#[doc(hidden)]
pub fn missing_window() -> semio_plugin::Fault {
    semio_plugin::Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new(WINDOW_CONTEXT_REQUIRED), "Choose the terrain window to configure.")
}

pub fn addressed(view: &semio_plugin::ViewModel, mutation: GisTerrainWindowConfigMutation) -> Result<semio_plugin::WindowConfigMutation, semio_plugin::Fault> {
    let window_id = view.window_id.as_deref().ok_or_else(missing_window)?;
    let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
    if kind != Some(super::GIS3D_PLAY_WINDOW_MAIN) {
        return Err(semio_plugin::Fault::from("gis-terrain-window-kind-required"));
    }
    Ok(semio_plugin::WindowConfigMutation::of::<GisTerrainWindowConfigOwner>(window_id, mutation))
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod configuration_diff;
pub use configuration_diff::{GisTerrainWindowConfigDelta, GisTerrainWindowConfigDiff, TerrainBrushChange};

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🔖️OpCodec
impl protocol::OpText for GisTerrainWindowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for GisTerrainWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
dsl::original_ordinal_operation_receiving!(GisTerrainWindowConfigMutation);

//#endregion 🔖️OpCodec

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying a `set-camera` mutation to a `GisTerrainWindowConfig`, for a
/// language-neutral test adapter — the identical shape and purpose
/// `crate::standards::v1::subsets::any::io::text::mutations::gis_terrain_mutation_report_json` already establishes for the
/// terrain's own document-level mutations, applied here to gis3d's editor-config artifact (shard
/// G4, this ticket).
///
/// The camera vector supplies the only field used by this bridge, so the bridge does not need
/// `serde_json::from_str::<GisTerrainWindowConfig>` or this struct's
/// `#[cfg_attr(test, derive(Serialize, Deserialize))]` (unavailable to a `sut`-feature adapter
/// crate, which links this crate as an ordinary dependency, not under `cfg(test)`). Every type in
/// this bridge's own signature is a `str`, matching `gis_terrain_mutation_report_json`'s own doc
/// comment on why that is the whole surface an adapter needs — a generated test host links only
/// `semio-repo-test-host` and, behind its `sut` feature, this crate, so neither `GisTerrainWindowConfigMutation`
/// nor `GisTerrainWindowConfig` can be named there, and hand-transcribing either into a Rust literal would be a
/// second copy of the committed specification vector, free to drift away from it.
///
/// `Mutation<GisTerrainWindowConfig>`/`MutationDiff<GisTerrainWindowConfig>` (via `#[derive(dsl::Mutations)]` on
/// `GisTerrainWindowConfigMutation` and `#[derive(dsl::MutationLeaf)]` on `SetCamera` are the
/// UNCONDITIONAL mutation-engine traits every leaf's own `diff`/`apply`/`inverse` already exercise
/// in this file's own `#[cfg(test)] mod tests` above — never gated by `cfg(test)`, unlike
/// `Serialize`/`Deserialize`/`ToValue`/`FromValue` — so this bridge reaches the exact same real
/// production behavior those unit tests already assert, through a route this crate's own default
/// build always compiles.
pub fn gis_terrain_window_config_mutation_report_json(camera_json: &str, kind: &str, value: &str) -> Result<String, String> {
    use protocol::{ColdMutation, Mutation};
    let base = GisTerrainWindowConfig { camera_json: camera_json.to_string(), ..Default::default() };
    let mutation: GisTerrainWindowConfigMutation = match kind {
        "set-camera" => GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: value.to_string() }),
        other => return Err(format!("gis_terrain_window_config_mutation_report_json: unknown kind {other:?}")),
    };
    let applied = protocol::apply_diff_cold(mutation.diff(&base).diff(), &base).map_err(|error| error.to_string())?;
    let inverse_steps = mutation.inverse(&base).map_err(semio_framework_value::ValueError::into_message)?;
    let mut undone = applied.clone();
    for step in &inverse_steps {
        undone = protocol::apply_diff_cold(step.diff(&undone).diff(), &undone).map_err(|error| error.to_string())?;
    }
    let report = semio_framework_pack_json::json!({
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
#[path = "🧪️tests/🧬️direct/🦀️.rs"]
mod direct_leaf_contracts;


//#endregion 🧪️Tests
