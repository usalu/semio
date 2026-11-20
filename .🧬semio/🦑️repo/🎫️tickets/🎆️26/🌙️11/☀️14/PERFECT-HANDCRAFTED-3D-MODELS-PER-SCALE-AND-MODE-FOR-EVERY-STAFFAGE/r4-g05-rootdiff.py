"""🧮 Lists the members outside `volumes` in which a candidate document differs from the catalog asset (top-level key, and per representation the number of changed shapes), so that a chain run is known to change only what is meant."""
import glob
import json
import os
import sys

T = os.path.dirname(os.path.abspath(__file__))
S = os.path.join(T, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
asset, candidate = sys.argv[1], sys.argv[2]
category, slug = asset.split("/")
cat = json.load(open(glob.glob(os.path.join(S, "*" + category, slug, "🔣️.json"))[0], encoding="utf-8"))["asset"]
new = json.load(open(candidate, encoding="utf-8"))["asset"]
for key in sorted(set(cat) | set(new)):
    if key == "volumes" or cat.get(key) == new.get(key):
        continue
    if key == "representations":
        for a, b in zip(cat[key], new[key]):
            if a == b:
                continue
            geo_a, geo_b = a.get("geometry", {}), b.get("geometry", {})
            for view in sorted(set(geo_a) | set(geo_b)):
                sa = {s["id"]: s for s in geo_a.get(view, [])}
                sb = {s["id"]: s for s in geo_b.get(view, [])}
                changed = [i for i in sa if i in sb and sa[i] != sb[i]]
                print(" rep", a.get("scale"), view, "changed", len(changed), "gone", sorted(set(sa) - set(sb)), "new", sorted(set(sb) - set(sa)), changed[:12])
            other = [k for k in set(a) | set(b) if k != "geometry" and a.get(k) != b.get(k)]
            if other:
                print(" rep", a.get("scale"), "other members", other)
    else:
        print("differs:", key)
