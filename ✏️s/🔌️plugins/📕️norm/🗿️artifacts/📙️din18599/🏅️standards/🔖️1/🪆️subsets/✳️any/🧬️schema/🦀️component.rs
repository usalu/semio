//! 🧬️ Din18599 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use crate::artifacts::din18599::{MonthlyClimate, UseClass};
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Din18599 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Artifact {
    #[state(persistent)] pub use_class: crate::artifacts::din18599::UseClass,
    #[state(persistent)] pub heated_area_m2: f64,
    #[state(persistent)] pub occupants: u32,
    #[state(persistent)] pub h_t: f64,
    #[state(persistent)] pub h_v: f64,
    #[state(persistent)] pub climate: crate::artifacts::din18599::MonthlyClimate,
    #[state(persistent)] pub internal_gains_w_m2: f64,
    #[state(persistent)] pub solar_gains_kwh: f64,
    #[state(persistent)] pub system_losses_kwh: f64,
    #[state(persistent)] pub renewable_kwh: f64,
    #[state(persistent)] pub annual_limit_kwh: f64,
    #[state(persistent)] pub energy_carrier: String,
    #[state(persistent)] pub reference_q_p_kwh: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Din18599Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::din18599::Din18599Snapshot {
        crate::artifacts::din18599::Din18599Snapshot {
            use_class: self.use_class,
            heated_area_m2: self.heated_area_m2,
            occupants: self.occupants,
            h_t: self.h_t,
            h_v: self.h_v,
            climate: self.climate.clone(),
            internal_gains_w_m2: self.internal_gains_w_m2,
            solar_gains_kwh: self.solar_gains_kwh,
            system_losses_kwh: self.system_losses_kwh,
            renewable_kwh: self.renewable_kwh,
            annual_limit_kwh: self.annual_limit_kwh,
            energy_carrier: self.energy_carrier.clone(),
            reference_q_p_kwh: self.reference_q_p_kwh,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::din18599::Din18599Snapshot) -> Self {
        Self {
            use_class: snapshot.use_class,
            heated_area_m2: snapshot.heated_area_m2,
            occupants: snapshot.occupants,
            h_t: snapshot.h_t,
            h_v: snapshot.h_v,
            climate: snapshot.climate,
            internal_gains_w_m2: snapshot.internal_gains_w_m2,
            solar_gains_kwh: snapshot.solar_gains_kwh,
            system_losses_kwh: snapshot.system_losses_kwh,
            renewable_kwh: snapshot.renewable_kwh,
            annual_limit_kwh: snapshot.annual_limit_kwh,
            energy_carrier: snapshot.energy_carrier.clone(),
            reference_q_p_kwh: snapshot.reference_q_p_kwh,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::din18599::Din18599Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.din18599` — twenty handcrafted schema leaves.
pub fn din18599_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.din18599",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor