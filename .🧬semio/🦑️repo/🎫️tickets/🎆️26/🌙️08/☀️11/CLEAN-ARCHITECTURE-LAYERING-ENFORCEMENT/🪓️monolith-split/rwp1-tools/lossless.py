#!/usr/bin/env python3
# 🧾️ Losslessness of the carve: the multiset of non-blank source lines of the WP1-owned trees before (snapshot) and after, with visibility widening folded.
import collections, os, re, sys
from common import *

CRATE_DIRS = [T_REL.split("/", 1)[0] and "🎯️targets/🧊️wgpu/" + d for d in CRATES.values()]
WIDE = re.compile(r"\bpub\((?:crate|super)\)")


def norm(line):
    line = line.strip()
    if not line:
        return None
    line = WIDE.sub("pub", line)
    return re.sub(r"\s+", " ", line)


def lines_of(base, rels, only_rs=True):
    c = collections.Counter()
    for rel in rels:
        root = base + "/" + rel
        if os.path.isfile(root):
            files = [root]
        else:
            files = []
            for b, dirs, fs in os.walk(root):
                dirs[:] = [d for d in dirs if d not in ("node_modules", "target", "📦️packages")]
                files += [os.path.join(b, f) for f in fs if f.endswith(".rs")]
        for f in files:
            for l in open(f, encoding="utf8", errors="ignore").read().split("\n"):
                n = norm(l)
                if n:
                    c[n] += 1
    return c


def main():
    owned = [d for d in OWNED if d not in CRATE_DIRS] + OWNED_FILES
    before = lines_of(SNAP + "/engine", owned + CRATE_DIRS)
    after = lines_of(ENGINE_LIVE, owned + CRATE_DIRS)
    lost = before - after
    added = after - before
    print("before", sum(before.values()), "after", sum(after.values()), "lost", sum(lost.values()), "added", sum(added.values()))
    out = open(GEN + "/rwp1-lossless.txt", "w")
    for k, v in sorted(lost.items(), key=lambda kv: -kv[1]):
        out.write(f"LOST {v} {k}\n")
    for k, v in sorted(added.items(), key=lambda kv: -kv[1]):
        out.write(f"ADDED {v} {k}\n")
    # classify
    kinds = collections.Counter()
    for k, v in lost.items():
        kind = "path-mount" if k.startswith(("#[path", "pub mod ", "mod ", "pub(crate) mod")) else "use-super" if k.startswith("use super::") else "use" if k.startswith(("use ", "pub use ")) else "region" if k.startswith("//#") else "other"
        kinds[kind] += v
    print("lost by kind:", dict(kinds))
    kinds = collections.Counter()
    for k, v in added.items():
        kind = "path-mount" if k.startswith(("#[path", "pub mod ", "mod ", "pub(crate) mod")) else "use" if k.startswith(("use ", "pub use ", "extern crate")) else "region" if k.startswith("//#") else "doc" if k.startswith("//!") or k.startswith("///") else "other"
        kinds[kind] += v
    print("added by kind:", dict(kinds))


ENGINE_LIVE = E
if __name__ == "__main__":
    main()
