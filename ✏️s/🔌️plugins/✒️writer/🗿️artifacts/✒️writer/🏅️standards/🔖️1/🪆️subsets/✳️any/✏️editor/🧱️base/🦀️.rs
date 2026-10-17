//! 🧱️ Writer editor core: the app id, the UI value factories, the interaction ids, the typed media I/O surface and the whole-document reset effect every other layer of the editor builds on.

use semio_framework_plugin::{ActionDescriptor, AppIo, MediaClass, MediaForm, MediaType};
use pack::snapshot::ArtifactPack;
use serde::{Deserialize, Serialize};
use semio_framework::kernel::Effect;
use crate::{WRITER_DOCUMENT_SCHEMA, WriterSnapshot, writer_text};

//#region 🔖️Constants
pub const WRITER_PLAY_APP_ID: &str = "s.writer.writer@1/*#editor";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn writer_action(action: &str, args: Option<semio_framework_value::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: WRITER_PLAY_APP_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits display text into a bounded semantic label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    value.as_ref().try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "writer label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub use semio_framework_plugin::ui_node_list;

/// 🕹️ The `ast` interaction domain this app declares, and its only granularity — the document tree
/// binds the domain once and stamps this granularity on every AST row.
pub const WRITER_INTERACTION_AST: &str = "ast";
pub const WRITER_INTERACTION_GRANULARITY: &str = "node";

//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports plus one
/// extra output, `text:out` (Text×Document, kind `text.document`, `Many` — a workflow may fan this
/// writer's text out to several consumers, e.g. `playbook`'s `chapters:in`).
pub fn writer_io() -> AppIo {
    AppIo {
        artifact_schema: WRITER_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "text:out".into(),
            label: "Text".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
            kind_id: Some("text.document".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "text.document".into(), name: "Text Document".into(), dimension: "text".into(), component_kind: "writer".into() },
    }
}

/// 📤️ The JSON shape `"text:out"` exports and `playbook`'s `"chapters:in"` imports — one writer
/// document's text as one "chapter" (`title` mirrors the document id, `language_id` lets an importer
/// route jack/wire content differently from prose if it ever wants to). Single consumer (this file's
/// `export_media`), so it lives here rather than in the artifact's `🧬️schema`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriterChapterPayload {
    pub id: String,
    pub title: String,
    pub text: String,
    pub language_id: String,
}

/// 🎞️ Projects a `WriterSnapshot` onto the `"text:out"` chapter payload shape.
pub fn writer_chapter_payload(document: &WriterSnapshot) -> WriterChapterPayload {
    WriterChapterPayload { id: document.id.clone(), title: document.id.clone(), text: writer_text(document), language_id: document.language_id.clone() }
}

/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE history —
/// the sanctioned non-mutation path for a whole-document replace (open file, load example, dev JSON
/// setters). Per the SMO-agreed mutation taxonomy, whole-document replace has NO mutation-enum
/// representative; every former "replace the whole document"
/// gesture builds this effect instead of an `Emit::mutations([...])` — mirrors `📐️cad`'s identical
/// `reset_document_effect` (`📓️wave3-reports/cad-report.md`). The spr is a fresh, edit-free op-log
/// for `scene`'s own `schema`/`id` — a genesis envelope with no history to encode.
fn reset_document_effect_now(scene: &WriterSnapshot) -> Effect {
    let pack = <WriterSnapshot as ArtifactPack>::encode_pack(scene);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr(&scene.id, &scene.schema));
    Effect::LoadDocument { pack, spr }
}

pub fn reset_document_effect(scene: &WriterSnapshot) -> Effect {
    reset_document_effect_now(scene)
}
//#endregion 🔖️Io
