import json, sys
f = sys.argv[1]
d = json.load(open(f, encoding="utf-8"))["asset"]
print("keys", list(d.keys()))
print("bones", [(b["id"], b["parent"], b["x"], b["y"], b["angleDeg"], b["lengthM"], b["joint"]["type"]) for b in d["rig"]["bones"]])
print("anchors", [(a["id"], a["bone"], a["x"], a["y"], a["kind"]) for a in d["anchors"]])
print("poses", [(p["id"], p.get("key")) for p in d["poses"]], "clips", [c["id"] for c in d["clips"]])
t = d["thermal"]
print("operating", t.get("operating"))
for r in t["regions"]:
    print("region", r["id"], r["kind"], r["model"], r["areaM2"], r["geometry"], r.get("hidden") is not None, r["model"] == "operating" and r.get("operating"))
print("emitters", [(e["id"], e["kind"], e["anchor"] if "anchor" in e else e.get("anchors"), e["directionDeg"]) for e in t["emitters"]])
for k, rep in d["representations"].items():
    if rep["status"] != "authored":
        print("rep", k, rep["status"]); continue
    g = rep["geometry"]
    print("rep", k, "pens", rep["pens"], "hatches", [h["id"] for h in rep["hatches"]], "elev", len(g["elevation"]), "plan", len(g.get("plan", [])))
    if k == sys.argv[2] if len(sys.argv) > 2 else False:
        for s in g["elevation"]:
            print("  ", json.dumps(s, ensure_ascii=False))
print("symbols", [(s["id"], s["semantics"]) for s in d["symbols"]])
print("presentation", d["presentation"])
