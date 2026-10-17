import os, re, sys, importlib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import c4p34 as P
from rsitems import items, name_kind
bad = 0
for tag in sys.argv[1:]:
    sp = importlib.import_module('specs.' + tag).SPEC
    root = os.path.join(P.SCR, 'stage', tag)
    texts = [open(os.path.join(root, '🦀️.rs'), encoding='utf-8').read()] + [open(os.path.join(root, f, '🦀️.rs'), encoding='utf-8').read() for f in P.rel_folders(sp)]
    mods = [c['mod'] for c in sp['children']] + ([sp['inner']['mod']] if sp.get('inner') else []) + [c['mod'] for c in (sp['inner']['children'] if sp.get('inner') else [])]
    names = {}
    for t in texts:
        for it in items(t):
            v, k, n = name_kind(it['toks'])
            if n and k not in ('impl',):
                names.setdefault(n, []).append(k)
        for m in re.finditer(r'\buse\b[^;]*;', t, re.S):
            for w in re.findall(r'(?:\bas\s+|::)([A-Za-z_]\w*)\s*(?=[,;}]|$)', m.group(0)):
                names.setdefault(w, []).append('use')
    for n in mods:
        kinds = names.get(n, [])
        extra = [k for k in kinds if k != 'mod'] + ([k for k in kinds if k == 'mod'][1:])
        if extra:
            print(tag, 'CLASH child module', n, kinds)
            bad += 1
print('modclash', 'OK' if not bad else 'FAIL')
sys.exit(1 if bad else 0)
