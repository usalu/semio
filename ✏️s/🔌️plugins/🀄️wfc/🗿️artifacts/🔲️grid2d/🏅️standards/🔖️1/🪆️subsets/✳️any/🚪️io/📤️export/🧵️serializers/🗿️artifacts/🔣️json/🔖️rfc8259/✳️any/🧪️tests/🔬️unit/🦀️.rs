use crate::Grid2dSnapshot;
use semio_framework::io_schema::IoFidelity;
use semio_s_artifact_stdio_json::JsonSnapshot;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::*;

#[test]
fn serializing_default_document_to_json_yields_a_non_empty_exact_pack() {
    let document = Grid2dSnapshot::default();
    let json = serialize(&document);
    let bytes = <JsonSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&json);
    assert!(!bytes.is_empty(), "json pack must carry the document");
    assert_eq!(Grid2dIntoJson::FIDELITY, IoFidelity::Exact);
    assert_eq!(Grid2dIntoJson::INTO, JSON_DIALECT);
}
