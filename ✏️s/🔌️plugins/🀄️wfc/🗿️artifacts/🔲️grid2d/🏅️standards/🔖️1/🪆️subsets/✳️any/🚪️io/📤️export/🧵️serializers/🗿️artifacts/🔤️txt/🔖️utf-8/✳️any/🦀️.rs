//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use crate::Grid2dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{serialize_dsl_txt, ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct Grid2dIntoTxt;

impl Serializer<Grid2dSnapshot> for Grid2dIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Grid2dSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}


#[cfg(test)]
#[path = "🧪️tests/🔁️dsl-txt-round-trip/🦀️.rs"]
mod dsl_txt_round_trip_tests;
