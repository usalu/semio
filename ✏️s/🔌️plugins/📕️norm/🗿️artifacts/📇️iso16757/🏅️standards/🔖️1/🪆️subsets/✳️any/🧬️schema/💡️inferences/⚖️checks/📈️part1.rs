//! 📈️ ISO 16757-1 catalogue integrity, naming, properties, selection, BIM embedding.

use super::common::{assess, catalogue_subject, copy, fail, na, name_locales, names_cover, pass, q_dim, subject, controlled_ordinal, reference_slot_score};
use crate::document::{CheckReport, CheckStatus, Quantity, QuantityKind, Remedy, RemedyBound};
use crate::part_1::{Catalogue, Product, PropertyDefinition, SelectionRequest};
use crate::CatalogueValue;
use crate::standards::v1::subsets::any::schema::part_1 as helpers;
use crate::Iso16757Snapshot;
use std::collections::{BTreeMap, HashMap, HashSet};

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

pub fn check_part_1(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    let catalogue = &doc.catalogue;
    let required = required_languages(catalogue);
    check_header(catalogue, &required, report);
    check_dictionary_catalogue_alignment(doc, report);
    check_unique_ids(catalogue, report);
    check_referential_integrity(catalogue, &doc.geometry, report);
    check_group_dictionary_subjects(catalogue, &doc.dictionary, report);
    check_multilingual(catalogue, &required, report);
    check_required_properties(catalogue, report);
    check_property_values(catalogue, &doc.dictionary, report);
    check_property_definition_units(catalogue, &doc.dictionary, report);
    check_variant_domains(catalogue, report);
    check_variant_article_and_geometry(catalogue, &doc.geometry, report);
    check_accessories_compositions(catalogue, report);
    check_product_indexes(catalogue, report);
    check_selection(catalogue, &doc.selection, report);
    check_bim_embeddings(catalogue, report);
}

fn check_header(catalogue: &Catalogue, required: &[String], report: &mut CheckReport) {
    let title = copy("Catalogue identification", "Katalogidentifikation");
    let missing_name = catalogue.metadata.names.preferred.text.trim().is_empty();
    let missing_mfg = catalogue.manufacturer.names.preferred.text.trim().is_empty() || catalogue.manufacturer.id.trim().is_empty();
    let missing_id = catalogue.id.0.trim().is_empty();
    if missing_id || missing_name || missing_mfg || required.is_empty() {
        let mut remedies = Vec::new();
        if missing_id {
            remedies.push(Remedy {
                target: subject("catalogue", "catalogue.id", "Catalogue id", "Katalog-Id"),
                current: q_dim(0.0),
                required: q_dim(1.0),
                bound: RemedyBound::Exactly,
                options: vec!["cat.manufacturer.series".into()],
                action: copy(
                    "Set catalogue.id to a non-empty stable identifier (e.g. 'cat.manufacturer.series').",
                    "catalogue.id auf eine nicht-leere stabile Kennung setzen (z. B. 'cat.manufacturer.series').",
                ),
                applicable: true,
            });
        }
        if missing_name {
            remedies.push(Remedy {
                target: subject("catalogue", "catalogue.metadata.names.preferred.text", "Catalogue name", "Katalogname"),
                current: q_dim(0.0),
                required: q_dim(1.0),
                bound: RemedyBound::Exactly,
                options: Vec::new(),
                action: copy("Set catalogue.metadata.names.preferred.text to the published catalogue title.", "catalogue.metadata.names.preferred.text auf den veröffentlichten Katalogtitel setzen."),
                applicable: true,
            });
        }
        if missing_mfg {
            remedies.push(Remedy {
                target: subject(&catalogue.manufacturer.id, "catalogue.manufacturer.names.preferred.text", "Manufacturer", "Hersteller"),
                current: q_dim(0.0),
                required: q_dim(1.0),
                bound: RemedyBound::Exactly,
                options: Vec::new(),
                action: copy("Identify the manufacturer with a non-empty id and preferred name.", "Hersteller mit nicht-leerer Id und bevorzugtem Namen angeben."),
                applicable: true,
            });
        }
        report.push(fail(
            "iso16757.1.3.1.header",
            "1",
            "3.1",
            catalogue_subject(),
            title,
            copy("Catalogue header is incomplete (id, name, or manufacturer missing).", "Katalogkopf ist unvollständig (Id, Name oder Hersteller fehlen)."),
            remedies,
        ));
    } else {
        report.push(pass(
            "iso16757.1.3.1.header",
            "1",
            "3.1",
            catalogue_subject(),
            title,
            copy(
                format!("Catalogue '{}' by '{}' is identified.", catalogue.metadata.names.preferred.text, catalogue.manufacturer.names.preferred.text),
                format!("Katalog '{}' von '{}' ist identifiziert.", catalogue.metadata.names.preferred.text, catalogue.manufacturer.names.preferred.text),
            ),
        ));
    }

    // Part 1 §5.1 — catalogue lifecycle + dictionary reference must be populated for exchange.
    let status = catalogue.metadata.lifecycle.status.trim();
    let revision = catalogue.metadata.lifecycle.revision.trim();
    let allowed_status = crate::artifact_schema::part_1::LIFECYCLE_STATUSES;
    if status.is_empty() || !allowed_status.iter().any(|s| *s == status) {
        report.push(fail(
            "iso16757.1.5.1.lifecycle.status",
            "1",
            "5.1",
            subject("catalogue", "catalogue.metadata.lifecycle.status", "Lifecycle", "Lebenszyklus"),
            copy("Catalogue lifecycle status", "Katalog-Lebenszyklusstatus"),
            copy(
                format!("Lifecycle status '{status}' is not one of draft|published|withdrawn|superseded."),
                format!("Lebenszyklusstatus '{status}' ist nicht draft|published|withdrawn|superseded."),
            ),
            vec![Remedy::one_of(
                subject("catalogue", "catalogue.metadata.lifecycle.status", "Lifecycle", "Lebenszyklus"),
                allowed_status.iter().map(|s| s.to_string()).collect(),
                copy("Set catalogue.metadata.lifecycle.status to a Part 1 exchange status.", "catalogue.metadata.lifecycle.status auf einen Teil-1-Austauschstatus setzen."),
            )],
        ));
    } else {
        let status_ord = match status { "draft" => 1.0, "published" => 2.0, "withdrawn" => 3.0, _ => 4.0 };
        report.push(assess(
            "iso16757.1.5.1.lifecycle.status",
            "1",
            "5.1",
            subject("catalogue", "catalogue.metadata.lifecycle.status", "Lifecycle", "Lebenszyklus"),
            copy("Catalogue lifecycle status", "Katalog-Lebenszyklusstatus"),
            copy(
                format!("Lifecycle status '{status}' is valid for Part 1 exchange."),
                format!("Lebenszyklusstatus '{status}' ist für den Teil-1-Austausch gültig."),
            ),
            CheckStatus::Pass,
            q_dim(status_ord),
            q_dim(status_ord),
            Vec::new(),
        ));
    }
    let rev_num = revision.parse::<f64>();
    match rev_num {
        Ok(n) if n > 0.0 => {
            report.push(assess(
                "iso16757.1.5.1.lifecycle.revision",
                "1",
                "5.1",
                subject("catalogue", "catalogue.metadata.lifecycle.revision", "Revision", "Revision"),
                copy("Catalogue revision", "Katalog-Revision"),
                copy(
                    format!("Catalogue revision {n} is a positive number."),
                    format!("Katalog-Revision {n} ist eine positive Zahl."),
                ),
                CheckStatus::Pass,
                q_dim(n),
                q_dim(n),
                Vec::new(),
            ));
        }
        _ => {
            report.push(fail(
                "iso16757.1.5.1.lifecycle.revision",
                "1",
                "5.1",
                subject("catalogue", "catalogue.metadata.lifecycle.revision", "Revision", "Revision"),
                copy("Catalogue revision", "Katalog-Revision"),
                copy(
                    format!("Catalogue revision '{revision}' must be a positive number (Part 1 §5.1 metadata)."),
                    format!("Katalog-Revision '{revision}' muss eine positive Zahl sein (Teil 1 §5.1 Metadaten)."),
                ),
                vec![Remedy {
                    target: subject("catalogue", "catalogue.metadata.lifecycle.revision", "Revision", "Revision"),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec!["1".into()],
                    action: copy("Set catalogue.metadata.lifecycle.revision to a positive integer string.", "catalogue.metadata.lifecycle.revision auf eine positive Ganzzahl als Text setzen."),
                    applicable: true,
                }],
            ));
        }
    }
    if catalogue.dictionary.id.trim().is_empty() || catalogue.dictionary.version.trim().is_empty() {
        report.push(fail(
            "iso16757.1.5.1.dictionaryRef",
            "1",
            "5.1",
            subject("catalogue", "catalogue.dictionary.id", "Dictionary reference", "Wörterbuch-Referenz"),
            copy("Catalogue dictionary reference", "Katalog-Wörterbuch-Referenz"),
            copy("Catalogue dictionary id/version must be non-empty for Part 1 exchange.", "Katalog-Wörterbuch-Id/Version müssen für den Teil-1-Austausch nicht leer sein."),
            vec![Remedy {
                target: subject("catalogue", "catalogue.dictionary.id", "Dictionary reference", "Wörterbuch-Referenz"),
                current: q_dim(0.0),
                required: q_dim(1.0),
                bound: RemedyBound::Exactly,
                options: vec![],
                action: copy("Set catalogue.dictionary.id and .version to the referenced data dictionary.", "catalogue.dictionary.id und .version auf das referenzierte Datenwörterbuch setzen."),
                applicable: true,
            }],
        ));
    } else {
        report.push(assess(
            "iso16757.1.5.1.dictionaryRef",
            "1",
            "5.1",
            subject("catalogue", "catalogue.dictionary.version", "Dictionary reference", "Wörterbuch-Referenz"),
            copy("Catalogue dictionary reference", "Katalog-Wörterbuch-Referenz"),
            copy(
                format!("Dictionary reference {}@{} is present.", catalogue.dictionary.id, catalogue.dictionary.version),
                format!("Wörterbuch-Referenz {}@{} ist vorhanden.", catalogue.dictionary.id, catalogue.dictionary.version),
            ),
            CheckStatus::Pass,
            q_dim(1.0),
            q_dim(1.0),
            Vec::new(),
        ));
    }


    // Part 1 §3.1 — catalogue id is scoped under the manufacturer id.
    let cat_id = catalogue.id.0.trim();
    let mfg_id = catalogue.manufacturer.id.trim();
    if !mfg_id.is_empty() && !cat_id.is_empty() {
        let expected_id = format!("cat.{mfg_id}");
        let scope_score = reference_slot_score(cat_id, std::iter::once(expected_id.as_str()));
        if cat_id.contains(mfg_id) {
            report.push(assess(
                "iso16757.1.3.1.catalogueManufacturerScope",
                "1",
                "3.1",
                subject("catalogue", "catalogue.id", "Catalogue id scope", "Katalog-Id-Bereich"),
                copy("Catalogue id manufacturer scope", "Katalog-Id Herstellerbereich"),
                copy(
                    format!("Catalogue id '{cat_id}' is scoped under manufacturer '{mfg_id}'."),
                    format!("Katalog-Id '{cat_id}' ist unter Hersteller '{mfg_id}' eingeordnet."),
                ),
                CheckStatus::Pass,
                q_dim(scope_score),
                q_dim(1.0),
                Vec::new(),
            ));
        } else {
            report.push(fail(
                "iso16757.1.3.1.catalogueManufacturerScope",
                "1",
                "3.1",
                subject("catalogue", "catalogue.id", "Catalogue id scope", "Katalog-Id-Bereich"),
                copy("Catalogue id manufacturer scope", "Katalog-Id Herstellerbereich"),
                copy(
                    format!("Catalogue id '{cat_id}' must contain manufacturer id '{mfg_id}'."),
                    format!("Katalog-Id '{cat_id}' muss Hersteller-Id '{mfg_id}' enthalten."),
                ),
                vec![Remedy {
                    target: subject("catalogue", "catalogue.id", "Catalogue id", "Katalog-Id"),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec![format!("cat.{mfg_id}")],
                    action: copy("Prefix or embed the manufacturer id in catalogue.id.", "Hersteller-Id in catalogue.id einbetten."),
                    applicable: true,
                }],
            ));
        }
    }

}

