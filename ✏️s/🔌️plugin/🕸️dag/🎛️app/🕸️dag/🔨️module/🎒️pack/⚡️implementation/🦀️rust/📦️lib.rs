//! 📦️ DAG app — binary document surface + laws (constitutional: pack).
//!
//! `store::DocumentPack for DagDocument` is implemented directly in the DAG kernel crate
//! (`infinite_board_port_directed_dag`); see `s/plugin/dag/app/rs/lib.rs` for why. This crate only adds
//! the thin app-facing `encode`/`decode` wrappers plus the pack↔dsl equivalence law.

use dag::DagDocument;
use store::PackError;

/// 📦️ Encodes a `DagDocument` to its binary pack form.
pub fn encode(document: &DagDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `DagDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<DagDocument, PackError> {
    <DagDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = dag_dsl::parse_dsl(dag_dsl::DAG_EXAMPLE_TEXT).expect("parse default fixture");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
//#endregion 🧪️Tests
