use super::*;
use protocol::Mutation;
#[test]
fn direct_leaf_neutral_schema_codec_and_outcome_laws() {
    super::super::super::direct_leaf_contracts::assert_leaf_contract::<SetCamera>("camera", include_str!("../../🔣️.json"));
}
#[test]
fn sparse_camera_inverse_and_codecs_restore_the_base() {
    let base = GisTerrainWindowConfig { camera_json: "default".into() };
    let mutation = GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: "next".into() });
    let next = protocol::apply_diff(mutation.diff(&base).diff(), &base).expect("apply");
    assert_eq!(next.camera_json, "next");
    assert_eq!(protocol::apply_diff(mutation.inverse(&base).expect("valid retained mutation inverse fixture")[0].diff(&next).diff(), &next).expect("inverse"), base);
    store::os_store::test_support::assert_op_line_round_trip(&mutation);
}
