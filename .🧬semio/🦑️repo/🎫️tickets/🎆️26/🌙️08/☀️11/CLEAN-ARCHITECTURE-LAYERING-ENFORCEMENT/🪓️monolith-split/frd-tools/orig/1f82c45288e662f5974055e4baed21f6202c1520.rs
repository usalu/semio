use crate::editor::gis3d::modes::view::windows::terrain::config::{GisTerrainWindowConfig, GisTerrainWindowConfigMutation};
#[allow(unused_imports)]
use crate::{editor::gis3d::modes::view::windows::terrain::config::GisTerrainWindowConfigDelta, editor::gis3d::modes::view::windows::terrain::config::GisTerrainWindowConfigDiff};
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::gis3d::modes::view::windows::terrain::config::mutations::set_camera::*;
use protocol::Mutation;
#[test]
fn direct_leaf_neutral_schema_codec_and_outcome_laws() {
    crate::editor::gis3d::modes::view::windows::terrain::config::direct_leaf_contracts::assert_leaf_contract::<SetCamera>("camera", runtime_fixture_text("../../../../../✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🧬️schema/🧬️mutations/🎥️set-camera/🔣️.json"));
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

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