fn check_dictionary_catalogue_alignment(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    let cat = &doc.catalogue.dictionary;
    let dict = &doc.dictionary.reference;
    let id_ok = cat.id == dict.id && !cat.id.is_empty();
    let ver_ok = cat.version == dict.version && !cat.version.is_empty();
    if !id_ok || !ver_ok {
        report.push(fail(
            "iso16757.1.5.1.dictionaryAlign",
            "1",
            "5.1",
            subject("catalogue", "catalogue.dictionary.id", "Dictionary alignment", "Wörterbuch-Ausrichtung"),
            copy("Catalogue↔dictionary reference alignment", "Katalog↔Wörterbuch-Referenz-Ausrichtung"),
            copy(
                format!("Catalogue dictionary {}@{} must equal dictionary.reference {}@{} (Part 1 §5.1 / Part 4 §4.3).", cat.id, cat.version, dict.id, dict.version),
                format!("Katalog-Wörterbuch {}@{} muss dictionary.reference {}@{} entsprechen (Teil 1 §5.1 / Teil 4 §4.3).", cat.id, cat.version, dict.id, dict.version),
            ),
            vec![
                Remedy {
                    target: subject("catalogue", "catalogue.dictionary.id", "Dictionary id", "Wörterbuch-Id"),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec![dict.id.clone()],
                    action: copy("Set catalogue.dictionary.id to dictionary.reference.id.", "catalogue.dictionary.id auf dictionary.reference.id setzen."),
                    applicable: true,
                },
                Remedy {
                    target: subject("catalogue", "catalogue.dictionary.version", "Dictionary version", "Wörterbuch-Version"),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec![dict.version.clone()],
                    action: copy("Set catalogue.dictionary.version to dictionary.reference.version.", "catalogue.dictionary.version auf dictionary.reference.version setzen."),
                    applicable: true,
                },
            ],
        ));
    } else {
        report.push(assess(
            "iso16757.1.5.1.dictionaryAlign",
            "1",
            "5.1",
            subject("catalogue", "catalogue.dictionary.id", "Dictionary alignment", "Wörterbuch-Ausrichtung"),
            copy("Catalogue↔dictionary reference alignment", "Katalog↔Wörterbuch-Referenz-Ausrichtung"),
            copy(
                format!("Catalogue and dictionary both reference {}@{}.", cat.id, cat.version),
                format!("Katalog und Wörterbuch referenzieren beide {}@{}.", cat.id, cat.version),
            ),
            CheckStatus::Pass,
            q_dim(1.0),
            q_dim(1.0),
            Vec::new(),
        ));
    }
}


fn check_unique_ids(catalogue: &Catalogue, report: &mut CheckReport) {
    let mut seen: HashMap<String, Vec<(String, String)>> = HashMap::new();
    let mut push_id = |id: &str, kind: &str, path: String| {
        if id.trim().is_empty() {
            return;
        }
        seen.entry(id.to_string()).or_default().push((kind.to_string(), path));
    };
    push_id(&catalogue.id.0, "catalogue", "catalogue.id".into());
    push_id(&catalogue.manufacturer.id, "manufacturer", "catalogue.manufacturer.id".into());
    for g in &catalogue.product_groups {
        push_id(&g.id, "productGroup", format!("catalogue.productGroups[id={}].id", g.id));
    }
    for c in &catalogue.product_classes {
        push_id(&c.id, "productClass", format!("catalogue.productClasses[id={}].id", c.id));
    }
    for s in &catalogue.product_series {
        push_id(&s.id, "productSeries", format!("catalogue.productSeries[id={}].id", s.id));
    }
    for p in &catalogue.products {
        push_id(&p.id, "product", format!("catalogue.products[id={}].id", p.id));
        for v in &p.variants {
            push_id(&v.id, "variant", format!("catalogue.products[id={}].variants[id={}].id", p.id, v.id));
        }
    }
    for d in &catalogue.property_definitions {
        push_id(&d.id, "propertyDefinition", format!("catalogue.propertyDefinitions[id={}].id", d.id));
    }
    for i in &catalogue.product_indexes {
        push_id(&i.id, "productIndex", format!("catalogue.productIndexes[id={}].id", i.id));
    }
    let mut duplicates = 0u32;
    for (id, kinds) in &seen {
        if kinds.len() > 1 {
            duplicates += 1;
            let (_kind, path) = &kinds[1];
            let rename = format!("{id}-renamed");
            report.push(fail(
                format!("iso16757.1.3.1.uniqueId.{id}"),
                "1",
                "3.1",
                subject(id, path.clone(), format!("Id {id}"), format!("Id {id}")),
                copy("Unique identifiers", "Eindeutige Kennungen"),
                copy(
                    format!("Identifier '{id}' is reused across {}.", kinds.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>().join(", ")),
                    format!("Kennung '{id}' wird mehrfach verwendet: {}.", kinds.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>().join(", ")),
                ),
                vec![Remedy {
                    target: subject(id, path.clone(), format!("Id {id}"), format!("Id {id}")),
                    current: q_dim(kinds.len() as f64),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec![rename.clone()],
                    action: copy(
                        format!("Rename the second use of '{id}' to '{rename}' at {path}."),
                        format!("Zweite Verwendung von '{id}' in '{rename}' umbenennen ({path})."),
                    ),
                    applicable: true,
                }],
            ));
        }
    }
    if duplicates == 0 {
        report.push(pass(
            "iso16757.1.3.1.uniqueIds",
            "1",
            "3.1",
            catalogue_subject(),
            copy("Unique identifiers", "Eindeutige Kennungen"),
            copy("All catalogue entity identifiers are unique.", "Alle Katalog-Kennungen sind eindeutig."),
        ));
    }
}


fn check_referential_integrity(catalogue: &Catalogue, geometry: &crate::part_2::GeometryCatalogue, report: &mut CheckReport) {
    let group_ids: HashSet<_> = catalogue.product_groups.iter().map(|g| g.id.clone()).collect();
    let class_ids: HashSet<_> = catalogue.product_classes.iter().map(|c| c.id.clone()).collect();
    let series_ids: HashSet<_> = catalogue.product_series.iter().map(|s| s.id.clone()).collect();
    let product_ids: HashSet<_> = catalogue.products.iter().map(|p| p.id.clone()).collect();
    let prop_ids: HashSet<_> = catalogue.property_definitions.iter().map(|d| d.id.clone()).collect();
    let mut issues = 0u32;

    for class in &catalogue.product_classes {
        if !group_ids.contains(&class.group_id) {
            issues += 1;
            let options: Vec<String> = group_ids.iter().cloned().collect();
            report.push(fail(
                format!("iso16757.1.3.2.classGroup.{}", class.id),
                "1",
                "3.2",
                subject(&class.id, format!("catalogue.productClasses[id={}].groupId", class.id), &class.names.preferred.text, &class.names.preferred.text),
                copy("Class group reference", "Klassen-Gruppenreferenz"),
                copy(
                    format!("Product class '{}' references unknown group '{}'.", class.id, class.group_id),
                    format!("Produktklasse '{}' verweist auf unbekannte Gruppe '{}'.", class.id, class.group_id),
                ),
                vec![Remedy {
                    target: subject(&class.id, format!("catalogue.productClasses[id={}].groupId", class.id), &class.names.preferred.text, &class.names.preferred.text),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: options.clone(),
                    action: copy(
                        format!("Set groupId of '{}' to one of: {}.", class.id, options.join(", ")),
                        format!("groupId von '{}' auf einen der Werte setzen: {}.", class.id, options.join(", ")),
                    ),
                    applicable: true,
                }],
            ));
        }
        for req in &class.required_property_ids {
            if !prop_ids.contains(req) {
                issues += 1;
                let options: Vec<String> = prop_ids.iter().cloned().collect();
                report.push(fail(
                    format!("iso16757.1.3.2.classProp.{}.{}", class.id, req),
                    "1",
                    "3.2",
                    subject(&class.id, format!("catalogue.productClasses[id={}].requiredPropertyIds", class.id), &class.names.preferred.text, &class.names.preferred.text),
                    copy("Required property definition", "Erforderliche Eigenschaftsdefinition"),
                    copy(
                        format!("Class '{}' requires unknown property definition '{}'.", class.id, req),
                        format!("Klasse '{}' fordert unbekannte Eigenschaftsdefinition '{}'.", class.id, req),
                    ),
                    vec![Remedy::one_of(
                        subject(&class.id, format!("catalogue.productClasses[id={}].requiredPropertyIds", class.id), &class.names.preferred.text, &class.names.preferred.text),
                        options,
                        copy(
                            format!("Replace '{req}' with an existing property definition id, or create that definition."),
                            format!("'{req}' durch eine vorhandene Eigenschaftsdefinitions-Id ersetzen oder die Definition anlegen."),
                        ),
                    )],
                ));
            }
        }
    }

    for series in &catalogue.product_series {
        if !class_ids.contains(&series.class_id) {
            issues += 1;
            let options: Vec<String> = class_ids.iter().cloned().collect();
            report.push(fail(
                format!("iso16757.1.3.2.seriesClass.{}", series.id),
                "1",
                "3.2",
                subject(&series.id, format!("catalogue.productSeries[id={}].classId", series.id), &series.names.preferred.text, &series.names.preferred.text),
                copy("Series class reference", "Serien-Klassenreferenz"),
                copy(
                    format!("Series '{}' references unknown class '{}'.", series.id, series.class_id),
                    format!("Serie '{}' verweist auf unbekannte Klasse '{}'.", series.id, series.class_id),
                ),
                vec![Remedy {
                    target: subject(&series.id, format!("catalogue.productSeries[id={}].classId", series.id), &series.names.preferred.text, &series.names.preferred.text),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: options.clone(),
                    action: copy(format!("Set classId to one of: {}.", options.join(", ")), format!("classId auf einen der Werte setzen: {}.", options.join(", "))),
                    applicable: true,
                }],
            ));
        }
        if let Some(gid) = &series.geometry_id {
            if !geometry.objects.contains_key(gid) {
                issues += 1;
                let options: Vec<String> = geometry.objects.keys().cloned().collect();
                report.push(fail(
                    format!("iso16757.1.3.2.seriesGeom.{}", series.id),
                    "1",
                    "3.2",
                    subject(&series.id, format!("catalogue.productSeries[id={}].geometryId", series.id), &series.names.preferred.text, &series.names.preferred.text),
                    copy("Series geometry reference", "Serien-Geometriereferenz"),
                    copy(
                        format!("Series '{}' references missing geometry '{}'.", series.id, gid),
                        format!("Serie '{}' verweist auf fehlende Geometrie '{}'.", series.id, gid),
                    ),
                    vec![Remedy {
                        target: subject(&series.id, format!("catalogue.productSeries[id={}].geometryId", series.id), &series.names.preferred.text, &series.names.preferred.text),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: options.clone(),
                        action: copy(
                            format!("Point geometryId at an existing object ({}) or add '{}'.", options.join(", "), gid),
                            format!("geometryId auf ein vorhandenes Objekt ({}) setzen oder '{}' anlegen.", options.join(", "), gid),
                        ),
                        applicable: !options.is_empty(),
                    }],
                ));
            }
        }
    }

    for product in &catalogue.products {
        if !series_ids.contains(&product.series_id) {
            issues += 1;
            let options: Vec<String> = series_ids.iter().cloned().collect();
            report.push(fail(
                format!("iso16757.1.3.2.productSeries.{}", product.id),
                "1",
                "3.2",
                subject(&product.id, format!("catalogue.products[id={}].seriesId", product.id), &product.names.preferred.text, &product.names.preferred.text),
                copy("Product series reference", "Produkt-Serienreferenz"),
                copy(
                    format!("Product '{}' references unknown series '{}'.", product.id, product.series_id),
                    format!("Produkt '{}' verweist auf unbekannte Serie '{}'.", product.id, product.series_id),
                ),
                vec![Remedy {
                    target: subject(&product.id, format!("catalogue.products[id={}].seriesId", product.id), &product.names.preferred.text, &product.names.preferred.text),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: options.clone(),
                    action: copy(format!("Set seriesId to one of: {}.", options.join(", ")), format!("seriesId auf einen der Werte setzen: {}.", options.join(", "))),
                    applicable: true,
                }],
            ));
        }
        for variant in &product.variants {
            if let Some(gid) = &variant.geometry_id {
                if !geometry.objects.contains_key(gid) {
                    issues += 1;
                    let options: Vec<String> = geometry.objects.keys().cloned().collect();
                    report.push(fail(
                        format!("iso16757.1.3.2.variantGeom.{}.{}", product.id, variant.id),
                        "1",
                        "3.2",
                        subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].geometryId", product.id, variant.id), &variant.id, &variant.id),
                        copy("Variant geometry reference", "Varianten-Geometriereferenz"),
                        copy(
                            format!("Variant '{}' references missing geometry '{}'.", variant.id, gid),
                            format!("Variante '{}' verweist auf fehlende Geometrie '{}'.", variant.id, gid),
                        ),
                        vec![Remedy {
                            target: subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].geometryId", product.id, variant.id), &variant.id, &variant.id),
                            current: q_dim(0.0),
                            required: q_dim(1.0),
                            bound: RemedyBound::OneOf,
                            options: options.clone(),
                            action: copy(
                                format!("Set geometryId to an existing id ({}) or create '{}'.", options.join(", "), gid),
                                format!("geometryId auf eine vorhandene Id ({}) setzen oder '{}' anlegen.", options.join(", "), gid),
                            ),
                            applicable: !options.is_empty(),
                        }],
                    ));
                }
            }
        }
    }

    for index in &catalogue.product_indexes {
        if !product_ids.contains(&index.product_id) {
            issues += 1;
            let options: Vec<String> = product_ids.iter().cloned().collect();
            report.push(fail(
                format!("iso16757.1.3.2.indexProduct.{}", index.id),
                "1",
                "3.2",
                subject(&index.id, format!("catalogue.productIndexes[id={}].productId", index.id), &index.id, &index.id),
                copy("Index product reference", "Index-Produktreferenz"),
                copy(
                    format!("Product index '{}' references unknown product '{}'.", index.id, index.product_id),
                    format!("Produktindex '{}' verweist auf unbekanntes Produkt '{}'.", index.id, index.product_id),
                ),
                vec![Remedy {
                    target: subject(&index.id, format!("catalogue.productIndexes[id={}].productId", index.id), &index.id, &index.id),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: options.clone(),
                    action: copy(format!("Set productId to one of: {}.", options.join(", ")), format!("productId auf einen der Werte setzen: {}.", options.join(", "))),
                    applicable: true,
                }],
            ));
        } else if let Some(vid) = &index.variant_id {
            let product = catalogue.products.iter().find(|p| p.id == index.product_id).unwrap();
            if product.variants.iter().all(|v| &v.id != vid) {
                issues += 1;
                let options: Vec<String> = product.variants.iter().map(|v| v.id.clone()).collect();
                report.push(fail(
                    format!("iso16757.1.3.2.indexVariant.{}", index.id),
                    "1",
                    "3.2",
                    subject(&index.id, format!("catalogue.productIndexes[id={}].variantId", index.id), &index.id, &index.id),
                    copy("Index variant reference", "Index-Variantenreferenz"),
                    copy(
                        format!("Product index '{}' references unknown variant '{}'.", index.id, vid),
                        format!("Produktindex '{}' verweist auf unbekannte Variante '{}'.", index.id, vid),
                    ),
                    vec![Remedy {
                        target: subject(&index.id, format!("catalogue.productIndexes[id={}].variantId", index.id), &index.id, &index.id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: options.clone(),
                        action: copy(format!("Set variantId to one of: {}.", options.join(", ")), format!("variantId auf einen der Werte setzen: {}.", options.join(", "))),
                        applicable: !options.is_empty(),
                    }],
                ));
            }
        }
    }

    if issues == 0 {
        report.push(pass(
            "iso16757.1.3.2.refs",
            "1",
            "3.2",
            catalogue_subject(),
            copy("Catalogue referential integrity", "Katalog-Referenzintegrität"),
            copy("All class/series/product/index/geometry references resolve.", "Alle Klassen-/Serien-/Produkt-/Index-/Geometrie-Referenzen sind auflösbar."),
        ));
    }
}

