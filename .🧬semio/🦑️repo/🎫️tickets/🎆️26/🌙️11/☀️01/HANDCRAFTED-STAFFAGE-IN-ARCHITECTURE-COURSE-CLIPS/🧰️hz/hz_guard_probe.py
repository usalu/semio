"""🧪 Probe for proposed staffage guard rules over architecture clip scripts; prints violations, changes nothing."""

import ast
import json
import re
import sys
from pathlib import Path

ARCH = Path("C:/git/semio/🎓️teaching/🏛️architecture")
VIS = ARCH / "⚡️energy/🏗️authoring/🎨️visuals"
OBJECT = re.compile(
    r"(derrick|hob|pot|kettle|valve|blind|radiator|lamp|person|occupant|worker|desk|chair|laptop|meter|tank|boiler|"
    r"pump|fan|mvhr|grille|toaster|heater|plant|tree|shower|thermo|clock|table|sink|cooker|hood|humidifier|"
    r"compressor|evaporator|condenser|wheel|exchanger|cylinder|device|figure|crate|rack|server|cooler|occupant|"
    r"stand|walker|sleeper|student|cook|blackboard|window|door|lattice|tealight|pendant|bulb|unit)",
    re.IGNORECASE,
)
PRIMITIVES = {"Circle", "Rectangle", "RoundedRectangle", "Square", "Polygon", "Ellipse", "Dot", "Line", "Arc",
              "SurroundingRectangle", "RegularPolygon", "Triangle", "Annulus"}
STAFFAGE = {"staffage", "fitted_staffage", "staffage_motion", "_cast", "build", "_staffage", "_staffage_motion",
            "_fitted_staffage", "pendant_lamp", "watt_anchor", "_use_cast", "_figure"}
THERMAL_EXEMPT = {"desk", "table", "chair", "blackboard", "crates", "tree", "blind", "office-chair"}
DARK = {"dark-line"}
manifest = json.loads((VIS / "🧍️staffage/🔣️.json").read_text(encoding="utf-8"))["casts"]
views = {k: v["view"] for k, v in manifest.items()}


def calls(node):
    names = set()
    for sub in ast.walk(node):
        if isinstance(sub, ast.Call):
            fn = sub.func
            if isinstance(fn, ast.Name):
                names.add(fn.id)
            elif isinstance(fn, ast.Attribute):
                names.add(fn.attr)
    return names


clips = [p for p in ARCH.rglob("🐍️.py") if "🎬️clip" in p.parts and ".venv" not in p.parts and "🧪️tests" not in p.parts]
out = []
out.append("## Rule A: object-named function drawing primitives without a staffage call")
for path in sorted(clips):
    rel = path.relative_to(ARCH).as_posix()
    tree = ast.parse(path.read_text(encoding="utf-8"))
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and OBJECT.search(node.name):
            c = calls(node)
            if c & PRIMITIVES and not c & STAFFAGE:
                out.append(f"{rel}:{node.lineno} {node.name} primitives={sorted(c & PRIMITIVES)}")
out.append("")
out.append("## Rule B: energy clips referencing a dark-line cast outside the furniture allowlist (literal ids)")
import re as _re
lit = _re.compile(r'"([a-z][a-z0-9]*(?:-[a-z0-9]+)+)"')
for path in sorted(clips):
    rel = path.relative_to(ARCH).as_posix()
    if "⚡️energy" not in rel:
        continue
    for ln, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        for m in lit.finditer(line):
            cid = m.group(1)
            if views.get(cid) in DARK and not any(f in cid for f in THERMAL_EXEMPT):
                out.append(f"{rel}:{ln} {cid}")
out.append("")
out.append("## Rule C: manifest dark-line casts whose id is not furniture (for review)")
for k, v in sorted(views.items()):
    if v in DARK and not any(f in k for f in THERMAL_EXEMPT):
        out.append(f"{k} asset={manifest[k]['asset']}")
print("\n".join(out))
