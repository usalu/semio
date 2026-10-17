//! 📚️ VDI 3805 document checks: catalog integrity, historical sheets, edition profiles, operative sheet coverage and `evaluate`, which assembles the compliance report.

use super::*;

//#region 🔖️ComplianceReport

#[path = "🗃️catalog-integrity/🦀️.rs"]
mod catalog_integrity;
use catalog_integrity::*;

#[path = "🗂️file-header-and-historical-checks/🦀️.rs"]
mod file_header_and_historical_checks;
use file_header_and_historical_checks::*;

/// 📋️ VDI 3805 Part 1 + claimed Blatt conformance report (no registry/IO plumbing noise).
pub fn evaluate(document: &Vdi3805Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    report.extend(check_part1(document));
    report.extend(check_catalog_integrity(document));
    for product in &document.catalog.products {
        report.extend(check_sheet_product(document, product));
    }
    report.extend(check_historical(document));
    report.extend(check_edition_profiles(document));
    report.extend(check_operative_sheet_coverage(document));
    if document.catalog.products.is_empty() {
        report.push(
            CheckResult::assess("vdi3805.1.empty", part_label(1), clause("1", "4.1"), subject_dataset(), copy("Catalogue products", "Katalogprodukte"))
                .annex(ANNEX)
                .not_applicable(copy("No products to assess against product sheets.", "Keine Produkte für Blattprüfungen vorhanden."))
                .build(),
        );
    }
    report
}
//#endregion 🔖️ComplianceReport
