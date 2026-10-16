"""🕸️ Item-level reference edges between the WP3 crates (staging layout): which crate names an item another WP3 crate defines, checked against the rank order of the plan."""
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import mv, crate_map as C
ORDER = ['host-plumbing', 'frame-actions', 'runtime-mailbox', 'ui-engine-focus', 'ui-engine-dump', 'ui-engine', 'app-runtime', 'frame-presentation', 'frame-transaction', 'frame-job', 'native-probe', 'host-winit', 'browser-worker']
# plan rank: ui-engine trio below the frame cluster; host-plumbing, frame-actions, runtime-mailbox, app-runtime, ...
RANK = {'ui-engine-focus': 0, 'ui-engine-dump': 1, 'ui-engine': 2, 'host-plumbing': 3, 'frame-actions': 4, 'runtime-mailbox': 5, 'app-runtime': 6, 'frame-presentation': 7, 'frame-transaction': 8, 'frame-job': 9, 'native-probe': 10, 'host-winit': 10, 'browser-worker': 11}
own = C.owners()
defs = collections.defaultdict(set)   # name -> crates defining it (top level)
files = {}
for crate, fs in C.MAP.items():
    for f in fs:
        t = open(f, encoding='utf8').read()
        files[f] = t
        for it in mv.split_items(t):
            if it.kind in ('marker', 'innerdoc', 'use', 'other', 'impl', 'mod'): continue
            m = re.search(r'\b(?:fn|struct|enum|const|static|type|trait|union)\s+([A-Za-z_]\w*)', mv.key(it))
            if m: defs[m.group(1)].add(crate)
            if it.kind == 'thread_local!':
                for n in re.findall(r'static\s+([A-Z_0-9]+)', it.text): defs[n].add(crate)
        for n in re.findall(r'thread_local!\s*\{(.*?)\n\}', t, re.S):
            for s in re.findall(r'static\s+([A-Z_0-9]+)', n): defs[s].add(crate)
back = collections.defaultdict(lambda: collections.defaultdict(set))
fwd = collections.defaultdict(lambda: collections.defaultdict(set))
for crate, fs in C.MAP.items():
    ids = set()
    for f in fs:
        t = re.sub(r'//[^\n]*', '', files[f])
        t = re.sub(r'"(?:[^"\\]|\\.)*"', '""', t)
        ids |= set(re.findall(r'[A-Za-z_]\w*', t))
    for n in ids:
        ds = defs.get(n)
        if not ds or crate in ds: continue
        if len(ds) > 1: continue
        d = next(iter(ds))
        (back if RANK[d] > RANK[crate] else fwd)[crate][d].add(n)
for crate in sorted(RANK, key=RANK.get):
    print(crate, '->', {d: len(n) for d, n in sorted(fwd[crate].items(), key=lambda x: RANK[x[0]])})
print('--- BACK EDGES (a crate naming an item of a higher-ranked WP3 crate)')
for crate in sorted(RANK, key=RANK.get):
    for d, ns in back[crate].items():
        print(f'{crate} -> {d}: {sorted(ns)[:25]}{" ..." if len(ns) > 25 else ""}')