fn check_group_dictionary_subjects(catalogue: &Catalogue, dictionary: &crate::part_4::Dictionary, report: &mut CheckReport) {
    let subject_ids: HashSet<_> = dictionary.subjects.iter().map(|s| s.id.clone()).collect();
    for group in &catalogue.product_groups {
        if let Some(sid) = &group.dictionary_subject_id {
            let path = format!("catalogue.productGroups[id={}].dictionarySubjectId", group.id);
            if subject_ids.contains(sid) {
                report.push(assess(
                    format!("iso16757.1.5.groupSubject.{}", group.id),
                    "1",
                    "5",
                    subject(&group.id, path, &group.names.preferred.text, &group.names.preferred.text),
                    copy("Product group dictionary subject", "Produktgruppen-Wörterbuchsubjekt"),
                    copy(
                        format!("Group '{}' dictionarySubjectId '{sid}' resolves.", group.id),
                        format!("Gruppe '{}' dictionarySubjectId '{sid}' löst auf.", group.id),
                    ),
                    CheckStatus::Pass,
                    q_dim(1.0),
                    q_dim(1.0),
                    Vec::new(),
                ));
            } else {
                report.push(fail(
                    format!("iso16757.1.5.groupSubject.{}", group.id),
                    "1",
                    "5",
                    subject(&group.id, path.clone(), &group.names.preferred.text, &group.names.preferred.text),
                    copy("Product group dictionary subject", "Produktgruppen-Wörterbuchsubjekt"),
                    copy(
                        format!("Group '{}' dictionarySubjectId '{sid}' is unknown.", group.id),
                        format!("Gruppe '{}' dictionarySubjectId '{sid}' ist unbekannt.", group.id),
                    ),
                    vec![Remedy::one_of(
                        subject(&group.id, path, &group.names.preferred.text, &group.names.preferred.text),
                        subject_ids.iter().cloned().collect(),
                        copy("Point dictionarySubjectId at an existing subject.", "dictionarySubjectId auf vorhandenes Subjekt setzen."),
                    )],
                ));
            }
        }
    }
}


fn check_multilingual(catalogue: &Catalogue, required: &[String], report: &mut CheckReport) {
    let mut named: Vec<(String, String, &crate::Names)> = Vec::new();
    named.push(("catalogue".into(), "catalogue.metadata.names".into(), &catalogue.metadata.names));
    named.push((catalogue.manufacturer.id.clone(), "catalogue.manufacturer.names".into(), &catalogue.manufacturer.names));
    for (i, g) in catalogue.product_groups.iter().enumerate() {
        named.push((g.id.clone(), format!("catalogue.productGroups[id={}].names", g.id), &g.names));
    }
    for (i, c) in catalogue.product_classes.iter().enumerate() {
        named.push((c.id.clone(), format!("catalogue.productClasses[id={}].names", c.id), &c.names));
    }
    for (i, s) in catalogue.product_series.iter().enumerate() {
        named.push((s.id.clone(), format!("catalogue.productSeries[id={}].names", s.id), &s.names));
    }
    for (i, p) in catalogue.products.iter().enumerate() {
        named.push((p.id.clone(), format!("catalogue.products[id={}].names", p.id), &p.names));
    }
    for (i, d) in catalogue.property_definitions.iter().enumerate() {
        named.push((d.id.clone(), format!("catalogue.propertyDefinitions[id={}].names", d.id), &d.names));
    }

    let mut missing_count = 0u32;
    for (id, path, names) in named {
        let missing = names_cover(names, required);
        if missing.is_empty() {
            continue;
        }
        missing_count += 1;
        for locale in missing {
            report.push(fail(
                format!("iso16757.1.5.1.name.{id}.{locale}"),
                "1",
                "5.1",
                subject(&id, path.clone(), names.preferred.text.clone(), names.preferred.text.clone()),
                copy("Multilingual name coverage", "Mehrsprachige Namensabdeckung"),
                copy(
                    format!("Entity '{id}' lacks a name in required language '{locale}' (catalogue declares: {}).", required.join(", ")),
                    format!("Entität '{id}' hat keinen Namen in der geforderten Sprache '{locale}' (Katalog fordert: {}).", required.join(", ")),
                ),
                vec![Remedy {
                    target: subject(&id, format!("{path}.alternatives"), names.preferred.text.clone(), names.preferred.text.clone()),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::Exactly,
                    options: vec![locale.clone()],
                    action: copy(
                        format!("Add a name in language '{locale}' for '{id}' (e.g. alternatives += {{ locale: '{locale}', text: '…' }})."),
                        format!("Namen in Sprache '{locale}' für '{id}' ergänzen (z. B. alternatives += {{ locale: '{locale}', text: '…' }})."),
                    ),
                    applicable: false,
                }],
            ));
        }
    }
    if missing_count == 0 {
        report.push(pass(
            "iso16757.1.5.1.names",
            "1",
            "5.1",
            catalogue_subject(),
            copy("Multilingual name coverage", "Mehrsprachige Namensabdeckung"),
            copy(
                format!("All named catalogue entities cover required languages ({}).", required.join(", ")),
                format!("Alle benannten Katalogentitäten decken die geforderten Sprachen ab ({}).", required.join(", ")),
            ),
        ));
    }
}

fn product_class<'a>(catalogue: &'a Catalogue, product: &Product) -> Option<&'a crate::part_1::ProductClass> {
    let series = catalogue.product_series.iter().find(|s| s.id == product.series_id)?;
    catalogue.product_classes.iter().find(|c| c.id == series.class_id)
}

fn check_required_properties(catalogue: &Catalogue, report: &mut CheckReport) {
    let mut checked = 0u32;
    let mut failed = 0u32;
    for (pi, product) in catalogue.products.iter().enumerate() {
        let Some(class) = product_class(catalogue, product) else {
            continue;
        };
        for req in &class.required_property_ids {
            checked += 1;
            let on_product = product.static_properties.iter().any(|pv| &pv.definition_id == req);
            let missing_variants: Vec<&str> = product
                .variants
                .iter()
                .filter(|v| !on_product && v.property_values.iter().all(|pv| &pv.definition_id != req))
                .map(|v| v.id.as_str())
                .collect();
            if !on_product && (product.variants.is_empty() || !missing_variants.is_empty()) {
                failed += 1;
                let variant_hint = if missing_variants.is_empty() {
                    "product.staticProperties".to_string()
                } else {
                    format!("variants {}", missing_variants.join(", "))
                };
                report.push(fail(
                    format!("iso16757.1.5.2.required.{}.{}", product.id, req),
                    "1",
                    "5.2",
                    subject(&product.id, format!("catalogue.products[id={}].staticProperties", product.id), &product.names.preferred.text, &product.names.preferred.text),
                    copy("Required class property", "Erforderliche Klasseneigenschaft"),
                    copy(
                        format!("Product '{}' (class '{}') is missing required property '{}'.", product.id, class.id, req),
                        format!("Produkt '{}' (Klasse '{}') fehlt die erforderliche Eigenschaft '{}'.", product.id, class.id, req),
                    ),
                    vec![Remedy {
                        target: subject(&product.id, format!("catalogue.products[id={}].staticProperties", product.id), &product.names.preferred.text, &product.names.preferred.text),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::Exactly,
                        options: vec![req.clone()],
                        action: copy(
                            format!("Add property value for '{req}' on {variant_hint}."),
                            format!("Eigenschaftswert für '{req}' auf {variant_hint} ergänzen."),
                        ),
                        applicable: false,
                    }],
                ));
            }
        }
    }
    if checked == 0 {
        report.push(na(
            "iso16757.1.5.2.required",
            "1",
            "5.2",
            catalogue_subject(),
            copy("Required class property", "Erforderliche Klasseneigenschaft"),
            copy("No products with class-required properties to check.", "Keine Produkte mit klassenpflichtigen Eigenschaften zu prüfen."),
        ));
    } else if failed == 0 {
        report.push(pass(
            "iso16757.1.5.2.required",
            "1",
            "5.2",
            catalogue_subject(),
            copy("Required class property", "Erforderliche Klasseneigenschaft"),
            copy(
                format!("All {checked} required class property bindings are present."),
                format!("Alle {checked} erforderlichen Klasseneigenschaftsbindungen sind vorhanden."),
            ),
        ));
    }
}

