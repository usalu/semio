
use super::*;
use protocol::{Mutation, MutationDiff};
#[test]
fn direct_leaf_neutral_schema_codec_and_outcome_laws() {
    super::super::super::direct_leaf_contracts::assert_leaf_contract::<SetCamera>("camera", include_str!("../../🔣️.json"));
}
#[test]
fn sparse_camera_inverse_and_codecs_preserve_locale() {
    let base = Gis3dConfig { camera_json: "default".into(), locale: "de-DE".into() };
    let mutation = Gis3dConfigMutation::SetCamera(SetCamera { camera_json: "next".into() });
    let next = mutation.diff(&base).diff().apply(&base).expect("apply");
    assert_eq!(next.locale, base.locale);
    assert_eq!(mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse"), base);
    store::os_store::test_support::assert_op_line_round_trip(&mutation);
}
