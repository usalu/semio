#!/usr/bin/env python3
"""Symbol-level module dependency scan for the app-surface crate (non-test and test files separately)."""
import os, re, sys, collections
R = sys.argv[1]
GROUPS = {
 'ids':'🏷️ids-chrome-and-labels','value_path':'🧭️value-path','field_meta':'🔖️field-meta','live_lanes':'🫧️live-lanes',
 'edit_rules':'✏️edit-rules','report_tree':'🌳️report-tree','checks':'🪟️visual-windows/✅️checks','doc_editors':'📝️document-editors',
 'outliner':'🌲️outliner','sel_mirror':'🪄️selection-mirror','properties':'🪟️visual-windows/🧾️properties','diagrams2d':'🪟️visual-windows/📈️diagrams2d',
 'viewport3d':'🪟️visual-windows/🧊️viewport3d','accessibility':'🪟️visual-windows/♿️accessibility','device_profiles':'🪟️visual-windows/📱️device-profiles',
 'themes':'🪟️visual-windows/🎨️themes','idomain':'🕹️direct-manipulation/🧭️domain','dm':'🕹️direct-manipulation','catalogue':'📚️catalogue-and-inspection',
 'manifest':'🪟️manifest-and-media','cmd_dispatch':'⌨️command-args-and-dispatch','job_macros':'🏭️command-and-tool-job-macros','retained':'🧵️retained-commands',
 'store_macros':'🛡️store-ownership-macros','oracle':'🧪️tests/🔬️retained-disposition-oracle','live_acc':'🧪️tests/🔬️live-acceptance','root_tests':'🧪️tests/🔬️unit',
}
order = sorted(GROUPS.items(), key=lambda kv: -len(kv[1]))
files = {}
for dp, dn, fn in os.walk(R):
    if 'target' in dp or '📦️packages' in dp: continue
    for f in fn:
        if f.endswith('.rs'):
            p = os.path.relpath(os.path.join(dp, f), R)
            if p == '🦀️.rs': continue
            g = next((k for k, v in order if p.startswith(v + '/')), '?')
            istest = '🧪️tests' in p and g not in ('oracle','live_acc','root_tests') or g in ('root_tests',)
            files[p] = (g, istest, open(os.path.join(dp, f), encoding='utf-8').read())
DEF = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?(?:fn|struct|enum|trait|const|static|type|mod)\s+(\w+)|macro_rules!\s*(\w+)|^\s+pub\s+(?:struct|enum|trait)\s+(\w+)', re.M)
defs = collections.defaultdict(set)
for p, (g, t, s) in files.items():
    if t: continue
    for m in DEF.finditer(s):
        n = m.group(1) or m.group(2) or m.group(3)
        defs[n].add(g)
owner = {n: next(iter(gs)) for n, gs in defs.items() if len(gs) == 1 and not (n.islower() and len(n) <= 8 and '_' not in n)}
dup = {n: gs for n, gs in defs.items() if len(gs) > 1}
edges = collections.defaultdict(lambda: collections.defaultdict(set))
size = collections.Counter(); testsize = collections.Counter()
for p, (g, t, s) in files.items():
    (testsize if t else size)[g] += len(s.encode())
    if t: continue
    for w in set(re.findall(r'\b\w+\b', s)):
        o = owner.get(w)
        if o and o != g: edges[g][o].add(w)
if '-q' not in sys.argv:
    for g in GROUPS:
        print(f'{g:16} {size[g]:7} test {testsize[g]:6} ->', ', '.join(f'{o}({",".join(sorted(ws)[:4])}{"+" if len(ws)>4 else ""})' for o, ws in sorted(edges[g].items())))
    print('dup names', {k: sorted(v) for k, v in dup.items() if len(k) > 3})
if len(sys.argv) > 3 and sys.argv[2] == '--pair':
    a, b = sys.argv[3], sys.argv[4]
    for p, (g, t, s) in sorted(files.items()):
        if g != a or t: continue
        for i, line in enumerate(s.split('\n'), 1):
            hit = [w for w in set(re.findall(r'\b\w+\b', line)) if owner.get(w) == b]
            if hit: print(f'{p}:{i}: {sorted(hit)}  {line.strip()[:110]}')
