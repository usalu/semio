//! 🗃️ The dispatch registry: one table per catalogue category, keyed by catalogue kind id, merged into one index.
//!
//! A compute lane mounts its category module below and adds its `COMPUTES` table to [`TABLES`]; nothing else changes.

use semio_s_artifact_procedural_3d_geometry_compute::failed;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::StartFn;
use semio_s_artifact_procedural_3d_geometry_compute::WidgetJob;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;
use semio_s_artifact_procedural_3d_geometry_value::FAULT_PREFIX;
use semio_s_artifact_procedural_3d_model::catalogue::Catalogue;
use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use std::collections::BTreeMap;
use std::sync::OnceLock;
pub use semio_s_artifact_procedural_3d_geometry_compute::original::OriginalFixedOutputJob;



/// 📚️ Every category table, in registration order.
pub const TABLES: &[&[ComputeEntry]] = &[semio_s_artifact_procedural_3d_math_values::COMPUTES, semio_s_artifact_procedural_3d_brep_primitive::COMPUTES, semio_s_artifact_procedural_3d_brep_curve::COMPUTES, semio_s_artifact_procedural_3d_brep_surface::COMPUTES, semio_s_artifact_procedural_3d_brep_solid::COMPUTES, semio_s_artifact_procedural_3d_brep_boolean::COMPUTES, semio_s_artifact_procedural_3d_math_arithmetic::COMPUTES, semio_s_artifact_procedural_3d_math_vector::COMPUTES, semio_s_artifact_procedural_3d_math_list::COMPUTES, semio_s_artifact_procedural_3d_analysis_measure::COMPUTES, semio_s_artifact_procedural_3d_analysis_check::COMPUTES, semio_s_artifact_procedural_3d_brep_feature::COMPUTES, semio_s_artifact_procedural_3d_brep_transform::COMPUTES, semio_s_artifact_procedural_3d_brep_intersect::COMPUTES, semio_s_artifact_procedural_3d_brep_evaluate::COMPUTES, semio_s_artifact_procedural_3d_brep_topology::COMPUTES, semio_s_artifact_procedural_3d_mesh_primitive::COMPUTES, semio_s_artifact_procedural_3d_mesh_convert::COMPUTES, semio_s_artifact_procedural_3d_mesh_transform::COMPUTES, semio_s_artifact_procedural_3d_mesh_component::COMPUTES, semio_s_artifact_procedural_3d_mesh_edit::COMPUTES, semio_s_artifact_procedural_3d_mesh_repair::COMPUTES, semio_s_artifact_procedural_3d_mesh_inspect::COMPUTES, semio_s_artifact_procedural_3d_mesh_shading::COMPUTES, semio_s_artifact_procedural_3d_mesh_uv::COMPUTES];

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

/// 🎟️ Starts the defining fixed-output producer with actual resolved inputs and the initiating caller.

pub fn start_original_fixed_output(kind:&Kind,inputs:&mut Option<WidgetInputs>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<OriginalFixedOutputJob>,semio_framework_job::OriginalTurnFailure>{semio_s_artifact_procedural_3d_geometry_compute::original::OriginalFixedOutputJob::admit(kind,inputs,cx)}

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
