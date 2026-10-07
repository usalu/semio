//! 🧬️ En1994 snapshot schema — complete composite structure subject (SI).

use crate::document::AnnexChoice;
use crate::{CompositeBeam, CompositeColumn, CompositeSlab};
use framework_schema::ArtifactSchema;


//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1994", layout = "lines")]
#[artifact_schema(id = "s.norm.en1994")]
pub struct En1994Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub structure_kind: String,
    #[dsl(unit = "Pa")]
    #[state(artifact)]
    pub steel_f_y_pa: f64,
    #[dsl(table)]
    #[state(artifact)]
    pub beams: Vec<CompositeBeam>,
    #[dsl(table)]
    #[state(artifact)]
    pub columns: Vec<CompositeColumn>,
    #[dsl(table)]
    #[state(artifact)]
    pub slabs: Vec<CompositeSlab>,
    #[state(artifact)]
    pub fire_rating: String,
    #[dsl(unit = "m")]
    #[state(artifact)]
    pub insulation_thickness_m: f64,
    #[state(artifact)]
    pub fatigue_detail: String,
}
//#region 🔖️HandcraftedArtifactCodecs

//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for En1994Snapshot {
    fn default() -> Self {
        let beam = CompositeBeam::default_placeholder();
        let mut beam = beam;
        beam.id = "beam-B1".into();
        let mut col = CompositeColumn::default_placeholder();
        col.id = "col-C1".into();
        let mut slab = CompositeSlab::default_placeholder();
        slab.id = "slab-S1".into();
        Self {
            annex: AnnexChoice::De,
            structure_kind: "building".into(),
            steel_f_y_pa: 355e6,
            beams: vec![beam],
            columns: vec![col],
            slabs: vec![slab],
            fire_rating: "r60".into(),
            insulation_thickness_m: 0.025,
            fatigue_detail: "stud_welded".into(),
        }
    }
}

impl En1994Snapshot {
    /// 🏗️ Unpropped construction-stage beam (LTB length governs).
    pub fn unpropped_building() -> Self {
        let mut s = Self::default();
        s.beams[0].construction = "unpropped".into();
        s.beams[0].ltb_length_m = 5.0;
        s.beams[0].support = "simply_supported".into();
        s
    }

    /// 🪛 Custom non-catalogue steel plate girder (tw/tf/geometry leaves govern).
    pub fn custom_plate_building() -> Self {
        let mut s = Self::default();
        s.beams[0].steel = crate::SteelSection::custom_plate();
        s
    }

    /// 🌉️ Bridge girder with FLM3 fatigue actions (EN 1994-2).
    pub fn bridge_girder() -> Self {
        let mut s = Self::default();
        s.structure_kind = "bridge".into();
        s.annex = AnnexChoice::De;
        s.fatigue_detail = "stud_welded".into();
        s.beams[0].n_cycles = 2.0e6;
        s.beams[0].span_m = 25.0;
        let mut fat = crate::CharacteristicAction::fatigue_flm3("FLM3");
        fat.delta_sigma_k_pa = 65e6;
        fat.delta_tau_k_pa = 40e6;
        s.beams[0].actions.push(fat);
        s
    }

    /// 🔥️ Elevated fire rating demanding thicker insulation.
    pub fn fire_demanding() -> Self {
        let mut s = Self::default();
        s.fire_rating = "r120".into();
        s.insulation_thickness_m = 0.010;
        s
    }
}

//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge











//#endregion 🌉️ExternalCodecBridge
