//! 🧬️ En1990 snapshot — complete basis-of-design subject (project → actions → members).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 📸️ Persisted EN 1990 basis-of-design subject. Forces in N, lengths in m, frequencies in Hz.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
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

impl Default for En1990Snapshot {
    fn default() -> Self {
        Self {
            annex: AnnexChoice::De,
            project_id: "office-cc2".into(),
            structure_kind: "building".into(),
            altitude_m: 1200.0,
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
                VariableAction { id: "Q-snow".into(), category: "snow".into(), qk: 15_000.0 },
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
                MemberEffect { member_id: "beam-B1".into(), action_id: "Q-snow".into(), influence: 1.0 },
            ],
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface;

/// ⚓️ Permanent action entry (G_k,sup / G_k,inf / prestress) — forces in N.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PermanentAction {
    pub id: String,
    pub kind: String,
    pub gk: f64,
}

/// 🏋️ Variable action — category selects ψ (site altitude is on the snapshot).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct VariableAction {
    pub id: String,
    pub category: String,
    pub qk: f64,
}

/// 💥 Accidental design action A_d [N] — EN 1990 §1.5.3.5 / §6.4.3.3 (A_d is already a design value).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct AccidentalAction {
    pub id: String,
    pub ad: f64,
}

/// 🏛 Seismic importance class — EN 1998-1 Table 4.3 / DIN EN 1998-1/NA Table NA.5 (γ_I).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum ImportanceClass {
    #[dsl(key = "i")]
    I,
    #[dsl(key = "ii")]
    II,
    #[dsl(key = "iii")]
    III,
    #[dsl(key = "iv")]
    IV,
}

impl ImportanceClass {
    /// 🏛 Importance factor γ_I per EN 1998-1 Table 4.3 (DE NA Table NA.5 identical).
    pub fn gamma_i(self) -> f64 {
        match self {
            Self::I => 0.8,
            Self::II => 1.0,
            Self::III => 1.2,
            Self::IV => 1.4,
        }
    }
}

/// 🌋️ Characteristic seismic action A_Ek [N] with importance class; A_Ed = γ_I · A_Ek (EN 1990 §6.4.3.4).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct SeismicAction {
    pub id: String,
    pub a_ek: f64,
    pub importance_class: ImportanceClass,
}

impl SeismicAction {
    /// 🌕️ Design seismic action A_Ed = γ_I · A_Ek (EN 1990 §6.4.3.4).
    pub fn a_ed(&self) -> f64 {
        self.importance_class.gamma_i() * self.a_ek
    }
}

/// 🏗️ Member / verification point with design resistances and SLS criteria (SI: N, m, Hz).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Member {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub rd_str: f64,
    pub rd_geo: f64,
    pub rd_equ_stab: f64,
    pub rd_equ_destab: f64,
    pub rd_fat: f64,
    pub span: f64,
    pub deflection_w: f64,
    pub deflection_limit_ratio: f64,
    pub vibration_frequency: f64,
    pub vibration_frequency_min: f64,
}

/// 🌉 Annex A2.4 SLS criteria for a bridge member (only used when structureKind is a bridge).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BridgeSls {
    pub id: String,
    pub member_id: String,
    pub deck_acceleration: f64,
    pub deck_acceleration_limit: f64,
    pub deck_twist: f64,
    pub deck_twist_limit: f64,
    pub bridge_deflection: f64,
    pub bridge_deflection_limit: f64,
}

/// 🔗 Influence coefficient mapping a catalogue action magnitude onto a member effect (dimensionless).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MemberEffect {
    pub member_id: String,
    pub action_id: String,
    pub influence: f64,
}

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1990", "EN 1990", EN1990_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1990_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1990", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1990_DOCUMENT_SCHEMA: &str = "semio.norm.en1990/v1";
