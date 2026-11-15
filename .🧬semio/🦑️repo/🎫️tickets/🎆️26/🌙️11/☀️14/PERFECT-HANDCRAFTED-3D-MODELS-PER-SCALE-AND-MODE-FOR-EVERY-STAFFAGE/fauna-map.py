"""🗺️ Prints the `kit-thermal --map` string for an asset scale: every body solid id goes to the region of the first matching regex rule of the rules file ({ "rules": [[regex, region], ...] }); unmatched ids are listed on stderr and left out.
Usage: python3 fauna-map.py <asset> <scale> <rules.json>"""
import json, re, sys
asset, scale, rules = sys.argv[1], sys.argv[2], json.load(open(sys.argv[3]))["rules"]
d = json.load(open(f"volumes/{asset}.json"))["volumes"][scale]
out = {}
for s in d["body"]:
    for rx, region in rules:
        if re.search(rx, s["id"]):
            out.setdefault(region, []).append(s["id"]); break
    else:
        print("unmapped", s["id"], file=sys.stderr)
print(",".join(f"{r}=" + "+".join(ids) for r, ids in out.items() if r))
