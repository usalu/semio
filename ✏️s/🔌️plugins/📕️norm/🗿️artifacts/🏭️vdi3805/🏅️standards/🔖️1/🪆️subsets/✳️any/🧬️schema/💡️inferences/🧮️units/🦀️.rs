//! 🧮️ VDI 3805 check units in exactly the order of `evaluate`: the head (Part 1 structure and catalogue integrity), one unit per catalogue product, then the tail (historical sheets, edition profiles, operative coverage, empty catalogue).

use super::*;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, UnitSource};

/// 🧮️ The unit source of a VDI 3805 manufacturer dataset: `SharedEvaluation<Vdi3805Units>` is the one evaluation path of editor and viewer.
pub struct Vdi3805Units;

/// 📖️ What a product row unit reads besides its own row: the edition profile of its sheet and the characteristic curves (their keys feed the remedies).
const SHARED: [&str; 2] = ["editionProfile", "curves"];
/// 📖️ What the head unit (Part 1 structure, catalogue integrity, index, geometry and curve keys, correction date, strict flag, limits) reads: everything but the edition profile.
const HEAD: [&str; 7] = ["catalog", "correctionAsOf", "strictMode", "geometry", "curves", "limits", "index"];
/// 📖️ What the tail unit (historical sheets, edition profiles, operative coverage, empty catalogue) reads: the profile map, the flags, the file header and the identity, sheet, records and configuration of the products; not the geometry, curves, limits, index, titles or links.
const TAIL: [&str; 9] = ["editionProfile", "strictMode", "correctionAsOf", "catalog.file", "catalog.products.*.id", "catalog.products.*.sheet", "catalog.products.*.identity", "catalog.products.*.records", "catalog.products.*.configuration"];
const PRODUCTS: &str = "catalog.products";

fn run(scope: Scope<'static>) -> impl Fn(&Vdi3805Snapshot) -> Vec<CheckResult> + Send + Sync + 'static {
    move |document| evaluate_scoped(document, scope).checks
}

impl UnitSource for Vdi3805Units {
    type Document = Vdi3805Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners { collections: &[], document_level: &["vdi3805.1.", "vdi3805.coverage."], filed: false };

    fn units(document: &Vdi3805Snapshot) -> Vec<CheckUnit<Vdi3805Snapshot>> {
        let mut rows = Rows::new(SHARED).global_reading(&HEAD, run(Scope::Part("head")));
        for (index, product) in document.catalog.products.iter().enumerate() {
            rows = rows.row("products", PRODUCTS, index, &product.id, CostTier::T0, &[], move |d: &Vdi3805Snapshot| evaluate_scoped(d, Scope::Row(PRODUCTS, index)).checks);
        }
        rows.part_reading("tail", &TAIL, run(Scope::Part("tail"))).finish()
    }
}
