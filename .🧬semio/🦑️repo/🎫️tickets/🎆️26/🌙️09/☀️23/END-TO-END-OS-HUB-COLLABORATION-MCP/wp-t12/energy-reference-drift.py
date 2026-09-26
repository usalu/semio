#!/usr/bin/env python3
"""🔋️ Prepared patch (rule 20, apply after W2's `--packages all`): `🏛️mutate-energy-model-1`'s Python reference fell
behind two model members the Rust `Model` gained — `Material.roughness` (carried by `create-material`'s payload) and
`Fenestration.vertices_m` (an aperture's own polygon; `create-fenestration` creates it empty, `replace-fenestration-
vertices` shapes it). 5 rows were red (create-material, create-fenestration, and the inverses of delete-material,
delete-fenestration, delete-surface). The reference now writes both members on creation and, when undoing a delete,
re-creates the aperture FIRST and puts a non-empty polygon back SECOND (this reference applies inverse steps in list
order; the Rust store replays its list reversed, which is why its list is written the other way round). Also drops the
dead `if false { return Vec::new(); }` block two Rust delete inverses (fenestration, shading surface) still carry.
Usage: energy-reference-drift.py --dry-run | --write [--root <dir>]"""
import sys
from pathlib import Path

root = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
PATH = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🏛️mutate-energy-model-1/🐍️.py"
WINDOW = '''{"id": {0}["id"], "name": {0}["name"], "surfaceId": {0}["surface_id"], "uValueWM2k": {0}["u_value_w_m2k"], "shgc": {0}["shgc"], "vlt": {0}["vlt"], "areaM2": {0}["area_m2"], "heightM": {0}["height_m"], "sillHeightM": {0}["sill_height_m"], "frameConductanceWK": {0}["frame_conductance_w_k"], "dividerConductanceWK": {0}["divider_conductance_w_k"], "overhangDepthM": {0}["overhang_depth_m"], "overhangOffsetM": {0}["overhang_offset_m"], "finDepthM": {0}["fin_depth_m"], "finOffsetM": {0}["fin_offset_m"], "glazingConstructionId": {0}["glazing_construction_id"]}'''
PAIRS = [
    ('"fin_offset_m": payload["finOffsetM"], "glazing_construction_id": payload["glazingConstructionId"]}',
     '"fin_offset_m": payload["finOffsetM"], "glazing_construction_id": payload["glazingConstructionId"], "vertices_m": []}'),
    ('"name": payload["name"], "thickness_m": payload["thicknessM"],',
     '"name": payload["name"], "roughness": payload["roughness"], "thickness_m": payload["thicknessM"],'),
    ('"name": item["name"], "thicknessM": item["thickness_m"],',
     '"name": item["name"], "roughness": item["roughness"], "thicknessM": item["thickness_m"],'),
    ('    return [("create-fenestration", ' + WINDOW.replace("{0}", "item") + ')]\n',
     '    return [("create-fenestration", ' + WINDOW.replace("{0}", "item") + ')] + ([("replace-fenestration-vertices", {"id": item["id"], "newVerticesM": item["vertices_m"]})] if item["vertices_m"] else [])\n'),
    ('            steps.append(("create-fenestration", ' + WINDOW.replace("{0}", "window") + '))\n',
     '            steps.append(("create-fenestration", ' + WINDOW.replace("{0}", "window") + '))\n            if window["vertices_m"]:\n                steps.append(("replace-fenestration-vertices", {"id": window["id"], "newVerticesM": window["vertices_m"]}))\n'),
]
write = "--write" in sys.argv
source = (root / PATH).read_text(encoding="utf-8")
problems = ["already applied"] if '"glazing_construction_id": payload["glazingConstructionId"], "vertices_m": []}' in source else []
for old, new in PAIRS:
    if source.count(old) != 1:
        problems.append(f"{source.count(old)} × {old[:70]!r}")
        continue
    source = source.replace(old, new)
edits = {PATH: source}
DEAD = "    if false {\n        return Vec::new();\n    }\n"
for leaf in ("🚪️delete-fenestration", "🪵️delete-shading-surface"):
    inverse = f"✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{leaf}/↩️inverse/🦀️.rs"
    text = (root / inverse).read_text(encoding="utf-8")
    if text.count(DEAD) != 1:
        problems.append(f"{inverse}: {text.count(DEAD)} dead `if false` blocks")
        continue
    edits[inverse] = text.replace(DEAD, "")
print(f"files={len(edits)} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, text in edits.items():
        (root / path).write_text(text, encoding="utf-8")
