//! 🧮️ Wires play app — view state (`WiresConfig`) and its operation enum (`WiresConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.wires` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/drag/locale edits are VCS'd exactly like
//! document content. Absorbs everything that used to live in the pre-B1 `ReasoningWiresPlayApp`'s
//! ephemeral `WiresPlayRuntime` (selection + in-flight pointer drag of one board node) plus the `locale`
//! the deleted `ViewModel` used to carry.

use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `ReasoningWiresPlayApp::Config` — the pure-trait `ArtifactApp::Config` for the wires app.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "reasoning.wirescfg")]
#[dsl(id = "wires.config")]
#[dsl(layout = "lines")]
pub struct WiresConfig {
    /// 🖱️ In-flight pointer-drag target node id — was `WiresDragState::node_id`
    /// (`WiresPlayRuntime::drag`); `None` means no drag is in progress.
    pub drag_node_id: Option<String>,
    /// 🖱️ Last observed drag pointer X (screen space) — was `WiresDragState::last_x`.
    pub drag_last_x: f64,
    /// 🖱️ Last observed drag pointer Y (screen space) — was `WiresDragState::last_y`.
    pub drag_last_y: f64,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for WiresConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
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
    async fn print_dsl(&self) -> String {
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
impl store::ArtifactPack for WiresConfig {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec


impl Default for WiresConfig {
    async fn default() -> Self {
        Self { drag_node_id: None, drag_last_x: 0.0, drag_last_y: 0.0, locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(WiresConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`WiresConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `WiresPlayRuntime` field writes). Every field already carries its own setter, so `backwards()`
/// returns the SAME variant re-addressed at `base`'s old value — a targeted, in-kind inverse per
/// this ticket's ban on whole-record replace, rather than a generic whole-config snapshot.
/// `Mutation::Diff` is the WHOLE `WiresConfig` (not a granular patch type): `diff()` returns "the
/// full config after this op", and `store::impl_whole_record_config!` supplies the
/// `MutationDiff<WiresConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum WiresConfigMutation {
    #[dsl(key = "drag")]
    SetDrag { node_id: Option<String>, last_x: f64, last_y: f64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for WiresConfigMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for WiresConfigMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
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
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
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


impl Mutation<WiresConfig> for WiresConfigMutation {
    type Diff = WiresConfig;

    async fn diff(&self, base: &WiresConfig) -> protocol::MutationOutcome<WiresConfig> {
        let mut next = base.clone();
        match self {
            WiresConfigMutation::SetDrag { node_id, last_x, last_y } => {
                next.drag_node_id = node_id.clone();
                next.drag_last_x = *last_x;
                next.drag_last_y = *last_y;
            }
            WiresConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &WiresConfig) -> Vec<Self> {
        match self {
            WiresConfigMutation::SetDrag { .. } => vec![WiresConfigMutation::SetDrag { node_id: base.drag_node_id.clone(), last_x: base.drag_last_x, last_y: base.drag_last_y }],
            WiresConfigMutation::SetLocale { .. } => vec![WiresConfigMutation::SetLocale { value: base.locale.clone() }],
        }
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️ConfigTests
    /// 🕹️ Selection lives in the framework-owned "graph" interaction domain now (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — `WiresConfig` only carries drag/locale.
    #[test]
    async fn wires_config_default_matches_no_drag_and_en_locale() {
        let config = WiresConfig::default();
        assert!(config.drag_node_id.is_none());
        assert_eq!(config.locale, "en-US");
    }

    /// 🔁️ B1 dsl/pack round-trip law for `WiresConfig` — a non-default fixture exercising every field.
    #[test]
    async fn wires_config_dsl_pack_round_trip() {
        let config = WiresConfig { drag_node_id: Some("node-1".into()), drag_last_x: 12.5, drag_last_y: -7.25, locale: "de-DE".into() };
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }
    //#endregion 🔖️ConfigTests

    //#region 🔖️ConfigOperationTests
    #[test]
    async fn config_drag_op_text_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&WiresConfigMutation::SetDrag { node_id: Some("node-1".into()), last_x: 12.5, last_y: -7.25 });
        store::os_store::test_support::assert_op_line_round_trip(&WiresConfigMutation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 });
    }

    #[test]
    async fn config_locale_op_text_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&WiresConfigMutation::SetLocale { value: "de-DE".into() });
    }

    /// ⏪️ `backwards()` returns the SAME variant re-addressed at the pre-op field value — a targeted,
    /// in-kind inverse, not a whole-config replace.
    #[test]
    async fn config_backwards_restores_the_same_field_from_base() {
        let base = WiresConfig { drag_node_id: Some("node-1".into()), drag_last_x: 1.0, drag_last_y: 2.0, ..Default::default() };
        let forward = WiresConfigMutation::SetDrag { node_id: Some("node-2".into()), last_x: 5.0, last_y: 6.0 };
        let inverse = forward.inverse(&base);
        assert_eq!(inverse, vec![WiresConfigMutation::SetDrag { node_id: base.drag_node_id.clone(), last_x: base.drag_last_x, last_y: base.drag_last_y }]);
        assert_eq!(forward.diff(&base).diff().clone(), WiresConfig { drag_node_id: Some("node-2".into()), drag_last_x: 5.0, drag_last_y: 6.0, ..base });
    }
    //#endregion 🔖️ConfigOperationTests
}
//#endregion 🧪️Tests
