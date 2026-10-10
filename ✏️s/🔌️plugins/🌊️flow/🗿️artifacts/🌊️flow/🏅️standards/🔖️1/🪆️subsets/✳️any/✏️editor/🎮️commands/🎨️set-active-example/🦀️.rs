//! 🎨️ Flow play app command — `set-active-example`.

use crate::examples::demo;
use crate::{FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🎨️ The navbar example id. An empty id is the picker's cleared row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "set-active-example")]
#[derive(semio_framework_value::RetireOwned)]
pub struct SetActiveExample {
    pub example_id: String,
}

/// 🗃️ The whole-document load an example switch answers with. A `LoadDocument` effect is NOT an edit: it carries no mutation rows, no diff and no history
/// row; the runtime composes the loaded document's content child through `genesis_child_pack`, so the example is exactly what the window, the next verb and a reload see.
pub fn load_document_effect(document: &FlowSnapshot) -> semio_framework_plugin::Effect {
    let pack = <FlowSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("flow", crate::FLOW_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}

/// 🎨️ Resolves the navbar example id to the document it loads: the published `demo` or the default graph.
pub fn set_active_example_document(payload: &SetActiveExample) -> Result<FlowSnapshot, Fault> {
    if payload.example_id.is_empty() {
        Ok(FlowSnapshot::default())
    } else if payload.example_id == demo::ID {
        demo::snapshot_from_text(demo::PRIMARY_TEXT).map_err(|error| Fault::from(error.to_string()))
    } else {
        Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.example-unknown"), format!("setActiveExample has no example \"{}\"", payload.example_id)))
    }
}

/// 🎨️ The emit of one example switch: a single load-document effect and no mutation rows.
pub fn set_active_example_edit(payload: &SetActiveExample) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(Emit::effect(load_document_effect(&set_active_example_document(payload)?)))
}

/// 🎨️ Loads the published `demo` document, or restores the default snapshot when the id is empty.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    set_active_example_edit(payload)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
