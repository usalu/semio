"""⚖️ Hand statics of a lifted part: weight and centre of mass of the subtree of the lifted bone (rest pose, forward kinematics of the rig) and the share of each lifting anchor, as an independent check of `lift-loads.ts`.

Usage: python3 lift_statics.py <slug> <lifted bone> <anchor> [<anchor> ...] [--pose <id>] [--pivot]
One anchor carries everything, or with `--pivot` shares with the joint origin of the lifted bone (a part lowered about its hinge and resting on one point); two anchors share by their lever arms about the centre of mass along the line between them; four anchors are solved as the two pairs nearest to each other.
"""
import json
import math
import os
import sys

G = 9.80665
args = [a for a in sys.argv[1:] if not a.startswith("--")]
pose_id = sys.argv[sys.argv.index("--pose") + 1] if "--pose" in sys.argv else None
if pose_id:
    args.remove(pose_id)
slug, lifted, names = args[0], args[1], args[2:]
asset = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "assets", slug, "🔣️.json")))["asset"]
bones = {b["id"]: b for b in asset["rig"]["bones"]}
offsets = next(p for p in asset["poses"] if p["id"] == pose_id)["bones"] if pose_id else {}


def frame(bid):
    """🦴 World origin, angle and depth of a bone: the parent's frame moved by the bone's rest position and the pose offsets."""
    b = bones[bid]
    o = offsets.get(bid, {})
    if b.get("parent") is None:
        return b.get("x", 0) + o.get("dx", 0), b.get("y", 0) + o.get("dy", 0), b.get("angleDeg", 0) + o.get("angleDeg", 0), b.get("z", 0)
    px, py, pa, pz = frame(b["parent"])
    c, s = math.cos(math.radians(pa)), math.sin(math.radians(pa))
    x, y = b.get("x", 0) + o.get("dx", 0), b.get("y", 0) + o.get("dy", 0)
    return px + c * x - s * y, py + s * x + c * y, pa + b.get("angleDeg", 0) + o.get("angleDeg", 0), pz + b.get("z", 0)


def world(bid, u, v, w=0):
    """📍 A bone-local point in world axes."""
    x, y, a, z = frame(bid)
    c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
    return x + c * u - s * v, y + s * u + c * v, z + w


def inside(bid):
    """🌿 Whether a bone belongs to the lifted subtree."""
    return bid is not None and (bid == lifted or inside(bones[bid].get("parent")))


kg = cx = cz = 0.0
for b in bones.values():
    m = b.get("mass")
    if not m or not inside(b["id"]):
        continue
    u = m.get("comX", b.get("lengthM", 0) / 2 if m.get("distribution") == "uniform" else 0)
    x, _, z = world(b["id"], u, m.get("comY", 0), m.get("comZ", 0))
    kg, cx, cz = kg + m["kg"], cx + m["kg"] * x, cz + m["kg"] * z
cx, cz = cx / kg, cz / kg
pts = {}
for n in names:
    a = next(a for a in asset["anchors"] if a["id"] == n)
    x, _, z = world(a["bone"], a.get("x", 0), a.get("y", 0), a.get("z", 0))
    pts[n] = (x, z)
print(f"{slug}: subtree of {lifted} {kg:.3f} kg = {kg * G:.1f} N, centre of mass x {cx:.4f} z {cz:.4f}")


def share(p, q, c):
    """➗ The fraction of a load at c carried at p when p and q carry it."""
    d = (q[0] - p[0]) ** 2 + (q[1] - p[1]) ** 2
    return 0.5 if d < 1e-12 else 1 - ((c[0] - p[0]) * (q[0] - p[0]) + (c[1] - p[1]) * (q[1] - p[1])) / d


if len(names) == 1 and "--pivot" in sys.argv:
    px, _, _, pz = frame(lifted)
    f = share(pts[names[0]], (px, pz), (cx, cz))
    out = {names[0]: f}
    print(f"  joint of {lifted}: at x {px:.4f} z {pz:.4f}  share {1 - f:.4f}  fyN {(1 - f) * kg * G:.1f}")
elif len(names) == 1:
    out = {names[0]: 1.0}
elif len(names) == 2:
    f = share(pts[names[0]], pts[names[1]], (cx, cz))
    out = {names[0]: f, names[1]: 1 - f}
else:
    a, b, c, d = names
    m1 = ((pts[a][0] + pts[b][0]) / 2, (pts[a][1] + pts[b][1]) / 2)
    m2 = ((pts[c][0] + pts[d][0]) / 2, (pts[c][1] + pts[d][1]) / 2)
    f = share(m1, m2, (cx, cz))
    fa, fc = share(pts[a], pts[b], (cx, cz)), share(pts[c], pts[d], (cx, cz))
    out = {a: f * fa, b: f * (1 - fa), c: (1 - f) * fc, d: (1 - f) * (1 - fc)}
for n, f in out.items():
    print(f"  {n}: at x {pts[n][0]:.4f} z {pts[n][1]:.4f}  share {f:.4f}  fyN {f * kg * G:.1f}")
