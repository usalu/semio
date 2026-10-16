"""🔎 Per scan test: the carved source files it needs (positive literals present in the original source)."""
import os, re, json, sys
from lib import *
files = {}
for root, ds, fs in os.walk(STAGE):
    for f in fs:
        if f.endswith('.rs'):
            p = os.path.join(root, f); files[os.path.relpath(p, STAGE)] = open(p, encoding='utf-8').read()
prist = {}
for root, ds, fs in os.walk(PRISTINE):
    for f in fs:
        if f.endswith('.rs'):
            p = os.path.join(root, f); prist[os.path.relpath(p, PRISTINE)] = open(p, encoding='utf-8').read()
def unesc(s):
    try: return bytes(s, 'utf-8').decode('unicode_escape').encode('latin-1').decode('utf-8')
    except Exception: return s
def literals(body):
    """Positive source-text literals of a test body: (literal, polarity_positive)."""
    out = []
    for m in re.finditer(r'"((?:[^"\\]|\\.)*)"', body):
        s = unesc(m.group(1))
        if len(s) <= 5 or s.startswith('../'):
            continue
        before = body[max(0, m.start() - 160):m.start()]
        k = max(before.rfind('assert!('), before.rfind('assert_eq!('))
        neg = k >= 0 and before[k:].startswith('assert!(!')
        out.append((s, not neg))
    return out
from model import parse_items
def fn_bodies(text):
    items, trail, lines = parse_items(text)
    for it in items:
        if it.kind == 'fn' and it.name:
            yield it.name, '\n'.join(lines[it.start:it.end + 1])
res = {}
for rel, text in files.items():
    if 'include_str!' not in text: continue
    for name, body in fn_bodies(text):
        incs = re.findall(r'include_str!\("([^"]*🦀️\.rs)"\)', body)
        if 'retained_submit_source()' in body:
            incs = incs + ['../../🦀️.rs' if rel.startswith('⚙️engine/🧪️') else '../../../🦀️.rs']
        if not incs: continue
        inc_rel = [os.path.normpath(os.path.join(os.path.dirname(rel), i)) for i in incs]
        pu = '\n'.join(prist.get(i, '') for i in inc_rel)
        su = '\n'.join(files.get(i, '') for i in inc_rel)
        need = {}
        lits = [l for l, positive in literals(body) if positive and l in pu]
        unsat = [l for l in lits if l not in su]
        cover = []
        pool = {k: t for k, t in files.items() if '🧪️tests/' not in k}
        remaining = set(unsat)
        while remaining:
            best = max(pool, key=lambda k: (sum(1 for l in remaining if l in pool[k]), -len(pool[k])))
            got = {l for l in remaining if l in pool[best]}
            if not got: break
            cover.append(best); remaining -= got
        for l in lits:
            if l in su:
                for i in inc_rel:
                    if l in files.get(i, ''): need.setdefault(i, 0)
        for c in cover: need.setdefault(c, 0)
        if remaining: need['UNCOVERED:' + '|'.join(sorted(remaining))[:60]] = 0
        res[(rel, name)] = (inc_rel, sorted(need))
tot = {}
for (rel, name), (incs, need) in sorted(res.items()):
    sz = sum(len(files[n].encode()) for n in need if n in files)
    print(f'{rel.replace("/🧪️tests/🔬️unit/🦀️.rs","")[:40]:40} {name[:70]:70} {sz:>7}', [n.replace('/🦀️.rs','') for n in need])
json.dump({f'{k[0]}::{k[1]}': v for k, v in res.items()}, open(SP + '/srcscan.json', 'w'), ensure_ascii=False, indent=1)
