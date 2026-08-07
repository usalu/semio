//! 🧮️ Procedural2d play app — view state (`Procedural2dConfig`) and its operation enum
//! (`Procedural2dConfigOperation`).
//!
//! This is APP state, not document state: selection, camera, show-mode and the derived generation
//! preview live here rather than under `🗿️artifacts/`, since none of it survives into the `.procedural2d`
//! document. It still round-trips through a real `DocumentStore` (with a real `backwards`), so every
//! edit is VCS'd exactly like document content.

use flow_core::CameraJson;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `Procedural2dPlayApp::Config` — the pure-trait config artifact. Selection, the graph camera, the
/// show-mode display toggle, the derived generation selection/preview, and locale all round-trip
/// through the config `DocumentStore` exactly like document content, with a real `backwards` per
/// [`Procedural2dConfigOperation`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "procedural2dcfg")]
#[dsl(layout = "lines")]
pub struct Procedural2dConfig {
    /// 👁️ Selected widget ids.
    pub selected_ids: Vec<String>,
    /// 🗺️ The node-graph camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 👁️ Display mode (`"preview"`/`"generate"`/`"wire"`).
    pub show_mode: String,
    /// 👁️ Active generation selection.
    pub selected_generation_id: Option<String>,
    /// 👁️ Derived generation preview text.
    pub generation_preview_text: Option<String>,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for Procedural2dConfig {
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
impl store::DocumentPack for Procedural2dConfig {
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


impl Default for Procedural2dConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, show_mode: default_show_mode(), selected_generation_id: None, generation_preview_text: None, locale: "en-US".into() }
    }
}

pub fn default_show_mode() -> String {
    "preview".into()
}

store::impl_whole_record_config!(Procedural2dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`Procedural2dConfig`]'s operation enum — one variant per settled config write, plus a generic
/// `Snapshot` every variant's `backwards()` returns (each config tick is its own distinct edit, so
/// "undo this tick" is "restore the whole-config snapshot from just before it").
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Procedural2dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Procedural2dConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "show-mode")]
    SetShowMode { value: String },
    #[dsl(key = "generation")]
    SetGeneration { selected_generation_id: Option<String>, generation_preview_text: Option<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for Procedural2dConfigOperation {
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
impl protocol::OpBinary for Procedural2dConfigOperation {
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


impl Operation<Procedural2dConfig> for Procedural2dConfigOperation {
    type Diff = Procedural2dConfig;

    fn diff(&self, base: &Procedural2dConfig) -> Procedural2dConfig {
        let mut next = base.clone();
        match self {
            Procedural2dConfigOperation::Snapshot { config } => return config.clone(),
            Procedural2dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Procedural2dConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            Procedural2dConfigOperation::SetShowMode { value } => next.show_mode = value.clone(),
            Procedural2dConfigOperation::SetGeneration { selected_generation_id, generation_preview_text } => {
                next.selected_generation_id = selected_generation_id.clone();
                next.generation_preview_text = generation_preview_text.clone();
            }
            Procedural2dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &Procedural2dConfig) -> Vec<Self> {
        vec![Procedural2dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_set_selection_round_trips_and_restores() {
        let base = Procedural2dConfig::default();
        let operation = Procedural2dConfigOperation::SetSelection { ids: vec!["w1".into(), "w2".into()] };
        let forward = operation.diff(&base);
        assert_eq!(forward.selected_ids, vec!["w1".to_string(), "w2".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards[0].diff(&forward), base);
    }

    #[test]
    fn config_set_camera_round_trips_and_restores() {
        let base = Procedural2dConfig::default();
        let camera = CameraJson { x: 9.0, y: -3.0, zoom: 2.5 };
        let forward = Procedural2dConfigOperation::SetCamera { camera: camera.clone() }.diff(&base);
        assert_eq!(forward.camera, camera);
    }

    #[test]
    fn config_set_show_mode_round_trips_and_restores() {
        let base = Procedural2dConfig::default();
        let forward = Procedural2dConfigOperation::SetShowMode { value: "wire".into() }.diff(&base);
        assert_eq!(forward.show_mode, "wire");
    }

    #[test]
    fn config_set_locale_round_trips_and_restores() {
        let base = Procedural2dConfig::default();
        let forward = Procedural2dConfigOperation::SetLocale { value: "de-DE".into() }.diff(&base);
        assert_eq!(forward.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        let config = Procedural2dConfig { selected_ids: vec!["a".into()], locale: "de-DE".into(), ..Procedural2dConfig::default() };
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetSelection { ids: vec!["a".into(), "b".into()] });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetShowMode { value: "generate".into() });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetGeneration { selected_generation_id: None, generation_preview_text: None });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetGeneration { selected_generation_id: Some("g1".into()), generation_preview_text: None });
        store::test_support::assert_op_line_round_trip(&Procedural2dConfigOperation::SetLocale { value: "en-US".into() });
    }
}
//#endregion 🧪️Tests
