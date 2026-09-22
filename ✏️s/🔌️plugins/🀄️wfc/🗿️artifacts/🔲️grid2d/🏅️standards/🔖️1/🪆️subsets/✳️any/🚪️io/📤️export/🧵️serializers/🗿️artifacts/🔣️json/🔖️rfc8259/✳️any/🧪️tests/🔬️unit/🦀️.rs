use super::*;
use crate::examples::grid2d::pipes;

#[test]
fn serializing_pipes_to_json_yields_a_non_empty_exact_pack() {
    let document = pipes::document();
    let json = serialize(&document);
    let bytes = <JsonSnapshot as store::ArtifactPack>::encode_pack(&json);
    assert!(!bytes.is_empty(), "json pack must carry the document");
    assert_eq!(Grid2dIntoJson::FIDELITY, IoFidelity::Exact);
    assert_eq!(Grid2dIntoJson::INTO, JSON_DIALECT);
}
