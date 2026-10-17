"""♻ Would adding a dependency <user> -> <target> create a cycle? Prints the path target ~> user over normal+dev dependencies if one exists.
usage: cyc.py <user> <target>"""
import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/pos")
os.environ["POS_PREFIX"] = "ohp"
from poslib import *
mans = Manifests(Source(None)); mans.build_index()
def deps(pkg, kinds=("dependencies", "build-dependencies")):
    m = mans.manifest(pkg)
    if not m: return []
    out = []
    for kind, name, p, feats, opt, ren in mans.deps(m):
        if kind in kinds: out.append(p)
    return out
user, target = [x.replace("_", "-") for x in sys.argv[1:3]]
seen = {}
stack = [(target, [target])]
while stack:
    cur, path = stack.pop()
    if cur in seen: continue
    seen[cur] = path
    if cur == user:
        print("CYCLE", " -> ".join(path)); sys.exit(1)
    for d in deps(cur):
        if d not in seen: stack.append((d, path + [d]))
print("no normal-dependency cycle (dev-dependency cycles are allowed by cargo); reachable crates:", len(seen))
