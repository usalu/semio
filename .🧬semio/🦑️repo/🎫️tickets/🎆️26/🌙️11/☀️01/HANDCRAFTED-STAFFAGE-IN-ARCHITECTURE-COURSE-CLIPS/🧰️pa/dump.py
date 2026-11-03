"""🔎 pa: condensed non-geometry sheet of one catalog asset - usage: dump.py <asset-id>."""
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
CATALOG = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"


def locate(asset_id):
    return next(CATALOG.glob(f"*/{asset_id}/🔣️.json"))


def roundtrip(path):
    raw = path.read_text(encoding="utf8")
    return json.dumps(json.loads(raw), ensure_ascii=False, indent=2) + "\n" == raw


def main():
    asset_id = sys.argv[1]
    path = locate(asset_id)
    data = json.loads(path.read_text(encoding="utf8"))
    a = data["asset"]
    print("roundtrip", roundtrip(path))
    print("dims", a["dimensions"])
    print("notes", json.dumps(a["provenance"].get("notes"), ensure_ascii=False)[:1800])
    print("materials", [m["id"] for m in a["materials"]])
    for b in a["rig"]["bones"]:
        print("bone", b["id"], b["parent"], b["x"], b["y"], b["angleDeg"], b.get("lengthM"), b.get("mass", {}).get("kg"), b.get("mass", {}).get("comY"))
    for an in a.get("anchors", []):
        print("anchor", an["id"], an["bone"], an["x"], an["y"], an["kind"])
    th = a.get("thermal")
    if th:
        print("env", th["environment"])
        for o in th.get("operating", []):
            print("opkey", o["id"], [lv["v"] for lv in o["levels"]])
        for r in th["regions"]:
            op = r.get("operating")
            print("region", r["id"], r["kind"], r["model"], r.get("material"), "A", r.get("areaM2"), "L", r.get("characteristicLengthM"), r.get("geometry"), "op", op, "fixed", r.get("fixedTempC"), "Cap", r.get("heatCapacityJM2K"), "R", [(l["id"], l.get("thicknessM"), l["resistanceM2KW"]) for l in r.get("layers", [])])
        for e in th.get("emitters", []):
            print("emitter", {k: v for k, v in e.items() if k not in ("title", "source", "reference")})
        for s in th.get("assumptions", []):
            print("assume", s["en"])
    for p in a["poses"]:
        t = p.get("thermal", {})
        print("pose", p["id"], "op", t.get("operating"), "em", t.get("emitters"), "loads", p.get("loads"))
    for c in a["clips"]:
        print("clip", c["id"], c["durationS"], c["loop"], [k["pose"] for k in c.get("keys", [])], [tr["target"] for tr in c.get("tracks", [])])
    st = a.get("structure")
    if st:
        print("structure", json.dumps(st.get("assumptions"), ensure_ascii=False)[:600])


main()
