//! 🧬️ En1993 diff schema — sparse scalar fields plus keyed row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_artifact_norm_contract::{norm_list_delta, norm_row_patch};

use crate::En1993Snapshot;

//#region 🔖️Rows
norm_row_patch! {
    /// 🩹 Sparse field patch of one `SteelMaterial`.
    pub En1993MaterialPatch of crate::SteelMaterial { set { grade: String, fy: f64, fu: f64, e_modulus: f64, g_modulus: f64, subgrade: String, kind: String } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `SteelMaterial` list.
    pub En1993MaterialDelta { addition: En1993MaterialAddition, modification: En1993MaterialModification, row: crate::SteelMaterial, patch: En1993MaterialPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `SteelSection`.
    pub En1993SectionPatch of crate::SteelSection { set { designation: String, kind: String, h: f64, b: f64, tw: f64, tf: f64, r: f64, area: f64, shear_area_y: f64, shear_area_z: f64, iy: f64, iz: f64, it: f64, iw: f64, w_el_y: f64, w_el_z: f64, w_pl_y: f64, w_pl_z: f64, area_net: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `SteelSection` list.
    pub En1993SectionDelta { addition: En1993SectionAddition, modification: En1993SectionModification, row: crate::SteelSection, patch: En1993SectionPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `SteelMember`.
    pub En1993MemberPatch of crate::SteelMember { set { label: String, member_type: String, section_id: String, material_id: String, length: f64, buckling_length_y: f64, buckling_length_z: f64, ltb_length: f64, ltb_restraint_spacing: f64, load_application: String, end_moment_ratio_psi: f64, moment_diagram: String, deflection_limit_ratio: f64, analysis: String } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `SteelMember` list.
    pub En1993MemberDelta { addition: En1993MemberAddition, modification: En1993MemberModification, row: crate::SteelMember, patch: En1993MemberPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `LoadCase`.
    pub En1993LoadCasePatch of crate::LoadCase { set { name: String, kind: String, category: String } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `LoadCase` list.
    pub En1993LoadCaseDelta { addition: En1993LoadCaseAddition, modification: En1993LoadCaseModification, row: crate::LoadCase, patch: En1993LoadCasePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `MemberAction`.
    pub En1993MemberActionPatch of crate::MemberAction { set { member_id: String, load_case_id: String, action: crate::DesignAction } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `MemberAction` list.
    pub En1993MemberActionDelta { addition: En1993MemberActionAddition, modification: En1993MemberActionModification, row: crate::MemberAction, patch: En1993MemberActionPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `SteelJoint`.
    pub En1993JointPatch of crate::SteelJoint { set { kind: String, member_id: String, bolt_class: String, bolt_diameter: f64, bolt_rows: u32, bolts_per_row: u32, pitch: f64, gauge: f64, end_distance: f64, edge_distance: f64, shear_planes: u32, plate_thickness: f64, plate_fu: f64, weld_throat: f64, weld_length: f64, weld_fu: f64, weld_grade: String, actions: Vec<crate::JointForceAction>, category: String, friction_mu: f64, preload_force: f64, slip_factor_ks: f64, friction_surfaces: u32 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `SteelJoint` list.
    pub En1993JointDelta { addition: En1993JointAddition, modification: En1993JointModification, row: crate::SteelJoint, patch: En1993JointPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `FatigueDetail`.
    pub En1993FatigueDetailPatch of crate::FatigueDetail { set { member_id: String, category: u8, method: String, spectrum: Vec<crate::FatigueBand> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `FatigueDetail` list.
    pub En1993FatigueDetailDelta { addition: En1993FatigueDetailAddition, modification: En1993FatigueDetailModification, row: crate::FatigueDetail, patch: En1993FatigueDetailPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `FireExposure`.
    pub En1993FireExposurePatch of crate::FireExposure { set { member_id: String, rating: String, protection_thickness: f64, section_factor: f64, mu0: f64, design_temperature: f64, protection_conductivity: f64, protection_density: f64, protection_specific_heat: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `FireExposure` list.
    pub En1993FireExposureDelta { addition: En1993FireExposureAddition, modification: En1993FireExposureModification, row: crate::FireExposure, patch: En1993FireExposurePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `ColdFormedMember`.
    pub En1993ColdFormedMemberPatch of crate::ColdFormedMember { set { b_bar: f64, thickness: f64, k_sigma: f64, psi: f64, fy: f64, gross_resistance: f64, actions: Vec<crate::ForceAction> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `ColdFormedMember` list.
    pub En1993ColdFormedMemberDelta { addition: En1993ColdFormedMemberAddition, modification: En1993ColdFormedMemberModification, row: crate::ColdFormedMember, patch: En1993ColdFormedMemberPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `PlatedPanel`.
    pub En1993PlatedPanelPatch of crate::PlatedPanel { set { a: f64, b: f64, thickness: f64, fy: f64, k_sigma: f64, actions: Vec<crate::ForceAction> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `PlatedPanel` list.
    pub En1993PlatedPanelDelta { addition: En1993PlatedPanelAddition, modification: En1993PlatedPanelModification, row: crate::PlatedPanel, patch: En1993PlatedPanelPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `SiloShell`.
    pub En1993SiloShellPatch of crate::SiloShell { set { thickness: f64, radius: f64, depth: f64, k: f64, gamma: f64, fy: f64 } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `SiloShell` list.
    pub En1993SiloShellDelta { addition: En1993SiloShellAddition, modification: En1993SiloShellModification, row: crate::SiloShell, patch: En1993SiloShellPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `TensionComponent`.
    pub En1993TensionComponentPatch of crate::TensionComponent { set { f_uk: f64, f_k: f64, actions: Vec<crate::ForceAction> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `TensionComponent` list.
    pub En1993TensionComponentDelta { addition: En1993TensionComponentAddition, modification: En1993TensionComponentModification, row: crate::TensionComponent, patch: En1993TensionComponentPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `BridgeFatigue`.
    pub En1993BridgeFatiguePatch of crate::BridgeFatigue { set { member_id: String, lambda: f64, phi2: f64, delta_sigma_p: f64, category: u8, method: String } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `BridgeFatigue` list.
    pub En1993BridgeFatigueDelta { addition: En1993BridgeFatigueAddition, modification: En1993BridgeFatigueModification, row: crate::BridgeFatigue, patch: En1993BridgeFatiguePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `TowerLeg`.
    pub En1993TowerLegPatch of crate::TowerLeg { set { member_id: String, force_coefficient: f64, dynamic_factor: f64, actions: Vec<crate::ForceAction> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `TowerLeg` list.
    pub En1993TowerLegDelta { addition: En1993TowerLegAddition, modification: En1993TowerLegModification, row: crate::TowerLeg, patch: En1993TowerLegPatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `SteelPile`.
    pub En1993PilePatch of crate::SteelPile { set { section_id: String, material_id: String, driving_stress: f64, embedded_length: f64, shaft_perimeter: f64, actions: Vec<crate::ForceAction> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `SteelPile` list.
    pub En1993PileDelta { addition: En1993PileAddition, modification: En1993PileModification, row: crate::SteelPile, patch: En1993PilePatch, key: id }
}

norm_row_patch! {
    /// 🩹 Sparse field patch of one `CraneRunway`.
    pub En1993CraneRunwayPatch of crate::CraneRunway { set { member_id: String, wheel_contact_length: f64, dispersion: f64, web_thickness: f64, fy: f64, phi: f64, actions: Vec<crate::ForceAction> } }
}

norm_list_delta! {
    /// 📋️ Keyed row delta of one `CraneRunway` list.
    pub En1993CraneRunwayDelta { addition: En1993CraneRunwayAddition, modification: En1993CraneRunwayModification, row: crate::CraneRunway, patch: En1993CraneRunwayPatch, key: id }
}
//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse delta for the En1993 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1993")]
pub struct En1993Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub materials: En1993MaterialDelta,
    #[state(artifact)]
    pub sections: En1993SectionDelta,
    #[state(artifact)]
    pub members: En1993MemberDelta,
    #[state(artifact)]
    pub load_cases: En1993LoadCaseDelta,
    #[state(artifact)]
    pub member_actions: En1993MemberActionDelta,
    #[state(artifact)]
    pub joints: En1993JointDelta,
    #[state(artifact)]
    pub fatigue_details: En1993FatigueDetailDelta,
    #[state(artifact)]
    pub fire_exposures: En1993FireExposureDelta,
    #[state(artifact)]
    pub cold_formed_members: En1993ColdFormedMemberDelta,
    #[state(artifact)]
    pub plated_panels: En1993PlatedPanelDelta,
    #[state(artifact)]
    pub silo_shells: En1993SiloShellDelta,
    #[state(artifact)]
    pub tension_components: En1993TensionComponentDelta,
    #[state(artifact)]
    pub bridge_fatigue: En1993BridgeFatigueDelta,
    #[state(artifact)]
    pub tower_legs: En1993TowerLegDelta,
    #[state(artifact)]
    pub piles: En1993PileDelta,
    #[state(artifact)]
    pub crane_runways: En1993CraneRunwayDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<En1993Snapshot> for En1993Diff {
    fn apply(&self, base: &En1993Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1993Snapshot> {
        let mut next = base.clone();
        if let Some(value) = self.annex {
            next.annex = value;
        }
        next.materials = self.materials.commit_onto(&base.materials).map_err(|error| error.under(["materials"]))?;
        next.sections = self.sections.commit_onto(&base.sections).map_err(|error| error.under(["sections"]))?;
        next.members = self.members.commit_onto(&base.members).map_err(|error| error.under(["members"]))?;
        next.load_cases = self.load_cases.commit_onto(&base.load_cases).map_err(|error| error.under(["loadCases"]))?;
        next.member_actions = self.member_actions.commit_onto(&base.member_actions).map_err(|error| error.under(["memberActions"]))?;
        next.joints = self.joints.commit_onto(&base.joints).map_err(|error| error.under(["joints"]))?;
        next.fatigue_details = self.fatigue_details.commit_onto(&base.fatigue_details).map_err(|error| error.under(["fatigueDetails"]))?;
        next.fire_exposures = self.fire_exposures.commit_onto(&base.fire_exposures).map_err(|error| error.under(["fireExposures"]))?;
        next.cold_formed_members = self.cold_formed_members.commit_onto(&base.cold_formed_members).map_err(|error| error.under(["coldFormedMembers"]))?;
        next.plated_panels = self.plated_panels.commit_onto(&base.plated_panels).map_err(|error| error.under(["platedPanels"]))?;
        next.silo_shells = self.silo_shells.commit_onto(&base.silo_shells).map_err(|error| error.under(["siloShells"]))?;
        next.tension_components = self.tension_components.commit_onto(&base.tension_components).map_err(|error| error.under(["tensionComponents"]))?;
        next.bridge_fatigue = self.bridge_fatigue.commit_onto(&base.bridge_fatigue).map_err(|error| error.under(["bridgeFatigue"]))?;
        next.tower_legs = self.tower_legs.commit_onto(&base.tower_legs).map_err(|error| error.under(["towerLegs"]))?;
        next.piles = self.piles.commit_onto(&base.piles).map_err(|error| error.under(["piles"]))?;
        next.crane_runways = self.crane_runways.commit_onto(&base.crane_runways).map_err(|error| error.under(["craneRunways"]))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        self.materials.absorb(other.materials);
        self.sections.absorb(other.sections);
        self.members.absorb(other.members);
        self.load_cases.absorb(other.load_cases);
        self.member_actions.absorb(other.member_actions);
        self.joints.absorb(other.joints);
        self.fatigue_details.absorb(other.fatigue_details);
        self.fire_exposures.absorb(other.fire_exposures);
        self.cold_formed_members.absorb(other.cold_formed_members);
        self.plated_panels.absorb(other.plated_panels);
        self.silo_shells.absorb(other.silo_shells);
        self.tension_components.absorb(other.tension_components);
        self.bridge_fatigue.absorb(other.bridge_fatigue);
        self.tower_legs.absorb(other.tower_legs);
        self.piles.absorb(other.piles);
        self.crane_runways.absorb(other.crane_runways);
    }
}

impl DiffAlgebra<En1993Snapshot> for En1993Diff {
    fn inverse(&self, base: &En1993Snapshot) -> Self {
        Self {
            annex: self.annex.map(|_| base.annex),
            materials: self.materials.inverse(&base.materials),
            sections: self.sections.inverse(&base.sections),
            members: self.members.inverse(&base.members),
            load_cases: self.load_cases.inverse(&base.load_cases),
            member_actions: self.member_actions.inverse(&base.member_actions),
            joints: self.joints.inverse(&base.joints),
            fatigue_details: self.fatigue_details.inverse(&base.fatigue_details),
            fire_exposures: self.fire_exposures.inverse(&base.fire_exposures),
            cold_formed_members: self.cold_formed_members.inverse(&base.cold_formed_members),
            plated_panels: self.plated_panels.inverse(&base.plated_panels),
            silo_shells: self.silo_shells.inverse(&base.silo_shells),
            tension_components: self.tension_components.inverse(&base.tension_components),
            bridge_fatigue: self.bridge_fatigue.inverse(&base.bridge_fatigue),
            tower_legs: self.tower_legs.inverse(&base.tower_legs),
            piles: self.piles.inverse(&base.piles),
            crane_runways: self.crane_runways.inverse(&base.crane_runways),
        }
    }

    fn between(base: &En1993Snapshot, other: &En1993Snapshot) -> Self {
        Self {
            annex: (base.annex != other.annex).then_some(other.annex),
            materials: En1993MaterialDelta::between(&base.materials, &other.materials),
            sections: En1993SectionDelta::between(&base.sections, &other.sections),
            members: En1993MemberDelta::between(&base.members, &other.members),
            load_cases: En1993LoadCaseDelta::between(&base.load_cases, &other.load_cases),
            member_actions: En1993MemberActionDelta::between(&base.member_actions, &other.member_actions),
            joints: En1993JointDelta::between(&base.joints, &other.joints),
            fatigue_details: En1993FatigueDetailDelta::between(&base.fatigue_details, &other.fatigue_details),
            fire_exposures: En1993FireExposureDelta::between(&base.fire_exposures, &other.fire_exposures),
            cold_formed_members: En1993ColdFormedMemberDelta::between(&base.cold_formed_members, &other.cold_formed_members),
            plated_panels: En1993PlatedPanelDelta::between(&base.plated_panels, &other.plated_panels),
            silo_shells: En1993SiloShellDelta::between(&base.silo_shells, &other.silo_shells),
            tension_components: En1993TensionComponentDelta::between(&base.tension_components, &other.tension_components),
            bridge_fatigue: En1993BridgeFatigueDelta::between(&base.bridge_fatigue, &other.bridge_fatigue),
            tower_legs: En1993TowerLegDelta::between(&base.tower_legs, &other.tower_legs),
            piles: En1993PileDelta::between(&base.piles, &other.piles),
            crane_runways: En1993CraneRunwayDelta::between(&base.crane_runways, &other.crane_runways),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.materials.is_empty()
            && self.sections.is_empty()
            && self.members.is_empty()
            && self.load_cases.is_empty()
            && self.member_actions.is_empty()
            && self.joints.is_empty()
            && self.fatigue_details.is_empty()
            && self.fire_exposures.is_empty()
            && self.cold_formed_members.is_empty()
            && self.plated_panels.is_empty()
            && self.silo_shells.is_empty()
            && self.tension_components.is_empty()
            && self.bridge_fatigue.is_empty()
            && self.tower_legs.is_empty()
            && self.piles.is_empty()
            && self.crane_runways.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
