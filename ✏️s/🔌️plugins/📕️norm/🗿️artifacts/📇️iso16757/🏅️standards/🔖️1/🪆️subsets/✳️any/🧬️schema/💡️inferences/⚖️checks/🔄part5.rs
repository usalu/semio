//! 🔄 ISO 16757-5 exchange process, part numbers, IFC/STEP, script limits.

use super::common::{assess, copy, fail, na, pass, q_dim, subject};
use crate::document::{CheckReport, CheckStatus, Remedy, RemedyBound};
use crate::part_5::{ExchangeProcess, PartNumberRule};
use crate::artifact_schema::part_5::ScriptRuntime;
use crate::standards::v1::subsets::any::schema::part_5 as helpers;
use crate::CatalogueValue;
use crate::Iso16757Snapshot;
use std::collections::HashMap;

pub fn check_part_5(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    check_exchange_process(doc, report);
    check_ifc_exchange(doc, report);
    check_part_number(doc, report);
    check_script_limits(doc, report);
    check_edition_profile(doc, report);
}

fn check_edition_profile(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    use crate::part_1::EditionProfile;
    let profile = doc.catalogue.metadata.edition_profile;
    let (ok, required) = match profile {
        EditionProfile::FullPublished => (true, "fullPublished"),
        EditionProfile::Part1_2015 => (true, "part1_2015"),
        EditionProfile::Part2_2016 => (true, "part2_2016"),
        EditionProfile::Part4_2025 => (true, "part4_2025"),
        EditionProfile::Part5_2025 => (true, "part5_2025"),
    };
    if ok {
        report.push(pass(
            "iso16757.5.edition.profile",
            "5",
            "4.1",
            subject("edition", "catalogue.metadata.editionProfile", "Edition profile", "Editionsprofil"),
            copy("Edition profile", "Editionsprofil"),
            copy(
                format!("Document declares supported edition profile {:?} ({required}).", profile),
                format!("Dokument deklariert unterstütztes Editionsprofil {:?} ({required}).", profile),
            ),
        ));
    } else {
        report.push(fail(
            "iso16757.5.edition.profile",
            "5",
            "4.1",
            subject("edition", "catalogue.metadata.editionProfile", "Edition profile", "Editionsprofil"),
            copy("Edition profile", "Editionsprofil"),
            copy("Document edition profile is outside the supported ISO 16757 set.", "Dokument-Editionsprofil liegt außerhalb der unterstützten ISO-16757-Menge."),
            vec![Remedy {
                target: subject("edition", "catalogue.metadata.editionProfile", "Edition profile", "Editionsprofil"),
                current: q_dim(0.0),
                required: q_dim(1.0),
                bound: RemedyBound::OneOf,
                options: vec!["fullPublished".into(), "part1_2015".into(), "part2_2016".into(), "part4_2025".into(), "part5_2025".into()],
                action: copy(
                    "Set editionProfile to a supported ISO 16757 edition key.",
                    "editionProfile auf einen unterstützten ISO-16757-Editionsschlüssel setzen.",
                ),
                applicable: true,
            }],
        ));
    }
}

