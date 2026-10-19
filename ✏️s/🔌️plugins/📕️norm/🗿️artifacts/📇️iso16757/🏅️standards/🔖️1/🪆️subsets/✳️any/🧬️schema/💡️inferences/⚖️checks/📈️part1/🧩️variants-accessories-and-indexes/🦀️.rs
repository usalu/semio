//! 🧩️ ISO 16757-1 variant article and geometry, accessory composition and product index checks.

use super::*;

pub(super) fn check_variant_article_and_geometry(catalogue: &Catalogue, geometry: &semio_s_artifact_norm_iso16757_model::part_2::GeometryCatalogue, report: &mut CheckReport) {
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

pub(super) fn check_accessories_compositions(catalogue: &Catalogue, report: &mut CheckReport) {
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
                {
                    let mut opts: Vec<String> = catalogue
                        .products
                        .iter()
                        .map(|p| p.id.clone())
                        .filter(|id| id != &product.id)
                        .collect();
                    if opts.is_empty() {
                        opts.push(format!("{}-open", product.id));
                    }
                    vec![Remedy::one_of(
                        subject(
                            &product.id,
                            format!("catalogue.compositions.{}[0].componentProductId", product.id),
                            &product.names.preferred.text,
                            &product.names.preferred.text,
                        ),
                        opts,
                        copy(
                            format!("Retarget the composition edge of '{}' to a non-cyclic component product id.", product.id),
                            format!("Zusammensetzungs-Kante von '{}' auf eine nicht-zyklische Komponenten-Produkt-Id umbiegen.", product.id),
                        ),
                    )]
                },
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

pub(super) fn check_product_indexes(catalogue: &Catalogue, report: &mut CheckReport) {
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
                vec![Remedy::one_of(
                    subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[0]", index.id), &index.id, &index.id),
                    expected_needles.clone(),
                    copy("Set a search tag to the product id or preferred name.", "Suchbegriff auf Produkt-Id oder bevorzugten Namen setzen."),
                )],
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
                        vec![Remedy::one_of(
                            subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[{ti}]", index.id), &index.id, &index.id),
                            expected_needles.clone(),
                            copy("Set a non-empty search tag containing the product id or name.", "Nicht-leeren Suchbegriff mit Produkt-Id oder Name setzen."),
                        )]
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
                        vec![Remedy::one_of(
                            subject(&index.id, format!("catalogue.productIndexes[id={}].searchTags[{ti}]", index.id), &index.id, &index.id),
                            expected_needles.clone(),
                            copy("Set a non-empty search tag containing the product id or name.", "Nicht-leeren Suchbegriff mit Produkt-Id oder Name setzen."),
                        )]
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
