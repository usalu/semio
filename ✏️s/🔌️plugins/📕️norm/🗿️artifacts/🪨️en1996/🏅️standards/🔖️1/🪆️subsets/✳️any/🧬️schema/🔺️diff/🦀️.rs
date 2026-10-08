//! 🧬️ EN1996 diff schema — sparse scalar fields plus positional row deltas for every list the document owns.

use framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};

use crate::En1996Snapshot;

//#region 🔖️Rows
protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallOpening`.
    pub En1996OpeningPatch of crate::WallOpening { set { width_m: f64, height_m: f64, sill_height_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallOpening` list.
    pub En1996OpeningDelta { removal: En1996OpeningRemoval, insertion: En1996OpeningInsertion, relocation: En1996OpeningRelocation, modification: En1996OpeningModification, row: crate::WallOpening, patch: En1996OpeningPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `ConcentratedLoad`.
    pub En1996ConcentratedPatch of crate::ConcentratedLoad { set { force_n: f64, bearing_area_m2: f64, bearing_length_m: f64 } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `ConcentratedLoad` list.
    pub En1996ConcentratedDelta { removal: En1996ConcentratedRemoval, insertion: En1996ConcentratedInsertion, relocation: En1996ConcentratedRelocation, modification: En1996ConcentratedModification, row: crate::ConcentratedLoad, patch: En1996ConcentratedPatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `WallLoadCase`.
    pub En1996LoadCasePatch of crate::WallLoadCase { set { design_situation: String, imposed_category: String, g_k_slab_n: f64, q_k_imposed_pa: f64, tributary_area_m2: f64, slab_span_m: f64, q_k_snow_pa: f64, q_p_wind_pa: f64, c_pe: f64, h_k_earth_n: f64 } nest { concentrated: En1996ConcentratedDelta } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `WallLoadCase` list.
    pub En1996LoadCaseDelta { removal: En1996LoadCaseRemoval, insertion: En1996LoadCaseInsertion, relocation: En1996LoadCaseRelocation, modification: En1996LoadCaseModification, row: crate::WallLoadCase, patch: En1996LoadCasePatch, key: id }
}

protocol::row_patch! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 🩹 Sparse field patch of one `MasonryWall`.
    pub En1996WallPatch of crate::MasonryWall { set { label_en: String, label_de: String, wall_type: crate::WallType, thickness_m: f64, height_m: f64, length_m: f64, support_sides: u8, slab_bearing_depth_m: f64, eccentricity_top_m: f64, eccentricity_bottom_m: f64, unit_group: crate::UnitGroup, unit_material: crate::UnitMaterial, f_b_pa: f64, unit_length_m: f64, unit_width_m: f64, unit_height_m: f64, mortar_type: crate::MortarType, mortar_class: crate::MortarClass, mortar_strength_pa: f64, bed_joint_thickness_m: f64, reinforced: bool, as_vertical_m2: f64, as_horizontal_m2: f64, f_yd_pa: f64, fire_rei_min: u32, exposure: crate::ExposureClass, mu: f64, density_kg_m3: f64, phi_infinity: f64, is_basement: bool } nest { openings: En1996OpeningDelta, load_cases: En1996LoadCaseDelta } }
}

protocol::list_delta! {
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(rename_all = "camelCase"))]
    /// 📋️ Positional row delta of one `MasonryWall` list.
    pub En1996WallDelta { removal: En1996WallRemoval, insertion: En1996WallInsertion, relocation: En1996WallRelocation, modification: En1996WallModification, row: crate::MasonryWall, patch: En1996WallPatch, key: id }
}

//#endregion 🔖️Rows

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1996 artifact: the scalar fields a mutation sets and the keyed row deltas of its lists.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Diff {
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub masonry_class: Option<crate::MasonryClass>,
    #[state(artifact)]
    pub design_situation: Option<crate::document::DesignSituation>,
    #[state(artifact)]
    pub storeys: Option<u32>,
    #[state(artifact)]
    pub walls: En1996WallDelta,
}
//#endregion 🔖️Diff

impl MutationDiff<En1996Snapshot> for En1996Diff {
    fn apply(&self, base: &En1996Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<En1996Snapshot> {
        Ok(En1996Snapshot {
            annex: self.annex.unwrap_or(base.annex),
            masonry_class: self.masonry_class.unwrap_or(base.masonry_class),
            design_situation: self.design_situation.unwrap_or(base.design_situation),
            storeys: self.storeys.unwrap_or(base.storeys),
            walls: self.walls.commit_onto(&base.walls, capability).map_err(|error| error.under(["walls"]))?,
        })
    }

    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.masonry_class.is_some() {
            self.masonry_class = other.masonry_class;
        }
        if other.design_situation.is_some() {
            self.design_situation = other.design_situation;
        }
        if other.storeys.is_some() {
            self.storeys = other.storeys;
        }
        self.walls.absorb(other.walls);
    }
}

impl DiffAlgebra<En1996Snapshot> for En1996Diff {
    fn inverse(&self, base: &En1996Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex),
            masonry_class: self.masonry_class.as_ref().map(|_| base.masonry_class),
            design_situation: self.design_situation.as_ref().map(|_| base.design_situation),
            storeys: self.storeys.as_ref().map(|_| base.storeys),
            walls: self.walls.inverse(&base.walls),
        }
    }

    fn is_empty(&self) -> bool {
        self.annex.is_none()
            && self.masonry_class.is_none()
            && self.design_situation.is_none()
            && self.storeys.is_none()
            && self.walls.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
