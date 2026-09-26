use super::*;
use crate::document::AnnexChoice;
use crate::{CompositeBeam, CompositeColumn, CompositeSlab};
use protocol::{Mutation, SemanticMutation};

fn round_trip(base: &En1994Snapshot, operation: &En1994Mutation) -> En1994Snapshot {
    let (forward, _messages) = protocol::apply_mutation(base, operation).expect("valid mutation");
    let mut restored = forward.clone();
    for back in operation.inverse(base) {
        let (next, _messages) = protocol::apply_mutation(&restored, &back).expect("valid inverse mutation");
        restored = next;
    }
    assert_eq!(&restored, base, "inverse(base) must restore the pre-mutation document");
    forward
}

pub fn demo_mutation_cases() -> Vec<En1994Mutation> {
    vec![
        En1994Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: AnnexChoice::En }),
        En1994Mutation::ChangeStructureKind(change_structure_kind::ChangeStructureKind { new_structure_kind: "bridge".into() }),
        En1994Mutation::ChangeSteelFYPa(change_steel_fy_pa::ChangeSteelFYPa { new_steel_f_y_pa: 460e6 }),
        En1994Mutation::ChangeFireRating(change_fire_rating::ChangeFireRating { new_fire_rating: "r90".into() }),
        En1994Mutation::ChangeInsulationThicknessM(change_insulation_thickness_m::ChangeInsulationThicknessM { new_insulation_thickness_m: 0.028 }),
        En1994Mutation::ChangeFatigueDetail(change_fatigue_detail::ChangeFatigueDetail { new_fatigue_detail: "flange_butt_weld".into() }),
        En1994Mutation::InsertBeam(insert_beam::InsertBeam { index: 0, beam: CompositeBeam::default_placeholder() }),
        En1994Mutation::RemoveBeam(remove_beam::RemoveBeam { index: 0 }),
        En1994Mutation::ChangeBeamActionQAreaPa(change_beam_action_q_area_pa::ChangeBeamActionQAreaPa { index: 0, action_index: 0, new_q_area_pa: 3.0e3 }),
        En1994Mutation::ChangeBeamStudSpacingM(change_beam_stud_spacing_m::ChangeBeamStudSpacingM { index: 0, new_spacing_m: 0.20 }),
        En1994Mutation::ChangeBeamSpanM(change_beam_span_m::ChangeBeamSpanM { index: 0, new_span_m: 10.0 }),
        En1994Mutation::ChangeBeamSlabThicknessM(change_beam_slab_thickness_m::ChangeBeamSlabThicknessM { index: 0, new_slab_thickness_m: 0.16 }),
        En1994Mutation::ChangeBeamStudDiameterM(change_beam_stud_diameter_m::ChangeBeamStudDiameterM { index: 0, new_diameter_m: 0.022 }),
        En1994Mutation::ChangeBeamStudCount(change_beam_stud_count::ChangeBeamStudCount { index: 0, new_total_count: 55 }),
        En1994Mutation::ChangeBeamStudFUPa(change_beam_stud_fu_pa::ChangeBeamStudFUPa { index: 0, new_f_u_pa: 500e6 }),
        En1994Mutation::ChangeBeamTransverseAs(change_beam_transverse_as::ChangeBeamTransverseAs { index: 0, new_transverse_as_m2_per_m: 4e-4 }),
        En1994Mutation::ChangeBeamConstruction(change_beam_construction::ChangeBeamConstruction { index: 0, new_construction: "unpropped".into() }),
        En1994Mutation::InsertColumn(insert_column::InsertColumn { index: 0, column: CompositeColumn::default_placeholder() }),
        En1994Mutation::RemoveColumn(remove_column::RemoveColumn { index: 0 }),
        En1994Mutation::ChangeColumnActionForceN(change_column_action_force_n::ChangeColumnActionForceN { index: 0, action_index: 0, new_n_k_n: 2500e3 }),
        En1994Mutation::ChangeColumnKind(change_column_kind::ChangeColumnKind { index: 0, new_kind: "partially_encased".into() }),
        En1994Mutation::InsertSlab(insert_slab::InsertSlab { index: 0, slab: CompositeSlab::default_placeholder() }),
        En1994Mutation::RemoveSlab(remove_slab::RemoveSlab { index: 0 }),
        En1994Mutation::ChangeSlabActionQAreaPa(change_slab_action_q_area_pa::ChangeSlabActionQAreaPa { index: 0, action_index: 0, new_q_area_pa: 4.0e3 }),
        En1994Mutation::ChangeSlabThicknessM(change_slab_thickness_m::ChangeSlabThicknessM { index: 0, new_concrete_thickness_m: 0.16 }),
    ]
}

#[test]
fn every_kind_round_trips_through_inverse() {
    let base = En1994Snapshot::default();
    for op in demo_mutation_cases() {
        let _ = round_trip(&base, &op);
    }
}

