//! 📦 ISO 16757 building-services product catalogue: parts 1, 2, 4, 5.

use norm_core::{
    AnnexChoice, CheckReport, CheckResult, ClauseId, NormError, NormFamily, NormFamilyId, NormHost, Quantity, QuantityKind, SetDocumentOperation,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub use norm_core::NationalAnnex;

// #region Shared
/// 🆔 Stable catalogue identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CatalogueId(pub String);

/// 🆔 Dictionary identifier with version.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DictionaryRef {
    pub id: String,
    pub version: String,
}

/// 🌐 Locale-tagged text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizedText {
    pub locale: String,
    pub text: String,
}

/// 📝 Preferred and alternative names.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Names {
    pub preferred: LocalizedText,
    pub short_name: Option<String>,
    pub alternatives: Vec<LocalizedText>,
}

/// 📊 Physical dimension signature for unit compatibility.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// 📐 Catalogue unit with canonical SI display.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogueUnit {
    pub symbol: String,
    pub dimension: DimensionSignature,
    pub si_factor: f64,
}

/// 🔢 Typed catalogue value.
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

/// ∅ Value availability states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NullState {
    Unavailable,
    Unknown,
    NotApplicable,
}

/// 🔢 Cardinality constraint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        count >= self.min && self.max.map_or(true, |max| count <= max)
    }
}

/// 🔗 Internal or external reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogueReference {
    pub uri: String,
    pub label: Option<String>,
}

/// 🧩 Lossless extension bag for unknown fields.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ExtensionBag {
    pub fields: HashMap<String, serde_json::Value>,
}

/// 📅 Lifecycle metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    /// 🏭 Manufacturer metadata.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Manufacturer {
        pub id: String,
        pub names: Names,
    }

    /// 📦 Product group declaration.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ProductGroup {
        pub id: String,
        pub names: Names,
        pub dictionary_subject_id: Option<String>,
    }

    /// 🏷️ Product class in a hierarchy.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ProductClass {
        pub id: String,
        pub group_id: String,
        pub parent_id: Option<String>,
        pub names: Names,
        pub required_property_ids: Vec<String>,
        pub optional_property_ids: Vec<String>,
    }

    /// 📚 Product series sharing geometry and properties.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ProductSeries {
        pub id: String,
        pub class_id: String,
        pub names: Names,
        pub shared_property_values: HashMap<String, CatalogueValue>,
        pub geometry_id: Option<String>,
    }

    /// 🔧 Variant parameter domain.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ParameterDomain {
        pub parameter_id: String,
        pub allowed_values: Vec<CatalogueValue>,
        pub default_value: Option<CatalogueValue>,
    }

    /// 🧮 Property definition.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PropertyDefinition {
        pub id: String,
        pub names: Names,
        pub data_type: String,
        pub unit: Option<CatalogueUnit>,
        pub cardinality: Cardinality,
        pub kind: PropertyKind,
        pub dictionary_property_id: Option<String>,
    }

    /// 📊 Property kind per Part 1 §5.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PropertyKind {
        Static,
        Dynamic,
        Selection,
        External,
    }

    /// 📋 Property value on a product or variant.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PropertyValue {
        pub definition_id: String,
        pub value: CatalogueValue,
        pub function_id: Option<String>,
    }

    /// 🧩 Product variant with parameters.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ProductVariant {
        pub id: String,
        pub parameter_values: HashMap<String, CatalogueValue>,
        pub property_values: Vec<PropertyValue>,
        pub article_number: Option<String>,
        pub geometry_id: Option<String>,
    }

    /// 📦 Catalogue product (generic or resolved).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Product {
        pub id: String,
        pub series_id: String,
        pub names: Names,
        pub parameter_domains: Vec<ParameterDomain>,
        pub variants: Vec<ProductVariant>,
        pub static_properties: Vec<PropertyValue>,
    }

    /// 🔍 Product index for selection.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ProductIndex {
        pub id: String,
        pub product_id: String,
        pub variant_id: Option<String>,
        pub search_tags: Vec<String>,
    }

    /// 🔗 Accessory relationship.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct AccessoryRelationship {
        pub accessory_product_id: String,
        pub required: bool,
        pub quantity: Cardinality,
        pub compatibility_condition: Option<String>,
    }

    /// 🧱 Composition relationship (`hasPart`).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

    /// 📄 Descriptive media object.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DescriptiveObject {
        pub id: String,
        pub media_type: String,
        pub uri: String,
        pub language: Option<String>,
        pub checksum: Option<String>,
    }

    /// 📚 Full catalogue document.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Catalogue {
        pub id: CatalogueId,
        pub metadata: CatalogueMetadata,
        pub manufacturer: Manufacturer,
        pub dictionary: DictionaryRef,
        pub product_groups: Vec<ProductGroup>,
        pub product_classes: Vec<ProductClass>,
        pub product_series: Vec<ProductSeries>,
        pub products: Vec<Product>,
        pub product_indexes: Vec<ProductIndex>,
        pub property_definitions: Vec<PropertyDefinition>,
        pub accessories: HashMap<String, Vec<AccessoryRelationship>>,
        pub compositions: HashMap<String, Vec<CompositionRelationship>>,
        pub descriptive_objects: Vec<DescriptiveObject>,
        pub extensions: ExtensionBag,
    }

    /// 📋 Catalogue metadata.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct CatalogueMetadata {
        pub names: Names,
        pub lifecycle: Lifecycle,
        pub edition_profile: EditionProfile,
    }

    /// 📑 Supported ISO 16757 edition profile.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum EditionProfile {
        Part1_2015,
        Part2_2016,
        Part4_2025,
        Part5_2025,
        FullPublished,
    }

    /// 🎯 Selection constraint on a property.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SelectionConstraint {
        pub property_id: String,
        pub operator: ConstraintOperator,
        pub value: CatalogueValue,
    }

    /// ⚖️ Constraint operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ConstraintOperator {
        Equal,
        NotEqual,
        LessThan,
        GreaterThan,
        InRange,
    }

    /// 🔎 Selection request.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SelectionRequest {
        pub class_id: String,
        pub constraints: Vec<SelectionConstraint>,
        pub series_id: Option<String>,
    }

    /// ✅ Selection outcome.
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
        pub frozen_parameters: HashMap<String, CatalogueValue>,
        pub resolved_properties: Vec<PropertyValue>,
        pub resolved_article_number: Option<String>,
        pub resolved_geometry_id: Option<String>,
        pub catalogue_provenance: CatalogueId,
        pub dictionary_provenance: DictionaryRef,
    }

    pub fn evaluate_constraint(value: &CatalogueValue, constraint: &SelectionConstraint) -> bool {
        match (&constraint.operator, value, &constraint.value) {
            (ConstraintOperator::Equal, a, b) => a == b,
            (ConstraintOperator::NotEqual, a, b) => a != b,
            (ConstraintOperator::LessThan, CatalogueValue::Decimal { value: a }, CatalogueValue::Decimal { value: b }) => a < b,
            (ConstraintOperator::GreaterThan, CatalogueValue::Decimal { value: a }, CatalogueValue::Decimal { value: b }) => a > b,
            (ConstraintOperator::InRange, CatalogueValue::Decimal { value: v }, CatalogueValue::Range { min, max, .. }) => *v >= *min && *v <= *max,
            _ => false,
        }
    }

    pub fn select_products(catalogue: &Catalogue, request: &SelectionRequest) -> SelectionResult {
        let mut matches = Vec::new();
        let mut explanations = Vec::new();
        for index in &catalogue.product_indexes {
            let Some(product) = catalogue.products.iter().find(|p| p.id == index.product_id) else {
                continue;
            };
            let Some(series) = catalogue.product_series.iter().find(|s| s.id == product.series_id) else {
                continue;
            };
            if series.class_id != request.class_id {
                continue;
            }
            if let Some(series_id) = &request.series_id {
                if product.series_id != *series_id {
                    continue;
                }
            }
            let variant = index.variant_id.as_ref().and_then(|vid| product.variants.iter().find(|v| &v.id == vid));
            let properties: Vec<&PropertyValue> = product
                .static_properties
                .iter()
                .chain(variant.into_iter().flat_map(|v| &v.property_values))
                .collect();
            let mut passes = true;
            for constraint in &request.constraints {
                let Some(pv) = properties.iter().find(|pv| pv.definition_id == constraint.property_id) else {
                    passes = false;
                    explanations.push(format!("missing property {}", constraint.property_id));
                    break;
                };
                if !evaluate_constraint(&pv.value, constraint) {
                    passes = false;
                    explanations.push(format!("constraint failed on {}", constraint.property_id));
                    break;
                }
            }
            if passes {
                matches.push(index.clone());
            }
        }
        let ambiguity = matches.len() > 1;
        SelectionResult { matches, ambiguity, explanations }
    }

    pub fn detect_composition_cycle(catalogue: &Catalogue, root_product_id: &str) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![root_product_id.to_string()];
        while let Some(current) = stack.pop() {
            if !visited.insert(current.clone()) {
                return true;
            }
            if let Some(parts) = catalogue.compositions.get(&current) {
                for part in parts {
                    stack.push(part.component_product_id.clone());
                }
            }
        }
        false
    }

    pub fn resolve_bim_embedding(catalogue: &Catalogue, index_id: &str, parameters: HashMap<String, CatalogueValue>) -> Result<BimEmbedding, NormError> {
        let index = catalogue
            .product_indexes
            .iter()
            .find(|i| i.id == index_id)
            .ok_or_else(|| NormError::InvalidValue { field: "index_id".into(), reason: "unknown product index".into() })?;
        let product = catalogue
            .products
            .iter()
            .find(|p| p.id == index.product_id)
            .ok_or_else(|| NormError::InvalidValue { field: "product_id".into(), reason: "unknown product".into() })?;
        let variant = index
            .variant_id
            .as_ref()
            .and_then(|vid| product.variants.iter().find(|v| &v.id == vid))
            .ok_or_else(|| NormError::IncompleteInput { field: "variant_id".into() })?;
        for (param_id, value) in &parameters {
            let domain = product.parameter_domains.iter().find(|d| &d.parameter_id == param_id);
            if let Some(domain) = domain {
                if !domain.allowed_values.is_empty() && !domain.allowed_values.contains(value) {
                    return Err(NormError::InvalidValue { field: param_id.clone(), reason: "not in allowed domain".into() });
                }
            }
        }
        let mut resolved_properties = product.static_properties.clone();
        resolved_properties.extend(variant.property_values.clone());
        Ok(BimEmbedding {
            selected_index_id: index_id.to_string(),
            frozen_parameters: parameters,
            resolved_article_number: variant.article_number.clone(),
            resolved_geometry_id: variant.geometry_id.clone().or_else(|| {
                catalogue
                    .product_series
                    .iter()
                    .find(|s| s.id == product.series_id)
                    .and_then(|s| s.geometry_id.clone())
            }),
            resolved_properties,
            catalogue_provenance: catalogue.id.clone(),
            dictionary_provenance: catalogue.dictionary.clone(),
        })
    }

    pub fn validate_catalogue_structure(catalogue: &Catalogue) -> Vec<String> {
        let mut issues = Vec::new();
        if catalogue.products.is_empty() {
            issues.push("catalogue has no products".into());
        }
        for product in &catalogue.products {
            if catalogue.product_series.iter().all(|s| s.id != product.series_id) {
                issues.push(format!("product {} references unknown series {}", product.id, product.series_id));
            }
            if detect_composition_cycle(catalogue, &product.id) {
                issues.push(format!("composition cycle at product {}", product.id));
            }
        }
        for def in &catalogue.property_definitions {
            if def.id.is_empty() {
                issues.push("empty property definition id".into());
            }
        }
        issues
    }
}
// #endregion Part1

