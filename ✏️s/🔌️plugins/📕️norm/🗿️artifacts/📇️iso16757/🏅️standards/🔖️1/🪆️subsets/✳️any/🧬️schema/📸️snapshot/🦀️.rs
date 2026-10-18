//! 🧬️ Iso16757 snapshot schema — artifact-lane fields only.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Snapshot

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.iso16757", layout = "lines")]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Snapshot {
    #[state(artifact)]
    pub catalogue: crate::part_1::Catalogue,
    #[state(artifact)]
    pub dictionary: crate::part_4::Dictionary,
    #[state(artifact)]
    pub geometry: crate::part_2::GeometryCatalogue,
    #[state(artifact)]
    pub selection: crate::part_1::SelectionRequest,
    #[state(artifact)]
    pub part_number_rule: crate::part_5::PartNumberRule,
    #[state(artifact)]
    pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    #[state(artifact)]
    pub script_limits: crate::part_5::ScriptLimits,
    #[state(artifact)]
    pub exchange_process: crate::part_5::ExchangeProcess,
}
//#region 🔖️HandcraftedArtifactCodecs
// 🧬️ Consolidated (W5a, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT): the fifteen norm families' identical
// ArtifactDsl/ArtifactPack envelope-wrap glue now lives once, in `crate::document`'s
// `NormArtifactRecord`/`norm_{parse,print}_dsl`/`norm_{encode,decode}_pack` (see that
// region's doc comment in `📄️artifact/🦀️.rs` for why it can't collapse further
// than this one macro call — Rust's orphan rule still needs a concrete per-type impl).

//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Iso16757Snapshot {
    fn default() -> Self {
        Self::reference_fixture()
    }
}
//#endregion 🔖️Snapshot

/// ♻️ Releases decoded catalogue owners without recursive value destruction.
pub fn retire_decoded_catalogue_value(value:CatalogueValue){let mut pending=vec![value];while let Some(value)=pending.pop(){if let CatalogueValue::List{items}=value{pending.extend(items);}}}

/// ♻️ Releases decoded catalogue owners without recursive value destruction.
pub(crate) fn retire_decoded_catalogue_geometry(node:part_2::GeometryNode){use crate::part_2::GeometryNode as N;let mut pending=vec![node];while let Some(node)=pending.pop(){match node{N::Transform{child,..}=>pending.push(*child),N::Boolean{children,..}=>pending.extend(children),_=>{}}}}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🧱️catalogue-core/🦀️.rs"), include_str!("📚️part-1-catalogue/🦀️.rs"), include_str!("📐️part-2-geometry/🦀️.rs"), include_str!("📖️part-4-dictionary/🦀️.rs"), include_str!("🔄️part-5-exchange/🦀️.rs"), include_str!("🧾️artifact-definition/🦀️.rs")), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

#[path = "🧱️catalogue-core/🦀️.rs"]
mod catalogue_core;
pub use catalogue_core::{Cardinality, CatalogueId, CatalogueUnit, CatalogueValue, DictionaryRef, DimensionSignature, ExtensionBag, Lifecycle, Names, NullState};

#[path = "📚️part-1-catalogue/🦀️.rs"]
pub mod part_1;

#[path = "📐️part-2-geometry/🦀️.rs"]
pub mod part_2;

#[path = "📖️part-4-dictionary/🦀️.rs"]
pub mod part_4;

#[path = "🔄️part-5-exchange/🦀️.rs"]
pub mod part_5;

#[path = "🧾️artifact-definition/🦀️.rs"]
mod artifact_definition;
pub use artifact_definition::{artifact_kind, definition, package_descriptor, ARTIFACT_DEFINITION_SCHEMA, ISO16757_DIALECT, ISO16757_DOCUMENT_SCHEMA};
