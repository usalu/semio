import json, collections, sys
edges = json.load(open(sys.argv[1] if len(sys.argv)>1 else '/tmp/hubw/edges.json')); files = json.load(open('/tmp/hubw/files.json'))
def grp(f):
    m = files[f]['mod'].split('::')
    if len(m) == 1: return 'root'
    t = m[1]
    if t in ('directory',):
        if len(m) == 2: return 'dir.root'
        s = m[2]
        if s in ('tests', 'backend_corpus'): return 'dir.TESTS'
        return 'dir.' + s
    if t == 'artifact_authority':
        if len(m) == 2: return 'aa.root'
        s = m[2]
        if s == 'tests': return 'aa.TESTS'
        if s == 'trusted_catalog' and len(m) > 3 and m[3] in ('trusted_catalog_fixture', 'count_trusted_catalog_fixture'): return 'aa.trusted_catalog'
        return 'aa.' + s
    if t == 'inference':
        if len(m) == 2: return 'inf.root'
        s = m[2]
        if s == 'runtime':
            return 'inf.runtime'
        return 'inf.' + s
    return t
G = collections.defaultdict(set)
for f, gs in edges.items():
    if any('test' in c for c in files[f]['cfg']): continue
    a = grp(f)
    for g, names in gs.items():
        if any('test' in c for c in files[g]['cfg']): continue
        b = grp(g)
        if a != b: G[(a, b)] |= set(names)
rows = sorted(G.items())
for (a, b), n in rows:
    print(f"{a:28s} -> {b:28s} {len(n):3d}  {', '.join(sorted(n)[:6])}")
