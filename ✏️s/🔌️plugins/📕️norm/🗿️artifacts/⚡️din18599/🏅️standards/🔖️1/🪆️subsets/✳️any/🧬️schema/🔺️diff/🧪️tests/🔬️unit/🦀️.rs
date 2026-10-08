//! 🧪️ Keyed-diff algebra of Din18599: section-patch merging, row patches, list-setter row diffs, the negative diff and the state delta.

use super::Din18599Diff;
use crate::mutations::change_element_u::ChangeElementU;
use crate::mutations::replace_zones::ReplaceZones;
use crate::mutations::update_renewables::UpdateRenewables;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};

fn diff_of(mutation: &Din18599Mutation, base: &Din18599Snapshot) -> Din18599Diff {
    let raised = mutation.diff(base);
    assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
    raised.diff().clone()
}

async fn law(base: &Din18599Snapshot, first: &Din18599Mutation, second: impl Fn(&Din18599Snapshot) -> Din18599Mutation) -> Din18599Diff {
    let d1 = diff_of(first, base);
    let mid = protocol::apply_diff(&d1, base).expect("first");
    let d2 = diff_of(&second(&mid), &mid);
    let mut sum = d1.clone();
    sum.absorb(d2.clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(base, d1, d2).await;
    sum
}

fn renewables(base: &Din18599Snapshot, area: f64, efficiency: f64) -> Din18599Mutation {
    let mut next = base.renewables.clone();
    next.pv_area_m2 = area;
    next.pv_efficiency = efficiency;
    Din18599Mutation::UpdateRenewables(UpdateRenewables { new_renewables: next })
}

#[semio_framework_async_macros::async_test]
async fn section_patches_merge_field_by_field() {
    let base = Din18599Snapshot::default();
    let sum = law(&base, &renewables(&base, base.renewables.pv_area_m2 + 3.0, base.renewables.pv_efficiency), |mid| renewables(mid, mid.renewables.pv_area_m2, mid.renewables.pv_efficiency + 0.01)).await;
    let patch = sum.renewables.expect("renewables");
    assert!(patch.pv_area_m2.is_some());
    assert!(patch.pv_efficiency.is_some());
    assert!(patch.solar_thermal_kwh_a.is_none());
}

#[semio_framework_async_macros::async_test]
async fn row_patches_on_one_element_coalesce() {
    let base = Din18599Snapshot::default();
    let id = base.elements[0].id.clone();
    let set = |value: f64| Din18599Mutation::ChangeElementU(ChangeElementU { element_id: id.clone(), new_u_value_w_m2k: value });
    let sum = law(&base, &set(0.31), |_| set(0.27)).await;
    let rows = sum.elements.expect("elements");
    assert_eq!(rows.modified.len(), 1);
    assert_eq!(rows.modified[0].u_value_w_m2k, Some(0.27));
}

#[semio_framework_async_macros::async_test]
async fn a_zone_list_setter_diffs_rows_by_identity_and_inverts() {
    let base = Din18599Snapshot::default();
    let mut zones = base.zones.clone();
    zones[0].area_m2 += 5.0;
    let mut extra = zones[0].clone();
    extra.id = "zone-extra".into();
    zones.push(extra);
    let mutation = Din18599Mutation::ReplaceZones(ReplaceZones { new_zones: zones.clone() });
    let forward = diff_of(&mutation, &base);
    let rows = forward.zones.as_ref().expect("zones");
    assert_eq!((rows.added.len(), rows.removed.len(), rows.modified.len()), (1, 0, 1));
    let after = protocol::apply_diff(&forward, &base).expect("apply");
    assert_eq!(after.zones, zones);
    assert_eq!(protocol::apply_diff(&forward.inverse(&base), &after).expect("inverse"), base);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    protocol::os_spr::protocol_laws::assert_diff_algebra_between_law::<Din18599Snapshot, Din18599Diff>(&base, &after).await;
}
