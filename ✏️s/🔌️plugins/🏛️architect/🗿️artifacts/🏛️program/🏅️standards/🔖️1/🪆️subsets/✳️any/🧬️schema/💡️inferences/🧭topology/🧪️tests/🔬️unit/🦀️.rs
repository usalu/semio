
use super::*;
use crate::kernel::{EntityHeader, EntityId, QuantitySpec};
use crate::standards::v1::subsets::any::schema::registers::ProgramElementKind;

fn element(id: &str, parent: Option<&str>) -> ProgramElement {
    ProgramElement {
        header: EntityHeader::new(EntityId(id.into()), id),
        code: id.into(),
        kind: ProgramElementKind::Room,
        parent_id: parent.map(|p| EntityId(p.into())),
        level: None,
        area: QuantitySpec::default(),
        volume: QuantitySpec::default(),
        height: QuantitySpec::default(),
        occupancy: QuantitySpec::default(),
        function_ids: Vec::new(),
        activity_ids: Vec::new(),
        user_profile_ids: Vec::new(),
        adjacency_ids: Vec::new(),
        quantity_ids: Vec::new(),
        requirement_ids: Vec::new(),
        location_hint: None,
        orientation: None,
        daylight_requirement: None,
        acoustic_class: None,
        security_zone: None,
        flexibility_notes: Vec::new(),
        growth_allocation: None,
        circulation_role: None,
        visibility_level: None,
        adjacency_preferences: Vec::new(),
        environmental_zone: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_elements_yield_default_topology() {
    assert_eq!(compute_topology(&[]), ProgramTopology::default());
}

#[semio_framework_async_macros::async_test]
async fn linear_chain_has_matching_depth_and_order() {
    let elements = vec![element("a", None), element("b", Some("a")), element("c", Some("b"))];
    let topology = compute_topology(&elements);
    assert_eq!(topology.node_count, 3);
    assert_eq!(topology.root_count, 1);
    assert_eq!(topology.max_depth, 2);
    assert!(topology.cycle_free);
    assert_eq!(topology.topo_order, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn a_cycle_is_detected_and_still_yields_a_total_order() {
    let elements = vec![element("a", Some("b")), element("b", Some("a"))];
    let topology = compute_topology(&elements);
    assert_eq!(topology.node_count, 2);
    assert_eq!(topology.root_count, 0);
    assert!(!topology.cycle_free);
    assert_eq!(topology.topo_order.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn a_dangling_parent_id_is_treated_as_a_root() {
    let elements = vec![element("a", Some("nonexistent"))];
    let topology = compute_topology(&elements);
    assert_eq!(topology.root_count, 1);
    assert_eq!(topology.max_depth, 0);
}