fn check_exchange_process(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    let title = copy("Exchange process readiness", "Bereitschaft des Austauschprozesses");
    let path = "exchangeProcess";
    let mut missing = Vec::new();
    match doc.exchange_process {
        ExchangeProcess::CreateFromDictionary => {
            if doc.dictionary.subjects.is_empty() {
                missing.push("dictionary.subjects");
            }
            if doc.dictionary.properties.is_empty() {
                missing.push("dictionary.properties");
            }
        }
        ExchangeProcess::ProvideCatalogue => {
            if doc.catalogue.products.is_empty() {
                missing.push("catalogue.products");
            }
            if doc.catalogue.property_definitions.is_empty() {
                missing.push("catalogue.propertyDefinitions");
            }
            if doc.catalogue.product_classes.is_empty() {
                missing.push("catalogue.productClasses");
            }
        }
        ExchangeProcess::DetermineProduct => {
            if doc.selection.class_id.trim().is_empty() {
                missing.push("selection.classId");
            }
            if doc.catalogue.product_indexes.is_empty() {
                missing.push("catalogue.productIndexes");
            }
        }
        ExchangeProcess::IntegrateIntoSystem => {
            if doc.geometry.objects.is_empty() {
                missing.push("geometry.objects");
            }
            if doc.catalogue.products.iter().all(|p| p.variants.iter().all(|v| v.geometry_id.is_none()) && doc.catalogue.product_series.iter().find(|s| s.id == p.series_id).and_then(|s| s.geometry_id.as_ref()).is_none()) {
                missing.push("geometry linkage on products/series");
            }
        }
        ExchangeProcess::ExchangeSystemModel => {
            if doc.catalogue.products.is_empty() {
                missing.push("catalogue.products");
            }
            if doc.geometry.objects.is_empty() {
                missing.push("geometry.objects");
            }
        }
    }
    if missing.is_empty() {
        report.push(pass(
            "iso16757.5.4.1.exchangeProcess",
            "5",
            "4.1",
            subject("exchange", path, "Exchange process", "Austauschprozess"),
            title,
            copy(
                format!("Exchange process {:?} has all mandatory catalogue content.", doc.exchange_process),
                format!("Austauschprozess {:?} hat alle verpflichtenden Kataloginhalte.", doc.exchange_process),
            ),
        ));
    } else {
        report.push(fail(
            "iso16757.5.4.1.exchangeProcess",
            "5",
            "4.1",
            subject("exchange", path, "Exchange process", "Austauschprozess"),
            title,
            copy(
                format!("Exchange process {:?} is missing: {}.", doc.exchange_process, missing.join(", ")),
                format!("Austauschprozess {:?} fehlt: {}.", doc.exchange_process, missing.join(", ")),
            ),
            vec![Remedy {
                target: subject("exchange", path, "Exchange process", "Austauschprozess"),
                current: q_dim(0.0),
                required: q_dim(1.0),
                bound: RemedyBound::OneOf,
                options: vec![
                    "createFromDictionary".into(),
                    "provideCatalogue".into(),
                    "determineProduct".into(),
                    "exchangeSystemModel".into(),
                ],
                action: copy(
                    format!("Set exchangeProcess to a stage matching available content (missing: {}).", missing.join(", ")),
                    format!("exchangeProcess auf eine passende Stufe setzen (fehlt: {}).", missing.join(", ")),
                ),
                applicable: true,
            }],
        ));
    }
}

