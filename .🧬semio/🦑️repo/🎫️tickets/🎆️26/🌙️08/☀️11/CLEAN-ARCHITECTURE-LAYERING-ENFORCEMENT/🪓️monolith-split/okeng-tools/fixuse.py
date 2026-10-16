# 🧹 Rewrites `use` leaves of carved crate files whose head names a kernel-root shim module or alias (`use snapshot_read::X;`, `use ledger::Y;`): the leaf is resolved in the kernel root namespace and replaced by the extern crate path it re-exports. usage: fixuse.py <dump.json> dry|write
import json, os, sys, re, collections
OKD = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, OKD + '/../okwp56-tools'); sys.path.insert(0, OKD + '/../okwp0-tools'); sys.path.insert(0, OKD + '/../okwp23-tools'); sys.path.insert(0, OKD)
import a2lib
from a2lib import *
import okusetree as U
from okrs import items, tokenize
ROOT = ns.S + '/🦀️.rs'
dump = json.load(open(sys.argv[1])); write = sys.argv[2] == 'write'
a2lib.register_all()
w = ns.world()
root = w.files[ROOT]
reloc = set(dump.get('relocated', []))
SKIP_HEADS = {'crate', 'self', 'super', 'std', 'core', 'alloc'}
stats = collections.Counter(); unresolved = []

def local_names(text):
    out = set()
    for it in items(text):
        if it['kind'] == 'mod' and it['name']: out.add(it['name'])
        elif it['kind'] in ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'union', 'macro_rules') and it['name']: out.add(it['name'])
        elif it['kind'] == 'use':
            body = re.search(r'\buse\b(.*);', ''.join(t[1] for t in tokenize(it['text']) if t[0] != 'comment'), re.S)
            if body:
                for leaf in U.parse(body.group(1)):
                    nm = U.local_name(leaf)
                    if nm: out.add(nm)
    return out

def new_leaf(path, alias, glob, label):
    head = path[0]
    r0 = ns.lookup(root, head)
    if r0 is None: return None
    rr = ns.resolve(root, list(path))
    if rr.kind == 'extern': newp = list(rr.extern)
    elif rr.kind == ns.EXTERN_GLOB:
        cand = ns.disambiguate(rr)
        if not cand: unresolved.append((label, '::'.join(path), 'extern-glob')); return None
        newp = list(cand[0])
    else:
        unresolved.append((label, '::'.join(path), rr.kind)); return None
    local = alias or (path[-1] if path[-1] != 'self' else path[-2])
    if glob: return (newp, None, True)
    return (newp, alias if alias else (None if newp[-1] == local else local), False)

def fix(text, label):
    its = items(text); edits = []
    loc = local_names(text)
    for it in its:
        if it['kind'] != 'use': continue
        toks = tokenize(it['text'])
        sig = [t for t in toks if t[0] not in ('ws', 'comment')]
        k = next((i for i, t in enumerate(sig) if t[0] == 'id' and t[1] == 'use'), None)
        if k is None: continue
        use_tok = sig[k]; end_tok = sig[-1]
        if end_tok[1] != ';': continue
        tree = it['text'][use_tok[3]:end_tok[2]]
        leaves = U.parse(tree)
        out = []; changed = False
        for path, alias, glob in leaves:
            head = path[0] if path else ''
            if not path or head in SKIP_HEADS or head in loc or head in ns._ident_roots or head.startswith('semio_framework_'):
                out.append((path, alias, glob)); continue
            nl = new_leaf(path, alias, glob, label)
            if nl is None: out.append((path, alias, glob)); continue
            out.append(nl); changed = True; stats['leaves'] += 1
        if not changed: continue
        # statement prefix = visibility tokens directly before `use`
        j = k - 1; pre_start = use_tok[2]
        if j >= 0 and sig[j][1] == ')':
            d = 0
            while j >= 0:
                if sig[j][1] == ')': d += 1
                elif sig[j][1] == '(':
                    d -= 1
                    if d == 0: break
                j -= 1
            j -= 1
        if j >= 0 and sig[j][1] == 'pub': pre_start = sig[j][2]
        prefix = it['text'][pre_start:use_tok[2]]
        lines = U.emit(out)
        new = '\n'.join(prefix + l for l in lines)
        edits.append((it['start'] + pre_start, it['start'] + end_tok[3], new))
    for s, e, r in sorted(edits, reverse=True): text = text[:s] + r + text[e:]
    return text, len(edits)

files = [p for p in dump['out'] if p != ROOT and p not in reloc and os.path.exists(p) and not p.endswith('/📦️packages/🦀️rust/🦀️.rs')]
tot = 0
for p in sorted(files):
    t = open(p, encoding='utf8').read()
    t2, n = fix(t, p.split('modules/')[-1][-60:])
    if n:
        tot += n
        print(('fix ' if write else 'would fix ') + p.split('modules/')[-1][-80:], n)
        if write: open(p, 'w', encoding='utf8').write(t2)
print('statements', tot, 'leaves', stats['leaves'])
for u in unresolved[:80]: print('UNRESOLVED', u)
print('unresolved', len(unresolved))
