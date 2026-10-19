//! 📦️ ISO 16757 building-services product catalogue: parts 1, 2, 4, 5 — document entities.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

/// 📸️ Persisted snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.

use std::collections::BTreeMap;

// #region Shared

/// 🌐️ Locale-tagged text — re-exported from `crate::document`, the single canonical definition
/// shared across every norm artifact (kills this type's former duplicate here and in `vdi3805`;
/// see `crate::document`'s `🔖️LocalizedText` region for the full rationale).

/// 🔗️ Internal or external reference.
#[derive(Clone, Debug, PartialEq, crate::value_derive::ToValue, crate::value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CatalogueReference {
    pub uri: String,
    pub label: Option<String>,
}

// #endregion Shared

// #region Part1
// #endregion Part1

// #region Part2
// #endregion Part2

// #region Part4
// #endregion Part4

// #region Part5
// #endregion Part5

// #region Session
/// 🏷️ Canonical DSL file extension for ISO 16757 documents.
pub const ISO16757_EXTENSION: &str = "iso16757";

/// 📋️ ISO 16757 evaluation session document.
impl Iso16757Snapshot {
    pub fn reference_fixture() -> Self {
        let dictionary = part_4::Dictionary {
            reference: DictionaryRef { id: "hvac-dict".into(), version: "2025-01".into() },
            subjects: vec![part_4::Subject {
                id: "subject-valve".into(),
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
                id: "prop-dn".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Nominal diameter".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Nennweite".into() }] },
                kind: part_1::PropertyKind::Static,
                data_type: "decimal".into(),
                unit: Some(CatalogueUnit { symbol: "mm".into(), dimension: DimensionSignature::LENGTH, si_factor: 0.001 }),
                applicable_subject_ids: vec!["subject-valve".into()],
                value_constraints: vec![part_4::ValueConstraint { min: Some(15.0), max: Some(300.0), allowed_values: Vec::new() }],
            }],
            controlled_lists: vec![part_4::ControlledValueList { id: "dn-list".into(), values: vec!["50".into(), "80".into(), "100".into()], context_subject_ids: vec!["subject-valve".into()] }],
            meta_subjects: Vec::new(),
        };
        let geometry_id: String = "geom-valve-50".into();
        let mut geometry_objects = BTreeMap::new();
        geometry_objects.insert(
            geometry_id.clone(),
            part_2::GeometryObject {
                id: geometry_id.clone(),
                shape: Some(part_2::GeometryNode::Primitive { kind: "box".into(), parameters: BTreeMap::from([("width".into(), 0.15), ("height".into(), 0.20), ("depth".into(), 0.10)]) }),
                symbolic: None,
                spaces: vec![part_2::SpaceEnvelope { id: "installation".into(), kind: part_2::SpaceKind::Installation, bounds: part_2::BoundingBox { min: [-0.05, -0.05, -0.05], max: [0.20, 0.25, 0.15] } }],
                surfaces: Vec::new(),
                ports: vec![part_2::PortDefinition { id: "port-in".into(), medium: "water".into(), position: [0.0, 0.1, 0.05], direction: [1.0, 0.0, 0.0], port_type: "inlet".into() }],
                parameter_bindings: BTreeMap::from([("width".into(), "prop-dn".into())]),
            },
        );
        let catalogue = part_1::Catalogue {
            id: CatalogueId("cat.mfg-demo".into()),
            metadata: part_1::CatalogueMetadata {
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Demo HVAC catalogue".into() }, short_name: Some("Demo".into()), alternatives: vec![LocalizedText { locale: "de".into(), text: "Demo TGA-Katalog".into() }] },
                lifecycle: Lifecycle { revision: "1".into(), status: "published".into(), valid_from: None, valid_to: None },
                edition_profile: part_1::EditionProfile::FullPublished,
            },
            manufacturer: part_1::Manufacturer {
                id: "mfg-demo".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Demo Manufacturer".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Demo Hersteller".into() }] },
            },
            dictionary: dictionary.reference.clone(),
            product_groups: vec![part_1::ProductGroup {
                id: "group-valves".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Valves".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Ventile".into() }] },
                dictionary_subject_id: Some("subject-valve".into()),
            }],
            product_classes: vec![part_1::ProductClass {
                id: "class-valve".into(),
                group_id: "group-valves".into(),
                parent_id: None,
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Control valve".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Regelventil".into() }] },
                required_property_ids: vec!["prop-dn".into()],
                optional_property_ids: Vec::new(),
            }],
            product_series: vec![part_1::ProductSeries {
                id: "series-cv".into(),
                class_id: "class-valve".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "CV series".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "CV-Serie".into() }] },
                shared_property_values: BTreeMap::new(),
                geometry_id: Some(geometry_id.clone()),
            }],
            products: vec![part_1::Product {
                id: "product-cv".into(),
                series_id: "series-cv".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "CV-50".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "CV-50".into() }] },
                parameter_domains: vec![part_1::ParameterDomain { parameter_id: "dn".into(), allowed_values: vec![CatalogueValue::Decimal { value: 50.0 }], default_value: Some(CatalogueValue::Decimal { value: 50.0 }) }],
                variants: vec![part_1::ProductVariant {
                    id: "variant-50".into(),
                    parameter_values: BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]),
                    property_values: vec![part_1::PropertyValue { definition_id: "prop-dn".into(), value: CatalogueValue::Decimal { value: 50.0 }, function_id: None }],
                    article_number: Some("CV-50".into()),
                    geometry_id: Some(geometry_id),
                }],
                static_properties: Vec::new(),
            }],
            product_indexes: vec![part_1::ProductIndex { id: "index-cv50".into(), product_id: "product-cv".into(), variant_id: Some("variant-50".into()), search_tags: vec!["CV-50".into(), "product-cv".into(), "dn50".into()] }],
            property_definitions: vec![part_1::PropertyDefinition {
                id: "prop-dn".into(),
                names: Names { preferred: LocalizedText { locale: "en".into(), text: "Nominal diameter".into() }, short_name: None, alternatives: vec![LocalizedText { locale: "de".into(), text: "Nennweite".into() }] },
                data_type: "decimal".into(),
                unit: Some(CatalogueUnit { symbol: "mm".into(), dimension: DimensionSignature::LENGTH, si_factor: 0.001 }),
                cardinality: Cardinality::required(),
                kind: part_1::PropertyKind::Static,
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
            geometry: part_2::GeometryCatalogue { objects: geometry_objects, primitive_registry: part_2::GeometryCatalogue::default_primitives() },
            selection: part_1::SelectionRequest {
                class_id: "class-valve".into(),
                constraints: vec![part_1::SelectionConstraint { id: "constraint-default".into(), property_id: "prop-dn".into(), operator: part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 50.0 } }],
                series_id: Some("series-cv".into()),
            },
            part_number_rule: part_5::PartNumberRule::Script { function_id: "partno".into(), source: "dn * 10 + 50".into() },
            part_number_inputs: BTreeMap::from([("dn".into(), CatalogueValue::Decimal { value: 50.0 })]),
            script_limits: part_5::ScriptLimits::default(),
            exchange_process: part_5::ExchangeProcess::DetermineProduct,
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
        doc.catalogue.product_indexes.push(part_1::ProductIndex {
            id: "index-cv50-dup".into(),
            product_id: "product-cv".into(),
            variant_id: Some("variant-50".into()),
            search_tags: vec!["dup".into()],
        });
        doc.catalogue.compositions.insert(
            "product-cv".into(),
            vec![part_1::CompositionRelationship { component_product_id: "product-cv".into(), quantity: 1 }],
        );
        if let Some(obj) = doc.geometry.objects.values_mut().next() {
            obj.spaces = vec![part_2::SpaceEnvelope { id: "installation".into(), kind: part_2::SpaceKind::Installation,
                bounds: part_2::BoundingBox::from_size(0.01, 0.01, 0.01),
            }];
        }
        doc.part_number_inputs.insert("dn".into(), CatalogueValue::Decimal { value: 40.0 });
        doc.script_limits = part_5::ScriptLimits { max_steps: 0, max_recursion: 0, timeout_ms: 0 };
        doc.exchange_process = part_5::ExchangeProcess::IntegrateIntoSystem;
        doc.dictionary.relationships.push(part_4::Relationship {
            id: "rel-dangling".into(),
            kind: part_4::RelationshipKind::HasPart,
            source_id: "subject-valve".into(),
            target_id: "subject-missing".into(),
            cardinality: Cardinality::required(),
        });
        doc
    }

}
// #endregion Session

//#region 🪪️Declaration

pub fn declaration(definition: semio_framework_plugin::ArtifactDefinition) -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition)
        .schema(semio_s_artifact_norm_iso16757_schema::iso16757_artifact_schema_descriptor())
        .inferences([semio_s_artifact_norm_iso16757_inferences::iso16757_artifact_inference_descriptor()])
        .composers(semio_s_artifact_norm_iso16757_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention below.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.document",
                    extension: Some("iso16757"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_norm_iso16757_io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_iso16757_io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_iso16757_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_iso16757_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_norm_iso16757_io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_iso16757_io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_norm_iso16757_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_iso16757_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_norm_iso16757_io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_norm_iso16757_io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_iso16757_io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_iso16757_io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "iso16757.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_norm_iso16757_io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_norm_iso16757_io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("iso16757.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

// ---- Shims: keep pre-migration module paths resolving for external callers ----

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs"]
mod diff_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs"]
mod inferences_compliance_report;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🪢️Mounts
