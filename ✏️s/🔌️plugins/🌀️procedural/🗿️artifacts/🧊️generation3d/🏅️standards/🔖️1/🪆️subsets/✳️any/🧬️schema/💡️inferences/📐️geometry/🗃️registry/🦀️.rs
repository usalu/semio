//! 🗃️ The dispatch registry: one table per catalogue category, keyed by catalogue kind id, merged into one index.
//!
//! A compute lane mounts its category module below and adds its `COMPUTES` table to [`TABLES`]; nothing else changes.

use super::compute::{failed, ComputeEntry, StartFn, WidgetJob};
use super::inputs::WidgetInputs;
use super::value::{WidgetFault, FAULT_PREFIX};
use crate::standards::v1::subsets::any::schema::catalogue::{Catalogue, Kind};
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[path = "../🔢️math-values/🦀️.rs"]
mod math_values;
#[path = "../🧊️brep-primitive/🦀️.rs"]
mod brep_primitive;
#[path = "../〰️brep-curve/🦀️.rs"]
mod brep_curve;
#[path = "../🏳️brep-surface/🦀️.rs"]
mod brep_surface;
#[path = "../🏗️brep-solid/🦀️.rs"]
mod brep_solid;
#[path = "../🔗️brep-boolean/🦀️.rs"]
mod brep_boolean;
#[path = "../🧮️math-arithmetic/🦀️.rs"]
mod math_arithmetic;
#[path = "../➡️math-vector/🦀️.rs"]
mod math_vector;
#[path = "../📚️math-list/🦀️.rs"]
mod math_list;
#[path = "../📏️analysis-measure/🦀️.rs"]
mod analysis_measure;
#[path = "../✅️analysis-check/🦀️.rs"]
mod analysis_check;

#[path = "../⏱️phased-job/🦀️.rs"]
pub(crate) mod phased_job;
#[path = "../🧵️brep-sources/🦀️.rs"]
mod brep_sources;
#[path = "../🛠️brep-feature/🦀️.rs"]
mod brep_feature;
#[path = "../🔁️brep-transform/🦀️.rs"]
mod brep_transform;
#[path = "../✂️brep-intersect/🦀️.rs"]
mod brep_intersect;
#[path = "../🎯️brep-evaluate/🦀️.rs"]
mod brep_evaluate;
#[path = "../🐚️brep-topology/🦀️.rs"]
mod brep_topology;
#[path = "../🧱️mesh-support/🦀️.rs"]
pub(crate) mod mesh_support;
#[path = "../🥽️mesh-primitive/🦀️.rs"]
mod mesh_primitive;
#[path = "../🔀️mesh-convert/🦀️.rs"]
mod mesh_convert;
#[path = "../↔️mesh-transform/🦀️.rs"]
mod mesh_transform;
#[path = "../🎚️mesh-component/🦀️.rs"]
mod mesh_component;
#[path = "../✏️mesh-edit/🦀️.rs"]
mod mesh_edit;
#[path = "../🩹️mesh-repair/🦀️.rs"]
mod mesh_repair;
#[path = "../🔎️mesh-inspect/🦀️.rs"]
mod mesh_inspect;
#[path = "../🌗️mesh-shading/🦀️.rs"]
mod mesh_shading;
#[path = "../🗺️mesh-uv/🦀️.rs"]
mod mesh_uv;

/// 📚️ Every category table, in registration order.
pub const TABLES: &[&[ComputeEntry]] = &[math_values::COMPUTES, brep_primitive::COMPUTES, brep_curve::COMPUTES, brep_surface::COMPUTES, brep_solid::COMPUTES, brep_boolean::COMPUTES, math_arithmetic::COMPUTES, math_vector::COMPUTES, math_list::COMPUTES, analysis_measure::COMPUTES, analysis_check::COMPUTES, brep_feature::COMPUTES, brep_transform::COMPUTES, brep_intersect::COMPUTES, brep_evaluate::COMPUTES, brep_topology::COMPUTES, mesh_primitive::COMPUTES, mesh_convert::COMPUTES, mesh_transform::COMPUTES, mesh_component::COMPUTES, mesh_edit::COMPUTES, mesh_repair::COMPUTES, mesh_inspect::COMPUTES, mesh_shading::COMPUTES, mesh_uv::COMPUTES];

/// 🗺️ The tables merged by kind id; the first registration of an id wins and a test refuses duplicates.
pub fn index() -> &'static BTreeMap<&'static str, StartFn> {
    static INDEX: OnceLock<BTreeMap<&'static str, StartFn>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut index = BTreeMap::new();
        for entry in TABLES.iter().flat_map(|table| table.iter()) {
            index.entry(entry.id).or_insert(entry.start);
        }
        index
    })
}

/// 🔎️ The compute that starts widgets of a kind.
pub fn lookup(kind_id: &str) -> Option<StartFn> {
    #[cfg(test)]
    if let Some(start) = probe::overridden(kind_id) {
        return Some(start);
    }
    index().get(kind_id).copied()
}

/// 🚀️ Starts the job of a widget; a kind without a compute answers with a localized `compute-missing` fault, never a placeholder result.
pub fn start(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    match lookup(&kind.id) {
        Some(start) => start(kind, inputs),
        None => failed(
            WidgetFault::new(format!("{FAULT_PREFIX}compute-missing"), format!("The widget kind \u{201c}{}\u{201d} cannot be computed yet.", kind.label.en), format!("Der Widget-Typ \u{201c}{}\u{201d} kann noch nicht berechnet werden.", kind.label.de)),
            kind.quality,
        ),
    }
}

/// 🗃️ The registered kind ids.
pub fn registered() -> Vec<&'static str> {
    index().keys().copied().collect()
}

/// 🕳️ The catalogue kinds without a compute.
pub fn missing_kinds(catalogue: &Catalogue) -> Vec<&str> {
    catalogue.kinds().map(|kind| kind.id.as_str()).filter(|id| !index().contains_key(id)).collect()
}

/// 🧪️ Test-only per-thread override of a kind's compute, so stepped and misbehaving jobs run through the real engine.
#[cfg(test)]
pub(crate) mod probe {
    use super::StartFn;
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    thread_local! {
        static OVERRIDES: RefCell<BTreeMap<String, StartFn>> = const { RefCell::new(BTreeMap::new()) };
    }

    pub(crate) fn overridden(kind_id: &str) -> Option<StartFn> {
        OVERRIDES.with(|overrides| overrides.borrow().get(kind_id).copied())
    }

    /// 🔧️ Restores the real compute of the kind when dropped.
    pub(crate) struct Override(String);

    impl Drop for Override {
        fn drop(&mut self) {
            OVERRIDES.with(|overrides| overrides.borrow_mut().remove(&self.0));
        }
    }

    pub(crate) fn install(kind_id: &str, start: StartFn) -> Override {
        OVERRIDES.with(|overrides| overrides.borrow_mut().insert(kind_id.to_string(), start));
        Override(kind_id.to_string())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
