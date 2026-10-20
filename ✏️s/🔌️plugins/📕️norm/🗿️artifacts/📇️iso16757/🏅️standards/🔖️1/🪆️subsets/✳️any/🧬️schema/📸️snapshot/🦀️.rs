//! 🧬️ Iso16757 snapshot schema — artifact-lane fields only.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;
use semio_s_plugin_norm_compliance::LocalizedText;
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

/// 📋️ ISO 16757 evaluation session document.
impl Iso16757Snapshot {
    pub fn reference_fixture() -> Self {
        let dictionary = crate::part_4::Dictionary {
            reference: DictionaryRef { id: "hvac-dict".into(), version: "2025-01".into() },
            subjects: vec![crate::part_4::Subject {
                id: "subject-valve".into(),
                kind: crate::part_4::SubjectKind::ProductClass,
                names: Names {
                    preferred: LocalizedText { locale: "en".into(), text: "Control valve".into() },
                    short_name: Some("Valve".into()),
                    alternatives: vec![LocalizedText { locale: "de".into(), text: "Regelventil".into() }, LocalizedText { locale: "fr".into(), text: "Vanne de régulation".into() }],
                },
                definition: LocalizedText { locale: "en".into(), text: "Flow control valve".into() },
                parent_id: None,
            }],
            relationships: Vec::new(),
            properties: vec![crate::part_4::DictionaryProperty {
                id: "prop-dn".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Nominal diameter".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Nennweite".into() }] },
                kind: crate::part_1::PropertyKind::Static,
                data_type: "decimal".into(),
                unit: Some(CatalogueUnit { symbol: "mm".into(), dimension: DimensionSignature::LENGTH, si_factor: 0.001 }),
                applicable_subject_ids: vec!["subject-valve".into()],
                value_constraints: vec![crate::part_4::ValueConstraint { min: Some(15.0), max: Some(300.0), allowed_values: Vec::new() }],
            }],
            controlled_lists: vec![crate::part_4::ControlledValueList { id: "dn-list".into(), values: vec!["50".into(), "80".into(), "100".into()], context_subject_ids: vec!["subject-valve".into()] }],
            meta_subjects: Vec::new(),
        };
        let geometry_id: String = "geom-valve-50".into();
        let mut geometry_objects = BTreeMap::new();
        geometry_objects.insert(
            geometry_id.clone(),
            crate::part_2::GeometryObject {
                id: geometry_id.clone(),
                shape: Some(crate::part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 0.15), ("height".into(), 0.20), ("depth".into(), 0.10)]) }),
                symbolic: None,
                spaces: vec![crate::part_2::SpaceEnvelope { id: "installation".into(), kind: crate::part_2::SpaceKind::Installation, bounds: crate::part_2::BoundingBox { min: [-0.05, -0.05, -0.05], max: [0.20, 0.25, 0.15] } }],
                surfaces: Vec::new(),
                ports: vec![crate::part_2::PortDefinition { id: "port-in".into(), medium: "water".into(), position: [0.0, 0.1, 0.05], direction: [1.0, 0.0, 0.0], port_type: "inlet".into() }],
                parameter_bindings: BTreeMap::from([("width".into(), "prop-dn".into())]),
            },
        );
        let catalogue = crate::part_1::Catalogue {
            id: CatalogueId("cat.mfg-demo".into()),
            metadata: crate::part_1::CatalogueMetadata {
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Demo HVAC catalogue".into() }, short_name: Some("Demo".into()), alternatives: vec![LocalizedText { locale: "de".into(), text: "Demo TGA-Katalog".into() }] },
                lifecycle: Lifecycle { revision: "1".into(), status: "published".into(), valid_from: None, valid_to: None },
                edition_profile: crate::part_1::EditionProfile::FullPublished,
            },
            manufacturer: crate::part_1::Manufacturer {
                id: "mfg-demo".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Demo Manufacturer".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Demo Hersteller".into() }] },
            },
            dictionary: dictionary.reference.clone(),
            product_groups: vec![crate::part_1::ProductGroup {
                id: "group-valves".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Valves".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Ventile".into() }] },
                dictionary_subject_id: Some("subject-valve".into()),
            }],
            product_classes: vec![crate::part_1::ProductClass {
                id: "class-valve".into(),
                group_id: "group-valves".into(),
                parent_id: None,
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Control valve".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Regelventil".into() }] },
                required_property_ids: vec!["prop-dn".into()],
                optional_property_ids: Vec::new(),
            }],
            product_series: vec![crate::part_1::ProductSeries {
                id: "series-cv".into(),
                class_id: "class-valve".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "CV series".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "CV-Serie".into() }] },
                shared_property_values: BTreeMap::new(),
                geometry_id: Some(geometry_id.clone()),
            }],
            products: vec![crate::part_1::Product {
                id: "product-cv".into(),
                series_id: "series-cv".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "CV-50".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "CV-50".into() }] },
                parameter_domains: vec![crate::part_1::ParameterDomain { parameter_id: "dn".into(), allowed_values: vec![CatalogueValue::Decimal { value: 50.0 }], default_value: Some(CatalogueValue::Decimal { value: 50.0 }) }],
                variants: vec![crate::part_1::ProductVariant {
                    id: "variant-50".into(),
                    parameter_values: BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]),
                    property_values: vec![crate::part_1::PropertyValue { definition_id: "prop-dn".into(), value: CatalogueValue::Decimal { value: 50.0 }, function_id: None }],
                    article_number: Some("CV-50".into()),
                    geometry_id: Some(geometry_id),
                }],
                static_properties: Vec::new(),
            }],
            product_indexes: vec![crate::part_1::ProductIndex { id: "index-cv50".into(), product_id: "product-cv".into(), variant_id: Some("variant-50".into()), search_tags: vec!["CV-50".into(), "product-cv".into(), "dn50".into()] }],
            property_definitions: vec![crate::part_1::PropertyDefinition {
                id: "prop-dn".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Nominal diameter".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Nennweite".into() }] },
                data_type: "decimal".into(),
                unit: Some(CatalogueUnit { symbol: "mm".into(), dimension: DimensionSignature::LENGTH, si_factor: 0.001 }),
                cardinality: Cardinality::required(),
                kind: crate::part_1::PropertyKind::Static,
                dictionary_property_id: Some("prop-dn".into()),
            }],
            accessories: BTreeMap::new(),
            compositions: BTreeMap::new(),
            descriptive_objects: Vec::new(),
            extensions: ExtensionBag::default(),
        };
        Self {
            catalogue,
            dictionary,
            geometry: crate::part_2::GeometryCatalogue { objects: geometry_objects, primitive_registry: crate::part_2::GeometryCatalogue::default_primitives() },
            selection: crate::part_1::SelectionRequest {
                class_id: "class-valve".into(),
                constraints: vec![crate::part_1::SelectionConstraint { id: "constraint-default".into(), property_id: "prop-dn".into(), operator: crate::part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 50.0 } }],
                series_id: Some("series-cv".into()),
            },
            part_number_rule: crate::part_5::PartNumberRule::Script { function_id: "partno".into(), source: "dn * 10 + 50".into() },
            part_number_inputs: BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]),
            script_limits: crate::part_5::ScriptLimits::default(),
            exchange_process: crate::part_5::ExchangeProcess::DetermineProduct,
        }
    }

    /// 🚫️ Non-compliant catalogue exercising many ISO 16757 failures at once.
    pub fn broken_fixture() -> Self {
        let mut doc = Self::reference_fixture();
        doc.catalogue.metadata.names.alternatives.clear();
        doc.catalogue.metadata.names.alternatives.push(LocalizedText { locale: "de".into(), text: "Demo TGA-Katalog".into() });
        if let Some(product) = doc.catalogue.products.get_mut(0) {
            product.names.alternatives.clear();
            product.names.alternatives.push(LocalizedText { locale: "fr".into(), text: "CV-50".into() });
            if let Some(variant) = product.variants.get_mut(0) {
                variant.geometry_id = Some("geom.missing".into());
                variant.parameter_values.insert("dn".into(), CatalogueValue::Decimal { value: 999.0 });
            }
        }
        if let Some(constraint) = doc.selection.constraints.get_mut(0) {
            constraint.value = CatalogueValue::Decimal { value: 999.0 };
        }
        doc.catalogue.product_indexes.push(crate::part_1::ProductIndex {
            id: "index-cv50-dup".into(),
            product_id: "product-cv".into(),
            variant_id: Some("variant-50".into()),
            search_tags: vec!["dup".into()],
        });
        doc.catalogue.compositions.insert(
            "product-cv".into(),
            vec![crate::part_1::CompositionRelationship { component_product_id: "product-cv".into(), quantity: 1 }],
        );
        if let Some(obj) = doc.geometry.objects.values_mut().next() {
            obj.spaces = vec![crate::part_2::SpaceEnvelope { id: "installation".into(), kind: crate::part_2::SpaceKind::Installation,
                bounds: crate::part_2::BoundingBox::from_size(0.01, 0.01, 0.01),
            }];
        }
        doc.part_number_inputs.insert("dn".into(), CatalogueValue::Decimal { value: 40.0 });
        doc.script_limits = crate::part_5::ScriptLimits { max_steps: 0, max_recursion: 0, timeout_ms: 0 };
        doc.exchange_process = crate::part_5::ExchangeProcess::IntegrateIntoSystem;
        doc.dictionary.relationships.push(crate::part_4::Relationship {
            id: "rel-dangling".into(),
            kind: crate::part_4::RelationshipKind::HasPart,
            source_id: "subject-valve".into(),
            target_id: "subject-missing".into(),
            cardinality: Cardinality::required(),
        });
        doc
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
