//! 🧬️ Din4108 artifact schema — every field of the artifact with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full Din4108 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Artifact {
    #[state(artifact)]
    pub climate_zone: ClimateZoneDe,
    #[state(artifact)]
    pub usage: String,
    #[state(artifact)]
    pub t_int_c: f64,
    #[state(artifact)]
    pub rh_int: f64,
    #[state(artifact)]
    pub has_mechanical_ventilation: bool,
    #[state(artifact)]
    pub airtightness_n50: f64,
    #[state(artifact)]
    pub bb2_details_conform: bool,
    #[state(artifact)]
    pub zones: Vec<ThermalZone>,
    #[state(artifact)]
    pub elements: Vec<EnvelopeElement>,
    #[state(artifact)]
    pub thermal_bridges: Vec<ThermalBridge>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din4108Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> semio_s_artifact_norm_din4108_model::Din4108Snapshot {
        semio_s_artifact_norm_din4108_model::Din4108Snapshot {
            climate_zone: self.climate_zone,
            usage: self.usage.clone(),
            t_int_c: self.t_int_c,
            rh_int: self.rh_int,
            has_mechanical_ventilation: self.has_mechanical_ventilation,
            airtightness_n50: self.airtightness_n50,
            bb2_details_conform: self.bb2_details_conform,
            zones: self.zones.clone(),
            elements: self.elements.clone(),
            thermal_bridges: self.thermal_bridges.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: semio_s_artifact_norm_din4108_model::Din4108Snapshot) -> Self {
        Self {
            climate_zone: snapshot.climate_zone,
            usage: snapshot.usage,
            t_int_c: snapshot.t_int_c,
            rh_int: snapshot.rh_int,
            has_mechanical_ventilation: snapshot.has_mechanical_ventilation,
            airtightness_n50: snapshot.airtightness_n50,
            bb2_details_conform: snapshot.bb2_details_conform,
            zones: snapshot.zones,
            elements: snapshot.elements,
            thermal_bridges: snapshot.thermal_bridges,
        }
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din4108` — twenty handcrafted schema leaves.
pub fn din4108_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.din4108",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_din4108_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_din4108_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_din4108_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

