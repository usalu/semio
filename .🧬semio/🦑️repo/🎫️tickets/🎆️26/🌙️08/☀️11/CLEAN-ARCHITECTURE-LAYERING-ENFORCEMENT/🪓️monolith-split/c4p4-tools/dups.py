import os, re, sys, importlib, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pfa
from rsitems import items as rs_items, name_kind

def names(text):
    out = collections.Counter()
    for it in rs_items(text):
        try:
            v, k, nm = name_kind(it['toks'])
        except Exception:
            continue
        if k in ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'mod', 'union', 'macro') and nm:
            out[(k if k != 'macro' else 'macro', nm)] += 1
    return out

bad = 0
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    root = os.path.join(pfa.SCR, 'stage', tag)
    seen = {}
    files = {'__parent__': os.path.join(root, '🦀️.rs')}
    for c in sp['children']:
        files[c['folder']] = os.path.join(root, c['folder'], '🦀️.rs')
    for f, p in files.items():
        for key, n in names(open(p, encoding='utf-8').read()).items():
            seen.setdefault(key, []).append((f, n))
    for key, rows in sorted(seen.items()):
        if len(rows) > 1 or rows[0][1] > 1:
            kinds = {r[0] for r in rows}
            print(tag, 'DUP', key, rows)
            bad += 1
print('DUPS', bad)
