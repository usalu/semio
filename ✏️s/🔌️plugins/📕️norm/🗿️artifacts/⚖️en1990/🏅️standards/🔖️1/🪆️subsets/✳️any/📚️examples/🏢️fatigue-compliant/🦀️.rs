//! 📚️ Example `fatigue-compliant` — fatigue (FAT) verification active (R_d,FAT > 0).

use semio_framework_plugin::{ExampleSource, LocalizedLabel};
use crate::{Member, MemberEffect, PermanentAction, VariableAction};

pub const ID: &str = "fatigue-compliant";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Fatigue Compliant", "Ermüdung — konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️fatigue-compliant/🏢️fatigue-compliant/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// ♻️ Fatigue-active subject with R_d,FAT = 200000.0.
pub fn reference_snapshot() -> crate::En1990Snapshot {
    crate::En1990Snapshot {
        annex: crate::document::AnnexChoice::De,
        project_id: "fatigue-compliant".into(),
        structure_kind: "building".into(),
        altitude_m: 0.0,
        consequence_class: 2,
        reliability_class: 2,
        design_working_life_category: 4,
        design_working_life_years: 50.0,
        reference_period_years: 50.0,
        supervision_level: "DSL2".into(),
        inspection_level: "IL2".into(),
        k_fi_declared: 1.0,
        beta_computed: 3.8,
        permanents: vec![
            PermanentAction { id: "G-sup".into(), kind: "g_sup".into(), gk: 80_000.0 },
            PermanentAction { id: "G-inf".into(), kind: "g_inf".into(), gk: 20_000.0 },
        ],
        variables: vec![
            VariableAction { id: "Q-office".into(), category: "office".into(), qk: 40000.0 },
            VariableAction { id: "Q-wind".into(), category: "wind".into(), qk: 20_000.0 },
        ],
        accidentals: vec![],
        seismics: vec![],
        members: vec![Member {
            id: "beam-B1".into(),
            label_en: "Fatigue beam B1".into(),
            label_de: "Ermüdungsträger B1".into(),
            rd_str: 250000.0,
            rd_geo: 250000.0,
            rd_equ_stab: 200_000.0,
            rd_equ_destab: 180_000.0,
            rd_fat: 200000.0,
            span: 6.0,
            deflection_w: 0.018,
            deflection_limit_ratio: 250.0,
            vibration_frequency: 5.5,
            vibration_frequency_min: 3.0,
        }],
        bridge_sls: vec![],
        effects: vec![
            MemberEffect { member_id: "beam-B1".into(), action_id: "G-sup".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "G-inf".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-office".into(), influence: 1.0 },
            MemberEffect { member_id: "beam-B1".into(), action_id: "Q-wind".into(), influence: 1.0 },
        ],
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
