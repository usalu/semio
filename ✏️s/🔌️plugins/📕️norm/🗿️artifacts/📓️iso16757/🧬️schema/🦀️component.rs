//! 🧬️ Iso16757 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Artifact
/// 🧬️ Full Iso16757 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Artifact {
    #[state(persistent)] pub catalogue: crate::artifacts::iso16757::part_1::Catalogue,
    #[state(persistent)] pub dictionary: crate::artifacts::iso16757::part_4::Dictionary,
    #[state(persistent)] pub geometry: crate::artifacts::iso16757::part_2::GeometryCatalogue,
    #[state(persistent)] pub selection: crate::artifacts::iso16757::part_1::SelectionRequest,
    #[state(persistent)] pub part_number_rule: crate::artifacts::iso16757::part_5::PartNumberRule,
    #[state(persistent)] pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    #[state(persistent)] pub script_limits: crate::artifacts::iso16757::part_5::ScriptLimits,
    #[state(persistent)] pub exchange_process: crate::artifacts::iso16757::part_5::ExchangeProcess,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Iso16757Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::iso16757::Iso16757Snapshot {
        crate::artifacts::iso16757::Iso16757Snapshot {
            catalogue: self.catalogue,
            dictionary: self.dictionary,
            geometry: self.geometry,
            selection: self.selection,
            part_number_rule: self.part_number_rule,
            part_number_inputs: self.part_number_inputs,
            script_limits: self.script_limits,
            exchange_process: self.exchange_process,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::iso16757::Iso16757Snapshot) -> Self {
        Self {
            catalogue: snapshot.catalogue,
            dictionary: snapshot.dictionary,
            geometry: snapshot.geometry,
            selection: snapshot.selection,
            part_number_rule: snapshot.part_number_rule,
            part_number_inputs: snapshot.part_number_inputs,
            script_limits: snapshot.script_limits,
            exchange_process: snapshot.exchange_process,
            selected_check_index: None,
        }
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.iso16757` — fifteen handcrafted schema leaves.
pub fn iso16757_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.iso16757",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor