"""📦 Print the Cargo.toml path of a crate and whether it declares given crates.
usage: deps.py <crate> [<needle>...]"""
import json, sys, re
idx = json.load(open("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/ohp-manifest-index.json", encoding="utf-8"))
n = sys.argv[1].replace("_", "-")
rel = idx[n]
print(rel)
t = open("/Users/ueli/Documents/semio/" + rel, encoding="utf-8").read()
for needle in sys.argv[2:]:
    needle = needle.replace("_", "-")
    ms = [l[:140] for l in t.split("\n") if l.startswith(needle + " ") or l.startswith(needle + ".") or ('"' + needle + '"') in l]
    print(needle, "->", ms if ms else "NOT DECLARED")
