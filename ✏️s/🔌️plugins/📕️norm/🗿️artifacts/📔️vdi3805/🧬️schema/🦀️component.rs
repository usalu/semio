//! 🧬️ Vdi3805 artifact schema — every field of the artifact with its state class.


use std::collections::BTreeMap;

use schema::ArtifactSchema;
use crate::artifacts::vdi3805::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full Vdi3805 artifact state across persistent and shared-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Artifact {
    #[state(persistent)] pub manufacturer_file: ManufacturerFile,
    #[state(persistent)] pub catalog: ManufacturerCatalog,
    #[state(persistent)] pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(persistent)] pub correction_as_of: EditionId,
    #[state(persistent)] pub strict_mode: bool,
    #[state(persistent)] pub index: CatalogIndex,
    #[state(persistent)] pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(persistent)] pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(persistent)] pub limits: SecurityLimits,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Vdi3805Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::vdi3805::Vdi3805Snapshot {
        crate::artifacts::vdi3805::Vdi3805Snapshot {
            manufacturer_file: self.manufacturer_file.clone(),
            catalog: self.catalog.clone(),
            edition_profile: self.edition_profile.clone(),
            correction_as_of: self.correction_as_of.clone(),
            strict_mode: self.strict_mode,
            index: self.index.clone(),
            geometry: self.geometry.clone(),
            curves: self.curves.clone(),
            limits: self.limits.clone(),
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
    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::vdi3805::Vdi3805Snapshot) {
        let selected = self.selected_check_index;
        *self = Self::from_snapshot(snapshot);
        self.selected_check_index = selected;
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