fn check_property_values(catalogue: &Catalogue, dictionary: &crate::part_4::Dictionary, report: &mut CheckReport) {
    let mut checked = 0u32;
    let mut failed = 0u32;
    for (pi, product) in catalogue.products.iter().enumerate() {
        let mut values: Vec<(String, &CatalogueValue, String)> = Vec::new();
        for pv in &product.static_properties {
            values.push((pv.definition_id.clone(), &pv.value, format!("product:{}", product.id)));
        }
        for v in &product.variants {
            for pv in &v.property_values {
                values.push((pv.definition_id.clone(), &pv.value, format!("variant:{}", v.id)));
            }
        }
        for (def_id, value, scope) in values {
            checked += 1;
            let Some(def) = def_by_id(catalogue, &def_id) else {
                failed += 1;
                report.push(fail(
                    format!("iso16757.1.5.3.unknownDef.{}.{}", scope, def_id),
                    "1",
                    "5.3",
                    subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                    copy("Property definition resolvable", "Eigenschaftsdefinition auflösbar"),
                    copy(
                        format!("Property value on {scope} references unknown definition '{def_id}'."),
                        format!("Eigenschaftswert auf {scope} verweist auf unbekannte Definition '{def_id}'."),
                    ),
                    vec![Remedy::one_of(
                        subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                        catalogue.property_definitions.iter().map(|d| d.id.clone()).collect(),
                        copy(
                            format!("Point definitionId at an existing property definition or create '{def_id}'."),
                            format!("definitionId auf eine vorhandene Definition setzen oder '{def_id}' anlegen."),
                        ),
                    )],
                ));
                continue;
            };

            if !quantity_dimension_ok(value, def) {
                failed += 1;
                report.push(fail(
                    format!("iso16757.1.5.3.unit.{}.{}", scope, def_id),
                    "1",
                    "5.3",
                    subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                    copy("Property unit / dimension", "Eigenschaftseinheit / Dimension"),
                    copy(
                        format!("Value of '{def_id}' on {scope} does not match the definition unit/dimension."),
                        format!("Wert von '{def_id}' auf {scope} entspricht nicht der Definitions-Einheit/Dimension."),
                    ),
                    vec![Remedy {
                        target: subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::Exactly,
                        options: def.unit.as_ref().map(|u| u.symbol.clone()).into_iter().collect(),
                        action: copy(
                            format!("Provide a quantity in the definition unit for '{def_id}' (symbol {:?}).", def.unit.as_ref().map(|u| u.symbol.as_str())),
                            format!("Größe in der Definitionseinheit für '{def_id}' angeben (Symbol {:?}).", def.unit.as_ref().map(|u| u.symbol.as_str())),
                        ),
                        applicable: true,
                    }],
                ));
            }

            if let Some(dict_prop_id) = &def.dictionary_property_id {
                if let Some(dict_prop) = crate::standards::v1::subsets::any::schema::part_4::resolve_property(dictionary, dict_prop_id) {
                    for constraint in &dict_prop.value_constraints {
                            if let Some(v) = value_as_f64(value) {
                                if let Some(min) = constraint.min {
                                    if v < min {
                                        failed += 1;
                                        report.push(assess(
                                            format!("iso16757.1.5.3.min.{scope}.{def_id}"),
                                            "1",
                                            "5.3",
                                            subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                                            copy("Property value minimum", "Eigenschaftswert-Minimum"),
                                            copy(
                                                format!("Value {v} of '{def_id}' on {scope} is below minimum {min}."),
                                                format!("Wert {v} von '{def_id}' auf {scope} liegt unter Minimum {min}."),
                                            ),
                                            CheckStatus::Fail,
                                            Quantity::new(QuantityKind::Dimensionless, v),
                                            Quantity::new(QuantityKind::Dimensionless, min),
                                            vec![Remedy::at_least(
                                                subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                                                Quantity::new(QuantityKind::Dimensionless, v),
                                                Quantity::new(QuantityKind::Dimensionless, min),
                                                copy(
                                                    format!("Increase '{def_id}' on {scope} from {v} to at least {min}."),
                                                    format!("'{def_id}' auf {scope} von {v} auf mindestens {min} erhöhen."),
                                                ),
                                            )],
                                        ));
                                    }
                                }
                                if let Some(max) = constraint.max {
                                    if v > max {
                                        failed += 1;
                                        report.push(assess(
                                            format!("iso16757.1.5.3.max.{scope}.{def_id}"),
                                            "1",
                                            "5.3",
                                            subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                                            copy("Property value maximum", "Eigenschaftswert-Maximum"),
                                            copy(
                                                format!("Value {v} of '{def_id}' on {scope} exceeds maximum {max}."),
                                                format!("Wert {v} von '{def_id}' auf {scope} überschreitet Maximum {max}."),
                                            ),
                                            CheckStatus::Fail,
                                            Quantity::new(QuantityKind::Dimensionless, v),
                                            Quantity::new(QuantityKind::Dimensionless, max),
                                            vec![Remedy::at_most(
                                                subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                                                Quantity::new(QuantityKind::Dimensionless, v),
                                                Quantity::new(QuantityKind::Dimensionless, max),
                                                copy(
                                                    format!("Reduce '{def_id}' on {scope} from {v} to at most {max}."),
                                                    format!("'{def_id}' auf {scope} von {v} auf höchstens {max} verringern."),
                                                ),
                                            )],
                                        ));
                                    }
                                }
                                if !constraint.allowed_values.is_empty() {
                                    let as_text = match value {
                                        CatalogueValue::Decimal { value } => value.to_string(),
                                        CatalogueValue::Integer { value } => value.to_string(),
                                        CatalogueValue::Text { value } | CatalogueValue::Enumeration { value } | CatalogueValue::Identifier { value } => value.clone(),
                                        _ => String::new(),
                                    };
                                    if !as_text.is_empty() && !constraint.allowed_values.iter().any(|a| a == &as_text) {
                                        failed += 1;
                                        report.push(fail(
                                            format!("iso16757.1.5.3.enum.{scope}.{def_id}"),
                                            "1",
                                            "5.3",
                                            subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                                            copy("Property allowed values", "Zulässige Eigenschaftswerte"),
                                            copy(
                                                format!("Value '{as_text}' of '{def_id}' on {scope} is not in {:?}.", constraint.allowed_values),
                                                format!("Wert '{as_text}' von '{def_id}' auf {scope} ist nicht in {:?}.", constraint.allowed_values),
                                            ),
                                            vec![Remedy::one_of(
                                                subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                                                constraint.allowed_values.clone(),
                                                copy(
                                                    format!("Set '{def_id}' on {scope} to one of {:?}.", constraint.allowed_values),
                                                    format!("'{def_id}' auf {scope} auf einen der Werte {:?} setzen.", constraint.allowed_values),
                                                ),
                                            )],
                                        ));
                                    }
                                }
                            }
                    }
                }
            }
        }
    }

    if checked == 0 {
        report.push(na(
            "iso16757.1.5.3.values",
            "1",
            "5.3",
            catalogue_subject(),
            copy("Property value conformance", "Eigenschaftswert-Konformität"),
            copy("No product property values to validate.", "Keine Produkteigenschaftswerte zu prüfen."),
        ));
    } else if failed == 0 {
        report.push(pass(
            "iso16757.1.5.3.values",
            "1",
            "5.3",
            catalogue_subject(),
            copy("Property value conformance", "Eigenschaftswert-Konformität"),
            copy(
                format!("All {checked} property values match definitions and dictionary constraints."),
                format!("Alle {checked} Eigenschaftswerte entsprechen Definitionen und Wörterbuchbeschränkungen."),
            ),
        ));
    }
}

