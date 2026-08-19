//! 🔺️ En1999 artifact — sparse field diff runtime.

use crate::artifacts::en1999::schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::artifacts::en1999::schema::En1999Artifact;
use crate::artifacts::en1999::En1999Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1999Diff {
    pub async fn apply_to_artifact(&self, artifact: &En1999Artifact) -> protocol::MutationApplyResult<En1999Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.n_ed_kn {
                next.n_ed_kn = value.clone();
            }
            if let Some(value) = &self.m_ed_knm {
                next.m_ed_knm = value.clone();
            }
            if let Some(value) = &self.a_mm2 {
                next.a_mm2 = value.clone();
            }
            if let Some(value) = &self.w_el_mm3 {
                next.w_el_mm3 = value.clone();
            }
            if let Some(value) = &self.alloy {
                next.alloy = value.clone();
            }
            if let Some(value) = &self.chi {
                next.chi = value.clone();
            }
            if let Some(value) = &self.i_t_mm4 {
                next.i_t_mm4 = value.clone();
            }
            if let Some(value) = &self.l_cr_mm {
                next.l_cr_mm = value.clone();
            }
            if let Some(value) = &self.theta_c {
                next.theta_c = value.clone();
            }
            if let Some(value) = &self.delta_sigma_ed {
                next.delta_sigma_ed = value.clone();
            }
            if let Some(value) = &self.delta_sigma_c {
                next.delta_sigma_c = value.clone();
            }
            if let Some(value) = &self.fatigue_m {
                next.fatigue_m = value.clone();
            }
            if let Some(value) = &self.n_cycles {
                next.n_cycles = value.clone();
            }
            if let Some(value) = &self.v_weld_ed_kn {
                next.v_weld_ed_kn = value.clone();
            }
            if let Some(value) = &self.weld_throat_mm {
                next.weld_throat_mm = value.clone();
            }
            if let Some(value) = &self.weld_length_mm {
                next.weld_length_mm = value.clone();
            }
            if let Some(value) = &self.beta_w {
                next.beta_w = value.clone();
            }
            if let Some(value) = &self.sheet_b_mm {
                next.sheet_b_mm = value.clone();
            }
            if let Some(value) = &self.sheet_t_mm {
                next.sheet_t_mm = value.clone();
            }
            if let Some(value) = &self.sheet_k_sigma {
                next.sheet_k_sigma = value.clone();
            }
            if let Some(value) = &self.sheet_w_el_mm3 {
                next.sheet_w_el_mm3 = value.clone();
            }
            if let Some(value) = &self.sheet_m_ed_knm {
                next.sheet_m_ed_knm = value.clone();
            }
            if let Some(value) = &self.shell_t_mm {
                next.shell_t_mm = value.clone();
            }
            if let Some(value) = &self.shell_r_mm {
                next.shell_r_mm = value.clone();
            }
            if let Some(value) = &self.sigma_ed_shell_mpa {
                next.sigma_ed_shell_mpa = value.clone();
            }
            if let Some(value) = &self.annex {
                next.annex = value.clone();
            }
            if let Some(value) = &self.selected_check_index {
                next.selected_check_index = *value;
            }
            next
        })
    }
}

impl MutationDiff<En1999Snapshot> for En1999Diff {
    async fn apply(&self, snapshot: &En1999Snapshot) -> protocol::MutationApplyResult<En1999Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.n_ed_kn {
                next.n_ed_kn = value.clone();
            }
            if let Some(value) = &self.m_ed_knm {
                next.m_ed_knm = value.clone();
            }
            if let Some(value) = &self.a_mm2 {
                next.a_mm2 = value.clone();
            }
            if let Some(value) = &self.w_el_mm3 {
                next.w_el_mm3 = value.clone();
            }
            if let Some(value) = &self.alloy {
                next.alloy = value.clone();
            }
            if let Some(value) = &self.chi {
                next.chi = value.clone();
            }
            if let Some(value) = &self.i_t_mm4 {
                next.i_t_mm4 = value.clone();
            }
            if let Some(value) = &self.l_cr_mm {
                next.l_cr_mm = value.clone();
            }
            if let Some(value) = &self.theta_c {
                next.theta_c = value.clone();
            }
            if let Some(value) = &self.delta_sigma_ed {
                next.delta_sigma_ed = value.clone();
            }
            if let Some(value) = &self.delta_sigma_c {
                next.delta_sigma_c = value.clone();
            }
            if let Some(value) = &self.fatigue_m {
                next.fatigue_m = value.clone();
            }
            if let Some(value) = &self.n_cycles {
                next.n_cycles = value.clone();
            }
            if let Some(value) = &self.v_weld_ed_kn {
                next.v_weld_ed_kn = value.clone();
            }
            if let Some(value) = &self.weld_throat_mm {
                next.weld_throat_mm = value.clone();
            }
            if let Some(value) = &self.weld_length_mm {
                next.weld_length_mm = value.clone();
            }
            if let Some(value) = &self.beta_w {
                next.beta_w = value.clone();
            }
            if let Some(value) = &self.sheet_b_mm {
                next.sheet_b_mm = value.clone();
            }
            if let Some(value) = &self.sheet_t_mm {
                next.sheet_t_mm = value.clone();
            }
            if let Some(value) = &self.sheet_k_sigma {
                next.sheet_k_sigma = value.clone();
            }
            if let Some(value) = &self.sheet_w_el_mm3 {
                next.sheet_w_el_mm3 = value.clone();
            }
            if let Some(value) = &self.sheet_m_ed_knm {
                next.sheet_m_ed_knm = value.clone();
            }
            if let Some(value) = &self.shell_t_mm {
                next.shell_t_mm = value.clone();
            }
            if let Some(value) = &self.shell_r_mm {
                next.shell_r_mm = value.clone();
            }
            if let Some(value) = &self.sigma_ed_shell_mpa {
                next.sigma_ed_shell_mpa = value.clone();
            }
            if let Some(value) = &self.annex {
                next.annex = value.clone();
            }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
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
        take!(n_ed_kn);
        take!(m_ed_knm);
        take!(a_mm2);
        take!(w_el_mm3);
        take!(alloy);
        take!(chi);
        take!(i_t_mm4);
        take!(l_cr_mm);
        take!(theta_c);
        take!(delta_sigma_ed);
        take!(delta_sigma_c);
        take!(fatigue_m);
        take!(n_cycles);
        take!(v_weld_ed_kn);
        take!(weld_throat_mm);
        take!(weld_length_mm);
        take!(beta_w);
        take!(sheet_b_mm);
        take!(sheet_t_mm);
        take!(sheet_k_sigma);
        take!(sheet_w_el_mm3);
        take!(sheet_m_ed_knm);
        take!(shell_t_mm);
        take!(shell_r_mm);
        take!(sigma_ed_shell_mpa);
        take!(annex);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub async fn diff_set_snapshot(snapshot: &En1999Snapshot) -> En1999Diff {
    En1999Diff { artifact: Some(Box::new(En1999Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1999::mutations::En1999Mutation;
    use protocol::{Mutation as _, MutationDiff};

    #[test]
    async fn change_mutation_diff_updates_only_its_field() {
        let base = En1999Snapshot::default();
        let mutation = En1999Mutation::ChangeNEdKn(crate::artifacts::en1999::mutations::change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: 95.0 });
        let outcome = mutation.diff(&base);
        let mut expected = base.clone();
        expected.n_ed_kn = 95.0;
        assert_eq!(outcome.diff().apply(&base).expect("valid mutation diff"), expected);
    }
}
//#endregion 🧪️Tests
