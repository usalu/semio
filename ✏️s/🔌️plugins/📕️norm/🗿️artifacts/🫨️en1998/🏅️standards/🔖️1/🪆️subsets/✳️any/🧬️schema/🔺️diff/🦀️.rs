//! 🧬️ En1998 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};

use crate::En1998Snapshot;

//#region 🔖️Rows
protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998System`.
    pub En1998SystemPatch of crate::En1998System { set { direction: String, system_type: String, material: String, ductility_class: String, q0: f64, alpha_u_over_alpha_1: f64, k_w: f64, base_shear_resistance_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998System` list.
    pub En1998SystemDelta { removal: En1998SystemRemoval, insertion: En1998SystemInsertion, relocation: En1998SystemRelocation, modification: En1998SystemModification, row: crate::En1998System, patch: En1998SystemPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Storey`.
    pub En1998StoreyPatch of crate::En1998Storey { set { height_m: f64, permanent_gk_n: f64, correlated_occupancy: bool, variables: Vec<crate::En1998VariableAction>, stiffness_x: f64, stiffness_y: f64, centre_of_mass_x_m: f64, centre_of_mass_y_m: f64, centre_of_stiffness_x_m: f64, centre_of_stiffness_y_m: f64, drift_x_m: f64, drift_y_m: f64, shear_resistance_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Storey` list.
    pub En1998StoreyDelta { removal: En1998StoreyRemoval, insertion: En1998StoreyInsertion, relocation: En1998StoreyRelocation, modification: En1998StoreyModification, row: crate::En1998Storey, patch: En1998StoreyPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Member`.
    pub En1998MemberPatch of crate::En1998Member { set { material: String, role: String, detailing_compatible_with_q: bool, min_dimension_m: f64, rho: f64, rho_prime: f64, omega_wd: f64, steel_section_class: u8 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Member` list.
    pub En1998MemberDelta { removal: En1998MemberRemoval, insertion: En1998MemberInsertion, relocation: En1998MemberRelocation, modification: En1998MemberModification, row: crate::En1998Member, patch: En1998MemberPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Building`.
    pub En1998BuildingPatch of crate::En1998Building { set { name: String, plan_width_m: f64, plan_length_m: f64, plan_regular: bool, elevation_regular: bool, t1_method: String, t1_given_s: f64, ct: f64, drift_limit_class: String, nu: f64, multiple_resisting_systems: bool, claims_simple_masonry: bool, masonry_wall_area_ratio: f64, accidental_eccentricity_ratio: f64 } nest { systems: En1998SystemDelta, storeys: En1998StoreyDelta, members: En1998MemberDelta } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Building` list.
    pub En1998BuildingDelta { removal: En1998BuildingRemoval, insertion: En1998BuildingInsertion, relocation: En1998BuildingRelocation, modification: En1998BuildingModification, row: crate::En1998Building, patch: En1998BuildingPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Bridge`.
    pub En1998BridgePatch of crate::En1998Bridge { set { period_ratio: f64, fundamental_period_s: f64, v_rd_n: f64, bearing_d_rd_m: f64, permanent_gk_n: f64, correlated_occupancy: bool, variables: Vec<crate::En1998VariableAction> } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Bridge` list.
    pub En1998BridgeDelta { removal: En1998BridgeRemoval, insertion: En1998BridgeInsertion, relocation: En1998BridgeRelocation, modification: En1998BridgeModification, row: crate::En1998Bridge, patch: En1998BridgePatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Assessment`.
    pub En1998AssessmentPatch of crate::En1998Assessment { set { knowledge_level: String, limit_state: String, supported_building_id: String, r_k_n: f64, gamma_el: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Assessment` list.
    pub En1998AssessmentDelta { removal: En1998AssessmentRemoval, insertion: En1998AssessmentInsertion, relocation: En1998AssessmentRelocation, modification: En1998AssessmentModification, row: crate::En1998Assessment, patch: En1998AssessmentPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Silo`.
    pub En1998SiloPatch of crate::En1998Silo { set { height_m: f64, radius_m: f64, permanent_gk_n: f64, content_qk_n: f64, content_category: String, filling_ratio: f64, n_rd_n: f64, v_rd_n: f64, q_nominal: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Silo` list.
    pub En1998SiloDelta { removal: En1998SiloRemoval, insertion: En1998SiloInsertion, relocation: En1998SiloRelocation, modification: En1998SiloModification, row: crate::En1998Silo, patch: En1998SiloPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Tank`.
    pub En1998TankPatch of crate::En1998Tank { set { height_m: f64, radius_m: f64, permanent_gk_n: f64, content_qk_n: f64, content_category: String, filling_ratio: f64, v_rd_n: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Tank` list.
    pub En1998TankDelta { removal: En1998TankRemoval, insertion: En1998TankInsertion, relocation: En1998TankRelocation, modification: En1998TankModification, row: crate::En1998Tank, patch: En1998TankPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Foundation`.
    pub En1998FoundationPatch of crate::En1998Foundation { set { supported_building_id: String, area_m2: f64, p_rd_pa: f64, h_rd_n: f64, k_foundation: f64, k_soil: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Foundation` list.
    pub En1998FoundationDelta { removal: En1998FoundationRemoval, insertion: En1998FoundationInsertion, relocation: En1998FoundationRelocation, modification: En1998FoundationModification, row: crate::En1998Foundation, patch: En1998FoundationPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998RetainingWall`.
    pub En1998RetainingWallPatch of crate::En1998RetainingWall { set { height_m: f64, phi_deg: f64, soil_gamma: f64, r: f64, h_rd_n_per_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998RetainingWall` list.
    pub En1998RetainingWallDelta { removal: En1998RetainingWallRemoval, insertion: En1998RetainingWallInsertion, relocation: En1998RetainingWallRelocation, modification: En1998RetainingWallModification, row: crate::En1998RetainingWall, patch: En1998RetainingWallPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `En1998Tower`.
    pub En1998TowerPatch of crate::En1998Tower { set { height_m: f64, m_rd_nm: f64, is_chimney: bool, q_nominal: f64, permanent_gk_n: f64, correlated_occupancy: bool, variables: Vec<crate::En1998VariableAction> } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Keyed row delta of one `En1998Tower` list.
    pub En1998TowerDelta { removal: En1998TowerRemoval, insertion: En1998TowerInsertion, relocation: En1998TowerRelocation, modification: En1998TowerModification, row: crate::En1998Tower, patch: En1998TowerPatch, key: id }
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
    fn apply(&self, base: &En1998Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1998Snapshot> {
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.site {
            next.site = value.clone();
        }
        next.buildings = self.buildings.commit_onto(&base.buildings, capability).map_err(|error| error.under(["buildings"]))?;
        next.bridges = self.bridges.commit_onto(&base.bridges, capability).map_err(|error| error.under(["bridges"]))?;
        next.assessments = self.assessments.commit_onto(&base.assessments, capability).map_err(|error| error.under(["assessments"]))?;
        next.silos = self.silos.commit_onto(&base.silos, capability).map_err(|error| error.under(["silos"]))?;
        next.tanks = self.tanks.commit_onto(&base.tanks, capability).map_err(|error| error.under(["tanks"]))?;
        next.foundations = self.foundations.commit_onto(&base.foundations, capability).map_err(|error| error.under(["foundations"]))?;
        next.retaining_walls = self.retaining_walls.commit_onto(&base.retaining_walls, capability).map_err(|error| error.under(["retainingWalls"]))?;
        next.towers = self.towers.commit_onto(&base.towers, capability).map_err(|error| error.under(["towers"]))?;
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
