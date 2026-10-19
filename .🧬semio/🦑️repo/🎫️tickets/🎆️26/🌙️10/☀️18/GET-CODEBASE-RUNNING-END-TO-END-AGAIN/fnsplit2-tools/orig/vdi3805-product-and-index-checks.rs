//! 🗃️ Product and index checks: the product ids, the dangling index entries, the per product geometry and the index to product consistency of the catalog integrity check. See [`super`].

use super::*;

/// 🆔 Checks that every product id is unique.
pub(super) fn check_unique_product_ids(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>, product_ids: &BTreeSet<String>) {
    let unique_ok = product_ids.len() == document.catalog.products.len();
    let mut dup = CheckResult::assess(
        "vdi3805.1.products.uniqueId",
        part_label(1),
        clause("1", "4.1"),
        subject_dataset(),
        copy("Unique product ids", "Eindeutige Produkt-IDs"),
    )
    .annex(ANNEX)
    .minimum(
        Quantity::new(QuantityKind::Dimensionless, product_ids.len() as f64),
        Quantity::new(QuantityKind::Dimensionless, document.catalog.products.len() as f64),
    )
    .status(if unique_ok { CheckStatus::Pass } else { CheckStatus::Fail });
    if unique_ok {
        dup = dup.explanation(copy("Every catalogue product id is unique.", "Jede Katalog-Produkt-ID ist eindeutig."));
    } else {
        dup = dup
            .explanation(copy("Duplicate product ids in catalog.products.", "Doppelte Produkt-IDs in catalog.products."))
            .remedy(Remedy::exactly(
                SubjectRef::new("", "catalog.products[0].id", copy("Product id", "Produkt-ID")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                copy("Give each catalogue product a unique id.", "Jeder Katalogprodukt eine eindeutige ID geben."),
            ));
    }
    out.push(dup.build());
}

/// 🔗 Checks that every index entry points at a declared product.
pub(super) fn check_dangling_index_entries(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>, product_ids: &BTreeSet<String>) {
    let mut dangling_index = Vec::new();
    for (i, entry) in document.index.entries.iter().enumerate() {
        if !product_ids.contains(&entry.product_id) {
            dangling_index.push(i);
        }
    }
    let mut idx = CheckResult::assess(
        "vdi3805.1.index",
        part_label(1),
        clause("1", "4.4"),
        subject_dataset(),
        copy("Catalogue index references", "Katalogindex-Referenzen"),
    )
    .annex(ANNEX);
    if dangling_index.is_empty() {
        idx = idx.status(CheckStatus::Pass).explanation(copy("Every index entry resolves to a catalogue product.", "Jeder Indexeintrag verweist auf ein Katalogprodukt."));
    } else {
        let i0 = dangling_index[0];
        let bad = &document.index.entries[i0].product_id;
        idx = idx
            .status(CheckStatus::Fail)
            .explanation(copy(
                format!("Index entries reference missing products (first `{bad}`)."),
                format!("Indexeinträge verweisen auf fehlende Produkte (zuerst `{bad}`)."),
            ))
            .remedy(Remedy::one_of(
                SubjectRef::new("", format!("index.entries[{i0}].productId"), copy("Index product id", "Index-Produkt-ID")),
                product_ids.iter().cloned().collect(),
                copy("Point index.entries[].productId at an existing catalogue product id.", "index.entries[].productId auf eine vorhandene Katalog-Produkt-ID setzen."),
            ));
    }
    out.push(idx.build());
}

/// 📦 Checks the identity, the records and the geometry of every product.
pub(super) fn check_products(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>, product_ids: &BTreeSet<String>) {
    for product in &document.catalog.products {
        let article = product.id.as_str();
        let record_100 = product.records.iter().find(|r| r.family.0 == "100");
        let group_matches = record_100.and_then(|r| r.fields.get(2)).map(|g| g == &product.identity.product_group).unwrap_or(false);
        let article_matches = record_100.and_then(|r| r.fields.get(3)).map(|a| a == &product.identity.article_number).unwrap_or(false);
        let mfr_matches = record_100.and_then(|r| r.fields.get(1)).map(|m| m == &product.identity.manufacturer_code).unwrap_or(false);
        let id_ok = product.id == product.identity.article_number
            && !product.identity.manufacturer_code.is_empty()
            && !product.identity.product_group.is_empty()
            && group_matches
            && article_matches
            && mfr_matches;
        let mut idc = CheckResult::assess(
            format!("vdi3805.1.identity.{article}"),
            part_label(1),
            clause("1", "4.1"),
            subject_product(product),
            copy("Product identity consistency", "Produktidentitätskonsistenz"),
        )
        .annex(ANNEX)
        .status(if id_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if id_ok {
            idc = idc.explanation(copy(
                format!("id `{}` matches identity.articleNumber; manufacturer/group present.", article),
                format!("id `{}` entspricht identity.articleNumber; Hersteller/Gruppe vorhanden.", article),
            ));
        } else {
            idc = idc
                .explanation(copy(
                    "product.id must equal identity.articleNumber; manufacturerCode and productGroup must be non-empty.",
                    "product.id muss identity.articleNumber entsprechen; manufacturerCode und productGroup dürfen nicht leer sein.",
                ))
            .remedy(Remedy::exactly(
                SubjectRef::new(article, product_path(article, "identity.articleNumber"), copy("Article number", "Artikelnummer")),
                Quantity::new(QuantityKind::Dimensionless, 0.0),
                Quantity::new(QuantityKind::Dimensionless, 1.0),
                copy("Align identity.articleNumber with product.id.", "identity.articleNumber an product.id angleichen."),
            ));
        }
        out.push(idc.build());

        // Configuration id must be `cfg.{articleNumber}` (Part 1 configuration record).
        let expected_cfg = format!("cfg.{article}");
        let cfg_ok = product.configuration.id == expected_cfg;
        let mut cfg = CheckResult::assess(
            format!("vdi3805.1.configuration.id.{article}"),
            part_label(1),
            clause("1", "4.1"),
            subject_product(product),
            copy("Configuration id", "Konfigurations-ID"),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, if cfg_ok { 1.0 } else { 0.0 }), Quantity::new(QuantityKind::Dimensionless, 1.0))
        .status(if cfg_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if cfg_ok {
            cfg = cfg.explanation(copy(format!("configuration.id `{expected_cfg}` matches article."), format!("configuration.id `{expected_cfg}` entspricht der Artikelnummer.")));
        } else {
            cfg = cfg
                .explanation(copy(
                    format!("configuration.id `{}` must equal `{expected_cfg}`.", product.configuration.id),
                    format!("configuration.id `{}` muss `{expected_cfg}` sein.", product.configuration.id),
                ))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, "configuration.id"), copy("Configuration id", "Konfigurations-ID")),
                    vec![expected_cfg.clone()],
                    copy(format!("Set configuration.id to `{expected_cfg}`."), format!("configuration.id auf `{expected_cfg}` setzen.")),
                ));
        }
        out.push(cfg.build());

        if let Some((ri, rec)) = product.records.iter().enumerate().find(|(_, r)| r.family.0 == "700") {
            let de = rec.fields.get(2).cloned().unwrap_or_default();
            let en = rec.fields.get(4).cloned().unwrap_or_default();
            let title_de = text_in(&product.title, "de");
            let title_en = text_in(&product.title, "en");
            let de_ok = !de.is_empty() && de == title_de;
            let en_ok = !en.is_empty() && en == title_en;
            for (locale, ok, field_i, got, want) in [
                ("de", de_ok, 2usize, de.clone(), title_de.clone()),
                ("en", en_ok, 4usize, en.clone(), title_en.clone()),
            ] {
                let mut tc = CheckResult::assess(
                    format!("vdi3805.1.title.700.{article}.{locale}"),
                    part_label(1),
                    clause("1", "4.1"),
                    subject_product(product),
                    copy("Record 700 title", "Satz-700-Titel"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Dimensionless, if ok { 1.0 } else { 0.0 }), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if ok {
                    tc = tc.explanation(copy(format!("700 {locale} title matches product.title."), format!("700-{locale}-Titel entspricht product.title.")));
                } else {
                    tc = tc
                        .explanation(copy(
                            format!("records[{ri}].fields[{field_i}] `{got}` must equal product.title[{locale}] `{want}`."),
                            format!("records[{ri}].fields[{field_i}] `{got}` muss product.title[{locale}] `{want}` entsprechen."),
                        ))
                        .remedy(Remedy::exactly(
                            SubjectRef::new(article, product_path(article, &format!("records[{ri}].fields[{field_i}]")), copy("Record 700 title field", "Satz-700-Titelfeld")),
                            Quantity::new(QuantityKind::Dimensionless, 0.0),
                            Quantity::new(QuantityKind::Dimensionless, 1.0),
                            copy("Align record 700 title fields with product.title.", "Satz-700-Titelfelder an product.title angleichen."),
                        ));
                }
                out.push(tc.build());
            }
        }

        // Native Part 1 records: fields[0] must echo the family code; identity fields must match record 100 (VDI 3805-1 §4.1).
        for (ri, record) in product.records.iter().enumerate() {
            let family = record.family.0.as_str();
            let head_ok = record.fields.first().map(|f| f.as_str() == family).unwrap_or(false);
            let mut rf = CheckResult::assess(
                format!("vdi3805.1.records.family.{article}.{ri}"),
                part_label(1),
                clause("1", "4.1"),
                subject_product(product),
                copy("Native record family field", "Nativer Datensatz-Familienfeld"),
            )
            .annex(ANNEX)
            .status(if head_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if head_ok {
                rf = rf.explanation(copy(format!("records[{ri}].fields[0] matches family `{family}`."), format!("records[{ri}].fields[0] entspricht Familie `{family}`.")));
            } else {
                rf = rf
                    .explanation(copy(
                        format!("records[{ri}].fields[0] must equal family code `{family}`."),
                        format!("records[{ri}].fields[0] muss dem Familiencode `{family}` entsprechen."),
                    ))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(article, product_path(article, &format!("records[{ri}].fields[0]")), copy("Record family field", "Datensatz-Familienfeld")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy(format!("Set records[{ri}].fields[0] to `{family}`."), format!("records[{ri}].fields[0] auf `{family}` setzen.")),
                    ));
            }
            out.push(rf.build());
            // Each subsequent field contributes to Part 1 arity — empty tokens Fail.
            for (fi, field) in record.fields.iter().enumerate().skip(1) {
                let ok = !field.is_empty();
                let mut ff = CheckResult::assess(
                    format!("vdi3805.1.records.field.{article}.{ri}.{fi}"),
                    part_label(1),
                    clause("1", "4.1"),
                    subject_product(product),
                    copy("Native record field", "Natives Datensatzfeld"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Dimensionless, if ok { 1.0 } else { 0.0 }), Quantity::new(QuantityKind::Dimensionless, 1.0));
                if ok {
                    ff = ff.explanation(copy(format!("records[{ri}].fields[{fi}] present."), format!("records[{ri}].fields[{fi}] vorhanden.")));
                } else {
                    ff = ff
                        .explanation(copy(format!("records[{ri}].fields[{fi}] is empty."), format!("records[{ri}].fields[{fi}] ist leer.")))
                        .remedy(Remedy::exactly(
                            SubjectRef::new(article, product_path(article, &format!("records[{ri}].fields[{fi}]")), copy("Record field", "Datensatzfeld")),
                            Quantity::new(QuantityKind::Dimensionless, 0.0),
                            Quantity::new(QuantityKind::Dimensionless, 1.0),
                            copy("Fill the native record field per Part 1 arity.", "Natives Datensatzfeld gemäß Teil-1-Stellenzahl füllen."),
                        ));
                }
                out.push(ff.build());
            }
        }

        let accessory_ids: Vec<String> = product.accessories.iter().map(|a| a.accessory_id.clone()).collect();
        let dangling_acc: Vec<_> = accessory_ids.iter().filter(|id| !product_ids.contains(*id)).cloned().collect();
        let mut c = CheckResult::assess(
            format!("vdi3805.1.accessories.{article}"),
            part_label(1),
            clause("1", "4.5"),
            subject_product(product),
            copy("Product accessories", "Produktzubehör"),
        )
        .annex(ANNEX);
        if dangling_acc.is_empty() {
            c = c.status(CheckStatus::Pass).explanation(copy(
                "All accessory references resolve.",
                "Alle Zubehörreferenzen sind auflösbar.",
            ));
        } else {
            let ai = product
                .accessories
                .iter()
                .position(|a| !product_ids.contains(&a.accessory_id))
                .unwrap_or(0);
            c = c
                .status(CheckStatus::Fail)
                .explanation(copy(
                    format!("accessories reference unknown products {dangling_acc:?}."),
                    format!("accessories verweist auf unbekannte Produkte {dangling_acc:?}."),
                ))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, &format!("accessories[{ai}].accessoryId")), copy("Accessory id", "Zubehör-ID")),
                    product_ids.iter().cloned().collect(),
                    copy(
                        "Point accessories[].accessoryId at an existing catalogue product id.",
                        "accessories[].accessoryId auf eine vorhandene Katalog-Produkt-ID setzen.",
                    ),
                ));
        }
        out.push(c.build());

        let component_ids: Vec<String> = product.components.iter().map(|c| c.component_id.clone()).collect();
        let dangling_comp: Vec<_> = component_ids.iter().filter(|id| !product_ids.contains(*id)).cloned().collect();
        let mut c = CheckResult::assess(
            format!("vdi3805.1.components.{article}"),
            part_label(1),
            clause("1", "4.5"),
            subject_product(product),
            copy("Product components", "Produktkomponenten"),
        )
        .annex(ANNEX);
        if dangling_comp.is_empty() {
            c = c.status(CheckStatus::Pass).explanation(copy(
                "All component references resolve.",
                "Alle Komponentenreferenzen sind auflösbar.",
            ));
        } else {
            let ci = product
                .components
                .iter()
                .position(|c| !product_ids.contains(&c.component_id))
                .unwrap_or(0);
            c = c
                .status(CheckStatus::Fail)
                .explanation(copy(
                    format!("components reference unknown products {dangling_comp:?}."),
                    format!("components verweist auf unbekannte Produkte {dangling_comp:?}."),
                ))
                .remedy(Remedy::one_of(
                    SubjectRef::new(article, product_path(article, &format!("components[{ci}].componentId")), copy("Component id", "Komponenten-ID")),
                    product_ids.iter().cloned().collect(),
                    copy(
                        "Point components[].componentId at an existing catalogue product id.",
                        "components[].componentId auf eine vorhandene Katalog-Produkt-ID setzen.",
                    ),
                ));
        }
        out.push(c.build());

        // Accessories quantity ≥1 and required accessories present (VDI 3805-1 §4.5).
        for (i, link) in product.accessories.iter().enumerate() {
            let qty_ok = link.quantity >= 1;
            let mut q = CheckResult::assess(
                format!("vdi3805.1.accessories.quantity.{article}.{i}"),
                part_label(1),
                clause("1", "4.5"),
                subject_product(product),
                copy("Accessory quantity", "Zubehöranzahl"),
            )
            .annex(ANNEX)
            .minimum(Quantity::new(QuantityKind::Dimensionless, link.quantity as f64), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .status(if qty_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if qty_ok {
                q = q.explanation(copy(format!("Accessory `{}` quantity {} ≥ 1.", link.accessory_id, link.quantity), format!("Zubehör `{}` Anzahl {} ≥ 1.", link.accessory_id, link.quantity)));
            } else {
                q = q
                    .explanation(copy("Accessory quantity must be ≥ 1.", "Zubehöranzahl muss ≥ 1 sein."))
                    .remedy(Remedy::at_least(
                        SubjectRef::new(article, product_path(article, &format!("accessories[{i}].quantity")), copy("Accessory quantity", "Zubehöranzahl")),
                        Quantity::new(QuantityKind::Dimensionless, link.quantity as f64),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Set accessories[].quantity to at least 1.", "accessories[].quantity auf mindestens 1 setzen."),
                    ));
            }
            out.push(q.build());
            let present = product_ids.contains(&link.accessory_id);
            let req_ok = !link.required || present;
            let mut rq = CheckResult::assess(
                format!("vdi3805.1.accessories.required.{article}.{i}"),
                part_label(1),
                clause("1", "4.5"),
                subject_product(product),
                copy("Required accessory present", "Erforderliches Zubehör vorhanden"),
            )
            .annex(ANNEX)
            .minimum(
                Quantity::new(QuantityKind::Dimensionless, if present { 1.0 } else { 0.0 }),
                Quantity::new(QuantityKind::Dimensionless, if link.required { 1.0 } else { 0.0 }),
            )
            .status(if req_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if req_ok {
                rq = rq.explanation(copy(
                    format!("Accessory `{}` required={} present={}.", link.accessory_id, link.required, present),
                    format!("Zubehör `{}` required={} present={}.", link.accessory_id, link.required, present),
                ));
            } else {
                rq = rq
                    .explanation(copy(
                        format!("Required accessory `{}` is missing from the catalogue.", link.accessory_id),
                        format!("Erforderliches Zubehör `{}` fehlt im Katalog.", link.accessory_id),
                    ))
                    .remedy(Remedy::exactly(
                        SubjectRef::new(article, product_path(article, &format!("accessories[{i}].accessoryId")), copy("Accessory id", "Zubehör-ID")),
                        Quantity::new(QuantityKind::Dimensionless, 0.0),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Add the required accessory product to the catalogue.", "Erforderliches Zubehörprodukt dem Katalog hinzufügen."),
                    ));
            }
            out.push(rq.build());
        }
        for (i, link) in product.components.iter().enumerate() {
            let qty_ok = link.quantity >= 1;
            let mut q = CheckResult::assess(
                format!("vdi3805.1.components.quantity.{article}.{i}"),
                part_label(1),
                clause("1", "4.6"),
                subject_product(product),
                copy("Component quantity", "Komponentenanzahl"),
            )
            .annex(ANNEX)
            .minimum(Quantity::new(QuantityKind::Dimensionless, link.quantity as f64), Quantity::new(QuantityKind::Dimensionless, 1.0))
            .status(if qty_ok { CheckStatus::Pass } else { CheckStatus::Fail });
            if qty_ok {
                q = q.explanation(copy(format!("Component `{}` quantity {} ≥ 1.", link.component_id, link.quantity), format!("Komponente `{}` Anzahl {} ≥ 1.", link.component_id, link.quantity)));
            } else {
                q = q
                    .explanation(copy("Component quantity must be ≥ 1.", "Komponentenanzahl muss ≥ 1 sein."))
                    .remedy(Remedy::at_least(
                        SubjectRef::new(article, product_path(article, &format!("components[{i}].quantity")), copy("Component quantity", "Komponentenanzahl")),
                        Quantity::new(QuantityKind::Dimensionless, link.quantity as f64),
                        Quantity::new(QuantityKind::Dimensionless, 1.0),
                        copy("Set components[].quantity to at least 1.", "components[].quantity auf mindestens 1 setzen."),
                    ));
            }
            out.push(q.build());
        }

        // Geometry bbox / connections / parameters when geometryRef set (VDI 3805-1 §5.2).
        if let Some(gref) = product.configuration.geometry_ref.as_ref() {
            if let Some(geom) = document.geometry.get(gref) {
                let bbox = &geom.bbox;
                let mut outside = Vec::new();
                for conn in &geom.connections {
                    let [x, y, z] = conn.position;
                    if x < bbox.min_x || x > bbox.max_x || y < bbox.min_y || y > bbox.max_y || z < bbox.min_z || z > bbox.max_z {
                        outside.push(conn.id.clone());
                    }
                }
                let bbox_ok = outside.is_empty() && bbox.max_x >= bbox.min_x && bbox.max_y >= bbox.min_y && bbox.max_z >= bbox.min_z;
                let volume = ((bbox.max_x - bbox.min_x) * (bbox.max_y - bbox.min_y) * (bbox.max_z - bbox.min_z)).abs();
                let mut g = CheckResult::assess(
                    format!("vdi3805.1.geometry.bbox.{article}"),
                    part_label(1),
                    clause("1", "5.2"),
                    subject_product(product),
                    copy("Geometry bounding box", "Geometrie-Begrenzungsrahmen"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Volume, if bbox_ok { volume.max(1e-12) } else { 0.0 }), Quantity::new(QuantityKind::Volume, 1e-12))
                .status(if bbox_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if bbox_ok {
                    g = g.explanation(copy("All connection points lie inside the geometry bbox.", "Alle Anschlusspunkte liegen im Geometrie-BBox."));
                } else {
                    g = g
                        .explanation(copy(format!("Connection points outside bbox: {outside:?}."), format!("Anschlusspunkte außerhalb der BBox: {outside:?}.")))
                        .remedy(Remedy::exactly(
                            SubjectRef::new(gref, format!("geometry[id={gref}].bbox.maxX"), copy("BBox maximum X", "Maximale X-Koordinate der Bounding-Box")),
                            Quantity::new(QuantityKind::Length, bbox.max_x),
                            Quantity::new(QuantityKind::Length, bbox.max_x.max(bbox.min_x) + 0.01),
                            copy("Expand geometry bbox so every connection point is inside.", "Geometrie-BBox so erweitern, dass jeder Anschlusspunkt innen liegt."),
                        ));
                }
                out.push(g.build());
                let scale = geom.parameters.get("scale").copied().unwrap_or(0.0);
                let params_ok = scale > 0.0;
                let mut p = CheckResult::assess(
                    format!("vdi3805.1.geometry.parameters.{article}"),
                    part_label(1),
                    clause("1", "5.2"),
                    subject_product(product),
                    copy("Geometry parameters", "Geometrieparameter"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Dimensionless, scale), Quantity::new(QuantityKind::Dimensionless, 1e-6))
                .status(if params_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if params_ok {
                    p = p.explanation(copy(format!("geometry.parameters.scale = {scale}."), format!("geometry.parameters.scale = {scale} (Geometrie).")));
                } else {
                    p = p
                        .explanation(copy("geometryRef set but parameters.scale is missing or not positive.", "geometryRef gesetzt, aber parameters.scale fehlt oder ist nicht positiv."))
                        .remedy(Remedy::at_least(
                            SubjectRef::new(gref, format!("geometry[id={gref}].parameters.scale"), copy("Geometry scale", "Geometrie-Maßstab")),
                            Quantity::new(QuantityKind::Dimensionless, scale),
                            Quantity::new(QuantityKind::Dimensionless, 1.0),
                            copy("Set geometry.parameters.scale to a positive value.", "geometry.parameters.scale auf einen positiven Wert setzen."),
                        ));
                }
                out.push(p.build());
                for (pk, pv) in &geom.parameters {
                    let mut pkc = CheckResult::assess(
                        format!("vdi3805.1.geometry.parameter.{article}.{pk}"),
                        part_label(1),
                        clause("1", "5.2"),
                        subject_product(product),
                        copy("Geometry parameter", "Geometrieparameter"),
                    )
                    .annex(ANNEX)
                    .minimum(Quantity::new(QuantityKind::Dimensionless, *pv), Quantity::new(QuantityKind::Dimensionless, 0.0))
                    .status(if pv.is_finite() && *pv >= 0.0 { CheckStatus::Pass } else { CheckStatus::Fail });
                    if pv.is_finite() && *pv >= 0.0 {
                        pkc = pkc.explanation(copy(format!("parameter `{pk}` = {pv}."), format!("Parameter `{pk}` = {pv} (Geometrie).")));
                    } else {
                        pkc = pkc
                            .explanation(copy(format!("parameter `{pk}` must be a finite non-negative number."), format!("Parameter `{pk}` muss eine endliche nicht-negative Zahl sein.")))
                            .remedy(Remedy::at_least(
                                SubjectRef::new(gref, format!("geometry[id={gref}].parameters.{pk}"), copy("Geometry parameter", "Geometrieparameter")),
                                Quantity::new(QuantityKind::Dimensionless, *pv),
                                Quantity::new(QuantityKind::Dimensionless, 0.0),
                                copy("Set the geometry parameter to a non-negative finite value.", "Geometrieparameter auf einen nicht-negativen endlichen Wert setzen."),
                            ));
                    }
                    out.push(pkc.build());
                }
                for (ci, conn) in geom.connections.iter().enumerate() {
                    let dia = conn.diameter_mm.unwrap_or(0.0);
                    let mut cd = CheckResult::assess(
                        format!("vdi3805.1.geometry.connection.{article}.{ci}"),
                        part_label(1),
                        clause("1", "5.2"),
                        subject_product(product),
                        copy("Geometry connection", "Geometrieanschluss"),
                    )
                    .annex(ANNEX)
                    .minimum(Quantity::new(QuantityKind::Length, dia / 1000.0), Quantity::new(QuantityKind::Length, 0.001));
                    let medium_ok = CONNECTION_MEDIUM_CODES.iter().any(|m| *m == conn.medium.as_str());
                    let id_ok = CONNECTION_ID_CODES.iter().any(|m| *m == conn.id.as_str());
                    let conn_ok = dia >= 1.0 && medium_ok && id_ok;
                    cd = cd.status(if conn_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                    if conn_ok {
                        cd = cd.explanation(copy(
                            format!("Connection `{}` medium={} Ø={} mm.", conn.id, conn.medium, dia),
                            format!("Anschluss `{}` Medium={} Ø={} mm.", conn.id, conn.medium, dia),
                        ));
                    } else if !id_ok {
                        cd = cd
                            .explanation(copy(
                                format!("Connection id `{}` is not in the admitted connection id list.", conn.id),
                                format!("Anschluss-ID `{}` ist nicht in der zugelassenen Anschluss-ID-Liste.", conn.id),
                            ))
                            .remedy(Remedy::one_of(
                                SubjectRef::new(gref, format!("geometry[id={gref}].connections[id={}].id", conn.id), copy("Connection id", "Anschluss-ID")),
                                CONNECTION_ID_CODES.iter().map(|s| (*s).to_string()).collect(),
                                copy("Set connection.id to an admitted connection id.", "connection.id auf eine zugelassene Anschluss-ID setzen."),
                            ));
                    } else {
                        cd = cd
                            .explanation(copy("Connection needs id, medium and diameterMm ≥ 1.", "Anschluss braucht id, medium und diameterMm ≥ 1."))
                            .remedy(Remedy::at_least(
                                SubjectRef::new(gref, format!("geometry[id={gref}].connections[id={}].diameterMm", conn.id), copy("Connection diameter", "Anschlussdurchmesser")),
                                Quantity::new(QuantityKind::Length, dia / 1000.0),
                                Quantity::new(QuantityKind::Length, 0.015),
                                copy("Set connection diameterMm to a positive millimetre value.", "connection diameterMm auf einen positiven Millimeterwert setzen."),
                            ));
                    }
                    out.push(cd.build());
                    let [px, py, pz] = conn.position;
                    for (axis, val, min_b, max_b) in [
                        ("x", px, bbox.min_x, bbox.max_x),
                        ("y", py, bbox.min_y, bbox.max_y),
                        ("z", pz, bbox.min_z, bbox.max_z),
                    ] {
                        let inside = val >= min_b && val <= max_b;
                        let span = (max_b - min_b).abs().max(1e-9);
                        let mid = (min_b + max_b) * 0.5;
                        let mut pos = CheckResult::assess(
                            format!("vdi3805.1.geometry.connection.position.{article}.{ci}.{axis}"),
                            part_label(1),
                            clause("1", "5.2"),
                            subject_product(product),
                            copy("Connection position", "Anschlussposition"),
                        )
                        .annex(ANNEX)
                        .minimum(Quantity::new(QuantityKind::Length, val), Quantity::new(QuantityKind::Length, min_b))
                        .utilization(Quantity::new(QuantityKind::Length, (val - min_b).abs()), Quantity::new(QuantityKind::Length, span))
                        .status(if inside { CheckStatus::Pass } else { CheckStatus::Fail });
                        if inside {
                            pos = pos.explanation(copy(
                                format!("Connection `{}` {axis}={val:.4} within bbox [{min_b:.4}, {max_b:.4}].", conn.id),
                                format!("Anschluss `{}` {axis}={val:.4} innerhalb BBox [{min_b:.4}, {max_b:.4}].", conn.id),
                            ));
                        } else {
                            pos = pos
                                .explanation(copy(
                                    format!("Connection `{}` {axis}={val:.4} outside bbox [{min_b:.4}, {max_b:.4}].", conn.id),
                                    format!("Anschluss `{}` {axis}={val:.4} außerhalb BBox [{min_b:.4}, {max_b:.4}].", conn.id),
                                ))
                                .remedy(Remedy::exactly(
                                    SubjectRef::new(gref, format!("geometry[id={gref}].connections[id={}].position.{axis}", conn.id), copy("Connection position", "Anschlussposition")),
                                    Quantity::new(QuantityKind::Length, val),
                                    Quantity::new(QuantityKind::Length, mid),
                                    copy("Move the connection point inside the geometry bbox.", "Anschlusspunkt in die Geometrie-BBox verschieben."),
                                ));
                        }
                        out.push(pos.build());
                    }
                    let dir_norm = (conn.direction[0].powi(2) + conn.direction[1].powi(2) + conn.direction[2].powi(2)).sqrt();
                    let dir_ok = (dir_norm - 1.0).abs() < 0.05;
                    let mut cp = CheckResult::assess(
                        format!("vdi3805.1.geometry.connection.direction.{article}.{ci}"),
                        part_label(1),
                        clause("1", "5.2"),
                        subject_product(product),
                        copy("Connection direction unit vector", "Anschlussrichtungs-Einheitsvektor"),
                    )
                    .annex(ANNEX)
                    .minimum(Quantity::new(QuantityKind::Dimensionless, dir_norm), Quantity::new(QuantityKind::Dimensionless, 0.95))
                    .status(if dir_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                    if dir_ok {
                        cp = cp.explanation(copy(format!("Connection `{}` direction ‖n‖={dir_norm:.3}.", conn.id), format!("Anschluss `{}` Richtung ‖n‖={dir_norm:.3}.", conn.id)));
                    } else {
                        cp = cp
                            .explanation(copy(format!("Connection direction norm {dir_norm:.3} must be ≈ 1.",), format!("Anschlussrichtungsbetrag {dir_norm:.3} muss ≈ 1 sein.")))
                            .remedy(Remedy::exactly(
                                SubjectRef::new(gref, format!("geometry[id={gref}].connections[id={}].direction[0]", conn.id), copy("Direction x", "Richtung x")),
                                Quantity::new(QuantityKind::Dimensionless, conn.direction[0]),
                                Quantity::new(QuantityKind::Dimensionless, conn.direction[0] / dir_norm.max(1e-12)),
                                copy("Normalize the connection direction to a unit vector.", "Anschlussrichtung auf einen Einheitsvektor normalisieren."),
                            ));
                    }
                    out.push(cp.build());
                }
            }
        }
    }
}

/// 📇 Checks the sheet, the nominal size and the tags of every index entry against its product.
pub(super) fn check_index_entries(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>) {
    // Index entry consistency with products (sheet / DN / tags) — VDI 3805-1 §4.4.
    for (i, entry) in document.index.entries.iter().enumerate() {
        let Some(product) = document.catalog.products.iter().find(|p| p.id == entry.product_id) else { continue };
        let sheet_ok = entry.sheet == product.sheet;
        let mut s = CheckResult::assess(
            format!("vdi3805.1.index.sheet.{i}"),
            part_label(1),
            clause("1", "4.4"),
            subject_dataset(),
            copy("Index sheet consistency", "Index-Blatt-Konsistenz"),
        )
        .annex(ANNEX)
        .status(if sheet_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if sheet_ok {
            s = s.explanation(copy(format!("Index sheet {} matches product.", entry.sheet.0), format!("Index-Blatt {} entspricht dem Produkt.", entry.sheet.0)));
        } else {
            s = s
                .explanation(copy(format!("Index sheet {} ≠ product sheet {}.", entry.sheet.0, product.sheet.0), format!("Index-Blatt {} ≠ Produktblatt {}.", entry.sheet.0, product.sheet.0)))
                .remedy(Remedy::exactly(
                    SubjectRef::new("", format!("index.entries[{i}].sheet"), copy("Index sheet", "Index-Blatt")),
                    Quantity::new(QuantityKind::Dimensionless, entry.sheet.0 as f64),
                    Quantity::new(QuantityKind::Dimensionless, product.sheet.0 as f64),
                    copy("Align index.entries[].sheet with the referenced product sheet.", "index.entries[].sheet an das referenzierte Produktblatt angleichen."),
                ));
        }
        out.push(s.build());

        if let Some(dn) = entry.dn {
            let product_dn = match &product.configuration.attributes {
                SheetAttributes::ValveHeating(a) => Some(a.dn),
                SheetAttributes::PumpHeating(a) => Some(a.dn_discharge),
                _ => record_kv(&product.records).get("dn").and_then(|v| v.parse().ok()),
            };
            if let Some(pdn) = product_dn {
                let dn_ok = dn == pdn;
                let mut d = CheckResult::assess(
                    format!("vdi3805.1.index.dn.{i}"),
                    part_label(1),
                    clause("1", "4.4"),
                    subject_dataset(),
                    copy("Index DN consistency", "Index-DN-Konsistenz"),
                )
                .annex(ANNEX)
                .status(if dn_ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if dn_ok {
                    d = d.explanation(copy(format!("Index DN {dn} matches product."), format!("Index-DN {dn} entspricht dem Produkt.")));
                } else {
                    d = d
                        .explanation(copy(format!("Index DN {dn} ≠ product DN {pdn}."), format!("Index-DN {dn} ≠ Produkt-DN {pdn}.")))
                        .remedy(Remedy::exactly(
                            SubjectRef::new("", format!("index.entries[{i}].dn"), copy("Index DN", "Index-DN")),
                            Quantity::new(QuantityKind::Dimensionless, dn as f64),
                            Quantity::new(QuantityKind::Dimensionless, pdn as f64),
                            copy("Align index.entries[].dn with the product DN.", "index.entries[].dn an die Produkt-DN angleichen."),
                        ));
                }
                out.push(d.build());
            }
        }

        let tag_match = |t: &str| {
            product.identity.product_group.eq_ignore_ascii_case(t)
                || product.title.iter().any(|title| title.text.to_lowercase().contains(&t.to_lowercase()))
        };
        let matched_tags = entry.tags.iter().filter(|t| tag_match(t)).count();
        let tag_total = entry.tags.len();
        let tag_ok = tag_total == 0 || matched_tags == tag_total;
        let (tag_computed, tag_limit) = if tag_total == 0 {
            (1.0, 1.0)
        } else {
            (matched_tags as f64, tag_total as f64)
        };
        let mut tg = CheckResult::assess(
            format!("vdi3805.1.index.tags.{i}"),
            part_label(1),
            clause("1", "4.4"),
            subject_dataset(),
            copy("Index tags consistency", "Index-Tag-Konsistenz"),
        )
        .annex(ANNEX)
        .minimum(
            Quantity::new(QuantityKind::Dimensionless, tag_computed),
            Quantity::new(QuantityKind::Dimensionless, tag_limit),
        )
        .status(if tag_ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if tag_ok {
            tg = tg.explanation(copy("Index tags relate to the product identity/title.", "Index-Tags beziehen sich auf Produktidentität/Titel."));
        } else {
            tg = tg
                .explanation(copy(format!("Index tags {:?} do not match product title/group.", entry.tags), format!("Index-Tags {:?} passen nicht zu Produkttitel/-gruppe.", entry.tags)))
                .remedy(Remedy::exactly(
                    SubjectRef::new("", format!("index.entries[{i}].tags[0]"), copy("Index tag", "Index-Tag")),
                    Quantity::new(QuantityKind::Dimensionless, matched_tags as f64),
                    Quantity::new(QuantityKind::Dimensionless, tag_total as f64),
                    copy("Align index.entries[].tags with the product title or product group.", "index.entries[].tags an Produkttitel oder Produktgruppe angleichen."),
                ));
        }
        out.push(tg.build());
    }
}
