//! 🔤️ grid2d ← `s.stdio.txt@utf-8` — the exact inverse of the txt export leaf.

use crate::Grid2dSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 📖️ Parses a `TxtSnapshot` body back through this artifact's own DSL grammar.
pub fn deserialize(from: &TxtSnapshot) -> Result<Grid2dSnapshot, store::TextError> {
    <Grid2dSnapshot as store::ArtifactDsl>::parse_dsl(&from.to_body())
}

pub struct TxtIntoGrid2d;

impl Deserializer<Grid2dSnapshot> for TxtIntoGrid2d {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<Grid2dSnapshot> {
        let txt = match payload {
            IoPayload::Binary(bytes) => <TxtSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("TxtIntoGrid2d: txt decode failed: {error}"), diagnostics: Vec::new() })?,
            IoPayload::Text(text) => TxtSnapshot::from_body(text),
        };
        let snapshot = deserialize(&txt).map_err(|error| IoError { message: format!("TxtIntoGrid2d: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
