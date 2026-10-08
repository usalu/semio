use super::*;
use crate::WorkshopMachine;
use protocol::MutationDiff as _;

#[semio_framework_async_macros::async_test]
async fn absorbing_coalesces_same_key_entries_and_later_scalars_win() {
    let base = crate::empty_process3d_snapshot();
    let machine = base.workshop.machines[0].clone();
    let created = Process3dDiff { stock_label: Some("First".into()), workshop: Some(Process3dMachinesDelta { added: vec![WorkshopMachine { id: "extra".into(), ..machine.clone() }], ..Default::default() }), ..Default::default() };
    let deleted = Process3dDiff { stock_label: Some("Beam".into()), workshop: Some(Process3dMachinesDelta { removed: vec!["extra".into()], ..Default::default() }), ..Default::default() };
    let mut diff = created;
    diff.absorb(deleted);
    assert!(diff.workshop.as_ref().is_none_or(keyed_is_empty), "create∘delete cancels");
    assert_eq!(protocol::apply_diff(&diff, &base).expect("valid mutation diff").stock_label, "Beam");
}

#[semio_framework_async_macros::async_test]
async fn stock_solid_handle_swap_applies() {
    let base = crate::empty_process3d_snapshot();
    let new_content = crate::brep_snapshot_for_working_solid(&crate::WorkingSolid::Sphere { radius: 0.5 });
    let new_handle = crate::brep_child_handle("stock", &new_content);
    let diff = Process3dDiff { stock_solid: Some(new_handle.clone()), ..Default::default() };
    let next = protocol::apply_diff(&diff, &base).expect("valid mutation diff");
    assert_eq!(next.stock_solid, new_handle);
}
