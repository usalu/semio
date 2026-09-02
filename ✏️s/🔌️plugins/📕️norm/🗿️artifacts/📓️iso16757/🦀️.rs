//! 📦️ ISO 16757 building-services product catalogue: parts 1, 2, 4, 5 — document entities.

pub use crate::artifacts::iso16757::schema::snapshot::Iso16757Snapshot;

use std::collections::BTreeMap;

// #region Shared
/// 🆔️ Stable catalogue identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(transparent)]
pub struct CatalogueId(pub String);

/// 🔗️ Hand `DslField` bridge for `CatalogueId`: a tuple ("newtype") struct has no named fields for
/// `#[derive(dsl::DslRecord)]` to enumerate, so it binds directly as `Shape::Text` instead of
/// changing its public tuple shape (used pervasively as `.0` across this crate).
impl dsl::DslField for CatalogueId {
    fn shape() -> dsl::Shape {
        dsl::Shape::Text
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Text(s) => Ok(CatalogueId(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

/// 🆔️ Dictionary identifier with version.
#[derive(Clone, Debug, PartialEq, Eq, Hash, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct DictionaryRef {
    pub id: String,
    pub version: String,
}

/// 🌐️ Locale-tagged text — re-exported from `crate::document`, the single canonical definition
/// shared across every norm artifact (kills this type's former duplicate here and in `vdi3805`;
/// see `crate::document`'s `🔖️LocalizedText` region for the full rationale).
pub use crate::document::LocalizedText;

/// 📝️ Preferred and alternative names.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Names {
    pub preferred: LocalizedText,
    pub short_name: Option<String>,
    #[dsl(table)]
    pub alternatives: Vec<LocalizedText>,
}

/// 📊️ Physical dimension signature for unit compatibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct DimensionSignature {
    pub length: i8,
    pub mass: i8,
    pub time: i8,
    pub temperature: i8,
}

impl DimensionSignature {
    pub const DIMENSIONLESS: Self = Self { length: 0, mass: 0, time: 0, temperature: 0 };
    pub const LENGTH: Self = Self { length: 1, mass: 0, time: 0, temperature: 0 };
    pub const LENGTH_3: Self = Self { length: 3, mass: 0, time: 0, temperature: 0 };

    pub fn compatible(self, other: Self) -> bool {
        self == other
    }
}

/// 📐️ Catalogue unit with canonical SI display.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CatalogueUnit {
    pub symbol: String,
    pub dimension: DimensionSignature,
    pub si_factor: f64,
}

/// 🔢️ Typed catalogue value.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum CatalogueValue {
    Boolean { value: bool },
    Integer { value: i64 },
    Decimal { value: f64 },
    Text { value: String },
    Identifier { value: String },
    Enumeration { value: String },
    Controlled { value: String, list_id: String },
    Quantity { value: f64, unit: CatalogueUnit },
    Range { min: f64, max: f64, unit: Option<CatalogueUnit> },
    Null { state: NullState },
    Reference { target_id: String },
    List { items: Vec<CatalogueValue> },
}

/// 🔗️ Hand `DslField` bridge for `CatalogueValue`: a deeply serde-tagged data enum that is also
/// embedded as a `BTreeMap`/`Vec` VALUE type in several places (`shared_property_values`,
/// `parameter_values`, `part_number_inputs`), which mechanically requires `DslField` (map/list
/// values bind through `DslField`, not `DslVariants`) — `#[derive(dsl::DslEnum)]` only produces
/// `DslVariants`, so it can't satisfy those sites. Binds through `Shape::Value` (the engine's
/// existing serde_json escape hatch), reusing the `Serialize`/`Deserialize` this type already has.
impl dsl::DslField for CatalogueValue {
    fn shape() -> dsl::Shape {
        dsl::Shape::Value
    }
    fn to_value(&self) -> dsl::FieldValue {
        dsl::FieldValue::Value(dsl::to_dsl_value(self).expect("CatalogueValue always serializes to DslValue"))
    }
    fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
        match value {
            dsl::FieldValue::Value(dsl_value) => {
                let normalized = store::pack_rt::renormalize_whole_number_floats(dsl_value.clone());
                dsl::from_dsl_value(normalized)
            }
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

/// ∅ Value availability states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum NullState {
    Unavailable,
    Unknown,
    /// 🔡️ `NotApplicable` auto-kebabs to `not-applicable`; kept camelCase to match this crate's
    /// ISO-16757-native external property naming convention (see `SubjectKind`/`RelationshipKind`).
    #[dsl(key = "notApplicable")]
    NotApplicable,
}

/// 🔢️ Cardinality constraint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Cardinality {
    pub min: u32,
    pub max: Option<u32>,
}

