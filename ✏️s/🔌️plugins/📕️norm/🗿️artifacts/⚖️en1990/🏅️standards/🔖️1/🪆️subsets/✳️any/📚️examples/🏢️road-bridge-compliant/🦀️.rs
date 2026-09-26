//! 📚️ Example `road-bridge-compliant` — CC2 road bridge with Annex A2.4 SLS within limits.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};
use crate::{BridgeSls, Member, MemberEffect, PermanentAction, VariableAction};

pub const ID: &str = "road-bridge-compliant";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Rail Bridge Compliant", "Eisenbahnbrücke — konform")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️road-bridge-compliant/🏢️road-bridge-compliant/🗣️.dsl.semio");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

/// 🌉 Compliant road bridge: STR/GEO/SLS and Annex A2.4 bridge deflection within limits.
pub fn reference_snapshot() -> crate::En1990Snapshot {
    crate::En1990Snapshot {
        annex: crate::document::AnnexChoice::De,
        project_id: "road-bridge-compliant".into(),
        structure_kind: "rail_bridge".into(),
        altitude_m: 200.0,
        consequence_class: 2,
        reliability_class: 2,
        design_working_life_category: 5,
        design_working_life_years: 100.0,
        reference_period_years: 100.0,
        supervision_level: "DSL2".into(),
        inspection_level: "IL2".into(),
        k_fi_declared: 1.0,
        beta_computed: 3.8,
        permanents: vec![
            PermanentAction { id: "G-sup".into(), kind: "g_sup".into(), gk: 120_000.0 },
            PermanentAction { id: "G-inf".into(), kind: "g_inf".into(), gk: 40_000.0 },
        ],
        variables: vec![
            VariableAction { id: "Q-traffic".into(), category: "rail_traffic".into(), qk: 80_000.0 },
            VariableAction { id: "Q-wind".into(), category: "wind".into(), qk: 20_000.0 },
        ],
        accidentals: vec![],
        seismics: vec![],
        members: vec![Member {
            id: "deck-D1".into(),
            label_en: "Main deck D1".into(),
            label_de: "Hauptfahrbahn D1".into(),
            rd_str: 400_000.0,
            rd_geo: 400_000.0,
            rd_equ_stab: 300_000.0,
            rd_equ_destab: 250_000.0,
            rd_fat: 0.0,
            span: 40.0,
            deflection_w: 0.05,
            deflection_limit_ratio: 500.0,
            vibration_frequency: 4.0,
            vibration_frequency_min: 3.0,
        }],
        bridge_sls: vec![BridgeSls {
            id: "sls-D1".into(),
            member_id: "deck-D1".into(),
            deck_acceleration: 0.2,
            deck_acceleration_limit: 0.5,
            deck_twist: 0.0002,
            deck_twist_limit: 0.001,
            bridge_deflection: 0.04,
            bridge_deflection_limit: 0.08,
        }],
        effects: vec![
            MemberEffect { member_id: "deck-D1".into(), action_id: "G-sup".into(), influence: 1.0 },
            MemberEffect { member_id: "deck-D1".into(), action_id: "G-inf".into(), influence: 1.0 },
            MemberEffect { member_id: "deck-D1".into(), action_id: "Q-traffic".into(), influence: 1.0 },
            MemberEffect { member_id: "deck-D1".into(), action_id: "Q-wind".into(), influence: 1.0 },
        ],
    }
}

//#region ️TaxonomyMounts
#[cfg(test)]
#[path = "🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion ️TaxonomyMounts
