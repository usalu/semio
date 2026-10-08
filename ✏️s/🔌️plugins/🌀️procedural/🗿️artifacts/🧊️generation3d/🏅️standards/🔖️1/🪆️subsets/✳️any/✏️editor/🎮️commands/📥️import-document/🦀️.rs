//! 📥️ Generation3d play app commands command — `import-document`: the verb the shell dispatches with the
//! file picked by `📂️import-document-request`. The host's chunked inbound lane is reassembled by the
//! framework before this action runs (`semio_framework::kernel::ImportStaging`, admitted in the SDK's
//! `dispatch_action`), so the action decodes ONE whole `{ name, payload }` and stages nothing itself.
//!
//! 🐛️ Until ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane the nine import leaves were
//! round-trip tested and unreachable from any command, menu or button
//! (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1).
//!
//! 🧬️ Importing REPLACES the whole document, and it does so the way `🎨️set-active-example` does: as the artifact's
//! load effect (`reset_generation3d_document_effect`, an `Effect::LoadDocument` outside undo history). A natural-file
//! import is the load/genesis path, never a mutation row diffed between the old and the imported document.
//!
//! 📷️ The camera and the selected generation ride the CONFIG lane (`config_load_mutations`), exactly as an example switch
//! does: one concrete config leaf per field the loaded document changes.
//!
//! ⏳️ Progress and cancellation belong to the transfer: the shell reports chunk progress and cancels
//! between chunks as a Tasks-window task (`dispatchOpenedFiles`, `🛠️ShellHelpers/🟦️.tsx`), and a
//! cancelled pick's unfinished run gives its framework staging slot to the next pick.
//!
//! @see ../../../🚪️io/🦀️.rs — `document_io::import_document_bytes`, the composition point this calls.
//! @see ../🎨️set/🦀️.rs — the same whole-document replacement, from a bundled example.

use crate::editor::generation3d::commands::set_active_example::config_load_mutations;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::io::document_io;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;

use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📏️Bounds
/// 📏️ Largest payload ONE import may carry.
///
/// 🧾️ Derived, never a literal: an imported file is planted in the graph as an `InputNote`'s text
/// (`mesh_bridge::import_document`), so the whole payload has to fit ONE Artifact-lane edit —
/// `GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES`. A payload past it is refused by name before anything
/// is decoded, rather than by the store after the graph was rebuilt.
pub const GENERATION3D_IMPORT_TOTAL_BYTES: usize = crate::editor::generation3d::GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES;

/// 🏷️ The stable code of the one size refusal (`🧫️fixtures/🚪️io/🗿️artifact-surface.json` `faultCodes.capacity`).
pub const GENERATION3D_IMPORT_CAPACITY_CODE: &str = "generation3d-import-capacity";
//#endregion 📏️Bounds

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "import-document")]
#[value(rename_all = "camelCase")]
pub struct ImportDocument {
    pub name: String,
    pub payload: String,
    pub widget_id: Option<String>,
    pub channel: Option<String>,
    pub texture_id: Option<String>,

}

fn import_fault(code: &str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message.into())
}

/// 📏️ Admits one whole payload against [`GENERATION3D_IMPORT_TOTAL_BYTES`] — a refusal is typed and named,
/// never a truncation.
pub fn admit_payload(payload: &str) -> Result<(), Fault> {
    if payload.len() > GENERATION3D_IMPORT_TOTAL_BYTES {
        return Err(import_fault(GENERATION3D_IMPORT_CAPACITY_CODE, format!("an imported file of {} bytes exceeds one artifact-lane edit ({GENERATION3D_IMPORT_TOTAL_BYTES} bytes)", payload.len())));
    }
    Ok(())
}

/// 📥️ Admits the whole payload and replaces the document with what it holds.
pub fn emit(payload: &ImportDocument, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    admit_payload(&payload.payload)?;
    match (&payload.widget_id, &payload.channel, &payload.texture_id) {
        (None, None, None) => apply_complete_payload(&payload.name, &payload.payload, doc, cfg),
        (Some(widget), Some(channel), Some(texture)) => {
            let (id, channel, text) = super::set_widget_input::mesh_source_text(&doc.snapshot.host_snapshot, widget, channel).map_err(Fault::from)?;
            let edited = super::set_widget_input::import_mesh_texture(&text, texture, &payload.payload).map_err(Fault::from)?;
            let artifact_mutations = if edited == text { Vec::new() } else { vec![crate::standards::v1::subsets::any::schema::mutations::change_widget_input::change_widget_input(&id, &channel, crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue::Text(edited))] };
            Ok(Emit { artifact_mutations, ..Default::default() })
        }
        _ => Err(Fault::from("Choose a complete texture target")),
    }
}

/// 📥️ Decodes one whole payload and replaces the document with what it holds.
///
/// 🚨️ Bytes that are not the format their name claims are a TYPED fault carrying why. They are never
/// an empty document reported as success — the exact failure mode ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END removed from the leaves themselves
/// (`📓️io-codecs-2026-09-09.md` §1), which a silent `Ok(Emit::default())` here would have
/// reintroduced one layer up.
pub fn apply_complete_payload(
    name: &str,
    payload: &str,
    _doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let imported = document_io::import_document(name, payload).map_err(|error| import_fault("generation3d.io.import", error.to_string()))?;
    let effect = crate::editor::generation3d::reset_generation3d_document_effect(&imported);
    let config_mutations = config_load_mutations(cfg.snapshot, &imported.host_snapshot.camera, imported.generation.selected_generation_id.clone());
    imported.retire_cold();
    Ok(Emit { effects: vec![effect], config_mutations, ..Default::default() })
}

/// 🧵️ The session-free entry point: the import needs no evaluation session, only the document and its config.
pub fn handle(
    payload: &ImportDocument,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit(payload, doc, cfg)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
