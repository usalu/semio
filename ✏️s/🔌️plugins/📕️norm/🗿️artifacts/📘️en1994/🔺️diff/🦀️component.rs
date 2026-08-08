//! 🔺️ En1994 artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::en1994::schema::En1994Artifact;
use crate::artifacts::en1994::En1994Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1994Diff {
    pub fn apply_to_artifact(&self, artifact: &En1994Artifact) -> En1994Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(value) = self.annex { next.annex = value; }
        if let Some(value) = self.m_ed_knm { next.m_ed_knm = value; }
        if let Some(value) = self.v_ed_kn { next.v_ed_kn = value; }
        if let Some(value) = self.m_pla { next.m_pla = value; }
        if let Some(value) = self.m_pl_rd { next.m_pl_rd = value; }
        if let Some(value) = self.eta { next.eta = value; }
        if let Some(value) = self.v_l_rd { next.v_l_rd = value; }
        if let Some(value) = self.insulation_thickness_mm { next.insulation_thickness_mm = value; }
        if let Some(value) = &self.fire_rating { next.fire_rating = value.clone(); }
        if let Some(value) = &self.deck_type { next.deck_type = value.clone(); }
        if let Some(value) = self.delta_sigma_mpa { next.delta_sigma_mpa = value; }
        if let Some(value) = &self.fatigue_detail { next.fatigue_detail = value.clone(); }
        if let Some(value) = self.d_mm { next.d_mm = value; }
        if let Some(value) = self.h_sc_mm { next.h_sc_mm = value; }
        if let Some(value) = self.f_ck_mpa { next.f_ck_mpa = value; }
        if let Some(value) = self.f_u_mpa { next.f_u_mpa = value; }
        if let Some(value) = self.e_cm_mpa { next.e_cm_mpa = value; }
        if let Some(value) = self.v_ed_per_stud_kn { next.v_ed_per_stud_kn = value; }
        if let Some(value) = self.span_m { next.span_m = value; }
        if let Some(value) = self.f_y_mpa { next.f_y_mpa = value; }
        if let Some(value) = self.n_cycles_stud { next.n_cycles_stud = value; }
        if let Some(value) = self.delta_tau_stud_mpa { next.delta_tau_stud_mpa = value; }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<En1994Snapshot> for En1994Diff {
    fn apply(&self, snapshot: &En1994Snapshot) -> En1994Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(value) = self.annex { next.annex = value; }
        if let Some(value) = self.m_ed_knm { next.m_ed_knm = value; }
        if let Some(value) = self.v_ed_kn { next.v_ed_kn = value; }
        if let Some(value) = self.m_pla { next.m_pla = value; }
        if let Some(value) = self.m_pl_rd { next.m_pl_rd = value; }
        if let Some(value) = self.eta { next.eta = value; }
        if let Some(value) = self.v_l_rd { next.v_l_rd = value; }
        if let Some(value) = self.insulation_thickness_mm { next.insulation_thickness_mm = value; }
        if let Some(value) = &self.fire_rating { next.fire_rating = value.clone(); }
        if let Some(value) = &self.deck_type { next.deck_type = value.clone(); }
        if let Some(value) = self.delta_sigma_mpa { next.delta_sigma_mpa = value; }
        if let Some(value) = &self.fatigue_detail { next.fatigue_detail = value.clone(); }
        if let Some(value) = self.d_mm { next.d_mm = value; }
        if let Some(value) = self.h_sc_mm { next.h_sc_mm = value; }
        if let Some(value) = self.f_ck_mpa { next.f_ck_mpa = value; }
        if let Some(value) = self.f_u_mpa { next.f_u_mpa = value; }
        if let Some(value) = self.e_cm_mpa { next.e_cm_mpa = value; }
        if let Some(value) = self.v_ed_per_stud_kn { next.v_ed_per_stud_kn = value; }
        if let Some(value) = self.span_m { next.span_m = value; }
        if let Some(value) = self.f_y_mpa { next.f_y_mpa = value; }
        if let Some(value) = self.n_cycles_stud { next.n_cycles_stud = value; }
        if let Some(value) = self.delta_tau_stud_mpa { next.delta_tau_stud_mpa = value; }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(annex);
        take!(m_ed_knm);
        take!(v_ed_kn);
        take!(m_pla);
        take!(m_pl_rd);
        take!(eta);
        take!(v_l_rd);
        take!(insulation_thickness_mm);
        take!(fire_rating);
        take!(deck_type);
        take!(delta_sigma_mpa);
        take!(fatigue_detail);
        take!(d_mm);
        take!(h_sc_mm);
        take!(f_ck_mpa);
        take!(f_u_mpa);
        take!(e_cm_mpa);
        take!(v_ed_per_stud_kn);
        take!(span_m);
        take!(f_y_mpa);
        take!(n_cycles_stud);
        take!(delta_tau_stud_mpa);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1994Snapshot) -> En1994Diff {
    En1994Diff {
        artifact: Some(Box::new(En1994Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers
