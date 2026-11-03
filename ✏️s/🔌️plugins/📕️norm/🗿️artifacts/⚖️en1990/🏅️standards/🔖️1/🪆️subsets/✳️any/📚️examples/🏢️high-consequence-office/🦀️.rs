//! 📚️ Example `high-consequence-office` — non-compliant multi-failure CC3 subject.

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;
use semio_s_artifact_norm_en1990_model::{ReliabilityClass, Member, MemberEffect, PermanentAction, VariableAction};

pub const ID: &str = "high-consequence-office";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("High Consequence Office", "Büro mit hoher Schadensfolge")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️high-consequence-office/🏢️high-consequence-office/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🏗️ Non-compliant CC3 office: undersized R_d, β below RC3 target, deflection over limit.
pub fn reference_snapshot() -> semio_s_artifact_norm_en1990_model::En1990Snapshot {
    semio_s_artifact_norm_en1990_model::En1990Snapshot {
        annex: semio_s_plugin_norm_compliance::AnnexChoice::En,
        project_id: "high-consequence-office".into(),
        structure_kind: semio_s_artifact_norm_en1990_model::StructureKind::Building,
        altitude_m: 0.0,
        consequence_class: 3,
        reliability_class: ReliabilityClass::RC3,
        design_working_life_category: 4,
        design_working_life_years: 50.0,
        reference_period_years: 50.0,
        supervision_level: semio_s_artifact_norm_en1990_model::SupervisionLevel::Dsl1,
        inspection_level: semio_s_artifact_norm_en1990_model::InspectionLevel::Il1,
        beta_computed: 3.9,
        permanents: vec![
            PermanentAction { id: "G-sup".into(), kind: semio_s_artifact_norm_en1990_model::PermanentKind::GSup, gk: 250_000.0 },
            PermanentAction { id: "G-inf".into(), kind: semio_s_artifact_norm_en1990_model::PermanentKind::GInf, gk: 50_000.0 },
        ],
        variables: vec![
            VariableAction { id: "Q-office".into(), category: "office".into(), qk: 60_000.0, group: String::new() },
            VariableAction { id: "Q-partition".into(), category: "retail".into(), qk: 12_000.0, group: String::new() },
            VariableAction { id: "Q-snow".into(), category: "snow".into(), qk: 18_000.0, group: String::new() },
        ],
        accidentals: vec![],
        seismics: vec![],
        members: vec![Member {
            id: "beam-B1".into(),
            label_en: "Office beam B1".into(),
            label_de: "Büroträger B1".into(),
            rd_str: 200_000.0,
            rd_geo: 200_000.0,
            rd_equ_stab: 180_000.0,
            rd_equ_destab: 150_000.0,
            rd_fat: 0.0,
            span: 8.0,
            precamber: 0.0,
            deflection_w1: 0.013333,
            deflection_w2: 0.006667,
            deflection_w3: 0.02,
            deflection_limit_ratio: 250.0,
            deflection_limit_ratio_w3: 300.0,
            sls_limit_char: 200_000.0,
            sls_limit_freq: 200_000.0,
            sls_limit_qp: 200_000.0,
            vibration_frequency: 2.5,
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
        bridge_sls: vec![],
        effects: vec![
            MemberEffect { member_id: "beam-B1".into(), action_id: "G-sup".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "G-inf".into(), influence: -1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-office".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-partition".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-snow".into(), influence: 1.0 },
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

