//! 🔺️ En1996 artifact — sparse field diff runtime.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

use crate::artifacts::en1996::schema::En1996Artifact;
use crate::artifacts::en1996::En1996Snapshot;
use protocol::MutationDiff;

//#region 🔖️Apply
impl En1996Diff {
    pub fn apply_to_artifact(&self, artifact: &En1996Artifact) -> En1996Artifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
if let Some(value) = &self.m_ed_knm { next.m_ed_knm = value.clone(); }
        if let Some(value) = &self.n_ed_kn { next.n_ed_kn = value.clone(); }
        if let Some(value) = &self.v_ed_kn { next.v_ed_kn = value.clone(); }
        if let Some(value) = &self.h_ed_kn { next.h_ed_kn = value.clone(); }
        if let Some(value) = &self.z_mm3 { next.z_mm3 = value.clone(); }
        if let Some(value) = &self.area_mm2 { next.area_mm2 = value.clone(); }
        if let Some(value) = &self.shear_area_mm2 { next.shear_area_mm2 = value.clone(); }
        if let Some(value) = &self.f_k_mpa { next.f_k_mpa = value.clone(); }
        if let Some(value) = &self.f_vk_mpa { next.f_vk_mpa = value.clone(); }
        if let Some(value) = &self.annex { next.annex = value.clone(); }
        if let Some(value) = &self.masonry_class { next.masonry_class = value.clone(); }
        if let Some(value) = &self.design_situation { next.design_situation = value.clone(); }
        if let Some(value) = &self.mu { next.mu = value.clone(); }
        if let Some(value) = &self.wall_thickness_mm { next.wall_thickness_mm = value.clone(); }
        if let Some(value) = &self.fire_resistance_min { next.fire_resistance_min = value.clone(); }
        if let Some(value) = &self.unit { next.unit = value.clone(); }
        if let Some(value) = &self.exposure { next.exposure = value.clone(); }
        if let Some(value) = &self.mortar { next.mortar = value.clone(); }
        if let Some(value) = &self.bed_joint_thickness_mm { next.bed_joint_thickness_mm = value.clone(); }
        if let Some(value) = &self.storeys { next.storeys = value.clone(); }
        if let Some(value) = &self.h_ef_mm { next.h_ef_mm = value.clone(); }
        if let Some(value) = &self.t_ef_mm { next.t_ef_mm = value.clone(); }
        if let Some(value) = &self.selected_check_index {
            next.selected_check_index = *value;
        }
        next
    }
}

impl MutationDiff<En1996Snapshot> for En1996Diff {
    fn apply(&self, snapshot: &En1996Snapshot) -> En1996Snapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
if let Some(value) = self.m_ed_knm { next.m_ed_knm = value; }
        if let Some(value) = self.n_ed_kn { next.n_ed_kn = value; }
        if let Some(value) = self.v_ed_kn { next.v_ed_kn = value; }
        if let Some(value) = self.h_ed_kn { next.h_ed_kn = value; }
        if let Some(value) = self.z_mm3 { next.z_mm3 = value; }
        if let Some(value) = self.area_mm2 { next.area_mm2 = value; }
        if let Some(value) = self.shear_area_mm2 { next.shear_area_mm2 = value; }
        if let Some(value) = self.f_k_mpa { next.f_k_mpa = value; }
        if let Some(value) = self.f_vk_mpa { next.f_vk_mpa = value; }
        if let Some(value) = self.annex { next.annex = value; }
        if let Some(value) = self.masonry_class { next.masonry_class = value; }
        if let Some(value) = self.design_situation { next.design_situation = value; }
        if let Some(value) = self.mu { next.mu = value; }
        if let Some(value) = self.wall_thickness_mm { next.wall_thickness_mm = value; }
        if let Some(value) = self.fire_resistance_min { next.fire_resistance_min = value; }
        if let Some(value) = self.unit { next.unit = value; }
        if let Some(value) = self.exposure { next.exposure = value; }
        if let Some(value) = self.mortar { next.mortar = value; }
        if let Some(value) = self.bed_joint_thickness_mm { next.bed_joint_thickness_mm = value; }
        if let Some(value) = self.storeys { next.storeys = value; }
        if let Some(value) = self.h_ef_mm { next.h_ef_mm = value; }
        if let Some(value) = self.t_ef_mm { next.t_ef_mm = value; }
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
        take!(m_ed_knm);
        take!(n_ed_kn);
        take!(v_ed_kn);
        take!(h_ed_kn);
        take!(z_mm3);
        take!(area_mm2);
        take!(shear_area_mm2);
        take!(f_k_mpa);
        take!(f_vk_mpa);
        take!(annex);
        take!(masonry_class);
        take!(design_situation);
        take!(mu);
        take!(wall_thickness_mm);
        take!(fire_resistance_min);
        take!(unit);
        take!(exposure);
        take!(mortar);
        take!(bed_joint_thickness_mm);
        take!(storeys);
        take!(h_ef_mm);
        take!(t_ef_mm);
        take!(selected_check_index);
    }
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
pub fn diff_set_snapshot(snapshot: &En1996Snapshot) -> En1996Diff {
    En1996Diff {
        artifact: Some(Box::new(En1996Artifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔖️Helpers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1996::mutations::En1996Mutation;
    use protocol::{Mutation as _, MutationDiff};

    #[test]
    fn set_snapshot_diff_replaces_the_whole_snapshot() {
        let base = En1996Snapshot::default();
        let mutation = En1996Mutation::SetSnapshot { snapshot: En1996Snapshot::default() };
        let diff = mutation.diff(&base);
        assert_eq!(diff.apply(&base), En1996Snapshot::default());
    }
}
//#endregion 🧪️Tests
