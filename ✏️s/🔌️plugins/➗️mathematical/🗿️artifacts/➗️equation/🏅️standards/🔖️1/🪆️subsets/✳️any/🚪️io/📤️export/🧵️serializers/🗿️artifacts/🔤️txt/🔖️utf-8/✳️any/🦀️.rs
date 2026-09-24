//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use crate::EquationSnapshot;
use semio_framework::io::io_mechanism::{serialize_dsl_txt, Serializer};
use semio_framework::io_schema::{Dialect, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct EquationIntoTxt;

impl Serializer<EquationSnapshot> for EquationIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &EquationSnapshot) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}
