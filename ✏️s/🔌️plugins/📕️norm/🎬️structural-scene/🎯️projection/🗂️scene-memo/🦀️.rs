//! 🗂️ Scene memo: the retained whole-scene cache of an artifact whose projection is one pure function of `(document, report, combination)`. Repeated paints of the same revision (viewport, viewer, diagrams) project once; a changed revision projects again and records the minimal [`SceneDelta`] against the previous scene for the viewport.

use super::*;
use norm::CheckReport;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

struct Slot<D> {
    document: D,
    report: CheckReport,
    combination: Option<String>,
    scene: Arc<StructuralScene>,
}

/// 🗂️ One retained scene with the delta that produced it from its predecessor.
pub struct SceneMemo<D> {
    slot: Option<Slot<D>>,
    delta: Option<SceneDelta>,
    projections: usize,
}

impl<D> Default for SceneMemo<D> {
    fn default() -> Self {
        Self { slot: None, delta: None, projections: 0 }
    }
}

impl<D: Clone + PartialEq> SceneMemo<D> {
    /// 🎬️ The scene of exactly this revision: served from the memo when document, report and combination are unchanged, otherwise `project` runs once and the delta against the previous scene is kept.
    pub fn scene(&mut self, document: &D, report: &CheckReport, combination: Option<&str>, project: impl FnOnce() -> StructuralScene) -> Arc<StructuralScene> {
        if let Some(slot) = &self.slot {
            if slot.document == *document && slot.report == *report && slot.combination.as_deref() == combination {
                return slot.scene.clone();
            }
        }
        let scene = Arc::new(project());
        self.projections += 1;
        self.delta = self.slot.as_ref().map(|slot| diff(&slot.scene, &scene));
        self.slot = Some(Slot { document: document.clone(), report: report.clone(), combination: combination.map(str::to_owned), scene: scene.clone() });
        scene
    }

    /// 🩹️ The delta from the previous scene to the current one (`None` before the second projection).
    pub fn delta(&self) -> Option<&SceneDelta> {
        self.delta.as_ref()
    }

    /// 🔢️ How many times `project` really ran.
    pub fn projections(&self) -> usize {
        self.projections
    }
}

struct DiagramSlot<D> {
    document: D,
    report: CheckReport,
    diagrams: Arc<Vec<Diagram2d>>,
}

const DIAGRAM_SLOTS: usize = 32;

/// 📈️ The diagrams of exactly this `(document, report, key)` of one artifact `variant`: served from the retained slot while document and report are unchanged (a repaint, the focus fallback, a second window), otherwise `compute` runs once and the slot is replaced. `key` separates the inputs that are not part of the document (the focused entity, the combination); at most [`DIAGRAM_SLOTS`] slots are kept.
pub fn memo_diagrams<D: Clone + PartialEq + Send + 'static>(variant: &'static str, document: &D, report: &CheckReport, key: &str, compute: impl FnOnce() -> Vec<Diagram2d>) -> Arc<Vec<Diagram2d>> {
    static SLOTS: OnceLock<Mutex<HashMap<(&'static str, String), Box<dyn Any + Send>>>> = OnceLock::new();
    let slots = SLOTS.get_or_init(|| Mutex::new(HashMap::new()));
    let id = (variant, key.to_owned());
    let hit = slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&id).and_then(|slot| slot.downcast_ref::<DiagramSlot<D>>()).filter(|slot| slot.document == *document && slot.report == *report).map(|slot| slot.diagrams.clone());
    if let Some(diagrams) = hit {
        return diagrams;
    }
    let diagrams = Arc::new(compute());
    let mut slots = slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if slots.len() >= DIAGRAM_SLOTS && !slots.contains_key(&id) {
        slots.clear();
    }
    slots.insert(id, Box::new(DiagramSlot { document: document.clone(), report: report.clone(), diagrams: diagrams.clone() }));
    diagrams
}

/// 🗂️ Declares `mod $module` with the retained [`SceneMemo`] of one artifact: `scene(document, report, combination, project)` and `delta()`; editor and viewer share it so one revision projects once.
#[macro_export]
macro_rules! live_whole_scene {
    ($vis:vis mod $module:ident($document:ty)) => {
        $vis mod $module {
            use super::*;

            fn slot() -> &'static ::std::sync::Mutex<$crate::projection::SceneMemo<$document>> {
                static SLOT: ::std::sync::OnceLock<::std::sync::Mutex<$crate::projection::SceneMemo<$document>>> = ::std::sync::OnceLock::new();
                SLOT.get_or_init(|| ::std::sync::Mutex::new($crate::projection::SceneMemo::default()))
            }

            pub fn scene(document: &$document, report: &$crate::norm::CheckReport, combination: ::std::option::Option<&str>, project: impl ::std::ops::FnOnce() -> $crate::StructuralScene) -> ::std::sync::Arc<$crate::StructuralScene> {
                slot().lock().unwrap_or_else(::std::sync::PoisonError::into_inner).scene(document, report, combination, project)
            }

            pub fn delta() -> ::std::option::Option<$crate::SceneDelta> {
                slot().lock().unwrap_or_else(::std::sync::PoisonError::into_inner).delta().cloned()
            }
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
