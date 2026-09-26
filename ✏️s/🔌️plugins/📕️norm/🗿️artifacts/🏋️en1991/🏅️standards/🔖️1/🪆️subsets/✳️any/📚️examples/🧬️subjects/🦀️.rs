//! 🧬️ Named example subjects for EN 1991 (design-load assumptions).

use crate::{AccidentalCase, AccidentalExplosion, AccidentalImpact, En1991Snapshot, FireMode, FloorArea, RoofArea, SelfWeightElement, StructureKind, WindFace};

/// 🏢 Realistic DE office building — no bridge traffic, no fire scenario.
pub fn de_office_compliant() -> En1991Snapshot {
    let mut s = En1991Snapshot::default();
    s.structure_kind = StructureKind::Building;
    s.fire_mode = FireMode::None;
    s.crane_claimed = true;
    s.crane_class = "HC2".into();
    s.hoist_class = "HC2".into();
    s.hoisting_speed = 0.5;
    s.assumed_crane_wheel = 150_000.0;
    s.assumed_crane_horizontal = 25_000.0;
    s.silo_claimed = true;
    s.silo_kind = "silo".into();
    s.silo_bulk_density = 8_000.0;
    s.silo_height = 12.0;
    s.silo_hydraulic_radius = 1.5;
    s.silo_mu = 0.4;
    s.silo_k = 0.4;
    s.assumed_silo_pressure = 60_000.0;
    s.assumed_silo_patch = 25_000.0;
    s.assumed_silo_wall_friction = 25_000.0;
    s
}

/// 🌉 DE road bridge — LM1 gr1a compliant assumptions.
pub fn de_bridge_compliant() -> En1991Snapshot {
    let mut s = En1991Snapshot::default();
    s.structure_kind = StructureKind::Bridge;
    s.fire_mode = FireMode::None;
    s.floors.clear();
    s.self_weight_elements.clear();
    s.roofs.clear();
    s.wind_faces.clear();
    s.crane_claimed = false;
    s.silo_claimed = false;
    s.bridge_lane = 1;
    s.bridge_span = 20.0;
    s.bridge_lane_width = 9.0;
    s.assumed_bridge_tandem = 270_000.0;
    s.assumed_bridge_udl = 3_600.0;
    s.assumed_bridge_lm2 = 720_000.0;
    s.assumed_bridge_lm3 = 1_200_000.0;
    s.assumed_bridge_lm4 = 10_000.0;
    s.assumed_bridge_footway = 5_000.0;
    s.bridge_load_group = "gr1a".into();
    s.thermal_element_type = "bridge1".into();
    s.thermal_bridge_type = 1;
    s.assumed_delta_t = 50.0;
    s
}

/// ⚠️ Bridge with under-assumed LM1 / LM3 / LM4.
pub fn de_bridge_noncompliant() -> En1991Snapshot {
    let mut s = de_bridge_compliant();
    s.assumed_bridge_tandem = 100_000.0;
    s.assumed_bridge_udl = 1_000.0;
    s.assumed_bridge_lm2 = 100_000.0;
    s.assumed_bridge_lm3 = 100_000.0;
    s.assumed_bridge_lm4 = 500.0;
    s.assumed_bridge_footway = 1_000.0;
    s.bridge_load_group = "gr5".into();
    s
}

/// 🔥 Parametric fire compartment — assumptions cover Annex A / E.
pub fn de_fire_parametric_compliant() -> En1991Snapshot {
    let mut s = En1991Snapshot::default();
    s.structure_kind = StructureKind::Building;
    s.fire_mode = FireMode::Parametric;
    s.fire_curve = crate::part_1_2::FireCurve::Parametric;
    s.fire_duration = 3600.0;
    s.fire_compartment_area = 100.0;
    s.fire_compartment_height = 3.0;
    s.fire_opening_factor = 0.04;
    s.fire_thermal_inertia = 1160.0;
    s.fire_occupancy = "office".into();
    s.fire_load_density_qf = 420.0e6;
    use crate::standards::v1::subsets::any::schema::part_1_2 as fire;
    let o = fire::compartment_opening_factor(s.fire_compartment_area, s.fire_compartment_height);
    s.fire_opening_factor = o;
    let theta = fire::parametric_gas_temp_k(s.fire_duration, o, s.fire_thermal_inertia);
    s.assumed_gas_temperature = theta + 50.0;
    let alpha_c = fire::alpha_c_for_curve(crate::part_1_2::FireCurve::Parametric);
    s.assumed_h_net = fire::h_net_w_m2(theta, 293.15, alpha_c, 1.0, 0.8, 1.0) + 100.0;
    let qfd = fire::design_fire_load_j_m2(s.fire_load_density_qf, s.annex, &s.fire_occupancy, 0.8);
    s.assumed_qf_d = qfd + 1.0e6;
    s.crane_claimed = false;
    s.silo_claimed = false;
    s
}

