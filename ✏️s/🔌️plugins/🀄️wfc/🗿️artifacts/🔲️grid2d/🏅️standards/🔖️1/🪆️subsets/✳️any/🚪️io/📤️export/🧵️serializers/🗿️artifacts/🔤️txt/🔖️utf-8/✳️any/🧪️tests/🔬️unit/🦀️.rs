use super::*;
use crate::examples::grid2d::pipes;

#[test]
fn serializing_pipes_to_txt_yields_a_non_empty_exact_pack() {
    let document = pipes::document();
    let txt = serialize(&document);
    let bytes = <TxtSnapshot as store::ArtifactPack>::encode_pack(&txt);
    assert!(!bytes.is_empty(), "txt pack must carry the DSL");
    assert_eq!(Grid2dIntoTxt::FIDELITY, IoFidelity::Exact);
    assert_eq!(Grid2dIntoTxt::INTO, TXT_DIALECT);
}
