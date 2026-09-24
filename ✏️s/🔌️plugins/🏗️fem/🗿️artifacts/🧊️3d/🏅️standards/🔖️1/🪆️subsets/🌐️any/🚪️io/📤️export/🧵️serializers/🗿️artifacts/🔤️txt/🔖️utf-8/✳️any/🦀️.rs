//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use crate::Fem3dSnapshot;
use semio_framework::io::io_mechanism::{serialize_dsl_txt, Serializer};
use semio_framework::io_schema::{Dialect, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct Fem3dIntoTxt;

/// 🏗 This subset snapshot as UTF-8 DSL text — also the body the zip container leaf embeds.
pub fn dsl_text(from: &Fem3dSnapshot) -> String {
    <Fem3dSnapshot as store::ArtifactDsl>::print_dsl(from)
}

impl Serializer<Fem3dSnapshot> for Fem3dIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Fem3dSnapshot) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}
