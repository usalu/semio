use super::*;

fn demo() -> ModelSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn the_manifest_declares_both_domains_with_one_granularity_per_kind() {
    let definitions = definitions();
    assert_eq!(definitions.iter().map(|definition| definition.id.as_str()).collect::<Vec<_>>(), vec![BIM_ELEMENT_DOMAIN, BIM_LIBRARY_DOMAIN]);
    assert_eq!(definitions[0].granularities.len(), ENTITIES.iter().filter(|row| !row.library).count());
    assert_eq!(definitions[1].granularities.len(), ENTITIES.iter().filter(|row| row.library).count() + 1, "the library kinds and the classification entry");
    assert!(definitions[0].granularities.iter().any(|granularity| granularity.id == "wall-sweep"));
}

#[semio_framework_async_macros::async_test]
async fn a_wall_sweep_is_a_node_of_the_element_domain_under_its_wall() {
    let create = crate::editor::bim::entities::kind_of("wall-sweep").and_then(|row| row.create).expect("wall sweep create");
    let snapshot = demo();
    let snapshot = crate::mutations::apply_model_mutation(&snapshot, &create(&snapshot, "sw-1", "w-south", "Baseboard").expect("creates")).expect("applies");
    let topology = element_topology(&snapshot);
    let position = |id: &str| topology.ordered.iter().position(|node| node.id == id).unwrap_or_else(|| panic!("{id} in topology"));
    assert!(position("w-south") < position("sw-1") && position("sw-1") < position("st-first"));
    let sweep = &topology.ordered[position("sw-1")];
    assert_eq!((sweep.granularity.as_str(), sweep.parent.as_deref()), ("wall-sweep", Some("w-south")));
}

#[semio_framework_async_macros::async_test]
async fn the_element_topology_nests_site_building_storey_wall_in_tree_order() {
    let topology = element_topology(&demo());
    let position = |id: &str| topology.ordered.iter().position(|node| node.id == id).unwrap_or_else(|| panic!("{id} in topology"));
    assert!(position("site-1") < position("bldg-1") && position("bldg-1") < position("st-ground") && position("st-ground") < position("w-south") && position("w-south") < position("st-first"));
    let wall = &topology.ordered[position("w-south")];
    assert_eq!((wall.granularity.as_str(), wall.parent.as_deref()), ("wall", Some("st-ground")));
}

#[semio_framework_async_macros::async_test]
async fn the_library_topology_lists_materials_and_types_flat() {
    let topology = library_topology(&demo());
    assert!(topology.ordered.iter().all(|node| node.parent.is_none()));
    assert!(topology.ordered.iter().any(|node| node.id == "wt-300" && node.granularity == "wall-type"));
    assert!(topology.ordered.iter().any(|node| node.id == "m-brick" && node.granularity == "material"));
}

#[semio_framework_async_macros::async_test]
async fn both_domains_are_published_for_pick_revalidation() {
    assert_eq!(topology(&demo()).domains.keys().cloned().collect::<Vec<_>>(), vec![BIM_ELEMENT_DOMAIN.to_string(), BIM_LIBRARY_DOMAIN.to_string()]);
}
