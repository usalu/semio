use semio_framework::io_schema::IoPayload;
use semio_s_artifact_stdio_json::JsonSnapshot;
#[allow(unused_imports)]
use crate::WriterSnapshot;
#[allow(unused_imports)]
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
#[allow(unused_imports)]
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoResult};
#[allow(unused_imports)]
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::*;

#[semio_framework_async_macros::async_test]
async fn json_into_writer_round_trips_a_real_snapshot() {
    let original = crate::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
    let json = JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&original)));
    let text = pack::snapshot::ArtifactDsl::print_dsl(&json);
    let outcome = JsonIntoWriter::deserialize(&IoPayload::Text(text)).await.expect("deserialize");
    assert_eq!(outcome.value, original);
}
