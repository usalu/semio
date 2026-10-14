#!/usr/bin/env python3
"""🕹️ Moves the fem3d editor-root items that lower editor layers name down to their owners (stage 2 of the P5 fem3d
cut, as the fem2d stage did): the app id, controller id, `ui_label`, `fem3d_action` and `fem3d_measure_action` to the
terminology module (region 🎛️Controller), `results_window_instance_id` to the results window, `reset_document_effect`
to io; every reference is relocated through `🚚️relocate-root.py`.
"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
A = REPO + "/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
ANY = A + "/🏅️standards/🔖️1/🪆️subsets/🌐️any"
E = ANY + "/✏️editor"
rd = lambda p: open(p, encoding="utf-8").read()
def wr(p, s): open(p, "w", encoding="utf-8").write(s)
def cut(s, a, b):
    i = s.index(a); j = s.index(b, i) + len(b)
    return s[:i] + s[j:], s[i:j]
def fn(s, sig):
    m = re.search(r"(?:///[^\n]*\n)*" + re.escape(sig), s); e = s.index("\n}\n", m.start()) + 3
    return s[:m.start()] + s[e:], s[m.start():e]
root = rd(E + "/🦀️.rs")
root, consts = cut(root, "//#region 🔖️Constants\n", "//#endregion 🔖️Constants\n")
root, label = fn(root, "pub fn ui_label(")
root, action = fn(root, "pub fn fem3d_action(")
root, measure = fn(root, "pub fn fem3d_measure_action(")
root, rwid = fn(root, "pub fn results_window_instance_id(")
root, reset = cut(root, "//#region 🔖️ResetDocument\n", "//#endregion 🔖️ResetDocument\n")
wr(E + "/🦀️.rs", root)
consts = consts.replace("//#region 🔖️Constants\n", "").replace("//#endregion 🔖️Constants\n", "").replace("pub const FEM3D_APP_ID", "/// 🪪️ The fem3d play app id.\npub const FEM3D_APP_ID")
label = label.replace("map_err(|_| PluginAssemblyError::new(", "map_err(|_| semio_framework_plugin::PluginAssemblyError::new(")
measure = measure.replace("args: Option<Value>) -> ActionDescriptor {\n    ActionDescriptor {", "args: Option<semio_framework_pack_json::Value>) -> semio_framework_plugin::ActionDescriptor {\n    semio_framework_plugin::ActionDescriptor {")
t = rd(E + "/🗣️terminology/🦀️.rs").rstrip() + "\n\n//#region 🎛️Controller\n" + consts + "\n" + label + "\n" + action + "\n" + measure + "//#endregion 🎛️Controller\n"
wr(E + "/🗣️terminology/🦀️.rs", t)
rw = E + "/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs"
rwid = rwid.replace("window_results::FEM3D_WINDOW_RESULTS", "FEM3D_WINDOW_RESULTS").replace("(view_state: &ViewModel)", "(view_state: &semio_framework_plugin::ViewModel)")
wr(rw, rd(rw).rstrip() + "\n\n//#region 🪟️InstanceLookup\n" + rwid + "//#endregion 🪟️InstanceLookup\n")
io = ANY + "/🚪️io/🦀️.rs"
reset = reset.replace("//#region 🔖️ResetDocument", "//#region 🌱️ResetDocument").replace("//#endregion 🔖️ResetDocument", "//#endregion 🌱️ResetDocument")
reset = reset.replace("(scene: &Fem3dSnapshot)", "(scene: &crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot)").replace("<Fem3dSnapshot as pack", "<crate::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot as pack").replace("semio_s_artifact_fem_3d_model::", "semio_s_artifact_fem_3d_model::")
wr(io, rd(io).rstrip() + "\n\n" + reset)
T = "crate::editor::fem3d::terminology"
m = {"FEM3D_APP_ID": T, "FEM3D_PLAY_CONTROLLER_ID": T, "ui_label": T, "fem3d_action": T, "fem3d_measure_action": T,
     "results_window_instance_id": "crate::editor::fem3d::modes::edit::windows::results", "reset_document_effect": "semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io"}
json.dump(m, open(HERE + "/🚚️fem3d-editor-root-map.json", "w"), ensure_ascii=False, indent=1)
files = []
for r, ds, fs in os.walk(A):
    files += [os.path.join(r, f) for f in fs if f.endswith(".rs")]
for r, ds, fs in os.walk(REPO + "/🌎️hub/🧩️compositions/🏗️fem"):
    files += [os.path.join(r, f) for f in fs if f.endswith(".rs")]
subprocess.run([sys.executable, HERE + "/🚚️relocate-root.py", HERE + "/🚚️fem3d-editor-root-map.json", "crate::editor::fem3d"] + files, check=True, stdout=subprocess.DEVNULL)
subprocess.run([sys.executable, HERE + "/🚚️relocate-root.py", HERE + "/🚚️fem3d-editor-root-map.json", "semio_s_artifact_fem_3d::editor::fem3d"] + files, check=True, stdout=subprocess.DEVNULL)
print("moved")
