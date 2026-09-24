//! txt import via framework `io_mechanism::deserialize_dsl_txt` (UTF-8 DSL carrier).

use crate::PresentationSnapshot;
use semio_framework::io::io_mechanism::{deserialize_dsl_txt, Deserializer};
use semio_framework::io_schema::{Dialect, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct TxtIntoPresentation;

impl Deserializer<PresentationSnapshot> for TxtIntoPresentation {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<PresentationSnapshot> {
        deserialize_dsl_txt(payload)
    }
}