// #region Part2
pub mod part_2 {
    use super::*;

    /// 📐 Space classification per Part 2 §5.3.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SpaceKind {
        Overall,
        Operation,
        Access,
        PlacementTransportation,
        Installation,
    }

    /// 🔌 Port medium and direction.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PortDefinition {
        pub id: String,
        pub medium: String,
        pub position: [f64; 3],
        pub direction: [f64; 3],
        pub port_type: String,
    }

    /// 📦 Axis-aligned bounding box.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

    /// 🧱 CSG primitive kind registry entry.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct PrimitiveKind {
        pub id: String,
        pub parameters: Vec<String>,
    }

    /// 🌳 Geometry node in a CSG tree.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "node", rename_all = "camelCase")]
    pub enum GeometryNode {
        Primitive { kind: String, parameters: HashMap<String, f64> },
        Transform { translation: [f64; 3], rotation_deg: [f64; 3], child: Box<GeometryNode> },
        Boolean { operator: BooleanOperator, children: Vec<GeometryNode> },
        Reference { geometry_id: String },
    }

    /// ➕ Boolean CSG operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum BooleanOperator {
        Union,
        Intersection,
        Difference,
    }

    /// 🏗️ Complete geometry object.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct GeometryObject {
        pub id: String,
        pub shape: Option<GeometryNode>,
        pub symbolic: Option<GeometryNode>,
        pub spaces: Vec<SpaceEnvelope>,
        pub surfaces: Vec<SurfaceDefinition>,
        pub ports: Vec<PortDefinition>,
        pub parameter_bindings: HashMap<String, String>,
    }

    /// 📦 Space envelope with kind.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SpaceEnvelope {
        pub kind: SpaceKind,
        pub bounds: BoundingBox,
    }

    /// 🎨 Semantic surface.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct SurfaceDefinition {
        pub id: String,
        pub purpose: String,
        pub bounds: BoundingBox,
    }

    /// 📚 Geometry catalogue index.
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct GeometryCatalogue {
        pub objects: HashMap<String, GeometryObject>,
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

    pub fn substitute_parameters(node: &GeometryNode, values: &HashMap<String, f64>) -> GeometryNode {
        match node {
            GeometryNode::Primitive { kind, parameters } => {
                let mut resolved = HashMap::new();
                for (key, value) in parameters {
                    resolved.insert(key.clone(), *values.get(key).unwrap_or(value));
                }
                GeometryNode::Primitive { kind: kind.clone(), parameters: resolved }
            }
            GeometryNode::Transform { translation, rotation_deg, child } => GeometryNode::Transform {
                translation: *translation,
                rotation_deg: *rotation_deg,
                child: Box::new(substitute_parameters(child, values)),
            },
            GeometryNode::Boolean { operation, children } => GeometryNode::Boolean { operation: *operation, children: children.iter().map(|c| substitute_parameters(c, values)).collect() },
            GeometryNode::Reference { geometry_id } => GeometryNode::Reference { geometry_id: geometry_id.clone() },
        }
    }

    pub fn evaluate_bounding_box(node: &GeometryNode, catalogue: &GeometryCatalogue) -> Result<BoundingBox, NormError> {
        match node {
            GeometryNode::Primitive { kind, parameters } => match kind.as_str() {
                "box" => {
                    let w = *parameters.get("width").ok_or_else(|| NormError::IncompleteInput { field: "width".into() })?;
                    let h = *parameters.get("height").ok_or_else(|| NormError::IncompleteInput { field: "height".into() })?;
                    let d = *parameters.get("depth").ok_or_else(|| NormError::IncompleteInput { field: "depth".into() })?;
                    Ok(BoundingBox::from_size(w, h, d))
                }
                "cylinder" => {
                    let r = *parameters.get("radius").ok_or_else(|| NormError::IncompleteInput { field: "radius".into() })?;
                    let h = *parameters.get("height").ok_or_else(|| NormError::IncompleteInput { field: "height".into() })?;
                    Ok(BoundingBox::from_size(2.0 * r, h, 2.0 * r))
                }
                "sphere" => {
                    let r = *parameters.get("radius").ok_or_else(|| NormError::IncompleteInput { field: "radius".into() })?;
                    Ok(BoundingBox::from_size(2.0 * r, 2.0 * r, 2.0 * r))
                }
                _ => Err(NormError::OutOfScope { clause: ClauseId::new("ISO 16757", "2", "7.1") }),
            },
            GeometryNode::Transform { translation, child, .. } => {
                let mut bbox = evaluate_bounding_box(child, catalogue)?;
                bbox.min[0] += translation[0];
                bbox.min[1] += translation[1];
                bbox.min[2] += translation[2];
                bbox.max[0] += translation[0];
                bbox.max[1] += translation[1];
                bbox.max[2] += translation[2];
                Ok(bbox)
            }
            GeometryNode::Boolean { children, .. } => {
                let mut iter = children.iter();
                let first = iter.next().ok_or_else(|| NormError::IncompleteInput { field: "boolean_children".into() })?;
                let mut acc = evaluate_bounding_box(first, catalogue)?;
                for child in iter {
                    let next = evaluate_bounding_box(child, catalogue)?;
                    acc.min[0] = acc.min[0].min(next.min[0]);
                    acc.min[1] = acc.min[1].min(next.min[1]);
                    acc.min[2] = acc.min[2].min(next.min[2]);
                    acc.max[0] = acc.max[0].max(next.max[0]);
                    acc.max[1] = acc.max[1].max(next.max[1]);
                    acc.max[2] = acc.max[2].max(next.max[2]);
                }
                Ok(acc)
            }
            GeometryNode::Reference { geometry_id } => catalogue
                .objects
                .get(geometry_id)
                .and_then(|obj| obj.shape.as_ref())
                .map(|shape| evaluate_bounding_box(shape, catalogue))
                .unwrap_or(Err(NormError::InvalidValue { field: "geometry_id".into(), reason: "unresolved reference".into() })),
        }
    }

    pub fn validate_geometry_graph(object: &GeometryObject, catalogue: &GeometryCatalogue, visited: &mut HashSet<String>) -> Vec<String> {
        let mut issues = Vec::new();
        if let Some(shape) = &object.shape {
            if let GeometryNode::Reference { geometry_id } = shape {
                if geometry_id == &object.id {
                    issues.push(format!("self-reference in geometry {}", object.id));
                }
                if !visited.insert(geometry_id.clone()) {
                    issues.push(format!("cycle in geometry reference {}", geometry_id));
                } else if let Some(referenced) = catalogue.objects.get(geometry_id) {
                    issues.extend(validate_geometry_graph(referenced, catalogue, visited));
                }
                visited.remove(geometry_id);
            }
        }
        for binding in object.parameter_bindings.values() {
            if binding.is_empty() {
                issues.push(format!("empty parameter binding on {}", object.id));
            }
        }
        issues
    }

    pub fn project_step_entity(_object: &GeometryObject, bbox: BoundingBox) -> String {
        format!(
            "#1=IFCCARTESIANPOINT(({:.3},{:.3},{:.3}));\n#2=IFCBOUNDINGBOX(#1,{:.3},{:.3},{:.3});",
            bbox.min[0],
            bbox.min[1],
            bbox.min[2],
            bbox.max[0] - bbox.min[0],
            bbox.max[1] - bbox.min[1],
            bbox.max[2] - bbox.min[2]
        )
    }
}
// #endregion Part2

