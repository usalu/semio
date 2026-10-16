"""Items (and re-export lines) gated by a bare `cfg(test)` in production crates that the test files name: they vanish for a bin, which builds its dependencies without cfg(test)."""
import collections, sys
from lib import *
import modtree
crates = crate_table()
prod = {}
for suffix, d in crates.items():
    if suffix.startswith("shell-tests") or suffix.endswith("tests") or suffix in ("fixtures", "test-support") or "-tests-" in suffix:
        continue
    for f in modtree.tree(T + "/" + d + "/🦀️.rs"):
        if "/🧪️tests/" in f or "test-hosts" in f:
            continue
        prod.setdefault(f, suffix)
BARE = re.compile(r"#\[cfg\((?:test|all\(test,[^\n]*\)|all\([^\n]*,\s*test\))\)\]")
gated = collections.defaultdict(list)
for f, c in prod.items():
    text = read(f)
    for it in itm.table(text):
        if it["kind"] in ("impl", "mod"):
            continue
        if any(BARE.fullmatch(a) for a in it["attrs"]) and not any("testkit" in a for a in it["attrs"]):
            gated[it["name"]].append((c, f))
    for cfg, vis, p, local, glob in uses(text, top_only=True):
        if local and vis.startswith("pub") and BARE.fullmatch(cfg.strip().split("\n")[-1] if cfg.strip() else ""):
            gated[local].append((c, f))
tests = [REPO + "/" + p for p in test_sources() if "/🧪️tests/" in p or "worker-cell" in p or "outcome" in p or "sqlite" in p]
need = collections.defaultdict(set)
for tf in tests:
    toks = tokens(read(tf))
    own = {i["name"] for i in itm.table(read(tf))}
    for n in toks:
        if n in gated and n not in own:
            for c, f in gated[n]:
                need[(c, os.path.relpath(f, E)[-58:])].add(n)
for k in sorted(need):
    print(k, sorted(need[k])[:14])
