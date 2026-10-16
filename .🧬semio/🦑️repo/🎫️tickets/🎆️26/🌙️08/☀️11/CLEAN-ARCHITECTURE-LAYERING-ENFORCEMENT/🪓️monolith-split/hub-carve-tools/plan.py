#!/usr/bin/env python3
"""📐️ Evaluates a crate assignment: sizes per crate (lib and test selections), crate-level edges, cycles, and violating file edges."""
import sys, json, collections, os
from pathlib import Path
SZ = json.load(open('/tmp/hubw/sizes.json'))

def load(plan_path, edges_path, files_path):
    plan = json.load(open(plan_path))
    edges = json.load(open(edges_path)); files = json.load(open(files_path))
    return plan, edges, files

def crate_of(mod, assign):
    best = None
    for crate, prefixes in assign.items():
        for p in prefixes:
            if mod == p or mod.startswith(p + '::'):
                if best is None or len(p) > best[1]:
                    best = (crate, len(p))
    return best[0] if best else None

def is_test(cfg):
    return any('test' in c for c in cfg)

def main(plan_path, edges_path, files_path):
    plan, edges, files = load(plan_path, edges_path, files_path)
    assign = plan['crates']
    owner = {}
    for f, m in files.items():
        c = crate_of(m['mod'], assign)
        owner[f] = c
    unassigned = [f for f, c in owner.items() if c is None]
    if unassigned:
        print('UNASSIGNED', unassigned)
    stats = collections.defaultdict(lambda: dict(lib=[0, 0, 0], test=[0, 0, 0]))
    for f, m in files.items():
        c = owner[f]
        if c is None: continue
        key = '/Users/ueli/Documents/semio/' + f if not f.startswith('/') else f
        s = SZ.get(key)
        # sizes keyed by repo-relative path "🌎️hub/..."
        s = SZ.get('🌎️hub/' + f)
        if not s: continue
        if 'lib' in s and not is_test(m['cfg']):
            stats[c]['lib'][0] += 1; stats[c]['lib'][1] += s['lib'][1]; stats[c]['lib'][2] += s['lib'][2]
        if 'test' in s:
            stats[c]['test'][0] += 1; stats[c]['test'][1] += s['test'][1]; stats[c]['test'][2] += s['test'][2]
    # embedded: attribute by longest shared directory prefix with a source file of a crate
    srcdirs = [(os.path.dirname(f), owner[f]) for f in files if owner[f]]
    for path, s in SZ.items():
        if s.get('test', s.get('lib', ('',))[0:1])[0] != 'embedded' and s.get('lib', ('',))[0] != 'embedded':
            continue
        if not path.startswith('🌎️hub/'): continue
        r = path[len('🌎️hub/'):]
        best = None
        for d, c in srcdirs:
            if r.startswith(d + '/'):
                if best is None or len(d) > best[0]: best = (len(d), c)
        if not best: continue
        c = best[1]
        if 'lib' in s: stats[c]['lib'][0] += 1; stats[c]['lib'][1] += s['lib'][1]
        if 'test' in s: stats[c]['test'][0] += 1; stats[c]['test'][1] += s['test'][1]
    print(f"{'crate':36s} {'lib f/b/l':>24s} {'test f/b/l':>24s}")
    for c in assign:
        s = stats[c]
        flag = lambda v: ' OVER' if (v[0] > 32 or v[1] > 262144 or v[2] > 4000) else ('  warn' if (v[0] > 28 or v[1] > 230000 or v[2] > 3500) else '')
        print(f"{c:36s} {s['lib'][0]:3d}/{s['lib'][1]:7d}/{s['lib'][2]:5d}{flag(s['lib']):6s} {s['test'][0]:3d}/{s['test'][1]:7d}/{s['test'][2]:5d}{flag(s['test'])}")
    ce = collections.defaultdict(lambda: collections.defaultdict(set))
    for a, gs in edges.items():
        ca = owner.get(a)
        if not ca: continue
        for g, names in gs.items():
            cg = owner.get(g)
            if not cg or cg == ca: continue
            kind = 'test' if (is_test(files[a]['cfg']) or is_test(files[g]['cfg'])) else 'lib'
            ce[(ca, cg, kind)][a + ' -> ' + g] |= set(names)
    print('\nCRATE EDGES (lib)')
    graph = collections.defaultdict(set)
    for (ca, cg, kind), d in sorted(ce.items()):
        if kind == 'lib':
            graph[ca].add(cg)
            print(f"  {ca} -> {cg}  ({len(d)} file edges)")
    print('\nCRATE EDGES (test only)')
    for (ca, cg, kind), d in sorted(ce.items()):
        if kind == 'test' and cg not in graph[ca]:
            print(f"  {ca} -> {cg}  ({len(d)} file edges)")
    # cycles (lib)
    WHITE, GREY, BLACK = 0, 1, 2
    color = collections.defaultdict(int); stack = []; cycles = []
    def dfs(u):
        color[u] = GREY; stack.append(u)
        for v in sorted(graph[u]):
            if color[v] == GREY:
                cycles.append(stack[stack.index(v):] + [v])
            elif color[v] == WHITE:
                dfs(v)
        stack.pop(); color[u] = BLACK
    for c in assign:
        if color[c] == WHITE: dfs(c)
    print('\nCYCLES:', cycles if cycles else 'none')
    if '-v' in sys.argv:
        for (ca, cg, kind), d in sorted(ce.items()):
            if kind != 'lib': continue
            for k, names in d.items():
                print(f"   {ca}->{cg}: {k}: {', '.join(sorted(names))[:160]}")

if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2] if len(sys.argv) > 2 else '/tmp/hubw/lib2.json', sys.argv[3] if len(sys.argv) > 3 else '/tmp/hubw/lib2-files.json')
