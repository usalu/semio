"""🦴 Round 5, furniture/bar-stool: the round-3 construction on a corrected rig, written as the same candidate document (`🗑️generated/r3-bar-stool/🔣️.json`) for `r3-bar-stool-apply.py`.

Rig at the root: the `frame` bone stands on the stool axis from the floor to the top of the seat pan (90 degrees, 0.686 m) instead of lying 0.70 m ahead along the floor; `ground` is a zero-length marker instead of a 0.42 m bar along the floor; `pad` runs from the swivel axis to the cushion rim (0.18 m) instead of 0.36 m ahead. Seat and pad keep their world rest frames (their offsets are restated in the turned frame), so seat, pad and backrest solids and shapes are unchanged; the frame's own solids, repeats, weld ellipsoids, elevation shapes and anchors are restated in the turned frame; the cushion slides along the frame axis (`axisDeg` 0, poses `dx`).

Usage: python3 r5-bar-stool-build.py
@see ./r3-bar-stool-build.py
@see ./r5-brief.md
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("bar_stool_build_r3", os.path.join(HERE, "r3-bar-stool-build.py"))
B = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(B)
D, M, K = B.D, B.D.M, B.D.K
R = K.rnd
T = K.text

FRAME_DEG = 90.0
K.FRAMES["frame"] = ((0.0, 0.0, 0.0), FRAME_DEG)
M.FOUR["rotationAxis"] = [1.0, 0.0, 0.0]
_ellipsoid = K.ellipsoid


def ellipsoid(solid_id, bone, material, role, tone, centre, radii):
    """🥚 An ellipsoid stated with world-axis radii: on the turned frame bone its u axis is the world vertical."""
    rx, ry, rz = radii
    return _ellipsoid(solid_id, bone, material, role, tone, centre, (ry, rx, rz) if K.FRAMES[bone][1] == 90.0 else (rx, ry, rz))


K.ellipsoid = ellipsoid


def up(x, y):
    """↕ A world side-view point (ahead, up) in the turned frame bone: u up, v backward."""
    return R(y), R(-x)


def turn_shape(s):
    """🔁 An elevation shape of the frame bone restated from world coordinates into the turned frame."""
    s = dict(s)
    if s["kind"] == "capsule":
        s["x0"], s["y0"] = up(s["x0"], s["y0"])
        s["x1"], s["y1"] = up(s["x1"], s["y1"])
    elif s["kind"] == "poly":
        s["points"] = [list(up(x, y)) for x, y in s["points"]]
    else:
        raise SystemExit(f"frame elevation shape {s['id']} of kind {s['kind']} has no restatement")
    return s


_geometry = D.geometry


def geometry(scale):
    g = _geometry(scale)
    g["elevation"] = [turn_shape(s) if s["bone"] == "frame" else s for s in g["elevation"]]
    return g


D.geometry = geometry
_roots = B.roots


def roots(a):
    """🌱 Round-3 root fixes, then the round-5 rig: frame on the stool axis, ground a marker, pad from the axis to the rim; children, anchors and poses restated in the turned frame."""
    _roots(a)
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones["ground"]["lengthM"] = 0
    bones["ground"]["title"] = T("Floor marker at the foot of the stool axis", "Bodenmarke am Fuß der Hockerachse")
    frame = bones["frame"]
    frame["angleDeg"], frame["lengthM"] = FRAME_DEG, K.PAN_TOP
    frame["mass"]["comX"], frame["mass"]["comY"] = frame["mass"]["comY"], 0
    seat = bones["seat"]
    seat["x"], seat["y"], seat["angleDeg"] = up(0.0, K.SEAT_JOINT_Y) + (0.0,)
    pad = bones["pad"]
    pad["x"], pad["y"], pad["angleDeg"], pad["lengthM"] = R(K.PAD_JOINT_Y), 0.0, -FRAME_DEG, K.SEAT_R
    pad["joint"]["axisDeg"] = 0
    for anchor in a["anchors"]:
        if anchor["bone"] == "frame":
            world_y = 0.0 if anchor["id"].startswith("foot-") else anchor["y"]
            anchor["x"], anchor["y"] = up(anchor["x"], world_y)
    for pose in a["poses"]:
        off = pose["bones"].get("pad")
        if off is not None and "dy" in off:
            off["dx"] = off.pop("dy")
    st = a["structure"]["assumptions"]
    st[0]["en"] = st[0]["en"].replace("the foam cushion sits on a prismatic joint", "the foam cushion sits on a prismatic joint sliding down the stool axis (the frame bone, floor to pan top)")
    st[0]["de"] = st[0]["de"].replace("das Schaumpolster sitzt auf einem Schiebegelenk", "das Schaumpolster sitzt auf einem Schiebegelenk entlang der Hockerachse (Gestellknochen, Boden bis Pfannenoberkante)")


B.roots = roots

if __name__ == "__main__":
    B.main()
