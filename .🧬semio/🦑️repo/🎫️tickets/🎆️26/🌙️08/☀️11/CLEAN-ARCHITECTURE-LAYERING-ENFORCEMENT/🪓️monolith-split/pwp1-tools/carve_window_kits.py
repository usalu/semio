#!/usr/bin/env python3
# 🪟️ Carve of `window-kits` (root `🪟️window-kits/🦀️.rs` + media/text/image/mesh kits + context menu + world3d sun) and `window-kits-rich` (tree, table, world3d, document).
import re, subprocess
from carvelib import *
import wp1files

K = PLUG + "/🪟️window-kits"
WK = K + "/🦀️.rs"
TREE = K + "/🌳️tree/🦀️.rs"
W3D = K + "/🌍️world3d/🦀️.rs"
RICH = K + "/🧱️rich/🦀️.rs"

# A/B. relocate the tests that name layers above the kits (companion hosts, see the test ledger) and drop their foreign cfg(test) imports
ledger = []
t = read(WK)
for name in ("app_testing", "document_kit", "table_kit", "tree_kit"):
    pat = re.compile(r"#\[cfg\(test\)\]\nuse crate::" + name + r"::[A-Za-z0-9_]+;\n")
    n = len(pat.findall(t))
    t = pat.sub("", t)
    ledger.append(("window_kits", name, n))
inc = '#[cfg(test)]\ninclude!("../🧪️tests/🔬️app-window-kits/🦀️.rs");\n'
assert t.count(inc) == 1
t = t.replace(inc, "")
write(WK, t)
tt = read(TREE)
for name in ("history_view", "command_history"):
    pat = re.compile(r"#\[cfg\(test\)\]\nuse crate::" + name + r"::[A-Za-z0-9_]+;\n")
    ledger.append(("tree_kit", name, len(pat.findall(tt))))
    tt = pat.sub("", tt)
inc = '#[cfg(test)]\ninclude!("../../🧪️tests/🔬️app-panel-kit/🦀️.rs");\n'
assert tt.count(inc) == 1
write(TREE, tt.replace(inc, ""))
print("stripped foreign cfg(test) imports:", ledger)

# C. world3d: the sun config and the scene constructor go down to window-kits
w = read(W3D)
a = w.index("/** 🌞️ Plugin-owned directional-light state")
b = w.index("/** 🌳️ Measure group with optional open state")
sun_block = w[a:b]
c = w.index("pub fn world3d_scene(")
d = w.index("pub fn world3d_default_camera()")
scene_fn = w[c:d]
assert "WorldSunConfig" in sun_block and "world3d_environment_json" in sun_block
new_dir = K + "/🌍️world3d/☀️sun"
assert not os.path.exists(new_dir)
head = '''//! 🌞️ World-3d sun state and scene constructor shared down to the base window kits (the mesh kit renders through them).
use semio_framework_ui_scene::World3dScene;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
'''
write(new_dir + "/🦀️.rs", head + sun_block + scene_fn)
w = w[:a] + "pub use semio_framework_plugin_window_kits::world3d_sun::{world3d_environment_json, world3d_scene, WorldSunConfig};\n" + w[b:c] + w[d:]
write(W3D, w)

# D. paths
for crate in ("window-kits", "window-kits-rich"):
    subprocess.check_call([sys.executable, TOOLS + "/rewrite_paths.py", crate, "--apply"], stdout=subprocess.DEVNULL)

# E. roots
t = read(WK)
t = t.replace("pub mod context_menu;\n", "pub mod context_menu;\n", 1)
t = t.rstrip("\n") + '''

#[path = "🎬️media/🦀️.rs"]
pub mod media_kit;
#[path = "📝️text/🦀️.rs"]
pub mod text_kit;
#[path = "🖼️image/🦀️.rs"]
pub mod image_kit;
#[path = "🧊️mesh/🦀️.rs"]
pub mod mesh_kit;
#[path = "🌍️world3d/☀️sun/🦀️.rs"]
pub mod world3d_sun;
'''
write(WK, t)
ensure_externs(WK, aliases_used([f for f, _ in wp1files.tree(WK)] + [new_dir + "/🦀️.rs"]))

r = '''//! 🧱️ Tree, table, world3d and document window kits of the plugin SDK.
extern crate semio_framework_os_kernel as protocol;

#[path = "../🌳️tree/🦀️.rs"]
pub mod tree_kit;
#[path = "../📊️table/🦀️.rs"]
pub mod table_kit;
#[path = "../🌍️world3d/🦀️.rs"]
pub mod world3d_host;
#[path = "../📃️document/🦀️.rs"]
pub mod document_kit;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
'''
write(RICH, r)
ensure_externs(RICH, aliases_used([f for f, _ in wp1files.tree(RICH)]))

hub_edit('''#[path = "../../🪟️window-kits/📃️document/🦀️.rs"]
pub(crate) mod document_kit;
''', '''pub(crate) use semio_framework_plugin_window_kits_rich::document_kit;
''')
hub_edit('''#[path = "../../🪟️window-kits/🖼️image/🦀️.rs"]
pub(crate) mod image_kit;
''', '''pub(crate) use semio_framework_plugin_window_kits::image_kit;
''')
hub_edit('''#[path = "../../🪟️window-kits/🎬️media/🦀️.rs"]
pub(crate) mod media_kit;
''', '''pub(crate) use semio_framework_plugin_window_kits::media_kit;
''')
hub_edit('''#[path = "../../🪟️window-kits/🧊️mesh/🦀️.rs"]
pub(crate) mod mesh_kit;
''', '''pub(crate) use semio_framework_plugin_window_kits::mesh_kit;
''')
hub_edit('''#[path = "../../🪟️window-kits/📊️table/🦀️.rs"]
pub(crate) mod table_kit;
''', '''pub(crate) use semio_framework_plugin_window_kits_rich::table_kit;
''')
hub_edit('''#[path = "../../🪟️window-kits/📝️text/🦀️.rs"]
pub(crate) mod text_kit;
''', '''pub(crate) use semio_framework_plugin_window_kits::text_kit;
''')
hub_edit('''#[path = "../../🪟️window-kits/🌳️tree/🦀️.rs"]
pub(crate) mod tree_kit;
''', '''pub(crate) use semio_framework_plugin_window_kits_rich::tree_kit;
''')
hub_edit('''#[path = "../../🪟️window-kits/🦀️.rs"]
pub(crate) mod window_kits;
''', '''pub(crate) extern crate semio_framework_plugin_window_kits as window_kits;
''')
hub_edit('''#[path = "../../🪟️window-kits/🌍️world3d/🦀️.rs"]
pub mod world3d_host;
''', '''pub use semio_framework_plugin_window_kits_rich::world3d_host;
''')
set_lib_path("🪟️window-kits")
remove_stub("🪟️window-kits")
mark_carved("window-kits")
mark_carved("window-kits-rich")
