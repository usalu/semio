//! 🧬️ En1994 artifact — document mutation dispatch for the composite structure subject.

use crate::{En1994Diff, En1994Snapshot};

//#region 🔖️Leaves
use super::change_annex;
use super::change_structure_kind;
use super::change_steel_fy_pa;
use super::change_fire_rating;
use super::change_insulation_thickness_m;
use super::change_fatigue_detail;
use super::insert_beam;
use super::remove_beam;
use super::change_beam_action_q_area_pa;
use super::change_beam_stud_spacing_m;
use super::change_beam_span_m;
use super::change_beam_slab_thickness_m;
use super::change_beam_stud_diameter_m;
use super::change_beam_stud_count;
use super::change_beam_stud_fu_pa;
use super::change_beam_transverse_as;
use super::change_beam_construction;
use super::insert_column;
use super::remove_column;
use super::change_column_action_force_n;
use super::change_column_kind;
use super::insert_slab;
use super::remove_slab;
use super::change_slab_action_q_area_pa;
use super::change_slab_thickness_m;
//#endregion 🔖️Leaves

//#region 🔖️Mutations
#[derive(Clone, Debug, PartialEq, dsl::Mutations, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutations(snapshot = En1994Snapshot, diff = En1994Diff, schema = "s.norm.en1994")]
pub enum En1994Mutation {
    ChangeAnnex(change_annex::ChangeAnnex),
    ChangeStructureKind(change_structure_kind::ChangeStructureKind),
    ChangeSteelFYPa(change_steel_fy_pa::ChangeSteelFYPa),
    ChangeFireRating(change_fire_rating::ChangeFireRating),
    ChangeInsulationThicknessM(change_insulation_thickness_m::ChangeInsulationThicknessM),
    ChangeFatigueDetail(change_fatigue_detail::ChangeFatigueDetail),
    InsertBeam(insert_beam::InsertBeam),
    RemoveBeam(remove_beam::RemoveBeam),
    ChangeBeamActionQAreaPa(change_beam_action_q_area_pa::ChangeBeamActionQAreaPa),
    ChangeBeamStudSpacingM(change_beam_stud_spacing_m::ChangeBeamStudSpacingM),
    ChangeBeamSpanM(change_beam_span_m::ChangeBeamSpanM),
    ChangeBeamSlabThicknessM(change_beam_slab_thickness_m::ChangeBeamSlabThicknessM),
    ChangeBeamStudDiameterM(change_beam_stud_diameter_m::ChangeBeamStudDiameterM),
    ChangeBeamStudCount(change_beam_stud_count::ChangeBeamStudCount),
    ChangeBeamStudFUPa(change_beam_stud_fu_pa::ChangeBeamStudFUPa),
    ChangeBeamTransverseAs(change_beam_transverse_as::ChangeBeamTransverseAs),
    ChangeBeamConstruction(change_beam_construction::ChangeBeamConstruction),
    InsertColumn(insert_column::InsertColumn),
    RemoveColumn(remove_column::RemoveColumn),
    ChangeColumnActionForceN(change_column_action_force_n::ChangeColumnActionForceN),
    ChangeColumnKind(change_column_kind::ChangeColumnKind),
    InsertSlab(insert_slab::InsertSlab),
    RemoveSlab(remove_slab::RemoveSlab),
    ChangeSlabActionQAreaPa(change_slab_action_q_area_pa::ChangeSlabActionQAreaPa),
    ChangeSlabThicknessM(change_slab_thickness_m::ChangeSlabThicknessM),
}

pub const KINDS: &[&str] = &[
    "change-annex",
    "change-structure-kind",
    "change-steel-fy-pa",
    "change-fire-rating",
    "change-insulation-thickness-m",
    "change-fatigue-detail",
    "insert-beam",
    "remove-beam",
    "change-beam-action-q-area-pa",
    "change-beam-stud-spacing-m",
    "change-beam-span-m",
    "change-beam-slab-thickness-m",
    "change-beam-stud-diameter-m",
    "change-beam-stud-count",
    "change-beam-stud-fu-pa",
    "change-beam-transverse-as",
    "change-beam-construction",
    "insert-column",
    "remove-column",
    "change-column-action-force-n",
    "change-column-kind",
    "insert-slab",
    "remove-slab",
    "change-slab-action-q-area-pa",
    "change-slab-thickness-m",
];
//#endregion 🔖️Mutations

