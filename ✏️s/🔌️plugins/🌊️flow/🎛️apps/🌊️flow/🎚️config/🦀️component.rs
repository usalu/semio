//! 🧮️ Flow play app — view state (`FlowConfig`) and its operation enum (`FlowConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.flow` document. It still round-trips through a real `DocumentStore`
//! (with a real `backwards`), so selection/camera/grid edits are VCS'd exactly like document content.

use crate::artifacts::flow::engine::{FLOW_DEFAULT_GRID_FACTOR, FLOW_DEFAULT_PROXIMITY_DISTANCE};
use flow_core::{CameraJson, FLOW_LOD_MODE_AUTOMATIC};
use playbook::GenerationPlayState;
use protocol::Operation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖️Config
/// 🧮️ `FlowPlayApp::Config` — the pure-trait `DocumentApp::Config` for the flow app. Absorbs everything
/// that used to live in an app-struct `RefCell` (`FlowPlayRuntime`) AND the locale the flow UI read off
/// the deleted host-pushed `ViewModel` — session-only view/generate-mode state now round-trips through the
/// config `DocumentStore` exactly like document content, with a real `backwards` per
/// [`FlowConfigOperation`] instead of never being VCS'd at all.
///
/// `automation_enabled_json`/`generation_json` hold JSON-encoded `HashMap<String, bool>`/
/// `playbook::GenerationPlayState` payloads rather than nested `#[dsl(block)]`/`#[dsl(table)]` fields:
/// none of those types derive `dsl::DslRecord`, mirroring `procedural_3d`'s identical `sun_json` escape
/// hatch for the same reason. Per-dispatch eval scratch uses a local `FlowEvalSession` in `handle` /
/// `pending_effects` / `render` (not process globals). `generation_json` stays config-tracked rather than becoming a
/// document operation (unlike the sibling `procedural_3d`/`procedural_2d` apps' `GenerationOperation`-backed
/// generations): flow's document model (`flow_core::FlowOperation`) is a shared kernel crate out of scope
/// for that conversion. `camera` stays a real `#[dsl(block)]` field since `flow_core::CameraJson` DOES
/// derive `dsl::DslRecord`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "flowcfg")]
#[dsl(layout = "lines")]
pub struct FlowConfig {
    /// 👁️ Selected widget ids.
    pub selected_node_ids: Vec<String>,
    /// 👁️ Selected synapse ids.
    pub selected_edge_ids: Vec<String>,
    /// 👁️ Selected handle ids.
    pub selected_handle_ids: Vec<String>,
    /// 👁️ Widget ids with their live-eval preview disabled.
    pub preview_off_node_ids: Vec<String>,
    /// 🎥️ The node-graph viewport camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 🎚️ LOD mode id (or `flow_core::FLOW_LOD_MODE_AUTOMATIC`).
    pub lod_mode: String,
    /// 🖱️ Proximity-select distance.
    pub proximity_distance: f64,
    /// 🔳️ Canvas grid visibility.
    pub grid_visible: bool,
    /// 🧲️ Canvas grid snap toggle.
    pub grid_snap_enabled: bool,
    /// 🔳️ Canvas grid factor.
    pub grid_factor: f64,
    /// 📚️ JSON-encoded extra catalogue sections.
    pub catalogue_sections_json: String,
    /// 🧩️ JSON-encoded `(extension id) -> enabled` map.
    pub automation_enabled_json: String,
    /// 🧩️ Host-pushed ProgramContributionEntry[] JSON for flow.extension hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
    /// 🧬️ JSON-encoded `playbook::GenerationPlayState` (Generate-mode exploration surface).
    pub generation_json: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for FlowConfig {
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
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for FlowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️DocumentCodec


impl Default for FlowConfig {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            selected_edge_ids: Vec::new(),
            selected_handle_ids: Vec::new(),
            preview_off_node_ids: Vec::new(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            lod_mode: FLOW_LOD_MODE_AUTOMATIC.into(),
            proximity_distance: FLOW_DEFAULT_PROXIMITY_DISTANCE,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: FLOW_DEFAULT_GRID_FACTOR,
            catalogue_sections_json: "[]".into(),
            automation_enabled_json: String::new(),
            contributions_json: "[]".into(),
            generation_json: String::new(),
            locale: "en-US".into(),
        }
    }
}

impl FlowConfig {
    /// 🧩️ Parses `automation_enabled_json` — falls back to an empty map.
    pub fn automation_enabled(&self) -> HashMap<String, bool> {
        serde_json::from_str(&self.automation_enabled_json).unwrap_or_default()
    }

