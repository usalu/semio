//! 📚️ ISO 16757-4 dictionary structure, constraints, ISO 12006 mappings.

use super::common::{assess, copy, fail, na, pass, q_dim, subject, controlled_ordinal, reference_slot_score};
use crate::document::{CheckReport, CheckStatus, Remedy, RemedyBound};
use crate::CatalogueValue;
use crate::standards::v1::subsets::any::schema::part_4 as helpers;
use crate::Iso16757Snapshot;
use std::collections::HashSet;

pub fn check_part_4(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    check_dictionary_typing(doc, report);
    let dictionary = &doc.dictionary;
    let issues = helpers::validate_dictionary(dictionary);
    if issues.is_empty() {
        report.push(pass(
            "iso16757.4.4.3.structure",
            "4",
            "4.3",
            subject("dictionary", "dictionary", "Dictionary", "Wörterbuch"),
            copy("Dictionary structure", "Wörterbuchstruktur"),
            copy("Dictionary relationships are acyclic with resolvable endpoints.", "Wörterbuchbeziehungen sind azyklisch mit auflösbaren Endpunkten."),
        ));
    } else {
        for issue in &issues {
            report.push(fail(
                format!("iso16757.4.4.3.{}", issue.chars().filter(|c| c.is_alphanumeric()).take(32).collect::<String>()),
                "4",
                "4.3",
                subject("dictionary", "dictionary.relationships", "Dictionary", "Wörterbuch"),
                copy("Dictionary structure", "Wörterbuchstruktur"),
                copy(
                    format!("Dictionary relationship issue: {issue}"),
                    format!("Wörterbuch-Beziehungsproblem: {issue}"),
                ),
                {
                    let rel_id = dictionary
                        .relationships
                        .iter()
                        .find(|r| !dictionary.subjects.iter().any(|s| s.id == r.target_id) || !dictionary.subjects.iter().any(|s| s.id == r.source_id))
                        .map(|r| r.id.clone())
                        .unwrap_or_else(|| "rel".into());
                    vec![Remedy {
                        target: subject("dictionary", format!("dictionary.relationships[id={rel_id}].targetId"), "Relationship target", "Beziehungsziel"),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: dictionary.subjects.iter().map(|s| s.id.clone()).collect(),
                        action: copy(
                            "Set the dangling relationship targetId to an existing dictionary subject.",
                            "targetId der hängenden Beziehung auf ein vorhandenes Wörterbuchsubjekt setzen.",
                        ),
                        applicable: true,
                    }]
                },
            ));
        }
    }

    if dictionary.subjects.is_empty() {
        report.push(na(
            "iso16757.4.5.1.mapping",
            "4",
            "5.1",
            subject("dictionary", "dictionary.subjects", "Dictionary subjects", "Wörterbuchsubjekte"),
            copy("ISO 12006-3 mapping", "ISO-12006-3-Abbildung"),
            copy("Dictionary has no subjects to map.", "Wörterbuch hat keine Subjekte zum Abbilden."),
        ));
    } else {
        let mappings = helpers::to_iso12006_mappings(dictionary);
        if mappings.len() == dictionary.subjects.len() {
            report.push(pass(
                "iso16757.4.5.1.mapping",
                "4",
                "5.1",
                subject("dictionary", "dictionary.subjects", "Dictionary subjects", "Wörterbuchsubjekte"),
                copy("ISO 12006-3 mapping", "ISO-12006-3-Abbildung"),
                copy(
                    format!("All {} subjects have ISO 12006-3 mapping URIs.", mappings.len()),
                    format!("Alle {} Subjekte haben ISO-12006-3-Abbildungs-URIs.", mappings.len()),
                ),
            ));
        } else {
            report.push(assess(
                "iso16757.4.5.1.mapping",
                "4",
                "5.1",
                subject("dictionary", "dictionary.subjects", "Dictionary subjects", "Wörterbuchsubjekte"),
                copy("ISO 12006-3 mapping", "ISO-12006-3-Abbildung"),
                copy(
                    format!("Mapped {} of {} subjects to ISO 12006-3.", mappings.len(), dictionary.subjects.len()),
                    format!("{} von {} Subjekten nach ISO 12006-3 abgebildet.", mappings.len(), dictionary.subjects.len()),
                ),
                CheckStatus::Fail,
                q_dim(mappings.len() as f64),
                q_dim(dictionary.subjects.len() as f64),
                vec![Remedy::at_least(
                    subject("dictionary", "dictionary.subjects", "Dictionary subjects", "Wörterbuchsubjekte"),
                    q_dim(mappings.len() as f64),
                    q_dim(dictionary.subjects.len() as f64),
                    copy("Ensure every dictionary subject is included in the ISO 12006-3 mapping set.", "Jedes Wörterbuchsubjekt in die ISO-12006-3-Abbildungsmenge aufnehmen."),
                )],
            ));
        }
    }

    if dictionary.controlled_lists.is_empty() {
        report.push(na(
            "iso16757.4.6.3.2.controlled",
            "4",
            "6.3.2",
            subject("dictionary", "dictionary.controlledLists", "Controlled lists", "Kontrollierte Listen"),
            copy("Controlled value lists", "Kontrollierte Wertelisten"),
            copy("No controlled value lists declared.", "Keine kontrollierten Wertelisten deklariert."),
        ));
        return;
    }

    let mut failed = 0u32;
    for list in &dictionary.controlled_lists {
        let contexts = if list.context_subject_ids.is_empty() {
            dictionary.subjects.iter().map(|s| s.id.clone()).collect::<Vec<_>>()
        } else {
            list.context_subject_ids.clone()
        };
        for ctx in &contexts {
            let allowed = helpers::filter_controlled_values(list, ctx, dictionary);
            if allowed.is_empty() && !list.values.is_empty() {
                failed += 1;
                report.push(fail(
                    format!("iso16757.4.6.3.2.{}.{}", list.id, ctx),
                    "4",
                    "6.3.2",
                    subject(&list.id, format!("dictionary.controlledLists[id={}]", list.id), &list.id, &list.id),
                    copy("Context-filtered controlled values", "Kontextgefilterte kontrollierte Werte"),
                    copy(
                        format!("Controlled list '{}' yields no values for subject context '{}'.", list.id, ctx),
                        format!("Kontrollierte Liste '{}' liefert keine Werte für Subjektkontext '{}'.", list.id, ctx),
                    ),
                    vec![Remedy {
                        target: subject(&list.id, format!("dictionary.controlledLists[id={}].contextSubjectIds", list.id), &list.id, &list.id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: dictionary.subjects.iter().map(|s| s.id.clone()).collect(),
                        action: copy(
                            format!("Align contextSubjectIds of '{}' with subjects in the subtype closure of '{ctx}', or clear the filter.", list.id),
                            format!("contextSubjectIds von '{}' an Subjekte in der Subtyp-Hülle von '{ctx}' anpassen oder den Filter leeren.", list.id),
                        ),
                        applicable: true,
                    }],
                ));
            }
        }
    }

    for product in &doc.catalogue.products {
        for pv in product.static_properties.iter().chain(product.variants.iter().flat_map(|v| v.property_values.iter())) {
            let Some(def) = doc.catalogue.property_definitions.iter().find(|d| d.id == pv.definition_id) else { continue };
            let Some(dict_id) = &def.dictionary_property_id else { continue };
            let Some(dict_prop) = helpers::resolve_property(dictionary, dict_id) else { continue };
            for list in &dictionary.controlled_lists {
                let applies = list.id == *dict_id || dict_prop.data_type == "controlled" || matches!(&pv.value, CatalogueValue::Controlled { list_id, .. } if list_id == &list.id);
                if !applies {
                    continue;
                }
                let text = match &pv.value {
                    CatalogueValue::Decimal { value } => value.to_string(),
                    CatalogueValue::Integer { value } => value.to_string(),
                    CatalogueValue::Text { value } | CatalogueValue::Enumeration { value } => value.clone(),
                    CatalogueValue::Controlled { value, .. } => value.clone(),
                    _ => continue,
                };
                let subject_hint = dict_prop.applicable_subject_ids.first().cloned().unwrap_or_default();
                let allowed = if subject_hint.is_empty() {
                    list.values.clone()
                } else {
                    helpers::filter_controlled_values(list, &subject_hint, dictionary)
                };
                if !allowed.is_empty() && !allowed.iter().any(|a| a == &text) {
                    failed += 1;
                    report.push(fail(
                        format!("iso16757.4.6.3.2.value.{}.{}.{}", product.id, pv.definition_id, text),
                        "4",
                        "6.3.2",
                        subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                        copy("Controlled value membership", "Zugehörigkeit zu kontrolliertem Wert"),
                        copy(
                            format!("Value '{text}' of '{}' is not in controlled list '{}' {:?}.", pv.definition_id, list.id, allowed),
                            format!("Wert '{text}' von '{}' ist nicht in kontrollierter Liste '{}' {:?}.", pv.definition_id, list.id, allowed),
                        ),
                        vec![Remedy::one_of(
                            subject(&product.id, format!("catalogue.products[id={}]", product.id), &product.names.preferred.text, &product.names.preferred.text),
                            allowed,
                            copy(
                                format!("Set '{}' to one of the allowed controlled values.", pv.definition_id),
                                format!("'{}' auf einen der zulässigen kontrollierten Werte setzen.", pv.definition_id),
                            ),
                        )],
                    ));
                }
            }
        }
    }

    for media in &doc.catalogue.descriptive_objects {
        let ok = !media.id.trim().is_empty() && !media.media_type.trim().is_empty() && !media.uri.trim().is_empty();
        if ok {
            report.push(pass(
                format!("iso16757.4.media.{}", media.id),
                "4",
                "4.3",
                subject(&media.id, format!("catalogue.descriptiveObjects[id={}]", media.id), &media.id, &media.id),
                copy("Descriptive media", "Beschreibende Medien"),
                copy(
                    format!("Descriptive object '{}' has mediaType '{}' and uri.", media.id, media.media_type),
                    format!("Beschreibendes Objekt '{}' hat mediaType '{}' und uri.", media.id, media.media_type),
                ),
            ));
        } else {
            report.push(fail(
                format!("iso16757.4.media.{}", media.id),
                "4",
                "4.3",
                subject(&media.id, format!("catalogue.descriptiveObjects[id={}].uri", media.id), &media.id, &media.id),
                copy("Descriptive media", "Beschreibende Medien"),
                copy(
                    format!("Descriptive object '{}' needs non-empty mediaType and uri.", media.id),
                    format!("Beschreibendes Objekt '{}' benötigt nicht-leere mediaType und uri.", media.id),
                ),
                vec![Remedy {
                    target: subject(&media.id, format!("catalogue.descriptiveObjects[id={}].uri", media.id), &media.id, &media.id),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec!["https://example.invalid/media/illustration.svg".into()],
                    action: copy(
                        format!("Set uri (and mediaType) for descriptive object '{}'.", media.id),
                        format!("uri (und mediaType) für beschreibendes Objekt '{}' setzen.", media.id),
                    ),
                    applicable: true,
                }],
            ));
        }
    }

    if failed == 0 {
        report.push(pass(
            "iso16757.4.6.3.2.controlled",
            "4",
            "6.3.2",
            subject("dictionary", "dictionary.controlledLists", "Controlled lists", "Kontrollierte Listen"),
            copy("Controlled value lists", "Kontrollierte Wertelisten"),
            copy("Controlled lists filter correctly and catalogue values comply.", "Kontrollierte Listen filtern korrekt und Katalogwerte entsprechen."),
        ));
    }

    {
        let subject_ids: HashSet<_> = dictionary.subjects.iter().map(|s| s.id.clone()).chain(dictionary.meta_subjects.iter().map(|s| s.id.clone())).collect();
        for rel in &dictionary.relationships {
            for (field, target) in [("sourceId", &rel.source_id), ("targetId", &rel.target_id)] {
                let ok = subject_ids.contains(target);
                report.push(assess(
                    format!("iso16757.4.4.relEndpoint.{}.{}", rel.id, field),
                    "4",
                    "4.4",
                    subject(&rel.id, format!("dictionary.relationships[id={}].{}", rel.id, field), &rel.id, &rel.id),
                    copy("Dictionary relationship endpoint", "Wörterbuch-Beziehungsendpunkt"),
                    copy(
                        format!("Relationship '{}' {} '{}' {}.", rel.id, field, target, if ok { "resolves" } else { "is unknown" }),
                        format!("Beziehung '{}' {} '{}' {}.", rel.id, field, target, if ok { "löst auf" } else { "ist unbekannt" }),
                    ),
                    if ok { CheckStatus::Pass } else { CheckStatus::Fail },
                    q_dim(1.0),
                    q_dim(1.0),
                    if ok {
                        Vec::new()
                    } else {
                        vec![Remedy::one_of(
                            subject(&rel.id, format!("dictionary.relationships[id={}].{}", rel.id, field), &rel.id, &rel.id),
                            subject_ids.iter().cloned().collect(),
                            copy("Retarget relationship endpoint.", "Beziehungsendpunkt neu setzen."),
                        )]
                    },
                ));
            }
            let max_c = rel.cardinality.max.unwrap_or(u32::MAX) as f64;
            let min_c = rel.cardinality.min as f64;
            report.push(assess(
                format!("iso16757.4.4.relCard.{}", rel.id),
                "4",
                "4.4",
                subject(&rel.id, format!("dictionary.relationships[id={}].cardinality.max", rel.id), &rel.id, &rel.id),
                copy("Dictionary relationship cardinality", "Wörterbuch-Beziehungskardinalität"),
                copy(
                    format!("Relationship '{}' cardinality [{min_c}, {max_c}].", rel.id),
                    format!("Beziehung '{}' Kardinalität [{min_c}, {max_c}].", rel.id),
                ),
                CheckStatus::Pass,
                q_dim(min_c),
                q_dim(if max_c < 1.0e9 { max_c } else { min_c + 1.0 }),
                Vec::new(),
            ));
        }
    }

}




/// 📚️ Part 4 §4.3 / §5.1 — dictionary reference, property typing, controlled lists, relationship endpoints.
fn check_dictionary_typing(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    let dict = &doc.dictionary;
    if dict.reference.id.trim().is_empty() || dict.reference.version.trim().is_empty() {
        report.push(fail(
            "iso16757.4.4.3.reference",
            "4",
            "4.3",
            subject("dictionary", "dictionary.reference.id", "Dictionary", "Wörterbuch"),
            copy("Dictionary reference", "Wörterbuch-Referenz"),
            copy("Dictionary reference id/version must be non-empty (Part 4 §4.3).", "Wörterbuch-Referenz Id/Version müssen nicht leer sein (Teil 4 §4.3)."),
            vec![Remedy {
                target: subject("dictionary", "dictionary.reference.id", "Dictionary", "Wörterbuch"),
                current: q_dim(0.0),
                required: q_dim(1.0),
                bound: RemedyBound::Exactly,
                options: vec![],
                action: copy("Set dictionary.reference.id and .version.", "dictionary.reference.id und .version setzen."),
                applicable: true,
            }],
        ));
    } else {
        report.push(assess(
            "iso16757.4.4.3.reference",
            "4",
            "4.3",
            subject("dictionary", "dictionary.reference.version", "Dictionary", "Wörterbuch"),
            copy("Dictionary reference", "Wörterbuch-Referenz"),
            copy(
                format!("Dictionary reference {}@{} is set.", dict.reference.id, dict.reference.version),
                format!("Wörterbuch-Referenz {}@{} ist gesetzt.", dict.reference.id, dict.reference.version),
            ),
            CheckStatus::Pass,
            q_dim(1.0),
            q_dim(1.0),
            Vec::new(),
        ));
    }
    let mut subject_ids = HashSet::new();
    for s in &dict.subjects {
        if !subject_ids.insert(s.id.clone()) {
            report.push(fail(
                format!("iso16757.4.4.3.subjectDup.{}", s.id),
                "4",
                "4.3",
                subject(&s.id, format!("dictionary.subjects[id={}].id", s.id), &s.id, &s.id),
                copy("Subject id uniqueness", "Subjekt-Id-Eindeutigkeit"),
                copy(format!("Duplicate dictionary subject id '{}'.", s.id), format!("Doppelte Wörterbuch-Subjekt-Id '{}'.", s.id)),
                vec![Remedy {
                    target: subject(&s.id, format!("dictionary.subjects[id={}].id", s.id), &s.id, &s.id),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec![format!("{}-b", s.id)],
                    action: copy("Rename the duplicate subject id.", "Doppelte Subjekt-Id umbenennen."),
                    applicable: true,
                }],
            ));
        }
    }
    for rel in &dict.relationships {
        let src_ok = subject_ids.contains(&rel.source_id);
        let tgt_ok = subject_ids.contains(&rel.target_id);
        let resolved = u32::from(src_ok) + u32::from(tgt_ok);
        let card_ok = rel.cardinality.satisfies(resolved);

        report.push(assess(
            format!("iso16757.4.4.4.relationship.target.{}", rel.id),
            "4",
            "4.4",
            subject(&rel.id, format!("dictionary.relationships[id={}].targetId", rel.id), &rel.id, &rel.id),
            copy("Relationship target", "Beziehungsziel"),
            copy(
                format!("Relationship '{}' targetId '{}' resolution slot.", rel.id, rel.target_id),
                format!("Beziehung '{}' targetId '{}' Auflösungsplatz.", rel.id, rel.target_id),
            ),
            if tgt_ok { CheckStatus::Pass } else { CheckStatus::Fail },
            q_dim(reference_slot_score(&rel.target_id, subject_ids.iter())),
            q_dim(1.0),
            if tgt_ok {
                Vec::new()
            } else {
                vec![Remedy {
                    target: subject(&rel.id, format!("dictionary.relationships[id={}].targetId", rel.id), &rel.id, &rel.id),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: subject_ids.iter().cloned().collect(),
                    action: copy("Point targetId at an existing subject.", "targetId auf ein vorhandenes Subjekt setzen."),
                    applicable: true,
                }]
            },
        ));
        if !src_ok || !tgt_ok {
            report.push(fail(
                format!("iso16757.4.4.4.relationship.{}", rel.id),
                "4",
                "4.4",
                subject(&rel.id, format!("dictionary.relationships[id={}].targetId", rel.id), &rel.id, &rel.id),
                copy("Relationship endpoints", "Beziehungsendpunkte"),
                copy(
                    format!("Relationship '{}' endpoints {}→{} must resolve to subjects.", rel.id, rel.source_id, rel.target_id),
                    format!("Beziehung '{}' Endpunkte {}→{} müssen auf Subjekte zeigen.", rel.id, rel.source_id, rel.target_id),
                ),
                vec![Remedy {
                    target: subject(&rel.id, format!("dictionary.relationships[id={}].targetId", rel.id), &rel.id, &rel.id),
                    current: q_dim(resolved as f64),
                    required: q_dim(2.0),
                    bound: RemedyBound::OneOf,
                    options: subject_ids.iter().cloned().collect(),
                    action: copy("Point relationship sourceId/targetId at existing subjects.", "relationship sourceId/targetId auf vorhandene Subjekte setzen."),
                    applicable: true,
                }],
            ));
        }
        if !card_ok {
            report.push(fail(
                format!("iso16757.4.4.4.relationship.cardinality.{}", rel.id),
                "4",
                "4.4",
                subject(&rel.id, format!("dictionary.relationships[id={}].cardinality.min", rel.id), &rel.id, &rel.id),
                copy("Relationship cardinality", "Beziehungskardinalität"),
                copy(
                    format!(
                        "Relationship '{}' resolves {resolved} endpoint(s) but cardinality.min={} requires at least that many.",
                        rel.id, rel.cardinality.min
                    ),
                    format!(
                        "Beziehung '{}' löst {resolved} Endpunkt(e) auf, cardinality.min={} verlangt mindestens so viele.",
                        rel.id, rel.cardinality.min
                    ),
                ),
                vec![Remedy {
                    target: subject(&rel.id, format!("dictionary.relationships[id={}].cardinality.min", rel.id), &rel.id, &rel.id),
                    current: q_dim(resolved as f64),
                    required: q_dim(rel.cardinality.min as f64),
                    bound: RemedyBound::AtMost,
                    options: vec![resolved.to_string()],
                    action: copy(
                        format!("Lower cardinality.min to ≤ {resolved} or resolve missing relationship endpoints."),
                        format!("cardinality.min auf ≤ {resolved} senken oder fehlende Beziehungsendpunkte auflösen."),
                    ),
                    applicable: true,
                }],
            ));
        } else if src_ok && tgt_ok {
            report.push(assess(
                format!("iso16757.4.4.4.relationship.{}", rel.id),
                "4",
                "4.4",
                subject(&rel.id, format!("dictionary.relationships[id={}].cardinality.min", rel.id), &rel.id, &rel.id),
                copy("Relationship cardinality", "Beziehungskardinalität"),
                copy(
                    format!("Relationship '{}' {}→{} satisfies cardinality min={} with {resolved} resolved endpoints.", rel.id, rel.source_id, rel.target_id, rel.cardinality.min),
                    format!("Beziehung '{}' {}→{} erfüllt Kardinalität min={} mit {resolved} aufgelösten Endpunkten.", rel.id, rel.source_id, rel.target_id, rel.cardinality.min),
                ),
                CheckStatus::Pass,
                q_dim(resolved as f64),
                q_dim(rel.cardinality.min.max(1) as f64),
                Vec::new(),
            ));
        }
    }
    for prop in &dict.properties {
        let unit_ok = prop.unit.as_ref().map(|u| !u.symbol.trim().is_empty() && u.si_factor > 0.0).unwrap_or(true);
        let (l, m, ti, te) = prop.unit.as_ref().map(|u| (u.dimension.length, u.dimension.mass, u.dimension.time, u.dimension.temperature)).unwrap_or((0, 0, 0, 0));
        let dim_sum = i32::from(l) + i32::from(m) + i32::from(ti) + i32::from(te);
        if prop.data_type.trim().is_empty() || !unit_ok {
            report.push(fail(
                format!("iso16757.4.5.1.property.{}", prop.id),
                "4",
                "5.1",
                subject(&prop.id, format!("dictionary.properties[id={}].dataType", prop.id), &prop.id, &prop.id),
                copy("Dictionary property typing", "Wörterbuch-Eigenschafts-Typisierung"),
                copy(
                    format!("Property '{}' needs dataType and a positive siFactor unit when present.", prop.id),
                    format!("Eigenschaft '{}' benötigt dataType und bei Einheit positiven siFactor.", prop.id),
                ),
                vec![Remedy {
                    target: subject(&prop.id, format!("dictionary.properties[id={}].dataType", prop.id), &prop.id, &prop.id),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::OneOf,
                    options: vec!["decimal".into()],
                    action: copy("Set property dataType (and unit.siFactor > 0).", "property dataType setzen (und unit.siFactor > 0)."),
                    applicable: true,
                }],
            ));
        } else {
            let factor = prop.unit.as_ref().map(|u| u.si_factor).unwrap_or(1.0);
            report.push(assess(
                format!("iso16757.4.5.1.property.{}", prop.id),
                "4",
                "5.1",
                subject(&prop.id, format!("dictionary.properties[id={}].dataType", prop.id), &prop.id, &prop.id),
                copy("Dictionary property typing", "Wörterbuch-Eigenschafts-Typisierung"),
                copy(
                    format!("Property '{}' dataType '{}' siFactor={factor} dimSum={dim_sum}.", prop.id, prop.data_type),
                    format!("Eigenschaft '{}' dataType '{}' siFactor={factor} dimSum={dim_sum}.", prop.id, prop.data_type),
                ),
                CheckStatus::Pass,
                q_dim(controlled_ordinal(&prop.data_type, &["decimal", "quantity", "integer", "text", "boolean", "controlled"]) + factor + dim_sum as f64),
                q_dim(1.0),
                Vec::new(),
            ));
            if let Some(unit) = &prop.unit {
                report.push(assess(
                    format!("iso16757.4.5.1.propertySymbol.{}", prop.id),
                    "4",
                    "5.1",
                    subject(&prop.id, format!("dictionary.properties[id={}].unit.symbol", prop.id), &prop.id, &prop.id),
                    copy("Dictionary property unit symbol", "Wörterbuch-Eigenschafts-Einheitssymbol"),
                    copy(
                        format!("Property '{}' unit symbol is '{}'.", prop.id, unit.symbol),
                        format!("Eigenschaft '{}' Einheitssymbol ist '{}'.", prop.id, unit.symbol),
                    ),
                    CheckStatus::Pass,
                    q_dim(controlled_ordinal(&unit.symbol, &["m", "mm", "cm", "kg", "g", "s", "K", "Pa", "bar", "W", "m3", "l", "-"])),
                    q_dim(1.0),
                    Vec::new(),
                ));
                report.push(assess(
                    format!("iso16757.4.5.1.propertySiFactor.{}", prop.id),
                    "4",
                    "5.1",
                    subject(&prop.id, format!("dictionary.properties[id={}].unit.siFactor", prop.id), &prop.id, &prop.id),
                    copy("Dictionary property SI factor", "Wörterbuch-Eigenschafts-SI-Faktor"),
                    copy(
                        format!("Property '{}' siFactor = {}.", prop.id, unit.si_factor),
                        format!("Eigenschaft '{}' siFactor = {}.", prop.id, unit.si_factor),
                    ),
                    CheckStatus::Pass,
                    q_dim(unit.si_factor),
                    q_dim(unit.si_factor),
                    Vec::new(),
                ));
                for (axis, value, path_suffix) in [
                    ("length", unit.dimension.length as f64, "unit.dimension.length"),
                    ("mass", unit.dimension.mass as f64, "unit.dimension.mass"),
                    ("time", unit.dimension.time as f64, "unit.dimension.time"),
                    ("temperature", unit.dimension.temperature as f64, "unit.dimension.temperature"),
                ] {
                    report.push(assess(
                        format!("iso16757.4.5.1.propertyDim.{}.{axis}", prop.id),
                        "4",
                        "5.1",
                        subject(&prop.id, format!("dictionary.properties[id={}].{path_suffix}", prop.id), &prop.id, &prop.id),
                        copy("Dictionary property dimension", "Wörterbuch-Eigenschaftsdimension"),
                        copy(
                            format!("Property '{}' dimension.{axis} = {value}.", prop.id),
                            format!("Eigenschaft '{}' Dimension.{axis} = {value}.", prop.id),
                        ),
                        CheckStatus::Pass,
                        q_dim(value),
                        q_dim(value),
                        Vec::new(),
                    ));
                }
            }
        }
        for (ci, c) in prop.value_constraints.iter().enumerate() {
            let lo = c.min.unwrap_or(f64::NEG_INFINITY);
            let hi = c.max.unwrap_or(f64::INFINITY);
            if c.min.is_some() && c.max.is_some() && lo > hi {
                report.push(fail(
                    format!("iso16757.4.5.1.constraint.{}.{}", prop.id, ci),
                    "4",
                    "5.1",
                    subject(&prop.id, format!("dictionary.properties[id={}].valueConstraints[{ci}].min", prop.id), &prop.id, &prop.id),
                    copy("Value constraint range", "Werteinschränkungsbereich"),
                    copy(format!("Constraint min {lo} > max {hi}."), format!("Einschränkung min {lo} > max {hi}.")),
                    vec![Remedy::at_least(
                        subject(&prop.id, format!("dictionary.properties[id={}].valueConstraints[{ci}].max", prop.id), &prop.id, &prop.id),
                        q_dim(hi),
                        q_dim(lo),
                        copy("Raise max to be ≥ min.", "max auf ≥ min anheben."),
                    )],
                ));
            } else if c.min.is_some() || c.max.is_some() {
                report.push(assess(
                    format!("iso16757.4.5.1.constraint.{}.{}", prop.id, ci),
                    "4",
                    "5.1",
                    subject(&prop.id, format!("dictionary.properties[id={}].valueConstraints[{ci}].min", prop.id), &prop.id, &prop.id),
                    copy("Value constraint range", "Werteinschränkungsbereich"),
                    copy(format!("Constraint range [{lo}, {hi}]."), format!("Einschränkungsbereich [{lo}, {hi}].")),
                    CheckStatus::Pass,
                    q_dim(if lo.is_finite() { lo } else { hi }),
                    q_dim(if hi.is_finite() { hi } else { lo }),
                    Vec::new(),
                ));
            }
        }
    }

    let subject_ids: HashSet<_> = dict
        .subjects
        .iter()
        .map(|s| s.id.clone())
        .chain(dict.meta_subjects.iter().map(|s| s.id.clone()))
        .collect();
    for prop in &dict.properties {
        for (si, sid) in prop.applicable_subject_ids.iter().enumerate() {
            let path = format!("dictionary.properties[id={}].applicableSubjectIds[{si}]", prop.id);
            if subject_ids.contains(sid) {
                report.push(assess(
                    format!("iso16757.4.5.applicable.{}.{}", prop.id, sid),
                    "4",
                    "5.1",
                    subject(&prop.id, path, &prop.id, &prop.id),
                    copy("Dictionary property applicable subject", "Wörterbuch-Eigenschaft anwendbares Subjekt"),
                    copy(
                        format!("Property '{}' applies to '{sid}'.", prop.id),
                        format!("Eigenschaft '{}' gilt für '{sid}'.", prop.id),
                    ),
                    CheckStatus::Pass,
                    q_dim(1.0),
                    q_dim(1.0),
                    Vec::new(),
                ));
            } else {
                report.push(fail(
                    format!("iso16757.4.5.applicable.{}.{}", prop.id, sid),
                    "4",
                    "5.1",
                    subject(&prop.id, path.clone(), &prop.id, &prop.id),
                    copy("Dictionary property applicable subject", "Wörterbuch-Eigenschaft anwendbares Subjekt"),
                    copy(
                        format!("Property '{}' lists unknown subject '{sid}'.", prop.id),
                        format!("Eigenschaft '{}' listet unbekanntes Subjekt '{sid}'.", prop.id),
                    ),
                    vec![Remedy::one_of(
                        subject(&prop.id, path, &prop.id, &prop.id),
                        subject_ids.iter().cloned().collect(),
                        copy("Retarget applicableSubjectIds.", "applicableSubjectIds neu setzen."),
                    )],
                ));
            }
        }
    }
    let mut catalogue_tokens: Vec<String> = Vec::new();
    for product in &doc.catalogue.products {
        for variant in &product.variants {
            for value in variant.parameter_values.values() {
                match value {
                    CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => {
                        catalogue_tokens.push(format!("{}", *value as i64));
                    }
                    CatalogueValue::Integer { value } => catalogue_tokens.push(format!("{value}")),
                    CatalogueValue::Text { value }
                    | CatalogueValue::Identifier { value }
                    | CatalogueValue::Enumeration { value } => catalogue_tokens.push(value.clone()),
                    _ => {}
                }
            }
        }
        for domain in &product.parameter_domains {
            for allowed in &domain.allowed_values {
                match allowed {
                    CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => {
                        catalogue_tokens.push(format!("{}", *value as i64));
                    }
                    CatalogueValue::Integer { value } => catalogue_tokens.push(format!("{value}")),
                    CatalogueValue::Text { value }
                    | CatalogueValue::Identifier { value }
                    | CatalogueValue::Enumeration { value } => catalogue_tokens.push(value.clone()),
                    _ => {}
                }
            }
        }
    }
    for list in &dict.controlled_lists {
        for (vi, val) in list.values.iter().enumerate() {
            let list_score = if val.trim().is_empty() {
                0.0
            } else {
                reference_slot_score(val, catalogue_tokens.iter().map(String::as_str))
            };
            report.push(assess(
                format!("iso16757.4.6.listValue.{}.{}", list.id, vi),
                "4",
                "6.3.2",
                subject(&list.id, format!("dictionary.controlledLists[id={}].values[{vi}]", list.id), &list.id, &list.id),
                copy("Controlled list value", "Kontrollierter Listenwert"),
                copy(
                    format!("Controlled list '{}' values[{vi}] = '{val}'.", list.id),
                    format!("Kontrollierte Liste '{}' values[{vi}] = '{val}'.", list.id),
                ),
                if val.trim().is_empty() { CheckStatus::Fail } else { CheckStatus::Pass },
                q_dim(list_score),
                q_dim(1.0),
                if val.trim().is_empty() {
                    vec![Remedy {
                        target: subject(&list.id, format!("dictionary.controlledLists[id={}].values[{vi}]", list.id), &list.id, &list.id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::Exactly,
                        options: catalogue_tokens.clone(),
                        action: copy("Set a non-empty controlled value.", "Nicht-leeren kontrollierten Wert setzen."),
                        applicable: true,
                    }]
                } else {
                    Vec::new()
                },
            ));
        }
    }

    for list in &dict.controlled_lists {
        if list.values.is_empty() {
            report.push(fail(
                format!("iso16757.4.5.1.controlledList.{}", list.id),
                "4",
                "5.1",
                subject(&list.id, format!("dictionary.controlledLists[id={}].values", list.id), &list.id, &list.id),
                copy("Controlled list values", "Kontrollierte Listenwerte"),
                copy(format!("Controlled list '{}' must declare ≥1 value.", list.id), format!("Kontrollierte Liste '{}' muss ≥1 Wert deklarieren.", list.id)),
                vec![Remedy {
                    target: subject(&list.id, format!("dictionary.controlledLists[id={}].values[0]", list.id), &list.id, &list.id),
                    current: q_dim(0.0),
                    required: q_dim(1.0),
                    bound: RemedyBound::Exactly,
                    options: vec!["50".into()],
                    action: copy("Add at least one controlled value.", "Mindestens einen kontrollierten Wert hinzufügen."),
                    applicable: true,
                }],
            ));
        } else {
            report.push(assess(
                format!("iso16757.4.5.1.controlledList.{}", list.id),
                "4",
                "5.1",
                subject(&list.id, format!("dictionary.controlledLists[id={}].values", list.id), &list.id, &list.id),
                copy("Controlled list values", "Kontrollierte Listenwerte"),
                copy(
                    format!("Controlled list '{}' has {} values.", list.id, list.values.len()),
                    format!("Kontrollierte Liste '{}' hat {} Werte.", list.id, list.values.len()),
                ),
                CheckStatus::Pass,
                q_dim(1.0),
                q_dim(1.0),
                Vec::new(),
            ));
        }
        for ctx in &list.context_subject_ids {
            if !subject_ids.contains(ctx) {
                report.push(fail(
                    format!("iso16757.4.5.1.controlledListCtx.{}.{}", list.id, ctx),
                    "4",
                    "5.1",
                    subject(&list.id, format!("dictionary.controlledLists[id={}].contextSubjectIds", list.id), &list.id, &list.id),
                    copy("Controlled list context", "Kontrollierte Listen-Kontext"),
                    copy(format!("Context subject '{ctx}' is unknown."), format!("Kontext-Subjekt '{ctx}' ist unbekannt.")),
                    vec![Remedy {
                        target: subject(&list.id, format!("dictionary.controlledLists[id={}].contextSubjectIds[0]", list.id), &list.id, &list.id),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: subject_ids.iter().cloned().collect(),
                        action: copy("Point contextSubjectIds at existing subjects.", "contextSubjectIds auf vorhandene Subjekte setzen."),
                        applicable: true,
                    }],
                ));
            }
        }
    }
}