fn check_property_definition_units(catalogue: &Catalogue, dictionary: &crate::part_4::Dictionary, report: &mut CheckReport) {
    for def in &catalogue.property_definitions {
        let path = format!("catalogue.propertyDefinitions[id={}]", def.id);
        // dictionaryPropertyId resolvable — ISO 16757-1 §5.3 / Part 4 property dictionary binding
        if let Some(dict_prop_id) = &def.dictionary_property_id {
            let link_path = format!("{path}.dictionaryPropertyId");
            let resolved_idx = dictionary
                .properties
                .iter()
                .position(|p| p.id == *dict_prop_id)
                .map(|i| (i + 1) as f64);
            let resolved = resolved_idx.is_some();
            if resolved {
                let ordinal = resolved_idx.unwrap_or(1.0);
                report.push(assess(
                    format!("iso16757.1.5.3.dictProp.{}", def.id),
                    "1",
                    "5.3",
                    subject(&def.id, link_path, &def.names.preferred.text, &def.names.preferred.text),
                    copy("Dictionary property link", "Wörterbuch-Eigenschaftsverknüpfung"),
                    copy(
                        format!("Property definition '{}' links to dictionary property '{}' (slot {ordinal}).", def.id, dict_prop_id),
                        format!("Eigenschaftsdefinition '{}' verknüpft Wörterbucheigenschaft '{}' (Slot {ordinal}).", def.id, dict_prop_id),
                    ),
                    CheckStatus::Pass,
                    q_dim(ordinal),
                    q_dim(1.0),
                    Vec::new(),
                ));
            } else {
                report.push(assess(
                    format!("iso16757.1.5.3.dictProp.{}", def.id),
                    "1",
                    "5.3",
                    subject(&def.id, link_path.clone(), &def.names.preferred.text, &def.names.preferred.text),
                    copy("Dictionary property link", "Wörterbuch-Eigenschaftsverknüpfung"),
                    copy(
                        format!("Property definition '{}' links to missing dictionary property '{}'.", def.id, dict_prop_id),
                        format!("Eigenschaftsdefinition '{}' verweist auf fehlende Wörterbucheigenschaft '{}'.", def.id, dict_prop_id),
                    ),
                    CheckStatus::Fail,
                    q_dim(0.0),
                    q_dim(1.0),
                    vec![Remedy::one_of(
                        subject(&def.id, link_path, &def.names.preferred.text, &def.names.preferred.text),
                        dictionary.properties.iter().map(|p| p.id.clone()).collect(),
                        copy(
                            format!("Set dictionaryPropertyId to an existing dictionary property or create '{}'.", dict_prop_id),
                            format!("dictionaryPropertyId auf eine vorhandene Wörterbucheigenschaft setzen oder '{}' anlegen.", dict_prop_id),
                        ),
                    )],
                ));
            }
        }
        // dataType non-empty (Part 1 §6 / Part 4 property typing)
        if def.data_type.trim().is_empty() {
            report.push(fail(
                format!("iso16757.1.6.dataType.{}", def.id),
                "1",
                "6",
                subject(&def.id, format!("{path}.dataType"), &def.names.preferred.text, &def.names.preferred.text),
                copy("Property data type", "Eigenschafts-Datentyp"),
                copy(
                    format!("Property '{0}' dataType must be non-empty.", def.id),
                    format!("Eigenschaft '{0}' dataType darf nicht leer sein.", def.id),
                ),
                vec![Remedy::one_of(
                    subject(&def.id, format!("{path}.dataType"), &def.names.preferred.text, &def.names.preferred.text),
                    vec!["decimal".into(), "quantity".into(), "integer".into(), "text".into()],
                    copy("Set dataType to a Part 4 property data type.", "dataType auf einen Teil-4-Eigenschaftsdatentyp setzen."),
                )],
            ));
        } else {
            report.push(assess(
                format!("iso16757.1.6.dataType.{}", def.id),
                "1",
                "6",
                subject(&def.id, format!("{path}.dataType"), &def.names.preferred.text, &def.names.preferred.text),
                copy("Property data type", "Eigenschafts-Datentyp"),
                copy(
                    format!("Property '{}' dataType is '{}'.", def.id, def.data_type),
                    format!("Eigenschaft '{}' dataType ist '{}'.", def.id, def.data_type),
                ),
                CheckStatus::Pass,
                q_dim(controlled_ordinal(&def.data_type, &["decimal", "quantity", "integer", "text", "boolean", "controlled"])),
                q_dim(1.0),
                Vec::new(),
            ));
        }
        // cardinality min <= max (Part 1 §6)
        let max_v = def.cardinality.max.unwrap_or(u32::MAX) as f64;
        let min_v = def.cardinality.min as f64;
        if min_v > max_v {
            report.push(fail(
                format!("iso16757.1.6.cardinality.{}", def.id),
                "1",
                "6",
                subject(&def.id, format!("{path}.cardinality.min"), &def.names.preferred.text, &def.names.preferred.text),
                copy("Property cardinality", "Eigenschafts-Kardinalität"),
                copy(
                    format!("Property '{}' cardinality min ({}) exceeds max.", def.id, def.cardinality.min),
                    format!("Eigenschaft '{}' Kardinalität min ({}) überschreitet max.", def.id, def.cardinality.min),
                ),
                vec![Remedy {
                    target: subject(&def.id, format!("{path}.cardinality.min"), &def.names.preferred.text, &def.names.preferred.text),
                    current: q_dim(min_v),
                    required: q_dim(max_v.min(min_v)),
                    bound: RemedyBound::AtMost,
                    options: Vec::new(),
                    action: copy("Lower cardinality.min or raise cardinality.max.", "cardinality.min senken oder cardinality.max erhöhen."),
                    applicable: true,
                }],
            ));
        } else {
            report.push(assess(
                format!("iso16757.1.6.cardinality.{}", def.id),
                "1",
                "6",
                subject(&def.id, format!("{path}.cardinality.min"), &def.names.preferred.text, &def.names.preferred.text),
                copy("Property cardinality", "Eigenschafts-Kardinalität"),
                copy(
                    format!("Property '{}' cardinality [{min_v}, {max_v}] is ordered.", def.id),
                    format!("Eigenschaft '{}' Kardinalität [{min_v}, {max_v}] ist geordnet.", def.id),
                ),
                CheckStatus::Pass,
                q_dim(min_v),
                q_dim(if max_v.is_finite() && max_v < 1.0e9 { max_v } else { min_v }),
                Vec::new(),
            ));
        }
        if let Some(unit) = &def.unit {
            let dim_score = (unit.dimension.length.abs()
                + unit.dimension.mass.abs()
                + unit.dimension.time.abs()
                + unit.dimension.temperature.abs()) as f64;
            if unit.symbol.trim().is_empty() || unit.si_factor <= 0.0 {
                report.push(fail(
                    format!("iso16757.1.6.unit.{}", def.id),
                    "1",
                    "6",
                    subject(&def.id, format!("{path}.unit.symbol"), &def.names.preferred.text, &def.names.preferred.text),
                    copy("Property unit", "Eigenschafts-Einheit"),
                    copy(
                        format!("Property '{}' unit needs a symbol and positive siFactor (Part 1 §6 / Part 4).", def.id),
                        format!("Eigenschaft '{}' Einheit braucht Symbol und positiven siFactor (Teil 1 §6 / Teil 4).", def.id),
                    ),
                    vec![Remedy {
                        target: subject(&def.id, format!("{path}.unit.siFactor"), &def.names.preferred.text, &def.names.preferred.text),
                        current: q_dim(unit.si_factor),
                        required: q_dim(1.0),
                        bound: RemedyBound::Exactly,
                        options: Vec::new(),
                        action: copy("Set unit.symbol and a positive unit.siFactor.", "unit.symbol und positiven unit.siFactor setzen."),
                        applicable: true,
                    }],
                ));
            } else {
                report.push(assess(
                    format!("iso16757.1.6.unit.{}", def.id),
                    "1",
                    "6",
                    subject(&def.id, format!("{path}.unit.siFactor"), &def.names.preferred.text, &def.names.preferred.text),
                    copy("Property unit", "Eigenschafts-Einheit"),
                    copy(
                        format!("Property '{}' unit '{}' siFactor {} (dim score {dim_score}).", def.id, unit.symbol, unit.si_factor),
                        format!("Eigenschaft '{}' Einheit '{}' siFactor {} (Dim-Score {dim_score}).", def.id, unit.symbol, unit.si_factor),
                    ),
                    CheckStatus::Pass,
                    q_dim(unit.si_factor + dim_score * 1.0),
                    q_dim(unit.si_factor + dim_score * 1.0),
                    Vec::new(),
                ));
                // Also bind symbol length so symbol edits change computed via a dedicated check id
                report.push(assess(
                    format!("iso16757.1.6.unitSymbol.{}", def.id),
                    "1",
                    "6",
                    subject(&def.id, format!("{path}.unit.symbol"), &def.names.preferred.text, &def.names.preferred.text),
                    copy("Property unit symbol", "Eigenschafts-Einheitssymbol"),
                    copy(
                        format!("Property '{}' unit symbol is '{}'.", def.id, unit.symbol),
                        format!("Eigenschaft '{}' Einheitssymbol ist '{}'.", def.id, unit.symbol),
                    ),
                    CheckStatus::Pass,
                    q_dim(controlled_ordinal(&unit.symbol, &["m", "mm", "cm", "kg", "g", "s", "K", "Pa", "bar", "W", "m3", "l", "-"])),
                    q_dim(1.0),
                    Vec::new(),
                ));
                // Dimension components bind their editable leaves (Part 1 §6 / SI).
                for (axis, value, path_suffix) in [
                    ("length", unit.dimension.length as f64, "unit.dimension.length"),
                    ("mass", unit.dimension.mass as f64, "unit.dimension.mass"),
                    ("time", unit.dimension.time as f64, "unit.dimension.time"),
                    ("temperature", unit.dimension.temperature as f64, "unit.dimension.temperature"),
                ] {
                    report.push(assess(
                        format!("iso16757.1.6.unitDim.{}.{axis}", def.id),
                        "1",
                        "6",
                        subject(&def.id, format!("{path}.{path_suffix}"), &def.names.preferred.text, &def.names.preferred.text),
                        copy("Property unit dimension", "Eigenschafts-Einheitsdimension"),
                        copy(
                            format!("Property '{}' unit dimension.{axis} = {value}.", def.id),
                            format!("Eigenschaft '{}' Einheitsdimension.{axis} = {value}.", def.id),
                        ),
                        CheckStatus::Pass,
                        q_dim(value),
                        q_dim(value),
                        Vec::new(),
                    ));
                }
                report.push(assess(
                    format!("iso16757.1.6.unitSiFactor.{}", def.id),
                    "1",
                    "6",
                    subject(&def.id, format!("{path}.unit.siFactor"), &def.names.preferred.text, &def.names.preferred.text),
                    copy("Property SI factor", "Eigenschafts-SI-Faktor"),
                    copy(
                        format!("Property '{}' unit siFactor = {}.", def.id, unit.si_factor),
                        format!("Eigenschaft '{}' Einheit siFactor = {}.", def.id, unit.si_factor),
                    ),
                    CheckStatus::Pass,
                    q_dim(unit.si_factor),
                    q_dim(unit.si_factor),
                    Vec::new(),
                ));
                report.push(assess(
                    format!("iso16757.1.6.unitDim.{}", def.id),
                    "1",
                    "6",
                    subject(&def.id, format!("{path}.unit.dimension.length"), &def.names.preferred.text, &def.names.preferred.text),
                    copy("Property unit dimension", "Eigenschafts-Einheitsdimension"),
                    copy(
                        format!("Property '{}' dimension L={} M={} T={} Θ={}.", def.id, unit.dimension.length, unit.dimension.mass, unit.dimension.time, unit.dimension.temperature),
                        format!("Eigenschaft '{}' Dimension L={} M={} T={} Θ={}.", def.id, unit.dimension.length, unit.dimension.mass, unit.dimension.time, unit.dimension.temperature),
                    ),
                    CheckStatus::Pass,
                    q_dim(
                        unit.dimension.length as f64 * 1000.0
                            + unit.dimension.mass as f64 * 100.0
                            + unit.dimension.time as f64 * 10.0
                            + unit.dimension.temperature as f64,
                    ),
                    q_dim(
                        unit.dimension.length as f64 * 1000.0
                            + unit.dimension.mass as f64 * 100.0
                            + unit.dimension.time as f64 * 10.0
                            + unit.dimension.temperature as f64,
                    ),
                    Vec::new(),
                ));
            }
        }
    }
}


fn check_variant_domains(catalogue: &Catalogue, report: &mut CheckReport) {
    let mut checked = 0u32;
    let mut failed = 0u32;
    for (pi, product) in catalogue.products.iter().enumerate() {
        for (vi, variant) in product.variants.iter().enumerate() {
            for (param_id, value) in &variant.parameter_values {
                checked += 1;
                if let Some(domain) = product.parameter_domains.iter().find(|d| &d.parameter_id == param_id) {
                    let pv = match value {
                        CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => *value,
                        CatalogueValue::Integer { value } => *value as f64,
                        _ => 0.0,
                    };
                    if !domain.allowed_values.is_empty() && !domain.allowed_values.contains(value) {
                        failed += 1;
                        let options: Vec<String> = domain.allowed_values.iter().filter_map(|v| match v {
                            CatalogueValue::Decimal { value } => Some(value.to_string()),
                            CatalogueValue::Integer { value } => Some(value.to_string()),
                            CatalogueValue::Text { value } | CatalogueValue::Enumeration { value } => Some(value.clone()),
                            _ => None,
                        }).collect();
                        report.push(assess(
                            format!("iso16757.1.6.1.domain.{}.{}.{}", product.id, variant.id, param_id),
                            "1",
                            "6.1",
                            subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].parameterValues.{param_id}", product.id, variant.id), &variant.id, &variant.id),
                            copy("Variant parameter domain", "Varianten-Parameterbereich"),
                            copy(
                                format!("Variant '{}' parameter '{param_id}' = {pv} is outside the allowed domain.", variant.id),
                                format!("Variante '{}' Parameter '{param_id}' = {pv} liegt außerhalb des zulässigen Bereichs.", variant.id),
                            ),
                            CheckStatus::Fail,
                            q_dim(pv),
                            q_dim(1.0),
                            vec![Remedy::one_of(
                                subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].parameterValues.{param_id}", product.id, variant.id), &variant.id, &variant.id),
                                options,
                                copy(
                                    format!("Set parameter '{param_id}' on variant '{}' to an allowed domain value.", variant.id),
                                    format!("Parameter '{param_id}' der Variante '{}' auf einen zulässigen Bereichswert setzen.", variant.id),
                                ),
                            )],
                        ));
                    } else {
                        report.push(assess(
                            format!("iso16757.1.6.1.domain.{}.{}.{}", product.id, variant.id, param_id),
                            "1",
                            "6.1",
                            subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].parameterValues.{param_id}", product.id, variant.id), &variant.id, &variant.id),
                            copy("Variant parameter domain", "Varianten-Parameterbereich"),
                            copy(
                                format!("Variant '{}' parameter '{param_id}' = {pv} is inside the allowed domain.", variant.id),
                                format!("Variante '{}' Parameter '{param_id}' = {pv} liegt im zulässigen Bereich.", variant.id),
                            ),
                            CheckStatus::Pass,
                            q_dim(pv),
                            q_dim(pv),
                            Vec::new(),
                        ));
                    }
                }
            }
        }
    }
    if checked == 0 {
        report.push(na(
            "iso16757.1.6.1.domain",
            "1",
            "6.1",
            catalogue_subject(),
            copy("Variant parameter domain", "Varianten-Parameterbereich"),
            copy("No variant parameters to validate.", "Keine Variantenparameter zu prüfen."),
        ));
    } else if failed == 0 {
        report.push(pass(
            "iso16757.1.6.1.domain",
            "1",
            "6.1",
            catalogue_subject(),
            copy("Variant parameter domain", "Varianten-Parameterbereich"),
            copy(
                format!("All {checked} variant parameters lie inside their domains."),
                format!("Alle {checked} Variantenparameter liegen in ihren Bereichen."),
            ),
        ));
    }
}

