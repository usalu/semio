#!/usr/bin/env python3
"""Extract kernel DAG DSL mirror into plugin snapshot schema (one-off helper)."""
from pathlib import Path

kernel = Path(
    "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs"
)
text = kernel.read_text()
start = text.index("//#region 🔖️DslMirror")
end = text.index("impl crate::os_store::DocumentDsl for DagDocument {")
mirror = text[start:end]
mirror = mirror.replace("DagDocumentDsl", "DagSnapshotDsl")
mirror = mirror.replace("dag_document_to_dsl", "dag_snapshot_to_dsl")
mirror = mirror.replace("dag_document_from_dsl", "dag_snapshot_from_dsl")
mirror = mirror.replace("struct DagSnapshotDsl", "pub(crate) struct DagSnapshotDsl")
mirror = mirror.replace("enum DagNodeKindDsl", "pub(crate) enum DagNodeKindDsl")
mirror = mirror.replace("struct DagNodeSpecDsl", "pub(crate) struct DagNodeSpecDsl")
mirror = mirror.replace("fn dag_node_kind_to_dsl", "pub(crate) fn dag_node_kind_to_dsl")
mirror = mirror.replace("fn dag_node_kind_from_dsl", "pub(crate) fn dag_node_kind_from_dsl")
mirror = mirror.replace("fn dag_node_spec_to_dsl", "pub(crate) fn dag_node_spec_to_dsl")
mirror = mirror.replace("fn dag_node_spec_from_dsl", "pub(crate) fn dag_node_spec_from_dsl")
mirror = mirror.replace(
    "DagDocument { schema: mirror.schema, nodes:",
    "DagSnapshot { schema: mirror.schema, nodes:",
)
mirror = mirror.replace("dag_snapshot_from_dsl(mirror: DagSnapshotDsl) -> DagDocument", "dag_snapshot_from_dsl(mirror: DagSnapshotDsl) -> DagSnapshot")
mirror = mirror.replace("DagNodeSpec {", "DagNodeSpec {")  # noop
# fix return type of from_dsl
mirror = mirror.replace(
    "fn dag_snapshot_from_dsl(mirror: DagSnapshotDsl) -> DagSnapshot {",
    "pub(crate) fn dag_snapshot_from_dsl(mirror: DagSnapshotDsl) -> DagSnapshot {",
)
mirror = mirror.replace(
    "fn dag_snapshot_to_dsl(document: &DagDocument)",
    "pub(crate) fn dag_snapshot_to_dsl(snapshot: &DagSnapshot)",
)
mirror = mirror.replace("document.schema.clone(), nodes: document.nodes", "snapshot.schema.clone(), nodes: snapshot.nodes")
mirror = mirror.replace("document.edges.clone()", "snapshot.edges.clone()")

impl = '''
impl store::DocumentDsl for DagSnapshotDsl {
    const EXTENSION: &'static str = "dag";
    fn envelope_id() -> &'static str {
        "dag.dag"
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

impl store::DocumentPack for DagSnapshotDsl {
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
'''

header = '''//! (mirror types injected below — see //#region 🔖️DslMirror)

use infinite_board_port_directed_dag::{
    DagMedia, DagNodeKind, DagNodeSpec, DagPreviewContent, EdgeRouteStyle, IoPortSpec,
};
use math::graph::manifest::PropertyBag;

'''

out = Path(
    "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS/🧪wave5-dag-dsl-mirror.rs.fragment"
)
out.write_text(header + mirror + impl)
print("wrote", out, "lines", len(out.read_text().splitlines()))
