//! 🔤️ grid2d → `s.stdio.txt@utf-8` — the document's own `.wfcgrid2d` DSL text, losslessly. This
//! artifact's native serialization already IS UTF-8 text (`ArtifactDsl::print_dsl`), so the hop is
//! `IoFidelity::Exact`.

use crate::Grid2dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_txt::TxtSnapshot;

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 🖨️ Prints the document's own DSL text into a `TxtSnapshot` body.
pub fn serialize(from: &Grid2dSnapshot) -> TxtSnapshot {
    TxtSnapshot::from_body(&<Grid2dSnapshot as store::ArtifactDsl>::print_dsl(from))
}

pub struct Grid2dIntoTxt;

impl Serializer<Grid2dSnapshot> for Grid2dIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Grid2dSnapshot) -> IoResult<IoPayload> {
        let txt = serialize(from);
        let bytes = <TxtSnapshot as store::ArtifactPack>::encode_pack(&txt);
        if bytes.is_empty() {
            return Err(IoError { message: "Grid2dIntoTxt: empty txt pack".to_string(), diagnostics: Vec::new() });
        }
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
