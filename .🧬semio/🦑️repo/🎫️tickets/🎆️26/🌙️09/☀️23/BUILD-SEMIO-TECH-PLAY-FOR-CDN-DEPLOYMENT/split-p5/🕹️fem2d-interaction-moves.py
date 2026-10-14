#!/usr/bin/env python3
"""🕹️ Breaks the fem2d editor's windows ↔ interaction cycle in the domain (stage 2 of the P5 fem2d cut): the canvas
geometry (`SCALE_2D`, `ORIGIN_2D`, `screen_2d`, `find_node_2d`, `fem2d_element_endpoints`) moves from the model window
down into the interaction module; the addressed-window lookups (`fem2d_addressed_window_kind`, `fem2d_addressed_camera`,
`fem2d_addressed_body_key`) move up into the edit mode, which owns both windows; the canvas gesture entry points take the
addressed body key from their caller, and `fem2d_gesture_window_id_for_render` takes a window kind instead of a body key.
"""
import re, os
E = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor"
def rd(p): return open(os.path.join(E, p), encoding="utf-8").read()
def wr(p, s): open(os.path.join(E, p), "w", encoding="utf-8").write(s)
def cut(s, a, b):
    i = s.index(a); j = s.index(b, i) + len(b)
    return s[:i] + s[j:], s[i:j]
MW = "🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs"
IN = "🕹️interaction/🦀️.rs"
CG = "🕹️interaction/🖱️canvas-gesture/🦀️.rs"
GB = "🕹️interaction/🧭️gumball/🦀️.rs"
ED = "🎭️modes/✏️edit/🦀️.rs"
mw = rd(MW)
mw, scale = cut(mw, "/// 📐️ Model-meters -> screen-pixels scale", "pub(crate) const SCALE_2D: f64 = 20.0;\n")
mw, origin = cut(mw, "/// 📐️ Screen-space origin offset", "pub(crate) const ORIGIN_2D: f64 = 40.0;\n")
mw, helpers = cut(mw, "pub(crate) fn screen_2d(", "        FemElement::Bar { start, end, .. } | FemElement::Beam { start, end, .. } => (start.as_str(), end.as_str()),\n    }\n}\n")
geometry = "//#region 🔖️CanvasGeometry\n" + (scale + origin + "\n" + helpers).replace("pub(crate) ", "pub ") + "//#endregion 🔖️CanvasGeometry\n"
geometry = geometry.replace("pub fn fem2d_element_endpoints(element: &FemElement)", "pub fn fem2d_element_endpoints(element: &semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement)")
used = [n for n in ["fem2d_element_endpoints", "find_node_2d", "screen_2d", "ORIGIN_2D", "SCALE_2D"] if re.search(r"\b" + n + r"\b", mw)]
mw = mw.replace("use std::fmt::Write;\n", "use crate::editor::fem2d::interaction::{" + ", ".join(used) + "};\nuse std::fmt::Write;\n", 1)
wr(MW, mw)
inn = rd(IN)
inn = inn.replace("use crate::editor::fem2d::modes::edit::windows::model as model_window;\n", "").replace("use crate::editor::fem2d::modes::edit::windows::results as results_window;\n", "")
inn = inn.replace("use model_window::{fem2d_element_endpoints, find_node_2d, screen_2d, ORIGIN_2D, SCALE_2D};\n", "")
inn, camera = cut(inn, "//#region 🔖️WindowCamera\n", "//#endregion 🔖️WindowCamera\n")
inn = inn.replace("//#region 🧪️Tests\n", geometry + "\n//#region 🧪️Tests\n", 1)
inn = inn.replace("use semio_framework_plugin::ConfigView;\n", "").replace("use semio_framework_plugin::NoConfig;\n", "") if "ConfigView" not in inn.replace("use semio_framework_plugin::ConfigView;\n", "") else inn
wr(IN, inn)
cg = rd(CG)
cg = cg.replace("use crate::editor::fem2d::modes::edit::windows::model::{fem2d_element_endpoints, find_node_2d, screen_2d};\n", "use super::{fem2d_element_endpoints, find_node_2d, screen_2d};\n")
old = '''pub fn fem2d_gesture_window_id_for_render(view: &ViewModel, body_key: &str) -> Option<String> {
    if let Some(id) = view.window_id.clone() {
        return Some(id);
    }
    let kind = match body_key {
        crate::editor::fem2d::modes::edit::windows::model::BODY_KEY => crate::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID,
        crate::editor::fem2d::modes::edit::windows::results::BODY_KEY => crate::editor::fem2d::modes::edit::windows::results::WINDOW_KIND_ID,
        _ => return None,
    };
    view.window_instances.iter().find(|window| window.window_kind_id == kind).map(|window| window.id.clone())
}'''
assert old in cg
cg = cg.replace(old, '''pub fn fem2d_gesture_window_id_for_render(view: &ViewModel, window_kind_id: &str) -> Option<String> {
    if let Some(id) = view.window_id.clone() {
        return Some(id);
    }
    view.window_instances.iter().find(|window| window.window_kind_id == window_kind_id).map(|window| window.id.clone())
}''')
cg, body_fn = cut(cg, "pub fn fem2d_addressed_body_key(", "    }\n}\n")
for verb in ["down", "move", "up"]:
    cg = cg.replace(f'let body = fem2d_addressed_body_key(view, "fem2d.canvas-pointer-{verb}")?;\n', "")
