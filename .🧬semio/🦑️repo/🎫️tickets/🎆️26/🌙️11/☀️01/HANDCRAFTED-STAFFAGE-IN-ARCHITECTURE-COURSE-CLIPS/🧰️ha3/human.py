import json, sys

for path in sys.argv[1:]:
    d = json.load(open(path, encoding="utf-8"))["asset"]
    th = d.get("thermal", {})
    print("==", d["id"], "dims", d.get("dimensions"), "pres", d.get("presentation", {}).get("defaultPose"))
    print("  title", d["title"]["en"], "| summary", d["summary"]["en"][:400])
    bd = th.get("body", {})
    print("  body", {k: bd.get(k) for k in ("massKg", "skinAreaM2", "basalMetabolismW", "metWM2", "coreSetPointC", "skinSetPointC", "sweating", "source", "reference")})
    print("  baseMet", th.get("baseMet"), "subjects", sorted({str(r.get("body", {}).get("subject")) for r in th.get("regions", [])}))
    for r in th.get("regions", []):
        b = r.get("body")
        print("   R", r["id"], r.get("model"), r.get("areaM2"), r.get("heatCapacityJM2K"), r.get("skyViewFactor"), (b or {}).get("segment"), (b or {}).get("metabolicShare"), (b or {}).get("skinFraction"), r.get("layers") and [ (l.get("id"), l.get("resistanceM2KW")) for l in r["layers"]], r.get("material"), r.get("hidden"))
    resp = th.get("respiration") or {}
    for p in resp.get("patterns", []):
        print("   P", p["id"], p["rateBpm"], p["tidalVolumeL"], p["inhaleFraction"], p["pauseAfterInhaleFraction"], p["exhaleFraction"], p["pauseAfterExhaleFraction"], p["route"], p.get("source"), p.get("reference"))
    print("  resp motion", resp.get("motion"), "exhaled", resp.get("exhaledTempC"), resp.get("exhaledRelativeHumidity"))
    for p in d.get("poses", []):
        t = p.get("thermal", {})
        print("   pose", p["id"], "key" if p.get("key") else "", "met", t.get("activityMet"), "breath", t.get("breathing"), "emit", t.get("emitters"), "exp", {k: v.get("exposedFraction") for k, v in t.get("regions", {}).items()}, "contacts", p.get("contacts"))
    for c in d.get("clips", []):
        print("   clip", c["id"], c["durationS"], "loop", c.get("loop"), "fps", c.get("fps"), "root", c.get("rootMotion"), "phases", [(x["id"], x["fromS"], x["toS"]) for x in c.get("phases", [])], "events", [(e["t"], e["kind"]) for e in c.get("events", [])])
        for tr in c.get("tracks", []):
            print("      track", tr["target"], tr.get("ease"), [(k["t"], k["v"]) for k in tr["keys"]][:8])
    print("  structure", d.get("structure", {}).get("dynamics"), "supports", json.dumps(d.get("structure", {}).get("supports"), ensure_ascii=False)[:300]); print("  OPER", json.dumps(th.get("operating"), ensure_ascii=False)[:400]); print("  EMIT", json.dumps(th.get("emitters"), ensure_ascii=False)[:700]); print("  REG-OP", [(r["id"], r.get("operating")) for r in th.get("regions", []) if r.get("operating")])
