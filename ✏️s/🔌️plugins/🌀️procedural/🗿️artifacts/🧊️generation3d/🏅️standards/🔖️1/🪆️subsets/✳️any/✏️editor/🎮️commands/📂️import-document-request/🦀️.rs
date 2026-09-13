//! 📂️ Generation3d play app commands command — `import-document-request`: the palette/menu verb that
//! opens the file picker, mirroring process3d's `load-model-request`
//! (`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/…/✏️editor/🎮️commands/📤️media/🦀️.rs`).
//!
//! 🗂️ The `accept` filter is DERIVED from `document_io::IMPORT_FORMATS` — every importable
//! extension as its own owning `s.stdio.<format>` artifact declares it — so a format that renames an
//! extension cannot leave a stale filter behind, and a file the picker offers is always a file this
//! artifact can really read.
//!
//! @see ../../../🚪️io/🦀️.rs — `document_io::import_accept_filter`.
//! @see ../📥️import-document/🦀️.rs — the verb the shell re-dispatches with the picked file.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::io::document_io;
use crate::standards::v1::subsets::any::schema::mutations::text::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪪️ This app's own file-open request id — distinct from every other plugin's in the repo
/// (process3d 111, home 124), so two pickers can never answer each other's request.
pub const GENERATION3D_IMPORT_REQUEST_ID: u64 = 131;

/// 🎬️ The action the shell re-dispatches once per picked file, with `{ payload, name }`.
pub const GENERATION3D_IMPORT_ACTION: &str = "importDocument";

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "import-document-request")]
pub struct ImportDocumentRequest {}

/// 📂️ Asks the shell for one file in any of this artifact's importable formats.
pub fn emit() -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let accept = document_io::import_accept_filter().map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("generation3d.io.import-accept"), error.to_string()))?;
    Ok(Emit::effect(Effect::RequestFileOpen {
        req: semio_framework_plugin::RequestId(GENERATION3D_IMPORT_REQUEST_ID),
        accept,
        read_as: Some("dataUrl".into()),
        import_action: GENERATION3D_IMPORT_ACTION.into(),
        multiple: false,
    }))
}

pub fn handle(
    _payload: &ImportDocumentRequest,
    _doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit()
}
