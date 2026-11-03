import json, sys, pathlib
root = pathlib.Path("🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans")
for name in sys.argv[1:]:
    d = json.loads((root / name / "🔣️.json").read_text(encoding="utf-8"))["asset"]
    body = d.get("body") or d.get("thermal", {}).get("body")
    print(name, "keys", list(d)[:12])
    for i, p in enumerate(d.get("poses", [])):
        t = p.get("thermal", {})
        print(" pose", i, p.get("id"), t.get("activityMet"))
    print(" clips", [(c.get("id")) for c in d.get("clips", [])])
