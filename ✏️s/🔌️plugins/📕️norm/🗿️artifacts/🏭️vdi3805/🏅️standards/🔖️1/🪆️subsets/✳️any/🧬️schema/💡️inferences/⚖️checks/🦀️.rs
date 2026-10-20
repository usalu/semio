//! ⚖️ VDI 3805 compliance checks: the clause-by-clause verification behind the compliance report.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_norm_vdi3805_model::ManufacturerCatalog;

//#region 🔖️ComplianceHelpers
use semio_s_plugin_norm_compliance::{AnnexChoice, ClauseId, LocalizedCopy, SubjectRef};
#[cfg(test)] use semio_s_plugin_norm_compliance::NormError;
/// 📐️ Pure VDI 3805 compliance helpers — native-text parsing with typed record families,
/// Part 1 structural validation, and sheet attribute catalogues.
use semio_s_artifact_norm_vdi3805_model::*;
use std::collections::BTreeSet;

const FAMILY: &str = "VDI 3805";
pub const ANNEX: AnnexChoice = AnnexChoice::De;

pub fn clause(part: &str, section: &str) -> ClauseId {
    ClauseId::new(FAMILY, part, section)
}

pub fn subject_product(product: &CatalogueProduct) -> SubjectRef {
    SubjectRef::new(
        product.id.clone(),
        format!("catalog.products[id={}]", product.id),
        LocalizedCopy::new(format!("Product {}", product.identity.article_number), format!("Produkt {}", product.identity.article_number)),
    )
}

pub fn subject_dataset() -> SubjectRef {
    SubjectRef::whole(LocalizedCopy::new("Manufacturer dataset", "Herstellerdatensatz"))
}

/// 🎛 Nominal diameters admitted by VDI 3805 Blatt 2 / ISO 6708 series.
pub const VALVE_DN_SERIES: &[u16] = &[10, 15, 20, 25, 32, 40, 50, 65, 80, 100, 125, 150, 200, 250, 300];

/// 📊️ Minimum kvs [m³/h] per DN for Blatt 2 control valves (catalogue floor).
pub fn valve_min_kvs_m3_h(dn: u16) -> f64 {
    match dn {
        10 => 0.16,
        15 => 0.25,
        20 => 0.40,
        25 => 0.63,
        32 => 1.00,
        40 => 1.60,
        50 => 2.50,
        65 => 4.00,
        80 => 6.30,
        100 => 10.0,
        125 => 16.0,
        150 => 25.0,
        200 => 40.0,
        250 => 63.0,
        300 => 100.0,
        _ => 0.63,
    }
}

/// 📐️ Minimum radiator heat exponent n (EN 442 / Blatt 3).
pub const RADIATOR_N_MIN: f64 = 1.1;
/// 📐️ Maximum radiator heat exponent n (EN 442 / Blatt 3).
pub const RADIATOR_N_MAX: f64 = 1.5;

/// 📔️ Minimum field arity (including family code) per record family.
pub fn record_min_fields(family: &str) -> Option<usize> {
    match family {
        "010" => Some(6),
        "100" => Some(5),
        "110" => Some(3),
        "200" => Some(5),
        "400" => Some(4),
        "700" => Some(3),
        "900" => Some(2),
        _ => None,
    }
}

fn count_native_records(catalog: &ManufacturerCatalog) -> u32 {
    catalog.products.iter().map(|p| p.records.len() as u32).sum()
}

/// ✅️ Part 1 structural validation over the full manufacturer dataset.
pub fn validate_structure(document: &Vdi3805Snapshot) -> Vec<Diagnostic> {
    let catalog = &document.catalog;
    let mut issues = Vec::new();
    if catalog.file.manufacturer.is_empty() {
        issues.push(Diagnostic::error("catalog.file.manufacturer", "missing manufacturer code"));
    }
    if catalog.file.charset.is_empty() {
        issues.push(Diagnostic::error("catalog.file.charset", "missing charset"));
    }
    if catalog.products.is_empty() {
        issues.push(Diagnostic::error("catalog.products", "empty product list"));
    }
    let actual = count_native_records(catalog);
    if catalog.file.record_count != actual {
        issues.push(Diagnostic::error("catalog.file.recordCount", format!("record_count {} != actual {}", catalog.file.record_count, actual)));
    }
    let product_ids: BTreeSet<String> = catalog.products.iter().map(|p| p.id.clone()).collect();
    for (pi, product) in catalog.products.iter().enumerate() {
        let base = format!("catalog.products[{pi}]");
        if product.identity.article_number.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.articleNumber"), "missing article number"));
        }
        if product.identity.manufacturer_code.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.manufacturerCode"), "missing manufacturer code"));
        }
        if product.identity.product_group.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.identity.productGroup"), "missing product group"));
        }
        if product.configuration.id.is_empty() {
            issues.push(Diagnostic::error(format!("{base}.configuration.id"), "missing configuration id"));
        }
        let has_100 = product.records.iter().any(|r| r.family.0 == "100");
        if !has_100 {
            issues.push(Diagnostic::error(format!("{base}.records"), "missing mandatory record 100"));
        }
        for (ri, record) in product.records.iter().enumerate() {
            if let Some(min) = record_min_fields(record.family.0.as_str()) {
                if record.fields.len() < min {
                    issues.push(Diagnostic::error(format!("{base}.records[{ri}]"), format!("record {} needs ≥{min} fields, has {}", record.family.0, record.fields.len())));
                }
            } else {
                let known: BTreeSet<&str> = RecordFamilyId::all_known().iter().copied().collect();
                if !known.contains(record.family.0.as_str()) && !record.family.0.starts_with('9') {
                    issues.push(Diagnostic::error(format!("{base}.records[{ri}].family"), format!("unknown record family {}", record.family.0)));
                }
            }
        }
        if let Some(geom_ref) = &product.configuration.geometry_ref {
            if !document.geometry.contains_key(geom_ref) {
                issues.push(Diagnostic::error(format!("{base}.configuration.geometryRef"), format!("dangling geometry ref {geom_ref}")));
            }
            let has_200 = product.records.iter().any(|r| r.family.0 == "200");
            if !has_200 {
                issues.push(Diagnostic::error(format!("{base}.records"), "geometry_ref set but record 200 missing"));
            }
        }
        for (fi, fun_ref) in product.configuration.function_refs.iter().enumerate() {
            if !document.curves.contains_key(fun_ref) {
                issues.push(Diagnostic::error(format!("{base}.configuration.functionRefs[{fi}]"), format!("dangling curve ref {fun_ref}")));
            }
        }
        for (ai, link) in product.accessories.iter().enumerate() {
            if !product_ids.contains(&link.accessory_id) {
                issues.push(Diagnostic::error(format!("{base}.accessories[{ai}].accessoryId"), format!("dangling accessory {}", link.accessory_id)));
            }
        }
        for (ci, link) in product.components.iter().enumerate() {
            if !product_ids.contains(&link.component_id) {
                issues.push(Diagnostic::error(format!("{base}.components[{ci}].componentId"), format!("dangling component {}", link.component_id)));
            }
        }
        let title_locales: BTreeSet<&str> = product.title.iter().map(|t| t.locale.as_str()).collect();
        if !title_locales.contains("de") || !title_locales.contains("en") {
            issues.push(Diagnostic::error(format!("{base}.title"), "multilingual texts require de and en"));
        }
    }
    issues
}

/// 🔢️ Linear map between two scalar domains.
pub fn linear_map(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    if (x1 - x0).abs() < f64::EPSILON {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}

//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "../../🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