    /// 🧬️ Parses `generation_json` — falls back to `GenerationPlayState::default()`.
    pub fn generation(&self) -> GenerationPlayState {
        serde_json::from_str(&self.generation_json).unwrap_or_default()
    }
}

store::impl_whole_record_config!(FlowConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`FlowConfig`]'s operation enum — one variant per settled interaction, plus a generic `Snapshot`
/// every variant's `backwards()` returns: since a config-only "View" dispatch is a plain `Apply` (not an
/// `AmendLast`), each tick is its own distinct, real config edit, and "undo this tick" is exactly "restore
/// the whole-config snapshot from just before it" — the simplest correct inverse, needing no per-field
/// reverse-patch bookkeeping. `Operation::Diff` is the WHOLE `FlowConfig` (not a granular patch type):
/// `diff()` returns "the full config after this op", and `store::impl_whole_record_config!` supplies the
/// `OperationDiff<FlowConfig>` that returns that snapshot verbatim, ignoring `base` — the same
/// "whole-record diff" shape the shooting/dag/procedural-3d config operations already use.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum FlowConfigOperation {
    /// 🧩️ Host-pushed contributions catalogue JSON.
    #[dsl(key = "contributions")]
    SetContributions { json: String },
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: FlowConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { node_ids: Vec<String>, edge_ids: Vec<String>, handle_ids: Vec<String> },
    #[dsl(key = "preview-off")]
    SetPreviewOff { node_ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "proximity-distance")]
    SetProximityDistance { value: f64 },
    #[dsl(key = "grid-visible")]
    SetGridVisible { value: bool },
    #[dsl(key = "grid-snap")]
    SetGridSnapEnabled { value: bool },
    #[dsl(key = "grid-factor")]
    SetGridFactor { value: f64 },
    #[dsl(key = "catalogue-sections")]
    SetCatalogueSections { sections_json: String },
    #[dsl(key = "extension-enabled")]
    SetAutomationEnabled { json: String },
    #[dsl(key = "generation")]
    SetGeneration { json: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for FlowConfigOperation {
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
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for FlowConfigOperation {
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


impl Operation<FlowConfig> for FlowConfigOperation {
    type Diff = FlowConfig;

    fn diff(&self, base: &FlowConfig) -> FlowConfig {
        let mut next = base.clone();
        match self {
            FlowConfigOperation::Snapshot { config } => return config.clone(),
            FlowConfigOperation::SetSelection { node_ids, edge_ids, handle_ids } => {
                next.selected_node_ids = node_ids.clone();
                next.selected_edge_ids = edge_ids.clone();
                next.selected_handle_ids = handle_ids.clone();
            }
            FlowConfigOperation::SetPreviewOff { node_ids } => next.preview_off_node_ids = node_ids.clone(),
            FlowConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            FlowConfigOperation::SetLodMode { value } => next.lod_mode = value.clone(),
            FlowConfigOperation::SetProximityDistance { value } => next.proximity_distance = *value,
            FlowConfigOperation::SetGridVisible { value } => next.grid_visible = *value,
            FlowConfigOperation::SetGridSnapEnabled { value } => next.grid_snap_enabled = *value,
            FlowConfigOperation::SetGridFactor { value } => next.grid_factor = *value,
            FlowConfigOperation::SetCatalogueSections { sections_json } => next.catalogue_sections_json = sections_json.clone(),
            FlowConfigOperation::SetAutomationEnabled { json } => next.automation_enabled_json = json.clone(),
            FlowConfigOperation::SetGeneration { json } => next.generation_json = json.clone(),
            FlowConfigOperation::SetContributions { json } => {
                next.contributions_json = json.clone();
                flow_core::sync_host_flow_extension_contributions(json);
            }
            FlowConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &FlowConfig) -> Vec<Self> {
        vec![FlowConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_config_default_matches_flow_play_runtime_defaults() {
        let config = FlowConfig::default();
        assert_eq!(config.camera, CameraJson { x: 0.0, y: 0.0, zoom: 1.0 });
        assert_eq!(config.lod_mode, FLOW_LOD_MODE_AUTOMATIC);
        assert_eq!(config.proximity_distance, FLOW_DEFAULT_PROXIMITY_DISTANCE);
        assert!(config.grid_visible);
        assert!(!config.grid_snap_enabled);
        assert_eq!(config.grid_factor, FLOW_DEFAULT_GRID_FACTOR);
        assert_eq!(config.catalogue_sections_json, "[]");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.automation_enabled(), HashMap::new());
        assert_eq!(config.generation(), GenerationPlayState::default());
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `FlowConfig`.
    #[test]
    fn flow_config_dsl_pack_round_trip() {
        let config = FlowConfig {
            selected_node_ids: vec!["n1".into(), "n2".into()],
            selected_edge_ids: vec!["e1".into()],
            selected_handle_ids: vec!["h1".into()],
            preview_off_node_ids: vec!["n2".into()],
            camera: CameraJson { x: 12.5, y: -3.0, zoom: 2.25 },
            lod_mode: "micro".into(),
            proximity_distance: 96.0,
            grid_visible: false,
            grid_snap_enabled: true,
            grid_factor: 5.0,
            catalogue_sections_json: "[{\"id\":\"custom\"}]".into(),
            automation_enabled_json: "{\"auto-layout\":true}".into(),
            contributions_json: "[]".into(),
            generation_json: "{\"generations\":[]}".into(),
            locale: "de-DE".into(),
        };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn flow_config_operation_text_binary_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::Snapshot { config: FlowConfig { selected_node_ids: vec!["n1".into()], locale: "de-DE".into(), ..FlowConfig::default() } });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetSelection { node_ids: vec!["n1".into(), "n2".into()], edge_ids: vec!["e1".into()], handle_ids: vec!["h1".into()] });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: Vec::new(), handle_ids: Vec::new() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetPreviewOff { node_ids: vec!["n1".into()] });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetLodMode { value: "micro".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetProximityDistance { value: 48.0 });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetGridVisible { value: true });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetGridSnapEnabled { value: false });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetGridFactor { value: 10.0 });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetCatalogueSections { sections_json: "[]".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetAutomationEnabled { json: "{\"auto-layout\":true}".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetGeneration { json: "{\"generations\":[]}".into() });
        store::test_support::assert_op_line_round_trip(&FlowConfigOperation::SetLocale { value: "de-DE".into() });
    }

    #[test]
    fn flow_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = FlowConfig { selected_node_ids: vec!["n1".into()], locale: "en-US".into(), ..FlowConfig::default() };
        let operation = FlowConfigOperation::SetSelection { node_ids: vec!["n2".into()], edge_ids: Vec::new(), handle_ids: Vec::new() };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_node_ids, vec!["n2".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![FlowConfigOperation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
