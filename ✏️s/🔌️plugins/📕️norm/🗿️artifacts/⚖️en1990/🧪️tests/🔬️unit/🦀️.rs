
use super::*;

trait En1990ChildOwnerOracle {
    fn expected() -> serde_json::Value;
}

struct SerdeJsonEn1990ChildOwnerOracle;

impl En1990ChildOwnerOracle for SerdeJsonEn1990ChildOwnerOracle {
    fn expected() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral EN 1990 child-owner fixture")
    }
}

#[test]
fn qk_working_table_is_owned_by_the_exact_child() {
    let owned = en1990_qk_child_from_entries(&[En1990QkEntry { category: "snow".into(), value: 42.0 }]);
    let wire = dsl::json::to_json_string(&owned);
    let reconstructed: En1990QkChild = dsl::json::from_json_str(&wire).expect("EN 1990 child wire roundtrip");
    let mut oracle_wire = Vec::new();
    crate::document::child_identity_oracle::serialize(&owned, &mut serde_json::Serializer::new(&mut oracle_wire)).expect("independent child identity oracle");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&wire).expect("first-party child identity JSON"), serde_json::from_slice::<serde_json::Value>(&oracle_wire).expect("Serde child identity JSON"));
    let observed = serde_json::json!({
        "ownedHasPayload": owned.local_owner::<En1990QkWorkingTable>().is_some(),
        "wireIdentityMatches": owned == reconstructed,
        "wireHasPayload": reconstructed.local_owner::<En1990QkWorkingTable>().is_some(),
    });

    assert_eq!(observed, SerdeJsonEn1990ChildOwnerOracle::expected());
}