/// ⚠️ Parametric fire with under-assumed θ_g / q_f,d.
pub fn de_fire_parametric_noncompliant() -> En1991Snapshot {
    let mut s = de_fire_parametric_compliant();
    s.assumed_gas_temperature = 400.0;
    s.assumed_h_net = 1_000.0;
    s.assumed_qf_d = 50.0e6;
    s
}

/// 💥 Accidental impact + explosion rows (discriminated variants).
pub fn de_accidental_variants() -> En1991Snapshot {
    let mut s = En1991Snapshot::default();
    s.structure_kind = StructureKind::Building;
    s.fire_mode = FireMode::None;
    s.crane_claimed = false;
    s.silo_claimed = false;
    s.accidental_cases = vec![
        AccidentalCase {
            id: "impact-1".into(),
            impact: vec![AccidentalImpact {
                vehicle_mass: 15_000.0,
                vehicle_speed: 13.9,
                assumed_force: 500_000.0,
            }],
            explosion: vec![],
        },
        AccidentalCase {
            id: "explosion-1".into(),
            impact: vec![],
            explosion: vec![AccidentalExplosion {
                explosion_mass: 25.0,
                standoff: 5.0,
                assumed_pressure: 50_000.0,
            }],
        },
    ];
    s
}

/// ⚠️ Same geometry with under-assumed loads across parts 1-1 / 1-3 / 1-4 / 1-5 / 3 / 4.
pub fn multi_fail_noncompliant() -> En1991Snapshot {
    let mut s = de_office_compliant();
    s.assumed_delta_t = 10.0;
    s.floors = vec![FloorArea {
        id: "office-l1".into(),
        category: "B1".into(),
        area: 240.0,
        assumed_qk: 1_000.0,
        assumed_qk_concentrated: 5_000.0,
        assumed_partitions: 200.0,
    }];
    s.self_weight_elements = vec![SelfWeightElement {
        id: "slab-rc".into(),
        material: "reinforced_concrete".into(),
        thickness: 0.2,
        assumed_gk: 2_000.0,
    }];
    s.roofs = vec![RoofArea {
        id: "roof-main".into(),
        roof_type: "duopitch".into(),
        pitch_deg: 15.0,
        c_e: 1.0,
        c_t: 1.0,
        has_parapet: false,
        parapet_height: 0.0,
        drift_obstruction_height: 0.0,
        multi_span: false,
        assumed_sk: 200.0,
    }];
    s.wind_faces = vec![WindFace {
        id: "facade-d".into(),
        zone: "D".into(),
        z: 12.0,
        c_pe10: 0.8,
        c_pe1: 1.0,
        c_pi: 0.2,
        c_s: 1.0,
        c_d: 1.0,
        loaded_area: 10.0,
        assumed_wp: 100.0,
    }];
    s.assumed_crane_wheel = 50_000.0;
    s.assumed_crane_horizontal = 5_000.0;
    s.assumed_silo_pressure = 5_000.0;
    s.assumed_silo_patch = 1_000.0;
    s.assumed_silo_wall_friction = 1_000.0;
    s.accidental_cases = vec![AccidentalCase {
        id: "impact-1".into(),
        impact: vec![AccidentalImpact {
            vehicle_mass: 15_000.0,
            vehicle_speed: 13.9,
            assumed_force: 50_000.0,
        }],
        explosion: vec![],
    }];
    s
}
