
use super::*;

#[test]
fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
    super::super::super::direct_mutation_tests::assert_leaf::<SetVectorStyle>(4, Gis2dConfigMutation::SetVectorStyle, include_str!("../../🔣️.json"));
}
