//! 🧬️ Vdi3805 snapshot schema — artifact-lane fields only.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.vdi3805", layout = "lines")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Snapshot {
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

impl Default for Vdi3805Snapshot {
    fn default() -> Self {
        crate::reference_fixture()
    }
}

#[path = "🔤️localized-text-and-units/🦀️.rs"]
mod localized_text_and_units;
pub use localized_text_and_units::{bilingual, text_in, VdiQuantityKind, VdiUnit};

#[path = "🏷️product-identity-and-links/🦀️.rs"]
mod product_identity_and_links;
pub use product_identity_and_links::{AccessoryLink, CompositionLink, ExtensionBag, ManufacturerFile, ProductIdentity};

#[path = "🛡️limits-and-code-tables/🦀️.rs"]
mod limits_and_code_tables;
pub use limits_and_code_tables::{
    SecurityLimits, CONNECTION_TYPE_CODES, CONNECTION_TYPE_ROWS, FILTER_CLASS_CODES, FILTER_CLASS_ROWS, PRESSURE_CLASS_CODES, PRESSURE_CLASS_ROWS, SHEET_NUMERIC_BOUNDS_16_PRESSURE_DROP_PA, SHEET_NUMERIC_BOUNDS_19_AIRFLOW,
    SHEET_NUMERIC_BOUNDS_53_COP, SHEET_NUMERIC_BOUNDS_53_DN, SHEET_NUMERIC_BOUNDS_53_HEAT_W, SHEET_NUMERIC_BOUNDS_AXIAL_FORCE_N, SHEET_NUMERIC_BOUNDS_DN, SHEET_NUMERIC_BOUNDS_DN_PIPE, SHEET_NUMERIC_BOUNDS_OUTER_DIAMETER_M,
    SHEET_NUMERIC_BOUNDS_VOLUME_M3, SHEET_NUMERIC_BOUNDS_WALL_THICKNESS_M, TYPE_CODE_CODES,
};

#[path = "🗂️sheet-registry/🦀️.rs"]
mod sheet_registry;
pub use sheet_registry::{Domain, EditionId, SchemaStatus, SheetEntry, SheetId, SHEET_ENTRIES};

#[path = "🧾️record-families/🦀️.rs"]
mod record_families;
pub use record_families::{BuildingSystemNumber, RecordFamilyId};

#[path = "🧮️sheet-attributes/🦀️.rs"]
mod sheet_attributes;
pub use sheet_attributes::{
    attributes_from_records, sheet_attribute_field, sheet_attribute_object, sheet_attribute_real, sheet_attribute_u16, GenericAttribute, GenericAttributes, HeatGeneratorAttributes, NativeRecord, PumpHeatingAttributes, RadiatorAttributes,
    SheetAttributes, ValveHeatingAttributes,
};

#[path = "📚️catalog-model/🦀️.rs"]
mod catalog_model;
pub use catalog_model::{BoundingBox, CatalogIndex, CatalogIndexEntry, CatalogueProduct, CharacteristicCurve, Configuration, ConnectionPoint, CurvePoint, ManufacturerCatalog, ParametricGeometry};

#[path = "🩺️diagnostics-and-edition-profile/🦀️.rs"]
mod diagnostics_and_edition_profile;
pub use diagnostics_and_edition_profile::{Diagnostic, EditionProfileChoice, Severity};

#[path = "🧪️reference-fixture/🦀️.rs"]
mod reference_dataset;
pub use reference_dataset::{conforming_valve_dataset, reference_fixture};

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: concat!(include_str!("🦀️.rs"), include_str!("🔤️localized-text-and-units/🦀️.rs"), include_str!("🏷️product-identity-and-links/🦀️.rs"), include_str!("🛡️limits-and-code-tables/🦀️.rs"), include_str!("🗂️sheet-registry/🦀️.rs"), include_str!("🧾️record-families/🦀️.rs"), include_str!("🧮️sheet-attributes/🦀️.rs"), include_str!("📚️catalog-model/🦀️.rs"), include_str!("🩺️diagnostics-and-edition-profile/🦀️.rs"), include_str!("🧪️reference-fixture/🦀️.rs")), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("vdi3805", "VDI 3805", VDI3805_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const VDI3805_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.vdi3805", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const VDI3805_DOCUMENT_SCHEMA: &str = "semio.norm.vdi3805/v1";
