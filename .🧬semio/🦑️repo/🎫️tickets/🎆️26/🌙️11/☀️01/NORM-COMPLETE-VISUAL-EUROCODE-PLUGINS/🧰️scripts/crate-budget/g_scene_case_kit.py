"""🪚 Crate-budget G: register the shared crate `semio-s-plugin-norm-structural-scene-case-kit` (members, script list, dependency, lock edges)."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *
OLD, NEW = "semio-s-plugin-norm-structural-scene", "semio-s-plugin-norm-structural-scene-case-kit"
REL = "🎬️structural-scene/🧰️projection-kit/🧱️case-kit/📦️packages/🦀️rust"
ws = NORM + "/Cargo.toml"
s = rd(ws)
if '"%s"' % REL not in s:
    anchor = '    "🎬️structural-scene/🧊️meshing/📦️packages/🦀️rust",\n'
    assert anchor in s
    wr(ws, s.replace(anchor, anchor + '    "%s",\n' % REL, 1))
sc = NORM + "/📇️registry/🧬️contract/📦️packages/🦀️rust/📜️script.ts"
t = rd(sc)
if '"%s"' % NEW not in t:
    a = '"semio-s-plugin-norm-structural-scene-meshing"'
    assert a in t
    wr(sc, t.replace(a, a + ',"%s"' % NEW, 1))
add_dep(NORM + "/🎬️structural-scene/📦️packages/🦀️rust/Cargo.toml", NEW)
tm = NORM + "/🎬️structural-scene/📦️packages/🦀️rust/Cargo.toml"
x = rd(tm)
if NEW + " =" in x and "case-kit = { workspace = true }" in x:
    wr(tm, x.replace(NEW + " = { workspace = true }", NEW + ' = { path = "../../🧰️projection-kit/🧱️case-kit/📦️packages/🦀️rust" }'))
lock_register(OLD, NEW, extra_old_deps=[NEW], new_deps=["semio-s-plugin-norm-compliance", "semio-s-plugin-norm-structural-scene-foundation", "serde_json"])
