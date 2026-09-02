//! 📦️ ISO 16757 app — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::iso16757::Iso16757Snapshot;
use store::PackError;

/// 📦️ Encodes a `Document` to its binary pack form.
pub fn encode(document: &Iso16757Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Document` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Iso16757Snapshot, PackError> {
    <Iso16757Snapshot as store::ArtifactPack>::decode_pack(bytes)
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
        store::os_store::test_support::assert_dsl_pack_equivalence(&Iso16757Snapshot::reference_fixture());
    }

    #[semio_framework_async_macros::async_test]
    fn pack_round_trips_the_reference_fixture() {
        let document = Iso16757Snapshot::reference_fixture();
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }
}