impl Cardinality {
    pub fn optional() -> Self {
        Self { min: 0, max: Some(1) }
    }

    pub fn required() -> Self {
        Self { min: 1, max: Some(1) }
    }

    pub fn unbounded() -> Self {
        Self { min: 0, max: None }
    }

    pub fn satisfies(&self, count: u32) -> bool {
        count >= self.min && self.max.is_none_or(|max| count <= max)
    }
}

/// 🔗️ Internal or external reference.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct CatalogueReference {
    pub uri: String,
    pub label: Option<String>,
}

/// 🧩️ Lossless extension bag for unknown fields.
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct ExtensionBag {
    pub fields: BTreeMap<String, dsl::DslValue>,
}

/// 📅️ Lifecycle metadata.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub struct Lifecycle {
    pub revision: String,
    pub status: String,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
}
// #endregion Shared

// #region Part1
pub mod part_1 {
    use super::*;

    /// 🏭️ Manufacturer metadata.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct Manufacturer {
        pub id: String,
        pub names: Names,
    }

    /// 📦️ Product group declaration.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ProductGroup {
        pub id: String,
        pub names: Names,
        pub dictionary_subject_id: Option<String>,
    }

    /// 🏷️ Product class in a hierarchy.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ProductClass {
        pub id: String,
        pub group_id: String,
        pub parent_id: Option<String>,
        pub names: Names,
        pub required_property_ids: Vec<String>,
        pub optional_property_ids: Vec<String>,
    }

    /// 📚️ Product series sharing geometry and properties.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ProductSeries {
        pub id: String,
        pub class_id: String,
        pub names: Names,
        pub shared_property_values: BTreeMap<String, CatalogueValue>,
        pub geometry_id: Option<String>,
    }

    /// 🔧️ Variant parameter domain.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ParameterDomain {
        pub parameter_id: String,
        pub allowed_values: Vec<CatalogueValue>,
        pub default_value: Option<CatalogueValue>,
    }

    /// 🧮️ Property definition.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct PropertyDefinition {
        pub id: String,
        pub names: Names,
        pub data_type: String,
        pub unit: Option<CatalogueUnit>,
        pub cardinality: Cardinality,
        pub kind: PropertyKind,
        pub dictionary_property_id: Option<String>,
    }

    /// 📊️ Property kind per Part 1 §5.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum PropertyKind {
        Static,
        Dynamic,
        Selection,
        External,
    }

    /// 📋️ Property value on a product or variant.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct PropertyValue {
        pub definition_id: String,
        pub value: CatalogueValue,
        pub function_id: Option<String>,
    }

    /// 🧩️ Product variant with parameters.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ProductVariant {
        pub id: String,
        pub parameter_values: BTreeMap<String, CatalogueValue>,
        pub property_values: Vec<PropertyValue>,
        pub article_number: Option<String>,
        pub geometry_id: Option<String>,
    }

    /// 📦️ Catalogue product (generic or resolved).
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct Product {
        pub id: String,
        pub series_id: String,
        pub names: Names,
        pub parameter_domains: Vec<ParameterDomain>,
        pub variants: Vec<ProductVariant>,
        pub static_properties: Vec<PropertyValue>,
    }

    /// 🔍️ Product index for selection.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ProductIndex {
        pub id: String,
        pub product_id: String,
        pub variant_id: Option<String>,
        pub search_tags: Vec<String>,
    }

    /// 🔗️ Accessory relationship.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct AccessoryRelationship {
        pub accessory_product_id: String,
        pub required: bool,
        pub quantity: Cardinality,
        pub compatibility_condition: Option<String>,
    }

    /// 🧱️ Composition relationship (`hasPart`).
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct CompositionRelationship {
        pub component_product_id: String,
        pub quantity: u32,
    }

    /// 🖼️ Geometry reference.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct GeometryReference {
        pub geometry_id: String,
        pub lod: Option<String>,
    }

    /// 📄️ Descriptive media object.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct DescriptiveObject {
        pub id: String,
        pub media_type: String,
        pub uri: String,
        pub language: Option<String>,
        pub checksum: Option<String>,
    }

    /// 📚️ Full catalogue document.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct Catalogue {
        pub id: CatalogueId,
        pub metadata: CatalogueMetadata,
        pub manufacturer: Manufacturer,
        pub dictionary: DictionaryRef,
        #[dsl(table)]
        pub product_groups: Vec<ProductGroup>,
        #[dsl(table)]
        pub product_classes: Vec<ProductClass>,
        #[dsl(table)]
        pub product_series: Vec<ProductSeries>,
        #[dsl(table)]
        pub products: Vec<Product>,
        #[dsl(table)]
        pub product_indexes: Vec<ProductIndex>,
        #[dsl(table)]
        pub property_definitions: Vec<PropertyDefinition>,
        pub accessories: BTreeMap<String, Vec<AccessoryRelationship>>,
        pub compositions: BTreeMap<String, Vec<CompositionRelationship>>,
        #[dsl(table)]
        pub descriptive_objects: Vec<DescriptiveObject>,
        pub extensions: ExtensionBag,
    }

    /// 📋️ Catalogue metadata.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct CatalogueMetadata {
        pub names: Names,
        pub lifecycle: Lifecycle,
        pub edition_profile: EditionProfile,
    }

    /// 📑️ Supported ISO 16757 edition profile.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum EditionProfile {
        #[dsl(key = "part1_2015")]
        Part1_2015,
        #[dsl(key = "part2_2016")]
        Part2_2016,
        #[dsl(key = "part4_2025")]
        Part4_2025,
        #[dsl(key = "part5_2025")]
        Part5_2025,
        #[dsl(key = "fullPublished")]
        FullPublished,
    }

    /// 🎯️ Selection constraint on a property.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct SelectionConstraint {
        pub property_id: String,
        pub operator: ConstraintOperator,
        pub value: CatalogueValue,
    }

    /// ⚖️ Constraint operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum ConstraintOperator {
        Equal,
        #[dsl(key = "notEqual")]
        NotEqual,
        #[dsl(key = "lessThan")]
        LessThan,
        #[dsl(key = "greaterThan")]
        GreaterThan,
        #[dsl(key = "inRange")]
        InRange,
    }

    /// 🔎️ Selection request.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct SelectionRequest {
        pub class_id: String,
        pub constraints: Vec<SelectionConstraint>,
        pub series_id: Option<String>,
    }

    /// ✅️ Selection outcome.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct SelectionResult {
        pub matches: Vec<ProductIndex>,
        pub ambiguity: bool,
        pub explanations: Vec<String>,
    }

    /// 🏗️ BIM embedding workflow state.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct BimEmbedding {
        pub selected_index_id: String,
        pub frozen_parameters: std::collections::HashMap<String, CatalogueValue>,
        pub resolved_properties: Vec<PropertyValue>,
        pub resolved_article_number: Option<String>,
        pub resolved_geometry_id: Option<String>,
        pub catalogue_provenance: CatalogueId,
        pub dictionary_provenance: DictionaryRef,
    }
}
// #endregion Part1

