//! 🧬️ Named example subjects for EN 1991 (design-load assumptions).

use semio_s_artifact_norm_en1991_model::{AccidentalCase, AccidentalExplosion, AccidentalImpact, ConstructionLoad, ConstructionStage, DerailmentCase, En1991Snapshot, ExecutionImperfection, FallingLoad, FireMode, FloorArea, HelicopterLanding, RoofArea, SelfWeightElement, StructureKind, TieRun, WindFace};

/// 🏢 Realistic DE office building — no bridge traffic, no fire scenario.
pub fn de_office_compliant() -> En1991Snapshot {
    let mut s = En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    s.structure_kind = StructureKind::Building;
    s.fire_mode = FireMode::None;
    s.crane_claimed = true;
    s.crane_class = "HC2".into();
    s.hoist_class = "HC2".into();
    s.hoisting_speed = 0.5;
    s.bridge_tandem_x = 10.0;
    s.rail_alpha = 1.0;
    s.assumed_rail_lm71 = 250_000.0;
    s.assumed_rail_lm71_udl = 80_000.0;
    s.assumed_rail_sw0 = 133_000.0;
    s.assumed_rail_sw2 = 150_000.0;
    s.crane_x = 15.0;
    s.crane_runway_length = 30.0;
    s.crane_wheel_base = 3.5;
    s.silo_diameter = 6.0;
    s.silo_eccentricity = 0.0;
    s.silo_hopper_angle_deg = 60.0;
    s.bridge_curve_radius = 0.0;
    s.assumed_crane_wheel = 150_000.0;
    s.assumed_crane_horizontal = 40_000.0;
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
    let mut s = En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
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
    s.assumed_bridge_tandem = 330_000.0;
    s.assumed_bridge_udl = 10_000.0;
    s.assumed_bridge_lm2 = 720_000.0;
    s.assumed_bridge_lm3 = 1_200_000.0;
    s.assumed_bridge_lm4 = 10_000.0;
    s.assumed_bridge_footway = 5_000.0;
    s.bridge_load_group = "gr1a".into();
    s.thermal_element_type = "bridge".into();
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

/// 🔥 Parametric fire compartment on the occupancy-table basis — assumptions cover Annex A / E.
pub fn de_fire_parametric_compliant() -> En1991Snapshot {
    fire_compartment(semio_s_artifact_norm_en1991_model::FireLoadBasis::OccupancyTable, 420.0e6)
}

/// 🧯 Parametric fire compartment on the user-specified basis: q_f,k = 600 MJ/m² is the entered value, the occupancy row is not consulted.
pub fn de_fire_user_specified_compliant() -> En1991Snapshot {
    fire_compartment(semio_s_artifact_norm_en1991_model::FireLoadBasis::UserSpecified, 600.0e6)
}

fn fire_compartment(basis: semio_s_artifact_norm_en1991_model::FireLoadBasis, qf_k: f64) -> En1991Snapshot {
    let mut s = En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    s.structure_kind = StructureKind::Building;
    s.fire_mode = FireMode::Parametric;
    s.fire_curve = semio_s_artifact_norm_en1991_model::part_1_2::FireCurve::Parametric;
    s.fire_duration = 3600.0;
    s.fire_compartment_area = 100.0;
    s.fire_compartment_height = 3.0;
    s.fire_opening_factor = 0.04;
    s.fire_thermal_inertia = 1160.0;
    s.fire_occupancy = semio_s_artifact_norm_en1991_model::FireOccupancy::Office;
    s.fire_load_density_qf = qf_k;
    s.fire_load_basis = basis;
    use semio_s_artifact_norm_en1991_checks::part_1_2 as fire;
    let o = fire::compartment_opening_factor(s.fire_compartment_area, s.fire_compartment_height);
    s.fire_opening_factor = o;
    let qfd = fire::design_fire_load_j_m2(s.fire_load_basis, s.fire_load_density_qf, s.annex, s.fire_occupancy, s.fire_compartment_area, 1.0).expect("the basis is satisfiable");
    let (theta, parametric) = fire::parametric_gas_temp_k(s.fire_duration, s.fire_compartment_area, s.fire_compartment_height, s.fire_thermal_inertia, qfd, Some(o), 20.0 / 60.0);
    let theta_required = theta.max(parametric.theta_max_c + 273.15);
    s.assumed_gas_temperature = theta_required + 50.0;
    let alpha_c = fire::alpha_c_for_curve(semio_s_artifact_norm_en1991_model::part_1_2::FireCurve::Parametric);
    s.assumed_h_net = fire::h_net_default_w_m2(s.assumed_gas_temperature, alpha_c) + 100.0;
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
    let mut s = En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
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
        z: 8.0,
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

/// 🚁 Execution and hazard collections: helicopter pad, ties, construction loads, falling load, execution imperfection and derailment case, each assumed above its hand-computed requirement (the derailment forces are entered design values, EN 1991-1-7 Tables 4.4-4.5 are not reproduced).
pub fn de_hazard_collections() -> En1991Snapshot {
    let mut s = En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    s.structure_kind = StructureKind::Building;
    s.fire_mode = FireMode::None;
    s.crane_claimed = false;
    s.silo_claimed = false;
    s.helicopter_landings = vec![HelicopterLanding { id: "heli-roof".into(), mass: 3_000.0, x: 5.0, y: 5.0, height: 20.0, assumed_force: 200_000.0 }];
    s.tie_runs = vec![
        TieRun { id: "tie-peripheral".into(), kind: 0, spacing: 5.0, span: 6.0, dead_load: 5_000.0, imposed_load: 3_000.0, psi: 0.3, x: 0.0, y: 0.0, provided_force: 80_000.0 },
        TieRun { id: "tie-internal".into(), kind: 1, spacing: 5.0, span: 6.0, dead_load: 5_000.0, imposed_load: 3_000.0, psi: 0.3, x: 0.0, y: 3.0, provided_force: 80_000.0 },
    ];
    s.construction_stages = vec![ConstructionStage { id: "short".into(), activity: "personnel".into(), duration_days: 30.0, return_years: 2.0, assumed_qk: 1_500.0 }];
    s.construction_loads = vec![
        ConstructionLoad { id: "q-ca".into(), stage: "short".into(), kind: 0, magnitude: 1_000.0 },
        ConstructionLoad { id: "q-cb".into(), stage: "short".into(), kind: 1, magnitude: 200.0 },
    ];
    s.falling_loads = vec![FallingLoad { id: "drop".into(), mass: 100.0, drop_height: 2.0, stopping_distance: 0.1, x: 2.0, y: 2.0, assumed_force: 25_000.0 }];
    s.execution_imperfections = vec![ExecutionImperfection { id: "column".into(), height: 4.0, tolerance: 0.02, vertical_load: 1.0e6, x: 1.0, y: 1.0, assumed_force: 6_000.0 }];
    s.derailment_cases = vec![DerailmentCase { id: "track".into(), distance: 3.0, design_force_x: 1_000_000.0, design_force_y: 300_000.0, x: 0.0, y: -6.0, assumed_force: 1_200_000.0 }];
    s
}
