//! 🧬️ En1991 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use crate::artifacts::en1991::part_1_2::FireCurve;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1991 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Artifact {
    #[state(persistent)] pub area_m2: f64,
    #[state(persistent)] pub category: crate::document::ImposedCategory,
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub self_weight_material: String,
    #[state(persistent)] pub self_weight_thickness_m: f64,
    #[state(persistent)] pub assumed_g_k_kn_m2: f64,
    #[state(persistent)] pub fire_curve: crate::artifacts::en1991::part_1_2::FireCurve,
    #[state(persistent)] pub fire_resistance_min: f64,
    #[state(persistent)] pub fire_member_capacity_c: f64,
    #[state(persistent)] pub snow_zone: u8,
    #[state(persistent)] pub snow_altitude_m: f64,
    #[state(persistent)] pub en_s_k_kn_m2: f64,
    #[state(persistent)] pub wind_zone: u8,
    #[state(persistent)] pub en_v_b_m_s: f64,
    #[state(persistent)] pub delta_t_k: f64,
    #[state(persistent)] pub construction_activity: String,
    #[state(persistent)] pub accidental_mass_t: f64,
    #[state(persistent)] pub accidental_speed_km_h: f64,
    #[state(persistent)] pub bridge_lane: u8,
    #[state(persistent)] pub bridge_span_m: f64,
    #[state(persistent)] pub bridge_lane_width_m: f64,
    #[state(persistent)] pub bridge_moment_resistance_knm: f64,
    #[state(persistent)] pub crane_class: String,
    #[state(persistent)] pub hoist_class: String,
    #[state(persistent)] pub hoisting_speed_m_s: f64,
    #[state(persistent)] pub silo_bulk_density_kn_m3: f64,
    #[state(persistent)] pub silo_height_m: f64,
    #[state(persistent)] pub silo_hydraulic_radius_m: f64,
    #[state(persistent)] pub silo_mu: f64,
    #[state(persistent)] pub silo_k: f64,
    #[state(persistent)] pub c_s: f64,
    #[state(persistent)] pub c_d: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1991Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1991::En1991Snapshot {
        crate::artifacts::en1991::En1991Snapshot {
            area_m2: self.area_m2,
            category: self.category,
            annex: self.annex,
            self_weight_material: self.self_weight_material.clone(),
            self_weight_thickness_m: self.self_weight_thickness_m,
            assumed_g_k_kn_m2: self.assumed_g_k_kn_m2,
            fire_curve: self.fire_curve,
            fire_resistance_min: self.fire_resistance_min,
            fire_member_capacity_c: self.fire_member_capacity_c,
            snow_zone: self.snow_zone,
            snow_altitude_m: self.snow_altitude_m,
            en_s_k_kn_m2: self.en_s_k_kn_m2,
            wind_zone: self.wind_zone,
            en_v_b_m_s: self.en_v_b_m_s,
            delta_t_k: self.delta_t_k,
            construction_activity: self.construction_activity.clone(),
            accidental_mass_t: self.accidental_mass_t,
            accidental_speed_km_h: self.accidental_speed_km_h,
            bridge_lane: self.bridge_lane,
            bridge_span_m: self.bridge_span_m,
            bridge_lane_width_m: self.bridge_lane_width_m,
            bridge_moment_resistance_knm: self.bridge_moment_resistance_knm,
            crane_class: self.crane_class.clone(),
            hoist_class: self.hoist_class.clone(),
            hoisting_speed_m_s: self.hoisting_speed_m_s,
            silo_bulk_density_kn_m3: self.silo_bulk_density_kn_m3,
            silo_height_m: self.silo_height_m,
            silo_hydraulic_radius_m: self.silo_hydraulic_radius_m,
            silo_mu: self.silo_mu,
            silo_k: self.silo_k,
            c_s: self.c_s,
            c_d: self.c_d,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1991::En1991Snapshot) -> Self {
        Self {
            area_m2: snapshot.area_m2,
            category: snapshot.category,
            annex: snapshot.annex,
            self_weight_material: snapshot.self_weight_material.clone(),
            self_weight_thickness_m: snapshot.self_weight_thickness_m,
            assumed_g_k_kn_m2: snapshot.assumed_g_k_kn_m2,
            fire_curve: snapshot.fire_curve,
            fire_resistance_min: snapshot.fire_resistance_min,
            fire_member_capacity_c: snapshot.fire_member_capacity_c,
            snow_zone: snapshot.snow_zone,
            snow_altitude_m: snapshot.snow_altitude_m,
            en_s_k_kn_m2: snapshot.en_s_k_kn_m2,
            wind_zone: snapshot.wind_zone,
            en_v_b_m_s: snapshot.en_v_b_m_s,
            delta_t_k: snapshot.delta_t_k,
            construction_activity: snapshot.construction_activity.clone(),
            accidental_mass_t: snapshot.accidental_mass_t,
            accidental_speed_km_h: snapshot.accidental_speed_km_h,
            bridge_lane: snapshot.bridge_lane,
            bridge_span_m: snapshot.bridge_span_m,
            bridge_lane_width_m: snapshot.bridge_lane_width_m,
            bridge_moment_resistance_knm: snapshot.bridge_moment_resistance_knm,
            crane_class: snapshot.crane_class.clone(),
            hoist_class: snapshot.hoist_class.clone(),
            hoisting_speed_m_s: snapshot.hoisting_speed_m_s,
            silo_bulk_density_kn_m3: snapshot.silo_bulk_density_kn_m3,
            silo_height_m: snapshot.silo_height_m,
            silo_hydraulic_radius_m: snapshot.silo_hydraulic_radius_m,
            silo_mu: snapshot.silo_mu,
            silo_k: snapshot.silo_k,
            c_s: snapshot.c_s,
            c_d: snapshot.c_d,
            selected_check_index: None,
        }
    }
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1991::En1991Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1991` — fifteen handcrafted schema leaves.
pub fn en1991_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1991",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor