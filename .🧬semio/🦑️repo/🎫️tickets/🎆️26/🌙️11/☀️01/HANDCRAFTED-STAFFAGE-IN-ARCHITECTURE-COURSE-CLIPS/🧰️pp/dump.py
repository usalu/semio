"""🔬 pp: dump the thermal facts of every thermal cast: dump.py [cast-id-substring ...]"""
import glob
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
CAT = ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
MAN = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"
ASSETS = {}
for f in glob.glob(str(CAT / "*" / "*" / "🔣️.json")):
    a = json.load(open(f, encoding="utf-8"))["asset"]
    ASSETS[a["id"]] = a
ENVS = {e["id"]: e["environment"] for e in json.load(open(CAT / "🌤️environments/🔣️.json", encoding="utf-8"))["presets"]}


def short(o):
    return json.dumps(o, ensure_ascii=False, separators=(",", ":"))


def show(cid, c):
    a = ASSETS[c["asset"]]
    th = a["thermal"]
    e = ENVS.get(c.get("environment"), {})
    print(f"\n### {cid}: {c['asset']} {c.get('pose') or c.get('clip')} env={c.get('environment')} air={e.get('airTempC')} mrt={e.get('meanRadiantTempC')} span={c.get('thermalSpanK')} tr={c.get('transport')} age={c.get('transportAgeS')}")
    print(" dims", short(a["dimensions"]), "operating", short([(o["id"], [(l["v"], l["title"]["en"]) for l in o["levels"]]) for o in th["operating"]]))
    for r in th["regions"]:
        extra = {k: r[k] for k in ("operating", "phaseChange", "internalGainW", "surfaceTempC", "heatW", "model") if k in r}
        print("  R", r["id"], r.get("kind"), "A=", r.get("areaM2"), r.get("material"), short(extra))
    for em in th.get("emitters", []):
        print("  E", short({k: v for k, v in em.items() if k not in ("title",)}))
    for k in th:
        if k not in ("environment", "operating", "passengers", "regions", "emitters", "assumptions"):
            print("  X", k, short(th[k])[:600])
    if c.get("pose"):
        for p in a["poses"]:
            if p["id"] == c["pose"]:
                print("  POSE", short(p.get("thermal")))
    if c.get("clip"):
        for cl in a["clips"]:
            if cl["id"] == c["clip"]:
                print("  CLIP dur", cl["durationS"], "loop", cl.get("loop"), "phases", short([(p["id"], p["fromS"], p["toS"]) for p in cl.get("phases", [])]))
                for t in cl.get("tracks", []):
                    print("   T", t["target"], short([(k["t"], k["v"]) for k in t["keys"]])[:300])
    for s in th.get("assumptions", []):
        print("  A", s["en"][:300] if isinstance(s, dict) and "en" in s else short(s)[:300])


man = json.load(open(MAN, encoding="utf-8"))["casts"]
want = sys.argv[1:]
for cid, c in man.items():
    if c["view"] != "thermal":
        continue
    if want and not any(w in cid or w == c["asset"] for w in want):
        continue
    show(cid, c)
