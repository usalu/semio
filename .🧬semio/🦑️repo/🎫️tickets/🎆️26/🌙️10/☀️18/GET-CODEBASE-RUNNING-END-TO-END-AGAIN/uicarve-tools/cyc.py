import json, re, sys
sys.path.insert(0, 'lib')
import carve
specs = [carve.Spec(p) for p in sys.argv[1:]]
crates = {}
for sp in specs: crates.update({c['pkg']: c for c in sp.crates.values()})
edges = {}
for pkg, c in crates.items():
    t = carve.rd(c['abs'] + '/' + carve.PKG + '/Cargo.toml')
    edges[pkg] = set()
    sect = ''
    for l in t.split('\n'):
        if l.startswith('['): sect = l.strip()
        m = re.match(r'(semio-framework-ui-[a-z0-9-]+)\s*=', l)
        if m and m.group(1) in crates and 'dev-dependencies' not in sect: edges[pkg].add(m.group(1))
# cycle detection
color = {}
def dfs(n, path):
    color[n] = 1
    for m in sorted(edges[n]):
        if color.get(m) == 1: print('CYCLE', ' -> '.join(path + [n, m])); return True
        if m not in color and dfs(m, path + [n]): return True
    color[n] = 2
for n in edges:
    if n not in color and dfs(n, []): break
else: print('acyclic', len(edges))
for n in sorted(edges): print(n.replace('semio-framework-ui-', ''), '->', sorted(e.replace('semio-framework-ui-', '') for e in edges[n]))
