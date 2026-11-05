//! 📚️ Example `accidental-seismic-failing`.

use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;
use semio_s_artifact_norm_en1990_model::{ReliabilityClass, AccidentalAction, ImportanceClass, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction};

pub const ID: &str = "accidental-seismic-failing";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Accidental Seismic Failing", "Außergewöhnlich/Erdbeben — nicht konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️accidental-seismic-failing/🏢️accidental-seismic-failing/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 💥🌋️ Non-compliant accidental + seismic design situations (EN 1990 6.11 / 6.12b).
pub fn reference_snapshot() -> semio_s_artifact_norm_en1990_model::En1990Snapshot {
    semio_s_artifact_norm_en1990_model::En1990Snapshot {
        annex: semio_s_plugin_norm_compliance::AnnexChoice::De,
        project_id: "accidental-seismic-failing".into(),
        structure_kind: semio_s_artifact_norm_en1990_model::StructureKind::Building,
        altitude_m: 120.0,
        consequence_class: 2,
        reliability_class: ReliabilityClass::RC2,
        design_working_life_category: 4,
        design_working_life_years: 50.0,
        reference_period_years: 50.0,
        supervision_level: semio_s_artifact_norm_en1990_model::SupervisionLevel::Dsl2,
        inspection_level: semio_s_artifact_norm_en1990_model::InspectionLevel::Il2,
        beta_computed: 3.8,
        permanents: vec![
            PermanentAction { id: "G-sup".into(), kind: semio_s_artifact_norm_en1990_model::PermanentKind::GSup, gk: 100_000.0 },
            PermanentAction { id: "G-inf".into(), kind: semio_s_artifact_norm_en1990_model::PermanentKind::GInf, gk: 20_000.0 },
        ],
        variables: vec![
            VariableAction { id: "Q-office".into(), category: "office".into(), qk: 50_000.0, group: String::new() },
            VariableAction { id: "Q-wind".into(), category: "wind".into(), qk: 25_000.0, group: String::new() },
        ],
        accidentals: vec![
            AccidentalAction { id: "A-impact".into(), ad: 180_000.0 },
        ],
        seismics: vec![
            SeismicAction { id: "E-1".into(), a_ek: 150_000.0, importance_class: ImportanceClass::III },
        ],
        members: vec![Member {
            id: "beam-B1".into(),
            label_en: "Office beam B1".into(),
            label_de: "Büroträger B1".into(),
            rd_str: 150_000.0,
            rd_geo: 150_000.0,
            rd_equ_stab: 120_000.0,
            rd_equ_destab: 100_000.0,
            rd_fat: 0.0,
            span: 6.0,
            precamber: 0.0,
            deflection_w1: 0.01,
            deflection_w2: 0.005,
            deflection_w3: 0.015,
            deflection_limit_ratio: 250.0,
            deflection_limit_ratio_w3: 300.0,
            sls_limit_char: 150_000.0,
            sls_limit_freq: 150_000.0,
            sls_limit_qp: 150_000.0,
            vibration_frequency: 4.0,
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
            MemberEffect { member_id: "beam-B1".into(), action_id: "A-impact".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "E-1".into(), influence: 1.0 },
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