// #region Part2
pub mod part_2 {
    use super::*;

    /// 📐️ Space classification per Part 2 §5.3.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum SpaceKind {
        Overall,
        Operation,
        Access,
        #[dsl(key = "placementTransportation")]
        PlacementTransportation,
        Installation,
    }

    /// 🔌️ Port medium and direction.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct PortDefinition {
        pub id: String,
        pub medium: String,
        pub position: [f64; 3],
        pub direction: [f64; 3],
        pub port_type: String,
    }

    /// 📦️ Axis-aligned bounding box.
    #[derive(Clone, Copy, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct BoundingBox {
        pub min: [f64; 3],
        pub max: [f64; 3],
    }

    impl BoundingBox {
        pub fn from_size(width: f64, height: f64, depth: f64) -> Self {
            Self { min: [0.0, 0.0, 0.0], max: [width, height, depth] }
        }

        pub fn volume_m3(self) -> f64 {
            let dx = self.max[0] - self.min[0];
            let dy = self.max[1] - self.min[1];
            let dz = self.max[2] - self.min[2];
            dx * dy * dz
        }

        pub fn overlaps(self, other: Self, clearance_m: f64) -> bool {
            self.min[0] - clearance_m < other.max[0]
                && self.max[0] + clearance_m > other.min[0]
                && self.min[1] - clearance_m < other.max[1]
                && self.max[1] + clearance_m > other.min[1]
                && self.min[2] - clearance_m < other.max[2]
                && self.max[2] + clearance_m > other.min[2]
        }
    }

    /// 🧱️ CSG primitive kind registry entry.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct PrimitiveKind {
        pub id: String,
        pub parameters: Vec<String>,
    }

    /// 🌳️ Geometry node in a CSG tree — recursive `#[derive(dsl::DslEnum)]`: `Transform.child` is
    /// `#[dsl(statements)] Box<GeometryNode>` (exactly one nested tagged value) and
    /// `Boolean.children` is `#[dsl(statements, block)] Vec<GeometryNode>` (a nested tagged
    /// collection), both recursing back into this same enum's own `DslVariants` impl.
    #[derive(Clone, Debug, PartialEq, dsl::DslEnum, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(tag = "node", rename_all = "camelCase"))]
    #[value(tag = "node", rename_all = "camelCase")]
    pub enum GeometryNode {
        Primitive {
            kind: String,
            parameters: BTreeMap<String, f64>,
        },
        Transform {
            translation: [f64; 3],
            rotation_deg: [f64; 3],
            #[dsl(statements)]
            child: Box<GeometryNode>,
        },
        Boolean {
            operator: BooleanOperator,
            #[dsl(statements, block)]
            children: Vec<GeometryNode>,
        },
        Reference {
            geometry_id: String,
        },
    }

    /// ➕️ Boolean CSG operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum BooleanOperator {
        Union,
        Intersection,
        Difference,
    }

    /// 🏗️ Complete geometry object.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct GeometryObject {
        pub id: String,
        #[dsl(statements, block)]
        pub shape: Option<GeometryNode>,
        #[dsl(statements, block)]
        pub symbolic: Option<GeometryNode>,
        pub spaces: Vec<SpaceEnvelope>,
        pub surfaces: Vec<SurfaceDefinition>,
        pub ports: Vec<PortDefinition>,
        pub parameter_bindings: BTreeMap<String, String>,
    }

    /// 📦️ Space envelope with kind.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct SpaceEnvelope {
        pub kind: SpaceKind,
        pub bounds: BoundingBox,
    }

    /// 🎨️ Semantic surface.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct SurfaceDefinition {
        pub id: String,
        pub purpose: String,
        pub bounds: BoundingBox,
    }

    /// 📚️ Geometry catalogue index.
    #[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct GeometryCatalogue {
        pub objects: BTreeMap<String, GeometryObject>,
        #[dsl(table)]
        pub primitive_registry: Vec<PrimitiveKind>,
    }

    impl GeometryCatalogue {
        pub fn default_primitives() -> Vec<PrimitiveKind> {
            vec![
                PrimitiveKind { id: "box".into(), parameters: vec!["width".into(), "height".into(), "depth".into()] },
                PrimitiveKind { id: "cylinder".into(), parameters: vec!["radius".into(), "height".into()] },
                PrimitiveKind { id: "sphere".into(), parameters: vec!["radius".into()] },
            ]
        }
    }
}
// #endregion Part2

