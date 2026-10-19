//! 🗃️ VDI 3805 catalog integrity: limits, correction date, product, index and geometry checks of a snapshot.

use super::*;

#[path = "📏️limit-and-extension-checks/🦀️.rs"]
mod limit_and_extension_checks;
use limit_and_extension_checks::{check_limits, check_extension_fields};

#[path = "🗓️correction-date-and-strict-mode-checks/🦀️.rs"]
mod correction_date_and_strict_mode_checks;
use correction_date_and_strict_mode_checks::{check_correction_date, check_strict_mode};

#[path = "🗃️product-and-index-checks/🦀️.rs"]
mod product_and_index_checks;
use product_and_index_checks::{check_unique_product_ids, check_dangling_index_entries, check_products, check_index_entries};

pub(super) fn check_catalog_integrity(document: &Vdi3805Snapshot) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let product_ids: BTreeSet<_> = document.catalog.products.iter().map(|p| p.id.clone()).collect();
    let lim = &document.limits;
    let actual_records: usize = document.catalog.products.iter().map(|p| p.records.len()).sum();
    let actual_field_len: usize = document
        .catalog
        .products
        .iter()
        .flat_map(|p| p.records.iter())
        .flat_map(|r| r.fields.iter())
        .map(|f| f.len())
        .max()
        .unwrap_or(0);
    let actual_bytes: usize = document
        .catalog
        .products
        .iter()
        .flat_map(|p| p.records.iter())
        .flat_map(|r| r.fields.iter())
        .map(|f| f.len())
        .sum::<usize>()
        .saturating_add(document.catalog.file.manufacturer.len())
        .saturating_add(document.catalog.file.created.len());
    let mut actual_depth: usize = 1;
    if !document.catalog.products.is_empty() {
        actual_depth = 3;
    }
    if document.catalog.products.iter().any(|p| !p.records.is_empty()) {
        actual_depth = 4;
    }
    if document.catalog.products.iter().any(|p| p.records.iter().any(|r| !r.fields.is_empty())) {
        actual_depth = 5;
    }
    if !document.geometry.is_empty() || !document.curves.is_empty() {
        actual_depth = actual_depth.max(2);
    }
    check_limits(&mut out, lim, actual_records, actual_field_len, actual_bytes, actual_depth);
    check_extension_fields(document, &mut out);
    check_correction_date(document, &mut out);
    check_strict_mode(document, &mut out);
    check_unique_product_ids(document, &mut out, &product_ids);
    check_dangling_index_entries(document, &mut out, &product_ids);
    check_products(document, &mut out, &product_ids);
    check_index_entries(document, &mut out);
    check_catalog_file_header(document, &mut out);
    out
}
