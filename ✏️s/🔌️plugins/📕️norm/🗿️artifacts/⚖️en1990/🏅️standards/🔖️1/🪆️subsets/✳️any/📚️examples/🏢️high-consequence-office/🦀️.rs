//! 📚️ Example `high-consequence-office` — non-compliant multi-failure CC3 subject.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};
use crate::{Member, MemberEffect, PermanentAction, VariableAction};

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
pub fn reference_snapshot() -> crate::En1990Snapshot {
    crate::En1990Snapshot {
        annex: crate::document::AnnexChoice::En,
        project_id: "high-consequence-office".into(),
        structure_kind: "building".into(),
        altitude_m: 0.0,
        consequence_class: 3,
        reliability_class: 3,
        design_working_life_category: 4,
        design_working_life_years: 50.0,
        reference_period_years: 50.0,
        supervision_level: "DSL1".into(),
        inspection_level: "IL1".into(),
        k_fi_declared: 1.0,
        beta_computed: 3.9,
        permanents: vec![
            PermanentAction { id: "G-sup".into(), kind: "g_sup".into(), gk: 250_000.0 },
            PermanentAction { id: "G-inf".into(), kind: "g_inf".into(), gk: 50_000.0 },
        ],
        variables: vec![
            VariableAction { id: "Q-office".into(), category: "office".into(), qk: 60_000.0 },
            VariableAction { id: "Q-partition".into(), category: "other".into(), qk: 12_000.0 },
            VariableAction { id: "Q-snow".into(), category: "snow".into(), qk: 18_000.0 },
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
            deflection_w: 0.04,
            deflection_limit_ratio: 250.0,
            vibration_frequency: 2.5,
            vibration_frequency_min: 3.0,
        }],
        bridge_sls: vec![],
        effects: vec![
            MemberEffect { member_id: "beam-B1".into(), action_id: "G-sup".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "G-inf".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-office".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-partition".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-snow".into(), influence: 1.0 },
        ],
    }
}

//#region ️TaxonomyMounts
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion ️TaxonomyMounts
