"""🛞️ Hand-writes distinct per-scale volumes and the one thermal region of the continuous-rotation fixture (the wheel on its stand), so that validateAsset accepts it: every scale models the wheel anew (rim thickens and the spokes merge into a hub as the scale gets coarser), the thermal list references the rim. usage: python3 verify-fixture-wheel.py [areaM2]"""
import copy, json, re, sys
P = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🔄️continuous-rotation/🔣️.json"
NUM = r"-?\d[\d.eE+-]*"
def dump(d):
    s = json.dumps(d, indent=2, ensure_ascii=False)
    s = re.sub(r"\[\n\s+((?:" + NUM + r",\n\s+)*" + NUM + r")\n\s*\]", lambda m: "[" + ", ".join(p.strip() for p in m.group(1).split(",\n")) + "]", s)
    return s + "\n"
text = open(P).read()
doc = json.loads(text)
assert dump(doc) == text, "formatting is not reproduced"
a = doc.get("asset", doc)
area = float(sys.argv[1]) if len(sys.argv) > 1 else 0.85
a["thermal"]["regions"] = [{"id": "rim", "title": {"en": "Wheel rim", "de": "Radfelge"}, "kind": "surface", "model": "passive", "material": "steel", "areaM2": area, "characteristicLengthM": 0.094, "geometry": "cylinder", "skyViewFactor": 0.8, "layers": [], "heatCapacityJM2K": 3000, "source": "authored-estimate", "hidden": {"en": "The fixture draws no thermal shape for the rim.", "de": "Die Fixture zeichnet keine thermische Form der Felge."}}]
for pose in a["poses"]:
    pose["thermal"]["regions"] = {"rim": {"exposedFraction": 1, "sunlitFraction": 0}}
old = a["volumes"]["50"]
def stand(w=0.094): return {"id": "stand", "kind": "box", "bone": "stand", "material": "steel", "role": "body", "tone": 0.5, "size": [0.2, 0.05, w], "transform": {"translation": [0, 0.025, 0]}}
def rim(inner): return {"id": "rim", "kind": "revolve", "bone": "wheel", "material": "steel", "role": "body", "tone": 0.5, "profile": [[-0.047, inner], [0.047, inner], [0.047, 0.494], [-0.047, 0.494], [-0.047, inner]], "axis": [0, 0, 1], "origin": [0, 0, 0]}
def hub(r, w=0.06): return {"id": "hub", "kind": "box", "bone": "wheel", "material": "steel", "role": "detail", "tone": 0.4, "size": [2 * r, 2 * r, w]}
def spoke(): return copy.deepcopy(old["body"][2])
def member(id, bone, r): return {"id": id, "kind": "capsule", "bone": bone, "material": "steel", "role": "body", "tone": 0.4, "p0": [0.0, 0.0, 0.0], "p1": [0.5, 0.0, 0.0], "r0": r, "r1": r, "member": bone, "part": "member"}
def support(s): return {"id": "support-foot", "kind": "box", "bone": "stand", "material": "steel", "role": "ground", "tone": 0.3, "size": [s, s, s], "transform": {"translation": [0, 0, 0]}, "member": "stand", "part": "support", "anchor": "foot"}
def thermal(inner, half=0.03): return [{"id": "th-rim", "kind": "revolve", "bone": "wheel", "material": "steel", "role": "body", "tone": 0.5, "profile": [[-half, inner + 0.01], [half, inner + 0.01], [half, 0.484], [-half, 0.484], [-half, inner + 0.01]], "axis": [0, 0, 1], "origin": [0, 0, 0], "region": "rim"}]
def decisions(scale, en, de): return [{"en": f"Fixture model at 1:{scale}: {en}", "de": f"Fixture-Modell bei 1:{scale}: {de}"}]
plan = {
    "50": (0.444, [stand(), rim(0.444), spoke()], 0.04, 0.05, "the rim and one spoke are modelled as in the drawing.", "Felge und eine Speiche wie in der Zeichnung."),
    "100": (0.438, [stand(), rim(0.438)], 0.045, 0.06, "the rim is drawn a little thicker and the spoke is left out.", "Die Felge ist etwas dicker gezeichnet, die Speiche entfällt."),
    "200": (0.43, [stand(), rim(0.43), hub(0.03)], 0.05, 0.07, "a hub block stands for the spokes.", "Ein Nabenblock steht für die Speichen."),
    "500": (0.42, [stand(0.1), rim(0.42), hub(0.045, 0.09)], 0.06, 0.09, "the stand is a little wider and the hub is a larger block.", "Der Ständer ist etwas breiter und die Nabe ein größerer Block."),
}
for scale, (inner, body, r, s, en, de) in plan.items():
    v = a["volumes"][scale]
    v["body"] = body
    v["thermal"] = thermal(inner, 0.04 if scale == "500" else 0.03)
    v["structure"] = [member("member-stand", "stand", r), member("member-wheel", "wheel", r), support(s)]
    v["decisions"] = decisions(scale, en, de)
open(P, "w").write(dump(doc))