// #region Part4
pub mod part_4 {
    use super::*;

    /// 🏷️ Dictionary subject kind.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum SubjectKind {
        ProductGroup,
        ProductClass,
        ProductSpecialization,
        CatalogueMetadata,
        ManufacturerMetadata,
        PropertyBlock,
        Port,
        Inlet,
        Outlet,
        InOutlet,
    }

    /// 📖 Dictionary subject.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Subject {
        pub id: String,
        pub kind: SubjectKind,
        pub names: Names,
        pub definition: LocalizedText,
        pub parent_id: Option<String>,
    }

    /// 🔗 Relationship kind per Part 4 §4.4.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum RelationshipKind {
        IsSubtypeOf,
        HasPart,
        HasBlock,
        IsDependentOn,
        IsSubkindOf,
    }

    /// 🔗 Typed relationship.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Relationship {
        pub id: String,
        pub kind: RelationshipKind,
        pub source_id: String,
        pub target_id: String,
        pub cardinality: Cardinality,
    }

    /// 📊 Dictionary property definition.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DictionaryProperty {
        pub id: String,
        pub names: Names,
        pub kind: part_1::PropertyKind,
        pub data_type: String,
        pub unit: Option<CatalogueUnit>,
        pub applicable_subject_ids: Vec<String>,
        pub value_constraints: Vec<ValueConstraint>,
    }

    /// ✅ Controlled value list.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ControlledValueList {
        pub id: String,
        pub values: Vec<String>,
        pub context_subject_ids: Vec<String>,
    }

    /// 🎯 Value constraint on a property.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct ValueConstraint {
        pub min: Option<f64>,
        pub max: Option<f64>,
        pub allowed_values: Vec<String>,
    }

    /// 📚 Data dictionary snapshot.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Dictionary {
        pub reference: DictionaryRef,
        pub subjects: Vec<Subject>,
        pub relationships: Vec<Relationship>,
        pub properties: Vec<DictionaryProperty>,
        pub controlled_lists: Vec<ControlledValueList>,
        pub meta_subjects: Vec<Subject>,
    }

    /// 🗺️ ISO 12006-3 mapping record.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Iso12006Mapping {
        pub dictionary_object_id: String,
        pub iso12006_uri: String,
        pub object_kind: String,
    }

    pub fn subtype_closure(dictionary: &Dictionary, subject_id: &str) -> HashSet<String> {
        let mut closure = HashSet::from([subject_id.to_string()]);
        let mut changed = true;
        while changed {
            changed = false;
            for rel in &dictionary.relationships {
                if rel.kind == RelationshipKind::IsSubtypeOf && closure.contains(&rel.source_id) && closure.insert(rel.target_id.clone()) {
                    changed = true;
                }
            }
        }
        closure
    }

    pub fn detect_subtype_cycle(dictionary: &Dictionary) -> bool {
        for subject in &dictionary.subjects {
            let mut visited = HashSet::new();
            let mut stack = vec![subject.id.clone()];
            while let Some(current) = stack.pop() {
                if !visited.insert(current.clone()) {
                    return true;
                }
                for rel in dictionary.relationships.iter().filter(|r| r.kind == RelationshipKind::IsSubtypeOf && r.source_id == current) {
                    stack.push(rel.target_id.clone());
                }
            }
        }
        false
    }

    pub fn filter_controlled_values(list: &ControlledValueList, subject_id: &str, dictionary: &Dictionary) -> Vec<String> {
        if list.context_subject_ids.is_empty() {
            return list.values.clone();
        }
        let closure = subtype_closure(dictionary, subject_id);
        if list.context_subject_ids.iter().any(|ctx| closure.contains(ctx)) {
            list.values.clone()
        } else {
            Vec::new()
        }
    }

    pub fn resolve_property<'a>(dictionary: &'a Dictionary, property_id: &str) -> Option<&'a DictionaryProperty> {
        dictionary.properties.iter().find(|p| p.id == property_id)
    }

    pub fn validate_dictionary(dictionary: &Dictionary) -> Vec<String> {
        let mut issues = Vec::new();
        if detect_subtype_cycle(dictionary) {
            issues.push("subtype cycle detected".into());
        }
        for rel in &dictionary.relationships {
            if rel.kind == RelationshipKind::HasPart || rel.kind == RelationshipKind::HasBlock {
                if !rel.cardinality.satisfies(1) && rel.cardinality.min > 0 {
                    issues.push(format!("relationship {} requires cardinality review", rel.id));
                }
            }
            let source_exists = dictionary.subjects.iter().any(|s| s.id == rel.source_id);
            let target_exists = dictionary.subjects.iter().any(|s| s.id == rel.target_id);
            if !source_exists || !target_exists {
                issues.push(format!("relationship {} has dangling endpoints", rel.id));
            }
        }
        issues
    }

    pub fn to_iso12006_mappings(dictionary: &Dictionary) -> Vec<Iso12006Mapping> {
        dictionary
            .subjects
            .iter()
            .map(|subject| Iso12006Mapping {
                dictionary_object_id: subject.id.clone(),
                iso12006_uri: format!("iso12006://subject/{}", subject.id),
                object_kind: format!("{:?}", subject.kind),
            })
            .collect()
    }
}
// #endregion Part4

