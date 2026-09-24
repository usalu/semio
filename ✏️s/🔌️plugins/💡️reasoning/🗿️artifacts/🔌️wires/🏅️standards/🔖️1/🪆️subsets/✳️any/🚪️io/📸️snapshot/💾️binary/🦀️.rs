//! 🎁 Wires artifact — `.wires` binary pack codec over the derived `WiresPackRecord` spec.

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

use crate::{wires_working_scene, WiresSnapshot};
use dsl::DslValue;

//#region 🔖️PackRecord
/// 🎁 Derived pack record of a `WiresSnapshot`: the fixture and meta values plus the composed graph
/// child's content (`nodes`/`edges`), from which decode re-mints the content-addressed child handle.
/// Every value travels as first-party JSON text: pack canonicalises map keys into sorted order, while
/// these free-form `DslValue` objects are order-significant The text facet prints this same record.
#[derive(dsl::DslRecord)]
#[dsl(extension = "wires")]
struct WiresPackRecord {
    wires_fixture: String,
    nodes: Vec<String>,
    edges: Vec<String>,
    meta: String,
}

fn json_of(value: &DslValue) -> String {
    dsl::os_pack::json::to_json_string(value)
}

fn value_of(text: &str) -> Result<DslValue, String> {
    dsl::os_pack::json::from_json_str::<DslValue>(text).map_err(|e| e.to_string())
}

impl WiresPackRecord {
    fn from_snapshot(snapshot: &WiresSnapshot) -> Self {
        let scene = wires_working_scene(snapshot);
        Self { wires_fixture: json_of(&snapshot.wires_fixture), nodes: scene.nodes.iter().map(json_of).collect(), edges: scene.edges.iter().map(json_of).collect(), meta: json_of(&snapshot.meta) }
    }

    fn into_snapshot(self) -> Result<WiresSnapshot, String> {
        let nodes = self.nodes.iter().map(|text| value_of(text)).collect::<Result<Vec<_>, _>>()?;
        let edges = self.edges.iter().map(|text| value_of(text)).collect::<Result<Vec<_>, _>>()?;
        Ok(WiresSnapshot { wires_fixture: value_of(&self.wires_fixture)?, content: crate::wires_content_child_with_owner(nodes, edges), meta: value_of(&self.meta)? })
    }
}

/// 🖨️ The derived text body: the same `WiresPackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &WiresSnapshot) -> String {
    dsl::print(&WiresPackRecord::from_snapshot(snapshot).__dsl_to_record(), &WiresPackRecord::__dsl_spec(), dsl::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `WiresPackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<WiresSnapshot, store::TextError> {
    let record = dsl::parse(body, &WiresPackRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
    WiresPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
}
//#endregion 🔖️PackRecord

//#region 🔖️ArtifactPack
impl store::ArtifactPack for WiresSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&WiresPackRecord::__dsl_spec(), &WiresPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &WiresPackRecord::__dsl_spec(), options)?;
        WiresPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(store::PackError::Schema)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(WiresPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactPack

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

