//! 🧬️ Din4108 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Din4108 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::din4108::schema::Din4108Artifact>>,
    #[state(artifact)]
    pub category: Option<String>,
    #[state(artifact)]
    pub layers: Option<Din4108LayerList>,
    #[state(artifact)]
    pub climate: Option<crate::document::ClimateZoneDe>,
    #[state(artifact)]
    pub airtightness_n50: Option<f64>,
    #[state(artifact)]
    pub psi_times_l_sum: Option<f64>,
    #[state(artifact)]
    pub rh_int: Option<f64>,
    #[state(artifact)]
    pub catalog_id: Option<String>,
    #[state(artifact)]
    pub material_id: Option<String>,
    #[state(artifact)]
    pub airtightness_class: Option<String>,
    #[state(artifact)]
    pub t_int_c: Option<f64>,
    #[state(artifact)]
    pub solar_absorptance: Option<f64>,
    #[state(artifact)]
    pub irradiance_w_m2: Option<f64>,
    #[state(artifact)]
    pub moisture_mu_exterior: Option<f64>,
    #[state(artifact)]
    pub moisture_mu_interior: Option<f64>,
    #[state(artifact)]
    pub envelope_area_m2: Option<f64>,
    #[state(artifact)]
    pub bb2_details_conform: Option<bool>,
    #[state(artifact)]
    pub application_type: Option<String>,
    #[state(artifact)]
    pub declared_application_class: Option<String>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108StringList {
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Din4108LayerList {
    pub values: Vec<crate::artifacts::din4108::LayerDocument>,
}
//#endregion 🔖️DeltaHelpers