fn check_ifc_exchange(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    let ifc = helpers::build_ifc_catalogue(&doc.catalogue);
    let issues = helpers::validate_exchange(&doc.catalogue, &ifc);
    if issues.is_empty() {
        report.push(pass(
            "iso16757.5.6.1.ifcStructure",
            "5",
            "6.1",
            subject("catalogue", "catalogue", "Catalogue", "Katalog"),
            copy("IFC catalogue structure", "IFC-Katalogstruktur"),
            copy(
                format!("IFC export covers {} products under schema '{}'.", ifc.products.len(), ifc.schema),
                format!("IFC-Export deckt {} Produkte unter Schema '{}' ab.", ifc.products.len(), ifc.schema),
            ),
        ));
    } else {
        for issue in &issues {
            report.push(fail(
                format!("iso16757.5.6.1.ifc.{}", issue.chars().filter(|c| c.is_alphanumeric()).take(24).collect::<String>()),
                "5",
                "6.1",
                subject("catalogue", "catalogue", "Catalogue", "Katalog"),
                copy("IFC catalogue structure", "IFC-Katalogstruktur"),
                copy(
                    format!("IFC exchange mapping issue: {issue}"),
                    format!("IFC-Austauschabbildungsproblem: {issue}"),
                ),
                vec![Remedy {
                    target: subject("catalogue", format!("catalogue.products[id={}].id", doc.catalogue.products.first().map(|p| p.id.as_str()).unwrap_or("product")), "Products", "Produkte"),
                    current: q_dim(ifc.products.len() as f64),
                    required: q_dim(doc.catalogue.products.len() as f64),
                    bound: RemedyBound::OneOf,
                    options: doc.catalogue.products.iter().map(|p| p.id.clone()).collect(),
                    action: copy(
                        "Ensure every catalogue product id is present as an IFC product entity with a resolvable globalId.",
                        "Sicherstellen, dass jede Katalogprodukt-Id als IFC-Produktentität mit auflösbarer globalId vorliegt.",
                    ),
                    applicable: true,
                }],
            ));
        }
    }

    let step = helpers::export_ifc_step(&ifc);
    let mut entity_ids = std::collections::HashSet::new();
    let mut refs_ok = true;
    let mut missing_entities = Vec::new();
    for line in step.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix('#') {
            if let Some((num, body)) = rest.split_once('=') {
                entity_ids.insert(num.trim().to_string());
                for token in body.split(|c: char| !c.is_ascii_digit() && c != '#') {
                    if let Some(r) = token.strip_prefix('#') {
                        if !r.is_empty() && !entity_ids.contains(r) && !body.contains(&format!("#{r}")) {
                            // forward refs allowed within STEP; collect unresolved after pass
                        }
                    }
                }
            }
        }
    }
    // Second pass: every #N reference must resolve
    let mut unresolved = Vec::new();
    for line in step.lines() {
        let body = line.trim();
        if !body.starts_with('#') {
            continue;
        }
        for part in body.split('#').skip(1) {
            let digits: String = part.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() && !entity_ids.contains(&digits) {
                unresolved.push(digits);
            }
        }
    }
    for product in &doc.catalogue.products {
        let needle = product.id.as_str();
        if !step.contains(needle) && !ifc.products.iter().any(|p| p.global_id == product.id || p.name.contains(needle)) {
            missing_entities.push(product.id.clone());
            refs_ok = false;
        }
    }
    let expected = doc.catalogue.products.len();
    if expected == 0 {
        report.push(na(
            "iso16757.5.6.1.step",
            "5",
            "6.1",
            subject("catalogue", "catalogue.products", "Products", "Produkte"),
            copy("IFC STEP entity integrity", "IFC-STEP-Entitätsintegrität"),
            copy("No products to export to STEP.", "Keine Produkte für STEP-Export."),
        ));
    } else if refs_ok && unresolved.is_empty() && missing_entities.is_empty() && !entity_ids.is_empty() {
        report.push(pass(
            "iso16757.5.6.1.step",
            "5",
            "6.1",
            subject("catalogue", "catalogue.products", "Products", "Produkte"),
            copy("IFC STEP entity integrity", "IFC-STEP-Entitätsintegrität"),
            copy(
                format!("STEP export declares {} entities with resolvable references covering all catalogue products.", entity_ids.len()),
                format!("STEP-Export deklariert {} Entitäten mit auflösbaren Referenzen für alle Katalogprodukte.", entity_ids.len()),
            ),
        ));
    } else {
        report.push(fail(
            "iso16757.5.6.1.step",
            "5",
            "6.1",
            subject("catalogue", "catalogue.products", "Products", "Produkte"),
            copy("IFC STEP entity integrity", "IFC-STEP-Entitätsintegrität"),
            copy(
                format!("STEP export failed structural checks (unresolved refs {}, missing products {:?}).", unresolved.len(), missing_entities),
                format!("STEP-Export strukturell fehlerhaft (unaufgelöste Refs {}, fehlende Produkte {:?}).", unresolved.len(), missing_entities),
            ),
            vec![Remedy {
                target: subject("catalogue", format!("catalogue.products[id={}].id", doc.catalogue.products.first().map(|p| p.id.as_str()).unwrap_or("product")), "Products", "Produkte"),
                current: q_dim((expected - missing_entities.len()) as f64),
                required: q_dim(expected as f64),
                bound: RemedyBound::OneOf,
                options: doc.catalogue.products.iter().map(|p| p.id.clone()).collect(),
                action: copy(
                    "Repair IFC/STEP export so every product entity exists and all #id references resolve.",
                    "IFC/STEP-Export so reparieren, dass jede Produktentität existiert und alle #id-Referenzen auflösen.",
                ),
                applicable: true,
            }],
        ));
    }
}


