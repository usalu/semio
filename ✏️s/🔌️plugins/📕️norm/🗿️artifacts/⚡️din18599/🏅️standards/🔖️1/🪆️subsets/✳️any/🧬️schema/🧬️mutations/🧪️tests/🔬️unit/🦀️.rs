use crate::*;
use crate::mutation_bridge::{apply_din18599_mutation, inverse_din18599_mutation};

#[test]
fn change_element_u_round_trips_via_apply() {
    let base = semio_s_artifact_norm_din18599_model::Din18599Snapshot::default();
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


fn applied(base: &semio_s_artifact_norm_din18599_model::Din18599Snapshot, edit: &semio_s_plugin_norm_app_surface::NormEdit) -> semio_s_artifact_norm_din18599_model::Din18599Snapshot {
    resolve_edit(base, edit).expect("resolves").iter().fold(base.clone(), |document, mutation| apply_din18599_mutation(&document, mutation).expect("apply").0)
}

#[test]
fn inserting_a_zone_and_an_element_appends_rows_with_fresh_ids() {
    use semio_s_plugin_norm_app_surface::NormEdit;
    let base = semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house();
    let zones = applied(&base, &NormEdit::InsertItem { path: "zones".into(), index: usize::MAX, value: None });
    assert_eq!(zones.zones.len(), base.zones.len() + 1);
    let ids: std::collections::BTreeSet<_> = zones.zones.iter().map(|z| z.id.clone()).collect();
    assert_eq!(ids.len(), zones.zones.len(), "ids stay unique");
    let elements = applied(&base, &NormEdit::InsertItem { path: "elements".into(), index: 0, value: None });
    assert_eq!(elements.elements.len(), base.elements.len() + 1);
    assert!(elements.zones.iter().any(|z| z.id == elements.elements[0].zone_id), "the new element stands on a zone");
}

#[test]
fn removing_a_zone_removes_the_elements_standing_on_it() {
    use semio_s_plugin_norm_app_surface::NormEdit;
    let base = semio_s_artifact_norm_din18599_model::subjects::compliant_two_zone_house();
    let gone = base.zones[0].id.clone();
    let after = applied(&base, &NormEdit::RemoveItem { path: format!("zones[id={gone}]"), index: 0 });
    assert_eq!(after.zones.len(), base.zones.len() - 1);
    assert!(after.elements.iter().all(|e| e.zone_id != gone));
    let element = base.elements[0].id.clone();
    let after = applied(&base, &NormEdit::RemoveItem { path: format!("elements[id={element}]"), index: 0 });
    assert!(after.elements.iter().all(|e| e.id != element));
    assert_eq!(after.elements.len(), base.elements.len() - 1);
}
