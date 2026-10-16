#!/usr/bin/env python3
# 🔀️ Applies the Interpreter flip (see ifx.py): rewritten member files, three crate roots, the retained-ui-engine intent mount and the facade parent.
import sys, os, re, json, shutil
sys.argv = [sys.argv[0]]
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ifx
from ed import *

PRE = SP + "/ri1/pre-ifx"
if not os.path.exists(PRE):
    os.makedirs(PRE)
    shutil.copytree(INT := ifx.INT, PRE + "/Interpreter")
    for c in ifx.FOLDER.values():
        shutil.copytree(T + "/" + c, PRE + "/" + c)
    shutil.copy(T + "/🧊️renderer/📥️mounts/🎞️wp2-scenes/🦀️.rs", PRE + "/hub-wp2.rs")
    shutil.copy(T + "/🧊️renderer/📥️mounts/🧵️wp3-frame-core/🦀️.rs", PRE + "/hub-wp3.rs")

INT = ifx.INT
parent_text = read(INT + "/🦀️.rs")
retained_text = read(ifx.MODS["ui_engine"]["retained_ui_engine"])
for crate, mods in ifx.MODS.items():
    for mod, path in mods.items():
        write(path, ifx.transform(path, crate))

INTENT_MOUNT = '''#[path = "🎬️scene-intents-and-clipboard/🦀️.rs"]
mod scene_intents_and_clipboard;
pub use scene_intents_and_clipboard::*;'''
RU = ifx.MODS["ui_engine"]["retained_ui_engine"]
sub1(RU, INTENT_MOUNT, "pub use semio_framework_os_renderer_wgpu_ui_engine_focus::scene_intents_and_clipboard::*;")

def rel(crate, path):
    return os.path.relpath(path, T + "/" + ifx.FOLDER[crate])

def mounts(crate):
    out = ""
    for mod, path in ifx.MODS[crate].items():
        out += f'#[path = "{rel(crate, path)}"]\npub mod {mod};\n\n'
    return out

TESTS = '#[cfg(test)]\n#[path = "🧪️tests/🦀️.rs"]\nmod tests;\n'
focus_imports = '''extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
use crate::element_contracts::WorkerCell;
use semio_framework_os_renderer_wgpu_contracts::{ScenePointerOwner, UI_ENGINE};
use ui_wgpu::wgpu::{ActionDescriptor, NodeId, Rect, UiNode};
#[cfg(not(target_arch = "wasm32"))]
use crate::ink_and_vfs_focus::complete_pending_native_ink_clipboard;
#[cfg(all(not(target_arch = "wasm32"), not(test)))]
use crate::ink_and_vfs_focus::{complete_pending_native_text_editor_clipboard, PendingNativeInkClipboardToken, PendingNativeTextEditorClipboardToken};
'''
write(T + "/👆️ui-engine-focus/🦀️.rs", '''//! 👆️ Focus and clipboard of the UI engine: text editor, node graph caret and table focus, virtual file system, ink editor and ink clipboard focus, and the scene intent queue with the mounted clipboard io.
//!
//! 🧭️ Native and wasm32: the focus state of the focusable scene surfaces, the caret application, the scene intent queue and the native clipboard streams that follow a focus change. The module files keep their place in the interpreter tree and are mounted here as public modules.

''' + focus_imports + "\n" + mounts("ui_engine_focus") + TESTS)
write(T + "/🔎️ui-engine-dump/🦀️.rs", '''//! 🔎️ Introspection of the UI engine: the dump and statistics surface, the chrome ledger with the chrome accessibility projection, and the mesh and board2d statistics.
//!
//! 🧭️ Native and wasm32: the diagnostic dumps the browser host reads and the chrome input ledger the shell publishes into. The module files keep their place in the interpreter tree and are mounted here as public modules.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;

''' + mounts("ui_engine_dump") + TESTS)
write(T + "/🖱️ui-engine/🦀️.rs", '''//! 🖱️ The retained UI engine: the retained engine cell users, scene interaction, the retained document consumer and the render plan validator.
//!
//! 🧭️ Native and wasm32: the dispatch of pointer, keyboard and accessibility events into the retained trees, the scene interaction step, the retained document consumer and the validation of component scene render plans. The module files keep their place in the interpreter tree and are mounted here as public modules; the `interpreter` module of the renderer is the facade over this crate and its two lower siblings.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;

''' + mounts("ui_engine") + TESTS)

new = parent_text
for crate, mods in ifx.MODS.items():
    for mod, path in mods.items():
        pat = re.compile(r'#\[path = "[^"]*"\]\n(?:pub )?mod ' + mod + r';\n')
        found = pat.findall(new)
        if mod == "scene_intents_and_clipboard":
            assert not found
            continue
        assert len(found) == 1, (mod, found)
        new = pat.sub(f"use {ifx.lib_of(crate)}::{mod};\n", new)
write(INT + "/🦀️.rs", new)
print("done")
