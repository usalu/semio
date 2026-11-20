"""🔎️ Compact listing of the 2D shapes of humans/runner-with-blade-prosthesis per scale and view (id:bone:role:kind:filled)."""
import json, sys
ASSET = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/runner-with-blade-prosthesis/🔣️.json"
a = json.load(open(ASSET))["asset"]
for sc in sys.argv[1:] or ["50", "100", "200", "500"]:
    r = a["representations"][sc]
    print("==", sc, r["pens"], r.get("sections"))
    for view in ["elevation", "front", "plan"]:
        print(" ", view, len(r["geometry"][view]))
        print("   ", " | ".join(f"{s['id']}:{s['bone']}:{s['role'][:3]}:{s['kind'][:4]}:{'F' if s['filled'] else 'o'}:{s['z']}" for s in r["geometry"][view]))
