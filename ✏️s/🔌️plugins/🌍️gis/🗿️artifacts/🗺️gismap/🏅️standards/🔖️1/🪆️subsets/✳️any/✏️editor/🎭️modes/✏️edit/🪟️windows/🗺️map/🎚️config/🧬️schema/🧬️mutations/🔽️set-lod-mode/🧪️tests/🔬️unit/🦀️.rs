use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfigMutation;
use semio_framework_test_fixtures::fixture_text;
#[allow(unused_imports)]
use crate::{editor::gis2d::modes::edit::windows::map::config::MapWindowConfig, editor::gis2d::modes::edit::windows::map::config::MapWindowConfigDelta, editor::gis2d::modes::edit::windows::map::config::MapWindowConfigDiff};
#[allow(unused_imports)]
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::gis2d::modes::edit::windows::map::config::mutations::set_lod_mode::*;

#[test]
fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
    crate::editor::gis2d::modes::edit::windows::map::config::direct_mutation_tests::assert_leaf::<SetLodMode>(5, MapWindowConfigMutation::SetLodMode, fixture_text!("../../🧬️schema/🧬️mutations/🔽️set-lod-mode/🔣️.json"));
}
