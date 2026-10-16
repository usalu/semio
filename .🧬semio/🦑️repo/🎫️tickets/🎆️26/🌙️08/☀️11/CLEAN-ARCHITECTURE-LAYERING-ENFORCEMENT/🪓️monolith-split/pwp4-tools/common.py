# 🧰️ WP4 shared helpers: paths, item outline (reuses pwp0-tools/plan_tools.py), file lists.
import os, re, sys, json, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../pwp0-tools')
os.environ.setdefault('CLOSURE', HERE + '/../🗑️generated/pwp4-closure.json')
import plan_tools as PT
ROOT = '/Users/ueli/Documents/semio/'
P = PT.P
PLUG = ROOT + P
GEN = os.path.abspath(HERE + '/../🗑️generated')
SCR = '/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/pwp4'
def rd(p): return open(p, encoding='utf-8').read()
def wr(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, 'w', encoding='utf-8').write(t)

_outline = PT.outline
def _safe_outline(path):
    if not os.path.exists(path): return [], [], []
    return _outline(path)
PT.outline = _safe_outline
PT.MAP.setdefault('📣️macros/', 'TOP-root'); PT.MAP.setdefault('🪝️mounts/', 'TOP-root'); PT.MAP.setdefault('🧷️companions/', 'TOP-root')
R_ = '⚛️reactor/'
PT.MAP.update({
 R_ + '⏲️tasks/': 'reactor-tasks', R_ + '🏃️runtime/📦️instances/': 'runtime-instances', R_ + '🏃️runtime/🧰️ops/': 'runtime-ops', R_ + '🏃️runtime/🎞️command/': 'runtime-ops',
 R_ + '🏃️runtime/🔄️exchange/': 'runtime-exchange', R_ + '🏃️runtime/📥️staging/♻️reactor-lifecycle/': 'reactor-lifecycle', R_ + '🏃️runtime/📥️staging/🔌️reactor-app/': 'reactor-app',
 R_ + '🧬️wit-bridge/': 'reactor-wit-bridge', R_ + '🧪️tests/🔬️job-render-binding/': 'reactor-tasks', R_ + '🔄️turn/🧪️tests/': 'tests-07',
})
if 'reactor-tasks' not in PT.ORDER:
    PT.ORDER.insert(PT.ORDER.index('reactor-app') + 1, 'reactor-tasks')
PT.SUBSYSTEM['reactor-tasks'] = 'R'
_orig_item_crate = PT.item_crate
def _item_crate(f, s):
    if f == PT.RT: return 'TOP-root'
    if f == PT.RC: return 'reactor-core'
    return _orig_item_crate(f, s)
PT.item_crate = _item_crate
_orig_file_crate = PT.file_crate
def _file_crate(f):
    c = _orig_file_crate(f)
    if c.startswith('UNMAPPED'):
        g = f.replace('/🛬️lifted/', '/')
        c2 = _orig_file_crate(g)
        return c2 if not c2.startswith('UNMAPPED') else 'TOP-root'
    return c
PT.file_crate = _file_crate
_ic2 = PT.item_crate
PT.item_crate = lambda f, s: _file_crate(f) if _ic2(f, s).startswith('UNMAPPED') else _ic2(f, s)
PT.MAP['🚪️lifetime/'] = 'reactor-lifecycle'
