#!/usr/bin/env python3
"""🧪️ Test-file component analysis: connected components of top-level items linked by helper use, with sizes and the test count."""
import sys, os, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import itemdeps

def components(path):
    byname, deps, sizes = itemdeps.analyse(path)
    adj = collections.defaultdict(set)
    for a, ds in deps.items():
        for d in ds:
            adj[a].add(d); adj[d].add(a)
    seen = set(); comps = []
    for n in byname:
        if n in seen: continue
        comp = []; stack = [n]
        while stack:
            v = stack.pop()
            if v in seen: continue
            seen.add(v); comp.append(v)
            stack.extend(adj[v] - seen)
        comps.append(comp)
    out = []
    for c in comps:
        lines = sum(sizes[k] for k in c)
        tests = [k for k in c if any(any(a.startswith('#[test') or a.startswith('#[tokio::test') or 'async_test' in a for a in it.attrs) for it in byname[k])]
        out.append((lines, len(tests), c))
    return sorted(out, key=lambda x: -x[0]), sizes, byname

if __name__ == '__main__':
    comps, sizes, byname = components(sys.argv[1])
    for lines, nt, c in comps:
        print(f'{lines:5d} lines {nt:3d} tests  {", ".join(c[:6])}{" ..." if len(c) > 6 else ""}')