// #region Part4
pub mod part_4 {
    use super::*;

    /// 🏷️ Dictionary subject kind.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum SubjectKind {
        #[dsl(key = "productGroup")]
        ProductGroup,
        #[dsl(key = "productClass")]
        ProductClass,
        #[dsl(key = "productSpecialization")]
        ProductSpecialization,
        #[dsl(key = "catalogueMetadata")]
        CatalogueMetadata,
        #[dsl(key = "manufacturerMetadata")]
        ManufacturerMetadata,
        #[dsl(key = "propertyBlock")]
        PropertyBlock,
        Port,
        Inlet,
        Outlet,
        #[dsl(key = "inOutlet")]
        InOutlet,
    }

    /// 📖️ Dictionary subject.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct Subject {
        pub id: String,
        pub kind: SubjectKind,
        pub names: Names,
        pub definition: LocalizedText,
        pub parent_id: Option<String>,
    }

    /// 🔗️ Relationship kind per Part 4 §4.4.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum RelationshipKind {
        #[dsl(key = "isSubtypeOf")]
        IsSubtypeOf,
        #[dsl(key = "hasPart")]
        HasPart,
        #[dsl(key = "hasBlock")]
        HasBlock,
        #[dsl(key = "isDependentOn")]
        IsDependentOn,
        #[dsl(key = "isSubkindOf")]
        IsSubkindOf,
    }

    /// 🔗️ Typed relationship.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct Relationship {
        pub id: String,
        pub kind: RelationshipKind,
        pub source_id: String,
        pub target_id: String,
        pub cardinality: Cardinality,
    }

    /// 📊️ Dictionary property definition.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct DictionaryProperty {
        pub id: String,
        pub names: Names,
        pub kind: part_1::PropertyKind,
        pub data_type: String,
        pub unit: Option<CatalogueUnit>,
        pub applicable_subject_ids: Vec<String>,
        pub value_constraints: Vec<ValueConstraint>,
    }

    /// ✅️ Controlled value list.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ControlledValueList {
        pub id: String,
        pub values: Vec<String>,
        pub context_subject_ids: Vec<String>,
    }

    /// 🎯️ Value constraint on a property.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ValueConstraint {
        pub min: Option<f64>,
        pub max: Option<f64>,
        pub allowed_values: Vec<String>,
    }

    /// 📚️ Data dictionary snapshot.
    #[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct Dictionary {
        pub reference: DictionaryRef,
        #[dsl(table)]
        pub subjects: Vec<Subject>,
        #[dsl(table)]
        pub relationships: Vec<Relationship>,
        #[dsl(table)]
        pub properties: Vec<DictionaryProperty>,
        #[dsl(table)]
        pub controlled_lists: Vec<ControlledValueList>,
        #[dsl(table)]
        pub meta_subjects: Vec<Subject>,
    }

    /// 🗺️ ISO 12006-3 mapping record.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct Iso12006Mapping {
        pub dictionary_object_id: String,
        pub iso12006_uri: String,
        pub object_kind: String,
    }
}
// #endregion Part4

