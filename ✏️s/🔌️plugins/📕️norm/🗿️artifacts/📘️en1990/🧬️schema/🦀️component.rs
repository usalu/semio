//! 🧬️ En1990 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full En1990 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Artifact {
    #[state(persistent)] pub g_k: f64,
    #[state(persistent)] pub q_k: Vec<En1990QkEntry>,
    #[state(persistent)] pub resistance_kn: f64,
    #[state(persistent)] pub consequence_class: u8,
    #[state(persistent)] pub annex: crate::document::AnnexChoice,
    #[state(persistent)] pub seismic_a_ed_kn: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Helpers
/// 📊️ One variable action category/value pair for `En1990Snapshot.q_k`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1990QkEntry {
    pub category: String,
    pub value: f64,
}
//#endregion 🔖️Helpers

//#region 🔖️Conversions
impl En1990Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::en1990::En1990Snapshot {
        crate::artifacts::en1990::En1990Snapshot {
            g_k: self.g_k,
            q_k: self.q_k.clone(),
            resistance_kn: self.resistance_kn,
            consequence_class: self.consequence_class,
            annex: self.annex,
            seismic_a_ed_kn: self.seismic_a_ed_kn,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::en1990::En1990Snapshot) -> Self {
        Self {
            g_k: snapshot.g_k,
            q_k: snapshot.q_k.clone(),
            resistance_kn: snapshot.resistance_kn,
            consequence_class: snapshot.consequence_class,
            annex: snapshot.annex,
            seismic_a_ed_kn: snapshot.seismic_a_ed_kn,
            selected_check_index: None,
        }
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1990` — fifteen handcrafted schema leaves.
pub fn en1990_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1990",
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