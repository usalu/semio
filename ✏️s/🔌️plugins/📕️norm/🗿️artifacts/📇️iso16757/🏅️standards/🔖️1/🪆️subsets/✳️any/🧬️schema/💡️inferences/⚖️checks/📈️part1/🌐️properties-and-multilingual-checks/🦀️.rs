//! 🌐️ ISO 16757-1 group dictionary subjects, multilingual texts, required properties, property values, units and variant domains.

use super::*;

pub(super) fn check_group_dictionary_subjects(catalogue: &Catalogue, dictionary: &semio_s_artifact_norm_iso16757_model::part_4::Dictionary, report: &mut CheckReport) {
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

pub(super) fn check_multilingual(catalogue: &Catalogue, required: &[String], report: &mut CheckReport) {
    let mut named: Vec<(String, String, &semio_s_artifact_norm_iso16757_model::Names)> = Vec::new();
    named.push(("catalogue".into(), "catalogue.metadata.names".into(), &catalogue.metadata.names));
    named.push((catalogue.manufacturer.id.clone(), "catalogue.manufacturer.names".into(), &catalogue.manufacturer.names));
    for (_i, g) in catalogue.product_groups.iter().enumerate() {
        named.push((g.id.clone(), format!("catalogue.productGroups[id={}].names", g.id), &g.names));
    }
    for (_i, c) in catalogue.product_classes.iter().enumerate() {
        named.push((c.id.clone(), format!("catalogue.productClasses[id={}].names", c.id), &c.names));
    }
    for (_i, s) in catalogue.product_series.iter().enumerate() {
        named.push((s.id.clone(), format!("catalogue.productSeries[id={}].names", s.id), &s.names));
    }
    for (_i, p) in catalogue.products.iter().enumerate() {
        named.push((p.id.clone(), format!("catalogue.products[id={}].names", p.id), &p.names));
    }
    for (_i, d) in catalogue.property_definitions.iter().enumerate() {
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
                {
                    let mut remedies = Vec::new();
                    if !names.alternatives.is_empty() {
                        remedies.push(Remedy::one_of(
                            subject(&id, format!("{path}.alternatives[0].locale"), names.preferred.text.clone(), names.preferred.text.clone()),
                            vec![locale.clone()],
                            copy(
                                format!("Set alternatives[0].locale to '{locale}' so '{id}' covers the required language."),
                                format!("alternatives[0].locale auf '{locale}' setzen, damit '{id}' die geforderte Sprache abdeckt."),
                            ),
                        ));
                        remedies.push(Remedy::one_of(
                            subject(&id, format!("{path}.alternatives[0].text"), names.preferred.text.clone(), names.preferred.text.clone()),
                            vec![names.preferred.text.clone()],
                            copy(
                                format!("Keep alternatives[0].text as a non-empty name for '{id}' in language '{locale}'."),
                                format!("alternatives[0].text als nicht-leeren Namen für '{id}' in Sprache '{locale}' belassen."),
                            ),
                        ));
                    }
                    remedies.push(Remedy::one_of(
                        subject(&id, format!("{path}.preferred.locale"), names.preferred.text.clone(), names.preferred.text.clone()),
                        vec![locale.clone()],
                        copy(
                            format!("Or set preferred.locale to '{locale}' if that name should be the primary language for '{id}'."),
                            format!("Oder preferred.locale auf '{locale}' setzen, wenn dieser Name die Primärsprache für '{id}' sein soll."),
                        ),
                    ));
                    remedies
                },
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

fn product_class<'a>(catalogue: &'a Catalogue, product: &Product) -> Option<&'a semio_s_artifact_norm_iso16757_model::part_1::ProductClass> {
    let series = catalogue.product_series.iter().find(|s| s.id == product.series_id)?;
    catalogue.product_classes.iter().find(|c| c.id == series.class_id)
}

pub(super) fn check_required_properties(catalogue: &Catalogue, report: &mut CheckReport) {
    let mut checked = 0u32;
    let mut failed = 0u32;
    for (_pi, product) in catalogue.products.iter().enumerate() {
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

pub(super) fn check_property_values(catalogue: &Catalogue, dictionary: &semio_s_artifact_norm_iso16757_model::part_4::Dictionary, report: &mut CheckReport) {
    let mut checked = 0u32;
    let mut failed = 0u32;
    for (_pi, product) in catalogue.products.iter().enumerate() {
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
                if let Some(dict_prop) = semio_s_artifact_norm_iso16757_checks_common::helpers::part_4::resolve_property(dictionary, dict_prop_id) {
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

pub(super) fn check_property_definition_units(catalogue: &Catalogue, dictionary: &semio_s_artifact_norm_iso16757_model::part_4::Dictionary, report: &mut CheckReport) {
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

pub(super) fn check_variant_domains(catalogue: &Catalogue, report: &mut CheckReport) {
    let mut checked = 0u32;
    let mut failed = 0u32;
    for (_pi, product) in catalogue.products.iter().enumerate() {
        for (_vi, variant) in product.variants.iter().enumerate() {
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
