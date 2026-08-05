//! 📦️ ISO 16757 app — binary document surface + laws (constitutional: pack).

use crate::artifacts::iso16757::Document;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Document) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Document, PackError> {
    <Document as store::DocumentPack>::decode_pack(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 🪲️ Blocked on the same confirmed upstream `pack` crate bug as `architect/spine/rs/lib.rs`'s
    // `sample_plugin_dsl_pack_equivalence` (see its comment there): `pack::value`'s self-describing
    // `TableSoA` decoder (`decode_table_soa`, `pack/value/rs/lib.rs`) has no `RecordSpec` for a
    // `#[dsl(table)]` row's nested non-primitive `Record` columns, so it can't backfill an `Option<T>`
    // sub-field of that nested record (here: `Names.short_name`, `None` on many rows of this fixture's
    // `#[dsl(table)]` catalogue tables — `ProductGroup`/`ProductClass`/`Product`/`Subject` etc. all
    fn document_dsl_pack_equivalence_the_reference_fixture() {
        store::test_support::assert_dsl_pack_equivalence(&Document::reference_fixture());
    }

    #[test]
    fn pack_round_trips_the_reference_fixture() {
        let document = Document::reference_fixture();
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
