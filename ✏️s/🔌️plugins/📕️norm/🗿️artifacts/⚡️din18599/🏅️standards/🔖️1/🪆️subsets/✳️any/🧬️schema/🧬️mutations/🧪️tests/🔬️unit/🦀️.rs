use super::*;
use crate::standards::v1::subsets::any::io::{apply_din18599_mutation, inverse_din18599_mutation};

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
    let inv = inverse_din18599_mutation(&mutation, &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inv.len(), 1);
}


