//! 🧮️ Shooting play app — view state (`ShootingConfig`) and its operation enum
//! (`ShootingConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.shooting` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/camera/utility edits are VCS'd exactly like
//! document content.

use crate::artifacts::shooting::ShootingCamera;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: shooting's real `ArtifactApp::Config` — the pure-trait pilot's config artifact. Absorbs
/// both the old sticky `ActionArgDef` defaults (`default_shot_format`/`shape`/`default_asset_format`)
/// AND everything that used to live in an app-struct `RefCell` runtime (selection, hover, selection
/// method, center-model toggle, fit-revision counter, camera draft label, and the free/live viewport
/// camera) — session-only view state now round-trips through the config `ArtifactStore` exactly like
/// document content, with a real `backwards` per [`ShootingConfigMutation`] instead of never being
/// VCS'd at all. `locale`/`active_utility_id` are the two view-state fields the shooting UI actually
/// reads (`resolve_labels`/the transform-gumball utility) — see `crate::apps::shooting::render`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "shooting.config")]
#[dsl(id = "shooting.config")]
#[dsl(layout = "lines")]
pub struct ShootingConfig {
    /// 🖼️ Mirrors `addShot`'s `format` `ActionArgDef` default (`"png"`).
    pub default_shot_format: String,
    /// 🖼️ Mirrors `addShot`'s `shape` `ActionArgDef` default (`"rectangle"`).
    pub default_shot_shape: String,
    /// 🧱️ Mirrors `addAsset`'s `format` `ActionArgDef` default (`"glb"`).
    pub default_asset_format: String,
    /// 👁️ Selected shot ids (gallery/document-tree multi-select) — genuinely app-specific, NOT part of
    /// the framework-owned `"assets"` interaction domain (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): asset selection/hover dissolved into that
    /// domain, but shots have no world-3d pick/marquee/hover surface of their own, so shot selection
    /// stays a plain config field, set via [`ShootingConfigMutation::SetShotSelection`].
    pub selected_shot_ids: Vec<String>,
    /// 👁️ "Center model in viewport" toggle.
    pub center_model: bool,
    /// 👁️ Bumped whenever the active asset changes to re-trigger a viewport fit.
    pub fit_revision: u32,
    /// 👁️ In-progress "save camera" label draft.
    pub camera_draft_label: String,
    /// 🎥️ The free/live viewport camera — session-only, never a document field.
    #[dsl(block)]
    pub camera: ShootingCamera,
    /// 🧰️ The active transform-gumball utility for the scene window.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for ShootingConfig {
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
impl store::ArtifactPack for ShootingConfig {
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


impl Default for ShootingConfig {
    fn default() -> Self {
        Self {
            default_shot_format: "png".into(),
            default_shot_shape: "rectangle".into(),
            default_asset_format: "glb".into(),
            selected_shot_ids: Vec::new(),
            center_model: true,
            fit_revision: 0,
            camera_draft_label: String::new(),
            camera: ShootingCamera::default(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(ShootingConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ B1: [`ShootingConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// ephemeral runtime field writes), plus a generic `Snapshot` every variant's `backwards()` returns:
/// since a config-only "View" dispatch is a plain `Apply` (not an `AmendLast`), each tick is its own
/// distinct, real config edit, and "undo this tick" is exactly "restore the whole-config snapshot from
/// just before it" — the simplest correct inverse, needing no per-field reverse-patch bookkeeping.
/// `Mutation::Diff` is the WHOLE `ShootingConfig` (not a granular patch type, unlike `ShootingDiff`):
/// `diff()` returns "the full config after this op", and `store::impl_whole_record_config!` supplies the
/// `MutationDiff<ShootingConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[allow(clippy::large_enum_variant, reason = "Snapshot{config: ShootingConfig} mirrors the pre-migration shape verbatim (a whole-record config snapshot, not a size regression this migration introduced); boxing it would change the wire shape")]
pub enum ShootingConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: ShootingConfig,
    },
    /// 🎞️ Sets the gallery/document-tree shot selection — the sole remaining app-owned selection
    /// field (see [`ShootingConfig::selected_shot_ids`]'s doc comment: asset selection/hover moved to
    /// the framework-owned `"assets"` interaction domain, shot selection did not).
    #[dsl(key = "shot-selection")]
    SetShotSelection { shot_ids: Vec<String> },
    #[dsl(key = "center-model")]
    SetCenterModel { value: bool },
    #[dsl(key = "fit-revision")]
    SetFitRevision { value: u32 },
    #[dsl(key = "camera-draft-label")]
    SetCameraDraftLabel { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: ShootingCamera,
    },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "defaults")]
    SetDefaults { shot_format: String, shot_shape: String, asset_format: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for ShootingConfigMutation {
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
impl protocol::OpBinary for ShootingConfigMutation {
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


impl Mutation<ShootingConfig> for ShootingConfigMutation {
    type Diff = ShootingConfig;

    fn diff(&self, base: &ShootingConfig) -> ShootingConfig {
        let mut next = base.clone();
        match self {
            ShootingConfigMutation::Snapshot { config } => return config.clone(),
            ShootingConfigMutation::SetShotSelection { shot_ids } => next.selected_shot_ids = shot_ids.clone(),
            ShootingConfigMutation::SetCenterModel { value } => next.center_model = *value,
            ShootingConfigMutation::SetFitRevision { value } => next.fit_revision = *value,
            ShootingConfigMutation::SetCameraDraftLabel { value } => next.camera_draft_label = value.clone(),
            ShootingConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            ShootingConfigMutation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            ShootingConfigMutation::SetLocale { value } => next.locale = value.clone(),
            ShootingConfigMutation::SetDefaults { shot_format, shot_shape, asset_format } => {
                next.default_shot_format = shot_format.clone();
                next.default_shot_shape = shot_shape.clone();
                next.default_asset_format = asset_format.clone();
            }
        }
        next
    }

    fn inverse(&self, base: &ShootingConfig) -> Vec<Self> {
        vec![ShootingConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shooting_config_default_matches_the_existing_action_arg_sticky_defaults() {
        let config = ShootingConfig::default();
        assert_eq!(config.default_shot_format, "png");
        assert_eq!(config.default_shot_shape, "rectangle");
        assert_eq!(config.default_asset_format, "glb");
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `ShootingConfig`.
    #[test]
    fn shooting_config_dsl_pack_round_trip() {
        let config = ShootingConfig {
            selected_shot_ids: vec!["s1".into()],
            center_model: false,
            fit_revision: 3,
            camera_draft_label: "Hero".into(),
            camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() },
            active_utility_id: "rotate".into(),
            locale: "de-DE".into(),
            ..ShootingConfig::default()
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn shooting_config_operation_text_binary_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::Snapshot { config: ShootingConfig { selected_shot_ids: vec!["s1".into()], locale: "de-DE".into(), ..ShootingConfig::default() } });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetShotSelection { shot_ids: vec!["s1".into(), "s2".into()] });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCenterModel { value: true });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetFitRevision { value: 4 });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCameraDraftLabel { value: "Hero".into() });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetActiveUtility { utility_id: "rotate".into() });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetLocale { value: "de-DE".into() });
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetDefaults { shot_format: "svg".into(), shot_shape: "ellipse".into(), asset_format: "glb".into() });
    }

    #[test]
    fn shooting_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = ShootingConfig { selected_shot_ids: vec!["s1".into()], locale: "en-US".into(), ..ShootingConfig::default() };
        let operation = ShootingConfigMutation::SetShotSelection { shot_ids: vec!["s2".into()] };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_shot_ids, vec!["s2".to_string()]);
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![ShootingConfigMutation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
