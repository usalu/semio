//! 🔺️ En1995 artifact — sparse field diff runtime.

use crate::artifacts::en1995::schema::diff::*;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::en1995::schema::En1995Artifact;
use crate::artifacts::en1995::En1995Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1995Diff {
    pub fn apply_to_artifact(&self, artifact: &En1995Artifact) -> En1995Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
if let Some(value) = &self.annex { next.annex = value.clone(); }
        if let Some(value) = &self.m_ed_knm { next.m_ed_knm = value.clone(); }
        if let Some(value) = &self.n_ed_kn { next.n_ed_kn = value.clone(); }
        if let Some(value) = &self.v_ed_kn { next.v_ed_kn = value.clone(); }
        if let Some(value) = &self.w_mm3 { next.w_mm3 = value.clone(); }
        if let Some(value) = &self.a_mm2 { next.a_mm2 = value.clone(); }
        if let Some(value) = &self.b_mm { next.b_mm = value.clone(); }
        if let Some(value) = &self.h_mm { next.h_mm = value.clone(); }
        if let Some(value) = &self.f_m_k { next.f_m_k = value.clone(); }
        if let Some(value) = &self.f_c_0_k { next.f_c_0_k = value.clone(); }
        if let Some(value) = &self.service_class { next.service_class = value.clone(); }
        if let Some(value) = &self.load_duration { next.load_duration = value.clone(); }
        if let Some(value) = &self.m_crit_knm { next.m_crit_knm = value.clone(); }
        if let Some(value) = &self.f_ed_kn { next.f_ed_kn = value.clone(); }
        if let Some(value) = &self.a_ef_mm2 { next.a_ef_mm2 = value.clone(); }
        if let Some(value) = &self.f_v_k { next.f_v_k = value.clone(); }
        if let Some(value) = &self.fire_duration_min { next.fire_duration_min = value.clone(); }
        if let Some(value) = &self.section_depth_mm { next.section_depth_mm = value.clone(); }
        if let Some(value) = &self.a_vert_m_s2 { next.a_vert_m_s2 = value.clone(); }
        if let Some(value) = &self.n_cycles_bridge { next.n_cycles_bridge = value.clone(); }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<En1995Snapshot> for En1995Diff {
    fn apply(&self, snapshot: &En1995Snapshot) -> En1995Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
if let Some(value) = &self.annex { next.annex = value.clone(); }
        if let Some(value) = &self.m_ed_knm { next.m_ed_knm = value.clone(); }
        if let Some(value) = &self.n_ed_kn { next.n_ed_kn = value.clone(); }
        if let Some(value) = &self.v_ed_kn { next.v_ed_kn = value.clone(); }
        if let Some(value) = &self.w_mm3 { next.w_mm3 = value.clone(); }
        if let Some(value) = &self.a_mm2 { next.a_mm2 = value.clone(); }
        if let Some(value) = &self.b_mm { next.b_mm = value.clone(); }
        if let Some(value) = &self.h_mm { next.h_mm = value.clone(); }
        if let Some(value) = &self.f_m_k { next.f_m_k = value.clone(); }
        if let Some(value) = &self.f_c_0_k { next.f_c_0_k = value.clone(); }
        if let Some(value) = &self.service_class { next.service_class = value.clone(); }
        if let Some(value) = &self.load_duration { next.load_duration = value.clone(); }
        if let Some(value) = &self.m_crit_knm { next.m_crit_knm = value.clone(); }
        if let Some(value) = &self.f_ed_kn { next.f_ed_kn = value.clone(); }
        if let Some(value) = &self.a_ef_mm2 { next.a_ef_mm2 = value.clone(); }
        if let Some(value) = &self.f_v_k { next.f_v_k = value.clone(); }
        if let Some(value) = &self.fire_duration_min { next.fire_duration_min = value.clone(); }
        if let Some(value) = &self.section_depth_mm { next.section_depth_mm = value.clone(); }
        if let Some(value) = &self.a_vert_m_s2 { next.a_vert_m_s2 = value.clone(); }
        if let Some(value) = &self.n_cycles_bridge { next.n_cycles_bridge = value.clone(); }
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
        take!(n_ed_kn);
        take!(v_ed_kn);
        take!(w_mm3);
        take!(a_mm2);
        take!(b_mm);
        take!(h_mm);
        take!(f_m_k);
        take!(f_c_0_k);
        take!(service_class);
        take!(load_duration);
        take!(m_crit_knm);
        take!(f_ed_kn);
        take!(a_ef_mm2);
        take!(f_v_k);
        take!(fire_duration_min);
        take!(section_depth_mm);
        take!(a_vert_m_s2);
        take!(n_cycles_bridge);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1995Snapshot) -> En1995Diff {
    En1995Diff {
        artifact: Some(Box::new(En1995Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1995::mutations::En1995Mutation;
    use protocol::{Mutation as _, MutationDiff};

    #[test]
    fn set_snapshot_diff_replaces_the_whole_snapshot() {
        let base = En1995Snapshot::default();
        let mutation = En1995Mutation::SetSnapshot { snapshot: En1995Snapshot::default() };
        let diff = mutation.diff(&base);
        assert_eq!(diff.apply(&base), En1995Snapshot::default());
    }
}
//#endregion 🧪️Tests
