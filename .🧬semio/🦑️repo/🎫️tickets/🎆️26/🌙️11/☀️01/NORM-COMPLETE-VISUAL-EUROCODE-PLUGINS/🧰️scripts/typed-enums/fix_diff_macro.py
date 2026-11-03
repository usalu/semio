#!/usr/bin/env python3
"""Typed enums v4: swaps `field: String` for the enum inside the `Patch of …::Host { set { … } }` macro of the diff crate (the converter only knew the `for Host {` spelling). Idempotent."""
import json, re, sys
sys.path.insert(0, ".")
import convert as C
art = sys.argv[1]
spec = json.load(open(f"specs/{art}.json", encoding="utf-8"))
root = C.artifact_root(art)
p = root + "/🧬️schema/🔺️diff/🦀️.rs"
t = open(p, encoding="utf-8").read()
crate = f"semio_s_artifact_norm_{art}_model"
n = 0
for c in spec["conversions"]:
    for hf in c["hosts"]:
        host, field = hf.split(".")
        pat = re.compile(r"(\bof\s+[\w:]*::%s\s*\{\s*set\s*\{[^}]*?\b%s:\s*)String\b" % (re.escape(host), re.escape(field)))
        t, k = pat.subn(r"\g<1>" + f"{crate}::{c['enum']}", t)
        n += k
open(p, "w", encoding="utf-8").write(t)
print(art, "macro fields typed:", n)
