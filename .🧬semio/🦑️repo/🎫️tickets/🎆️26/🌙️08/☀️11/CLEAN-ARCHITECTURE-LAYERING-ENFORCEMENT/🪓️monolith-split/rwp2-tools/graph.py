"""🕸️ File-level reference graph over the WP2 sources: which crate-bound names cross a planned crate boundary upward (back edges)."""
import collections, re, sys, os
from model import *
KW = set('as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn'.split())
FILTER = lambda n: ('_' in n or n[0].isupper()) and n not in ('_', 'Self')
owner = {}
for c, fs in FILES.items():
    for f in fs: owner[f] = c
defs = collections.defaultdict(set)
texts = {}
for f in owner:
    raw = open(f, encoding='utf8').read(); texts[f] = raw
    for it in items(raw):
        nm = item_name(it)
        if nm and it['kind'] not in ('use', 'impl'): defs[nm].add(f)
        if it['kind'] == 'macro_rules!' or it['kind'] == 'thread_local!':
            pass
    for m in re.finditer(r'macro_rules!\s+(\w+)', raw): defs[m.group(1)].add(f)
back = collections.defaultdict(lambda: collections.defaultdict(set)); fwd = collections.defaultdict(lambda: collections.defaultdict(set))
for f, raw in texts.items():
    code = strip(raw)
    toks = set(re.findall(r'(?<![\w.])([A-Za-z_]\w*)', code)) - KW
    cf = owner[f]
    for name in toks:
        for g in defs.get(name, ()):
            if g == f: continue
            cg = owner[g]
            if cg == cf: continue
            if not FILTER(name): continue
            (back if RANK.index(cg) > RANK.index(cf) else fwd)[(cf, cg)][(os.path.basename(os.path.dirname(f)), os.path.basename(os.path.dirname(g)))].add(name)
mode = sys.argv[1] if len(sys.argv) > 1 else 'back'
for (cf, cg), pairs in sorted((back if mode == 'back' else fwd).items()):
    names = set().union(*pairs.values())
    print(f'{cf} -> {cg}: {len(names)} names')
    for (a, b), ns in sorted(pairs.items()): print(f'    {a} -> {b}: {sorted(ns)[:60]}')
