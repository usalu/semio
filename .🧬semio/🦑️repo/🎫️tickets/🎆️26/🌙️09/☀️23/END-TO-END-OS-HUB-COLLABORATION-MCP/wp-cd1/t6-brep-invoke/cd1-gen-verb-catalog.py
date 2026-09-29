"""🧾️ CD1 one-off: derives the brep_invoke verb catalog (args/result/kernel op) from the dispatcher source with the
transform arms inserted, and merges the hand-written en/de terminology. Output: brep-invoke-verbs.json (payload)."""
import json, re, sys, pathlib
HERE = pathlib.Path(__file__).parent
ROOT = pathlib.Path("/Users/ueli/Documents/semio")
SRC = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs"
text = SRC.read_text(encoding="utf-8")
arms = (HERE / "transform-arms.rs.txt").read_text(encoding="utf-8")
anchor = '        "sewFaces" => {\n'
if '"translate" => {' not in text:
    assert text.count(anchor) == 1
    text = text.replace(anchor, arms + anchor)
body = text[text.index("fn brep_invoke_inner("):text.index("/// 🌐️ `brep_invoke` implementation shared")]
parts = re.split(r'\n        "([A-Za-z]+)" => \{\n', body)
TYPES = {"f64": "number", "f64_or": "number", "usize": "integer", "bool_or": "boolean", "string": "string", "vec3": "vec3", "points": "points", "handle": "handle", "handles": "handles"}
RESULTS = [("handles_result", "handles"), ("handle_result", "handle"), ("number_result", "number"), ("vec3_result", "vec3"), ("string_result", "string"), ("mesh_result", "mesh"), ("topology_result", "topology"), ("unit_result", "unit")]
LABELS = {
    "box": ("Box", "Quader"), "sphere": ("Sphere", "Kugel"), "cylinder": ("Cylinder", "Zylinder"), "cone": ("Cone", "Kegel"),
    "lineCurve": ("Line", "Linie"), "circleCurve": ("Circle", "Kreis"), "arcCurve": ("Arc", "Bogen"), "ellipseCurve": ("Ellipse", "Ellipse"),
    "interpolateCurve": ("Interpolated Curve", "Interpolierte Kurve"), "approximateCurve": ("Approximated Curve", "Approximierte Kurve"),
    "polylineWire": ("Polyline", "Polylinie"), "rectangleWire": ("Rectangle", "Rechteck"),
    "planarFaceFromPoints": ("Planar Face From Points", "Ebene Fläche aus Punkten"), "planarFaceFromWire": ("Planar Face From Wire", "Ebene Fläche aus Drahtzug"),
    "extrudeWire": ("Extrude Wire", "Drahtzug extrudieren"), "extrude": ("Extrude", "Extrudieren"), "revolve": ("Revolve", "Rotationskörper"),
    "loft": ("Loft", "Ausformung"), "sweep": ("Sweep", "Austragung"), "thickenFace": ("Thicken Face", "Fläche aufdicken"), "offsetFace": ("Offset Face", "Fläche versetzen"),
    "fuse": ("Fuse", "Vereinigen"), "cut": ("Cut", "Abziehen"), "intersect": ("Intersect", "Schnittmenge"),
    "translate": ("Translate", "Verschieben"), "rotate": ("Rotate", "Drehen"), "rotateAbout": ("Rotate About Point", "Um Punkt drehen"), "scale": ("Scale", "Skalieren"), "mirror": ("Mirror", "Spiegeln"),
    "sewFaces": ("Sew Faces", "Flächen vernähen"), "faceFromWire": ("Face From Wire", "Fläche aus Drahtzug"), "healSolid": ("Heal Solid", "Körper reparieren"),
    "volume": ("Volume", "Volumen"), "area": ("Area", "Flächeninhalt"), "length": ("Length", "Länge"), "centerOfMass": ("Center Of Mass", "Schwerpunkt"), "distance": ("Distance", "Abstand"),
    "deconstruct": ("Deconstruct", "Zerlegen"), "tessellate": ("Tessellate", "Tessellieren"), "exportStep": ("Export STEP", "STEP exportieren"), "importStep": ("Import STEP", "STEP importieren"),
    "dispose": ("Dispose", "Freigeben"), "retain": ("Retain", "Behalten"),
}
verbs = []
for i in range(1, len(parts), 2):
    method, arm = parts[i], parts[i + 1]
    arm = arm[: arm.index("\n        }")] if "\n        }" in arm else arm
    op = re.search(r"guard\s*\.\s*([a-z_]+)\(", arm).group(1)
    args = []
    for m in re.finditer(r'arg_(f64_or|f64|usize|bool_or|string|vec3|points|handles|handle)\(&args, "([A-Za-z]+)"(?:, ([^)]+))?\)', arm):
        kind, name, default = m.groups()
        row = {"name": name, "type": TYPES[kind]}
        if default is not None:
            row["default"] = json.loads(default.replace("1e-", "1e-")) if default not in ("true", "false") else default == "true"
        args.append(row)
    result = next(r for key, r in RESULTS if key in arm)
    en, de = LABELS[method]
    verbs.append({"method": method, "kernelOperation": op, "args": args, "result": result, "label": {"en": en, "de": de}})
assert len(verbs) == len(LABELS), (len(verbs), sorted(set(LABELS) - {v["method"] for v in verbs}))
catalog = {"$schema": "./🧬️schema/🔣️.json", "schema": "semio.flow.brep-invoke.verbs/v1", "verbs": verbs}
out = HERE / "brep-invoke-verbs.json"
lines = ['{', '  "$schema": "./🧬️schema/🔣️.json",', '  "schema": "semio.flow.brep-invoke.verbs/v1",', '  "verbs": [']
for n, v in enumerate(verbs):
    row = json.dumps(v, ensure_ascii=False, separators=(", ", ": "))
    row = re.sub(r'"default": ([0-9.]+e-[0-9]+)', lambda m: '"default": ' + format(float(m.group(1)), "f").rstrip("0"), row)
    lines.append("    " + row + ("," if n + 1 < len(verbs) else ""))
lines += ["  ]", "}", ""]
out.write_text("\n".join(lines), encoding="utf-8")
json.loads(out.read_text(encoding="utf-8"))
print(len(verbs), "verbs ->", out)
