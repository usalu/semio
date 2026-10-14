//! 🧬️ Iso16757 artifact schema — every field of the artifact with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use std::collections::BTreeMap;

use semio_s_artifact_norm_iso16757_model::CatalogueValue;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full Iso16757 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Artifact {
    #[state(artifact)]
    pub catalogue: semio_s_artifact_norm_iso16757_model::part_1::Catalogue,
    #[state(artifact)]
    pub dictionary: semio_s_artifact_norm_iso16757_model::part_4::Dictionary,
    #[state(artifact)]
    pub geometry: semio_s_artifact_norm_iso16757_model::part_2::GeometryCatalogue,
    #[state(artifact)]
    pub selection: semio_s_artifact_norm_iso16757_model::part_1::SelectionRequest,
    #[state(artifact)]
    pub part_number_rule: semio_s_artifact_norm_iso16757_model::part_5::PartNumberRule,
    #[state(artifact)]
    pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    #[state(artifact)]
    pub script_limits: semio_s_artifact_norm_iso16757_model::part_5::ScriptLimits,
    #[state(artifact)]
    pub exchange_process: semio_s_artifact_norm_iso16757_model::part_5::ExchangeProcess,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Iso16757Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> semio_s_artifact_norm_iso16757_model::Iso16757Snapshot {
        semio_s_artifact_norm_iso16757_model::Iso16757Snapshot {
            catalogue: self.catalogue.clone(),
            dictionary: self.dictionary.clone(),
            geometry: self.geometry.clone(),
            selection: self.selection.clone(),
            part_number_rule: self.part_number_rule.clone(),
            part_number_inputs: self.part_number_inputs.clone(),
            script_limits: self.script_limits,
            exchange_process: self.exchange_process,
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: semio_s_artifact_norm_iso16757_model::Iso16757Snapshot) -> Self {
        Self {
            catalogue: snapshot.catalogue,
            dictionary: snapshot.dictionary,
            geometry: snapshot.geometry,
            selection: snapshot.selection,
            part_number_rule: snapshot.part_number_rule,
            part_number_inputs: snapshot.part_number_inputs,
            script_limits: snapshot.script_limits,
            exchange_process: snapshot.exchange_process,
        }
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.iso16757` — twenty handcrafted schema leaves.
pub fn iso16757_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.iso16757",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_iso16757_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_iso16757_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_iso16757_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

