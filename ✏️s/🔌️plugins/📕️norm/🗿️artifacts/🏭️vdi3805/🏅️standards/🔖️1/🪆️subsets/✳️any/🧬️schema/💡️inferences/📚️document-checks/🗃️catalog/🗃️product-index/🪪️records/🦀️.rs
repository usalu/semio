//! 🪪️ Product identity and record checks: the identity, the configuration id, the part 7 record and the native part 1 records of one catalogue product. See [`super`].

use super::*;

/// 🪪 Checks the product id against the identity record 100 fields.
pub(super) fn check_product_identity(out: &mut Vec<CheckResult>, product: &semio_s_artifact_norm_vdi3805_model::CatalogueProduct, article: &str) -> bool {
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
    id_ok
}

/// ⚙ Checks that the configuration id is cfg dot the article number.
pub(super) fn check_product_configuration(out: &mut Vec<CheckResult>, product: &semio_s_artifact_norm_vdi3805_model::CatalogueProduct, article: &str) {
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
}

/// 🧾 Checks the part 7 record of the product.
pub(super) fn check_product_record_700(out: &mut Vec<CheckResult>, product: &semio_s_artifact_norm_vdi3805_model::CatalogueProduct, article: &str) {
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
}

/// 📚 Checks that every native record echoes its family code and the identity fields.
pub(super) fn check_native_records(out: &mut Vec<CheckResult>, product: &semio_s_artifact_norm_vdi3805_model::CatalogueProduct, article: &str) {
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
}
