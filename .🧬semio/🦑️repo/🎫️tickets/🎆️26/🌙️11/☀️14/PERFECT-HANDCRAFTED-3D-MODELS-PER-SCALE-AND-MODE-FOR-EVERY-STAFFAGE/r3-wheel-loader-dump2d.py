"""🔎 Lists the 2D shapes of the wheel loader per scale and view (ids, bones, roles, coordinates) for the round-3 audit."""
import json, sys
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/wheel-loader/🔣️.json"
a = json.load(open(A))["asset"]
full = "--full" in sys.argv
only = [x for x in sys.argv[1:] if not x.startswith("--")]
for k, r in a["representations"].items():
    if only and k not in only:
        continue
    print("=====", k, r["pens"], r.get("hatches"))
    for dk, dv in r["decisions"].items():
        print("  D:", dk, json.dumps(dv, ensure_ascii=False)[:700])
    for vk, vv in r["geometry"].items():
        print("  --", vk, len(vv))
        for s in vv:
            core = {kk: v for kk, v in s.items() if kk not in ("kind", "id", "bone", "role", "z", "filled", "tone", "region", "material", "closed")}
            txt = json.dumps(core, ensure_ascii=False)
            print("    ", s.get("id"), s["kind"], s.get("bone"), s.get("role"), "z", s.get("z"), "f", s.get("filled"), "t", s.get("tone"), s.get("region"), s.get("material"), "c", s.get("closed"), txt if full else txt[:160])