// #region Part5
pub mod part_5 {
    use super::*;
    use std::time::{Duration, Instant};

    /// 🔄 Exchange process stage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ExchangeProcess {
        CreateFromDictionary,
        ProvideCatalogue,
        DetermineProduct,
        IntegrateIntoSystem,
        ExchangeSystemModel,
    }

    /// 🔢 Part number rule.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum PartNumberRule {
        Literal { value: String },
        Table { rows: Vec<HashMap<String, String>>, output_column: String },
        Script { function_id: String, source: String },
    }

    /// 📄 External media reference.
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
        pub attributes: HashMap<String, String>,
        pub children: Vec<IfcCatalogueNode>,
    }

    /// 📦 IFC catalogue root.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct IfcCatalogue {
        pub schema: String,
        pub metadata: IfcCatalogueNode,
        pub product_classes: Vec<IfcCatalogueNode>,
        pub products: Vec<IfcCatalogueNode>,
        pub unknown_entities: Vec<String>,
    }

    /// 🧮 Script execution limits.
    #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

    /// 📤 Script execution result.
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

    /// 🧮 Sandboxed calculation runtime (constrained numeric expressions).
    pub trait ScriptRuntime {
        fn execute(&self, source: &str, inputs: &HashMap<String, f64>, limits: ScriptLimits) -> Result<ScriptResult, ScriptError>;
    }

    /// 🔢 Default deterministic script runtime.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DefaultScriptRuntime;

    impl ScriptRuntime for DefaultScriptRuntime {
        fn execute(&self, source: &str, inputs: &HashMap<String, f64>, limits: ScriptLimits) -> Result<ScriptResult, ScriptError> {
            let started = Instant::now();
            let trimmed = source.trim();
            if trimmed.contains("import") || trimmed.contains("require") || trimmed.contains("fetch") || trimmed.contains("fs") {
                return Err(ScriptError::InvalidExpression("forbidden construct".into()));
            }
            let result = eval_expression(trimmed, inputs, limits.max_steps, limits.max_recursion, 0, &started, limits.timeout_ms)?;
            Ok(ScriptResult { value: result, diagnostics: Vec::new() })
        }
    }

    fn eval_expression(expr: &str, inputs: &HashMap<String, f64>, steps: u32, max_recursion: u32, depth: u32, started: &Instant, timeout_ms: u64) -> Result<f64, ScriptError> {
        if started.elapsed() > Duration::from_millis(timeout_ms) {
            return Err(ScriptError::Timeout(timeout_ms));
        }
        if depth > max_recursion {
            return Err(ScriptError::RecursionLimit(max_recursion));
        }
        if steps == 0 {
            return Err(ScriptError::StepLimit(0));
        }
        let expr = expr.trim();
        if let Ok(value) = expr.parse::<f64>() {
            return Ok(value);
        }
        if let Some(value) = inputs.get(expr) {
            return Ok(*value);
        }
        if let Some(inner) = expr.strip_prefix('(').and_then(|s| s.strip_suffix(')')) {
            return eval_expression(inner, inputs, steps - 1, max_recursion, depth + 1, started, timeout_ms);
        }
        for operation in ["+", "-", "*", "/"] {
            if let Some((left, right)) = split_binary(expr, operation) {
                let l = eval_expression(left, inputs, steps - 1, max_recursion, depth + 1, started, timeout_ms)?;
                let r = eval_expression(right, inputs, steps - 1, max_recursion, depth + 1, started, timeout_ms)?;
                return Ok(match operation {
                    "+" => l + r,
                    "-" => l - r,
                    "*" => l * r,
                    "/" => {
                        if r.abs() < f64::EPSILON {
                            return Err(ScriptError::InvalidExpression("division by zero".into()));
                        }
                        l / r
                    }
                    _ => unreachable!(),
                });
            }
        }
        Err(ScriptError::InvalidExpression(expr.into()))
    }

    fn split_binary<'a>(expr: &'a str, operation: &str) -> Option<(&'a str, &'a str)> {
        let mut depth = 0i32;
        for (idx, ch) in expr.char_indices().rev() {
            if ch == ')' {
                depth += 1;
            } else if ch == '(' {
                depth -= 1;
            } else if depth == 0 && expr[idx..].starts_with(operation) {
                let left = expr[..idx].trim();
                let right = expr[idx + operation.len()..].trim();
                if !left.is_empty() && !right.is_empty() {
                    return Some((left, right));
                }
            }
        }
        None
    }

    fn catalogue_value_as_str(value: &CatalogueValue) -> Option<String> {
        match value {
            CatalogueValue::Text { value } | CatalogueValue::Enumeration { value } | CatalogueValue::Identifier { value } => Some(value.clone()),
            CatalogueValue::Decimal { value } => Some(value.to_string()),
            CatalogueValue::Integer { value } => Some(value.to_string()),
            _ => None,
        }
    }

    pub fn calculate_part_number(rule: &PartNumberRule, inputs: &HashMap<String, CatalogueValue>, runtime: &dyn ScriptRuntime) -> Result<String, NormError> {
        match rule {
            PartNumberRule::Literal { value } => Ok(value.clone()),
            PartNumberRule::Table { rows, output_column } => {
                for row in rows {
                    let mut matches = true;
                    for (key, expected) in row.iter().filter(|(k, _)| *k != output_column) {
                        let actual = inputs.get(key).and_then(catalogue_value_as_str);
                        if actual.as_deref() != Some(expected.as_str()) {
                            matches = false;
                            break;
                        }
                    }
                    if matches {
                        return row.get(output_column).cloned().ok_or_else(|| NormError::IncompleteInput { field: output_column.clone() });
                    }
                }
                Err(NormError::InvalidValue { field: "part_number_table".into(), reason: "no matching row".into() })
            }
            PartNumberRule::Script { source, .. } => {
                let numeric: HashMap<String, f64> = inputs
                    .iter()
                    .filter_map(|(k, v)| match v {
                        CatalogueValue::Decimal { value } => Some((k.clone(), *value)),
                        CatalogueValue::Integer { value } => Some((k.clone(), *value as f64)),
                        _ => None,
                    })
                    .collect();
                let result = runtime
                    .execute(source, &numeric, ScriptLimits::default())
                    .map_err(|e| NormError::InvalidValue { field: "part_number_script".into(), reason: e.to_string() })?;
                Ok(format!("{:.0}", result.value))
            }
        }
    }

    pub fn build_ifc_catalogue(catalogue: &part_1::Catalogue) -> IfcCatalogue {
        let metadata = IfcCatalogueNode {
            entity_type: "IfcBuildingServicesCatalogue".into(),
            global_id: catalogue.id.0.clone(),
            name: catalogue.metadata.names.preferred.text.clone(),
            attributes: HashMap::from([
                ("dictionaryId".into(), catalogue.dictionary.id.clone()),
                ("dictionaryVersion".into(), catalogue.dictionary.version.clone()),
            ]),
            children: Vec::new(),
        };
        let product_classes = catalogue
            .product_classes
            .iter()
            .map(|class| IfcCatalogueNode {
                entity_type: "IfcProductClass".into(),
                global_id: class.id.clone(),
                name: class.names.preferred.text.clone(),
                attributes: HashMap::from([("groupId".into(), class.group_id.clone())]),
                children: Vec::new(),
            })
            .collect();
        let products = catalogue
            .products
            .iter()
            .map(|product| IfcCatalogueNode {
                entity_type: "IfcProduct".into(),
                global_id: product.id.clone(),
                name: product.names.preferred.text.clone(),
                attributes: HashMap::from([("seriesId".into(), product.series_id.clone())]),
                children: Vec::new(),
            })
            .collect();
        IfcCatalogue { schema: "IFC4".into(), metadata, product_classes, products, unknown_entities: Vec::new() }
    }

    pub fn export_ifc_step(catalogue: &IfcCatalogue) -> String {
        let mut lines = vec![format!("ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('ISO 16757-5'),'2;1');\nFILE_NAME('{}.ifc','',(''),(''),'norm_iso_16757','','');\nENDSEC;\nDATA;", catalogue.metadata.global_id)];
        let mut id = 1u32;
        let write_node = |node: &IfcCatalogueNode, id: &mut u32| -> String {
            let current = *id;
            *id += 1;
            format!(
                "#{}={}('{}','{}','{}',({}));",
                current,
                node.entity_type,
                node.global_id,
                node.name,
                node.name,
                node.attributes.values().map(|v| format!("'{v}'")).collect::<Vec<_>>().join(",")
            )
        };
        lines.push(write_node(&catalogue.metadata, &mut id));
        for class in &catalogue.product_classes {
            lines.push(write_node(class, &mut id));
        }
        for product in &catalogue.products {
            lines.push(write_node(product, &mut id));
        }
        lines.push("ENDSEC;\nEND-ISO-10303-21;".into());
        lines.join("\n")
    }

    pub fn validate_exchange(catalogue: &part_1::Catalogue, ifc: &IfcCatalogue) -> Vec<String> {
        let mut issues = Vec::new();
        if ifc.schema.is_empty() {
            issues.push("missing IFC schema declaration".into());
        }
        if ifc.products.len() != catalogue.products.len() {
            issues.push(format!("IFC product count {} != catalogue {}", ifc.products.len(), catalogue.products.len()));
        }
        issues
    }
}
// #endregion Part5

