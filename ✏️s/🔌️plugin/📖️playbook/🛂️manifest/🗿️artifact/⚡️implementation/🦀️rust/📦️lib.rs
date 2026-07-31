//! 🧩️ Playbook plugin — standalone strict-list, Blockly-like builder app bundled as a hot-swappable
//! WASM component. Independently launchable/testable without going through `forms`.
//!
//! 🪶️ Thin bundle after the constitutional split: `PlaybookPlayApp`'s `DocumentApp` impl, render, and
//! manifest live in `s/plugin/playbook/app/ui/rs` (`semio-s-app-playbook-ui`) — see
//! `s/plugin/playbook/app/{rs,engine,dsl,op,pack,protocol}` for its entities/compute/dsl/operations/
//! pack/protocol surfaces. The underlying Playbook domain model itself stays owned by the kernel
//! crate `s/kernel/playbook/rs` (`semio-s-kernel-playbook`), untouched by this split. This file keeps
//! only the pack<->dsl codec registration and the manual `PluginBundle` builder (not the
//! `semio_plugin!` macro — matches `space`'s precedent for plugins with a manual builder).

//#region 🔖️Constants
const PLAYBOOK_PLAY_PLUGIN_ID: &str = "playbook-play";
//#endregion 🔖️Constants

//#region 🔖️Manifest
/// 🗂️ Registers `PlaybookSpec`'s pack<->dsl codec under its real `document_schema()` string so
/// `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
/// playbook documents without depending on this crate's concrete `Projection`/`Operation` types.
fn register_playbook_play_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<playbook_ui::PlaybookPlayApp>(playbook::PLAYBOOK_DOCUMENT_SCHEMA);
}

fn playbook_play_bundle() -> semio_framework_plugin::PluginBundle {
    register_playbook_play_exports();
    semio_framework_plugin::PluginBundle::new(PLAYBOOK_PLAY_PLUGIN_ID, "Playbook", "0.1.0")
        .register_document_app(playbook_ui::create_playbook_play_app(), playbook_ui::PlaybookPlayApp::default)
}

semio_framework_plugin::plugin_exports!(playbook_play_bundle);
//#endregion 🔖️Manifest
