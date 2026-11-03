import json, sys, pathlib
root = pathlib.Path("🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
for name in sys.argv[1:]:
    p = next(root.rglob(name + "/🔣️.json"))
    d = json.loads(p.read_text(encoding="utf-8"))["asset"]
    print("=====", name, p.parent.parent.name)
    print("summary:", d.get("summary"))
    th = d.get("thermal", {})
    print("thermal keys:", list(th))
    for r in th.get("regions", []):
        print("  region", r.get("id"), r.get("operating"))
    for k in ("emitters", "airflows", "slots", "heatSources"):
        if k in th:
            print(" ", k, json.dumps(th[k], ensure_ascii=False)[:600])
    print("poses", [(q.get("id"), q.get("thermal", {}).get("level")) for q in d.get("poses", [])])
    print("clips", [(c.get("id")) for c in d.get("clips", [])])
    for k in d:
        if k not in ("rig", "representations", "symbols", "materials", "anchors", "poses", "clips", "thermal", "dimensions", "provenance", "tags"):
            print(" key", k, json.dumps(d[k], ensure_ascii=False)[:300])
