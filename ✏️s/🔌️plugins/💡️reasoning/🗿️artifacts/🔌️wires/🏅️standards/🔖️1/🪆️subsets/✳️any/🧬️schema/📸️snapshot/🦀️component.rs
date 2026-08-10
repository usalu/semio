//! 🧬️ Wires snapshot schema — persistent fields only.

use crate::artifacts::wires::{BoardFixtureDsl, WiresFixtureDsl};
use dsl::DslValue;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted wires document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresSnapshot {
    #[state(persistent)]
    pub wires_fixture: DslValue,
    #[state(persistent)]
    pub board_fixture: DslValue,
}
//#endregion 🔖️Snapshot

//#region 🔖️DslMirror
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
#[dsl(extension = "reasoning.wires", layout = "lines")]
struct WiresSnapshotDsl {
    #[dsl(key = "wires", block)]
    wires_fixture: WiresFixtureDsl,
    #[dsl(key = "board", block)]
    board_fixture: BoardFixtureDsl,
}

fn wires_snapshot_to_dsl(snapshot: &WiresSnapshot) -> WiresSnapshotDsl {
    WiresSnapshotDsl {
        wires_fixture: dsl::from_dsl_value(snapshot.wires_fixture.clone()).unwrap_or_else(|error| panic!("wires_fixture does not match the reasoning.wires.fixture schema: {error}")),
        board_fixture: dsl::from_dsl_value(snapshot.board_fixture.clone()).unwrap_or_else(|error| panic!("board_fixture does not match the reasoning.mindmap.fixture schema: {error}")),
    }
}

fn wires_snapshot_from_dsl(parsed: &WiresSnapshotDsl) -> Result<WiresSnapshot, store::TextError> {
    Ok(WiresSnapshot {
        wires_fixture: dsl::to_dsl_value(&parsed.wires_fixture).map_err(|error| store::TextError::new(format!("invalid wires fixture: {error}"), store::TextSpan::at(1, 1)))?,
        board_fixture: dsl::to_dsl_value(&parsed.board_fixture).map_err(|error| store::TextError::new(format!("invalid board fixture: {error}"), store::TextSpan::at(1, 1)))?,
    })
}
//#endregion 🔖️DslMirror

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for WiresSnapshot {
    const EXTENSION: &'static str = "wires";
    fn envelope_id() -> &'static str {
        "reasoning.wires"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let parsed = <WiresSnapshotDsl as store::ArtifactDsl>::parse_dsl(text)?;
        wires_snapshot_from_dsl(&parsed)
    }
    fn print_dsl(&self) -> String {
        <WiresSnapshotDsl as store::ArtifactDsl>::print_dsl(&wires_snapshot_to_dsl(self))
    }
}

impl store::ArtifactPack for WiresSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <WiresSnapshotDsl as store::ArtifactPack>::encode_pack_with(&wires_snapshot_to_dsl(self), options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <WiresSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        wires_snapshot_from_dsl(&parsed).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(WiresSnapshotDsl::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl store::ArtifactDsl for WiresSnapshotDsl {
    const EXTENSION: &'static str = "wires";
    fn envelope_id() -> &'static str {
        "reasoning.wires"
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

impl store::ArtifactPack for WiresSnapshotDsl {
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
