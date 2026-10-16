#!/usr/bin/env python3
# 🧮️ Losslessness of the WP1 carve: line multiset of the WP1 trees (+ root, hub, macro mount, lift files, document / reactor mount files) before (snapshot) and after, with a normalisation that folds the mechanical rewrites.
import collections, glob, os, re, sys
from lib import *
import uo

DIRS = uo.OWNED_DIRS
EXTRA = ["🪝️mounts/🌱️wp1-foundation/🦀️.rs", "📣️macros/🌱️wp1-window/🦀️.rs"] + [f"📜️declaration/🧩️app/🛬️lifted/{d}/🦀️.rs" for d in ("⚠️fault", "🚫️refusal", "♻️retirement", "⏳️operation-progress")]
EXTRA_DIRS = ["🪝️mounts/🌱️wp1-foundation/🧪️test-hosts", "🧪️tests/🔬️app-window-kits", "🧪️tests/🔬️app-panel-kit"]


def files_under(base):
    out = []
    for d in DIRS + EXTRA_DIRS:
        for dp, dn, fn in os.walk(base + "/" + d):
            dn[:] = [x for x in dn if x not in ("node_modules", "📦️packages")]
            out += [os.path.join(dp, f) for f in fn if f.endswith(".rs")]
    out += [base + "/" + e for e in EXTRA if os.path.exists(base + "/" + e)]
    return sorted(set(out))


def norm(line):
    s = line.strip()
    if not s or s.startswith("//"):
        return None
    if s == "#[doc(hidden)]":
        return None
    s = re.sub(r"pub\((?:crate|super|in [^)]*)\)", "pub", s)
    s = re.sub(r"\b(?:crate|super)::[a-z_0-9]+::", "§::", s)
    s = re.sub(r"\bsemio_framework_plugin_[a-z_0-9]+::", "§::", s)
    s = re.sub(r"\bcrate::", "§::", s)
    s = s.replace("§::§::", "§::")
    return s


def bag(base):
    c = collections.Counter()
    for f in files_under(base):
        for line in open(f, encoding="utf8", errors="ignore").read().split("\n"):
            n = norm(line)
            if n:
                c[n] += 1
    return c

before = bag(SNAP + "/" + PLUG_REL)
after = bag(PLUG)
lost, gained = before - after, after - before
print("normalised distinct lines before", sum(before.values()), "after", sum(after.values()))
print("lost", sum(lost.values()), "gained", sum(gained.values()))
for k, v in sorted(lost.items(), key=lambda kv: -kv[1])[:40]:
    print("  LOST", v, k[:150])
json_out = {"lost": sum(lost.values()), "gained": sum(gained.values())}
open(GEN + "/pwp1-lossless.txt", "w").write("\n".join(f"LOST {v} {k}" for k, v in sorted(lost.items(), key=lambda kv: (-kv[1], kv[0]))) + "\n\n" + "\n".join(f"GAINED {v} {k}" for k, v in sorted(gained.items(), key=lambda kv: (-kv[1], kv[0]))) + "\n")
