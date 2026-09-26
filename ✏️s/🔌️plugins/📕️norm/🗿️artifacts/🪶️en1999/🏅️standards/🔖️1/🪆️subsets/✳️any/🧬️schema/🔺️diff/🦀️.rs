//! 🧬️ EN 1999 diff schema — sparse field delta over the aluminium-structure subject.

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::En1999Artifact>>,
    #[state(artifact)]
    pub annex: Option<crate::document::AnnexChoice>,
    #[state(artifact)]
    pub materials: Option<Vec<crate::snapshot::AluminiumMaterial>>,
    #[state(artifact)]
    pub sections: Option<Vec<crate::snapshot::AluminiumSection>>,
    #[state(artifact)]
    pub members: Option<Vec<crate::snapshot::AluminiumMember>>,
    #[state(artifact)]
    pub connections: Option<Vec<crate::snapshot::AluminiumConnection>>,
    #[state(artifact)]
    pub fire_scenarios: Option<Vec<crate::snapshot::FireScenario>>,
    #[state(artifact)]
    pub fatigue_details: Option<Vec<crate::snapshot::FatigueDetail>>,
    #[state(artifact)]
    pub cold_formed: Option<Vec<crate::snapshot::ColdFormedSheet>>,
    #[state(artifact)]
    pub shells: Option<Vec<crate::snapshot::AluminiumShell>>,
}

//#endregion 🔖️Diff
