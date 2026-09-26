//! 🧬️ En1990 snapshot — complete basis-of-design subject (project → actions → members).

use crate::document::AnnexChoice;
use crate::{AccidentalAction, BridgeSls, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 📸️ Persisted EN 1990 basis-of-design subject. Forces in N, lengths in m, frequencies in Hz.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(extension = "en1990")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub project_id: String,
    #[state(artifact)]
    pub structure_kind: String,
    #[state(artifact)]
    pub altitude_m: f64,
    #[state(artifact)]
    pub consequence_class: u8,
    #[state(artifact)]
    pub reliability_class: u8,
    #[state(artifact)]
    pub design_working_life_category: u8,
    #[state(artifact)]
    pub design_working_life_years: f64,
    #[state(artifact)]
    pub reference_period_years: f64,
    #[state(artifact)]
    pub supervision_level: String,
    #[state(artifact)]
    pub inspection_level: String,
    #[state(artifact)]
    pub k_fi_declared: f64,
    #[state(artifact)]
    pub beta_computed: f64,
    #[dsl(table)]
    #[state(artifact)]
    pub permanents: Vec<PermanentAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub variables: Vec<VariableAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub accidentals: Vec<AccidentalAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub seismics: Vec<SeismicAction>,
    #[dsl(table)]
    #[state(artifact)]
    pub members: Vec<Member>,
    #[dsl(table)]
    #[state(artifact)]
    pub bridge_sls: Vec<BridgeSls>,
    #[dsl(table)]
    #[state(artifact)]
    pub effects: Vec<MemberEffect>,
}

crate::impl_norm_artifact_record!(En1990Snapshot, extension = "en1990", envelope_id = "norm.en1990");

impl Default for En1990Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            project_id: "office-cc2".into(),
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
            beta_computed: 3.85,
            permanents: vec![
                PermanentAction { id: "G-sup".into(), kind: "g_sup".into(), gk: 80_000.0 },
                PermanentAction { id: "G-inf".into(), kind: "g_inf".into(), gk: 20_000.0 },
            ],
            variables: vec![
                VariableAction { id: "Q-office".into(), category: "office".into(), qk: 40_000.0 },
                VariableAction { id: "Q-wind".into(), category: "wind".into(), qk: 20_000.0 },
            ],
            accidentals: vec![],
            seismics: vec![],
            members: vec![Member {
                id: "beam-B1".into(),
                label_en: "Office beam B1".into(),
                label_de: "Büroträger B1".into(),
                rd_str: 250_000.0,
                rd_geo: 250_000.0,
                rd_equ_stab: 200_000.0,
                rd_equ_destab: 180_000.0,
                rd_fat: 0.0,
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
}
//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge
/// 📤️ Canonical JSON projection of [`En1990Snapshot`].
pub fn encode_en1990_snapshot_json(snapshot: &En1990Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ Inverse of [`encode_en1990_snapshot_json`].
pub fn decode_en1990_snapshot_json(text: &str) -> Result<En1990Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses committed `.dsl.semio` into [`En1990Snapshot`].
pub fn decode_en1990_dsl(text: &str) -> Result<En1990Snapshot, String> {
    <En1990Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints [`En1990Snapshot`] to canonical `.dsl.semio`.
pub fn encode_en1990_dsl(snapshot: &En1990Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes `.pack.semio` into [`En1990Snapshot`].
pub fn decode_en1990_pack(bytes: &[u8]) -> Result<En1990Snapshot, String> {
    <En1990Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes [`En1990Snapshot`] to `.pack.semio`.
pub fn encode_en1990_pack(snapshot: &En1990Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