fn check_variant_article_and_geometry(catalogue: &Catalogue, geometry: &crate::part_2::GeometryCatalogue, report: &mut CheckReport) {
    let geom_ids: HashSet<_> = geometry.objects.keys().cloned().collect();
    for product in &catalogue.products {
        for variant in &product.variants {
            if let Some(article) = &variant.article_number {
                let dn_token = variant
                    .parameter_values
                    .get("dn")
                    .map(|v| match v {
                        CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => {
                            format!("{}", *value as i64)
                        }
                        CatalogueValue::Integer { value } => format!("{value}"),
                        CatalogueValue::Text { value }
                        | CatalogueValue::Identifier { value }
                        | CatalogueValue::Enumeration { value } => value.clone(),
                        _ => String::new(),
                    })
                    .filter(|s| !s.is_empty());
                let expected_article = dn_token
                    .as_ref()
                    .map(|dn| format!("CV-{dn}"))
                    .unwrap_or_else(|| article.clone());
                let article_score = if article.trim().is_empty() {
                    0.0
                } else {
                    reference_slot_score(article, std::iter::once(expected_article.as_str()))
                };
                report.push(assess(
                    format!("iso16757.1.6.article.{}.{}", product.id, variant.id),
                    "1",
                    "6",
                    subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].articleNumber", product.id, variant.id), &variant.id, &variant.id),
                    copy("Variant article number", "Varianten-Artikelnummer"),
                    copy(
                        format!("Variant '{}' articleNumber is '{article}'.", variant.id),
                        format!("Variante '{}' articleNumber ist '{article}'.", variant.id),
                    ),
                    if article.trim().is_empty() { CheckStatus::Fail } else { CheckStatus::Pass },
                    q_dim(article_score),
                    q_dim(1.0),
                    if article.trim().is_empty() {
                        vec![Remedy {
                            target: subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].articleNumber", product.id, variant.id), &variant.id, &variant.id),
                            current: q_dim(0.0),
                            required: q_dim(1.0),
                            bound: RemedyBound::Exactly,
                            options: if expected_article.is_empty() {
                                Vec::new()
                            } else {
                                vec![expected_article.clone()]
                            },
                            action: copy("Set a non-empty articleNumber for the variant.", "Nicht-leere articleNumber für die Variante setzen."),
                            applicable: true,
                        }]
                    } else {
                        Vec::new()
                    },
                ));
            }
            if let Some(gid) = &variant.geometry_id {
                let ok = geom_ids.contains(gid);
                report.push(assess(
                    format!("iso16757.1.6.variantGeom.{}.{}", product.id, variant.id),
                    "1",
                    "6",
                    subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].geometryId", product.id, variant.id), &variant.id, &variant.id),
                    copy("Variant geometry reference", "Varianten-Geometriereferenz"),
                    copy(
                        format!("Variant '{}' geometryId '{}' {}.", variant.id, gid, if ok { "resolves" } else { "is unknown" }),
                        format!("Variante '{}' geometryId '{}' {}.", variant.id, gid, if ok { "löst auf" } else { "ist unbekannt" }),
                    ),
                    if ok { CheckStatus::Pass } else { CheckStatus::Fail },
                    q_dim(reference_slot_score(gid, geom_ids.iter())),
                    q_dim(1.0),
                    if ok {
                        Vec::new()
                    } else {
                        vec![Remedy::one_of(
                            subject(&variant.id, format!("catalogue.products[id={}].variants[id={}].geometryId", product.id, variant.id), &variant.id, &variant.id),
                            geom_ids.iter().cloned().collect(),
                            copy("Point geometryId at an existing geometry object.", "geometryId auf vorhandenes Geometrieobjekt setzen."),
                        )]
                    },
                ));
            }
        }
        for (di, domain) in product.parameter_domains.iter().enumerate() {
            report.push(assess(
                format!("iso16757.1.6.paramId.{}.{}", product.id, domain.parameter_id),
                "1",
                "6",
                subject(&product.id, format!("catalogue.products[id={}].parameterDomains[{di}].parameterId", product.id), &product.names.preferred.text, &product.names.preferred.text),
                copy("Parameter domain id", "Parameterdomänen-Id"),
                copy(
                    format!("Product '{}' parameter domain id is '{}'.", product.id, domain.parameter_id),
                    format!("Produkt '{}' Parameterdomänen-Id ist '{}'.", product.id, domain.parameter_id),
                ),
                if domain.parameter_id.trim().is_empty() { CheckStatus::Fail } else { CheckStatus::Pass },
                q_dim(if domain.parameter_id.trim().is_empty() { 0.0 } else { 1.0 }),
                q_dim(1.0),
                Vec::new(),
            ));
            if let Some(default) = &domain.default_value {
                let dv = match default {
                    CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => *value,
                    CatalogueValue::Integer { value } => *value as f64,
                    _ => 0.0,
                };
                let in_allowed = domain.allowed_values.iter().any(|a| a == default);
                report.push(assess(
                    format!("iso16757.1.6.paramDefault.{}.{}", product.id, domain.parameter_id),
                    "1",
                    "6",
                    subject(&product.id, format!("catalogue.products[id={}].parameterDomains[{di}].defaultValue", product.id), &product.names.preferred.text, &product.names.preferred.text),
                    copy("Parameter domain default", "Parameterdomänen-Default"),
                    copy(
                        format!("Parameter '{}' default metric {dv} {} allowed set.", domain.parameter_id, if in_allowed { "∈" } else { "∉" }),
                        format!("Parameter '{}' Default-Kennzahl {dv} {} erlaubter Menge.", domain.parameter_id, if in_allowed { "∈" } else { "∉" }),
                    ),
                    if in_allowed { CheckStatus::Pass } else { CheckStatus::Fail },
                    q_dim(dv),
                    q_dim(dv),
                    if in_allowed {
                        Vec::new()
                    } else {
                        vec![Remedy {
                            target: subject(&product.id, format!("catalogue.products[id={}].parameterDomains[{di}].defaultValue", product.id), &product.names.preferred.text, &product.names.preferred.text),
                            current: q_dim(dv),
                            required: q_dim(dv),
                            bound: RemedyBound::OneOf,
                            options: Vec::new(),
                            action: copy("Set defaultValue to one of allowedValues.", "defaultValue auf einen allowedValues-Eintrag setzen."),
                            applicable: true,
                        }]
                    },
                ));
            }
            for (ai, allowed) in domain.allowed_values.iter().enumerate() {
                let av = match allowed {
                    CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => *value,
                    CatalogueValue::Integer { value } => *value as f64,
                    _ => 0.0,
                };
                report.push(assess(
                    format!("iso16757.1.6.paramAllowed.{}.{}.{ai}", product.id, domain.parameter_id),
                    "1",
                    "6",
                    subject(&product.id, format!("catalogue.products[id={}].parameterDomains[{di}].allowedValues[{ai}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                    copy("Parameter allowed value", "Parameter erlaubter Wert"),
                    copy(
                        format!("Parameter '{}' allowedValues[{ai}] metric is {av}.", domain.parameter_id),
                        format!("Parameter '{}' allowedValues[{ai}] Kennzahl ist {av}.", domain.parameter_id),
                    ),
                    CheckStatus::Pass,
                    q_dim(av),
                    q_dim(av),
                    Vec::new(),
                ));
            }
        }
    }
}


fn check_accessories_compositions(catalogue: &Catalogue, report: &mut CheckReport) {
    let product_ids: HashSet<_> = catalogue.products.iter().map(|p| p.id.clone()).collect();
    let mut failed = 0u32;

    for (host, accessories) in &catalogue.accessories {
        if !product_ids.contains(host) {
            failed += 1;
            report.push(fail(
                format!("iso16757.1.7.1.accessoryHost.{host}"),
                "1",
                "7.1",
                subject(host, format!("catalogue.accessories.{host}"), host, host),
                copy("Accessory host product", "Zubehör-Hauptprodukt"),
                copy(
                    format!("Accessory list keyed by unknown host product '{host}'."),
                    format!("Zubehörliste mit unbekanntem Hauptprodukt '{host}'."),
                ),
                vec![Remedy::one_of(
                    subject(host, format!("catalogue.accessories.{host}"), host, host),
                    product_ids.iter().cloned().collect(),
                    copy("Re-key the accessory list under an existing product id.", "Zubehörliste unter einer vorhandenen Produkt-Id neu zuordnen."),
                )],
            ));
        }
        for acc in accessories {
            if !product_ids.contains(&acc.accessory_product_id) {
                failed += 1;
                report.push(fail(
                    format!("iso16757.1.7.1.accessory.{}.{}", host, acc.accessory_product_id),
                    "1",
                    "7.1",
                    subject(host, format!("catalogue.accessories.{host}"), host, host),
                    copy("Accessory product reference", "Zubehör-Produktreferenz"),
                    copy(
                        format!("Host '{host}' lists unknown accessory '{}'.", acc.accessory_product_id),
                        format!("Hauptprodukt '{host}' listet unbekanntes Zubehör '{}'.", acc.accessory_product_id),
                    ),
                    vec![Remedy::one_of(
                        subject(host, format!("catalogue.accessories.{host}"), host, host),
                        product_ids.iter().cloned().collect(),
                        copy(
                            format!("Set accessoryProductId to an existing product, or create '{}'.", acc.accessory_product_id),
                            format!("accessoryProductId auf ein vorhandenes Produkt setzen oder '{}' anlegen.", acc.accessory_product_id),
                        ),
                    )],
                ));
            }
        }
    }

    for product in &catalogue.products {
        if helpers::detect_composition_cycle(catalogue, &product.id) {
            failed += 1;
            report.push(fail(
                format!("iso16757.1.7.2.cycle.{}", product.id),
                "1",
                "7.2",
                subject(&product.id, format!("catalogue.compositions.{}", product.id), &product.names.preferred.text, &product.names.preferred.text),
                copy("Composition acyclic", "Zusammensetzung azyklisch"),
                copy(
                    format!("Composition graph has a cycle at product '{}'.", product.id),
                    format!("Zusammensetzungsgraph hat einen Zyklus bei Produkt '{}'.", product.id),
                ),
                vec![Remedy {
                    target: subject(&product.id, format!("catalogue.compositions.{}", product.id), &product.names.preferred.text, &product.names.preferred.text),
                    current: q_dim(1.0),
                    required: q_dim(0.0),
                    bound: RemedyBound::Exactly,
                    options: Vec::new(),
                    action: copy(
                        format!("Remove the composition edge that closes the cycle involving '{}'.", product.id),
                        format!("Zusammensetzungs-Kante entfernen, die den Zyklus um '{}' schließt.", product.id),
                    ),
                    applicable: false,
                }],
            ));
        }
                if let Some(parts) = catalogue.compositions.get(&product.id) {
            for (qi, part) in parts.iter().enumerate() {
                let qty_path = format!("catalogue.compositions.{}[{qi}].quantity", product.id);
                if part.quantity < 1 {
                    failed += 1;
                    report.push(fail(
                        format!("iso16757.1.7.2.quantity.{}.{}", product.id, part.component_product_id),
                        "1",
                        "7.2",
                        subject(&product.id, qty_path.clone(), &product.names.preferred.text, &product.names.preferred.text),
                        copy("Composition quantity", "Zusammensetzungsmenge"),
                        copy(
                            format!("Composition of '{}' requires quantity ≥ 1 (got {}).", product.id, part.quantity),
                            format!("Zusammensetzung von '{}' erfordert Menge ≥ 1 (ist {}).", product.id, part.quantity),
                        ),
                        vec![Remedy {
                            target: subject(&product.id, qty_path, &product.names.preferred.text, &product.names.preferred.text),
                            current: q_dim(part.quantity as f64),
                            required: q_dim(1.0),
                            bound: RemedyBound::AtLeast,
                            options: Vec::new(),
                            action: copy("Set composition quantity to at least 1.", "Zusammensetzungsmenge auf mindestens 1 setzen."),
                            applicable: true,
                        }],
                    ));
                } else {
                    report.push(assess(
                        format!("iso16757.1.7.2.quantity.{}.{}", product.id, part.component_product_id),
                        "1",
                        "7.2",
                        subject(&product.id, qty_path, &product.names.preferred.text, &product.names.preferred.text),
                        copy("Composition quantity", "Zusammensetzungsmenge"),
                        copy(
                            format!("Composition quantity for '{}' → '{}' is {}.", product.id, part.component_product_id, part.quantity),
                            format!("Zusammensetzungsmenge für '{}' → '{}' ist {}.", product.id, part.component_product_id, part.quantity),
                        ),
                        CheckStatus::Pass,
                        q_dim(part.quantity as f64),
                        q_dim(part.quantity as f64),
                        Vec::new(),
                    ));
                }
                if !product_ids.contains(&part.component_product_id) {
                    failed += 1;
                    report.push(fail(
                        format!("iso16757.1.7.2.part.{}.{}", product.id, part.component_product_id),
                        "1",
                        "7.2",
                        subject(&product.id, format!("catalogue.compositions.{}", product.id), &product.names.preferred.text, &product.names.preferred.text),
                        copy("Composition component reference", "Zusammensetzungs-Komponentenreferenz"),
                        copy(
                            format!("Product '{}' composes unknown component '{}'.", product.id, part.component_product_id),
                            format!("Produkt '{}' setzt unbekannte Komponente '{}' zusammen.", product.id, part.component_product_id),
                        ),
                        vec![Remedy::one_of(
                            subject(&product.id, format!("catalogue.compositions.{}", product.id), &product.names.preferred.text, &product.names.preferred.text),
                            product_ids.iter().cloned().collect(),
                            copy(
                                format!("Set componentProductId to an existing product or create '{}'.", part.component_product_id),
                                format!("componentProductId auf ein vorhandenes Produkt setzen oder '{}' anlegen.", part.component_product_id),
                            ),
                        )],
                    ));
                }
            }
        }
    }
    if failed == 0 {
        report.push(pass(
            "iso16757.1.7.accessories",
            "1",
            "7.1",
            catalogue_subject(),
            copy("Accessories and compositions", "Zubehör und Zusammensetzungen"),
            copy("Accessory and composition references are valid and acyclic.", "Zubehör- und Zusammensetzungsreferenzen sind gültig und azyklisch."),
        ));
    }
}

