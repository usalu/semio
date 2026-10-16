import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/ri1-tools")
from ed import *

C = T + "/🤝️contracts"
sub1(C + "/🦀️.rs", '''#[path = "📥️lifted/🧩️retained-ui-engine/🦀️.rs"]
mod retained_ui_engine;
pub use retained_ui_engine::*;
''', '''#[path = "📥️lifted/🧩️retained-ui-engine/🦀️.rs"]
mod retained_ui_engine;
pub use retained_ui_engine::*;

#[path = "📥️lifted/🗃️scene-state/🦀️.rs"]
mod admitted_surfaces;
pub use admitted_surfaces::*;

#[path = "📥️lifted/🔖️identity-and-types/🦀️.rs"]
mod find_items;
pub use find_items::*;

#[path = "📥️lifted/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod hub_remote;
pub use hub_remote::*;
''')
sub1(C + "/🦀️.rs", "//! 🧭️ Native and wasm32, no first-party dependencies: types, constants, statics and pure accessors only. The element contracts module file keeps its place in the target tree; items lifted out of other packages arrive below `📥️lifted` and are mounted here.",
 "//! 🧭️ Native and wasm32, no first-party dependencies: types, constants, statics and pure accessors only. The element contracts module file keeps its place in the target tree; items lifted out of other packages arrive below `📥️lifted` and are mounted here: the program fault, the framework widget context, the retained UI engine cell with its close queue and pointer owners, the admitted surface map, the shell find items and the hub remote status.")
M = C + "/📦️packages/🦀️rust/Cargo.toml"
sub1(M, 'serde_json = "1.0.140"\nui_contract', 'serde = { version = "1.0.219", features = ["derive"] }\nserde_json = "1.0.140"\nui_contract')

L = C + "/📥️lifted/🧩️retained-ui-engine/🦀️.rs"
sub1(L, '''pub fn ui_document_close_pending_for(window_id: &str) -> bool {
    UI_DOCUMENT_CLOSE_QUEUE.with(|cell| cell.borrow().owns_window(window_id))
}
''', '''pub fn ui_document_close_pending_for(window_id: &str) -> bool {
    UI_DOCUMENT_CLOSE_QUEUE.with(|cell| cell.borrow().owns_window(window_id))
}

pub fn component_scene_is_presented(witness: &ui_wgpu::wgpu::reconcile::UiComponentSceneWitness<'_>) -> bool {
    UI_ENGINE.with(|cell| cell.borrow().presented_component_scene_matches(witness))
}
''')
I = EL + "/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs"
sub1(I, '''pub(crate) fn component_scene_is_presented(witness: &ui_wgpu::wgpu::reconcile::UiComponentSceneWitness<'_>) -> bool {
    UI_ENGINE.with(|cell| cell.borrow().presented_component_scene_matches(witness))
}

''', '')
sub1(I, "    install_ui_engine_locale, ui_document_close_pending, ui_document_close_pending_for, ScenePointerOwner,", "    component_scene_is_presented, install_ui_engine_locale, ui_document_close_pending, ui_document_close_pending_for, ScenePointerOwner,")

NP = T + "/🖥️native-platform"
sub1(NP + "/🦀️.rs", '''#[path = "../♿️native-accessibility/🦀️.rs"]
pub mod native_accessibility;
''', '''#[path = "../♿️native-accessibility/🦀️.rs"]
pub mod native_accessibility;

#[cfg(not(target_arch = "wasm32"))]
#[path = "📥️lifted/🎞️Scenes/🦀️.rs"]
mod host_temporal_formatter;
#[cfg(not(target_arch = "wasm32"))]
pub use host_temporal_formatter::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../🕰️native-temporal/🦀️.rs"]
pub mod native_temporal;
''')
sub1(NP + "/🦀️.rs", "The temporal adapters (`native_temporal`) join when the scenes package lifts the host temporal formatter trait into `📥️lifted/🎞️Scenes` (plan section 5.2).", "The host temporal formatter port arrives from the scenes package through `📥️lifted/🎞️Scenes`, and the temporal adapters behind it live in the public module `native_temporal`.")
sub1(T + "/🕰️native-temporal/🦀️.rs", "use crate::scenes::NativeHostTemporalFormatter;", "use crate::NativeHostTemporalFormatter;")
