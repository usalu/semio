"""Generates the en1993 scalar-grip leaf tables from the committed snapshot schema (labels, units, steps).

Run: /home/deck/semio/.venv/bin/python -I gen.py > <scene-members>/🎚️scalar-grips/leaves.rs.txt
The output is pasted between the markers of the scalar-grips module (see the module doc).
"""
import json, pathlib, re, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
schema = json.loads((ROOT / "🧬️schema/📸️snapshot/🔣️.json").read_text(encoding="utf-8"))["$defs"]
model = (ROOT / "🧬️schema/📸️snapshot/🦀️.rs").read_text(encoding="utf-8")
DEFS = [("SteelJoint", "joints", "joint_leaves", "SteelJoint"), ("JointPlate", "jointPlates", "joint_plate_leaves", "JointPlate"), ("HollowJoint", "hollowJoints", "hollow_joint_leaves", "HollowJoint"), ("PlatedPanel", "platedPanels", "panel_leaves", "PlatedPanel"), ("ColdFormedMember", "coldFormedMembers", "cold_formed_leaves", "ColdFormedMember")]
SIGNED_NAME = re.compile(r"(?i)eccentric|offset|stress|force|moment|reaction|load|psi|angle|shear|pressure|temperature|rotation|slope|deviation")

def snake(name):
    return re.sub(r"([A-Z])", lambda m: "_" + m.group(1).lower(), name)

def rust(text):
    return text.replace("\\", "\\\\").replace('"', '\\"')

def struct_fields(name):
    block = re.search(r"pub struct " + name + r" \{(.*?)\n\}", model, re.S).group(1)
    return set(re.findall(r"pub (\w+):", block))

out = []
for name, collection, fn, ty in DEFS:
    fields = struct_fields(ty)
    rows = []
    for key, prop in schema[name]["properties"].items():
        if prop.get("type") not in ("number", "integer"):
            continue
        field = snake(key)
        if field not in fields:
            sys.exit(f"{name}.{key}: no field {field}")
        ui = prop.get("x-semio-ui", {})
        unit = ui.get("unit", "1")
        integer = prop["type"] == "integer"
        step = ui.get("step", 1 if integer else 0.01)
        kind = "Count" if integer else "Signed" if SIGNED_NAME.search(key) else "Positive"
        label = ui["label"]
        rows.append(f'        ({field}, "{key}", "{unit}", {float(step)!r}, {kind}, "{rust(label["en"])}", "{rust(label["de"])}"),')
    out.append(f"/// 🎚️ The numeric leaves of `{collection}` rows.\npub fn {fn}(row: &model::{ty}) -> Vec<Scalar> {{\n    scalars![row;\n" + "\n".join(rows) + "\n    ]\n}\n")
print("\n".join(out))
