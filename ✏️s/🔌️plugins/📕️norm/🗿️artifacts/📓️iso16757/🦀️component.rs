//! 📦️ ISO 16757 building-services product catalogue: parts 1, 2, 4, 5 — document entities.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// #region Shared
/// 🆔️ Stable catalogue identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, dsl::DslRecord)]
pub struct DictionaryRef {
    pub id: String,
    pub version: String,
}

/// 🌐️ Locale-tagged text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct LocalizedText {
    pub locale: String,
    pub text: String,
}

/// 📝️ Preferred and alternative names.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct Names {
    pub preferred: LocalizedText,
    pub short_name: Option<String>,
    #[dsl(table)]
    pub alternatives: Vec<LocalizedText>,
}

/// 📊️ Physical dimension signature for unit compatibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct CatalogueUnit {
    pub symbol: String,
    pub dimension: DimensionSignature,
    pub si_factor: f64,
}

/// 🔢️ Typed catalogue value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum NullState {
    Unavailable,
    Unknown,
    /// 🔡️ `NotApplicable` auto-kebabs to `not-applicable`; kept camelCase to match this crate's
    /// ISO-16757-native external property naming convention (see `SubjectKind`/`RelationshipKind`).
    #[dsl(key = "notApplicable")]
    NotApplicable,
}

/// 🔢️ Cardinality constraint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogueReference {
    pub uri: String,
    pub label: Option<String>,
}

/// 🧩️ Lossless extension bag for unknown fields.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ExtensionBag {
    pub fields: BTreeMap<String, dsl::DslValue>,
}

/// 📅️ Lifecycle metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct Manufacturer {
        pub id: String,
        pub names: Names,
    }

    /// 📦️ Product group declaration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ProductGroup {
        pub id: String,
        pub names: Names,
        pub dictionary_subject_id: Option<String>,
    }

    /// 🏷️ Product class in a hierarchy.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ProductClass {
        pub id: String,
        pub group_id: String,
        pub parent_id: Option<String>,
        pub names: Names,
        pub required_property_ids: Vec<String>,
        pub optional_property_ids: Vec<String>,
    }

    /// 📚️ Product series sharing geometry and properties.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ProductSeries {
        pub id: String,
        pub class_id: String,
        pub names: Names,
        pub shared_property_values: BTreeMap<String, CatalogueValue>,
        pub geometry_id: Option<String>,
    }

    /// 🔧️ Variant parameter domain.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ParameterDomain {
        pub parameter_id: String,
        pub allowed_values: Vec<CatalogueValue>,
        pub default_value: Option<CatalogueValue>,
    }

    /// 🧮️ Property definition.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum PropertyKind {
        Static,
        Dynamic,
        Selection,
        External,
    }

    /// 📋️ Property value on a product or variant.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct PropertyValue {
        pub definition_id: String,
        pub value: CatalogueValue,
        pub function_id: Option<String>,
    }

    /// 🧩️ Product variant with parameters.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ProductVariant {
        pub id: String,
        pub parameter_values: BTreeMap<String, CatalogueValue>,
        pub property_values: Vec<PropertyValue>,
        pub article_number: Option<String>,
        pub geometry_id: Option<String>,
    }

    /// 📦️ Catalogue product (generic or resolved).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct Product {
        pub id: String,
        pub series_id: String,
        pub names: Names,
        pub parameter_domains: Vec<ParameterDomain>,
        pub variants: Vec<ProductVariant>,
        pub static_properties: Vec<PropertyValue>,
    }

    /// 🔍️ Product index for selection.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ProductIndex {
        pub id: String,
        pub product_id: String,
        pub variant_id: Option<String>,
        pub search_tags: Vec<String>,
    }

    /// 🔗️ Accessory relationship.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct AccessoryRelationship {
        pub accessory_product_id: String,
        pub required: bool,
        pub quantity: Cardinality,
        pub compatibility_condition: Option<String>,
    }

    /// 🧱️ Composition relationship (`hasPart`).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct CompositionRelationship {
        pub component_product_id: String,
        pub quantity: u32,
    }

    /// 🖼️ Geometry reference.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct GeometryReference {
        pub geometry_id: String,
        pub lod: Option<String>,
    }

    /// 📄️ Descriptive media object.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct DescriptiveObject {
        pub id: String,
        pub media_type: String,
        pub uri: String,
        pub language: Option<String>,
        pub checksum: Option<String>,
    }

    /// 📚️ Full catalogue document.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct CatalogueMetadata {
        pub names: Names,
        pub lifecycle: Lifecycle,
        pub edition_profile: EditionProfile,
    }

    /// 📑️ Supported ISO 16757 edition profile.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct SelectionConstraint {
        pub property_id: String,
        pub operator: ConstraintOperator,
        pub value: CatalogueValue,
    }

    /// ⚖️ Constraint operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct SelectionRequest {
        pub class_id: String,
        pub constraints: Vec<SelectionConstraint>,
        pub series_id: Option<String>,
    }

    /// ✅️ Selection outcome.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SelectionResult {
        pub matches: Vec<ProductIndex>,
        pub ambiguity: bool,
        pub explanations: Vec<String>,
    }

    /// 🏗️ BIM embedding workflow state.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum SpaceKind {
        Overall,
        Operation,
        Access,
        #[dsl(key = "placementTransportation")]
        PlacementTransportation,
        Installation,
    }

    /// 🔌️ Port medium and direction.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct PortDefinition {
        pub id: String,
        pub medium: String,
        pub position: [f64; 3],
        pub direction: [f64; 3],
        pub port_type: String,
    }

    /// 📦️ Axis-aligned bounding box.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct PrimitiveKind {
        pub id: String,
        pub parameters: Vec<String>,
    }

    /// 🌳️ Geometry node in a CSG tree — recursive `#[derive(dsl::DslEnum)]`: `Transform.child` is
    /// `#[dsl(statements)] Box<GeometryNode>` (exactly one nested tagged value) and
    /// `Boolean.children` is `#[dsl(statements, block)] Vec<GeometryNode>` (a nested tagged
    /// collection), both recursing back into this same enum's own `DslVariants` impl.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
    #[serde(tag = "node", rename_all = "camelCase")]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum BooleanOperator {
        Union,
        Intersection,
        Difference,
    }

    /// 🏗️ Complete geometry object.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct SpaceEnvelope {
        pub kind: SpaceKind,
        pub bounds: BoundingBox,
    }

    /// 🎨️ Semantic surface.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct SurfaceDefinition {
        pub id: String,
        pub purpose: String,
        pub bounds: BoundingBox,
    }

    /// 📚️ Geometry catalogue index.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, dsl::DslScalar)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct Subject {
        pub id: String,
        pub kind: SubjectKind,
        pub names: Names,
        pub definition: LocalizedText,
        pub parent_id: Option<String>,
    }

    /// 🔗️ Relationship kind per Part 4 §4.4.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct Relationship {
        pub id: String,
        pub kind: RelationshipKind,
        pub source_id: String,
        pub target_id: String,
        pub cardinality: Cardinality,
    }

    /// 📊️ Dictionary property definition.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ControlledValueList {
        pub id: String,
        pub values: Vec<String>,
        pub context_subject_ids: Vec<String>,
    }

    /// 🎯️ Value constraint on a property.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct ValueConstraint {
        pub min: Option<f64>,
        pub max: Option<f64>,
        pub allowed_values: Vec<String>,
    }

    /// 📚️ Data dictionary snapshot.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ExternalMedia {
        pub id: String,
        pub uri: String,
        pub media_type: String,
        pub checksum: Option<String>,
        pub language: Option<String>,
    }

    /// 🏛️ Minimal IFC catalogue node.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IfcCatalogueNode {
        pub entity_type: String,
        pub global_id: String,
        pub name: String,
        pub attributes: std::collections::HashMap<String, String>,
        pub children: Vec<IfcCatalogueNode>,
    }

    /// 📦️ IFC catalogue root.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IfcCatalogue {
        pub schema: String,
        pub metadata: IfcCatalogueNode,
        pub product_classes: Vec<IfcCatalogueNode>,
        pub products: Vec<IfcCatalogueNode>,
        pub unknown_entities: Vec<String>,
    }

    /// 🧮️ Script execution limits.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ScriptResult {
        pub value: f64,
        pub diagnostics: Vec<String>,
    }

    /// ⚠️ Script execution error.
    #[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
    pub enum ScriptError {
        #[error("timeout after {0} ms")]
        Timeout(u64),
        #[error("recursion limit {0} exceeded")]
        RecursionLimit(u32),
        #[error("step limit {0} exceeded")]
        StepLimit(u32),
        #[error("invalid expression: {0}")]
        InvalidExpression(String),
        #[error("missing input: {0}")]
        MissingInput(String),
    }
}
// #endregion Part5

