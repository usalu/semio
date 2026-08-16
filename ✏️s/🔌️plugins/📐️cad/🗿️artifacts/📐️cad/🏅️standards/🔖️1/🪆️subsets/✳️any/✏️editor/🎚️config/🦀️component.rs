//! 🧮️ CAD app — `ArtifactApp::Config`: every field that used to live in the app struct's ephemeral
//! `CadPlayRuntime` (selection, hover, engagement session, per-pane cameras, sun, dislocate handles)
//! plus the locale/terminology/active-utility the shell used to push through the deleted `ViewModel`.
//! Session view state round-trips through the config `ArtifactStore` exactly like document content,
//! with a real `backwards` via `CadConfigMutation` at the bottom of this file.

use crate::artifacts::cad::CadCamera;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🎛️ Per-pane handle groups exposed by the Dislocate gumball utility — was keyed by an arbitrary
/// host-pushed `ViewModel.window_id` (`cad_ui::CadPlayRuntime::dislocate_options_by_window_id`); the
/// pure `ArtifactApp::render`/`window_measures` surface has no per-window-instance parameter anymore
/// (only `body_key`, which already resolves 1:1 to one of the 4 fixed CAD panes), so `CadConfig` keys
/// this by PANE instead — one named field per pane, mirroring `camera`/`camera_building`/…
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadDislocateOptions {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

impl Default for CadDislocateOptions {
    fn default() -> Self {
        Self { move_enabled: true, rotate_enabled: true }
    }
}

/// 🌞️ Local `dsl::DslRecord`-able mirror of `semio_framework_plugin::WorldSunConfig` (foreign,
/// out-of-scope crate — cannot gain a `dsl` derive there). `cad_sun_config_from_world`/
/// `cad_sun_config_to_world` convert at the boundary; field-for-field identical otherwise.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CadSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

impl Default for CadSunConfig {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
    }
}

pub fn cad_sun_config_from_world(sun: &semio_framework_plugin::WorldSunConfig) -> CadSunConfig {
    CadSunConfig { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color.clone() }
}

pub fn cad_sun_config_to_world(sun: &CadSunConfig) -> semio_framework_plugin::WorldSunConfig {
    semio_framework_plugin::WorldSunConfig { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color.clone() }
}

