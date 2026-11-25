"""✂️ Cuts a straight stroke of a turning bone in two, in all three views: `python3 halve.py <id> <scale> <bone> <shape id> [--at 0.5] [--views elevation,front,plan] [--overlay] [--ops file]` (`--at`: the share of the stroke the inner piece keeps; `--views`: the views it is cut in, the elevation always among them). A flat open stroke that shares its id with its elevation twin is carried in front and plan at ONE depth, the middle of the twin, so in a pose that turns the bone its far end stays behind the elevation (cause D of `multiview-diagnosis.md`); as a `solid` the stroke would enter the silhouette guard as a body the volume does not have there. Cut at its middle into `<shape id>` (the inner half) and `<shape id>-tip` (the outer half, the same look, drawn right after it), each half has a twin of its own and is carried at the middle depth of that half: the drawing at rest is the same line, the depth error in a turned pose halves. With `--overlay` the stroke itself stays as it is and the outer piece is laid over it as a second stroke on the same line (for a drawing whose strokes were fitted cell by cell to the silhouette guard, as the 1:50 strokes of `miscanthus-grass` were by SV-2: every cut of the stroke itself moved the guard's reference there from 13.3 to 17.0 mm, limit 15 mm; the overlay leaves it at 13.3 mm). Works on strokes that are a `path` of one `M … L …` segment in elevation, front and plan; writes `set` and `add` ops into `ops/<id>.json` for `apply.ts`; a stroke that already has its `-tip` (in the file or in the ops) is left alone, so a second run changes nothing."""
import json, os, re, sys

S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
HERE = os.path.dirname(os.path.abspath(__file__))
asset_id, scale, bone, shape_id = sys.argv[1:5]
out = sys.argv[sys.argv.index("--ops") + 1] if "--ops" in sys.argv else os.path.join(HERE, "ops", f"{asset_id}.json")
folder = next(os.path.join(S, "🗂️catalog", c, asset_id) for c in os.listdir(os.path.join(S, "🗂️catalog")) if os.path.isdir(os.path.join(S, "🗂️catalog", c, asset_id)))
geometry = json.load(open(os.path.join(folder, "🔣️.json")))["asset"]["representations"][scale]["geometry"]
ops = json.load(open(out)) if os.path.exists(out) else []
at = float(sys.argv[sys.argv.index("--at") + 1]) if "--at" in sys.argv else 0.5
views = sys.argv[sys.argv.index("--views") + 1].split(",") if "--views" in sys.argv else ["elevation", "front", "plan"]
tip = f"{shape_id}-tip"
if any(op.get("shape", {}).get("id") == tip and op.get("scale") == scale for op in ops) or any(shape["id"] == tip and shape["bone"] == bone for shape in geometry["elevation"]):
    print(f"{asset_id} 1:{scale} {shape_id}: already cut, nothing to do")
    sys.exit(0)
added = []
for view in views:
    shape = next(s for s in geometry[view] if s["id"] == shape_id and s["bone"] == bone)
    match = re.fullmatch(r"M (\S+) (\S+) L (\S+) (\S+)", shape["d"]) if shape.get("kind") == "path" and "solid" not in shape else None
    if match is None:
        sys.exit(f"{asset_id} 1:{scale} {view} {shape_id}: not a flat stroke of one segment")
    x0, y0, x1, y1 = map(float, match.groups())
    mx, my = round(x0 + (x1 - x0) * at, 4), round(y0 + (y1 - y0) * at, 4)
    following = geometry[view][geometry[view].index(shape) + 1]["id"] if geometry[view].index(shape) + 1 < len(geometry[view]) else None
    if "--overlay" not in sys.argv:
        added.append({"op": "set", "scale": scale, "view": view, "id": shape_id, "bone": bone, "set": {"d": f"M {x0:g} {y0:g} L {mx:g} {my:g}"}})
    op = {"op": "add", "scale": scale, "view": view, "shape": {**shape, "id": tip, "d": f"M {mx:g} {my:g} L {x1:g} {y1:g}"}}
    if following is not None:
        op["before"] = following
    added.append(op)
ops += added
open(out, "w").write("[\n" + ",\n".join("  " + json.dumps(op, ensure_ascii=False) for op in ops) + "\n]\n")
print(f"{asset_id} 1:{scale} {shape_id}: {'outer piece laid over it' if '--overlay' in sys.argv else 'cut in two'} at {at:g} in {' and '.join(views)}, {len(ops)} ops")