cg = cg.replace("ui_scope: partial_window_refresh(body)", "ui_scope: partial_window_refresh(body)")
cg = re.sub(r"pub fn pointer_down\(doc: &Fem2dSnapshot, camera: &Viewport2d, view: &ViewModel,", "pub fn pointer_down(doc: &Fem2dSnapshot, camera: &Viewport2d, view: &ViewModel, body: &str,", cg)
cg = re.sub(r"pub fn pointer_move\(doc: &Fem2dSnapshot, camera: &Viewport2d, view: &ViewModel,", "pub fn pointer_move(doc: &Fem2dSnapshot, camera: &Viewport2d, view: &ViewModel, body: &str,", cg)
cg = cg.replace("pub fn pointer_up(\n    doc: &Fem2dSnapshot,\n    camera: &Viewport2d,\n    view: &ViewModel,\n", "pub fn pointer_up(\n    doc: &Fem2dSnapshot,\n    camera: &Viewport2d,\n    view: &ViewModel,\n    body: &str,\n")
wr(CG, cg)
gb = rd(GB).replace("use crate::editor::fem2d::modes::edit::windows::model::{fem2d_element_endpoints, find_node_2d, screen_2d, ORIGIN_2D, SCALE_2D};", "use crate::editor::fem2d::interaction::{fem2d_element_endpoints, find_node_2d, screen_2d, ORIGIN_2D, SCALE_2D};")
wr(GB, gb)
ed = rd(ED)
camera = camera.replace("model_window::", "windows::model::").replace("results_window::", "windows::results::")
body_fn = body_fn.replace("super::fem2d_addressed_window_kind", "fem2d_addressed_window_kind").replace("crate::editor::fem2d::modes::edit::windows::model::", "windows::model::").replace("crate::editor::fem2d::modes::edit::windows::results::", "windows::results::")
camera = camera.replace("//#endregion 🔖️WindowCamera\n", "\n/// 🪟️ The body key of the window one command is addressed to.\n" + body_fn + "//#endregion 🔖️WindowCamera\n")
ed = ed.rstrip("\n") + "\n\nuse crate::editor::fem2d::modes::edit::windows;\nuse semio_framework_os_kernel::Viewport2d;\nuse semio_framework_plugin::{ConfigView, Fault, NoConfig, ViewModel};\n\n" + camera
wr(ED, ed)
print("moved geometry", used, "camera region", len(camera))
