#!/usr/bin/env python3
"""🪶️ Thins the wgpu renderer top crate: drops the empty Shell chunk files, moves the five wasm host hooks to the renderer root, gates the test-only chunk hosts behind cfg(test).

Asserts every anchor is unique before it writes; --dry prints the plan only.
"""
import os, re, sys

R = "/Users/ueli/Documents/semio"
E = R + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SH = E + "/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu"
ROOT = E + "/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs"
PARENT = SH + "/🦀️.rs"
WORLD_TEST = R + "/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🧪️tests/🔬️unit/🦀️.rs"
DOCS_STAGE = SH + "/📥️staging/📄️docs/🧵️shell-async-runners/🦀️.rs"
DRY = "--dry" in sys.argv

DELETE = {
    "🎬️shell-actions": ("shell_actions", "pub use"),
    "🧩️session-and-topology": ("session_and_topology", None),
    "💬️chrome-dialog-and-tooltips": ("chrome_dialog_and_tooltips", None),
    "🧭️navbar-footer-and-overlays": ("navbar_footer_and_overlays", None),
    "📨️command-dispatch": ("command_dispatch", None),
    "🖼️chrome-frame-and-main-window": ("chrome_frame_and_main_window", "pub(crate) use"),
    "🏛️space-admin-and-settle": ("space_admin_and_settle", "pub use"),
    "🧰️window-actions-and-commands": ("window_actions_and_commands", "pub(crate) use"),
    "🎓️tutorial-runtime": ("tutorial_runtime", "pub use"),
    "🗨️chrome-overlays-and-tour": ("chrome_overlays_and_tour", "pub use"),
    "📤️icon-export": ("icon_export", None),
    "🪟️tree-windows": ("tree_windows", None),
    "🔖️identity-and-types": ("identity_and_types", "pub use"),
    "🖱️pointer-input": ("pointer_input", "pub use"),
    "🧵️shell-async-runners": ("shell_async_runners", "pub(crate) use"),
}
GATE = {
    "⏪️time-travel": ("time_travel", None),
    "📎️local-folders": ("local_folders", None),
    "⚓️dock-model-and-measures": ("dock_model_and_measures", "pub use"),
    "🧾️panel-records": ("panel_records", "pub(crate) use"),
    "⚙️settings-and-dock-persistence": ("settings_and_dock_persistence", None),
    "⌨️keyboard-and-palette": ("keyboard_and_palette", None),
    "🧱️chrome-paint-and-navbar": ("chrome_paint_and_navbar", "pub use"),
    "🔑️storage-prefs-and-i18n": ("storage_prefs_and_i18n", "pub use"),
    "🎨️theme-file-import-and-accessibility": ("theme_file_import_and_accessibility", "pub use"),
}


def once(text, needle):
    assert text.count(needle) == 1, (needle[:90], text.count(needle))
    return text


def mount_block(slug, mod, reexport):
    block = f'#[path = "{slug}/🦀️.rs"]\nmod {mod};\n'
    if reexport:
        block += f"{reexport} {mod}::*;\n"
    return block