//#region 🔖️FromSnapshot
impl En1994Mutation {
    /// 📤️ Diff `base` → `target` into a semantic mutation bundle (B2 undoable edit).
    pub fn from_snapshot(base: &En1994Snapshot, target: &En1994Snapshot) -> Vec<En1994Mutation> {
        let mut out = Vec::new();
        if base.annex != target.annex {
            out.push(En1994Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: target.annex }));
        }
        if base.structure_kind != target.structure_kind {
            out.push(En1994Mutation::ChangeStructureKind(change_structure_kind::ChangeStructureKind {
                new_structure_kind: target.structure_kind.clone(),
            }));
        }
        if (base.steel_f_y_pa - target.steel_f_y_pa).abs() > f64::EPSILON {
            out.push(En1994Mutation::ChangeSteelFYPa(change_steel_fy_pa::ChangeSteelFYPa { new_steel_f_y_pa: target.steel_f_y_pa }));
        }
        if base.fire_rating != target.fire_rating {
            out.push(En1994Mutation::ChangeFireRating(change_fire_rating::ChangeFireRating { new_fire_rating: target.fire_rating.clone() }));
        }
        if (base.insulation_thickness_m - target.insulation_thickness_m).abs() > f64::EPSILON {
            out.push(En1994Mutation::ChangeInsulationThicknessM(change_insulation_thickness_m::ChangeInsulationThicknessM {
                new_insulation_thickness_m: target.insulation_thickness_m,
            }));
        }
        if base.fatigue_detail != target.fatigue_detail {
            out.push(En1994Mutation::ChangeFatigueDetail(change_fatigue_detail::ChangeFatigueDetail {
                new_fatigue_detail: target.fatigue_detail.clone(),
            }));
        }

        for index in (target.beams.len()..base.beams.len()).rev() {
            out.push(En1994Mutation::RemoveBeam(remove_beam::RemoveBeam { index }));
        }
        let shared_beams = base.beams.len().min(target.beams.len());
        for index in 0..shared_beams {
            let before = &base.beams[index];
            let after = &target.beams[index];
            if before == after {
                continue;
            }
            for (action_index, (ba, aa)) in before.actions.iter().zip(after.actions.iter()).enumerate() {
                if (ba.q_area_pa - aa.q_area_pa).abs() > f64::EPSILON {
                    out.push(En1994Mutation::ChangeBeamActionQAreaPa(change_beam_action_q_area_pa::ChangeBeamActionQAreaPa {
                        index,
                        action_index,
                        new_q_area_pa: aa.q_area_pa,
                    }));
                }
            }
            if (before.studs.spacing_m - after.studs.spacing_m).abs() > f64::EPSILON {
                out.push(En1994Mutation::ChangeBeamStudSpacingM(change_beam_stud_spacing_m::ChangeBeamStudSpacingM {
                    index,
                    new_spacing_m: after.studs.spacing_m,
                }));
            }
            if (before.span_m - after.span_m).abs() > f64::EPSILON {
                out.push(En1994Mutation::ChangeBeamSpanM(change_beam_span_m::ChangeBeamSpanM { index, new_span_m: after.span_m }));
            }
            if (before.slab_thickness_m - after.slab_thickness_m).abs() > f64::EPSILON {
                out.push(En1994Mutation::ChangeBeamSlabThicknessM(change_beam_slab_thickness_m::ChangeBeamSlabThicknessM {
                    index,
                    new_slab_thickness_m: after.slab_thickness_m,
                }));
            }
            if (before.studs.diameter_m - after.studs.diameter_m).abs() > f64::EPSILON {
                out.push(En1994Mutation::ChangeBeamStudDiameterM(change_beam_stud_diameter_m::ChangeBeamStudDiameterM {
                    index,
                    new_diameter_m: after.studs.diameter_m,
                }));
            }
            if before.studs.total_count != after.studs.total_count {
                out.push(En1994Mutation::ChangeBeamStudCount(change_beam_stud_count::ChangeBeamStudCount {
                    index,
                    new_total_count: after.studs.total_count,
                }));
            }
            if (before.studs.f_u_pa - after.studs.f_u_pa).abs() > f64::EPSILON {
                out.push(En1994Mutation::ChangeBeamStudFUPa(change_beam_stud_fu_pa::ChangeBeamStudFUPa {
                    index,
                    new_f_u_pa: after.studs.f_u_pa,
                }));
            }
            if (before.transverse_as_m2_per_m - after.transverse_as_m2_per_m).abs() > f64::EPSILON {
                out.push(En1994Mutation::ChangeBeamTransverseAs(change_beam_transverse_as::ChangeBeamTransverseAs {
                    index,
                    new_transverse_as_m2_per_m: after.transverse_as_m2_per_m,
                }));
            }
            if before.construction != after.construction {
                out.push(En1994Mutation::ChangeBeamConstruction(change_beam_construction::ChangeBeamConstruction {
                    index,
                    new_construction: after.construction.clone(),
                }));
            }
            let mut probe = before.clone();
            probe.actions = after.actions.clone();
            probe.studs.spacing_m = after.studs.spacing_m;
            probe.span_m = after.span_m;
            probe.slab_thickness_m = after.slab_thickness_m;
            probe.studs.diameter_m = after.studs.diameter_m;
            probe.studs.total_count = after.studs.total_count;
            probe.studs.f_u_pa = after.studs.f_u_pa;
            probe.transverse_as_m2_per_m = after.transverse_as_m2_per_m;
            probe.construction = after.construction.clone();
            if probe != *after {
                out.push(En1994Mutation::RemoveBeam(remove_beam::RemoveBeam { index }));
                out.push(En1994Mutation::InsertBeam(insert_beam::InsertBeam { index, beam: after.clone() }));
            }
        }
        for (index, beam) in target.beams.iter().enumerate().skip(shared_beams) {
            out.push(En1994Mutation::InsertBeam(insert_beam::InsertBeam { index, beam: beam.clone() }));
        }

        for index in (target.columns.len()..base.columns.len()).rev() {
            out.push(En1994Mutation::RemoveColumn(remove_column::RemoveColumn { index }));
        }
        let shared_cols = base.columns.len().min(target.columns.len());
        for index in 0..shared_cols {
            let before = &base.columns[index];
            let after = &target.columns[index];
            if before == after {
                continue;
            }
            for (action_index, (ba, aa)) in before.actions.iter().zip(after.actions.iter()).enumerate() {
                if (ba.n_k_n - aa.n_k_n).abs() > f64::EPSILON {
                    out.push(En1994Mutation::ChangeColumnActionForceN(change_column_action_force_n::ChangeColumnActionForceN {
                        index,
                        action_index,
                        new_n_k_n: aa.n_k_n,
                    }));
                }
            }
            if before.kind != after.kind {
                out.push(En1994Mutation::ChangeColumnKind(change_column_kind::ChangeColumnKind {
                    index,
                    new_kind: after.kind.clone(),
                }));
            }
            let mut probe = before.clone();
            probe.actions = after.actions.clone();
            probe.kind = after.kind.clone();
            if probe != *after {
                out.push(En1994Mutation::RemoveColumn(remove_column::RemoveColumn { index }));
                out.push(En1994Mutation::InsertColumn(insert_column::InsertColumn { index, column: after.clone() }));
            }
        }
        for (index, column) in target.columns.iter().enumerate().skip(shared_cols) {
            out.push(En1994Mutation::InsertColumn(insert_column::InsertColumn { index, column: column.clone() }));
        }

        for index in (target.slabs.len()..base.slabs.len()).rev() {
            out.push(En1994Mutation::RemoveSlab(remove_slab::RemoveSlab { index }));
        }
        let shared_slabs = base.slabs.len().min(target.slabs.len());
        for index in 0..shared_slabs {
            let before = &base.slabs[index];
            let after = &target.slabs[index];
            if before == after {
                continue;
            }
            for (action_index, (ba, aa)) in before.actions.iter().zip(after.actions.iter()).enumerate() {
                if (ba.q_area_pa - aa.q_area_pa).abs() > f64::EPSILON {
                    out.push(En1994Mutation::ChangeSlabActionQAreaPa(change_slab_action_q_area_pa::ChangeSlabActionQAreaPa {
                        index,
                        action_index,
                        new_q_area_pa: aa.q_area_pa,
                    }));
                }
            }
            if (before.concrete_thickness_m - after.concrete_thickness_m).abs() > f64::EPSILON {
                out.push(En1994Mutation::ChangeSlabThicknessM(change_slab_thickness_m::ChangeSlabThicknessM {
                    index,
                    new_concrete_thickness_m: after.concrete_thickness_m,
                }));
            }
            let mut probe = before.clone();
            probe.actions = after.actions.clone();
            probe.concrete_thickness_m = after.concrete_thickness_m;
            if probe != *after {
                out.push(En1994Mutation::RemoveSlab(remove_slab::RemoveSlab { index }));
                out.push(En1994Mutation::InsertSlab(insert_slab::InsertSlab { index, slab: after.clone() }));
            }
        }
        for (index, slab) in target.slabs.iter().enumerate().skip(shared_slabs) {
            out.push(En1994Mutation::InsertSlab(insert_slab::InsertSlab { index, slab: slab.clone() }));
        }

        out
    }
}
//#endregion 🔖️FromSnapshot


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️kinds-catalog/🦀️.rs"]
mod kinds_catalog;
//#endregion 🧪️Tests

//#region 🌉️ExternalCodecBridge
pub fn decode_en1994_mutation_json(text: &str) -> Result<En1994Mutation, String> {
    pack::json::from_json_str(text).map_err(|e| e.to_string())
}
pub fn encode_en1994_mutation_json(mutation: &En1994Mutation) -> String {
    pack::json::to_json_string(mutation)
}
//#endregion 🌉️ExternalCodecBridge

#[cfg(test)]
pub use tests::demo_mutation_cases;
