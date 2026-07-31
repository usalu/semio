//! 📦️ Flow app — binary document surface + laws (constitutional: pack).
//!
//! `store::DocumentPack for FlowFixture` is implemented directly in the flow kernel crate (`flow_core`);
//! see `s/plugin/flow/app/rs/lib.rs` for why. This crate only adds the thin app-facing
//! `encode`/`decode` wrappers plus the pack↔dsl equivalence law.

use flow::FlowFixture;
use store::PackError;

/// 📦️ Encodes a `FlowFixture` to its binary pack form.
pub fn encode(fixture: &FlowFixture) -> Vec<u8> {
    store::DocumentPack::encode_pack(fixture)
}

/// 📖️ Decodes a `FlowFixture` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<FlowFixture, PackError> {
    <FlowFixture as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let fixture = flow_dsl::parse_dsl(flow_dsl::FLOW_EXAMPLE_TEXT).expect("parse default fixture");
        store::test_support::assert_dsl_pack_equivalence(&fixture);
        let bytes = encode(&fixture);
        assert_eq!(decode(&bytes).expect("decode"), fixture);
    }
}
//#endregion 🧪️Tests