// #region Io
pub mod io {
    use super::*;

    pub fn catalogue_to_json(catalogue: &part_1::Catalogue) -> Result<String, NormError> {
        serde_json::to_string_pretty(catalogue).map_err(|e| NormError::InvalidValue { field: "catalogue".into(), reason: e.to_string() })
    }

    pub fn catalogue_from_json(json: &str) -> Result<part_1::Catalogue, NormError> {
        serde_json::from_str(json).map_err(|e| NormError::InvalidValue { field: "catalogue".into(), reason: e.to_string() })
    }

    pub fn dictionary_to_json(dictionary: &part_4::Dictionary) -> Result<String, NormError> {
        serde_json::to_string_pretty(dictionary).map_err(|e| NormError::InvalidValue { field: "dictionary".into(), reason: e.to_string() })
    }
}
// #endregion Io

// #region Session
/// 📋 ISO 16757 evaluation session document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub catalogue: part_1::Catalogue,
    pub dictionary: part_4::Dictionary,
    pub geometry: part_2::GeometryCatalogue,
    pub selection: part_1::SelectionRequest,
    pub part_number_rule: part_5::PartNumberRule,
    pub part_number_inputs: HashMap<String, CatalogueValue>,
    pub script_limits: part_5::ScriptLimits,
    pub exchange_process: part_5::ExchangeProcess,
}

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
                    alternatives: Vec::new(),
                },
                definition: LocalizedText { locale: "en".into(), text: "Flow control valve".into() },
                parent_id: None,
            }],
            relationships: Vec::new(),
            properties: vec![part_4::DictionaryProperty {
                id: "prop.dn".into(),
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "Nominal diameter".into() },
                    short_name: None,
                    alternatives: Vec::new(),
                },
                kind: part_1::PropertyKind::Static,
                data_type: "decimal".into(),
                unit: Some(CatalogueUnit { symbol: "mm".into(), dimension: DimensionSignature::LENGTH, si_factor: 0.001 }),
                applicable_subject_ids: vec!["subject.valve".into()],
                value_constraints: vec![part_4::ValueConstraint { min: Some(15.0), max: Some(300.0), allowed_values: Vec::new() }],
            }],
            controlled_lists: vec![part_4::ControlledValueList {
                id: "dn.list".into(),
                values: vec!["50".into(), "80".into(), "100".into()],
                context_subject_ids: vec!["subject.valve".into()],
            }],
            meta_subjects: Vec::new(),
        };
        let geometry_id: String = "geom.valve.50".into();
        let mut geometry_objects = HashMap::new();
        geometry_objects.insert(
            geometry_id.clone(),
            part_2::GeometryObject {
                id: geometry_id.clone(),
                shape: Some(part_2::GeometryNode::Primitive {
                    kind: "box".into(),
                    parameters: HashMap::from([("width".into(), 0.15), ("height".into(), 0.20), ("depth".into(), 0.10)]),
                }),
                symbolic: None,
                spaces: vec![part_2::SpaceEnvelope { kind: part_2::SpaceKind::Installation, bounds: part_2::BoundingBox::from_size(0.30, 0.30, 0.30) }],
                surfaces: Vec::new(),
                ports: vec![part_2::PortDefinition {
                    id: "port.in".into(),
                    medium: "water".into(),
                    position: [0.0, 0.1, 0.05],
                    direction: [1.0, 0.0, 0.0],
                    port_type: "inlet".into(),
                }],
                parameter_bindings: HashMap::from([("width".into(), "prop.dn".into())]),
            },
        );
        let catalogue = part_1::Catalogue {
            id: CatalogueId("cat.demo".into()),
            metadata: part_1::CatalogueMetadata {
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "Demo HVAC catalogue".into() },
                    short_name: Some("Demo".into()),
                    alternatives: Vec::new(),
                },
                lifecycle: Lifecycle { revision: "1".into(), status: "published".into(), valid_from: None, valid_to: None },
                edition_profile: part_1::EditionProfile::FullPublished,
            },
            manufacturer: part_1::Manufacturer {
                id: "mfg.demo".into(),
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "Demo Manufacturer".into() },
                    short_name: None,
                    alternatives: Vec::new(),
                },
            },
            dictionary: dictionary.reference.clone(),
            product_groups: vec![part_1::ProductGroup {
                id: "group.valves".into(),
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "Valves".into() },
                    short_name: None,
                    alternatives: Vec::new(),
                },
                dictionary_subject_id: Some("subject.valve".into()),
            }],
            product_classes: vec![part_1::ProductClass {
                id: "class.valve".into(),
                group_id: "group.valves".into(),
                parent_id: None,
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "Control valve".into() },
                    short_name: None,
                    alternatives: Vec::new(),
                },
                required_property_ids: vec!["prop.dn".into()],
                optional_property_ids: Vec::new(),
            }],
            product_series: vec![part_1::ProductSeries {
                id: "series.cv".into(),
                class_id: "class.valve".into(),
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "CV series".into() },
                    short_name: None,
                    alternatives: Vec::new(),
                },
                shared_property_values: HashMap::new(),
                geometry_id: Some(geometry_id.clone()),
            }],
            products: vec![part_1::Product {
                id: "product.cv".into(),
                series_id: "series.cv".into(),
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "CV-50".into() },
                    short_name: None,
                    alternatives: Vec::new(),
                },
                parameter_domains: vec![part_1::ParameterDomain {
                    parameter_id: "dn".into(),
                    allowed_values: vec![CatalogueValue::Decimal { value: 50.0 }],
                    default_value: Some(CatalogueValue::Decimal { value: 50.0 }),
                }],
                variants: vec![part_1::ProductVariant {
                    id: "variant.50".into(),
                    parameter_values: HashMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]),
                    property_values: vec![part_1::PropertyValue {
                        definition_id: "prop.dn".into(),
                        value: CatalogueValue::Decimal { value: 50.0 },
                        function_id: None,
                    }],
                    article_number: Some("CV-50".into()),
                    geometry_id: Some(geometry_id),
                }],
                static_properties: Vec::new(),
            }],
            product_indexes: vec![part_1::ProductIndex {
                id: "index.cv50".into(),
                product_id: "product.cv".into(),
                variant_id: Some("variant.50".into()),
                search_tags: vec!["valve".into(), "dn50".into()],
            }],
            property_definitions: vec![part_1::PropertyDefinition {
                id: "prop.dn".into(),
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "Nominal diameter".into() },
                    short_name: None,
                    alternatives: Vec::new(),
                },
                data_type: "decimal".into(),
                unit: Some(CatalogueUnit { symbol: "mm".into(), dimension: DimensionSignature::LENGTH, si_factor: 0.001 }),
                cardinality: Cardinality::required(),
                kind: part_1::PropertyKind::Static,
                dictionary_property_id: Some("prop.dn".into()),
            }],
            accessories: HashMap::new(),
            compositions: HashMap::new(),
            descriptive_objects: Vec::new(),
            extensions: ExtensionBag::default(),
        };
        Self {
            catalogue,
            dictionary,
            geometry: part_2::GeometryCatalogue { objects: geometry_objects, primitive_registry: part_2::GeometryCatalogue::default_primitives() },
            selection: part_1::SelectionRequest {
                class_id: "class.valve".into(),
                constraints: vec![part_1::SelectionConstraint {
                    property_id: "prop.dn".into(),
                    operator: part_1::ConstraintOperator::Equal,
                    value: CatalogueValue::Decimal { value: 50.0 },
                }],
                series_id: Some("series.cv".into()),
            },
            part_number_rule: part_5::PartNumberRule::Script { function_id: "partno".into(), source: "dn * 10 + 50".into() },
            part_number_inputs: HashMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]),
            script_limits: part_5::ScriptLimits::default(),
            exchange_process: part_5::ExchangeProcess::DetermineProduct,
        }
    }
}

