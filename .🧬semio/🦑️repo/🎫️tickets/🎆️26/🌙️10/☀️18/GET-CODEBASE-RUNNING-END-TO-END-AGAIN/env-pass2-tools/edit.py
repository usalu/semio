#!/usr/bin/env python3
# usage: edit.py <file> <jsonfile-with-[[old,new,count?],...]>  (direct anchored edit for non-frozen files; refuses library path)
import json, sys, os
ROOT = "/Users/ueli/Documents/semio/"
FROZEN = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
rel = sys.argv[1]
if rel.startswith(ROOT):
    rel = rel[len(ROOT):]
if rel.startswith(FROZEN):
    raise SystemExit("frozen path: stage it instead")
pairs = json.load(open(sys.argv[2], encoding="utf-8"))
t = open(ROOT + rel, encoding="utf-8").read()
for p in pairs:
    n = t.count(p[0])
    want = p[2] if len(p) > 2 else 1
    if n != want:
        raise SystemExit("anchor mismatch (%d != %d): %r" % (n, want, p[0][:100]))
    t = t.replace(p[0], p[1])
open(ROOT + rel, "w", encoding="utf-8").write(t)
print("edited", rel[-80:], len(pairs))
