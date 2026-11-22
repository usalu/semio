"""🔎 Author 07: prints the body solids of a volumes source as compact section tables.

Usage: python3 rework-07-dump.py <slug> <scale> [id-substring ...]
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
slug, scale, wanted = sys.argv[1], sys.argv[2], sys.argv[3:]
for e in json.load(open(os.path.join(HERE, "volumes", "animals", slug + ".json"), encoding="utf-8"))["volumes"][scale]["body"]:
    k = e["id"]
    if k.startswith("far-") or (wanted and not any(w in k for w in wanted)):
        continue
    if e["kind"] == "loft":
        r = e["sections"][0]["rotation"]
        print(k, e["bone"], e["material"], "" if r == [0.5, 0.5, 0.5, 0.5] else r, "W" if e.get("weights") else "", e["sections"][0].get("exponent"))
        print("   ", " ".join("(%g,%g,%g,%g,%g)" % (*s["at"], s["rx"], s["ry"]) for s in e["sections"]))
    else:
        print(k, e["bone"], e["kind"], e["material"], json.dumps({x: y for x, y in e.items() if x not in ("id", "bone", "kind", "material", "role", "tone")}))