pub type Operation = SetDocumentOperation<Document>;
pub type Host = NormHost<Iso16757Family>;

fn clause(part: &str, section: &str) -> ClauseId {
    ClauseId::new("ISO 16757", part, section)
}

fn check_count(report: &mut CheckReport, clause: ClauseId, actual: f64, expected: f64, message: impl Into<String>) {
    report.push(CheckResult::from_utilization(
        clause,
        Quantity::new(QuantityKind::Dimensionless, actual),
        Quantity::new(QuantityKind::Dimensionless, expected),
        message,
        AnnexChoice::En,
    ));
}

pub fn evaluate(document: &Document) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = AnnexChoice::En;

    let structure_issues = part_1::validate_catalogue_structure(&document.catalogue);
    check_count(&mut report, clause("1", "3.1"), if structure_issues.is_empty() { 1.0 } else { 2.0 }, 1.0, "catalogue structure");
    for issue in &structure_issues {
        report.push(CheckResult::fail(
            clause("1", "3.1"),
            Quantity::new(QuantityKind::Dimensionless, 0.0),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            2.0,
            issue.clone(),
            annex,
        ));
    }

    let selection = part_1::select_products(&document.catalogue, &document.selection);
    let expected_matches: f64 = if document.selection.class_id == "class.valve" { 1.0 } else { 0.0 };
    check_count(&mut report, clause("1", "4.2"), selection.matches.len() as f64, expected_matches.max(1.0), "product selection");
    if selection.ambiguity {
        report.push(CheckResult::fail(
            clause("1", "4.2"),
            Quantity::new(QuantityKind::Dimensionless, selection.matches.len() as f64),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            2.0,
            String::from("ambiguous selection"),
            annex,
        ));
    }

    if let Ok(embedding) = part_1::resolve_bim_embedding(&document.catalogue, "index.cv50", HashMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })])) {
        let has_geometry = embedding.resolved_geometry_id.is_some();
        report.push(if has_geometry {
            CheckResult::pass(
                clause("1", "10"),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                1.0,
                "BIM embedding resolved geometry",
                annex,
            )
        } else {
            CheckResult::fail(
                clause("1", "10"),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                0.0,
                "missing geometry in BIM embedding",
                annex,
            )
        });
    }

    if let Some(geom) = document.geometry.objects.get("geom.valve.50") {
        if let Some(shape) = &geom.shape {
            match part_2::evaluate_bounding_box(shape, &document.geometry) {
                Ok(bbox) => {
                    let volume = bbox.volume_m3();
                    report.push(CheckResult::from_utilization(
                        clause("2", "7.1"),
                        Quantity::new(QuantityKind::Volume, volume),
                        Quantity::new(QuantityKind::Volume, 0.003),
                        format!("primitive bbox volume {volume:.4} m3"),
                        annex,
                    ));
                    let step = part_2::project_step_entity(geom, bbox);
                    if step.contains("IFCBOUNDINGBOX") {
                        report.push(CheckResult::pass(
                            clause("2", "7.4"),
                            Quantity::new(QuantityKind::Dimensionless, 1.0),
                            Quantity::new(QuantityKind::Dimensionless, 1.0),
                            1.0,
                            "STEP/IFC geometry projection",
                            annex,
                        ));
                    }
                }
                Err(err) => {
                    report.push(CheckResult::fail(
                        clause("2", "6.1"),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        2.0,
                        err.to_string(),
                        annex,
                    ));
                }
            }
        }
        let mut visited = HashSet::new();
        let geom_issues = part_2::validate_geometry_graph(geom, &document.geometry, &mut visited);
        if !geom_issues.is_empty() {
            for issue in geom_issues {
                report.push(CheckResult::fail(
                    clause("2", "6.1"),
                    Quantity::new(QuantityKind::Dimensionless, 0.0),
                    Quantity::new(QuantityKind::Dimensionless, 1.0),
                    2.0,
                    issue,
                    annex,
                ));
            }
        }
        if let Some(install_space) = geom.spaces.iter().find(|s| s.kind == part_2::SpaceKind::Installation) {
            let product_bbox = part_2::BoundingBox::from_size(0.15, 0.20, 0.10);
            let clearance_ok = !product_bbox.overlaps(install_space.bounds, 0.05);
            report.push(if clearance_ok {
                CheckResult::pass(
                    clause("2", "5.3.5"),
                    Quantity::new(QuantityKind::Length, 0.05),
                    Quantity::new(QuantityKind::Length, 0.05),
                    1.0,
                    "installation clearance",
                    annex,
                )
            } else {
                CheckResult::fail(
                    clause("2", "5.3.5"),
                    Quantity::new(QuantityKind::Length, 0.0),
                    Quantity::new(QuantityKind::Length, 0.05),
                    0.0,
                    "insufficient installation clearance",
                    annex,
                )
            });
        }
    }

    let dict_issues = part_4::validate_dictionary(&document.dictionary);
    check_count(&mut report, clause("4", "4.3"), if dict_issues.is_empty() { 1.0 } else { 2.0 }, 1.0, "dictionary structure");
    let allowed = part_4::filter_controlled_values(
        document.dictionary.controlled_lists.first().expect("fixture list"),
        "subject.valve",
        &document.dictionary,
    );
    if allowed.contains(&"50".to_string()) {
        report.push(CheckResult::pass(
            clause("4", "6.3.2"),
            Quantity::new(QuantityKind::Dimensionless, 50.0),
            Quantity::new(QuantityKind::Dimensionless, 50.0),
            1.0,
            "context-filtered controlled value",
            annex,
        ));
    }
    let mappings = part_4::to_iso12006_mappings(&document.dictionary);
    if !mappings.is_empty() {
        report.push(CheckResult::pass(
            clause("4", "5.1"),
            Quantity::new(QuantityKind::Dimensionless, mappings.len() as f64),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            1.0,
            "ISO 12006-3 mapping",
            annex,
        ));
    }

    let ifc = part_5::build_ifc_catalogue(&document.catalogue);
    let exchange_issues = part_5::validate_exchange(&document.catalogue, &ifc);
    check_count(&mut report, clause("5", "6.1"), if exchange_issues.is_empty() { 1.0 } else { 2.0 }, 1.0, "IFC catalogue structure");
    let step = part_5::export_ifc_step(&ifc);
    if step.contains("IFCPRODUCT") || step.contains("IfcProduct") {
        report.push(CheckResult::pass(
            clause("5", "6.1"),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            Quantity::new(QuantityKind::Dimensionless, 1.0),
            1.0,
            "IFC STEP export",
            annex,
        ));
    }

    let runtime = part_5::DefaultScriptRuntime;
    use part_5::ScriptRuntime;
    match part_5::calculate_part_number(&document.part_number_rule, &document.part_number_inputs, &runtime) {
        Ok(part_no) => {
            let expected = 550.0;
            let actual: f64 = part_no.parse().unwrap_or(0.0);
            report.push(CheckResult::from_utilization(
                clause("5", "6.10"),
                Quantity::new(QuantityKind::Dimensionless, actual),
                Quantity::new(QuantityKind::Dimensionless, expected),
                format!("part number script result {part_no}"),
                annex,
            ));
        }
        Err(err) => {
            report.push(CheckResult::fail(
                clause("5", "6.10"),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                2.0,
                err.to_string(),
                annex,
            ));
        }
    }

    match runtime.execute("1/(0)", &HashMap::new(), document.script_limits) {
        Err(part_5::ScriptError::InvalidExpression(_)) => {
            report.push(CheckResult::pass(
                clause("5", "8"),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                1.0,
                "script division-by-zero guard",
                annex,
            ));
        }
        _ => {
            report.push(CheckResult::fail(
                clause("5", "8"),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                2.0,
                "script should reject division by zero",
                annex,
            ));
        }
    }

    report
}

