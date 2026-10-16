"""🕸️ Item-level crate graph of the plugin partition on the CURRENT tree: file-based crate assignment (lift files belong to their destination crate), edges by token reference, back edges against the Appendix H rank order. usage: CLOSURE=<closure.json> python3 graph3.py [crate-filter,...] [--all]"""
import collections, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
os.environ.setdefault('CLOSURE', HERE + '/../🗑️generated/pwp3-closure-base.json')
sys.path.insert(0, HERE)
import plan_tools as pt
LIFT = '/🛬️lifted/'
EXTRA = {'⏪️time-travel/🪟️panel/': 'time-travel-panel', '⚛️reactor/⏲️tasks/': 'reactor-tasks', '🧩️composition/📨️emission/📦️preparation/': 'composition-emission', '📜️declaration/🛬️lifted/': 'decl-registry'}
_orig = pt.file_crate
def file_crate(f):
    best = None
    for k, v in EXTRA.items():
        if f.startswith(k) and (best is None or len(k) > len(best[0])): best = (k, v)
    if best: return best[1]
    if LIFT in f:
        f = f.split(LIFT)[0] + '/'
    return _orig(f)
pt.file_crate = file_crate
ORDER = ["abi", "actor-host", "identity", "interaction", "operation", "window-config", "window-transient", "window-kits", "window-kits-rich", "decl-views", "decl-emission", "decl-app", "app-builder", "decl-surface", "child", "composition-emission", "composition-publication", "document-media", "job", "command", "peer", "publication-mounted", "retained-command", "reactor-app", "reactor-tasks", "decl-registry", "plugin-program", "reactor-lifecycle", "runtime-instances", "runtime-ops", "extension", "reactor-patches", "reactor-surfaces", "reactor-core", "reactor-jobs", "runtime-exchange", "reactor-turn", "reactor-wit-bridge", "reactor-context", "time-travel-panel", "tool-run", "time-travel"]
RANK = {c: i for i, c in enumerate(ORDER)}
WP3 = {'child', 'composition-emission', 'composition-publication', 'publication-mounted', 'document-media', 'job', 'command', 'peer', 'retained-command', 'extension'}

def run():
    files, items, defs, owner = pt.build()
    unit = lambda i: owner.get(i, i)
    size = collections.Counter(); adj = collections.defaultdict(set); crate_of = {}
    for it in items:
        f = it['file']
        if '/🧪️tests/' in f or f.startswith('🧪️') or it['cfgtest']: continue
        size[unit(it['id'])] += it['e'] - it['s'] + 1
    for u in size: crate_of[u] = pt.item_crate(items[u]['file'], items[u]['s'])
    for it in items:
        u = unit(it['id'])
        if u not in size: continue
        for t in it['toks']:
            if t in pt.GENERIC or t == (it['defname'] or it.get('target')): continue
            ds = defs.get(t)
            if not ds or len(ds) > 3: continue
            for j in ds:
                v = unit(j)
                if v != u and v in size: adj[u].add(v)
    return items, size, adj, crate_of

def label(items, u):
    it = items[u]
    return (it['defname'] or it['head'][:40]) + '@' + it['file'].replace('🧰️framework/', '')[:60] + ':' + str(it['s'])

if __name__ == '__main__':
    flt = set(a for a in sys.argv[1:] if not a.startswith('--'))
    items, size, adj, crate_of = run()
    lines = collections.Counter()
    for u, c in crate_of.items(): lines[c] += size[u]
    print('units', len(size), 'crates', len(lines))
    back = collections.defaultdict(list)
    for u, vs in adj.items():
        cu = crate_of[u]
        if cu.startswith(('TOP', 'UNMAPPED')) or cu not in RANK: continue
        for v in vs:
            cv = crate_of[v]
            if cv.startswith(('TOP', 'UNMAPPED')) or cv not in RANK: continue
            if RANK[cv] > RANK[cu]: back[(cu, cv)].append((u, v))
    tot = sum(len(v) for v in back.values())
    print('back edges', tot, 'crate pairs', len(back))
    for (cu, cv), es in sorted(back.items(), key=lambda kv: -len(kv[1])):
        if flt and not (cu in flt or cv in flt): continue
        print(f'{cu} -> {cv}: {len(es)} edges; e.g. ' + '; '.join(f"{items[u]['defname'] or items[u]['head'][:25]}->{items[v]['defname'] or items[v]['head'][:25]}" for u, v in es[:4]))
    um = [(crate_of[u], label(items, u)) for u in crate_of if crate_of[u].startswith('UNMAPPED')]
    print('unmapped', len(um), um[:5])
    if '--sizes' in sys.argv:
        for c in ORDER: print(f'{c:24s} {lines[c]:6d}')
