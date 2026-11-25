"""🪕️ Writes the spec of `used-concert-harp`: playing tilts the harp 18 degrees onto the shoulder, and the soundbox was three flat outlines in the front and the plan, carried at one depth, so the contact `shoulder` on the back of the box lay 122 to 128 mm off every plan shape of the box (tolerance 39 mm; measured 2026-11-25 at 39 or 40 subjects per band). The box becomes solids, read from the body of the band (`volumes.<band>.body`), so the views and the 3D body are one thing:
- the back, a half cone with half-elliptic sections, is eight prisms along the box (`sweep: "u"`), each with the section the body has at its upper end (so every prism lies inside the body); all but the uppermost are 6 mm narrower, so they stay under the soundboard in the plan; the uppermost keeps the id and shows the rounded head end the plan drew before; in the front the seven lower prisms are painted before the base (z 1.9, the base has 2), which stands in front of the lower end of the box and must cover it when the harp is tilted (the depth order of a pose only turns pairs round that changed sides against the rest pose, and at rest the two do not overlap);
- the soundboard is its side outline swept over its width, 0.45 m at the foot and 0.13 m at the head (`zTip`), and the centre strip its side outline over 36 mm;
- the foot, a wedge under a half-elliptic face that stands on the base, is its front face: the side outline at the full width (a thin triangle between the soundboard edge and the base) swept over 0.45 m. Its underside lies on the base and is never seen; nested slabs for the body behind the face were tried and dropped (they drew a staircase in front of the base in the played pose and did not print at 1:200).
Which view of which band gets what (variant C, the one written; A and B were tried beside it on 2026-11-25 and are kept for the record: A the plan only, B the plan without any foot):
- plan, all four bands: back, soundboard, centre strip; the foot is drawn at 1:50 and 1:100 and is left out at 1:200 and 1:500, where it lies under the soundboard at rest and at the played tilt would show a strip about 43 mm wide (computed from the tilt, not measured), 0.12 mm on paper at 1:350;
- front, 1:50 and 1:100: all parts. Front, 1:200 and 1:500: unchanged, flat (the first trial with solids there did not print: `soundbox-foot` 0.188 mm at 1:350 and a slab behind it 0.195 mm at 1:200, against 0.2 mm).
A tapering half cone has no solid hint of its own (request of batch MO in `requests.md`); this is the nearest body the hints can state. Run again it writes the same spec: every number comes from the body, the side elevation and the paint of the shapes.
 python3 used-concert-harp.py [A|B|C] [<other output file>]"""
import json, math, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import asset, r6, write

GEOMETRY = ["kind", "x", "y", "w", "h", "r", "points", "closed", "d", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "followTip", "rotationDeg"]
SEGMENTS, INSET = 8, 0.006
variant = sys.argv[1] if len(sys.argv) > 1 else "C"
a = asset("used-concert-harp")
length = next(bone for bone in a["rig"]["bones"] if bone["id"] == "soundbox")["lengthM"]
ops = []
for key, representation in a["representations"].items():
    geometry = representation.get("geometry")
    if geometry is None:
        continue
    fine = representation["validRange"]["maxDenominator"] <= 150
    body = {solid["id"]: solid for solid in a["volumes"][key]["body"]}
    elevation = {shape["id"]: shape for shape in geometry["elevation"]}
    back = body["soundbox-back"]["vertices"][:-2]
    foot_end, head_end = [v for v in back if v[0] == 0], [v for v in back if v[0] == length]
    assert len(foot_end) == len(head_end) == len(back) // 2, key
    foot = body["soundbox-foot"]["vertices"][:-2]
    face, cut = foot[: len(foot) // 2], foot[len(foot) // 2 :]
    assert all(v[0] == 0 for v in face) and all(abs(p[2] - q[2]) < 1e-9 for p, q in zip(face, cut)), key
    outer = next(i for i in range(1, len(face)) if face[i][2] > 0 and face[i][1] >= 0)
    for view in ("front", "plan"):
        if view == "front" and (variant == "A" or not fine):
            continue
        shapes = {shape["id"]: shape for shape in geometry[view]}
        paint = lambda shape: {member: value for member, value in shape.items() if member not in GEOMETRY}
        for i in range(SEGMENTS):
            t = (i + 1) / SEGMENTS
            ring = [[p[1] + (q[1] - p[1]) * t, p[2] + (q[2] - p[2]) * t] for p, q in zip(foot_end, head_end)]
            widest = max(abs(p[1]) for p in ring)
            shrink = 1 if i == SEGMENTS - 1 else (widest - INSET) / widest
            last = i == SEGMENTS - 1
            shape = {**paint(shapes["soundbox-back"]), "id": "soundbox-back" if last else f"soundbox-back-{i + 1}", **({} if last else {"z": 1.9 if view == "front" else 2.99}), "kind": "poly", "points": [[r6(p[0]), r6(p[1] * shrink)] for p in ring], "closed": True, "solid": {"kind": "extruded", "sweep": "u", "z0": r6(length * i / SEGMENTS), "z1": r6(length * t)}}
            ops.append({"op": "shape", "scale": key, "view": view, "id": "soundbox-back", "shape": shape} if last else {"op": "shape-add", "scale": key, "view": view, "after": "soundbox-back", "shape": shape})
        board = elevation["soundboard"]
        ops.append({"op": "shape", "scale": key, "view": view, "id": "soundboard", "shape": {**paint(shapes["soundboard"]), "kind": "poly", "points": board["points"], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.225, "z1": 0.225, "zTip": [-0.065, 0.065]}}})
        if "centre-strip" in shapes and "centre-strip" in elevation:
            ops.append({"op": "shape", "scale": key, "view": view, "id": "centre-strip", "shape": {**paint(shapes["centre-strip"]), "kind": "poly", "points": elevation["centre-strip"]["points"], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": -0.018, "z1": 0.018}}})
        if view == "plan" and (variant != "C" or not fine):
            ops.append({"op": "shape-drop", "scale": key, "view": "plan", "id": "soundbox-foot"})
            continue
        if "soundbox-foot" not in shapes:
            continue
        profile = [[face[0][0], face[0][1]], [face[outer][0], face[outer][1]], [cut[outer][0], cut[outer][1]], [cut[0][0], cut[0][1]]]
        ops.append({"op": "shape", "scale": key, "view": view, "id": "soundbox-foot", "shape": {**paint(shapes["soundbox-foot"]), "kind": "poly", "points": [[r6(p[0]), r6(p[1])] for p in profile], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": r6(-face[outer][2]), "z1": r6(face[outer][2])}}})
if len(sys.argv) > 2:
    with open(sys.argv[2], "w", encoding="utf-8") as file:
        json.dump({"ops": ops}, file, ensure_ascii=False, indent=1)
    print(f"variant {variant}: {len(ops)} ops in {sys.argv[2]}")
else:
    write("used-concert-harp", {"ops": ops})
