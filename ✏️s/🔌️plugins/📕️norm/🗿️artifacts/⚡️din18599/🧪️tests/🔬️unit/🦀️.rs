
use super::*;

trait Din18599ChildOwnerOracle {
    fn expected() -> serde_json::Value;
}

struct SerdeJsonDin18599ChildOwnerOracle;

impl Din18599ChildOwnerOracle for SerdeJsonDin18599ChildOwnerOracle {
    fn expected() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/🧫️child-owner-isolation/🔣️.json")).expect("language-neutral DIN 18599 child-owner fixture")
    }
}

#[test]
fn climate_working_data_is_owned_by_the_exact_child() {
    let owned = din18599_climate_child_from_data(&MonthlyClimate { theta_e_c: [1.0; 12], g_h_w_m2: [2.0; 12] });
    let wire = dsl::json::to_json_string(&owned);
    let reconstructed: Din18599ClimateChild = dsl::json::from_json_str(&wire).expect("DIN 18599 child wire roundtrip");
    let mut oracle_wire = Vec::new();
    crate::document::child_identity_oracle::serialize(&owned, &mut serde_json::Serializer::new(&mut oracle_wire)).expect("independent child identity oracle");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&wire).expect("first-party child identity JSON"), serde_json::from_slice::<serde_json::Value>(&oracle_wire).expect("Serde child identity JSON"));
    let observed = serde_json::json!({
        "ownedHasPayload": owned.local_owner::<Din18599ClimateWorkingData>().is_some(),
        "wireIdentityMatches": owned == reconstructed,
        "wireHasPayload": reconstructed.local_owner::<Din18599ClimateWorkingData>().is_some(),
    });

    assert_eq!(observed, SerdeJsonDin18599ChildOwnerOracle::expected());
}
