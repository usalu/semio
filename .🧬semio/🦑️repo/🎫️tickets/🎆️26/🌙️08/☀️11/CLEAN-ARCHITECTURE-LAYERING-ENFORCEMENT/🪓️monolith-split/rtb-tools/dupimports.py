"""Duplicate local names among the `use` lines of a bin root after test/testkit cfg simplification (E0252), and relative module paths with no import."""
import collections, sys
from build import *
roots = []
for suffix, folder in sorted(crate_table().items()):
    if suffix.startswith("shell-tests-") or suffix in ("frame-tests-1", "runtime-mailbox-tests", "engine-attach-tests", "ui-engine-tests", "scene-ink-tests"):
        roots.append((suffix, T + "/" + folder + "/🦀️.rs"))
bad = 0
for suffix, root in roots:
    text = read(root)
    # module-level scope blocks: split on lines starting a top-level `mod x {`
    scopes = re.split(r"(?m)^(?=(?:#\[path = \"\.\"\]\n)?mod \w+ \{)", text)
    for si, sc in enumerate(scopes):
        cnt = collections.Counter()
        paths = collections.defaultdict(set)
        for cfg, vis, p, local, glob in uses(sc):
            if glob or not local:
                continue
            cfg2, alive = simplify_test_cfg(cfg) if cfg else ("", True)
            if not alive:
                continue
            cnt[local] += 1 if not cfg2 else 0
            paths[local].add("::".join(p))
            if cfg2:
                cnt[(cfg2, local)] += 1
        for k, n in cnt.items():
            if n > 1:
                bad += 1
                print(suffix, "scope", si, "duplicate", k, sorted(paths[k if isinstance(k, str) else k[1]]))
print("duplicate groups:", bad)
