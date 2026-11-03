//! 🌳️ One value tree per document revision: the retained session, the scene fingerprints and every other reader of the same revision share a single converted `DslValue` instead of converting the document once each.

use super::*;
use std::any::{Any, TypeId};
use std::sync::{Mutex, PoisonError};

/// 🗃️ Revisions kept per document type: the committed document and the preview document of a drag alternate, so two would do; four leave room for an undo step and a peer's preview.
const REVISIONS_PER_TYPE: usize = 4;

struct Slot {
    kind: TypeId,
    document: Box<dyn Any + Send>,
    tree: Arc<DslValue>,
}

static REVISIONS: Mutex<Vec<Slot>> = Mutex::new(Vec::new());

/// 🌳️ The value tree of exactly `document`: served from the most recent revisions of its type by equality (a compare is far cheaper than a conversion), converted once otherwise. The conversion runs outside the lock and the least recent revision of the type is dropped, so memory stays bounded.
pub fn shared_value_tree<D: Clone + PartialEq + ToValue + Send + 'static>(document: &D) -> Arc<DslValue> {
    let kind = TypeId::of::<D>();
    {
        let mut slots = REVISIONS.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(at) = slots.iter().position(|slot| slot.kind == kind && slot.document.downcast_ref::<D>() == Some(document)) {
            let slot = slots.remove(at);
            let tree = slot.tree.clone();
            slots.insert(0, slot);
            return tree;
        }
    }
    let tree = Arc::new(document.to_value());
    let mut slots = REVISIONS.lock().unwrap_or_else(PoisonError::into_inner);
    slots.insert(0, Slot { kind, document: Box::new(document.clone()), tree: tree.clone() });
    let mut kept = 0;
    slots.retain(|slot| {
        kept += usize::from(slot.kind == kind);
        slot.kind != kind || kept <= REVISIONS_PER_TYPE
    });
    tree
}
