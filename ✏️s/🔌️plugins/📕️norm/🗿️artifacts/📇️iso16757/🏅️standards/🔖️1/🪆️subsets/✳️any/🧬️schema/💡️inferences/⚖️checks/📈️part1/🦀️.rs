//! 📈️ ISO 16757-1 catalogue integrity, naming, properties, selection, BIM embedding.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_iso16757_checks_common::{Section, ALL_FACETS, assess, catalogue_subject, copy, fail, na, name_locales, names_cover, pass, q_dim, subject, controlled_ordinal, reference_slot_score};
use semio_s_plugin_norm_compliance::{CheckReport, CheckStatus, Quantity, QuantityKind, Remedy, RemedyBound};
use semio_s_artifact_norm_iso16757_model::part_1::{Catalogue, Product, PropertyDefinition, SelectionRequest};
use semio_s_artifact_norm_iso16757_model::CatalogueValue;
use semio_s_artifact_norm_iso16757_checks_common::part_libraries::part_1 as helpers;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use std::collections::{HashMap, HashSet};

fn required_languages(catalogue: &Catalogue) -> Vec<String> {
    let mut langs = name_locales(&catalogue.metadata.names);
    if langs.is_empty() {
        langs.push("en".into());
    }
    langs
}

fn def_by_id<'a>(catalogue: &'a Catalogue, id: &str) -> Option<&'a PropertyDefinition> {
    catalogue.property_definitions.iter().find(|d| d.id == id)
}

fn value_as_f64(value: &CatalogueValue) -> Option<f64> {
    match value {
        CatalogueValue::Decimal { value } => Some(*value),
        CatalogueValue::Integer { value } => Some(*value as f64),
        CatalogueValue::Quantity { value, .. } => Some(*value),
        _ => None,
    }
}

fn quantity_dimension_ok(value: &CatalogueValue, def: &PropertyDefinition) -> bool {
    match (value, &def.unit) {
        (CatalogueValue::Quantity { unit, .. }, Some(expected)) => unit.dimension == expected.dimension,
        (CatalogueValue::Quantity { .. }, None) => false,
        (_, Some(_)) => matches!(value, CatalogueValue::Decimal { .. } | CatalogueValue::Integer { .. } | CatalogueValue::Null { .. }),
        _ => true,
    }
}

/// 🧮️ The sections of Part 1 in report order, each with the facets it reads.
pub const SECTIONS: &[Section] = &[
    Section { id: "header", reads: &["catalogue"], run: |d, r| check_header(&d.catalogue, &required_languages(&d.catalogue), r) },
    Section { id: "dictionary-alignment", reads: ALL_FACETS, run: |d, r| check_dictionary_catalogue_alignment(d, r) },
    Section { id: "unique-ids", reads: &["catalogue"], run: |d, r| check_unique_ids(&d.catalogue, r) },
    Section { id: "referential-integrity", reads: &["catalogue", "geometry"], run: |d, r| check_referential_integrity(&d.catalogue, &d.geometry, r) },
    Section { id: "group-subjects", reads: &["catalogue", "dictionary"], run: |d, r| check_group_dictionary_subjects(&d.catalogue, &d.dictionary, r) },
    Section { id: "multilingual", reads: &["catalogue"], run: |d, r| check_multilingual(&d.catalogue, &required_languages(&d.catalogue), r) },
    Section { id: "required-properties", reads: &["catalogue"], run: |d, r| check_required_properties(&d.catalogue, r) },
    Section { id: "property-values", reads: &["catalogue", "dictionary"], run: |d, r| check_property_values(&d.catalogue, &d.dictionary, r) },
    Section { id: "property-units", reads: &["catalogue", "dictionary"], run: |d, r| check_property_definition_units(&d.catalogue, &d.dictionary, r) },
    Section { id: "variant-domains", reads: &["catalogue"], run: |d, r| check_variant_domains(&d.catalogue, r) },
    Section { id: "variant-article-geometry", reads: &["catalogue", "geometry"], run: |d, r| check_variant_article_and_geometry(&d.catalogue, &d.geometry, r) },
    Section { id: "accessories-compositions", reads: &["catalogue"], run: |d, r| check_accessories_compositions(&d.catalogue, r) },
    Section { id: "product-indexes", reads: &["catalogue"], run: |d, r| check_product_indexes(&d.catalogue, r) },
    Section { id: "selection", reads: &["catalogue", "selection"], run: |d, r| check_selection(&d.catalogue, &d.selection, r) },
    Section { id: "bim-embeddings", reads: &["catalogue"], run: |d, r| check_bim_embeddings(&d.catalogue, r) },
];

pub fn check_part_1(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    SECTIONS.iter().for_each(|section| (section.run)(doc, report));
}

#[path = "🪪️header-ids-and-referential-integrity/🦀️.rs"]
mod header_ids_and_referential_integrity;
use header_ids_and_referential_integrity::*;

#[path = "🌐️properties-and-multilingual-checks/🦀️.rs"]
mod properties_and_multilingual_checks;
use properties_and_multilingual_checks::*;

#[path = "🧩️variants-accessories-and-indexes/🦀️.rs"]
mod variants_accessories_and_indexes;
use variants_accessories_and_indexes::*;

#[path = "🎯️selection-and-bim-checks/🦀️.rs"]
mod selection_and_bim_checks;
use selection_and_bim_checks::*;
