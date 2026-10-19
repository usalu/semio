//! 📏️ Limit and extension checks: the catalog limits and the extension field bags of the catalog integrity check. See [`super`].

use super::*;

/// 📏 Checks the record count, the file bytes, the field length and the nesting depth against the declared limits.
pub(super) fn check_limits(
    out: &mut Vec<CheckResult>,
    lim: &semio_s_artifact_norm_vdi3805_model::SecurityLimits,
    actual_records: usize,
    actual_field_len: usize,
    actual_bytes: usize,
    actual_depth: usize,
) {
    for (leaf, actual, limit_v, en, de) in [
        ("limits.maxRecords", actual_records as f64, lim.max_records as f64, "Max records", "Max. Datensätze"),
        ("limits.maxFileBytes", actual_bytes as f64, lim.max_file_bytes as f64, "Max file bytes", "Max. Dateibytes"),
        ("limits.maxFieldLength", actual_field_len as f64, lim.max_field_length as f64, "Max field length", "Max. Feldlänge"),
        ("limits.maxNestingDepth", actual_depth as f64, lim.max_nesting_depth as f64, "Max nesting depth", "Max. Verschachtelungstiefe"),
    ] {
        let ok = actual <= limit_v && limit_v >= 1.0;
        let mut c = CheckResult::assess(
            format!("vdi3805.1.{leaf}"),
            part_label(1),
            clause("1", "4.1"),
            subject_dataset(),
            copy(en, de),
        )
        .annex(ANNEX)
        .minimum(Quantity::new(QuantityKind::Dimensionless, limit_v - actual), Quantity::new(QuantityKind::Dimensionless, 0.0))
        .utilization(Quantity::new(QuantityKind::Dimensionless, actual), Quantity::new(QuantityKind::Dimensionless, limit_v.max(1.0)))
        .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
        if !ok {
            c = c.remedy(Remedy::at_least(
                SubjectRef::new("", leaf, copy(en, de)),
                Quantity::new(QuantityKind::Dimensionless, limit_v),
                Quantity::new(QuantityKind::Dimensionless, actual.max(1.0)),
                copy(format!("Raise {leaf} so the catalogue fits the security envelope."), format!("{leaf} anheben, damit der Katalog in die Sicherheitsgrenzen passt.")),
            ));
        } else {
            c = c.explanation(copy(
                format!("{leaf}: actual {actual} ≤ limit {limit_v}."),
                format!("{leaf}: Ist {actual} ≤ Grenze {limit_v}."),
            ));
        }
        out.push(c.build());
    }
}

/// 🧩 Checks the extension field bags of the file, the catalog and the products.
pub(super) fn check_extension_fields(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>) {
    {
        let mut bags: Vec<(String, &std::collections::BTreeMap<String, String>)> = vec![
            ("catalog.file.extensions.fields".into(), &document.catalog.file.extensions.fields),
            ("catalog.extensions.fields".into(), &document.catalog.extensions.fields),
        ];
        for product in &document.catalog.products {
            bags.push((format!("catalog.products[id={}].extensions.fields", product.id), &product.extensions.fields));
            for (ri, record) in product.records.iter().enumerate() {
                bags.push((format!("catalog.products[id={}].records[{ri}].extensions.fields", product.id), &record.extensions.fields));
            }
        }
        for (base, fields) in bags {
            for (key, value) in fields.iter() {
                let ok = !key.is_empty() && !value.is_empty();
                let path = format!("{base}[{key}]");
                let mut ec = CheckResult::assess(
                    format!("vdi3805.1.extensions.{}", path.replace('.', "-").replace('[', "-").replace(']', "")),
                    part_label(1),
                    clause("1", "4.1"),
                    subject_dataset(),
                    copy("Extension field", "Erweiterungsfeld"),
                )
                .annex(ANNEX)
                .minimum(Quantity::new(QuantityKind::Dimensionless, if ok { 1.0 } else { 0.0 }), Quantity::new(QuantityKind::Dimensionless, 1.0))
                .status(if ok { CheckStatus::Pass } else { CheckStatus::Fail });
                if ok {
                    ec = ec.explanation(copy(format!("Extension `{key}` present."), format!("Erweiterung `{key}` vorhanden.")));
                } else {
                    ec = ec
                        .explanation(copy(format!("Extension `{key}` must have a non-empty value."), format!("Erweiterung `{key}` braucht einen nicht-leeren Wert.")))
                        .remedy(Remedy::exactly(
                            SubjectRef::new("", path, copy("Extension field", "Erweiterungsfeld")),
                            Quantity::new(QuantityKind::Dimensionless, 0.0),
                            Quantity::new(QuantityKind::Dimensionless, 1.0),
                            copy("Fill the extension field with a non-empty value.", "Erweiterungsfeld mit nicht-leerem Wert füllen."),
                        ));
                }
                out.push(ec.build());
            }
        }
    }
}
