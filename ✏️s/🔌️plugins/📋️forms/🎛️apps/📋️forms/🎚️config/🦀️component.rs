//! 🧮️ Forms play app — view state (`FormsConfig`) and its operation enum (`FormsConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.forms` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/wizard edits are VCS'd exactly like document
//! content. B1: absorbs every field that used to live on `forms_ui::FormsPlayApp`'s
//! `RefCell<FormsPlayRuntime>` (blueprint selection, the Try wizard's active step, its in-progress answer
//! values) plus `locale` (was read off `view_state.locale`) and `contributions_json` (was read off
//! `view_state.contributions_json` — the host-declared `Contribution::FormsQuestionKind` (legacy:
//! `PlaybookBlockKind`) list backing extension question kinds in the blueprint builder, try wizard, and
//! extension question rendering; the host now pushes contributions into config via
//! `SetContributions`, mirroring how it now pushes locale via `SetLocale`).

use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `FormsPlayApp::Config` — the pure-trait `DocumentApp::Config` for the forms app.
/// `try_values_json`/`contributions_json` are both heterogeneous JSON (per-question-kind value shapes; an
/// arbitrary `Contribution` list) with no single concrete `dsl`-typed shape, so both stay JSON-blob
/// strings — the same idiom `layout_engine::LayoutConfig`'s port-recipe sibling
/// (`LayoutDocument::data_fields_json`) and `shooting_protocol::ShootingCommand::SetFixtureJson` use for
/// "opaque JSON payload, never a document/config field type of its own" data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "formscfg")]
#[dsl(layout = "lines")]
pub struct FormsConfig {
    /// 👁️ Selected blueprint step/question ids — was `FormsPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 👁️ The Try wizard's active step index — was `FormsPlayRuntime::current_step_index`.
    pub current_step_index: u32,
    /// 👁️ The Try wizard's in-progress answer overrides (JSON object text, question id -> value) — was
    /// `FormsPlayRuntime::try_values`.
    pub try_values_json: String,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🧩️ Host-declared plugin contributions (JSON array of `{pluginId, contribution}`, only
    /// `Contribution::FormsQuestionKind` entries matter; legacy `PlaybookBlockKind` still accepted) — was read off `view_state.contributions_json`.
    pub contributions_json: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for FormsConfig {
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
impl store::DocumentPack for FormsConfig {
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


impl Default for FormsConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), current_step_index: 0, try_values_json: "{}".into(), locale: "en-US".into(), contributions_json: "[]".into() }
    }
}

store::impl_whole_record_config!(FormsConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS Config recipe: [`FormsConfig`]'s operation enum — mirrors
/// `shooting_op::ShootingConfigOperation`'s shape exactly: one variant per settled interaction (was a
/// `FormsPlayRuntime` field write pre-B1), plus a generic `Snapshot` every variant's `backwards()` returns.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum FormsConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: FormsConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "step-index")]
    SetStepIndex { index: u32 },
    #[dsl(key = "try-values")]
    SetTryValues { json: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for FormsConfigOperation {
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
impl protocol::OpBinary for FormsConfigOperation {
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


impl Operation<FormsConfig> for FormsConfigOperation {
    type Diff = FormsConfig;

    fn diff(&self, base: &FormsConfig) -> FormsConfig {
        let mut next = base.clone();
        match self {
            FormsConfigOperation::Snapshot { config } => return config.clone(),
            FormsConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            FormsConfigOperation::SetStepIndex { index } => next.current_step_index = *index,
            FormsConfigOperation::SetTryValues { json } => next.try_values_json = json.clone(),
            FormsConfigOperation::SetLocale { value } => next.locale = value.clone(),
            FormsConfigOperation::SetContributions { json } => next.contributions_json = json.clone(),
        }
        next
    }

    fn backwards(&self, base: &FormsConfig) -> Vec<Self> {
        vec![FormsConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_config_default_matches_the_existing_runtime_defaults() {
        let config = FormsConfig::default();
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.current_step_index, 0);
        assert_eq!(config.try_values_json, "{}");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.contributions_json, "[]");
    }

    #[test]
    fn forms_config_dsl_and_pack_round_trip() {
        let config = FormsConfig { selected_ids: vec!["q1".into(), "q2".into()], current_step_index: 2, try_values_json: r#"{"name":"Ada"}"#.into(), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    fn config_round_trip(base: &FormsConfig, operation: &FormsConfigOperation) -> FormsConfig {
        let forward = operation.diff(base);
        let backwards = operation.backwards(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_operations_apply_and_restore_every_field() {
        let base = FormsConfig::default();
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetSelection { ids: vec!["q1".into()] }).selected_ids, vec!["q1".to_string()]);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetStepIndex { index: 2 }).current_step_index, 2);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetTryValues { json: r#"{"a":1}"#.into() }).try_values_json, r#"{"a":1}"#);
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
        assert_eq!(config_round_trip(&base, &FormsConfigOperation::SetContributions { json: "[]".into() }).contributions_json, "[]");
    }

    #[test]
    fn config_snapshot_op_text_round_trips() {
        let config = FormsConfig { selected_ids: vec!["q1".into(), "q2".into()], current_step_index: 1, try_values_json: r#"{"name":"Ada"}"#.into(), locale: "de-DE".into(), contributions_json: "[]".into() };
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::Snapshot { config });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetSelection { ids: vec!["a".into()] });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetStepIndex { index: 3 });
        store::test_support::assert_op_line_round_trip(&FormsConfigOperation::SetLocale { value: "en-US".into() });
    }
}
//#endregion 🧪️Tests
