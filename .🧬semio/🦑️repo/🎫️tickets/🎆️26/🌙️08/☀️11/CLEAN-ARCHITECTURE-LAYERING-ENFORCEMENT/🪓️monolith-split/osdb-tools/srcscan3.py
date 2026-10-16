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
        if not incs: continue
        inc_rel = [os.path.normpath(os.path.join(os.path.dirname(rel), i)) for i in incs]
        pu = '\n'.join(prist.get(i, '') for i in inc_rel)
        su = '\n'.join(files.get(i, '') for i in inc_rel)
        need = {}
        for l, positive in literals(body):
            if not positive or l not in pu: continue
            if l in su:
                for i in inc_rel:
                    if l in files.get(i, ''): need.setdefault(i, 0)
                continue
            hold = [k for k, t in files.items() if l in t and '🧪️tests/' not in k]
            fam = {i.split('/')[0] for i in inc_rel}
            hold2 = [h for h in hold if h.split('/')[0] in fam] or hold
            for h in hold2[:2]: need.setdefault(h, 0)
        res[(rel, name)] = (inc_rel, sorted(need))
tot = {}
for (rel, name), (incs, need) in sorted(res.items()):
    sz = sum(len(files[n].encode()) for n in need)
    print(f'{rel.replace("/🧪️tests/🔬️unit/🦀️.rs","")[:40]:40} {name[:70]:70} {sz:>7}', [n.replace('/🦀️.rs','') for n in need])
json.dump({f'{k[0]}::{k[1]}': v for k, v in res.items()}, open(SP + '/srcscan.json', 'w'), ensure_ascii=False, indent=1)
