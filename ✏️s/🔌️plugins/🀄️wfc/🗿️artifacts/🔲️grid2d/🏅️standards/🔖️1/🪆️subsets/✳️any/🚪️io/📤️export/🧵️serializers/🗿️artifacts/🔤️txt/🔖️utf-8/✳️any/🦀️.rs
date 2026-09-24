//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use crate::Grid2dSnapshot;
use semio_framework::io::io_mechanism::{serialize_dsl_txt, Serializer};
use semio_framework::io_schema::{Dialect, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct Grid2dIntoTxt;

impl Serializer<Grid2dSnapshot> for Grid2dIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Grid2dSnapshot) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}


#[cfg(test)]
#[path = "🧪️tests/🔁️dsl-txt-round-trip/🦀️.rs"]
mod dsl_txt_round_trip_tests;