// #region Part5
pub mod part_5 {
    use super::*;

    /// 🔄️ Exchange process stage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::DslScalar, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub enum ExchangeProcess {
        #[dsl(key = "createFromDictionary")]
        CreateFromDictionary,
        #[dsl(key = "provideCatalogue")]
        ProvideCatalogue,
        #[dsl(key = "determineProduct")]
        DetermineProduct,
        #[dsl(key = "integrateIntoSystem")]
        IntegrateIntoSystem,
        #[dsl(key = "exchangeSystemModel")]
        ExchangeSystemModel,
    }

    /// 🔢️ Part number rule.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum PartNumberRule {
        Literal { value: String },
        Table { rows: Vec<BTreeMap<String, String>>, output_column: String },
        Script { function_id: String, source: String },
    }

    /// 🔗️ Hand `DslField` bridge for `PartNumberRule`: embedded as a BARE (non-`Vec`/`Option`/`Box`)
    /// field on `Document`, so `#[dsl(statements)]` has no effect (the derive only recognizes that
    /// attribute on `Box<T>`/`Vec<T>`/`Option<T>` wrappers) — binding through `Shape::Value` avoids
    /// changing `Document.part_number_rule`'s plain-enum public shape just for the DSL boundary.
    impl dsl::DslField for PartNumberRule {
        fn shape() -> dsl::Shape {
            dsl::Shape::Value
        }
        fn to_value(&self) -> dsl::FieldValue {
            dsl::FieldValue::Value(dsl::to_dsl_value(self).expect("PartNumberRule always serializes to DslValue"))
        }
        fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
            match value {
                dsl::FieldValue::Value(dsl_value) => dsl::from_dsl_value(dsl_value.clone()),
                other => Err(format!("expected Value, found {other:?}")),
            }
        }
    }

    /// 📄️ External media reference.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ExternalMedia {
        pub id: String,
        pub uri: String,
        pub media_type: String,
        pub checksum: Option<String>,
        pub language: Option<String>,
    }

    /// 🏛️ Minimal IFC catalogue node.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct IfcCatalogueNode {
        pub entity_type: String,
        pub global_id: String,
        pub name: String,
        pub attributes: std::collections::HashMap<String, String>,
        pub children: Vec<IfcCatalogueNode>,
    }

    /// 📦️ IFC catalogue root.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct IfcCatalogue {
        pub schema: String,
        pub metadata: IfcCatalogueNode,
        pub product_classes: Vec<IfcCatalogueNode>,
        pub products: Vec<IfcCatalogueNode>,
        pub unknown_entities: Vec<String>,
    }

    /// 🧮️ Script execution limits.
    #[derive(Clone, Copy, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ScriptLimits {
        pub max_steps: u32,
        pub max_recursion: u32,
        pub timeout_ms: u64,
    }

    impl Default for ScriptLimits {
        fn default() -> Self {
            Self { max_steps: 10_000, max_recursion: 64, timeout_ms: 50 }
        }
    }

    /// 📤️ Script execution result.
    #[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    #[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
    pub struct ScriptResult {
        pub value: f64,
        pub diagnostics: Vec<String>,
    }

    /// ⚠️ Script execution error.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum ScriptError {
        Timeout(u64),
        RecursionLimit(u32),
        StepLimit(u32),
        InvalidExpression(String),
        MissingInput(String),
    }

    impl std::fmt::Display for ScriptError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Timeout(milliseconds) => write!(formatter, "timeout after {milliseconds} ms"),
                Self::RecursionLimit(limit) => write!(formatter, "recursion limit {limit} exceeded"),
                Self::StepLimit(limit) => write!(formatter, "step limit {limit} exceeded"),
                Self::InvalidExpression(expression) => write!(formatter, "invalid expression: {expression}"),
                Self::MissingInput(input) => write!(formatter, "missing input: {input}"),
            }
        }
    }

    impl std::error::Error for ScriptError {}
}
// #endregion Part5
/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.