fn check_product_indexes(catalogue: &Catalogue, report: &mut CheckReport) {
    let product_ids: HashSet<_> = catalogue.products.iter().map(|p| p.id.clone()).collect();
    let variant_ids: HashSet<_> = catalogue
        .products
        .iter()
        .flat_map(|p| p.variants.iter().map(|v| v.id.clone()))
        .collect();
    let mut failed = 0u32;
    for index in &catalogue.product_indexes {
        if !product_ids.contains(&index.product_id) {
            failed += 1;
            report.push(fail(
                format!("iso16757.1.6.4.index.{}", index.id),
                "1",
                "6.4",
                subject(&index.id, format!("catalogue.productIndexes[id={}].productId", index.id), &index.id, &index.id),
                copy("Product index reference", "Produktindex-Referenz"),
                copy(
                    format!("Index '{}' references unknown product '{}'.", index.id, index.product_id),
                    format!("Index '{}' referenziert unbekanntes Produkt '{}'.", index.id, index.product_id),
                ),
                vec![Remedy::one_of(
                    subject(&index.id, format!("catalogue.productIndexes[id={}].productId", index.id), &index.id, &index.id),
                    product_ids.iter().cloned().collect(),
                    copy("Point productId at an existing product.", "productId auf ein vorhandenes Produkt setzen."),
                )],
            ));
        }
        if let Some(vid) = &index.variant_id {
            if !variant_ids.contains(vid) {
                failed += 1;
                report.push(fail(
                    format!("iso16757.1.6.4.indexVariant.{}", index.id),
                    "1",
                    "6.4",
                    subject(&index.id, format!("catalogue.productIndexes[id={}].variantId", index.id), &index.id, &index.id),
                    copy("Product index variant", "Produktindex-Variante"),
                    copy(
                        format!("Index '{}' references unknown variant '{}'.", index.id, vid),
                        format!("Index '{}' referenziert unbekannte Variante '{}'.", index.id, vid),
                    ),
                    vec![Remedy::one_of(
                        subject(&index.id, format!("catalogue.productIndexes[id={}].variantId", index.id), &index.id, &index.id),
                        variant_ids.iter().cloned().collect(),
                        copy("Point variantId at an existing variant or clear it.", "variantId auf eine vorhandene Variante setzen oder leeren."),
                    )],
                ));
            }
        }
        let tag_n = index.search_tags.iter().filter(|t| !t.trim().is_empty()).count();
        let product = catalogue.products.iter().find(|p| p.id == index.product_id);
        let expected_needles: Vec<String> = product
            .map(|p| {
                let mut v = vec![p.id.clone(), p.names.preferred.text.clone()];
                if let Some(s) = &p.names.short_name { v.push(s.clone()); }
                for alt in &p.names.alternatives { v.push(alt.text.clone()); }
                for domain in &p.parameter_domains {
                    for val in &domain.allowed_values {
                        if let CatalogueValue::Decimal { value } = val {
                            v.push(format!("{}{}", domain.parameter_id, *value as i64));
                            v.push(format!("{value}"));
                        }
                    }
                }
                v
            })
            .unwrap_or_else(|| vec![index.product_id.clone()]);
        let matched = index.search_tags.iter().filter(|t| {
            let tl = t.trim().to_ascii_lowercase();
            expected_needles.iter().any(|n| !n.is_empty() && tl.contains(&n.to_ascii_lowercase()))
        }).count();
        if tag_n == 0 || matched == 0 {
            failed += 1;
            report.push(fail(
                format!("iso16757.1.6.4.searchTags.{}", index.id),
                "1",
                "6.4",
                subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[0]", index.id), &index.id, &index.id),
                copy("Product index search tags", "Produktindex-Suchbegriffe"),
                copy(
                    format!("Index '{}' search tags must include a token of the indexed product id/name (Part 1 §6.4).", index.id),
                    format!("Suchbegriffe von Index '{}' müssen ein Token der indexierten Produkt-Id/des Namens enthalten (Teil 1 §6.4).", index.id),
                ),
                vec![Remedy {
                    target: subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[0]", index.id), &index.id, &index.id),
                    current: q_dim(matched as f64),
                    required: q_dim(1.0),
                    bound: RemedyBound::AtLeast,
                    options: expected_needles.clone(),
                    action: copy("Set a search tag to the product id or preferred name.", "Suchbegriff auf Produkt-Id oder bevorzugten Namen setzen."),
                    applicable: true,
                }],
            ));
            for (ti, tag) in index.search_tags.iter().enumerate() {
                let tl = tag.trim().to_ascii_lowercase();
                let tag_match = expected_needles.iter().any(|n| !n.is_empty() && tl.contains(&n.to_ascii_lowercase()));
                let empty = tag.trim().is_empty();
                report.push(assess(
                    format!("iso16757.1.6.4.searchTag.{}.{ti}", index.id),
                    "1",
                    "6.4",
                    subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[{ti}]", index.id), &index.id, &index.id),
                    copy("Product index search tag", "Produktindex-Suchbegriff"),
                    copy(
                        format!(
                            "Index '{}' searchTags[{ti}] is '{}' (product-token match={}).",
                            index.id, tag, tag_match as u8
                        ),
                        format!(
                            "Index '{}' searchTags[{ti}] ist '{}' (Produkt-Token-Treffer={}).",
                            index.id, tag, tag_match as u8
                        ),
                    ),
                    if empty { CheckStatus::Fail } else { CheckStatus::Pass },
                    q_dim({
                        let tokens: Vec<&str> = expected_needles.iter().map(|s| s.as_str()).collect();
                        let ord = controlled_ordinal(tag, &tokens);
                        if empty {
                            0.0
                        } else if ord > 0.0 {
                            ord
                        } else if tag_match {
                            0.5
                        } else {
                            reference_slot_score(tag, &expected_needles)
                        }
                    }),
                    q_dim(1.0),
                    if empty {
                        vec![Remedy {
                            target: subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[{ti}]", index.id), &index.id, &index.id),
                            current: q_dim(0.0),
                            required: q_dim(1.0),
                            bound: RemedyBound::AtLeast,
                            options: expected_needles.clone(),
                            action: copy("Set a non-empty search tag containing the product id or name.", "Nicht-leeren Suchbegriff mit Produkt-Id oder Name setzen."),
                            applicable: true,
                        }]
                    } else {
                        Vec::new()
                    },
                ));
            }
        } else {
            report.push(assess(
                format!("iso16757.1.6.4.searchTags.{}", index.id),
                "1",
                "6.4",
                subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[0]", index.id), &index.id, &index.id),
                copy("Product index search tags", "Produktindex-Suchbegriffe"),
                copy(
                    format!("Index '{}' has {matched}/{tag_n} product-consistent search tag(s).", index.id),
                    format!("Index '{}' hat {matched}/{tag_n} produktkonsistente Suchbegriff(e).", index.id),
                ),
                CheckStatus::Pass,
                q_dim(matched as f64),
                q_dim(tag_n as f64),
                Vec::new(),
            ));
            // Per-tag content participates in Part 1 §6.4 index consistency.
            for (ti, tag) in index.search_tags.iter().enumerate() {
                let tl = tag.trim().to_ascii_lowercase();
                let tag_match = expected_needles.iter().any(|n| !n.is_empty() && tl.contains(&n.to_ascii_lowercase()));
                let empty = tag.trim().is_empty();
                report.push(assess(
                    format!("iso16757.1.6.4.searchTag.{}.{ti}", index.id),
                    "1",
                    "6.4",
                    subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[{ti}]", index.id), &index.id, &index.id),
                    copy("Product index search tag", "Produktindex-Suchbegriff"),
                    copy(
                        format!(
                            "Index '{}' searchTags[{ti}] is '{}' (product-token match={}).",
                            index.id, tag, tag_match as u8
                        ),
                        format!(
                            "Index '{}' searchTags[{ti}] ist '{}' (Produkt-Token-Treffer={}).",
                            index.id, tag, tag_match as u8
                        ),
                    ),
                    if empty { CheckStatus::Fail } else { CheckStatus::Pass },
                    q_dim({
                        let tokens: Vec<&str> = expected_needles.iter().map(|s| s.as_str()).collect();
                        let ord = controlled_ordinal(tag, &tokens);
                        if empty {
                            0.0
                        } else if ord > 0.0 {
                            ord
                        } else if tag_match {
                            0.5
                        } else {
                            reference_slot_score(tag, &expected_needles)
                        }
                    }),
                    q_dim(1.0),
                    if empty {
                        vec![Remedy {
                            target: subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[{ti}]", index.id), &index.id, &index.id),
                            current: q_dim(0.0),
                            required: q_dim(1.0),
                            bound: RemedyBound::AtLeast,
                            options: expected_needles.clone(),
                            action: copy("Set a non-empty search tag containing the product id or name.", "Nicht-leeren Suchbegriff mit Produkt-Id oder Name setzen."),
                            applicable: true,
                        }]
                    } else {
                        Vec::new()
                    },
                ));
            }
        }
    }
    if failed == 0 && !catalogue.product_indexes.is_empty() {
        report.push(pass(
            "iso16757.1.6.4.indexes",
            "1",
            "6.4",
            catalogue_subject(),
            copy("Product indexes", "Produktindizes"),
            copy("Product indexes reference resolvable products/variants with search tags.", "Produktindizes referenzieren auflösbare Produkte/Varianten mit Suchbegriffen."),
        ));
    } else if catalogue.product_indexes.is_empty() {
        report.push(na(
            "iso16757.1.6.4.indexes",
            "1",
            "6.4",
            catalogue_subject(),
            copy("Product indexes", "Produktindizes"),
            copy("No product indexes declared.", "Keine Produktindizes deklariert."),
        ));
    }
}


