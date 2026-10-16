"""Private (no `pub`) top-level items of production files that a test file names (candidates for E0603/E0425 once the test leaves the module)."""
import collections, sys
from lib import *
import modtree
crates = crate_table()
prod_files = {}
for suffix, d in crates.items():
    if suffix.startswith("shell-tests") or suffix.endswith("tests") or suffix in ("fixtures", "test-support"):
        continue
    for f in modtree.tree(T + "/" + d + "/🦀️.rs"):
        if "/🧪️tests/" in f or "test-hosts" in f:
            continue
        prod_files.setdefault(f, suffix)
# the facade parents and hub files are production too
for f in [R + "/🦀️.rs"] + [R + "/📥️mounts/" + h + "/🦀️.rs" for h in os.listdir(R + "/📥️mounts")]:
    prod_files.setdefault(f, "root")
priv = collections.defaultdict(list)
public = collections.defaultdict(list)
for f, c in prod_files.items():
    for it in itm.table(read(f)):
        if it["kind"] in ("impl", "use", "mod"):
            continue
        if it["vis"].startswith("pub"):
            public[it["name"]].append(c)
        else:
            priv[it["name"]].append((c, f))
tests = all_test_files = [REPO + "/" + p for p in test_sources() if "/🧪️tests/" in p or "worker-cell" in p or "outcome" in p or "sqlite" in p]
res = collections.defaultdict(set)
for tf in tests:
    toks = tokens(read(tf))
    own = {i["name"] for i in itm.table(read(tf))}
    for n in toks:
        if n in priv and n not in public and n not in own:
            for c, f in priv[n]:
                res[(c, os.path.relpath(f, E)[-50:])].add(n)
for k in sorted(res):
    print(k, sorted(res[k])[:12])
print(sum(len(v) for v in res.values()))
