"""Writes the two prestressed check-outcome fixtures (a resolved tendon with control points) from the tank wall mutation snapshot. Run with python3 -I."""
import copy, json, pathlib
ROOT = pathlib.Path("/home/deck/semio")
ART = next(ROOT.glob("✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any"))
BASE = ART / "🧪️tests/🧫️fixtures/🧬️mutations/⭕change-bar-layer-diameter/✅apply/🔣️.after.json"
OUT = ART / "🧪️tests/🧫️fixtures/🚦️check-outcome"
base = json.loads(BASE.read_text(encoding="utf-8"))
def prestressed(title, force, loss, area=8.0e-4, drop=-0.12):
    doc = copy.deepcopy(base)
    doc["title"] = title
    member = doc["members"][0]
    member["prestressSteelId"] = "yp1860"
    member["prestress"] = {"force": force, "area": area, "eccentricity": 0.05, "lossRatio": loss}
    member["tendonPoints"] = [{"id": "t0", "x": 0.0, "y": 0.0, "z": 0.0}, {"id": "t1", "x": member["span"] / 2, "y": 0.0, "z": drop}, {"id": "t2", "x": member["span"], "y": 0.0, "z": 0.0}]
    return doc
for name, doc in {"prestressed-wall-pass": prestressed("Prestressed tank wall within the stress limits", 1.0e6, 0.2), "prestressed-wall-fail": prestressed("Prestressed tank wall above the stressing limits", 1.4e6, 0.2), "prestressed-wall-concrete": prestressed("Prestressed tank wall with a concrete stress above 0.6 fck", 1.1e6, 0.2, 8.8e-4, -0.24)}.items():
    (OUT / name).mkdir(parents=True, exist_ok=True)
    (OUT / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(name)