def main():
    parent = open(PARENT, encoding="utf8").read()
    for slug, (mod, reexport) in DELETE.items():
        block = mount_block(slug, mod, reexport)
        once(parent, block)
        parent = parent.replace(block + "\n", "") if (block + "\n") in parent else parent.replace(block, "")
        inc = f'    include_str!("{slug}/🦀️.rs"),\n'
        assert parent.count(inc) <= 1, slug
        parent = parent.replace(inc, "")
    for slug, (mod, reexport) in GATE.items():
        block = mount_block(slug, mod, reexport)
        once(parent, block)
        gated = f'#[cfg(test)]\n#[path = "{slug}/🦀️.rs"]\nmod {mod};\n'
        if reexport:
            gated += f"#[cfg(test)]\n{reexport} {mod}::*;\n"
        parent = parent.replace(block, gated)
    contracts = (
        "pub use semio_framework_os_renderer_wgpu_contracts::{try_push_find_item, ActiveShellFindItems, ShellFindItem, ShellFindItemBinding, ShellFindItems, SHELL_FIND_ITEM_CAPACITY, SHELL_FIND_PAYLOAD_BYTES};\n"
    )
    anchor = "//#region 🧬️RendererWp4Components\n"
    once(parent, anchor)
    parent = parent.replace(anchor, contracts + "\n" + anchor)
    doc_old = "//! 🧩 This file keeps the shared imports, the extension-ledger types, [`ShellState`] and its `Drop`; every other\n//! concern lives in a sibling chunk module wired below, each a `<emoji>️<kebab>/🦀️.rs` folder that reaches the shared\n//! imports through `use super::*;` and is re-exported here, so every `crate::shell::...` path is unchanged.\n"
    doc_new = "//! 🧩 This file keeps the shared imports and re-exports the Shell crates, so every `crate::shell::...` path is unchanged.\n//! The chunk folders that remain next to it are `cfg(test)` hosts of the tests that need the monolith fixtures; each reaches\n//! the shared imports through `use super::*;`.\n"
    once(parent, doc_old)
    parent = parent.replace(doc_old, doc_new)

    root = open(ROOT, encoding="utf8").read()
    hooks = '''
//#region 🏠️ShellHostHooks
/// 🏠️ wasm hub environment hook: the page hands the hub url, user and data directory to the Shell before the first frame.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHubEnv)]
pub fn semio_wgpu_set_hub_env(hub_url: String, user: String, data_dir: String) {
    let data_dir = if data_dir.is_empty() { None } else { Some(data_dir) };
    shell::BOOT_HUB_ENV.with(|cell| *cell.borrow_mut() = Some((hub_url, user, data_dir)));
}

/// 🪟️ wasm window blur hook: the page tells the Shell its host window lost focus.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuHostWindowBlur)]
pub fn semio_wgpu_host_window_blur() {
    shell::note_host_window_blur();
}

/// 🗄️ wasm host storage hook: the page seeds the Shell's persisted keys from its own storage.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHostStorage)]
pub fn semio_wgpu_set_host_storage(snapshot_json: String) {
    let accepted = shell::seed_host_storage(&snapshot_json);
    shell::debug_log(&format!("[TRACE] wgpu-shell host storage seeded entries={accepted}"));
}

/// 🔭️ wasm agent bridge diagnostics hook: reads the offer scope the Shell published.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = dumpAgentBridgeOfferScope)]
pub fn dump_agent_bridge_offer_scope() -> String {
    shell::PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE.with(|slot| slot.borrow().clone())
}

/// 🌉️ wasm agent bridge hook: arms the bridge configuration the next frame admits.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetAgentBridgeConfig)]
pub fn semio_wgpu_set_agent_bridge_config(url: String, admission_proof: String) {
    let config = if url.trim().is_empty() {
        None
    } else {
        match crate::agent_bridge::AgentBridgeConfig::admit(&url, &admission_proof) {
            Ok(config) => Some(config),
            Err(error) => {
                shell::debug_log(&format!("[TRACE] wgpu-shell agent bridge config refused: {error}"));
                return;
            }
        }
    };
    let armed = config.is_some();
    shell::PENDING_AGENT_BRIDGE_CONFIG.with(|slot| *slot.borrow_mut() = Some(config));
    shell::debug_log(&format!("[TRACE] wgpu-shell agent bridge config armed={armed}"));
}
//#endregion 🏠️ShellHostHooks
'''
    tail = '''#[cfg(target_arch="wasm32")]
#[wasm_bindgen(js_name=semioWgpuSetPluginCatalog)]
pub fn semio_wgpu_set_plugin_catalog(serialized:String)->Result<(),JsValue>{install_plugin_catalog_json_v1(&serialized).map_err(|error|JsValue::from_str(&error))}
'''
    once(root, tail)
    root = root.replace(tail, tail + hooks)

    world = open(WORLD_TEST, encoding="utf8").read()
    prefix = "../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/"
    for slug in DELETE:
        item = f'"{prefix}{slug}/🦀️.rs", '
        item2 = f', "{prefix}{slug}/🦀️.rs"'
        if item in world:
            assert world.count(item) == 1, slug
            world = world.replace(item, "")
        elif item2 in world:
            assert world.count(item2) == 1, slug
            world = world.replace(item2, "")

    docs = open(DOCS_STAGE, encoding="utf8").read()
    const_line = "pub const SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES: usize"
    idx = docs.index(const_line)
    end = docs.index("\n", idx)
    assert "const _: [(); 72]" not in docs
    docs = docs[: end + 1] + "const _: [(); 72] = [(); SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES];\n" + docs[end + 1:]

    if DRY:
        print("dry run ok")
        return
    for path, text in ((PARENT, parent), (ROOT, root), (WORLD_TEST, world), (DOCS_STAGE, docs)):
        open(path, "w", encoding="utf8").write(text)
    for slug in DELETE:
        path = f"{SH}/{slug}/🦀️.rs"
        os.remove(path)
        os.rmdir(f"{SH}/{slug}")
    print("applied")


main()