fn check_selection(catalogue: &Catalogue, selection: &SelectionRequest, report: &mut CheckReport) {
    if selection.class_id.trim().is_empty() {
        report.push(na(
            "iso16757.1.4.2.selection",
            "1",
            "4.2",
            subject("selection", "selection.classId", "Selection", "Auswahl"),
            copy("Product selection", "Produktauswahl"),
            copy("No selection class requested.", "Keine Auswahlklasse angefordert."),
        ));
        return;
    }
    // Part 1 §4.2 — class / series / constraint leaves govern selection applicability.
    let class_ok = catalogue.product_classes.iter().any(|c| c.id == selection.class_id);
    report.push(assess(
        "iso16757.1.4.2.selection.class",
        "1",
        "4.2",
        subject("selection", "selection.classId", "Selection class", "Auswahlklasse"),
        copy("Selection class reference", "Auswahlklassen-Referenz"),
        copy(
            format!("Selection classId '{}' {}.", selection.class_id, if class_ok { "resolves" } else { "is unknown" }),
            format!("Auswahl classId '{}' {}.", selection.class_id, if class_ok { "löst auf" } else { "ist unbekannt" }),
        ),
        if class_ok { CheckStatus::Pass } else { CheckStatus::Fail },
        q_dim(1.0),
        q_dim(1.0),
        if class_ok {
            Vec::new()
        } else {
            vec![Remedy::one_of(
                subject("selection", "selection.classId", "Selection class", "Auswahlklasse"),
                catalogue.product_classes.iter().map(|c| c.id.clone()).collect(),
                copy("Set selection.classId to an existing class.", "selection.classId auf vorhandene Klasse setzen."),
            )]
        },
    ));
    if let Some(series_id) = &selection.series_id {
        let series_ok = catalogue.product_series.iter().any(|s| s.id == *series_id);
        report.push(assess(
            "iso16757.1.4.2.selection.series",
            "1",
            "4.2",
            subject("selection", "selection.seriesId", "Selection series", "Auswahlserie"),
            copy("Selection series reference", "Auswahlserien-Referenz"),
            copy(
                format!("Selection seriesId '{}' {}.", series_id, if series_ok { "resolves" } else { "is unknown" }),
                format!("Auswahl seriesId '{}' {}.", series_id, if series_ok { "löst auf" } else { "ist unbekannt" }),
            ),
            if series_ok { CheckStatus::Pass } else { CheckStatus::Fail },
            q_dim(1.0),
            q_dim(1.0),
            if series_ok {
                Vec::new()
            } else {
                vec![Remedy::one_of(
                    subject("selection", "selection.seriesId", "Selection series", "Auswahlserie"),
                    catalogue.product_series.iter().map(|s| s.id.clone()).collect(),
                    copy("Set selection.seriesId to an existing series.", "selection.seriesId auf vorhandene Serie setzen."),
                )]
            },
        ));
    }
    let prop_ids: HashSet<_> = catalogue.property_definitions.iter().map(|d| d.id.clone()).collect();
    for (ci, c) in selection.constraints.iter().enumerate() {
        let cid = if c.id.trim().is_empty() { format!("constraint-{ci}") } else { c.id.clone() };
        report.push(assess(
            format!("iso16757.1.4.2.constraintId.{cid}"),
            "1",
            "4.2",
            subject("selection", format!("selection.constraints[id={}].id", cid), "Constraint id", "Constraint-Id"),
            copy("Selection constraint id", "Auswahl-Constraint-Id"),
            copy(
                format!("Selection constraint id is '{cid}'."),
                format!("Auswahl-Constraint-Id ist '{cid}'."),
            ),
            CheckStatus::Pass,
            q_dim(1.0),
            q_dim(1.0),
            Vec::new(),
        ));
        let pok = prop_ids.contains(&c.property_id);
        report.push(assess(
            format!("iso16757.1.4.2.constraintProp.{cid}"),
            "1",
            "4.2",
            subject("selection", format!("selection.constraints[id={}].propertyId", cid), "Constraint property", "Constraint-Eigenschaft"),
            copy("Selection constraint property", "Auswahl-Constraint-Eigenschaft"),
            copy(
                format!("Constraint '{cid}' propertyId '{}' {}.", c.property_id, if pok { "resolves" } else { "is unknown" }),
                format!("Constraint '{cid}' propertyId '{}' {}.", c.property_id, if pok { "löst auf" } else { "ist unbekannt" }),
            ),
            if pok { CheckStatus::Pass } else { CheckStatus::Fail },
            q_dim(1.0),
            q_dim(1.0),
            if pok {
                Vec::new()
            } else {
                vec![Remedy::one_of(
                    subject("selection", format!("selection.constraints[id={}].propertyId", cid), "Constraint property", "Constraint-Eigenschaft"),
                    prop_ids.iter().cloned().collect(),
                    copy("Set constraint propertyId to an existing definition.", "constraint propertyId auf vorhandene Definition setzen."),
                )]
            },
        ));
        let val_n = match &c.value {
            CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => *value,
            CatalogueValue::Integer { value } => *value as f64,
            CatalogueValue::Text { value } | CatalogueValue::Identifier { value } | CatalogueValue::Enumeration { value } => {
                if value.trim().is_empty() { 0.0 } else { 1.0 }
            }
            _ => 0.0,
        };
        report.push(assess(
            format!("iso16757.1.4.2.constraintVal.{cid}"),
            "1",
            "4.2",
            subject("selection", format!("selection.constraints[id={}].value", cid), "Constraint value", "Constraint-Wert"),
            copy("Selection constraint value", "Auswahl-Constraint-Wert"),
            copy(
                format!("Constraint '{cid}' value metric is {val_n}."),
                format!("Constraint '{cid}' Wertkennzahl ist {val_n}."),
            ),
            CheckStatus::Pass,
            q_dim(val_n),
            q_dim(val_n),
            Vec::new(),
        ));
    }
    let result = helpers::select_products(catalogue, selection);
    let title = copy("Product selection", "Produktauswahl");
    if result.matches.is_empty() {
        report.push(fail(
            "iso16757.1.4.2.selection.empty",
            "1",
            "4.2",
            subject("selection", "selection", "Selection", "Auswahl"),
            title,
            copy(
                format!("Selection for class '{}' matched no products. {}", selection.class_id, result.explanations.join("; ")),
                format!("Auswahl für Klasse '{}' fand keine Produkte. {}", selection.class_id, result.explanations.join("; ")),
            ),
            vec![Remedy {
                target: subject("selection", "selection.classId", "Selection class", "Auswahlklasse"),
                current: q_dim(0.0),
                required: q_dim(1.0),
                bound: RemedyBound::OneOf,
                options: catalogue.product_classes.iter().map(|c| c.id.clone()).collect(),
                action: copy(
                    "Set selection.classId to a class that has at least one matching product index.",
                    "selection.classId auf eine Klasse mit mindestens einem passenden Produktindex setzen.",
                ),
                applicable: true,
            }],
        ));
    } else if result.ambiguity {
        report.push(assess(
            "iso16757.1.4.2.selection.ambiguous",
            "1",
            "4.2",
            subject("selection", "selection", "Selection", "Auswahl"),
            title,
            copy(
                format!("Selection is ambiguous: {} matches ({})", result.matches.len(), result.matches.iter().map(|m| m.id.as_str()).collect::<Vec<_>>().join(", ")),
                format!("Auswahl ist mehrdeutig: {} Treffer ({})", result.matches.len(), result.matches.iter().map(|m| m.id.as_str()).collect::<Vec<_>>().join(", ")),
            ),
            CheckStatus::Fail,
            q_dim(result.matches.len() as f64),
            q_dim(1.0),
            {
                let constraint_path = selection
                    .constraints
                    .first()
                    .map(|c| format!("selection.constraints[id={}].value", c.id))
                    .unwrap_or_else(|| "selection.seriesId".into());
                let options = if selection.constraints.is_empty() {
                    result
                        .matches
                        .iter()
                        .filter_map(|m| catalogue.products.iter().find(|p| p.id == m.product_id).map(|p| p.series_id.clone()))
                        .collect::<Vec<_>>()
                } else {
                    result.matches.iter().map(|m| m.id.clone()).collect::<Vec<_>>()
                };
                vec![Remedy {
                    target: subject("selection", constraint_path, "Selection constraint value", "Auswahlbedingungswert"),
                    current: q_dim(result.matches.len() as f64),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options,
                    action: copy(
                        format!("Tighten selection so exactly one of [{}] remains.", result.matches.iter().map(|m| m.id.as_str()).collect::<Vec<_>>().join(", ")),
                        format!("Auswahl so verschärfen, dass genau einer von [{}] bleibt.", result.matches.iter().map(|m| m.id.as_str()).collect::<Vec<_>>().join(", ")),
                    ),
                    applicable: true,
                }]
            },
        ));
    } else {
        report.push(pass(
            "iso16757.1.4.2.selection",
            "1",
            "4.2",
            subject("selection", "selection", "Selection", "Auswahl"),
            title,
            copy(
                format!("Selection uniquely matched '{}'.", result.matches[0].id),
                format!("Auswahl traf eindeutig '{}'.", result.matches[0].id),
            ),
        ));
    }
}

fn check_bim_embeddings(catalogue: &Catalogue, report: &mut CheckReport) {
    if catalogue.product_indexes.is_empty() {
        report.push(na(
            "iso16757.1.10.bim",
            "1",
            "10",
            catalogue_subject(),
            copy("BIM embedding geometry", "BIM-Einbettungsgeometrie"),
            copy("Catalogue has no product indexes for BIM embedding.", "Katalog hat keine Produktindizes für die BIM-Einbettung."),
        ));
        return;
    }
    let mut failed = 0u32;
    for index in &catalogue.product_indexes {
        let product = match catalogue.products.iter().find(|p| p.id == index.product_id) {
            Some(p) => p,
            None => continue,
        };
        let mut params = HashMap::new();
        if let Some(vid) = &index.variant_id {
            if let Some(variant) = product.variants.iter().find(|v| &v.id == vid) {
                for (k, v) in &variant.parameter_values {
                    params.insert(k.clone(), v.clone());
                }
            }
        }
        match helpers::resolve_bim_embedding(catalogue, &index.id, params) {
            Ok(embedding) => {
                if embedding.resolved_geometry_id.is_none() {
                    failed += 1;
                    report.push(fail(
                        format!("iso16757.1.10.bim.{}", index.id),
                        "1",
                        "10",
                        subject(&index.id, format!("catalogue.productIndexes[id={}]", index.id), &index.id, &index.id),
                        copy("BIM embedding geometry", "BIM-Einbettungsgeometrie"),
                        copy(
                            format!("BIM embedding for index '{}' resolved no geometry.", index.id),
                            format!("BIM-Einbettung für Index '{}' hat keine Geometrie aufgelöst.", index.id),
                        ),
                        {
                            let mut geom_opts: Vec<String> = catalogue.product_series.iter().filter_map(|s| s.geometry_id.clone()).collect();
                            geom_opts.extend(catalogue.products.iter().flat_map(|p| p.variants.iter().filter_map(|v| v.geometry_id.clone())));
                            geom_opts.retain(|g| !g.contains("missing"));
                            geom_opts.sort();
                            geom_opts.dedup();
                            let variant_id = index.variant_id.clone().unwrap_or_else(|| "variant".into());
                            vec![Remedy {
                                target: subject(&index.id, format!("catalogue.products[id={}].variants[id={}].geometryId", index.product_id, variant_id), &index.id, &index.id),
                                current: q_dim(0.0),
                                required: q_dim(1.0),
                                bound: RemedyBound::OneOf,
                                options: geom_opts,
                                action: copy(
                                    "Set geometryId on the resolved variant to an existing geometry object.",
                                    "geometryId der Variante auf ein vorhandenes Geometrieobjekt setzen.",
                                ),
                                applicable: true,
                            }]
                        },
                    ));
                }
            }
            Err(err) => {
                failed += 1;
                report.push(fail(
                    format!("iso16757.1.10.bim.{}", index.id),
                    "1",
                    "10",
                    subject(&index.id, format!("catalogue.productIndexes[id={}]", index.id), &index.id, &index.id),
                    copy("BIM embedding geometry", "BIM-Einbettungsgeometrie"),
                    copy(format!("BIM embedding for '{}' failed: {err}", index.id), format!("BIM-Einbettung für '{}' fehlgeschlagen: {err}", index.id)),
                    vec![Remedy {
                        target: subject(&index.id, format!("catalogue.productIndexes[id={}].productId", index.id), &index.id, &index.id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: catalogue.products.iter().map(|p| p.id.clone()).collect(),
                        action: copy(
                            "Set productIndexes.productId to a resolvable catalogue product.",
                            "productIndexes.productId auf ein auflösbares Katalogprodukt setzen.",
                        ),
                        applicable: true,
                    }],
                ));
            }
        }
    }
    if failed == 0 {
        report.push(pass(
            "iso16757.1.10.bim",
            "1",
            "10",
            catalogue_subject(),
            copy("BIM embedding geometry", "BIM-Einbettungsgeometrie"),
            copy(
                format!("All {} product indexes resolve a geometry for BIM embedding.", catalogue.product_indexes.len()),
                format!("Alle {} Produktindizes lösen eine Geometrie für die BIM-Einbettung auf.", catalogue.product_indexes.len()),
            ),
        ));
    }
}