// #region Session
/// 🏷️ Canonical DSL file extension for ISO 16757 documents.
pub const ISO16757_EXTENSION: &str = "iso16757";

/// 📋️ ISO 16757 evaluation session document.
impl Iso16757Snapshot {
    pub fn reference_fixture() -> Self {
        let dictionary = part_4::Dictionary {
            reference: DictionaryRef { id: "hvac-dict".into(), version: "2025-01".into() },
            subjects: vec![part_4::Subject {
                id: "subject.valve".into(),
                kind: part_4::SubjectKind::ProductClass,
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "Control valve".into() },
                    short_name: Some("Valve".into()),
                    alternatives: vec![LocalizedText { locale: "de".into(), text: "Regelventil".into() }, LocalizedText { locale: "fr".into(), text: "Vanne de régulation".into() }],
                },
                definition: LocalizedText { locale: "en".into(), text: "Flow control valve".into() },
                parent_id: None,
            }],
            relationships: Vec::new(),
            properties: vec![part_4::DictionaryProperty {
                id: "prop.dn".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Nominal diameter".into() }, short_name: None, alternatives: Vec::new() },
                kind: part_1::PropertyKind::Static,
                data_type: "decimal".into(),
                unit: Some(CatalogueUnit { symbol: "mm".into(), dimension: DimensionSignature::LENGTH, si_factor: 0.001 }),
                applicable_subject_ids: vec!["subject.valve".into()],
                value_constraints: vec![part_4::ValueConstraint { min: Some(15.0), max: Some(300.0), allowed_values: Vec::new() }],
            }],
            controlled_lists: vec![part_4::ControlledValueList { id: "dn.list".into(), values: vec!["50".into(), "80".into(), "100".into()], context_subject_ids: vec!["subject.valve".into()] }],
            meta_subjects: Vec::new(),
        };
        let geometry_id: String = "geom.valve.50".into();
        let mut geometry_objects = BTreeMap::new();
        geometry_objects.insert(
            geometry_id.clone(),
            part_2::GeometryObject {
                id: geometry_id.clone(),
                shape: Some(part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 0.15), ("height".into(), 0.20), ("depth".into(), 0.10)]) }),
                symbolic: None,
                spaces: vec![part_2::SpaceEnvelope { kind: part_2::SpaceKind::Installation, bounds: part_2::BoundingBox::from_size(0.30, 0.30, 0.30) }],
                surfaces: Vec::new(),
                ports: vec![part_2::PortDefinition { id: "port.in".into(), medium: "water".into(), position: [0.0, 0.1, 0.05], direction: [1.0, 0.0, 0.0], port_type: "inlet".into() }],
                parameter_bindings: BTreeMap::from([("width".into(), "prop.dn".into())]),
            },
        );
        let catalogue = part_1::Catalogue {
            id: CatalogueId("cat.demo".into()),
            metadata: part_1::CatalogueMetadata {
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Demo HVAC catalogue".into() }, short_name: Some("Demo".into()), alternatives: Vec::new() },
                lifecycle: Lifecycle { revision: "1".into(), status: "published".into(), valid_from: None, valid_to: None },
                edition_profile: part_1::EditionProfile::FullPublished,
            },
            manufacturer: part_1::Manufacturer {
                id: "mfg.demo".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Demo Manufacturer".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Demo Hersteller".into() }] },
            },
            dictionary: dictionary.reference.clone(),
            product_groups: vec![part_1::ProductGroup {
                id: "group.valves".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Valves".into() }, short_name: None, alternatives: Vec::new() },
                dictionary_subject_id: Some("subject.valve".into()),
            }],
            product_classes: vec![part_1::ProductClass {
                id: "class.valve".into(),
                group_id: "group.valves".into(),
                parent_id: None,
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Control valve".into() }, short_name: None, alternatives: Vec::new() },
                required_property_ids: vec!["prop.dn".into()],
                optional_property_ids: Vec::new(),
            }],
            product_series: vec![part_1::ProductSeries {
                id: "series.cv".into(),
                class_id: "class.valve".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "CV series".into() }, short_name: None, alternatives: Vec::new() },
                shared_property_values: BTreeMap::new(),
                geometry_id: Some(geometry_id.clone()),
            }],
            products: vec![part_1::Product {
                id: "product.cv".into(),
                series_id: "series.cv".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "CV-50".into() }, short_name: None, alternatives: Vec::new() },
                parameter_domains: vec![part_1::ParameterDomain { parameter_id: "dn".into(), allowed_values: vec![CatalogueValue::Decimal { value: 50.0 }], default_value: Some(CatalogueValue::Decimal { value: 50.0 }) }],
                variants: vec![part_1::ProductVariant {
                    id: "variant.50".into(),
                    parameter_values: BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]),
                    property_values: vec![part_1::PropertyValue { definition_id: "prop.dn".into(), value: CatalogueValue::Decimal { value: 50.0 }, function_id: None }],
                    article_number: Some("CV-50".into()),
                    geometry_id: Some(geometry_id),
                }],
                static_properties: Vec::new(),
            }],
            product_indexes: vec![part_1::ProductIndex { id: "index.cv50".into(), product_id: "product.cv".into(), variant_id: Some("variant.50".into()), search_tags: vec!["valve".into(), "dn50".into()] }],
            property_definitions: vec![part_1::PropertyDefinition {
                id: "prop.dn".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Nominal diameter".into() }, short_name: None, alternatives: Vec::new() },
                data_type: "decimal".into(),
                unit: Some(CatalogueUnit { symbol: "mm".into(), dimension: DimensionSignature::LENGTH, si_factor: 0.001 }),
                cardinality: Cardinality::required(),
                kind: part_1::PropertyKind::Static,
                dictionary_property_id: Some("prop.dn".into()),
            }],
            accessories: BTreeMap::new(),
            compositions: BTreeMap::new(),
            descriptive_objects: Vec::new(),
            extensions: ExtensionBag::default(),
        };
        Self {
            catalogue,
            dictionary,
            geometry: part_2::GeometryCatalogue { objects: geometry_objects, primitive_registry: part_2::GeometryCatalogue::default_primitives() },
            selection: part_1::SelectionRequest {
                class_id: "class.valve".into(),
                constraints: vec![part_1::SelectionConstraint { property_id: "prop.dn".into(), operator: part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 50.0 } }],
                series_id: Some("series.cv".into()),
            },
            part_number_rule: part_5::PartNumberRule::Script { function_id: "partno".into(), source: "dn * 10 + 50".into() },
            part_number_inputs: BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]),
            script_limits: part_5::ScriptLimits::default(),
            exchange_process: part_5::ExchangeProcess::DetermineProduct,
        }
    }
}
// #endregion Session

