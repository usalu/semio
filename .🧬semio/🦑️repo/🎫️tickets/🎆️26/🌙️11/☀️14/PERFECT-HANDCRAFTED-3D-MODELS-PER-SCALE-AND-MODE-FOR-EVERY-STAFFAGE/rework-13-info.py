"""🔎 Prints what a plant script has to bind to: dimensions, materials, bones, anchors, thermal regions, emitters, pens and the old solid ids of a catalog asset.

Usage: `python3 rework-13-info.py <slug>`"""
import collections
import json
import os
import re
import sys

T = os.path.dirname(os.path.abspath(__file__))
slug = sys.argv[1]
path = os.path.join(T, *[".."] * 7, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🌳️vegetation", slug, "🔣️.json")
a = json.load(open(path, encoding="utf-8"))["asset"]
print(a["title"]["en"], "|", a["summary"]["en"][:300])
print("dimensions", a["dimensions"])
print("materials", [m["id"] for m in a["materials"]])
for b in a["rig"]["bones"]:
    print("bone", b["id"], "<-", b.get("parent"), "x", b["x"], "y", b["y"], "z", b.get("z", 0), "angle", b["angleDeg"], "len", b["lengthM"], b["joint"], "mass", (b.get("mass") or {}).get("kg"), "fibre", (b.get("section") or {}).get("extremeFibreM"))
for x in a["anchors"]:
    print("anchor", x["id"], x["bone"], x["kind"], x["x"], x["y"], x.get("z"), {k: x[k] for k in x if k.startswith("footprint")})
for r in a["thermal"]["regions"]:
    print("region", r["id"], r["kind"], r["material"], "area", r["areaM2"], r.get("areaBasis"), "layers", r.get("layers"))
for e in a["thermal"]["emitters"]:
    print("emitter", e["id"], e["kind"], e.get("anchors") or e.get("anchor"), e["region"])
print("poses", [p["id"] for p in a["poses"]], "clips", [c["id"] for c in a["clips"]], a["presentation"])
for scale, r in a["representations"].items():
    print("rep", scale, r["status"], r["validRange"], r["pens"], [h["id"] for h in r.get("hatches", [])], {v: len(s) for v, s in r["geometry"].items()}, list(r["decisions"]))
for scale, v in a["volumes"].items():
    names = collections.Counter(re.sub(r"[-_]?\d+", "#", e["id"]) for e in v["body"])
    print("vol", scale, len(v["body"]), len(v["thermal"]), len(v["structure"]), dict(names.most_common(25)))
    print("   structure", dict(collections.Counter((e.get("part"), e.get("kind", "of")) for e in v["structure"])))
    print("   thermal own", [(e["id"], e["region"], e.get("tags")) for e in v["thermal"] if "of" not in e][:14])
print("symbols", [s["id"] for s in a.get("symbols", [])])
