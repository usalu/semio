# 🧭️ Flip checklist data: per WP4 crate, the monolith paths its files name (`crate::<module>::…`, `super::…`, `use super::*`) grouped by the crate that will own the target.
import json, re, collections, os
from common import *
m = json.load(open(GEN + '/pwp4-crates4.json'))
OWN = m['own']
def owner(rel):
    best = None
    for c, ps in OWN.items():
        for p in ps:
            if (p.endswith('/') and rel.startswith(p)) or rel == p:
                if not best or len(p) > best[1]: best = (c, len(p))
    return best[0] if best else None
# module name -> crate through the hub units of Appendix I and the root's pub mod / pub use lists
hub_units = {}
for hub in os.listdir(PLUG + '/🪝️mounts'):
    t = open(PLUG + '/🪝️mounts/' + hub + '/🦀️.rs', encoding='utf-8').read()
    for mm in re.finditer(r'#\[path\s*=\s*"\.\./\.\./([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)', t):
        hub_units[mm.group(2)] = mm.group(1)
def crate_of_module(name):
    f = hub_units.get(name)
    if f is None: return None
    c = PT.file_crate(f)
    return c
files = {}
for dp, dn, fn in os.walk(PLUG + '/⚛️reactor'):
    dn[:] = [d for d in dn if d != 'node_modules' and d != '📦️packages']
    for f in fn:
        if f.endswith('.rs'):
            p = os.path.join(dp, f); rel = os.path.relpath(p, PLUG)
            if '/🧪️tests/' in rel or '🧪️tests' in rel.split('/')[-2:]: continue
            c = owner(rel)
            if c: files.setdefault(c, []).append(p)
for p in [PLUG + '/🚪️lifetime/🦀️.rs']: files.setdefault('reactor-lifecycle', []).append(p)
out = {}
for c, fl in sorted(files.items()):
    cnt = collections.Counter(); sup = 0; glob = 0
    for p in fl:
        t = PT.strip(open(p, encoding='utf-8').read())
        for mm in re.finditer(r'\bcrate::([A-Za-z_]\w*)', t): cnt[mm.group(1)] += 1
        sup += len(re.findall(r'\bsuper::', t)); glob += len(re.findall(r'use super::\*', t))
    byc = collections.Counter()
    for mod, n in cnt.items():
        tgt = crate_of_module(mod) or ('facade-root' if mod[0].isupper() or mod in ('component_persistent_local', 'encode_fault_bytes') else 'unmapped:' + mod)
        byc[tgt] += n
    out[c] = dict(files=len(fl), crate_paths=sum(cnt.values()), super_paths=sup, glob_imports=glob, by_target_crate=dict(byc.most_common(12)))
    print(f"{c:20s} files {len(fl):2d} crate:: {sum(cnt.values()):4d} super:: {sup:4d} use super::* {glob:2d} | " + ', '.join(f"{k} {v}" for k, v in byc.most_common(8)))
json.dump(out, open(GEN + '/pwp4-flipmap.json', 'w'), ensure_ascii=False, indent=1)
