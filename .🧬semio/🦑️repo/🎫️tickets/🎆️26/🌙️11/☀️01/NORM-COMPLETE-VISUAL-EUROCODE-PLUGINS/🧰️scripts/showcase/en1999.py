"""Generates the EN 1999 showcase document `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`: the committed neutral 14-collection subject extended with a welded, a free-outline, a slender and a pinned-connection case so every check family of the spec trace has an entity to run on (python3 -I en1999.py)."""
import copy, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from common import artifact, clone, finish, load

ART = artifact("🪶️en1999")
BASE = load(ART / "🧪️tests/🧫️fixtures/🧬️mutations/🎯️change-category-m1/✅apply/🔣️.before.json")
doc = copy.deepcopy(BASE)
doc["annex"] = "De"
doc["reliabilityClass"] = "RC2"
purlin = next(m for m in doc["members"] if m["id"] == "purlin-1")
i_section = next(s for s in doc["sections"] if s["id"] == "sec-i120")
conn0 = doc["connections"][0]


def vertex(vid, section, ring, element, y, z):
    return {"id": vid, "sectionId": section, "ring": ring, "elementId": element, "y": y, "z": z}


def i_polygon(section, b, h, tf, tw):
    pts = [(-b / 2, -h / 2), (b / 2, -h / 2), (b / 2, -h / 2 + tf), (tw / 2, -h / 2 + tf), (tw / 2, h / 2 - tf), (b / 2, h / 2 - tf), (b / 2, h / 2), (-b / 2, h / 2), (-b / 2, h / 2 - tf), (-tw / 2, h / 2 - tf), (-tw / 2, -h / 2 + tf), (-b / 2, -h / 2 + tf)]
    names = ["flange-bot", "", "", "web", "", "", "flange-top", "", "", "", "", ""]
    return [vertex(f"{section}-v{i}", section, "outer", n, round(p[0], 6), round(p[1], 6)) for i, (p, n) in enumerate(zip(pts, names))]


free_section = clone(i_section, id="sec-free-i", kind="free")
slender_elements = []
for e in i_section["elements"]:
    lip = {} if e["id"] == "web" else {"edgeStiffenerDepth": 0.008, "edgeStiffenerThickness": 0.003}
    slender_elements.append(clone(e, width=0.0235 if e["id"] != "web" else 0.074, thickness=0.003, weldPosition=0.003, **lip))
slender_section = clone(i_section, id="sec-slender", height=0.08, width=0.05, flangeThickness=0.003, webThickness=0.003, elements=slender_elements)
doc["sections"] += [free_section, slender_section]
doc["materials"].append({"id": "mat-6060", "designation": "aw6060-t6", "productForm": "extrudedProfile"})
doc["outlineVertices"] += i_polygon("sec-free-i", 0.07, 0.14, 0.01, 0.008)

welded = clone(purlin, id="purlin-welded", transverseWeld=True, transverseWeldOffset=0.75, bearingLength=0.1, actions=copy.deepcopy(purlin["actions"]))
torsion = clone(purlin, id="purlin-free", sectionId="sec-free-i", actions=copy.deepcopy(purlin["actions"]))
torsion["actions"][1]["torsionK"] = 800.0
slender_actions = [
    {"id": "G", "kind": "permanent", "category": "selfWeight", "source": "external", "gKLine": 300.0, "qKLine": 0.0, "nK": 20000.0, "vYK": 0.0, "vZK": 8000.0, "mYK": 4000.0, "mZK": 500.0, "torsionK": 0.0, "localLoadK": 0.0},
    {"id": "Q-imposed", "kind": "imposed", "category": "office", "source": "external", "gKLine": 0.0, "qKLine": 500.0, "nK": 40000.0, "vYK": 5000.0, "vZK": 12000.0, "mYK": 6000.0, "mZK": 1000.0, "torsionK": 0.0, "localLoadK": 0.0},
]
slender = clone(purlin, id="beam-slender", sectionId="sec-slender", materialId="mat-6060", length=8.0, bucklingLengthY=8.0, bucklingLengthZ=8.0, bucklingLengthT=8.0, ltbLength=8.0, c1=1.0, actions=slender_actions)
doc["members"] += [welded, torsion, slender]

bolt = clone(conn0, id="bolt-1", memberId="purlin-free", kind="bolted")
pin = clone(conn0, id="pin-1", memberId="purlin-welded", kind="pinned", bolts=clone(conn0["bolts"], diameter=0.02, plateThickness=0.01, rows=1, boltsPerRow=1))
weld_thin = clone(conn0, id="weld-thin", memberId="beam-slender", materialId="mat-6060", bolts=clone(conn0["bolts"], diameter=0.008, plateThickness=0.003), welds=clone(conn0["welds"], throat=0.002, length=0.05))
doc["connections"] += [bolt, pin, weld_thin]

doc["fireScenarios"].append({"id": "fire-2", "memberId": "purlin-welded", "thetaA": 250.0, "durationS": 1800.0})
doc["fatigueDetails"].append(clone(doc["fatigueDetails"][0], id="fat-2", detailCategory="40", deltaSigmaC=40000000.0, deltaSigmaEd=15000000.0, nCycles=2000000.0, m1=3.4, m2=5.4))
doc["patchLoads"].append({"id": "patch-sheet", "subjectId": "sheet-pass", "position": "end", "x": 0.0, "bearingLength": 0.05, "webHeight": 0.0, "webAngle": 0.0, "cornerRadius": 0.004, "load": 1000.0})
doc["weldRuns"].append(clone(doc["weldRuns"][0], id="weld-2", memberId="purlin-welded", elementId="web", direction="transverse", tJunction=True))
doc.setdefault("transverseStiffeners", [])
doc["transverseStiffeners"] += [{"id": "stiffener-1", "girderId": "purlin-1", "position": 2.0, "width": 0.05, "thickness": 0.006, "sides": "two"}, {"id": "stiffener-2", "girderId": "purlin-1", "position": 4.0, "width": 0.05, "thickness": 0.006, "sides": "one"}]
doc["transverseStiffeners"] += [{"id": f"stiffener-w{i}", "girderId": "purlin-welded", "position": 1.5 * i, "width": 0.05, "thickness": 0.006, "sides": "two"} for i in (1, 2, 3)]
finish(doc, ART, "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json")