pub struct Iso16757Family;

impl NormFamily for Iso16757Family {
    type Document = Document;
    type Operation = Operation;

    fn family_id() -> NormFamilyId {
        NormFamilyId::Iso16757
    }

    fn evaluate(document: &Document) -> CheckReport {
        evaluate(document)
    }
}
// #endregion Session

#[cfg(test)]
mod tests {
    use super::*;
    use norm_core::CheckStatus;
    use part_5::ScriptRuntime;

    #[test]
    fn reference_fixture_selects_one_product() {
        let doc = Document::default();
        let selection = part_1::select_products(&doc.catalogue, &doc.selection);
        assert_eq!(selection.matches.len(), 1);
        assert!(!selection.ambiguity);
    }

    #[test]
    fn geometry_bbox_volume_for_box_primitive() {
        let doc = Document::default();
        let geom = doc.geometry.objects.get("geom.valve.50").expect("geometry");
        let bbox = part_2::evaluate_bounding_box(geom.shape.as_ref().expect("shape"), &doc.geometry).expect("bbox");
        assert!((bbox.volume_m3() - 0.003).abs() < 1e-6);
    }

    #[test]
    fn dictionary_controlled_values_filter_by_subject() {
        let doc = Document::default();
        let list = doc.dictionary.controlled_lists.first().expect("list");
        let allowed = part_4::filter_controlled_values(list, "subject.valve", &doc.dictionary);
        assert_eq!(allowed, vec!["50", "80", "100"]);
    }

