use super::*;

#[test]
fn change_element_u_round_trips_via_apply() {
    let base = crate::Din18599Snapshot::default();
    let id = base.elements[0].id.clone();
    let mutation = Din18599Mutation::ChangeElementU(change_element_u::ChangeElementU {
        element_id: id.clone(),
        new_u_value_w_m2k: 0.22,
    });
    let (after, _) = apply_din18599_mutation(&base, &mutation).expect("apply");
    let el = after.elements.iter().find(|e| e.id == id).unwrap();
    assert!((el.u_value_w_m2k - 0.22).abs() < 1e-12);
    let inv = inverse_din18599_mutation(&mutation, &base);
    assert_eq!(inv.len(), 1);
}

#[test]
fn from_snapshot_emits_diffs_only() {
    let base = crate::Din18599Snapshot::default();
    let mut target = base.clone();
    target.net_floor_area_m2 = 200.0;
    let mutations = Din18599Mutation::from_snapshot(&base, &target);
    assert_eq!(mutations.len(), 1);
    assert!(matches!(mutations[0], Din18599Mutation::ChangeNetFloorAreaM2(_)));
}


