"""Reviewer 2 pass: re-runs the audit and lists rows of machinery, vehicles, components, objects that have no Reviewer 2 decision or whose file value differs from the decided one (author changed it again)."""
import glob, importlib.util, json, os, subprocess
T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *[".."] * 7))
ROOT = os.path.join(REPO, "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
s = importlib.util.spec_from_file_location("dec2", os.path.join(T, "areareview2-decisions.py")); dec = importlib.util.module_from_spec(s); s.loader.exec_module(dec)
subprocess.run(["python3", "areaaudit.py"], cwd=T, capture_output=True)
a = json.load(open(os.path.join(T, "area-audit.json")))
mine = ("machinery", "vehicles", "components", "objects")
new = []
for r in a["areas"]:
    if not any(c in r["category"] for c in mine):
        continue
    k = (r["slug"], r["region"])
    if k not in dec.R:
        new.append(("UNDECIDED", k, r["old"], r["new"]))
        continue
    act, val, why = dec.R[k]
    if act == "S" or act == "SX":
        if abs(r["new"] - val) > 1e-9:
            new.append(("CHANGED-AGAIN", k, val, r["new"]))
# also open rows of regions never in the audit: changed area in file vs decision for every decided key
for (slug, region), (act, val, why) in dec.R.items():
    if act in ("S", "SX"):
        doc = json.load(open(glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]))["asset"]
        reg = next((x for x in doc["thermal"]["regions"] if x["id"] == region), None)
        if reg is None or abs(reg["areaM2"] - val) > 1e-9:
            t = ("CHANGED-AGAIN", (slug, region), val, reg and reg["areaM2"])
            if t not in new:
                new.append(t)
snap = os.path.join(T, "🗑️generated", "areareview2", "snapshot.json")
cur = {}
for (slug, region) in dec.R:
    doc = json.load(open(glob.glob(f"{ROOT}/*/{slug}/🔣️.json")[0]))["asset"]
    reg = next((x for x in doc["thermal"]["regions"] if x["id"] == region), None)
    cur[f"{slug}/{region}"] = reg["areaM2"] if reg else None
if os.path.exists(snap):
    old = json.load(open(snap))
    for k, v in cur.items():
        if k in old and old[k] != v and not any(x[1] == tuple(k.split("/", 1)) for x in new):
            new.append(("CHANGED-AGAIN", tuple(k.split("/", 1)), old[k], v))
json.dump(cur, open(snap, "w"), indent=0)
print(len(a["areas"]), "audit rows;", len(new), "new for reviewer 2")
for x in new:
    print(x)
