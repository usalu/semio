"""🔎️ FH1: prints every family-H (or A) violation of one owner with source context from the overlay."""
import json, sys
O = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults/"
fam, owner = sys.argv[1], sys.argv[2]
ctx = int(sys.argv[3]) if len(sys.argv) > 3 else 3
rules = set(sys.argv[4].split(",")) if len(sys.argv) > 4 else None
d = json.load(open(f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-fh1-census/family-{fam}.json"))
seen = set()
for v in sorted(d["violations"], key=lambda v: (v["path"], v["line"])):
    if owner not in v["path"] and owner != "*":
        continue
    if rules and v["rule"] not in rules:
        continue
    print(f"### {v['rule']} {v['path'].split('🔌️plugins/')[-1]}:{v['line']} {v['detail'][:160]}")
    key = (v["path"], v["line"])
    if key in seen or v["line"] == 0:
        continue
    seen.add(key)
    lines = open(O + v["path"]).read().split("\n")
    for i in range(max(0, v["line"] - 1 - ctx), min(len(lines), v["line"] + ctx)):
        print(f"{i+1:6}| {lines[i][:220]}")