//#region 🔖️ArtifactKind
/// 🗿️ The computed-compliance artifact this standard publishes on its app's `report:out` port —
/// lifted out of the pre-migration manifest's inline `.artifact_kind(ArtifactKindSpec { .. })` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("iso16757", "ISO 16757")
}
//#endregion 🔖️ArtifactKind

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const ISO16757_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.iso16757", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
pub const ISO16757_DOCUMENT_SCHEMA: &str = "semio.norm.iso16757/v1";

#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn dimension_signature_compatible_checks_equality() {
        assert!(DimensionSignature::LENGTH.compatible(DimensionSignature::LENGTH));
        assert!(!DimensionSignature::LENGTH.compatible(DimensionSignature::LENGTH_3));
        assert!(!DimensionSignature::DIMENSIONLESS.compatible(DimensionSignature::LENGTH));
    }

    #[semio_framework_async_macros::async_test]
    fn cardinality_variants_and_satisfies() {
        let optional = Cardinality::optional();
        assert!(optional.satisfies(0));
        assert!(optional.satisfies(1));
        assert!(!optional.satisfies(2));
        let required = Cardinality::required();
        assert!(!required.satisfies(0));
        assert!(required.satisfies(1));
        let unbounded = Cardinality::unbounded();
        assert!(unbounded.satisfies(0));
        assert!(unbounded.satisfies(1_000));
    }

    #[semio_framework_async_macros::async_test]
    fn geometry_catalogue_default_primitives() {
        let primitives = part_2::GeometryCatalogue::default_primitives();
        let ids: Vec<&str> = primitives.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, vec!["box", "cylinder", "sphere"]);
    }

    #[semio_framework_async_macros::async_test]
    fn bounding_box_overlaps() {
        let a = part_2::BoundingBox::from_size(1.0, 1.0, 1.0);
        let touching = part_2::BoundingBox { min: [0.9, 0.9, 0.9], max: [1.9, 1.9, 1.9] };
        assert!(a.overlaps(touching, 0.0));
        let far = part_2::BoundingBox { min: [5.0, 5.0, 5.0], max: [6.0, 6.0, 6.0] };
        assert!(!a.overlaps(far, 0.0));
        assert!(a.overlaps(far, 10.0));
    }

    #[semio_framework_async_macros::async_test]
    fn script_limits_default_values() {
        let limits = part_5::ScriptLimits::default();
        assert_eq!(limits.max_steps, 10_000);
        assert_eq!(limits.max_recursion, 64);
        assert_eq!(limits.timeout_ms, 50);
    }
}

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`/`register_pilot_languages()`/`register_artifact_schema()`/
/// `register_artifact_inferences()`/`register_io()`, each of which called a global registry directly
/// from the plugin root's `.setup()` fan-out (`register_norm_exports`, deleted by this same wave).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use crate::artifacts::definition::{CapabilitySpec as C, ClaimSpec as Q, LocalizationSpec as L};
    const S: &[Q] = &[Q { namespace: "schema", value: "s.norm.iso16757" }];
    const I: &[Q] = &[Q { namespace: "schema", value: "s.norm.iso16757.inference" }];
    const M: &[Q] = &[Q { namespace: "dialect", value: "s.iso16757@1/*" }];
    const K: &[Q] = &[Q { namespace: "codec", value: "semio.norm.iso16757/v1" }, Q { namespace: "extension", value: "iso16757" }];
    const EN: &[L] = &[L { locale: "en", text: "ISO 16757 building-services product catalogue exchange" }];
    const DE: &[L] = &[L { locale: "de", text: "ISO 16757 Austausch von Produktdaten der technischen Gebäudeausrüstung" }];
    const ROWS: &[C] = &[
        C { identity: "s.iso16757.standard.v1", kind: "standard", descriptor: "v1", claims: &[], localizations: &[] },
        C { identity: "s.iso16757.standard.v1.profile.any", kind: "profile", descriptor: "any", claims: &[], localizations: &[] },
        C { identity: "s.iso16757.schema.artifact", kind: "schema", descriptor: "s.norm.iso16757", claims: S, localizations: &[] },
        C { identity: "s.iso16757.inference.outline", kind: "inference", descriptor: "s.norm.iso16757.inference", claims: I, localizations: &[] },
        C { identity: "s.iso16757.composer.any", kind: "composer", descriptor: "s.iso16757@1/*", claims: M, localizations: &[] },
        C { identity: "s.iso16757.grammar.document", kind: "grammar", descriptor: "iso16757.document", claims: &[Q { namespace: "grammar", value: "iso16757.document" }], localizations: &[] },
        C { identity: "s.iso16757.grammar.op", kind: "grammar", descriptor: "iso16757.op", claims: &[Q { namespace: "grammar", value: "iso16757.op" }], localizations: &[] },
        C { identity: "s.iso16757.grammar.diff", kind: "grammar", descriptor: "iso16757.diff", claims: &[Q { namespace: "grammar", value: "iso16757.diff" }], localizations: &[] },
        C { identity: "s.iso16757.grammar.pack", kind: "grammar", descriptor: "iso16757.pack", claims: &[Q { namespace: "grammar", value: "iso16757.pack" }], localizations: &[] },
        C { identity: "s.iso16757.grammar.spr", kind: "grammar", descriptor: "iso16757.spr", claims: &[Q { namespace: "grammar", value: "iso16757.spr" }], localizations: &[] },
        C { identity: "s.iso16757.codec.document.v1", kind: "codec", descriptor: "semio.norm.iso16757/v1:iso16757", claims: K, localizations: &[] },
        C { identity: "s.iso16757.localization.en", kind: "localization", descriptor: "ISO 16757 building-services product catalogue exchange", claims: &[], localizations: EN },
        C { identity: "s.iso16757.localization.de", kind: "localization", descriptor: "ISO 16757 Austausch von Produktdaten der technischen Gebäudeausrüstung", claims: &[], localizations: DE },
    ];
    crate::artifacts::definition::assemble_definition("s.iso16757", ROWS)
}

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(crate::artifacts::iso16757::schema::iso16757_artifact_schema_descriptor())
        .inferences([crate::artifacts::iso16757::standards::v1::subsets::any::schema::inferences::iso16757_artifact_inference_descriptor()])
        .composers(crate::artifacts::iso16757::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::iso16757::Iso16757PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "iso16757.document",
                    extension: Some("iso16757"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::en1999::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1999::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1999::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1999::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("iso16757.document"),
                },
                dsl::LanguageSpec {
                    id: "iso16757.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::en1999::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1999::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::en1999::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1999::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("iso16757.op"),
                },
                dsl::LanguageSpec {
                    id: "iso16757.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::en1999::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::en1999::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("iso16757.diff"),
                },
                dsl::LanguageSpec {
                    id: "iso16757.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1999::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1999::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("iso16757.pack"),
                },
                dsl::LanguageSpec {
                    id: "iso16757.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::en1999::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::en1999::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("iso16757.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
