"""🔬️ Lists the shapes of an asset document per scale and bone in the three views (id, kind, solid, role, point count, extent of the raw numbers): `python3 dump.py <file.json> <scale> [bone-regex]`; with `--poses` the bone offsets of every pose and the clip names."""
import json, re, sys
doc = json.load(open(sys.argv[1]))["asset"]
if "--poses" in sys.argv:
    poses = doc["poses"]
    for pose in (poses if isinstance(poses, list) else poses.values()):
        print(pose.get("id"), json.dumps(pose.get("bones", {}))[:900])
    clips = doc.get("clips", [])
    for clip in (clips if isinstance(clips, list) else clips.values()):
        print("clip", clip.get("id"), [k for k in clip.keys()])
    sys.exit()
scale = sys.argv[2]
pattern = re.compile(sys.argv[3] if len(sys.argv) > 3 else ".")
geometry = doc["representations"][scale]["geometry"]
bones = [bone["id"] for bone in doc["rig"]["bones"]]
def extent(shape):
    if shape.get("kind") == "poly":
        xs = [p[0] for p in shape["points"]]; ys = [p[1] for p in shape["points"]]
        return f"x[{min(xs):.3f},{max(xs):.3f}] y[{min(ys):.3f},{max(ys):.3f}] n={len(xs)}"
    return " ".join(f"{k}={shape[k]}" for k in ("x", "y", "w", "h", "cx", "cy", "rx", "ry", "r", "x0", "y0", "x1", "y1", "r0", "r1") if k in shape) + (" path" if "d" in shape else "")
for bone in bones:
    if not pattern.search(bone): continue
    for view in ("elevation", "front", "plan"):
        for shape in geometry[view]:
            if shape.get("bone") != bone: continue
            solid = shape.get("solid")
            tag = "flat" if solid is None else json.dumps({k: v for k, v in solid.items() if k != "kind"})[:110] + " " + solid["kind"]
            print(f"{bone:11} {view:9} {shape['id']:22} {shape['kind']:7} {shape.get('role','-'):8} {'tip' if shape.get('followTip') else '   '} {extent(shape)} | {tag}")
