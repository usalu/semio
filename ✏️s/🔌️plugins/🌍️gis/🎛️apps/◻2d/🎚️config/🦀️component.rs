//! 🧮️ GIS 2D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: it round-trips through the config `ArtifactStore` exactly
//! like document content, with a true `backwards` per operation. Nothing here is document state — the
//! map's positions/routes/regions live in `crate::artifacts::gismap`.

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Config
/// 🧮️ gis2d's `ArtifactApp::Config` — per-layer visibility/stroke-weight, camera, render/vector/LOD
/// mode, plus `locale`. Layer AND feature selection/hover/method/mode moved to the framework-owned
/// `"features"` interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// read via `InteractionView::selection("features")`/`.hover("features", "pointer")`, never stored
/// here again. Per-layer maps are `BTreeMap` (not `HashMap`) because the DSL derive only binds
/// string-keyed maps through `dsl_schema::Shape::Map`'s `BTreeMap<String, V>` case.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "gis2dcfg")]
#[dsl(id = "gis.gis2dcfg")]
#[dsl(layout = "lines")]
pub struct Gis2dConfig {
    /// 👁️ Per-layer visibility; a missing entry defaults to visible.
    #[dsl(block)]
    pub layer_visibility: BTreeMap<String, bool>,
    /// 🎥️ The free/live map camera (`{x,y,zoom}` JSON).
    pub camera_json: String,
    /// 🖼️ `"image" | "vector" | "combined"`.
    pub render_mode: String,
    /// 🎨️ `"colored" | "figureGround" | "invertedFigure"`.
    pub vector_style: String,
    /// 🔽️ Active LOD tier id.
    pub lod_mode: String,
    /// 👁️ Per-layer stroke-weight multiplier; a missing entry defaults to `1.0`.
    #[dsl(block)]
    pub layer_stroke_scale: BTreeMap<String, f64>,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Gis2dConfig {
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
impl store::ArtifactPack for Gis2dConfig {
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


fn default_gis2d_camera_json() -> String {
    r#"{"x":0,"y":0,"zoom":1}"#.into()
}

fn default_gis2d_render_mode() -> String {
    "combined".into()
}

fn default_gis2d_vector_style() -> String {
    "colored".into()
}

impl Default for Gis2dConfig {
    fn default() -> Self {
        Self {
            layer_visibility: BTreeMap::new(),
            camera_json: default_gis2d_camera_json(),
            render_mode: default_gis2d_render_mode(),
            vector_style: default_gis2d_vector_style(),
            // 🔽️ Mirrors `framework_surface::tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC`, spelled out here so
            // the config type stays independent of the tiled-map surface crate.
            lod_mode: "automatic".into(),
            layer_stroke_scale: BTreeMap::new(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(Gis2dConfig);

/// 👁️ Whether a map layer is currently shown; a layer with no explicit entry defaults to visible.
pub fn layer_visible(cfg: &Gis2dConfig, layer_id: &str) -> bool {
    cfg.layer_visibility.get(layer_id).copied().unwrap_or(true)
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `Gis2dConfig`'s operation enum — one variant per settled interaction; each variant's
/// `backwards()` re-emits the SAME variant with the old field value read from `base` (no
/// whole-config snapshot sentinel). `Mutation::Diff` is the WHOLE `Gis2dConfig` (not a granular
/// patch type): `diff()` returns "the full config after this op", and
/// `MutationDiff<Gis2dConfig>::apply` for `Gis2dConfig` itself (generated by
/// `store::impl_whole_record_config!`) just returns that snapshot verbatim, ignoring `base`.
/// `SetLayerVisibility`/`SetLayerStrokeScale` remove the map entry when the written value equals
/// the field's default (`true` / `1.0`) rather than storing it explicitly — this keeps "missing"
/// and "explicitly default" the SAME map state, which is what makes their inverse (re-emitting the
/// old value, defaulted via `unwrap_or`) byte-exact even when the pre-operation map had no entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Gis2dConfigMutation {
    #[dsl(key = "layer-visibility")]
    SetLayerVisibility { layer_id: String, visible: bool },
    #[dsl(key = "camera")]
    SetCamera { camera_json: String },
    #[dsl(key = "render-mode")]
    SetRenderMode { value: String },
    #[dsl(key = "vector-style")]
    SetVectorStyle { value: String },
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "layer-stroke-scale")]
    SetLayerStrokeScale { layer_id: String, value: f64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for Gis2dConfigMutation {
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
impl protocol::OpBinary for Gis2dConfigMutation {
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


impl Mutation<Gis2dConfig> for Gis2dConfigMutation {
    type Diff = Gis2dConfig;

    fn diff(&self, base: &Gis2dConfig) -> protocol::MutationOutcome<Gis2dConfig> {
        let mut next = base.clone();
        match self {
            Gis2dConfigMutation::SetLayerVisibility { layer_id, visible } => {
                if base.layer_visibility.get(layer_id).copied().unwrap_or(true) == *visible {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" visibility is already {}.", layer_id, visible));
                }
                if *visible {
                    next.layer_visibility.remove(layer_id);
                } else {
                    next.layer_visibility.insert(layer_id.clone(), *visible);
                }
            }
            Gis2dConfigMutation::SetCamera { camera_json } => {
                if &base.camera_json == camera_json {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", "Camera is already at the requested position.");
                }
                next.camera_json = camera_json.clone();
            }
            Gis2dConfigMutation::SetRenderMode { value } => {
                if &base.render_mode == value {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Render mode is already \"{}\".", value));
                }
                next.render_mode = value.clone();
            }
            Gis2dConfigMutation::SetVectorStyle { value } => {
                if &base.vector_style == value {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Vector style is already \"{}\".", value));
                }
                next.vector_style = value.clone();
            }
            Gis2dConfigMutation::SetLodMode { value } => {
                if &base.lod_mode == value {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("LOD mode is already \"{}\".", value));
                }
                next.lod_mode = value.clone();
            }
            Gis2dConfigMutation::SetLayerStrokeScale { layer_id, value } => {
                if base.layer_stroke_scale.get(layer_id).copied().unwrap_or(1.0) == *value {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Layer \"{}\" stroke scale is already {}.", layer_id, value));
                }
                if *value == 1.0 {
                    next.layer_stroke_scale.remove(layer_id);
                } else {
                    next.layer_stroke_scale.insert(layer_id.clone(), *value);
                }
            }
            Gis2dConfigMutation::SetLocale { value } => {
                if &base.locale == value {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Locale is already \"{}\".", value));
                }
                next.locale = value.clone();
            }
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Gis2dConfig) -> Vec<Self> {
        match self {
            Gis2dConfigMutation::SetLayerVisibility { layer_id, .. } => {
                vec![Gis2dConfigMutation::SetLayerVisibility { layer_id: layer_id.clone(), visible: base.layer_visibility.get(layer_id).copied().unwrap_or(true) }]
            }
            Gis2dConfigMutation::SetCamera { .. } => vec![Gis2dConfigMutation::SetCamera { camera_json: base.camera_json.clone() }],
            Gis2dConfigMutation::SetRenderMode { .. } => vec![Gis2dConfigMutation::SetRenderMode { value: base.render_mode.clone() }],
            Gis2dConfigMutation::SetVectorStyle { .. } => vec![Gis2dConfigMutation::SetVectorStyle { value: base.vector_style.clone() }],
            Gis2dConfigMutation::SetLodMode { .. } => vec![Gis2dConfigMutation::SetLodMode { value: base.lod_mode.clone() }],
            Gis2dConfigMutation::SetLayerStrokeScale { layer_id, .. } => {
                vec![Gis2dConfigMutation::SetLayerStrokeScale { layer_id: layer_id.clone(), value: base.layer_stroke_scale.get(layer_id).copied().unwrap_or(1.0) }]
            }
            Gis2dConfigMutation::SetLocale { .. } => vec![Gis2dConfigMutation::SetLocale { value: base.locale.clone() }],
        }
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gis2d_config_default_matches_the_existing_action_arg_sticky_defaults() {
        let config = Gis2dConfig::default();
        assert_eq!(config.render_mode, "combined");
        assert_eq!(config.vector_style, "colored");
        assert_eq!(config.lod_mode, "automatic");
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn gis2d_config_default_lod_mode_matches_the_tiled_map_surface_constant() {
        assert_eq!(Gis2dConfig::default().lod_mode, framework_surface::tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC);
    }

    #[test]
    fn layer_visible_defaults_to_true_and_honours_explicit_entries() {
        let mut config = Gis2dConfig::default();
        assert!(layer_visible(&config, "water"), "a layer with no entry is visible");
        config.layer_visibility.insert("water".into(), false);
        assert!(!layer_visible(&config, "water"));
    }

    #[test]
    fn gis2d_config_dsl_round_trips_default_and_populated() {
        store::os_store::test_support::assert_dsl_round_trip(&Gis2dConfig::default());
        let mut populated = Gis2dConfig::default();
        populated.layer_visibility.insert("water".into(), false);
        populated.layer_stroke_scale.insert("roads".into(), 1.5);
        store::os_store::test_support::assert_dsl_round_trip(&populated);
        store::os_store::test_support::assert_dsl_pack_equivalence(&populated);
    }

    #[test]
    fn gis2d_config_operation_diff_writes_the_targeted_field_and_leaves_the_rest() {
        let base = Gis2dConfig::default();
        let next = Gis2dConfigMutation::SetRenderMode { value: "vector".into() }.diff(&base).diff().clone();
        assert_eq!(next.render_mode, "vector");
        assert_eq!(next.vector_style, base.vector_style, "untouched fields survive the diff");
    }

    #[test]
    fn gis2d_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Gis2dConfig::default();
        let operation = Gis2dConfigMutation::SetLayerVisibility { layer_id: "water".into(), visible: false };
        let next = operation.diff(&base).diff().clone();
        assert_eq!(next.layer_visibility.get("water"), Some(&false));
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![Gis2dConfigMutation::SetLayerVisibility { layer_id: "water".into(), visible: true }]);
        let restored = backwards[0].diff(&next).diff().clone();
        assert_eq!(restored, base, "the per-field inverse restores the exact pre-operation config, including the absent map entry");
    }

    /// ⚖️ `SetLayerStrokeScale`'s inverse has the same absent-entry-vs-default subtlety as
    /// `SetLayerVisibility` above, covered separately since it defaults to `1.0` not `true`.
    #[test]
    fn gis2d_config_layer_stroke_scale_backwards_restores_an_absent_entry() {
        let base = Gis2dConfig::default();
        let operation = Gis2dConfigMutation::SetLayerStrokeScale { layer_id: "roads".into(), value: 2.0 };
        let next = operation.diff(&base).diff().clone();
        assert_eq!(next.layer_stroke_scale.get("roads"), Some(&2.0));
        let backwards = operation.inverse(&base);
        let restored = backwards[0].diff(&next).diff().clone();
        assert_eq!(restored, base);
        assert!(!restored.layer_stroke_scale.contains_key("roads"));
    }

    #[test]
    fn gis2d_config_operation_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLayerVisibility { layer_id: "water".into(), visible: false });
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetCamera { camera_json: r#"{"x":1,"y":2,"zoom":3}"#.into() });
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetRenderMode { value: "vector".into() });
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetVectorStyle { value: "figureGround".into() });
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLodMode { value: "automatic".into() });
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLayerStrokeScale { layer_id: "roads".into(), value: 1.5 });
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
