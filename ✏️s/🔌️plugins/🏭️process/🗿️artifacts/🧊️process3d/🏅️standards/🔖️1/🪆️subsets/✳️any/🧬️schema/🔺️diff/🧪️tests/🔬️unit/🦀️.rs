use super::*;
use crate::WorkshopMachine;
use protocol::MutationDiff as _;

#[semio_framework_async_macros::async_test]
async fn absorbing_coalesces_same_key_entries_and_later_scalars_win() {
    let base = crate::empty_process3d_snapshot();
    let machine = base.workshop.machines[0].clone();
    let at = base.workshop.machines.len();
    let extra = WorkshopMachine { id: "extra".into(), ..machine.clone() };
    let middle = [base.workshop.machines.clone(), vec![extra.clone()]].concat();
    let created = Process3dDiff { stock_label: Some("First".into()), workshop: Some(Process3dMachinesDelta::insertion(at, extra)), ..Default::default() };
    let deleted = Process3dDiff { stock_label: Some("Beam".into()), workshop: Some(Process3dMachinesDelta::removal(&middle, at)), ..Default::default() };
    let mut diff = created;
    diff.absorb(deleted);
    assert!(diff.workshop.as_ref().is_none_or(Process3dMachinesDelta::is_empty), "create∘delete cancels");
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

#[semio_framework_async_macros::async_test]
async fn positional_machine_delta_moves_and_inverts_row_by_row() {
    let mut base = crate::empty_process3d_snapshot();
    let machine = base.workshop.machines[0].clone();
    base.workshop.machines = ["a", "b", "c", "d"].iter().map(|id| WorkshopMachine { id: (*id).into(), ..machine.clone() }).collect();
    let diff = |delta: Process3dMachinesDelta| Process3dDiff { workshop: Some(delta), ..Default::default() };
    let relocate = diff(Process3dMachinesDelta::relocation(&base.workshop.machines, 3, 1));
    let moved = protocol::apply_diff(&relocate, &base).expect("valid relocation");
    assert_eq!(moved.workshop.machines.iter().map(|row| row.id.as_str()).collect::<Vec<_>>(), ["a", "d", "b", "c"]);
    let inverse = protocol::DiffAlgebra::inverse(&relocate, &base);
    assert_eq!(protocol::apply_diff(&inverse, &moved).expect("valid inverse"), base);
}
