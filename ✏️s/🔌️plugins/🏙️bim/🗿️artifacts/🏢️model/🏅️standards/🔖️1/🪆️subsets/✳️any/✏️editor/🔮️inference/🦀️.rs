//! 🧵️ BIM inference session: one per mounted editor instance, holding the last inference and the snapshot it was inferred from, so a render after a mutation recomputes only
//! the inference fields whose declared `reads` the diff touched (the tier-1 gate of `infer_field_after_diff`, applied to the whole `ModelInference`). The snapshot stays
//! authored-only; everything a window draws that is derived comes from here. Adding an inference field is one row of [`fields!`] below; the fidelity test fails until it is there.

use crate::standards::v1::subsets::any::schema::inferences as inf;
use crate::{ModelDiff, ModelInference, ModelSnapshot};
use protocol::{DiffAlgebra, DiffRegions, TouchedPaths};
use semio_framework_plugin::PluginCloseStep;
use std::cell::RefCell;
use std::collections::BTreeMap;

//#region 🔖️Constants
/// 🪪️ The pseudo instance of renders that carry no mounted instance (fixtures, previews).
const UNMOUNTED: u32 = u32::MAX;
//#endregion 🔖️Constants

//#region 🔖️Session
/// 🧠️ The inference state of one editor instance.
#[derive(Default)]
pub struct ModelInferenceSession {
    previous: Option<ModelSnapshot>,
    inference: ModelInference,
    recomputed: Vec<&'static str>,
}

/// 🚦️ Whether the diff touches any collection the field `id` reads.
fn touches_field(touched: &TouchedPaths, id: &str) -> bool {
    <ModelInference as protocol::InferenceSpec<ModelSnapshot>>::fields().iter().find(|spec| spec.id == id).is_none_or(|spec| touched.intersects_any(spec.reads))
}

/// 🧩️ The field table: `struct field => field id, |snapshot, inference so far| compute`. A field is recomputed when the session is empty or the diff touches its reads; a row sees the
/// fields above it already refreshed, so derived-from-derived fields (quantities) go last.
macro_rules! fields {
    ($( $field:ident => $id:literal, |$snapshot:ident, $inferred:pat_param| $compute:expr; )+) => {
        impl ModelInferenceSession {
            fn recompute(&mut self, snapshot: &ModelSnapshot, touched: Option<&TouchedPaths>) {
                self.recomputed.clear();
                $(
                    if touched.is_none_or(|touched| touches_field(touched, $id)) {
                        let value = { let $snapshot = snapshot; let $inferred = &self.inference; $compute };
                        self.inference.$field = value;
                        self.recomputed.push(stringify!($field));
                    }
                )+
            }
        }
    };
}

fields! {
    storey_levels => "s.bim.model.inference.storey-levels", |snapshot, _| inf::storey_levels::compute_storey_levels(snapshot);
    wall_layout => "s.bim.model.inference.wall-layout", |snapshot, _| inf::wall_layout::compute_wall_layout(snapshot);
    curtain_layout => "s.bim.model.inference.curtain-layout", |snapshot, _| inf::curtain_layout::compute_curtain_layout(snapshot);
    stair_runs => "s.bim.model.inference.stair-runs", |snapshot, _| inf::stair_runs::compute_stair_runs(snapshot);
    spaces => "s.bim.model.inference.spaces", |snapshot, _| inf::spaces::compute_spaces(snapshot);
    opening_frames => "s.bim.model.inference.opening-frames", |snapshot, _| inf::opening_frames::compute_opening_frames(snapshot);
    element_solids => "s.bim.model.inference.element-solids", |snapshot, _| inf::element_solids::compute_element_solids(snapshot);
    plan_linework => "s.bim.model.inference.plan-linework", |snapshot, _| inf::plan_linework::compute_plan_linework(snapshot);
    diagnostics => "s.bim.model.inference.diagnostics", |snapshot, _| inf::diagnostics::compute_diagnostics(snapshot);
    quantities => "s.bim.model.inference.quantities", |snapshot, inferred| inf::quantities::compute_quantities(snapshot, inferred);
}

impl ModelInferenceSession {
    pub fn new() -> Self {
        Self::default()
    }

    /// 🔁️ Brings the held inference up to `snapshot`: an unchanged snapshot answers from memory, a changed one recomputes exactly the fields its diff touches.
    pub fn refresh(&mut self, snapshot: &ModelSnapshot) -> &ModelInference {
        if self.previous.as_ref() == Some(snapshot) {
            return &self.inference;
        }
        let touched = self.previous.as_ref().map(|previous| <ModelDiff as DiffAlgebra<ModelSnapshot>>::between(previous, snapshot).touches());
        self.recompute(snapshot, touched.as_ref());
        self.previous = Some(snapshot.clone());
        &self.inference
    }

    /// 📊️ The fields the last refresh recomputed: the proof an edit recomputed only what it touched.
    pub fn recomputed(&self) -> &[&'static str] {
        &self.recomputed
    }
}
//#endregion 🔖️Session

//#region 🔖️Registry
thread_local! {
    static SESSIONS: RefCell<BTreeMap<u32, ModelInferenceSession>> = const { RefCell::new(BTreeMap::new()) };
}

/// 💡️ Reads the inference of `snapshot` for one mounted instance, refreshing that instance's session first.
pub fn with_inference<R>(instance: Option<u32>, snapshot: &ModelSnapshot, read: impl FnOnce(&ModelInference) -> R) -> R {
    SESSIONS.with(|sessions| {
        let mut sessions = sessions.borrow_mut();
        let session = sessions.entry(instance.unwrap_or(UNMOUNTED)).or_default();
        read(session.refresh(snapshot))
    })
}

/// 📊️ The fields the last refresh of one instance's session recomputed.
pub fn recomputed(instance: Option<u32>) -> Vec<&'static str> {
    SESSIONS.with(|sessions| sessions.borrow().get(&instance.unwrap_or(UNMOUNTED)).map(|session| session.recomputed().to_vec()).unwrap_or_default())
}

/// 🧹️ Drops the session of a closed instance.
pub fn close(instance: u32) -> PluginCloseStep {
    SESSIONS.with(|sessions| sessions.borrow_mut().remove(&instance));
    PluginCloseStep::Complete
}

/// ✅️ Whether the instance holds no session any more.
pub fn terminal_is_empty(instance: u32) -> bool {
    SESSIONS.with(|sessions| !sessions.borrow().contains_key(&instance))
}
//#endregion 🔖️Registry

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
