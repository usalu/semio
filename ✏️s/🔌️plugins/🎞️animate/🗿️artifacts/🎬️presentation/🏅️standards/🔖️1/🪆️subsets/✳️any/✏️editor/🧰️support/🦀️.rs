//! 🧰️ Animate editor support — the action factory, the fixed-capacity UI admission helpers, the interaction domain, the per-dispatch context and the shared effect and roster helpers that every command, panel and window of the editor builds with, so none of them reaches back into the editor root.

use crate::mutations::create_tile::CreateTile;
use crate::standards::v1::subsets::any::io::text::snapshot::build_tile_morph_prompt;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::{FigureTileDraft, PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use semio_framework_plugin::Effect;
use std::collections::HashSet;

//#region 🔖️Constants
pub const PRESENTATION_PLAY_APP_ID: &str = "s.animate.presentation@1/*#editor";

/// 🎯️ Binds catalogue actions to the canonical presentation editor.
pub fn animate_presentation_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(PRESENTATION_PLAY_APP_ID).action(action, args)
}

/// 🌳️ The framework's one node-list admission — the panel-local copy is gone (ticket
/// 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.3).
pub use semio_framework_plugin::ui_node_list;

/// 🏷️ Admits a semantic presentation label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| ui_capacity_error())
}

/// 📝️ Admits a fixed presentation UI string.
pub fn ui_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::UiText> {
    semio_framework_ui_contract::UiText::try_from_str(value.as_ref()).ok_or_else(ui_capacity_error)
}

/// 🧱️ Finalizes a presentation UI node with explicit identity.
pub fn ui_node<B: semio_framework_ui_contract::HasBase + semio_framework_ui_contract::Buildable>(builder: B, id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    builder.try_id(id).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())
}

