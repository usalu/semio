"""🔧️ Tool parts stated once: a part is a bone-local profile with its 2D solid placement (revolved about the bone axis or extruded across the bone plane) and the 3D solid of the same body; `emit` writes the shapes of the three views and the body, thermal and structure solids of one scale.

Revolved parts give a half profile (v >= 0) drawn mirrored and a loft of round sections in 3D; plates give a full (u, v) profile with a lateral thickness and an extrude in 3D; both are exact projections of the same body in every pose, because the evaluator builds the 2D solid in 3D from its bone frame.
"""
import math

import lib


def revolved(pid, bone, half, region, material, tone, z, role="body", slender=False, sections=None, pen=None, filled=True):
    """A part round about the bone axis: `half` is [(u, r)] from the first to the last station; 2D is the closed half profile, 3D a loft through the same stations (or a capsule for two equal radii)."""
    prof = [(half[0][0], 0.0)] + [(u, r) for u, r in half] + [(half[-1][0], 0.0)]
    return {"id": pid, "bone": bone, "kind": "revolved", "half": half, "profile": prof, "region": region, "material": material, "tone": tone, "z": z, "role": role, "slender": slender, "pen": pen, "filled": filled}


def plate(pid, bone, profile, w0, w1, region, material, tone, z, role="body", sheet=False, pen=None, filled=True):
    """A flat part: `profile` [(u, v)] closed in the bone plane, from `w0` to `w1` across it (bone-local w)."""
    return {"id": pid, "bone": bone, "kind": "plate", "profile": profile, "w0": w0, "w1": w1, "region": region, "material": material, "tone": tone, "z": z, "role": role, "sheet": sheet, "pen": pen, "filled": filled}


def solid2d(part, rest_z):
    if part["kind"] == "revolved":
        return {"kind": "revolved", "axisBone": part["bone"], "axis": "u", "axisX": 0, "mirror": True}
    return {"kind": "extruded", "sweep": "w", "z0": lib.r5(rest_z[part["bone"]] + part["w0"]), "z1": lib.r5(rest_z[part["bone"]] + part["w1"])}


ROT = [0.5, 0.5, 0.5, 0.5]


def solid3d(part):
    base = {"id": part["id"], "bone": part["bone"], "material": part["material"], "role": part["role"] if part["role"] in ("body", "detail") else "detail", "tone": part["tone"]}
    if part["kind"] == "revolved":
        h = part["half"]
        if len(h) == 2 and abs(h[0][1] - h[1][1]) < 1e-9:
            s = {**base, "kind": "capsule", "p0": [lib.r5(h[0][0] + h[0][1]), 0, 0], "p1": [lib.r5(h[1][0] - h[1][1]), 0, 0], "r0": h[0][1], "r1": h[1][1]}
        else:
            s = {**base, "kind": "loft", "sections": [{"at": [lib.r5(u), 0, 0], "rx": lib.r5(max(r, 0.0015)), "ry": lib.r5(max(r, 0.0015)), "exponent": 2, "rotation": ROT} for u, r in h], "capStart": True, "capEnd": True}
        if part["slender"]:
            s["slender"] = True
        return s
    s = {**base, "kind": "extrude", "plane": "xy", "profile": [[lib.r5(u), lib.r5(v)] for u, v in part["profile"]], "from": lib.r5(part["w0"]), "to": lib.r5(part["w1"])}
    if part["sheet"]:
        s["sheet"] = True
    return s


def emit(a, scale, parts, rest_z, member_of=None, mass_marks=()):
    """✍️ Writes the parts of one scale: shapes in the three views, body solids, thermal solids (`of` the body solid, region of the part) and structure member solids (`of` the body solid, member = bone, or the bone named by `member_of`)."""
    for p in parts:
        spec = {"id": p["id"], "bone": p["bone"], "role": p["role"], "z": p["z"], "tone": p["tone"], "region": p["region"], "material": p["material"], "profile": p["profile"], "solid": solid2d(p, rest_z), "filled": p["filled"]}
        if p["pen"] is not None:
            spec["penMm"] = p["pen"]
        lib.add_shapes(a, scale, lib.tool_shapes(spec))
    lib.add_solids(a, scale, "body", [solid3d(p) for p in parts])
    lib.add_solids(a, scale, "thermal", [{"id": "th-" + p["id"], "of": p["id"], "region": p["region"]} for p in parts])
    st = [{"id": "st-" + p["id"], "of": p["id"], "member": (member_of or {}).get(p["id"], p["bone"]), "part": "member"} for p in parts]
    lib.add_solids(a, scale, "structure", st + list(mass_marks))


def mass_mark(mid, bone, at, r, material):
    return {"id": mid, "kind": "ellipsoid", "bone": bone, "material": material, "role": "body", "tone": 0.5, "radii": [r, r, r], "transform": {"translation": [lib.r5(x) for x in at]}, "member": bone, "part": "mass"}


def arc(cu, cv, r, a0, a1, n):
    return [(cu + r * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cv + r * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]
