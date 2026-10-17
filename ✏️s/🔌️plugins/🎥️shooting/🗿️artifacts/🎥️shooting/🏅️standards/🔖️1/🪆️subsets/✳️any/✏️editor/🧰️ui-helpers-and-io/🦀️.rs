//! 🧰️ UI node builders and ports of the shooting editor: the action constructors, the scene value helpers, the tree item builder and the photo io ports. See [`super`].

use crate::{ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_ui_locale::Label;

//#region 🔖️Constants
pub const SHOOTING_PLAY_APP_ID: &str = "s.shooting.shooting@1/*#editor";
const SHOOTING_PLAY_CONTROLLER_ID: &str = SHOOTING_PLAY_APP_ID;
/// 🕹️ The framework-owned interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)?
/// covering asset pick/marquee selection and pointer hover in the 3d scene window — granularity `"asset"`
/// only, `HierarchyProvider::Flat`. Shot selection is NOT part of this domain — see
/// `ShootingConfig::selected_shot_ids`'s doc comment.
pub const SHOOTING_INTERACTION_DOMAIN: &str = "assets";
//#endregion 🔖️Constants

//#region 🔖️Commands
/// 🕹️ Per-dispatch app-struct state that is neither document nor config: a read-only snapshot of the
/// `"assets"` interaction domain's current selection ids (see [`SHOOTING_INTERACTION_DOMAIN`]). The
/// `semio_framework_plugin::app_commands!`-generated `dispatch` has no way to thread `InteractionView`
/// itself (see that macro's own doc comment on `ctx`), so `ArtifactApp::handle` reads it once and hands
/// it down through this app-owned context instead — used by the retained `translate/rotate/scale-
/// Selection` gumball verbs (`🎮️commands/🧭️gumball`) as their fallback-to-current-selection source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShootingDispatchCtx {
    pub selected_asset_ids: Vec<String>,
}
//#endregion 🔖️Commands

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example, dev document load). Per `📓️taxonomy.md`, whole-document replace is banned outright with NO
/// replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this package (`import_media`'s
/// `"artifact:in"` above, `commands::document::{import_snapshot_json,set_active_example,reset_snapshot}`)
/// builds this effect instead of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log
/// for `scene` — `store::os_store::text_format::empty_document_spr`, never a minted `create_document_envelope` (an envelope
/// dropped without its bounded retirement authority traps the guest on Drop).
pub fn reset_document_effect(scene: &ShootingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <ShootingSnapshot as pack::snapshot::ArtifactPack>::encode_pack(scene);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("shooting", SHOOTING_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument


//#region 🔖️Utilities
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`☑️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn shooting_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(SHOOTING_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎛️ Addresses host window chrome with its native action descriptor.
pub fn shooting_window_action(action: &str, args: Option<DslValue>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: SHOOTING_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits localized text into the semantic UI contract.
pub fn ui_label(value: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    value.try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.label.capacity", "fixed UI label admission failed"))
}

/// 📝️ Admits a fixed shooting UI string.
pub fn ui_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::UiText> {
    semio_framework_ui_contract::UiText::try_from_str(value.as_ref()).ok_or_else(ui_capacity_error)
}

/// 🧱️ Finalizes a shooting UI node with explicit identity.
pub fn ui_node<B: semio_framework_ui_contract::HasBase + semio_framework_ui_contract::Buildable>(builder: B, id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    builder.try_id(id).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())
}

/// 👶️ Admits a complete collection of shooting UI children.
pub fn ui_children<B: semio_framework_ui_contract::HasChildren>(builder: B, children: impl IntoIterator<Item = semio_framework_plugin::BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<B> {
    builder.try_children(children).map_err(|_| ui_capacity_error())
}

/// 🚧️ Reports fixed-capacity UI admission failure.
pub fn ui_capacity_error() -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("shooting.ui.capacity", "shooting UI admission failed")
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

/// 🌳️ Layers an `icon_id` onto the SDK's `tree_item_with_action` skeleton — the SDK primitive's third
/// parameter is `description`, not an icon, so the shooting-specific icon assignment stays local. Shared
/// by the document and catalogue panels (two consumers)?.
pub fn tree_item_with_icon(
    id: impl AsRef<str>,
    label: impl TryInto<Label>,
    icon_id: &str,
    action: semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let label: Label = label.try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.label", "tree-item label conversion failed"))?;
    let mut node = tree_item_with_action(id, label.as_str(), None, action?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.icon", "fixed tree-item icon admission failed"))?);
    }
    Ok(node)
}
//#endregion 🔖️Utilities

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifact_kind` already declares (schema/media type/presentation fields
/// copied verbatim); the sole app-specific port is `photos:out` (see `shooting_photos_out_port` below)
/// — the implicit document in/out ports cover the rest.
///
/// ⚠️ `export_formats`/`import_formats` stay empty: `AppIo` (unlike `ArtifactKindSpec`) carries no
/// `export_stdio_kinds`/`import_stdio_kinds` string peer to hold the real `["s.stdio.svg",
/// "s.stdio.png"]` list, and its field type (a `Vec` of the framework's closed media-format enum) is
/// framework-owned (`🧰️framework/🔨️modules/🛂️manifest`), out of this plugin's write scope. Confirmed
/// dead as of this migration —
/// `app.io.export_formats`/`import_formats` have no framework reader (`app.io.all_ports()`/
/// `document_schema`/`artifact.component_kind` are the only fields anything consumes) — so emptying
/// them drops no live behavior. `crate::artifact_kind()`'s `export_stdio_kinds`/
/// `import_stdio_kinds` remain the live source of truth for this artifact's real format list.
pub fn shooting_io() -> AppIo {
    AppIo {
        artifact_schema: SHOOTING_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        ports: vec![shooting_photos_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.shooting".into(), name: "2D Shooting".into(), dimension: "2d".into(), component_kind: "shooting".into() },
    }
}

/// 🔌️ `photos:out` — the shooting document's captured photo(s), as `2d.image` raster media (workflow
/// port surface; WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe).
/// `Many`/optional: a shooting document may carry several shots, and downstream consumers (e.g.
/// remodel's `photos:in`) may connect before any shot exists.
pub fn shooting_photos_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "photos:out".into(),
        label: "Photos".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Exports the active shot's rendered scene as a `2d.image` `Media` payload for the `photos:out`
/// port — reuses the same SVG-then-rasterize pipeline (`crate::standards::v1::subsets::any::io::text::snapshot::shooting_scene_svg` +
/// `rasterize_svg_to_png_base64`) as the `exportActiveShot`/PNG shell action, so there is exactly one
/// photo renderer.
pub fn shooting_photo_media(snapshot: &ShootingSnapshot) -> Result<Media, MediaError> {
    let (svg, width, height) = crate::standards::v1::subsets::any::io::text::snapshot::shooting_scene_svg(snapshot).map_err(|error| MediaError::Payload("photos:out".into(), error))?;
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| MediaError::Payload("photos:out".into(), error))?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 } })
}
//#endregion 🔖️Io
