import json, os, re, sys, collections
sys.path.insert(0, 'lib')
import carve, rs, reg, crategen, peel
from carve import R, UI, FW, PKG, rd, wr, remove
spec = carve.Spec('specs/wgpu.json')
W = UI + '/🎯️targets/🧊️wgpu'
DOC = {'light': ('🪶️', 'wgpu light tier: minimap geometry, tree window request rule, presence bar and tabs builders.'),
       'layout': ('🧱️', 'wgpu layout stack: chrome metrics, layout rules and the flex solver.'),
       'controls': ('🔘️', 'wgpu control renderers: select, slider, stepper, input, button, toggle, key value, ring and icon selector.'),
       'hosting': ('🧭️', 'wgpu hosting: accessibility projection, cursors, shell, window host and scene slots.')}
# 1. cfg fix in the retained hit module
p = W + '/📌️mounted_layout/🎯️hit/🦀️.rs'
s = rd(p)
s = s.replace('#[cfg(all(test, feature = "wgpu-engine"))]', '#[cfg(test)]')
wr(p, s)
# 2. mounts
def add_mount(path, block):
    s = rd(path)
    if block.strip() in s: return
    us = peel.header_uses(s)
    at = max(u.end for u in us) if us else 0
    s = s[:at] + '\n\n' + block.strip('\n') + '\n' + s[at:]
    wr(path, s)
add_mount(W + '/📌️mounted_layout/🦀️.rs', '#[path = "🎯️hit/🦀️.rs"]\npub mod hit;')
add_mount(W + '/🪀️widgets/🦀️.rs', '#[path = "../../../🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs"]\npub mod tree_element;')
# 3. group roots
for k, c in spec.crates.items():
    if not c.get('mounts'): continue
    emoji, doc = DOC[k]
    wr(spec.root_file(c), crategen.root_text(spec, c, emoji, doc))
# 4. save + remove the old root
oldroot = W + '/🦀️.rs'
if os.path.exists(oldroot):
    txt = rd(oldroot)
    tmounts = [l for l in txt.split('\n')]
    open('lists/wgpu-old-root.rs.txt', 'w', encoding='utf8').write(txt)
# 5. rewrite
stats = collections.Counter()
owner = {}
for k, c in spec.crates.items():
    for f in crategen.crate_files(spec, c):
        if f in owner and owner[f] != k: print('OVERLAP', f, owner[f], k)
        owner.setdefault(f, k)
for f, k in owner.items():
    if not f.endswith('.rs') or not os.path.exists(f): continue
    c = spec.crates[k]
    if f == spec.root_file(c) and c.get('mounts'): continue
    old = rd(f); st = {}
    new = carve.rewrite_text(old, spec, k, st)
    if new != old:
        wr(f, new); stats[k] += 1
    for a, b in st.items(): stats[a] += b
print(dict(stats))
remove(oldroot)
print('old wgpu root removed; files owned:', len(owner))
json.dump({os.path.relpath(f, R): k for f, k in owner.items()}, open('lists/wgpu-owner.json', 'w'), indent=0, ensure_ascii=False)
