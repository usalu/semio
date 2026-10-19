//! 🪪️ ISO 16757-1 header, dictionary alignment, unique id and referential integrity checks.

use super::*;

pub(super) fn check_header(catalogue: &Catalogue, required: &[String], report: &mut CheckReport) {
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
    let allowed_status = semio_s_artifact_norm_iso16757_checks_common::part_libraries::part_1::LIFECYCLE_STATUSES;
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

pub(super) fn check_dictionary_catalogue_alignment(doc: &Iso16757Snapshot, report: &mut CheckReport) {
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

pub(super) fn check_unique_ids(catalogue: &Catalogue, report: &mut CheckReport) {
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

pub(super) fn check_referential_integrity(catalogue: &Catalogue, geometry: &semio_s_artifact_norm_iso16757_model::part_2::GeometryCatalogue, report: &mut CheckReport) {
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
