//! 🎯️ ISO 16757-1 selection and BIM embedding checks.

use super::*;

pub(super) fn check_selection(catalogue: &Catalogue, selection: &SelectionRequest, report: &mut CheckReport) {
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
        let mut prop_metric = 0.0;
        let mut value_opts: Vec<String> = Vec::new();
        let mut prop_opts: Vec<String> = Vec::new();
        for product in &catalogue.products {
            let Some(series) = catalogue.product_series.iter().find(|s| s.id == product.series_id) else { continue };
            if series.class_id != selection.class_id { continue; }
            for pv in product.static_properties.iter().chain(product.variants.iter().flat_map(|v| v.property_values.iter())) {
                match &pv.value {
                    CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => {
                        prop_metric += *value;
                        prop_opts.push(value.to_string());
                    }
                    CatalogueValue::Integer { value } => {
                        prop_metric += *value as f64;
                        prop_opts.push(value.to_string());
                    }
                    CatalogueValue::Text { value } | CatalogueValue::Enumeration { value } | CatalogueValue::Identifier { value } => {
                        prop_opts.push(value.clone());
                    }
                    _ => {}
                }
            }
            for variant in &product.variants {
                for (_k, v) in &variant.parameter_values {
                    match v {
                        CatalogueValue::Decimal { value } | CatalogueValue::Quantity { value, .. } => value_opts.push(value.to_string()),
                        CatalogueValue::Integer { value } => value_opts.push(value.to_string()),
                        _ => {}
                    }
                }
            }
        }
        prop_opts.sort();
        prop_opts.dedup();
        value_opts.sort();
        value_opts.dedup();
        let mut opts = prop_opts;
        for v in value_opts {
            if !opts.contains(&v) {
                opts.push(v);
            }
        }
        let constraint_id = selection.constraints.first().map(|c| c.id.clone()).unwrap_or_else(|| "constraint".into());
        let mut remedies = Vec::new();
        if !opts.is_empty() {
            remedies.push(Remedy::one_of(
                subject("selection", format!("selection.constraints[id={}].value.value", constraint_id), "Selection constraint value", "Auswahlbedingungswert"),
                opts,
                copy(
                    "Set the selection constraint value to a real property/parameter value so at least one product index matches.",
                    "Auswahl-Constraint-Wert auf einen realen Eigenschaft-/Parameterwert setzen, damit mindestens ein Produktindex passt.",
                ),
            ));
        }
        remedies.push(Remedy::one_of(
            subject("selection", "selection.classId", "Selection class", "Auswahlklasse"),
            catalogue.product_classes.iter().map(|c| c.id.clone()).collect(),
            copy(
                "Set selection.classId to a class that has at least one matching product index.",
                "selection.classId auf eine Klasse mit mindestens einem passenden Produktindex setzen.",
            ),
        ));
        report.push(assess(
            "iso16757.1.4.2.selection.empty",
            "1",
            "4.2",
            subject("selection", "selection", "Selection", "Auswahl"),
            title,
            copy(
                format!("Selection for class '{}' matched no products. {}", selection.class_id, result.explanations.join("; ")),
                format!("Auswahl für Klasse '{}' fand keine Produkte. {}", selection.class_id, result.explanations.join("; ")),
            ),
            CheckStatus::Fail,
            q_dim(prop_metric),
            q_dim(1.0),
            remedies,
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

pub(super) fn check_bim_embeddings(catalogue: &Catalogue, report: &mut CheckReport) {
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
        match helpers::resolve_bim_embedding(catalogue, &index.id, params.clone()) {
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
                            vec![Remedy::one_of(
                                subject(&index.id, format!("catalogue.products[id={}].variants[id={}].geometryId", index.product_id, variant_id), &index.id, &index.id),
                                geom_opts,
                                copy(
                                    "Set geometryId on the resolved variant to an existing geometry object.",
                                    "geometryId der Variante auf ein vorhandenes Geometrieobjekt setzen.",
                                ),
                            )]
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
{
                        let variant_id = index.variant_id.clone().unwrap_or_else(|| "variant".into());
                        let mut remedies = Vec::new();
                        for (param_id, value) in &params {
                            if let Some(domain) = product.parameter_domains.iter().find(|d| &d.parameter_id == param_id) {
                                if !domain.allowed_values.is_empty() && !domain.allowed_values.contains(value) {
                                    let options: Vec<String> = domain
                                        .allowed_values
                                        .iter()
                                        .filter_map(|v| match v {
                                            CatalogueValue::Decimal { value } => Some(value.to_string()),
                                            CatalogueValue::Integer { value } => Some(value.to_string()),
                                            CatalogueValue::Text { value } | CatalogueValue::Enumeration { value } => Some(value.clone()),
                                            _ => None,
                                        })
                                        .collect();
                                    if !options.is_empty() {
                                        remedies.push(Remedy::one_of(
                                            subject(
                                                &index.id,
                                                format!(
                                                    "catalogue.products[id={}].variants[id={}].parameterValues.{param_id}.value",
                                                    index.product_id, variant_id
                                                ),
                                                &index.id,
                                                &index.id,
                                            ),
                                            options,
                                            copy(
                                                format!("Set parameter '{param_id}' to an allowed domain value so BIM embedding can resolve."),
                                                format!("Parameter '{param_id}' auf einen zulässigen Bereichswert setzen, damit die BIM-Einbettung auflöst."),
                                            ),
                                        ));
                                    }
                                }
                            }
                        }
                        let mut geom_opts: Vec<String> = catalogue.product_series.iter().filter_map(|s| s.geometry_id.clone()).collect();
                        geom_opts.extend(catalogue.products.iter().flat_map(|p| p.variants.iter().filter_map(|v| v.geometry_id.clone())));
                        geom_opts.retain(|g| !g.contains("missing"));
                        geom_opts.sort();
                        geom_opts.dedup();
                        if !geom_opts.is_empty() {
                            remedies.push(Remedy::one_of(
                                subject(
                                    &index.id,
                                    format!("catalogue.products[id={}].variants[id={}].geometryId", index.product_id, variant_id),
                                    &index.id,
                                    &index.id,
                                ),
                                geom_opts,
                                copy(
                                    "Set geometryId on the resolved variant to an existing geometry object.",
                                    "geometryId der Variante auf ein vorhandenes Geometrieobjekt setzen.",
                                ),
                            ));
                        }
                        let product_opts: Vec<String> = catalogue.products.iter().map(|p| p.id.clone()).filter(|id| id != &index.product_id).collect();
                        if !product_opts.is_empty() {
                            remedies.push(Remedy::one_of(
                                subject(&index.id, format!("catalogue.productIndexes[id={}].productId", index.id), &index.id, &index.id),
                                product_opts,
                                copy(
                                    "Set productIndexes.productId to a resolvable catalogue product.",
                                    "productIndexes.productId auf ein auflösbares Katalogprodukt setzen.",
                                ),
                            ));
                        }
                        if remedies.is_empty() {
                            remedies.push(Remedy::one_of(
                                subject(&index.id, format!("catalogue.productIndexes[id={}].productId", index.id), &index.id, &index.id),
                                catalogue.products.iter().map(|p| p.id.clone()).collect(),
                                copy(
                                    "Set productIndexes.productId to a resolvable catalogue product.",
                                    "productIndexes.productId auf ein auflösbares Katalogprodukt setzen.",
                                ),
                            ));
                        }
                        remedies
                    }
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
