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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "import-document-request")]
#[value(rename_all = "camelCase")]
pub struct ImportDocumentRequest {
    pub widget_id: Option<String>,
    pub channel: Option<String>,
    pub texture_id: Option<String>,
}

/// 📂️ Asks the shell for one file in any of this artifact's importable formats.
pub fn emit(payload: &ImportDocumentRequest, host: &semio_framework_artifact_flow_flow::FlowHostSnapshot) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let target = match (&payload.widget_id, &payload.channel, &payload.texture_id) {
        (None, None, None) => None,
        (Some(widget), Some(channel), Some(texture)) if !texture.is_empty() && texture.chars().count() <= 128 => {
            let (widget, channel, _) = super::set_widget_input::mesh_source_text(host, widget, channel).map_err(Fault::from)?;
            Some(semio_framework_value::DslValue::Object(vec![("widgetId".into(), semio_framework_value::DslValue::String(widget)), ("channel".into(), semio_framework_value::DslValue::String(channel)), ("textureId".into(), semio_framework_value::DslValue::String(texture.clone()))]))
        }
        _ => return Err(Fault::from("Choose a complete texture target")),
    };
    let accept = if target.is_some() { ".png,.jpg,.jpeg".into() } else { document_io::import_accept_filter().map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new(crate::GENERATION3D_IO_IMPORT_ACCEPT), error.to_string()))? };
    Ok(Emit::effect(Effect::RequestFileOpen {
        req: semio_framework_plugin::RequestId(GENERATION3D_IMPORT_REQUEST_ID),
        accept,
        read_as: Some("dataUrl".into()),
        import_action: GENERATION3D_IMPORT_ACTION.into(),
        multiple: false,
    args: target, }))
}

pub fn handle(
    payload: &ImportDocumentRequest,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    _cfg: &ConfigView<'_, Generation3dConfig>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    emit(payload, &doc.snapshot.host_snapshot)
}
