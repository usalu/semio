//! 🧪️ Keyed-diff algebra of Din18599: section-patch merging, row patches, list-setter rows and the negative diff.

use semio_s_artifact_norm_din18599_diff::Din18599Diff;
use semio_s_artifact_norm_din18599_mut_zoning::change_element_u::ChangeElementU;
use semio_s_artifact_norm_din18599_mut_zone_rows::insert_zone::InsertZone;
use semio_s_artifact_norm_din18599_mut_systems::update_renewables::UpdateRenewables;
use semio_s_artifact_norm_din18599_mutations::Din18599Mutation;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use protocol::{DiffAlgebra as _, Mutation as _, MutationDiff as _};
use protocol::ColdMutationDiff;

fn diff_of(mutation: &Din18599Mutation, base: &Din18599Snapshot) -> Din18599Diff {
    let raised = mutation.diff(base);
    assert!(raised.messages().is_empty(), "{mutation:?} raised {:?}", raised.messages());
    raised.diff().clone()
}

async fn law(base: &Din18599Snapshot, first: &Din18599Mutation, second: impl Fn(&Din18599Snapshot) -> Din18599Mutation) -> Din18599Diff {
    let d1 = diff_of(first, base);
    let mid = protocol::apply_diff_cold(&d1, base).expect("first");
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
    assert_eq!(rows.modified[0].patch.u_value_w_m2k, Some(0.27));
}

#[semio_framework_async_macros::async_test]
async fn a_zone_insertion_names_one_row_and_inverts() {
    let base = Din18599Snapshot::default();
    let mut extra = base.zones[0].clone();
    extra.id = "zone-extra".into();
    let mutation = Din18599Mutation::InsertZone(InsertZone { index: base.zones.len(), zone: extra.clone() });
    let forward = diff_of(&mutation, &base);
    let rows = forward.zones.as_ref().expect("zones");
    assert_eq!((rows.removed.len(), rows.inserted.len()), (0, 1));
    let after = protocol::apply_diff_cold(&forward, &base).expect("apply");
    assert_eq!(after.zones.last(), Some(&extra));
    assert_eq!(protocol::apply_diff_cold(&forward.inverse(&base), &after).expect("inverse"), base);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
}
