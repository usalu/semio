#!/usr/bin/env python3
"""Checks a candidate crate assignment: every file may only reference symbols owned by its own crate or a lower one."""
import re, os, collections, sys
src = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "app-surface-deps.py"), encoding="utf-8").read().split("DEF =")[0].replace("R = sys.argv[1]", "R = '.'")
exec(src)
CR = {'F': ['ids','value_path','field_meta','report_tree','doc_editors','store_macros'], 'V': ['viewport3d'], 'D': ['diagrams2d','accessibility','themes','device_profiles','idomain','manifest'],
 'G': ['live_lanes','dm','edit_rules','cmd_dispatch'], 'P': ['checks','properties','outliner','sel_mirror','catalogue'], 'E': ['job_macros','retained','oracle','live_acc'], 'X': ['root_tests']}
AV = {'F': 'F', 'V': 'V', 'D': 'DFV', 'G': 'GF', 'P': 'PFVDG', 'E': 'EPFVDG', 'X': 'XEPFVDG'}
g2c = {g: c for c, gs in CR.items() for g in gs}
def crate(p, g):
    if 'viewport-panels' in p: return 'P'
    if g == '?': return None
    if 'toolbar' in p or 'lane-overlays' in p: return 'P'
    if 'editor-wiring' in p: return 'E'
    return g2c[g]
DEF = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(?:fn|struct|enum|trait|const|static|type)\s+(\w+)|^\s*macro_rules!\s*(\w+)|^\s+pub\s+(?:struct|enum|trait)\s+(\w+)', re.M)
defs = collections.defaultdict(set)
fc = {}
for p, (g, t, s) in files.items():
    fc[p] = crate(p, g)
    if t or fc[p] is None: continue
    for m in DEF.finditer(s): defs[m.group(1) or m.group(2) or m.group(3)].add(fc[p])
owner = {n: next(iter(c)) for n, c in defs.items() if len(c) == 1 and not (n.islower() and len(n) <= 8 and '_' not in n)}
for p, (g, t, s) in sorted(files.items()):
    c = fc[p]
    if c is None: continue
    bad = collections.defaultdict(set)
    for i, line in enumerate(s.split('\n'), 1):
        if line.lstrip().startswith('//'): continue
        for w in set(re.findall(r'\b\w+\b', line)):
            o = owner.get(w)
            if o and o not in AV[c]: bad[o].add(f'{w}@{i}')
    if bad: print(p, c, {k: sorted(v)[:6] for k, v in bad.items()})
