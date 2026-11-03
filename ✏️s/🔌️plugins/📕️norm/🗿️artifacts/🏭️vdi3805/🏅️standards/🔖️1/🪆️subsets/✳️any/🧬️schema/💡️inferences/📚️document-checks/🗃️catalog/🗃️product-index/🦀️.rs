//! 🗃️ Product and index checks: the product ids, the dangling index entries, the per product geometry and the index to product consistency of the catalog integrity check. See [`super`].

use super::*;

#[path = "🪪️records/🦀️.rs"]
mod records;
use records::{check_product_identity, check_product_configuration, check_product_record_700, check_native_records};

#[path = "🔗️links/🦀️.rs"]
mod links;
use links::{check_dangling_links, check_link_quantities, check_product_geometry};

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
        let id_ok = check_product_identity(out, product, article);
        check_product_configuration(out, product, article);
        check_product_record_700(out, product, article);
        check_native_records(out, product, article);
        check_dangling_links(out, product_ids, product, article);
        check_link_quantities(out, product_ids, product, article);
        check_product_geometry(document, out, product, article, id_ok);
    }
}

/// 📇 Checks the sheet, the nominal size and the tags of every index entry against its product.
pub(super) fn check_index_entries(document: &Vdi3805Snapshot, out: &mut Vec<CheckResult>) {
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
