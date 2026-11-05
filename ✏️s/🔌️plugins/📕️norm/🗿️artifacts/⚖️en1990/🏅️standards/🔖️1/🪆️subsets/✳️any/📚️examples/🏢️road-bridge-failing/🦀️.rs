//! 📚️ Example `road-bridge-failing` — CC2 road bridge failing Annex A2.4 SLS and ULS.

use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;
use semio_s_artifact_norm_en1990_model::{ReliabilityClass, BridgeSls, Member, MemberEffect, PermanentAction, VariableAction};

pub const ID: &str = "road-bridge-failing";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Rail Bridge Failing", "Eisenbahnbrücke — nicht konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️road-bridge-failing/🏢️road-bridge-failing/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🌉 Non-compliant road bridge: A2.4 deflection over limit and undersized R_d.
pub fn reference_snapshot() -> semio_s_artifact_norm_en1990_model::En1990Snapshot {
    semio_s_artifact_norm_en1990_model::En1990Snapshot {
        annex: semio_s_plugin_norm_compliance::AnnexChoice::De,
        project_id: "road-bridge-failing".into(),
        structure_kind: semio_s_artifact_norm_en1990_model::StructureKind::RailBridge,
        altitude_m: 200.0,
        consequence_class: 2,
        reliability_class: ReliabilityClass::RC2,
        design_working_life_category: 5,
        design_working_life_years: 100.0,
        reference_period_years: 100.0,
        supervision_level: semio_s_artifact_norm_en1990_model::SupervisionLevel::Dsl2,
        inspection_level: semio_s_artifact_norm_en1990_model::InspectionLevel::Il2,
        beta_computed: 3.8,
        permanents: vec![
            PermanentAction { id: "G-sup".into(), kind: semio_s_artifact_norm_en1990_model::PermanentKind::GSup, gk: 200_000.0 },
            PermanentAction { id: "G-inf".into(), kind: semio_s_artifact_norm_en1990_model::PermanentKind::GInf, gk: 40_000.0 },
        ],
        variables: vec![
            VariableAction { id: "Q-traffic".into(), category: "rail_traffic".into(), qk: 150_000.0, group: String::new() },
            VariableAction { id: "Q-wind".into(), category: "wind".into(), qk: 30_000.0, group: String::new() },
        ],
        accidentals: vec![],
        seismics: vec![],
        members: vec![Member {
            id: "deck-D1".into(),
            label_en: "Main deck D1".into(),
            label_de: "Hauptfahrbahn D1".into(),
            rd_str: 200_000.0,
            rd_geo: 200_000.0,
            rd_equ_stab: 180_000.0,
            rd_equ_destab: 150_000.0,
            rd_fat: 0.0,
            span: 40.0,
            precamber: 0.0,
            deflection_w1: 0.04,
            deflection_w2: 0.02,
            deflection_w3: 0.06,
            deflection_limit_ratio: 500.0,
            deflection_limit_ratio_w3: 300.0,
            sls_limit_char: 200_000.0,
            sls_limit_freq: 200_000.0,
            sls_limit_qp: 200_000.0,
            vibration_frequency: 2.0,
            vibration_frequency_min: 3.0,
            frame_height: 0.0,
            frame_drift: 0.0,
            storey_height: 0.0,
            storey_drift: 0.0,
            material_strength_k: 0.0,
            material_conversion_factor: 0.0,
            material_partial_factor: 0.0,
            material_design_strength: 0.0,
            nominal_dimension: 0.0,
            dimension_tolerance: 0.0,
            design_dimension: 0.0,
            member_alpha_cr: 0.0,
            second_order_analysis: 0.0,
        }],
        bridge_sls: vec![BridgeSls {
            id: "sls-D1".into(),
            member_id: "deck-D1".into(),
            deck_acceleration: 0.8,
            deck_acceleration_limit: 0.5,
            deck_twist: 0.002,
            deck_twist_limit: 0.001,
            bridge_deflection: 0.12,
            bridge_deflection_limit: 0.05,
            track_gauge: 0.0,
            line_speed: 0.0,
            ballast_depth: 0.0,
            deck_mass: 0.0,
            natural_frequency: 0.0,
            end_rotation: 0.0,
            basic_wind_speed: 0.0,
            damping_ratio: 0.0,
        }],
        effects: vec![
            MemberEffect { member_id: "deck-D1".into(), action_id: "G-sup".into(), influence: 1.0 },
            MemberEffect { member_id: "deck-D1".into(), action_id: "G-inf".into(), influence: -1.0 },
            MemberEffect { member_id: "deck-D1".into(), action_id: "Q-traffic".into(), influence: 1.0 },
            MemberEffect { member_id: "deck-D1".into(), action_id: "Q-wind".into(), influence: 1.0 },
        ],
        fatigue_spectra: vec![],
        reliability_analyses: vec![],
        test_series: vec![],
        placements: vec![],
        cases: vec![],
        factor_overrides: vec![],
        traffic_groups: vec![],
    }
}

