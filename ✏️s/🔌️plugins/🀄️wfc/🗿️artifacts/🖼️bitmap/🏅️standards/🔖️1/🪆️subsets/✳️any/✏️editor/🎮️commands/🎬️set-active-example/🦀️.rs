//! 🎬️ `setActiveExample` — loads one of this subset's committed examples into the open document.
//! The shell's navbar example picker and its automatic boot announcement both dispatch this verb, so
//! without it every bitmap playground boot dropped the action on the undeclared-action gate and the
//! picker was inert.
//!
//! An example switch is a whole-document LOAD (`Effect::LoadDocument`), never a mutation set: it carries
//! no diff and no history row, and a document that already IS the requested example answers nothing at
//! all, so the boot announcement and a re-selection write no edit (`canUndo=false`).

use crate::schema::snapshot::{BitmapSnapshot};
use crate::BitmapMutation;
use semio_framework_plugin::{ArtifactView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}
//#endregion 🔖️Payload

//#region 🔖️Registry
/// 🚪️ The example the editor boots on — the id an empty `exampleId` argument means.
pub const BITMAP_EXAMPLE_BOOT_ID: &str = crate::examples::rooms_16::ID;

/// 📚️ The committed snapshot behind one example id. An unknown id answers `None`, which the handler
/// turns into a no-op rather than a fault: the picker is a navigation affordance, not a destructive
/// verb.
pub fn example_snapshot(example_id: &str) -> Option<BitmapSnapshot> {
    match example_id {
        crate::examples::rooms_16::ID => Some(crate::examples::rooms_16::snapshot()),
        crate::examples::flowers_24::ID => Some(crate::examples::flowers_24::snapshot()),
        _ => None,
    }
}
//#endregion 🔖️Registry

//#region 🔖️Handler
/// 🗃️ The whole-document load an example switch answers with: a freshly packed snapshot and an edit-free op log. It is NOT an
/// edit, so no mutation row, no diff and no history row exists for it.
pub fn load_document_effect(document: &BitmapSnapshot) -> semio_framework::kernel::Effect {
    let pack = <BitmapSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("wfc-bitmap", crate::WFC_BITMAP_DOCUMENT_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}

/// 🎬️ Loads the named example as the open document; an unknown id or the already-open example answers nothing.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, BitmapSnapshot>) -> Result<Emit<BitmapMutation>, Fault> {
    let example_id = match payload.example_id.trim() {
        "" => BITMAP_EXAMPLE_BOOT_ID,
        id => id,
    };
    match example_snapshot(example_id) {
        Some(next) if &next != doc.snapshot => Ok(Emit::effect(load_document_effect(&next))),
        _ => Ok(Emit::default()),
    }
}
//#endregion 🔖️Handler

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
