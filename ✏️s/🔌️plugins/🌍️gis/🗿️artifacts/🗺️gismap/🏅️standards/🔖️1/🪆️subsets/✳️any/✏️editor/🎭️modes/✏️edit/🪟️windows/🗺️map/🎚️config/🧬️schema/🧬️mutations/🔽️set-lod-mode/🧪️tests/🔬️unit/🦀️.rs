use super::*;

#[test]
fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
    super::super::super::direct_mutation_tests::assert_leaf::<SetLodMode>(5, MapWindowConfigMutation::SetLodMode, include_str!("../../🔣️.json"));
}
