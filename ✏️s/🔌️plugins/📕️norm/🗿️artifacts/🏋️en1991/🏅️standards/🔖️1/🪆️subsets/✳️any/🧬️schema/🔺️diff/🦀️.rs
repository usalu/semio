//! 🧬️ En1991 diff schema — sparse field delta over the artifact.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1991 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1991Artifact>>,

    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub snow_zone: Option<String>,
    #[state(artifact)]
    pub altitude: Option<f64>,
    #[state(artifact)]
    pub en_sk: Option<f64>,
    #[state(artifact)]
    pub exceptional_snow_north_german_lowlands: Option<bool>,
    #[state(artifact)]
    pub wind_zone: Option<u8>,
    #[state(artifact)]
    pub en_vb: Option<f64>,
    #[state(artifact)]
    pub terrain_category: Option<u8>,
    #[state(artifact)]
    pub mixed_terrain_upwind: Option<u8>,
    #[state(artifact)]
    pub mixed_terrain_distance: Option<f64>,
    #[state(artifact)]
    pub orography_factor: Option<f64>,
    #[state(artifact)]
    pub coast_or_island: Option<bool>,
    #[state(artifact)]
    pub air_density: Option<f64>,
    #[state(artifact)]
    pub height: Option<f64>,
    #[state(artifact)]
    pub width: Option<f64>,
    #[state(artifact)]
    pub depth: Option<f64>,
    #[state(artifact)]
    pub assumed_delta_t: Option<f64>,
    #[state(artifact)]
    pub t_max: Option<f64>,
    #[state(artifact)]
    pub t_min: Option<f64>,
    #[state(artifact)]
    pub t_0: Option<f64>,
    #[state(artifact)]
    pub thermal_element_type: Option<String>,
    #[state(artifact)]
    pub thermal_bridge_type: Option<u8>,
    #[state(artifact)]
    pub delta_t_m: Option<f64>,
    #[state(artifact)]
    pub storey_count: Option<u8>,
    #[state(artifact)]
    pub fire_mode: Option<crate::FireMode>,
    #[state(artifact)]
    pub fire_curve: Option<crate::part_1_2::FireCurve>,
    #[state(artifact)]
    pub fire_duration: Option<f64>,
    #[state(artifact)]
    pub assumed_gas_temperature: Option<f64>,
    #[state(artifact)]
    pub assumed_h_net: Option<f64>,
    #[state(artifact)]
    pub fire_compartment_area: Option<f64>,
    #[state(artifact)]
    pub fire_compartment_height: Option<f64>,
    #[state(artifact)]
    pub fire_opening_factor: Option<f64>,
    #[state(artifact)]
    pub fire_thermal_inertia: Option<f64>,
    #[state(artifact)]
    pub fire_occupancy: Option<String>,
    #[state(artifact)]
    pub fire_load_density_qf: Option<f64>,
    #[state(artifact)]
    pub assumed_qf_d: Option<f64>,
    #[state(artifact)]
    pub construction_activity: Option<String>,
    #[state(artifact)]
    pub assumed_construction_qk: Option<f64>,
    #[state(artifact)]
    pub structure_kind: Option<crate::StructureKind>,
    #[state(artifact)]
    pub bridge_lane: Option<u8>,
    #[state(artifact)]
    pub bridge_span: Option<f64>,
    #[state(artifact)]
    pub bridge_lane_width: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_tandem: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_udl: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm2: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_footway: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm3: Option<f64>,
    #[state(artifact)]
    pub assumed_bridge_lm4: Option<f64>,
    #[state(artifact)]
    pub bridge_load_group: Option<String>,
    #[state(artifact)]
    pub crane_claimed: Option<bool>,
    #[state(artifact)]
    pub crane_class: Option<String>,
    #[state(artifact)]
    pub hoist_class: Option<String>,
    #[state(artifact)]
    pub hoisting_speed: Option<f64>,
    #[state(artifact)]
    pub assumed_crane_wheel: Option<f64>,
    #[state(artifact)]
    pub assumed_crane_horizontal: Option<f64>,
    #[state(artifact)]
    pub silo_claimed: Option<bool>,
    #[state(artifact)]
    pub silo_kind: Option<String>,
    #[state(artifact)]
    pub silo_bulk_density: Option<f64>,
    #[state(artifact)]
    pub silo_height: Option<f64>,
    #[state(artifact)]
    pub silo_hydraulic_radius: Option<f64>,
    #[state(artifact)]
    pub silo_mu: Option<f64>,
    #[state(artifact)]
    pub silo_k: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_pressure: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_patch: Option<f64>,
    #[state(artifact)]
    pub assumed_silo_wall_friction: Option<f64>,
    #[state(artifact)]
    pub floors: Option<En1991FloorsList>,
    #[state(artifact)]
    pub self_weight_elements: Option<En1991SelfWeightElementsList>,
    #[state(artifact)]
    pub roofs: Option<En1991RoofsList>,
    #[state(artifact)]
    pub wind_faces: Option<En1991WindFacesList>,
    #[state(artifact)]
    pub accidental_cases: Option<En1991AccidentalCasesList>,
}
//#endregion 🔖️Diff


//#region 🔖️DeltaHelpers

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1991FloorsList {
    pub values: Vec<crate::FloorArea>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1991SelfWeightElementsList {
    pub values: Vec<crate::SelfWeightElement>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1991RoofsList {
    pub values: Vec<crate::RoofArea>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1991WindFacesList {
    pub values: Vec<crate::WindFace>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1991AccidentalCasesList {
    pub values: Vec<crate::AccidentalCase>,
}

//#endregion 🔖️DeltaHelpers