/// 👶️ Admits a complete collection of presentation UI children.
pub fn ui_children<B: semio_framework_ui_contract::HasChildren>(builder: B, children: impl IntoIterator<Item = semio_framework_plugin::BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<B> {
    builder.try_children(children).map_err(|_| ui_capacity_error())
}

/// 🗺️ Admits structured presentation action arguments.
pub fn ui_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(ui_capacity_error)?;
    for (key, value) in values {
        builder.push(key.to_string(), value).map_err(|_| ui_capacity_error())?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🚧️ Reports fixed-capacity UI admission failure.
pub fn ui_capacity_error() -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("animate.ui.capacity", "presentation UI admission failed")
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "tiles" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Flat` over the tile grid (a document panel tree binds it
/// directly; the canvas hit-tests a click and asks the framework to apply it, mirroring `🖍️draw`'s
/// "strokes" domain).
pub const PRESENTATION_INTERACTION_DOMAIN: &str = "tiles";
pub const PRESENTATION_INTERACTION_GRANULARITY: &str = "tile";

/// 🕹️ Per-dispatch scratch: the "tiles" domain's current selection, resolved once by
/// `ArtifactApp::handle` from `InteractionView` and threaded to every leaf command handler —
/// `app_commands!`'s generated `dispatch` has no way to thread `InteractionView` itself.
pub struct PresentationDispatchCtx {
    pub selected_ids: Vec<String>,
}

/// 🕹️ JSON-encodes `ids` as the `Vec<InteractionTarget>` string the framework's `interactionSelect`
/// action requires in its `targets` arg — every hit id shares the domain's one granularity.
fn interaction_targets_json(ids: &[String]) -> String {
    let targets = ids.iter().map(|id| semio_framework_pack_json::object([("granularity".to_string(), semio_framework_pack_json::Value::from(PRESENTATION_INTERACTION_GRANULARITY)), ("id".to_string(), semio_framework_pack_json::Value::from(id.clone()))])).collect();
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::Array(targets))
}

/// 🕹️ Requests the shell to redispatch the framework-owned `interactionSelect` verb through its
/// normal action funnel — the only way `canvas-pointer-down`'s hit test can drive selection now that
/// it is framework-owned state, never a `PresentationConfigMutation` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn interaction_select_effect(ids: &[String], merge: &str) -> Effect {
    Effect::ReplayShellCommand {
        action_id: semio_framework::INTERACTION_SELECT_ACTION_ID.into(),
        args: Some(semio_framework_value::DslValue::object([
            ("domainId".to_string(), semio_framework_value::DslValue::String(PRESENTATION_INTERACTION_DOMAIN.into())),
            ("targets".to_string(), semio_framework_value::DslValue::String(interaction_targets_json(ids))),
            ("merge".to_string(), semio_framework_value::DslValue::String(merge.into())),
            ("method".to_string(), semio_framework_value::DslValue::String("pick".into())),
        ])),
    }
}
//#endregion 🔖️Interaction

//#region 🔖️Helpers
/// 🔢️ Mints a fresh, process-unique tile id — shared by `🎮️commands/🀄️add-tile::add_tile` and
/// `🎮️commands/⌨️engagement::engagement_submit`'s `"add"` keyword.
pub fn new_tile_id(prefix: &str) -> String {
    let serial = {
        let hex = framework_hash::hash_bytes(concat!(file!(), line!()).as_bytes());
        u64::from_str_radix(&hex[..8], 16).unwrap_or(1)
    };
    format!("{prefix}-{serial}")
}

/// 🧱️ The concrete rows that make the tile roster `next`: one `delete-tiles` of every tile the deck holds, then one position-exact
/// `create-tile` per new tile — never a whole-roster replace.
pub fn tile_roster_mutations(deck: &PresentationSnapshot, next: Vec<FigureTileDraft>) -> Vec<PresentationMutation> {
    let (_, held) = crate::presentation_working_scene(deck);
    let cleared = (!held.is_empty()).then(|| PresentationMutation::DeleteTiles(crate::mutations::delete_tiles::DeleteTiles { ids: held.iter().map(|tile| tile.id.clone()).collect() }));
    cleared.into_iter().chain(next.into_iter().enumerate().map(|(index, tile)| PresentationMutation::CreateTile(CreateTile { index, tile }))).collect()
}

/// 🧹️ Retains only the ids that reference an existing tile in `deck` — shared by every command that
/// accepts a selection/target id list.
pub fn valid_tile_ids(deck: &PresentationSnapshot, ids: Vec<String>) -> Vec<String> {
    let (_, tiles) = crate::presentation_working_scene(deck);
    let valid: HashSet<&str> = tiles.iter().map(|tile| tile.id.as_str()).collect();
    ids.into_iter().filter(|id| valid.contains(id.as_str())).collect()
}

/// 📋️ Host effect delivering the generated tile-morph prompt to the user as a downloadable markdown
/// file — the genuine shell side-effect that replaces the retired ephemeral clipboard scratch (the
/// landed `Effect` contract carries no clipboard variant, so the prompt is exported as media).
/// Shared by `🎮️commands/📋️copy-prompt::copy_prompt` and `🎮️commands/⌨️engagement::engagement_submit`'s
/// `"copy"`/`"copy prompt"` keywords.
pub fn tile_morph_prompt_effect(deck: &PresentationSnapshot) -> Effect {
    let (source, tiles) = crate::presentation_working_scene(deck);
    Effect::DownloadMediaExport { filename: "tile-morph-prompt.md".into(), mime_type: "text/markdown".into(), data: build_tile_morph_prompt(&source, &tiles), encoding: None }
}

/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "reset the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) that
/// `🎮️commands/📥️set-source::set_active_example` uses instead of the banned whole-snapshot mutation. The
/// spr is a fresh, edit-free op-log — a genesis envelope with no history to encode.
pub fn reset_presentation_document_effect(document: &PresentationSnapshot) -> Effect {
    let pack = <PresentationSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    // 🪦️ NOT `create_document_envelope` + `print_document_spr`: an envelope is a terminal store shell
    // whose `Drop` asserts that its app-owned bounded retirement authority detached every nested
    // owner first, and nothing here ever mounts or retires it — so building this effect panicked the
    // guest (`unreachable`) at boot, before any window kind existed. `🗒️note`/`✒️writer`/`📐️cad` all
    // take the `empty_document_spr` route; the log is edit-free by construction either way.
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("presentation", PRESENTATION_DOCUMENT_SCHEMA));
    Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Helpers
