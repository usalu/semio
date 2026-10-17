use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfigMutation;
#[allow(unused_imports)]
use crate::editor::gis2d::modes::edit::windows::map::config::required_nullable;
#[allow(unused_imports)]
use crate::{editor::gis2d::modes::edit::windows::map::config::MapWindowConfig, editor::gis2d::modes::edit::windows::map::config::MapWindowConfigDelta, editor::gis2d::modes::edit::windows::map::config::MapWindowConfigDiff};
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::gis2d::modes::edit::windows::map::config::mutations::set_layer_visibility::*;

#[test]
fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
    crate::editor::gis2d::modes::edit::windows::map::config::direct_mutation_tests::assert_leaf::<SetLayerVisibility>(0, MapWindowConfigMutation::SetLayerVisibility, runtime_fixture_text("../../🧬️schema/🧬️mutations/👁️set-layer-visibility/🔣️.json"));
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
