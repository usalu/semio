use super::*;

#[test]
fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
    super::super::super::direct_mutation_tests::assert_leaf::<SetRenderMode>(3, Gis2dConfigMutation::SetRenderMode, include_str!("../../🔣️.json"));
}
