"""📋 Round 4 group G11: prints the sections of named solids of one scale and list of the ticket source. Usage: python3 r4-g11-show.py <slug> <scale> <list> <id>..."""
import json
import sys

slug, scale, lst = sys.argv[1:4]
v = json.load(open(f"volumes/humans/{slug}.json", encoding="utf-8"))["volumes"][scale][lst]
for i in sys.argv[4:]:
    for e in v:
        if e["id"] == i:
            if e.get("kind") == "loft":
                print(i, e["bone"], " | ".join(f"{q['at'][0]:.3f},{q['at'][1]:.3f},{q['at'][2]:.3f} {q['rx']:.3f}/{q['ry']:.3f}" + (f" e{q['exponent']}" if "exponent" in q else "") for q in e["sections"]))
            else:
                print(i, json.dumps({k: e[k] for k in e if k not in ("material", "role", "tone")}))
