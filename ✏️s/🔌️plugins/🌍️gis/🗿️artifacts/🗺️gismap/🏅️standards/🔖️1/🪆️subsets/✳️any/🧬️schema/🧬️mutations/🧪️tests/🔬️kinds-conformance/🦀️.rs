use crate::GisMapSnapshot;
use semio_framework_test_fixtures::fixture_text;
#[allow(unused_imports)]
use crate::diff::GisMapDiff;
#[allow(unused_imports)]
use crate::{mutations::create_position, mutations::create_region, mutations::create_route, mutations::delete_position, mutations::delete_region, mutations::delete_route, mutations::reorder_positions, mutations::reorder_regions, mutations::reorder_routes, mutations::replace_position_data, mutations::replace_region_data, mutations::replace_route_data, mutations::set_position_property, mutations::remove_position_property, mutations::set_route_property, mutations::remove_route_property, mutations::set_region_property, mutations::remove_region_property};
#[allow(unused_imports)]
use protocol::Mutation;
#[allow(unused_imports)]
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore};
use crate::standards::v1::subsets::any::schema::mutations::*;

/// 🏷️ [`KINDS`] must name every declared variant, in the exact order and spelling
/// `#[derive(dsl::Mutations)]` assigns, and every one of them must appear in the committed oracle
/// manifest's catalog. The framework never parses Rust, so this is what keeps the declaration
/// honest in both directions at once.
#[test]
fn kinds_match_the_enum_and_the_catalog() {
    let descriptors = <GisMapMutation as protocol::SemanticMutation<GisMapSnapshot>>::kinds();
    assert_eq!(KINDS.len(), descriptors.len(), "KINDS must name exactly one entry per declared variant");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind, "KINDS must match #[derive(dsl::Mutations)]'s own declaration order and spelling");
    }
    let manifest = fixture_text!("../../../../🔮️oracles/🔣️.json");
    for kind in KINDS {
        assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
    }
}
