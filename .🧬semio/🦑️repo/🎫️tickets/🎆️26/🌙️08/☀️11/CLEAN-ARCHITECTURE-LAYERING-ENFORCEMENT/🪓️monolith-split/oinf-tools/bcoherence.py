"""Orphan/inherent-impl coherence check of the generated world crates."""
import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wgen import chunk_file
from bcrates import CRATES

def check(root):
    defs = {}      # type/trait name -> crate id
    kinds = {}
    texts = {}
    for cid, bands, folder in CRATES:
        paths = [f"{root}/{folder}/🦀️.rs"]
        for sub in ('🧰️testkit', '⏯️tool-run-trace'):
            p = f"{root}/{folder}/{sub}/🦀️.rs"
            if os.path.exists(p): paths.append(p)
        for p in paths:
            text = open(p, encoding='utf-8').read()
            items, _ = chunk_file(text)
            texts[(cid, p)] = items
            for it in items:
                if it.kind in ('struct','enum','union','type','trait') and it.name: defs[it.name] = cid; kinds[it.name] = it.kind
    bad = []
    for (cid, p), items in texts.items():
        for it in items:
            if it.kind == 'macro_call':
                tys = [t for t in it.plain if t in defs]
                for t in tys:
                    if defs[t] != cid: bad.append((cid, 'macro', it.name, t, defs[t]))
            if it.kind != 'impl': continue
            ty = it.impl_ty; tr = it.impl_trait
            if ty in defs:
                if tr is None:
                    if defs[ty] != cid: bad.append((cid, 'inherent', ty, '-', defs[ty]))
                else:
                    if defs[ty] != cid and not (tr in defs and defs[tr] == cid): bad.append((cid, 'trait-impl', f'{tr} for {ty}', '-', defs[ty]))
            else:
                if tr is None: bad.append((cid, 'inherent-external', ty, '-', '-'))
                elif not (tr in defs and defs[tr] == cid): bad.append((cid, 'orphan?', f'{tr} for {ty}', '-', '-'))
    return bad

if __name__ == '__main__':
    bad = check(sys.argv[1])
    print('coherence findings:', len(bad))
    for b in bad[:60]: print(b)
