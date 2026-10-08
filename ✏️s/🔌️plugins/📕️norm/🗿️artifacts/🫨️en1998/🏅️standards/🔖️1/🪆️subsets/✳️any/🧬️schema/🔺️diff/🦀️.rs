//! 🧬️ En1998 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_artifact_norm_contract::{norm_list_delta, norm_row_patch};

use crate::En1998Snapshot;

//#region 🔖️Rows
norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998System`.
    pub En1998SystemPatch of crate::En1998System { set { direction: String, system_type: String, material: String, ductility_class: String, q0: f64, alpha_u_over_alpha_1: f64, k_w: f64, base_shear_resistance_n: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998System` list.
    pub En1998SystemDelta { addition: En1998SystemAddition, modification: En1998SystemModification, row: crate::En1998System, patch: En1998SystemPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Storey`.
    pub En1998StoreyPatch of crate::En1998Storey { set { height_m: f64, permanent_gk_n: f64, correlated_occupancy: bool, variables: Vec<crate::En1998VariableAction>, stiffness_x: f64, stiffness_y: f64, centre_of_mass_x_m: f64, centre_of_mass_y_m: f64, centre_of_stiffness_x_m: f64, centre_of_stiffness_y_m: f64, drift_x_m: f64, drift_y_m: f64, shear_resistance_n: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Storey` list.
    pub En1998StoreyDelta { addition: En1998StoreyAddition, modification: En1998StoreyModification, row: crate::En1998Storey, patch: En1998StoreyPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Member`.
    pub En1998MemberPatch of crate::En1998Member { set { material: String, role: String, detailing_compatible_with_q: bool, min_dimension_m: f64, rho: f64, rho_prime: f64, omega_wd: f64, steel_section_class: u8 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Member` list.
    pub En1998MemberDelta { addition: En1998MemberAddition, modification: En1998MemberModification, row: crate::En1998Member, patch: En1998MemberPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Building`.
    pub En1998BuildingPatch of crate::En1998Building { set { name: String, plan_width_m: f64, plan_length_m: f64, plan_regular: bool, elevation_regular: bool, t1_method: String, t1_given_s: f64, ct: f64, drift_limit_class: String, nu: f64, multiple_resisting_systems: bool, claims_simple_masonry: bool, masonry_wall_area_ratio: f64, accidental_eccentricity_ratio: f64 } nest { systems: En1998SystemDelta, storeys: En1998StoreyDelta, members: En1998MemberDelta } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Building` list.
    pub En1998BuildingDelta { addition: En1998BuildingAddition, modification: En1998BuildingModification, row: crate::En1998Building, patch: En1998BuildingPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Bridge`.
    pub En1998BridgePatch of crate::En1998Bridge { set { period_ratio: f64, fundamental_period_s: f64, v_rd_n: f64, bearing_d_rd_m: f64, permanent_gk_n: f64, correlated_occupancy: bool, variables: Vec<crate::En1998VariableAction> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Bridge` list.
    pub En1998BridgeDelta { addition: En1998BridgeAddition, modification: En1998BridgeModification, row: crate::En1998Bridge, patch: En1998BridgePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Assessment`.
    pub En1998AssessmentPatch of crate::En1998Assessment { set { knowledge_level: String, limit_state: String, supported_building_id: String, r_k_n: f64, gamma_el: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Assessment` list.
    pub En1998AssessmentDelta { addition: En1998AssessmentAddition, modification: En1998AssessmentModification, row: crate::En1998Assessment, patch: En1998AssessmentPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Silo`.
    pub En1998SiloPatch of crate::En1998Silo { set { height_m: f64, radius_m: f64, permanent_gk_n: f64, content_qk_n: f64, content_category: String, filling_ratio: f64, n_rd_n: f64, v_rd_n: f64, q_nominal: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Silo` list.
    pub En1998SiloDelta { addition: En1998SiloAddition, modification: En1998SiloModification, row: crate::En1998Silo, patch: En1998SiloPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Tank`.
    pub En1998TankPatch of crate::En1998Tank { set { height_m: f64, radius_m: f64, permanent_gk_n: f64, content_qk_n: f64, content_category: String, filling_ratio: f64, v_rd_n: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Tank` list.
    pub En1998TankDelta { addition: En1998TankAddition, modification: En1998TankModification, row: crate::En1998Tank, patch: En1998TankPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Foundation`.
    pub En1998FoundationPatch of crate::En1998Foundation { set { supported_building_id: String, area_m2: f64, p_rd_pa: f64, h_rd_n: f64, k_foundation: f64, k_soil: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Foundation` list.
    pub En1998FoundationDelta { addition: En1998FoundationAddition, modification: En1998FoundationModification, row: crate::En1998Foundation, patch: En1998FoundationPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998RetainingWall`.
    pub En1998RetainingWallPatch of crate::En1998RetainingWall { set { height_m: f64, phi_deg: f64, soil_gamma: f64, r: f64, h_rd_n_per_m: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998RetainingWall` list.
    pub En1998RetainingWallDelta { addition: En1998RetainingWallAddition, modification: En1998RetainingWallModification, row: crate::En1998RetainingWall, patch: En1998RetainingWallPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `En1998Tower`.
    pub En1998TowerPatch of crate::En1998Tower { set { height_m: f64, m_rd_nm: f64, is_chimney: bool, q_nominal: f64, permanent_gk_n: f64, correlated_occupancy: bool, variables: Vec<crate::En1998VariableAction> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `En1998Tower` list.
    pub En1998TowerDelta { addition: En1998TowerAddition, modification: En1998TowerModification, row: crate::En1998Tower, patch: En1998TowerPatch, key: id }
}
//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1998 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Diff {
    #[state(artifact)]
    pub annex: Option<String>,
    #[state(artifact)]
    pub site: Option<crate::En1998Site>,
    #[state(artifact)]
    pub buildings: En1998BuildingDelta,
    #[state(artifact)]
    pub bridges: En1998BridgeDelta,
    #[state(artifact)]
    pub assessments: En1998AssessmentDelta,
    #[state(artifact)]
    pub silos: En1998SiloDelta,
    #[state(artifact)]
    pub tanks: En1998TankDelta,
    #[state(artifact)]
    pub foundations: En1998FoundationDelta,
    #[state(artifact)]
    pub retaining_walls: En1998RetainingWallDelta,
    #[state(artifact)]
    pub towers: En1998TowerDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<En1998Snapshot> for En1998Diff {
    fn apply(&self, base: &En1998Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1998Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.site {
            next.site = value.clone();
        }
        next.buildings = self.buildings.commit_onto(&base.buildings).map_err(|error| error.under(["buildings"]))?;
        next.bridges = self.bridges.commit_onto(&base.bridges).map_err(|error| error.under(["bridges"]))?;
        next.assessments = self.assessments.commit_onto(&base.assessments).map_err(|error| error.under(["assessments"]))?;
        next.silos = self.silos.commit_onto(&base.silos).map_err(|error| error.under(["silos"]))?;
        next.tanks = self.tanks.commit_onto(&base.tanks).map_err(|error| error.under(["tanks"]))?;
        next.foundations = self.foundations.commit_onto(&base.foundations).map_err(|error| error.under(["foundations"]))?;
        next.retaining_walls = self.retaining_walls.commit_onto(&base.retaining_walls).map_err(|error| error.under(["retainingWalls"]))?;
        next.towers = self.towers.commit_onto(&base.towers).map_err(|error| error.under(["towers"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.site.is_some() {
            self.site = other.site;
        }
        self.buildings.absorb(other.buildings);
        self.bridges.absorb(other.bridges);
        self.assessments.absorb(other.assessments);
        self.silos.absorb(other.silos);
        self.tanks.absorb(other.tanks);
        self.foundations.absorb(other.foundations);
        self.retaining_walls.absorb(other.retaining_walls);
        self.towers.absorb(other.towers);
    }
}

impl DiffAlgebra<En1998Snapshot> for En1998Diff {
    fn inverse(&self, base: &En1998Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            site: self.site.as_ref().map(|_| base.site.clone()),
            buildings: self.buildings.inverse(&base.buildings),
            bridges: self.bridges.inverse(&base.bridges),
            assessments: self.assessments.inverse(&base.assessments),
            silos: self.silos.inverse(&base.silos),
            tanks: self.tanks.inverse(&base.tanks),
            foundations: self.foundations.inverse(&base.foundations),
            retaining_walls: self.retaining_walls.inverse(&base.retaining_walls),
            towers: self.towers.inverse(&base.towers),
        }
    }

    fn between(base: &En1998Snapshot, other: &En1998Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then(|| other.annex.clone()),
            site: (base.site != other.site).then(|| other.site.clone()),
            buildings: En1998BuildingDelta::between(&base.buildings, &other.buildings),
            bridges: En1998BridgeDelta::between(&base.bridges, &other.bridges),
            assessments: En1998AssessmentDelta::between(&base.assessments, &other.assessments),
            silos: En1998SiloDelta::between(&base.silos, &other.silos),
            tanks: En1998TankDelta::between(&base.tanks, &other.tanks),
            foundations: En1998FoundationDelta::between(&base.foundations, &other.foundations),
            retaining_walls: En1998RetainingWallDelta::between(&base.retaining_walls, &other.retaining_walls),
            towers: En1998TowerDelta::between(&base.towers, &other.towers),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.site.is_none()
            && self.buildings.is_empty()
            && self.bridges.is_empty()
            && self.assessments.is_empty()
            && self.silos.is_empty()
            && self.tanks.is_empty()
            && self.foundations.is_empty()
            && self.retaining_walls.is_empty()
            && self.towers.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