/// 🧮️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: cad's real `ArtifactApp::Config` — see the region doc
/// comment above for the full absorption story.
/// `engagement_session_json` is the pre-serialized JSON of `cad_document_engine::interaction::
/// CadEngagementScratch` — that type's `context: HashMap<String, Value>` field has no `dsl` shape
/// (arbitrary JSON), so it round-trips as an opaque string rather than a nested `#[dsl(block)]`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "cadcfg")]
#[dsl(id = "cad.config")]
#[dsl(layout = "lines")]
pub struct CadConfig {
    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh object/vertex/edge/face
    /// selection AND hover are now the framework-owned `"cad"` interaction domain
    /// (`InteractionView::selection("cad")`/`.hover("cad", "pointer")`) — `selected_object_ids`,
    /// `selection_method`, `hovered_object_id`, `hovered_target`, `active_object_id` (now
    /// `DomainSelection::anchor_id`) and `component_selection` are DELETED, not migrated in place.
    /// 👁️ Node (document-tree) selection stays app-owned — not a mesh-geometry granularity.
    pub selected_node_ids: Vec<String>,
    /// 🐁️ Hovered reference-overlay id (per-pane background image) — app-owned, distinct from the
    /// framework `"cad"` domain's mesh hover; was piggy-backed onto the deleted `hovered_object_id`
    /// via a `"reference:"` string prefix, now its own field.
    pub hovered_reference_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 👁️ Was `CadPlayRuntime::engagement_step`.
    pub engagement_step: String,
    /// 👁️ Was `CadPlayRuntime::active_example_id`.
    pub active_example_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_reference_model_definition_id`.
    pub selected_reference_model_definition_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_reference_id`.
    pub selected_reference_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::engagement_pane`.
    pub engagement_pane: Option<String>,
    /// 👁️ Was `CadPlayRuntime::engagement_session` (`Option<CadEngagementScratch>`) — see the struct
    /// doc comment for why this is an opaque JSON string here.
    pub engagement_session_json: Option<String>,
    /// 👁️ Was `CadPlayRuntime::last_finalized_interaction_id`.
    pub last_finalized_interaction_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::sun` (`WorldSunConfig`).
    #[dsl(block)]
    pub sun: CadSunConfig,
    /// 🎥️ Per-pane camera pose — was `CadPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_building`.
    #[dsl(block)]
    pub camera_building: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_energy`.
    #[dsl(block)]
    pub camera_energy: CadCamera,
    /// 🎥️ Was `CadPlayRuntime::camera_structure_classic`.
    #[dsl(block)]
    pub camera_structure_classic: CadCamera,
    /// 🎛️ Was `CadPlayRuntime::dislocate_options_by_window_id.get(CAD_PLAY_WINDOW_SHAPE)` — see
    /// `CadDislocateOptions`'s doc comment for the per-window-id → per-pane simplification.
    #[dsl(block)]
    pub dislocate_shape: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_building: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_energy: CadDislocateOptions,
    #[dsl(block)]
    pub dislocate_structure_classic: CadDislocateOptions,
    /// 🧰️ The active transform-gumball utility — was read off `view_state.active_utility_id`
    /// (host-pushed `ViewModel`, deleted by B1).
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🗣️ Terminology id (`"native"`/`"reuse"`) — was read off `view_state.terminology`.
    pub terminology: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `cad.computer` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for CadConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
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
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for CadConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec


fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for CadConfig {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            hovered_reference_id: None,
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            active_example_id: None,
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            engagement_pane: None,
            engagement_session_json: None,
            last_finalized_interaction_id: None,
            sun: CadSunConfig::default(),
            camera: CadCamera::default(),
            camera_building: CadCamera::default(),
            camera_energy: CadCamera::default(),
            camera_structure_classic: CadCamera::default(),
            dislocate_shape: CadDislocateOptions::default(),
            dislocate_building: CadDislocateOptions::default(),
            dislocate_energy: CadDislocateOptions::default(),
            dislocate_structure_classic: CadDislocateOptions::default(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
            terminology: "native".into(),
            contributions_json: default_contributions_json(),
        }
    }
}

store::impl_whole_record_config!(CadConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// @emoji 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS config recipe: `CadConfig`'s
/// operation enum. Unlike `CadMutation` (many narrow document-mutating variants), this is a single
/// whole-record `Snapshot`: `cad_ui`'s pure `handle()` converts its (former `RefCell`-backed)
/// `CadPlayRuntime` scratch struct into the next `CadConfig` once per dispatch and diffs it against the
/// pre-command config, the same "whole-record replace" shape `reset_document_effect` now uses for
/// a whole-DOCUMENT replace (SEMANTIC-MUTATIONS-OVERHAUL retired `CadMutation::SetSnapshot`
/// entirely; document-level whole-content replace is no longer an in-history mutation at all) —
/// session state (selection/hover/camera/engagement/…) mutates in tight clusters (e.g.
/// `worldSelect` touches 5+ fields together), so per-field variants would just be wide-argument
/// snapshots in miniature with none of a real granular diff's benefit. `backwards()` restores the
/// exact pre-command `CadConfig`, giving real, exact undo without any per-field reverse-patch
/// bookkeeping — the same justification `shooting_op::ShootingConfigOperation` documents for its own
/// `Snapshot` fallback, generalized here to the sole variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum CadConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: CadConfig,
    },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for CadConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
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

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for CadConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}

//#endregion 🔖️OpCodec


impl Mutation<CadConfig> for CadConfigMutation {
    type Diff = CadConfig;

    fn diff(&self, base: &CadConfig) -> protocol::MutationOutcome<CadConfig> {
        match self {
            CadConfigMutation::Snapshot { config } => {
                if config == base {
                    return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Config snapshot is already up to date.");
                }
                protocol::MutationOutcome::new(config.clone())
            }
            CadConfigMutation::SetContributions { json } => {
                if &base.contributions_json == json {
                    return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Contributions are already up to date.");
                }
                let mut next = base.clone();
                next.contributions_json = json.clone();
                crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::sync_cad_computer_contributions(json);
                protocol::MutationOutcome::new(next)
            }
        }
    }

    fn inverse(&self, base: &CadConfig) -> Vec<Self> {
        vec![CadConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cad_config_default_matches_the_existing_runtime_defaults() {
        let config = CadConfig::default();
        assert_eq!(config.engagement_step, "Idle");
        assert_eq!(config.active_utility_id, "move");
        assert_eq!(config.locale, "en-US");
        assert!(config.dislocate_shape.move_enabled);
        assert!(config.dislocate_shape.rotate_enabled);
    }

    #[test]
    fn cad_config_dsl_round_trips_a_populated_record() {
        let config = CadConfig {
            selected_node_ids: vec!["node-1".into(), "node-2".into()],
            hovered_reference_id: Some("ref-1".into()),
            engagement_session_json: Some("{\"interactionId\":\"box\"}".into()),
            active_utility_id: "rotate".into(),
            locale: "de-DE".into(),
            camera: CadCamera { position: [1.0, 2.0, 3.0], ..CadCamera::default() },
            ..CadConfig::default()
        };
        let text = store::ArtifactDsl::print_dsl(&config);
        let parsed = <CadConfig as store::ArtifactDsl>::parse_dsl(&text).expect("cad config dsl parses");
        assert_eq!(parsed, config);
    }

    #[test]
    fn cad_config_pack_round_trips() {
        let mut config = CadConfig { selected_node_ids: vec!["node-1".into()], ..CadConfig::default() };
        config.dislocate_building.rotate_enabled = false;
        let bytes = store::ArtifactPack::encode_pack(&config);
        let decoded = <CadConfig as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, config);
    }

    #[test]
    fn cad_sun_config_round_trips_through_world_sun_config() {
        let world = semio_framework_plugin::WorldSunConfig { enabled: true, azimuth: 12.0, elevation: 34.0, intensity: 0.5, color: "#112233".into() };
        let cad_sun = cad_sun_config_from_world(&world);
        let back = cad_sun_config_to_world(&cad_sun);
        assert_eq!(back, world);
    }

    #[test]
    fn cad_config_operation_snapshot_round_trips_and_restores_exactly() {
        let base = CadConfig { active_utility_id: "move".into(), ..CadConfig::default() };
        let next = CadConfig { active_utility_id: "rotate".into(), selected_node_ids: vec!["node-1".into()], ..CadConfig::default() };
        let operation = CadConfigMutation::Snapshot { config: next.clone() };
        let forward = operation.diff(&base).diff().clone();
        assert_eq!(forward, next);
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![CadConfigMutation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward).diff().clone();
        assert_eq!(restored, base);
        store::os_store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn cad_config_set_contributions_round_trips() {
        let base = CadConfig::default();
        let json = r#"[{"pluginId":"cad-extension-spatial-shape","contribution":{"kind":"cadComputer","appId":"cad-play","moduleId":"spatial-shape","label":"Spatial Shape","iconId":"box","computersJson":"{}"}}]"#;
        let operation = CadConfigMutation::SetContributions { json: json.into() };
        let next = operation.diff(&base).diff().clone();
        assert_eq!(next.contributions_json, json);
        store::os_store::test_support::assert_op_line_round_trip(&operation);
    }
}
//#endregion 🧪️Tests
