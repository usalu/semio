//! 🧬️ Vdi3805 artifact schema — every field of the artifact with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use std::collections::BTreeMap;

use semio_s_artifact_norm_vdi3805_model::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ParametricGeometry, SecurityLimits};
use ::framework_schema::ArtifactSchema;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;

//#region 🔖️Artifact
/// 🧬️ Full Vdi3805 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Artifact {
    #[state(artifact)]
    pub catalog: ManufacturerCatalog,
    #[state(artifact)]
    pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(artifact)]
    pub correction_as_of: EditionId,
    #[state(artifact)]
    pub strict_mode: bool,
    #[state(artifact)]
    pub index: CatalogIndex,
    #[state(artifact)]
    pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(artifact)]
    pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(artifact)]
    pub limits: SecurityLimits,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Vdi3805Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Vdi3805Snapshot {
        Vdi3805Snapshot {
            catalog: self.catalog.clone(),
            edition_profile: self.edition_profile.clone(),
            correction_as_of: self.correction_as_of,
            strict_mode: self.strict_mode,
            index: self.index.clone(),
            geometry: self.geometry.clone(),
            curves: self.curves.clone(),
            limits: self.limits,
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Vdi3805Snapshot) -> Self {
        Self {
            catalog: snapshot.catalog,
            edition_profile: snapshot.edition_profile,
            correction_as_of: snapshot.correction_as_of,
            strict_mode: snapshot.strict_mode,
            index: snapshot.index,
            geometry: snapshot.geometry,
            curves: snapshot.curves,
            limits: snapshot.limits,
        }
    }
}

//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.vdi3805` — twenty handcrafted schema leaves.
pub fn vdi3805_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.vdi3805",
        artifact: ::semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_vdi3805_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_vdi3805_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_vdi3805_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