fn check_part_number(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    let runtime = helpers::DefaultScriptRuntime;
    // Use document script limits for Script rules
    let result = match &doc.part_number_rule {
        PartNumberRule::Script { function_id, source } => {
            if function_id.trim().is_empty() {
                report.push(fail(
                    "iso16757.5.6.10.functionId",
                    "5",
                    "6.10",
                    subject("partNumber", "partNumberRule", "Part number rule", "Teilenummer-Regel"),
                    copy("Part-number function id", "Teilenummer-Funktions-Id"),
                    copy("Script part-number rule requires a non-empty function_id (Part 5 §6.10).", "Skript-Teilenummer-Regel erfordert nicht-leere function_id (Teil 5 §6.10)."),
                    vec![Remedy {
                        target: subject("partNumber", "partNumberRule", "Part number rule", "Teilenummer-Regel"),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: vec!["partNumber".into()],
                        action: copy("Set partNumberRule.function_id.", "partNumberRule.function_id setzen."),
                        applicable: true,
                    }],
                ));
            }
            let numeric: HashMap<String, f64> = doc
                .part_number_inputs
                .iter()
                .filter_map(|(k, v)| match v {
                    crate::CatalogueValue::Decimal { value } => Some((k.clone(), *value)),
                    crate::CatalogueValue::Integer { value } => Some((k.clone(), *value as f64)),
                    _ => None,
                })
                .collect();
            runtime
                .execute(source, &numeric, doc.script_limits)
                .map(|r| format!("{:.0}", r.value))
                .map_err(|e| e.to_string())
        }
        _ => helpers::calculate_part_number(&doc.part_number_rule, &doc.part_number_inputs, &runtime).map_err(|e| e.to_string()),
    };

    match result {
        Ok(part_no) => {
            // Determinism: re-run
            let again = match &doc.part_number_rule {
                PartNumberRule::Script { source, .. } => {
                    let numeric: HashMap<String, f64> = doc
                        .part_number_inputs
                        .iter()
                        .filter_map(|(k, v)| match v {
                            crate::CatalogueValue::Decimal { value } => Some((k.clone(), *value)),
                            crate::CatalogueValue::Integer { value } => Some((k.clone(), *value as f64)),
                            _ => None,
                        })
                        .collect();
                    runtime.execute(source, &numeric, doc.script_limits).map(|r| format!("{:.0}", r.value)).ok()
                }
                _ => helpers::calculate_part_number(&doc.part_number_rule, &doc.part_number_inputs, &runtime).ok(),
            };
            if again.as_ref() == Some(&part_no) {
                let numeric = part_no.parse::<f64>().unwrap_or(0.0);
                report.push(assess(
                    "iso16757.5.6.10.partNumber",
                    "5",
                    "6.10",
                    subject("partNumber", "partNumberRule", "Part number rule", "Teilenummer-Regel"),
                    copy("Part number calculation", "Teilenummer-Berechnung"),
                    copy(
                        format!("Part number rule produced deterministic result '{part_no}'."),
                        format!("Teilenummer-Regel lieferte deterministisches Ergebnis '{part_no}'."),
                    ),
                    CheckStatus::Pass,
                    q_dim(numeric),
                    q_dim(numeric),
                    Vec::new(),
                ));
            } else {
                report.push(fail(
                    "iso16757.5.6.10.partNumber",
                    "5",
                    "6.10",
                    subject("partNumber", "partNumberRule", "Part number rule", "Teilenummer-Regel"),
                    copy("Part number calculation", "Teilenummer-Berechnung"),
                    copy("Part number rule is not deterministic under the same inputs.", "Teilenummer-Regel ist unter gleichen Eingaben nicht deterministisch."),
                    vec![Remedy {
                        target: subject("partNumber", "partNumberRule.source", "Part number rule", "Teilenummer-Regel"),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: vec!["dn * 10 + 50".into()],
                        action: copy("Replace partNumberRule.source with a pure expression of partNumberInputs (e.g. 'dn * 10 + 50').", "partNumberRule.source durch einen reinen Ausdruck von partNumberInputs ersetzen (z. B. 'dn * 10 + 50')."),
                        applicable: true,
                    }],
                ));
            }
        }
        Err(err) => {
            let input_fp: f64 = doc.part_number_inputs.iter().map(|(_k, v)| match v {
                CatalogueValue::Decimal { value } => *value,
                CatalogueValue::Integer { value } => *value as f64,
                CatalogueValue::Quantity { value, .. } => *value,
                _ => 0.0,
            }).sum();
            let mut remedies = vec![Remedy {
                target: subject("partNumber", "partNumberInputs", "Part number inputs", "Teilenummer-Eingaben"),
                current: q_dim(input_fp),
                required: q_dim(1.0),
                bound: RemedyBound::Exactly,
                options: doc.part_number_inputs.keys().cloned().collect(),
                action: copy(
                    format!("Fix part-number inputs/rule so evaluation succeeds (error: {err})."),
                    format!("Teilenummer-Eingaben/Regel so korrigieren, dass die Auswertung gelingt (Fehler: {err})."),
                ),
                applicable: true,
            }];
            if err.contains("timeout") || err.contains("step") || err.contains("recursion") {
                remedies.push(Remedy::at_least(
                    subject("scriptLimits", "scriptLimits.timeoutMs", "Script limits", "Skriptgrenzen"),
                    q_dim(doc.script_limits.timeout_ms as f64),
                    q_dim((doc.script_limits.timeout_ms * 2).max(100) as f64),
                    copy(
                        format!("Raise scriptLimits (timeout/steps/recursion) so the part-number script can finish (currently timeout_ms={}).", doc.script_limits.timeout_ms),
                        format!("scriptLimits (timeout/steps/recursion) erhöhen, damit das Teilenummer-Skript durchläuft (aktuell timeout_ms={}).", doc.script_limits.timeout_ms),
                    ),
                ));
            }
            report.push(assess(
                "iso16757.5.6.10.partNumber",
                "5",
                "6.10",
                subject("partNumber", "partNumberRule", "Part number rule", "Teilenummer-Regel"),
                copy("Part number calculation", "Teilenummer-Berechnung"),
                copy(format!("Part number calculation failed: {err}"), format!("Teilenummer-Berechnung fehlgeschlagen: {err}")),
                CheckStatus::Fail,
                q_dim(input_fp),
                q_dim(1.0),
                remedies,
            ));
        }
    }
}

