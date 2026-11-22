"""🔎 Prints the identity, poses, clips, dimensions, provenance and per-scale decision counts of asset files: python3 summary.py <slug|file.json>..."""
import json
import sys

CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
for arg in sys.argv[1:]:
    path = arg if arg.endswith(".json") else f"{CAT}/{arg}/🔣️.json"
    a = json.load(open(path, encoding="utf-8"))["asset"]
    print(f"== {a['id']} | {a['title']} | completion {a.get('completion')}")
    print("   summary:", a["summary"]["en"][:260])
    print("   de:", a["summary"]["de"][:160])
    print("   tags:", a.get("tags"))
    print("   dims:", a["dimensions"], "| bones", len(a["rig"]["bones"]), "| materials", len(a["materials"]), "| refs", len(a.get("references", [])))
    print("   poses:", [(p["id"], p.get("thermal", {}).get("metabolicMet", p.get("thermal", {}).get("met")), "key" if p.get("key") else "") for p in a["poses"]])
    print("   clips:", [(c["id"], c.get("durationS"), c.get("loop")) for c in a["clips"]], "| presentation", a["presentation"])
    print("   provenance:", {k: (v if not isinstance(v, (dict, list)) else "…") for k, v in a["provenance"].items()}, "| notes", json.dumps(a["provenance"].get("notes"), ensure_ascii=False)[:200])
    print("   thermal: base", a["thermal"].get("baseMet"), "env", a["thermal"]["environment"].get("airTempC"), "regions", [r["id"] for r in a["thermal"]["regions"]])
    print("   emitters:", [(e["id"], e.get("powerW")) for e in a["thermal"].get("emitters", [])])
    for sc in ("500", "200", "100", "50"):
        rep, vol = a["representations"][sc], a["volumes"][sc]
        g = rep.get("geometry", {})
        print(f"   1:{sc} {rep.get('status')} shapes e/f/p {[len(g.get(v, [])) for v in ('elevation', 'front', 'plan')]} solids b/t/s {[len(vol.get(k, [])) for k in ('body', 'thermal', 'structure')]} decisions {len(rep.get('decisions', []))}/{len(vol.get('decisions', []))} | {json.dumps(rep.get('decisions'), ensure_ascii=False)[:150]}")
