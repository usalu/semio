//! 📚️ Example `fatigue-failing` — fatigue (FAT) verification active (R_d,FAT > 0).

use semio_framework_plugin::ExampleSource;
use semio_framework_ui_locale::LocalizedLabel;
use semio_s_artifact_norm_en1990_model::{ReliabilityClass, Member, MemberEffect, PermanentAction, VariableAction, FatigueSpectrum};

pub const ID: &str = "fatigue-failing";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Fatigue Failing", "Ermüdung — nicht konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️fatigue-failing/🏢️fatigue-failing/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// ♻️ Fatigue-active subject with R_d,FAT = 50000.0.
pub fn reference_snapshot() -> semio_s_artifact_norm_en1990_model::En1990Snapshot {
    semio_s_artifact_norm_en1990_model::En1990Snapshot {
        annex: semio_s_plugin_norm_compliance::AnnexChoice::De,
        project_id: "fatigue-failing".into(),
        structure_kind: semio_s_artifact_norm_en1990_model::StructureKind::Building,
        altitude_m: 0.0,
        consequence_class: 2,
        reliability_class: ReliabilityClass::RC2,
        design_working_life_category: 4,
        design_working_life_years: 50.0,
        reference_period_years: 50.0,
        supervision_level: semio_s_artifact_norm_en1990_model::SupervisionLevel::Dsl2,
        inspection_level: semio_s_artifact_norm_en1990_model::InspectionLevel::Il2,
        beta_computed: 3.8,
        permanents: vec![
            PermanentAction { id: "G-sup".into(), kind: semio_s_artifact_norm_en1990_model::PermanentKind::GSup, gk: 80_000.0 },
            PermanentAction { id: "G-inf".into(), kind: semio_s_artifact_norm_en1990_model::PermanentKind::GInf, gk: 20_000.0 },
        ],
        variables: vec![
            VariableAction { id: "Q-office".into(), category: "office".into(), qk: 80000.0, group: String::new() },
            VariableAction { id: "Q-wind".into(), category: "wind".into(), qk: 20_000.0, group: String::new() },
        ],
        accidentals: vec![],
        seismics: vec![],
        members: vec![Member {
            id: "beam-B1".into(),
            label_en: "Fatigue beam B1".into(),
            label_de: "Ermüdungsträger B1".into(),
            rd_str: 200000.0,
            rd_geo: 200000.0,
            rd_equ_stab: 200_000.0,
            rd_equ_destab: 180_000.0,
            rd_fat: 50000.0,
            span: 6.0,
            precamber: 0.0,
            deflection_w1: 0.006,
            deflection_w2: 0.003,
            deflection_w3: 0.009,
            deflection_limit_ratio: 250.0,
            deflection_limit_ratio_w3: 300.0,
            sls_limit_char: 200000.0,
            sls_limit_freq: 200000.0,
            sls_limit_qp: 200000.0,
            vibration_frequency: 5.5,
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
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-wind".into(), influence: 1.0 },
        ],
        fatigue_spectra: vec![FatigueSpectrum { id: "FS-1".into(), member_id: "beam-B1".into(), detail_category: 71.0e6, stress_range: 80.0e6, cycles: 2.0e6 }],
        reliability_analyses: vec![],
        test_series: vec![],
        placements: vec![],
        cases: vec![],
        factor_overrides: vec![],
        traffic_groups: vec![],
    }
}

