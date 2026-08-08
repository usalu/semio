//! 🧬️ Vdi3805 artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Artifact
/// 🧬️ Full Vdi3805 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Artifact {
    #[state(persistent)] pub manufacturer_file: String,
    #[state(persistent)] pub catalog: String,
    #[state(persistent)] pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(persistent)] pub correction_as_of: String,
    #[state(persistent)] pub strict_mode: bool,
    #[state(persistent)] pub index: String,
    #[state(persistent)] pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(persistent)] pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(persistent)] pub limits: String,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Vdi3805Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::vdi3805::Vdi3805Snapshot {
        crate::artifacts::vdi3805::Vdi3805Snapshot {
            manufacturer_file: self.manufacturer_file,
            catalog: self.catalog,
            edition_profile: self.edition_profile,
            correction_as_of: self.correction_as_of,
            strict_mode: self.strict_mode,
            index: self.index,
            geometry: self.geometry,
            curves: self.curves,
            limits: self.limits,
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::vdi3805::Vdi3805Snapshot) -> Self {
        Self {
            manufacturer_file: snapshot.manufacturer_file,
            catalog: snapshot.catalog,
            edition_profile: snapshot.edition_profile,
            correction_as_of: snapshot.correction_as_of,
            strict_mode: snapshot.strict_mode,
            index: snapshot.index,
            geometry: snapshot.geometry,
            curves: snapshot.curves,
            limits: snapshot.limits,
            selected_check_index: None,
        }
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.vdi3805` — fifteen handcrafted schema leaves.
pub fn vdi3805_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.vdi3805",
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