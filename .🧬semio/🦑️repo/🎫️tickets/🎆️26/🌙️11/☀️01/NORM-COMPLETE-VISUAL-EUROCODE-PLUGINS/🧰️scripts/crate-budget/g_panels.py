"""🪚 Crate-budget G: the inspection half of the app-surface panels becomes `semio-s-plugin-norm-app-surface-panels-inspection` (root `🔎️inspection`); panels depends on it and re-exports it."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *
AS = NORM + "/📇️registry/🧬️contract/🖥️app-surface"
OLD, NEW = "semio-s-plugin-norm-app-surface-panels", "semio-s-plugin-norm-app-surface-panels-inspection"
old_toml = AS + "/🧾️panels/📦️packages/🦀️rust/Cargo.toml"
new_toml = AS + "/🧾️panels/🔎️inspection/📦️packages/🦀️rust/Cargo.toml"
t = rd(old_toml)
if not os.path.exists(new_toml):
    n = t.replace('name = "%s"' % OLD, 'name = "%s"' % NEW).replace('workspace = "../../../../../.."', 'workspace = "../../../../../../.."')
    n = re.sub(r'description = "[^"]+"', 'description = "Norm app surface inspection panels: checks, properties, outliner, selection mirror, catalogue, check parameters, bulk edit, host routes, view steps, shell hooks and shortcut dispatch"', n)
    n = re.sub(r'path = "((?:\.\./)+)', lambda m: 'path = "../' + m.group(1), n)
    wr(new_toml, n)
x = rd(old_toml)
if NEW not in x:
    x = x.replace("[dependencies]\n", '[dependencies]\n%s = { path = "../../🔎️inspection/📦️packages/🦀️rust" }\n' % NEW, 1)
    wr(old_toml, x)
ws = NORM + "/Cargo.toml"
s = rd(ws)
rel = "📇️registry/🧬️contract/🖥️app-surface/🧾️panels/🔎️inspection/📦️packages/🦀️rust"
if rel not in s:
    a = '    "📇️registry/🧬️contract/🖥️app-surface/🧾️panels/📦️packages/🦀️rust",\n'
    assert a in s
    wr(ws, s.replace(a, a + '    "%s",\n' % rel, 1))
sc = NORM + "/📇️registry/🧬️contract/📦️packages/🦀️rust/📜️script.ts"
c = rd(sc)
if '"%s"' % NEW not in c:
    a = '"%s"' % OLD
    assert a in c
    wr(sc, c.replace(a, a + ',"%s"' % NEW, 1))
lock_register(OLD, NEW, extra_old_deps=[NEW])