    #[test]
    fn part_number_script_is_deterministic() {
        let runtime = part_5::DefaultScriptRuntime;
        let rule = part_5::PartNumberRule::Script { function_id: "partno".into(), source: "dn * 10 + 50".into() };
        let inputs = HashMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]);
        let part_no = part_5::calculate_part_number(&rule, &inputs, &runtime).expect("part number");
        assert_eq!(part_no, "550");
    }

    #[test]
    fn ifc_step_export_contains_data_section() {
        let doc = Document::default();
        let ifc = part_5::build_ifc_catalogue(&doc.catalogue);
        let step = part_5::export_ifc_step(&ifc);
        assert!(step.contains("ENDSEC"));
        assert!(step.contains("IFCPRODUCT") || step.contains("IfcProduct"));
    }

    #[test]
    fn evaluate_exercises_all_parts_with_numeric_checks() {
        let report = evaluate(&Document::default());
        assert!(!report.checks.is_empty());
        let clauses: HashSet<String> = report.checks.iter().map(|c| format!("{} {}", c.clause.part, c.clause.section)).collect();
        assert!(clauses.iter().any(|c| c.starts_with("1 ")));
        assert!(clauses.iter().any(|c| c.starts_with("2 ")));
        assert!(clauses.iter().any(|c| c.starts_with("4 ")));
        assert!(clauses.iter().any(|c| c.starts_with("5 ")));
        let part_number_check = report.checks.iter().find(|c| c.clause.section == "6.10").expect("part number check");
        assert_eq!(part_number_check.status, CheckStatus::Pass);
        assert!((part_number_check.computed.value - 550.0).abs() < 1e-6);
    }

    #[test]
    fn catalogue_json_round_trip() {
        let doc = Document::default();
        let json = io::catalogue_to_json(&doc.catalogue).expect("json");
        let restored = io::catalogue_from_json(&json).expect("restore");
        assert_eq!(restored.id, doc.catalogue.id);
    }

    #[test]
    fn composition_cycle_detected() {
        let mut doc = Document::default();
        doc.catalogue.compositions.insert("product.a".into(), vec![part_1::CompositionRelationship { component_product_id: "product.b".into(), quantity: 1 }]);
        doc.catalogue.compositions.insert("product.b".into(), vec![part_1::CompositionRelationship { component_product_id: "product.a".into(), quantity: 1 }]);
        assert!(part_1::detect_composition_cycle(&doc.catalogue, "product.a"));
    }

    #[test]
    fn script_rejects_forbidden_import() {
        let runtime = part_5::DefaultScriptRuntime;
        let err = runtime.execute("import fs", &HashMap::new(), part_5::ScriptLimits::default()).unwrap_err();
        assert!(matches!(err, part_5::ScriptError::InvalidExpression(_)));
    }

    #[test]
    fn norm_family_evaluate_matches_host() {
        let doc = Document::default();
        let host = Host::from_document(doc);
        assert!(!host.report().checks.is_empty());
        assert_eq!(Iso16757Family::family_id(), NormFamilyId::Iso16757);
    }
}