// #region Session
/// 🏷️ Canonical DSL file extension for ISO 16757 documents.
pub const ISO16757_EXTENSION: &str = "iso16757";

/// 📋️ ISO 16757 evaluation session document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.iso16757", layout = "lines")]
pub struct Document {
    pub catalogue: part_1::Catalogue,
    pub dictionary: part_4::Dictionary,
    pub geometry: part_2::GeometryCatalogue,
    pub selection: part_1::SelectionRequest,
    pub part_number_rule: part_5::PartNumberRule,
    pub part_number_inputs: BTreeMap<String, CatalogueValue>,
    pub script_limits: part_5::ScriptLimits,
    pub exchange_process: part_5::ExchangeProcess,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for Document {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for Document {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️DocumentCodec


impl Default for Document {
    fn default() -> Self {
        Self::reference_fixture()
    }
}

impl Document {
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
    crate::core::app::artifact_kind_spec("iso16757", "ISO 16757")
}
//#endregion 🔖️ArtifactKind

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimension_signature_compatible_checks_equality() {
        assert!(DimensionSignature::LENGTH.compatible(DimensionSignature::LENGTH));
        assert!(!DimensionSignature::LENGTH.compatible(DimensionSignature::LENGTH_3));
        assert!(!DimensionSignature::DIMENSIONLESS.compatible(DimensionSignature::LENGTH));
    }

    #[test]
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

    #[test]
    fn geometry_catalogue_default_primitives() {
        let primitives = part_2::GeometryCatalogue::default_primitives();
        let ids: Vec<&str> = primitives.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, vec!["box", "cylinder", "sphere"]);
    }

    #[test]
    fn bounding_box_overlaps() {
        let a = part_2::BoundingBox::from_size(1.0, 1.0, 1.0);
        let touching = part_2::BoundingBox { min: [0.9, 0.9, 0.9], max: [1.9, 1.9, 1.9] };
        assert!(a.overlaps(touching, 0.0));
        let far = part_2::BoundingBox { min: [5.0, 5.0, 5.0], max: [6.0, 6.0, 6.0] };
        assert!(!a.overlaps(far, 0.0));
        assert!(a.overlaps(far, 10.0));
    }

    #[test]
    fn script_limits_default_values() {
        let limits = part_5::ScriptLimits::default();
        assert_eq!(limits.max_steps, 10_000);
        assert_eq!(limits.max_recursion, 64);
        assert_eq!(limits.timeout_ms, 50);
    }
}