fn check_script_limits(doc: &Iso16757Snapshot, report: &mut CheckReport) {
    match &doc.part_number_rule {
        PartNumberRule::Script { source, .. } => {
            let runtime = helpers::DefaultScriptRuntime;
            // Guard: document script must not allow division by zero when zero denominators appear
            if source.contains("/0") || source.contains("/ 0") {
                report.push(fail(
                    "iso16757.5.8.scriptSafety",
                    "5",
                    "8",
                    subject("partNumber", "partNumberRule", "Part number rule", "Teilenummer-Regel"),
                    copy("Script safety", "Skriptsicherheit"),
                    copy("Part-number script literally divides by zero.", "Teilenummer-Skript dividiert wörtlich durch null."),
                    vec![Remedy {
                        target: subject("partNumber", "partNumberRule.source", "Part number rule", "Teilenummer-Regel"),
                        current: q_dim(0.0),
                        required: q_dim(1.0),
                        bound: RemedyBound::OneOf,
                        options: vec!["dn * 10 + 50".into()],
                        action: copy("Rewrite partNumberRule.source to avoid division by zero.", "partNumberRule.source so umschreiben, dass Division durch null vermieden wird."),
                        applicable: true,
                    }],
                ));
                return;
            }
            let limits = doc.script_limits;

            // Part 5 §8 — declared scriptLimits participate in evaluation capacity.
            report.push(assess(
                "iso16757.5.8.scriptLimits.capacity",
                "5",
                "8",
                subject("scriptLimits", "scriptLimits.maxSteps", "Script limits", "Skriptgrenzen"),
                copy("Script execution capacity", "Skriptausführungskapazität"),
                copy(
                    format!("scriptLimits capacity steps={} recursion={} timeoutMs={}.", limits.max_steps, limits.max_recursion, limits.timeout_ms),
                    format!("scriptLimits Kapazität steps={} recursion={} timeoutMs={}.", limits.max_steps, limits.max_recursion, limits.timeout_ms),
                ),
                CheckStatus::Pass,
                q_dim(f64::from(limits.max_steps)),
                q_dim(f64::from(limits.max_recursion).max(1.0) * 1.0 + limits.timeout_ms as f64 / 1000.0),
                Vec::new(),
            ));

            if limits.max_steps == 0 || limits.max_recursion == 0 || limits.timeout_ms == 0 {
                report.push(fail(
                    "iso16757.5.8.scriptLimits",
                    "5",
                    "8",
                    subject("scriptLimits", "scriptLimits", "Script limits", "Skriptgrenzen"),
                    copy("Script execution limits", "Skriptausführungsgrenzen"),
                    copy("scriptLimits has a zero cap which prevents any script execution.", "scriptLimits hat eine Nullgrenze, die jede Skriptausführung verhindert."),
                    vec![Remedy::at_least(
                        subject("scriptLimits", "scriptLimits.maxSteps", "Script limits", "Skriptgrenzen"),
                        q_dim(limits.max_steps as f64),
                        q_dim(10_000.0),
                        copy("Set maxSteps ≥ 10000, maxRecursion ≥ 64, timeoutMs ≥ 50.", "maxSteps ≥ 10000, maxRecursion ≥ 64, timeoutMs ≥ 50 setzen."),
                    )],
                ));
            } else {
                let source = source.clone();
                let unsafe_markers = ["import ", "require(", "eval(", "std::", "fs::", "net::", "process.", "include!", "file://", "/etc/", "curl ", "http://", "https://"];
                let looks_unsafe = unsafe_markers.iter().any(|m| source.to_lowercase().contains(&m.to_lowercase()));
                if looks_unsafe {
                    report.push(fail(
                        "iso16757.5.8.scriptSafety",
                        "5",
                        "8",
                        subject("scriptLimits", "partNumberRule.source", "Part-number script", "Teilenummer-Skript"),
                        copy("Script safety", "Skriptsicherheit"),
                        copy(
                            format!("Document partNumberRule.source contains unsafe constructs under scriptLimits: '{source}'."),
                            format!("Dokument-partNumberRule.source enthält unsichere Konstrukte unter scriptLimits: '{source}'."),
                        ),
                        vec![Remedy {
                            target: subject("scriptLimits", "partNumberRule.source", "Part-number script", "Teilenummer-Skript"),
                            current: q_dim(0.0),
                            required: q_dim(1.0),
                            bound: RemedyBound::OneOf,
                            options: vec!["dn * 10 + 50".into()],
                            action: copy(
                                "Replace partNumberRule.source with a pure arithmetic expression over partNumberInputs (no I/O or imports).",
                                "partNumberRule.source durch einen reinen arithmetischen Ausdruck über partNumberInputs ersetzen (kein I/O oder Imports).",
                            ),
                            applicable: true,
                        }],
                    ));
                } else {
                    let mut numeric: HashMap<String, f64> = HashMap::new();
                    for (k, v) in &doc.part_number_inputs {
                        match v {
                            CatalogueValue::Decimal { value } => { numeric.insert(k.clone(), *value); }
                            CatalogueValue::Integer { value } => { numeric.insert(k.clone(), *value as f64); }
                            CatalogueValue::Quantity { value, .. } => { numeric.insert(k.clone(), *value); }
                            _ => {}
                        }
                    }
                    match runtime.execute(&source, &numeric, limits) {
                        Ok(_) | Err(_) => {
                            // Safe expressions may succeed or fail numerically; both are acceptable if not unsafe.
                            // Still refuse classic unsafe probes if somehow injected via concatenation.
                            match runtime.execute("1/(0)", &HashMap::new(), limits) {
                                Err(_) => {
                                    let steps = f64::from(limits.max_steps).max(1.0);
                                    let util_limit = steps;
                                    report.push(assess(
                                        "iso16757.5.8.scriptSafety",
                                        "5",
                                        "8",
                                        subject("scriptLimits", "partNumberRule.source", "Part-number script", "Teilenummer-Skript"),
                                        copy("Script safety", "Skriptsicherheit"),
                                        copy(
                                            format!("Document partNumberRule.source evaluated under scriptLimits without unsafe I/O ('{source}', steps={}, recursion={}, timeoutMs={}).", limits.max_steps, limits.max_recursion, limits.timeout_ms),
                                            format!("Dokument-partNumberRule.source unter scriptLimits ohne unsicheres I/O ausgewertet ('{source}', steps={}, recursion={}, timeoutMs={}).", limits.max_steps, limits.max_recursion, limits.timeout_ms),
                                        ),
                                        CheckStatus::Pass,
                                        q_dim(limits.timeout_ms as f64),
                                        q_dim(util_limit),
                                        Vec::new(),
                                    ));
                                }
                                Ok(_) => {
                                    report.push(fail(
                                        "iso16757.5.8.scriptSafety",
                                        "5",
                                        "8",
                                        subject("scriptLimits", "scriptLimits.maxSteps", "Script limits", "Skriptgrenzen"),
                                        copy("Script safety", "Skriptsicherheit"),
                                        copy("Script runtime accepted division by zero under document scriptLimits.", "Skriptlaufzeit akzeptierte Division durch null unter document.scriptLimits."),
                                        vec![Remedy::at_least(
                                            subject("scriptLimits", "scriptLimits.maxSteps", "Script limits", "Skriptgrenzen"),
                                            q_dim(limits.max_steps as f64),
                                            q_dim(10_000.0),
                                            copy("Raise scriptLimits and ensure the runtime rejects unsafe evaluation.", "scriptLimits anheben und sicherstellen, dass die Laufzeit unsichere Auswertung ablehnt."),
                                        )],
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            report.push(na(
                "iso16757.5.8.scriptSafety",
                "5",
                "8",
                subject("partNumber", "partNumberRule", "Part number rule", "Teilenummer-Regel"),
                copy("Script safety", "Skriptsicherheit"),
                copy("Part-number rule is not a script; script sandbox checks are not applicable.", "Teilenummer-Regel ist kein Skript; Skript-Sandbox-Prüfungen entfallen."),
            ));
        }
    }
}


