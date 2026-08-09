//! 🧬️ Sequence snapshot schema — persistent fields only.

use crate::artifacts::sequence::{
    SequenceEdge, SequenceStep, StepParams, SEQUENCE_DOCUMENT_SCHEMA,
};
use crate::artifacts::sequence::dsl::{sequence_edge_from_dsl, sequence_edge_to_dsl, SequenceEdgeDsl};
use neural_engine::{Atom, Value};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted sequence document snapshot (steps + flow edges).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub steps: Vec<SequenceStep>,
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<SequenceEdge>,
}

impl Default for SequenceSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> SequenceSnapshot {
    SequenceSnapshot {
        schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
        steps: vec![
            SequenceStep {
                id: "step-1".into(),
                kind: "state.set".into(),
                params: StepParams::new()
                    .insert("key", Value::Atom(Atom::String("counter".into())))
                    .insert("value", Value::Atom(Atom::Integer(0))),
                x: 0.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
            SequenceStep {
                id: "step-2".into(),
                kind: "log.print".into(),
                params: StepParams::new().insert("message", Value::Atom(Atom::String("hello sequence".into()))),
                x: 280.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
        ],
        edges: vec![SequenceEdge { id: "edge-1".into(), from: "step-1".into(), to: "step-2".into() }],
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️DslMirror
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(extension = "sequence")]
#[dsl(layout = "lines")]
pub(crate) struct SequenceSnapshotDsl {
    schema: String,
    #[dsl(table)]
    steps: Vec<SequenceStep>,
    #[dsl(table)]
    edges: Vec<SequenceEdgeDsl>,
}

pub(crate) fn sequence_snapshot_to_dsl(snapshot: &SequenceSnapshot) -> SequenceSnapshotDsl {
    SequenceSnapshotDsl {
        schema: snapshot.schema.clone(),
        steps: snapshot.steps.clone(),
        edges: snapshot.edges.iter().map(sequence_edge_to_dsl).collect(),
    }
}

pub(crate) fn sequence_snapshot_from_dsl(dsl_snapshot: SequenceSnapshotDsl) -> Result<SequenceSnapshot, String> {
    Ok(SequenceSnapshot {
        schema: dsl_snapshot.schema,
        steps: dsl_snapshot.steps,
        edges: dsl_snapshot.edges.into_iter().map(sequence_edge_from_dsl).collect::<Result<Vec<_>, _>>()?,
    })
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for SequenceSnapshotDsl {
    const EXTENSION: &'static str = "sequence";
    fn envelope_id() -> &'static str {
        "sequence.sequence"
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

impl store::DocumentPack for SequenceSnapshotDsl {
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

impl store::DocumentDsl for SequenceSnapshot {
    const EXTENSION: &'static str = "sequence";
    fn envelope_id() -> &'static str {
        "sequence.sequence"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <SequenceSnapshotDsl as store::DocumentDsl>::parse_dsl(text)?;
        sequence_snapshot_from_dsl(parsed).map_err(|message| store::TextError::new(message, store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        <SequenceSnapshotDsl as store::DocumentDsl>::print_dsl(&sequence_snapshot_to_dsl(self))
    }
}

impl store::DocumentPack for SequenceSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <SequenceSnapshotDsl as store::DocumentPack>::encode_pack_with(&sequence_snapshot_to_dsl(self), options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <SequenceSnapshotDsl as store::DocumentPack>::decode_pack_with(bytes, options)?;
        sequence_snapshot_from_dsl(parsed).map_err(|message| store::text_error_to_pack_error(store::TextError::new(message, store::TextSpan::at(1, 1))))
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
