"""Names a test file uses that another test file of the same unit defines at its top level (the helpers that tie tests together)."""
import collections, sys
from lib import *
files = [REPO + "/" + p for p in test_sources() if "/🧪️tests/" in p or "worker-cell" in p or "outcome" in p or "sqlite" in p or "test-hosts" in p]
defs = {}
for f in files:
    t = read(f)
    for it in itm.table(t):
        if it["kind"] in ("impl", "use", "mod") or it["name"] in ("tests",):
            continue
        if it["kind"] == "fn" and "#[test]" in it["attrs"]:
            continue
        defs.setdefault(f, {})[it["name"]] = it
toks = {f: tokens(read(f)) for f in files}
edges = collections.defaultdict(lambda: collections.defaultdict(set))
for a in files:
    for b in files:
        if a == b:
            continue
        for name in defs.get(b, {}):
            if name in toks[a] and name not in defs.get(a, {}):
                edges[b][name].add(os.path.relpath(a, E).split("/")[-2][:28])
short = lambda f: os.path.relpath(f, E).replace("🧱️elements/", "").replace("🧪️tests/", "").replace("🎯️targets/🧊️wgpu/", "T/")[-62:-9]
for b in sorted(edges, key=lambda b: -sum(len(v) for v in edges[b].values())):
    print("==", short(b))
    for name, users in sorted(edges[b].items(), key=lambda kv: -len(kv[1])):
        print(f"   {name:48} <- {len(users)}: {sorted(users)[:5]}")
