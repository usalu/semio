//! 🔺️ En1997 artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::en1997::schema::En1997Artifact;
use crate::artifacts::en1997::En1997Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1997Diff {
    pub fn apply_to_artifact(&self, artifact: &En1997Artifact) -> En1997Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
if let Some(value) = &self.v_ed_kn { next.v_ed_kn = value.clone(); }
        if let Some(value) = &self.h_ed_kn { next.h_ed_kn = value.clone(); }
        if let Some(value) = &self.footing_area_m2 { next.footing_area_m2 = value.clone(); }
        if let Some(value) = &self.phi_deg { next.phi_deg = value.clone(); }
        if let Some(value) = &self.c_kpa { next.c_kpa = value.clone(); }
        if let Some(value) = &self.gamma_kn_m3 { next.gamma_kn_m3 = value.clone(); }
        if let Some(value) = &self.b_m { next.b_m = value.clone(); }
        if let Some(value) = &self.d_f_m { next.d_f_m = value.clone(); }
        if let Some(value) = &self.e_s_mpa { next.e_s_mpa = value.clone(); }
        if let Some(value) = &self.nu { next.nu = value.clone(); }
        if let Some(value) = &self.design_approach { next.design_approach = value.clone(); }
        if let Some(value) = &self.annex { next.annex = value.clone(); }
        if let Some(value) = &self.settlement_limit_mm { next.settlement_limit_mm = value.clone(); }
        if let Some(value) = &self.n_pile_ed_kn { next.n_pile_ed_kn = value.clone(); }
        if let Some(value) = &self.alpha_s { next.alpha_s = value.clone(); }
        if let Some(value) = &self.pile_d_m { next.pile_d_m = value.clone(); }
        if let Some(value) = &self.q_s_kpa { next.q_s_kpa = value.clone(); }
        if let Some(value) = &self.pile_l_m { next.pile_l_m = value.clone(); }
        if let Some(value) = &self.q_b_kpa { next.q_b_kpa = value.clone(); }
        if let Some(value) = &self.pile_base_area_m2 { next.pile_base_area_m2 = value.clone(); }
        if let Some(value) = &self.pile_n_profiles { next.pile_n_profiles = value.clone(); }
        if let Some(value) = &self.z_investigated_m { next.z_investigated_m = value.clone(); }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<En1997Snapshot> for En1997Diff {
    fn apply(&self, snapshot: &En1997Snapshot) -> En1997Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
if let Some(value) = self.v_ed_kn { next.v_ed_kn = value; }
        if let Some(value) = self.h_ed_kn { next.h_ed_kn = value; }
        if let Some(value) = self.footing_area_m2 { next.footing_area_m2 = value; }
        if let Some(value) = self.phi_deg { next.phi_deg = value; }
        if let Some(value) = self.c_kpa { next.c_kpa = value; }
        if let Some(value) = self.gamma_kn_m3 { next.gamma_kn_m3 = value; }
        if let Some(value) = self.b_m { next.b_m = value; }
        if let Some(value) = self.d_f_m { next.d_f_m = value; }
        if let Some(value) = self.e_s_mpa { next.e_s_mpa = value; }
        if let Some(value) = self.nu { next.nu = value; }
        if let Some(value) = self.design_approach { next.design_approach = value; }
        if let Some(value) = self.annex { next.annex = value; }
        if let Some(value) = self.settlement_limit_mm { next.settlement_limit_mm = value; }
        if let Some(value) = self.n_pile_ed_kn { next.n_pile_ed_kn = value; }
        if let Some(value) = self.alpha_s { next.alpha_s = value; }
        if let Some(value) = self.pile_d_m { next.pile_d_m = value; }
        if let Some(value) = self.q_s_kpa { next.q_s_kpa = value; }
        if let Some(value) = self.pile_l_m { next.pile_l_m = value; }
        if let Some(value) = self.q_b_kpa { next.q_b_kpa = value; }
        if let Some(value) = self.pile_base_area_m2 { next.pile_base_area_m2 = value; }
        if let Some(value) = self.pile_n_profiles { next.pile_n_profiles = value; }
        if let Some(value) = self.z_investigated_m { next.z_investigated_m = value; }
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
        take!(v_ed_kn);
        take!(h_ed_kn);
        take!(footing_area_m2);
        take!(phi_deg);
        take!(c_kpa);
        take!(gamma_kn_m3);
        take!(b_m);
        take!(d_f_m);
        take!(e_s_mpa);
        take!(nu);
        take!(design_approach);
        take!(annex);
        take!(settlement_limit_mm);
        take!(n_pile_ed_kn);
        take!(alpha_s);
        take!(pile_d_m);
        take!(q_s_kpa);
        take!(pile_l_m);
        take!(q_b_kpa);
        take!(pile_base_area_m2);
        take!(pile_n_profiles);
        take!(z_investigated_m);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1997Snapshot) -> En1997Diff {
    En1997Diff {
        artifact: Some(Box::new(En1997Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1997::mutations::En1997Mutation;
    use protocol::{Mutation as _, MutationDiff};

    #[test]
    fn set_snapshot_diff_replaces_the_whole_snapshot() {
        let base = En1997Snapshot::default();
        let mutation = En1997Mutation::SetSnapshot { snapshot: En1997Snapshot::default() };
        let diff = mutation.diff(&base);
        assert_eq!(diff.apply(&base), En1997Snapshot::default());
    }
}
//#endregion 🧪️Tests
