import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/rg-rest-tools")
from delegate import *
I = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance"
OLD = "semio-framework-plugin-instance-app"
NEW = "semio-framework-plugin-instance-surface"
SURF = I + "/🛬️lifted/🌳️tree"
lf = SURF + "/🦀️.rs"
t = rd(lf)
assert "use crate::VcsArtifactApp;\n" in t
t = t.replace("use crate::VcsArtifactApp;\n", "use semio_framework_plugin_instance_app::VcsArtifactApp;\n", 1)
t = t.replace("//! 🌳️ Lift file: 4 item(s), about 63 lines, of `📜️declaration/🌳️tree/🦀️.rs` that belong to crate `instance-app` (WP7).", "//! 🌳️ The editor and viewer surface declarations of the instance app: 4 item(s), about 63 lines, lifted out of `📜️declaration/🌳️tree/🦀️.rs`, held above `instance-app` so the app crate stays inside the source gate.", 1)
wr(lf, t)
npkg = register_crate("Cargo.toml", OLD, I + "/📦️packages/🦀️rust", NEW, SURF, "Editor and viewer surface declarations of the instance app", "../../🦀️.rs")
mt = rd(npkg + "/Cargo.toml")
mt = mt.replace('id = "plugin-instance-app"', 'id = "plugin-instance-surface"')
mt = re.sub(r'(?ms)^\[features\]\n.*?(?=^\[|\Z)', '', mt)
mt = re.sub(r'(?ms)^\[dev-dependencies\]\n.*?(?=^\[|\Z)', '', mt)
wr(npkg + "/Cargo.toml", mt.rstrip("\n") + "\n")
add_dep_line(npkg + "/Cargo.toml", "dependencies", OLD, '%s = { workspace = true }' % OLD)
root = rd(I + "/🦀️.rs")
blk = '#[path = "🛬️lifted/🌳️tree/🦀️.rs"]\npub mod lifted_tree;\npub use lifted_tree::*;\n'
assert blk in root
wr(I + "/🦀️.rs", root.replace(blk, "", 1))
# consumers
subs = [
 ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/📜️wp2-declaration/🦀️.rs", "pub use semio_framework_plugin_instance_app::{editor_surface, viewer_surface, EditorSurfaceApp, ViewerSurfaceApp};", "pub use semio_framework_plugin_instance_surface::{editor_surface, viewer_surface, EditorSurfaceApp, ViewerSurfaceApp};"),
 ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/⚠️refusal/🦀️.rs", "use semio_framework_plugin_instance_app::lifted_tree::editor_surface;", "use semio_framework_plugin_instance_surface::editor_surface;"),
 ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/⚠️refusal/🦀️.rs", "use semio_framework_plugin_instance_app::lifted_tree::viewer_surface;", "use semio_framework_plugin_instance_surface::viewer_surface;"),
 ("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/📜️wp2-declaration/🧪️hosts/🔬️declarations-fixture/🦀️.rs", "use semio_framework_plugin_instance_app::lifted_tree::{editor_surface, viewer_surface}", "use semio_framework_plugin_instance_surface::{editor_surface, viewer_surface}"),
]
for f, a, b in subs:
    x = rd(f)
    assert a in x, (f, a)
    wr(f, x.replace(a, b, 1))
# manifests of the consumers
import subprocess
mfs = subprocess.run(["git", "grep", "-l", "-e", OLD, "--", "*Cargo.toml"], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
mfs = [m for m in mfs if m.endswith("Cargo.toml") and m not in (I + "/📦️packages/🦀️rust/Cargo.toml", npkg + "/Cargo.toml", "Cargo.toml")]
for mf in mfs:
    txt = closure_text(rgp.closure(mf))
    if re.search(r'\bsemio_framework_plugin_instance_surface\b', txt):
        add_dep_like(mf, OLD, NEW, npkg)
        print("dep ->", mf[-80:])
print(run_owners("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin", True))
