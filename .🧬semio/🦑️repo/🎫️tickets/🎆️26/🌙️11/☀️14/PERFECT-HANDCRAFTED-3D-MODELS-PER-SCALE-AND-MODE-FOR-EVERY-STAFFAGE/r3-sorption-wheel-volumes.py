"""🧊 Round 3, machinery/sorption-wheel: the handcrafted solids of 1:50, 1:100 and 1:200 (body, thermal, structure, decisions), every part stated by hand from the measures of `r3-sorption-wheel-geo.py`; writes `volumes/machinery/sorption-wheel.json`.

Usage: python3 r3-sorption-wheel-volumes.py
"""
import importlib.util
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("sw_geo", os.path.join(HERE, "r3-sorption-wheel-geo.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)
_texts = importlib.util.spec_from_file_location("sw_texts", os.path.join(HERE, "r3-sorption-wheel-texts.py"))
X = importlib.util.module_from_spec(_texts)
_texts.loader.exec_module(X)

Z = [0.0, 0.0, 1.0]
C_CASING = [0.0, G.CY, 0.0]
C_LOCAL = [0.0, 0.0, 0.0]
STEP = 2 * math.pi / G.SPOKES


def base(id, bone, material, tone, role="body"):
    """🧱 The members every solid starts with."""
    return {"id": id, "bone": bone, "material": material, "role": role, "tone": tone}


def revolve(id, bone, material, tone, profile, origin, **kw):
    """🌀 A revolve about the wheel axis; profile points are [z, radius]."""
    out = base(id, bone, material, tone)
    out.update({"kind": "revolve", "profile": [G.pt(a, r) for a, r in profile], "axis": Z, "origin": origin})
    out.update(kw)
    return out


def extrude(id, bone, material, tone, profile, z0, z1, plane="xy", holes=None, **kw):
    """🧇 An extrude of a hand-stated outline between two positions on the third axis."""
    out = base(id, bone, material, tone)
    out.update({"kind": "extrude", "profile": [G.pt(*p) for p in profile], "plane": plane, "from": G.r6(z0), "to": G.r6(z1)})
    if holes:
        out["holes"] = [[G.pt(*p) for p in hole] for hole in holes]
    out.update(kw)
    return out


def box(id, bone, material, tone, size, at, **kw):
    """📦 A box of size (u, v, w) centred at `at`."""
    out = base(id, bone, material, tone)
    out.update({"kind": "box", "size": [G.r6(v) for v in size], "transform": {"translation": [G.r6(v) for v in at]}})
    out.update(kw)
    return out


def loop(points):
    """🔁 A closed revolve loop (first point repeated)."""
    return points + [points[0]]


def turn8():
    """🎡 Eight copies about the wheel axis."""
    return {"count": G.SPOKES, "rotationRad": round(STEP, 9), "rotationAxis": Z, "origin": C_LOCAL}


def body50():
    """🏭 1:50: every assembly of the machine."""
    b, f, t, hd = G.R_BAND, G.R_FLANGE, G.T_SHEET, G.HALF_DEPTH
    my = G.motor_y(50)
    outer, inner = G.belt_loops(50)
    guard = G.guard_profile(50)
    x_right = guard[0][0]
    out = [
        revolve("band-near", "casing", G.STEEL, 0.35, loop([[-hd, f], [-hd, b], [0.022, b], [0.022, b - t], [-hd + t, b - t], [-hd + t, f]]), C_CASING),
        revolve("band-far", "casing", G.STEEL, 0.35, loop([[hd - t, f], [hd - t, b - t], [0.038, b - t], [0.038, b], [hd, b], [hd, f]]), C_CASING),
        extrude("band-belt-strip", "casing", G.STEEL, 0.35, G.sector(b - t, b, -17.0, 286.0, 3.0, 0.0, G.CY), 0.022, 0.038),
        revolve("rim-seal-near", "casing", G.RUBBER, 0.12, loop([[-0.088, 0.285], [-0.097, 0.285], [-0.097, 0.292], [-0.109, 0.292], [-0.109, 0.3], [-0.088, 0.3]]), C_CASING, role="detail"),
        revolve("rim-seal-far", "casing", G.RUBBER, 0.12, loop([[0.088, 0.285], [0.088, 0.3], [0.109, 0.3], [0.109, 0.292], [0.097, 0.292], [0.097, 0.285]]), C_CASING, role="detail"),
        extrude("beam-near", "casing", G.STEEL, 0.3, [[-0.109, -0.02], [-0.109, 0.02], [-0.101, 0.02], [-0.101, 0.0185], [-0.1075, 0.0185], [-0.1075, -0.0185], [-0.101, -0.0185], [-0.101, -0.02]], G.CY - 0.29, G.CY + 0.29, plane="zx"),
        extrude("beam-far", "casing", G.STEEL, 0.3, [[0.109, -0.02], [0.101, -0.02], [0.101, -0.0185], [0.1075, -0.0185], [0.1075, 0.0185], [0.101, 0.0185], [0.101, 0.02], [0.109, 0.02]], G.CY - 0.29, G.CY + 0.29, plane="zx"),
        revolve("shaft", "casing", G.STEEL, 0.25, [[-0.1075, G.R_SHAFT], [0.1075, G.R_SHAFT]], C_CASING, role="detail"),
        box("foot-plate-left", "casing", G.STEEL, 0.3, [G.FOOT_X1 - G.FOOT_X0, 0.008, 0.18], [-(G.FOOT_X0 + G.FOOT_X1) / 2, 0.004, 0.0]),
        box("foot-plate-right", "casing", G.STEEL, 0.3, [G.PLATE_X1 - G.FOOT_X0, 0.008, 0.18], [(G.FOOT_X0 + G.PLATE_X1) / 2, 0.004, 0.0]),
        extrude("gusset-left", "casing", G.STEEL, 0.3, G.gusset_profile(-1, 0.008), -0.09, -0.082, repeat={"count": 2, "translation": [0.0, 0.0, 0.172]}),
        extrude("gusset-right", "casing", G.STEEL, 0.3, G.gusset_profile(1, 0.008), -0.09, -0.082, repeat={"count": 2, "translation": [0.0, 0.0, 0.172]}),
        extrude("guard-plate-near", "casing", G.STEEL, 0.4, guard, G.GUARD_Z0, 0.022, holes=[G.circle(0.007, G.MOTOR_X, my, 12)]),
        extrude("guard-plate-far", "casing", G.STEEL, 0.4, guard, 0.038, G.GUARD_Z1),
        box("guard-edge", "casing", G.STEEL, 0.4, [0.0075, 0.235 - 0.008, 0.016], [x_right - 0.00375, (0.235 + 0.008) / 2, 0.03]),
        box("gearhead", "casing", G.PAINT, 0.25, [0.07, 0.07, 0.0385], [G.MOTOR_X, my, -0.00475], cornerRadius=0.003),
        box("motor", "casing", G.PAINT, 0.2, [0.07, 0.07, 0.055], [G.MOTOR_X, my, -0.0515], cornerRadius=0.012),
        revolve("motor-end-cap", "casing", G.PAINT, 0.2, [[-0.079, 0.031], [-0.086, 0.031], [-0.09, 0.024], [-0.09, 0.0]], [G.MOTOR_X, my, 0.0]),
        box("terminal-box", "casing", G.PAINT, 0.25, [0.015, 0.03, 0.04], [0.2975, my, -0.05], cornerRadius=0.002),
        revolve("motor-shaft", "casing", G.STEEL, 0.25, [[0.014, 0.005], [0.04, 0.005]], [G.MOTOR_X, my, 0.0], role="detail"),
        revolve("pulley", "casing", G.STEEL, 0.3, [[0.022, 0.0], [0.022, 0.023], [0.0235, 0.023], [0.025, G.PULLEY_R], [0.035, G.PULLEY_R], [0.0365, 0.023], [0.038, 0.023], [0.038, 0.0]], [G.MOTOR_X, my, 0.0], role="detail"),
        extrude("belt", "casing", G.RUBBER, 0.1, outer, G.BELT_Z0, G.BELT_Z1, holes=[inner], role="detail", slender=True),
        revolve("hub", "rotor", G.STEEL, 0.4, loop([[-0.1, G.R_SHAFT], [-0.1, G.R_HUB], [0.1, G.R_HUB], [0.1, G.R_SHAFT], [0.088, G.R_SHAFT], [0.088, 0.041], [-0.088, 0.041], [-0.088, G.R_SHAFT]]), C_LOCAL),
        revolve("matrix", "rotor", G.GEL, 0.6, loop([[-0.1, G.R_HUB], [-0.1, G.R_ROTOR - G.T_SHELL], [0.1, G.R_ROTOR - G.T_SHELL], [0.1, G.R_HUB]]), C_LOCAL),
        box("spoke", "rotor", G.STEEL, 0.3, [G.R_ROTOR - G.T_SHELL - G.R_HUB, G.SPOKE_T, 0.2006], [(G.R_ROTOR - G.T_SHELL + G.R_HUB) / 2, 0.0, 0.0], role="detail", texture=True, repeat=turn8()),
        revolve("rim-shell", "rotor", G.STEEL, 0.45, loop([[-0.1, G.R_ROTOR - G.T_SHELL], [-0.1, G.R_ROTOR], [0.1, G.R_ROTOR], [0.1, G.R_ROTOR - G.T_SHELL]]), C_LOCAL),
    ]
    out.append(extrude("sector-frame-near", "seal", G.STEEL, 0.3, G.v_profile(0.29), -0.109, -0.101))
    out.append(extrude("sector-frame-far", "seal", G.STEEL, 0.3, G.v_profile(0.29), 0.101, 0.109))
    return out


def zones(s, emitters_proud=0.0003):
    """🌡️ The stationary thermal zones of the turning matrix (seal frame): the process zone, the core of the hot sector, the two hot face skins, the two outlet patches."""
    rh, f = G.SCALE[s]["rotorHalf"], G.SCALE[s]["feature"]
    skin = 0.02 if s != 200 else 0.03
    r0, r1 = G.R_HUB, (G.R_ROTOR if s == 50 else G.R_FLANGE)
    patch = max(f, 0.008)
    hot = G.sector(r0, r1, -44.0, 44.0)
    out = [
        extrude("th-matrix-process", "seal", G.GEL, 0.6, G.sector(r0, r1, 44.0, 316.0), -rh, rh, region="disc"),
        extrude("th-matrix-sector-core", "seal", G.GEL, 0.75, hot, -rh + skin, rh - skin, region="disc"),
        extrude("th-regen-face-near", "seal", G.GEL, 0.9, hot, -rh, -rh + skin, region="regen", sectionFaces=["+w", "+v", "-v"]),
        extrude("th-regen-face-far", "seal", G.GEL, 0.9, hot, rh - skin, rh, region="regen", sectionFaces=["-w", "+v", "-v"]),
        extrude("th-process-air-outlet", "seal", G.GEL, 0.7, G.sector(r0 + 0.01, r1 - 0.012, 48.0, 312.0), -rh - emitters_proud, -rh - emitters_proud + patch, region="disc", tags=["process-air"]),
        extrude("th-regen-air-outlet", "seal", G.GEL, 0.95, G.sector(r0 + 0.01, r1 - 0.012, -40.0, 40.0), rh + emitters_proud - patch, rh + emitters_proud, region="regen", tags=["regen-air"]),
    ]
    if s != 200:
        out[2]["layer"] = "regen-skin"
        out[3]["layer"] = "regen-skin"
    return out


def ref(id, of, **kw):
    """🔗 A reference to a body solid."""
    out = {"id": id, "of": of}
    out.update(kw)
    return out


def sphere(id, bone, material, tone, r, at, **kw):
    """🔵 A ball of radius r."""
    out = base(id, bone, material, tone)
    out.update({"kind": "ellipsoid", "radii": [r, r, r], "transform": {"translation": [G.r6(v) for v in at]}})
    out.update(kw)
    return out


def masses(s):
    """⚖️ The three lumped masses at the centres of mass of their bones."""
    k = {50: 1.0, 100: 1.0, 200: 1.0}[s]
    return [
        sphere("mass-casing", "casing", G.STEEL, 0.35, 0.03 * k, [X.CASING_COM[0], X.CASING_COM[1], 0.0], member="casing", part="mass"),
        sphere("mass-rotor", "rotor", G.GEL, 0.6, 0.058 * k, [0.0, 0.0, 0.0], member="rotor", part="mass"),
        sphere("mass-seal", "seal", G.STEEL, 0.9, 0.02 * k, [0.15, 0.0, 0.0], member="seal", part="mass"),
    ]


def support(s):
    """🟩 The contact under the two feet: one pad solid, repeated."""
    f = G.SCALE[s]["feature"]
    w = G.FOOT_X1 - G.FOOT_X0 + 0.004
    return box("base-contact-pad", "casing", G.STEEL, 0.35, [w, f, 0.184], [-(G.FOOT_X0 + G.FOOT_X1) / 2, f / 2, 0.0], member="casing", part="support", anchor="base-contact", repeat={"count": 2, "translation": [G.FOOT_X0 + G.FOOT_X1, 0.0, 0.0]})


def quadrants(r0, r1, **kw):
    """🧭 The casing wall as four quarter solids (casing frame), each with the face that looks into the sealed gap round the rotor marked as no surface."""
    out = []
    for name, mid, face in (("ahead", 0.0, "-u"), ("top", 90.0, "-v"), ("behind", 180.0, "+u"), ("bottom", 270.0, "+v")):
        out.append(extrude(f"th-band-{name}", "casing", G.STEEL, 0.35, G.sector(r0, r1, mid - 45.0, mid + 45.0, 3.0, 0.0, G.CY), -G.HALF_DEPTH, G.HALF_DEPTH, region="casing", sectionFaces=[face], **kw))
    return out


def thermal50():
    """🌡️ 1:50 thermal list."""
    casing = ["beam-near", "beam-far", "foot-plate-left", "foot-plate-right", "gusset-left", "gusset-right", "sector-frame-near", "sector-frame-far"]
    out = quadrants(G.R_BAND - G.T_SHEET, G.R_BAND, layer="casing-skin")
    b, f, t, hd = G.R_BAND, G.R_FLANGE, G.T_SHEET, G.HALF_DEPTH
    out.append(revolve("th-flange-near", "casing", G.STEEL, 0.35, loop([[-hd, f], [-hd, b - t], [-hd + t, b - t], [-hd + t, f]]), C_CASING, region="casing", layer="casing-skin", sectionFaces=["+w"]))
    out.append(revolve("th-flange-far", "casing", G.STEEL, 0.35, loop([[hd - t, f], [hd - t, b - t], [hd, b - t], [hd, f]]), C_CASING, region="casing", layer="casing-skin", sectionFaces=["-w"]))
    out += [ref(f"th-{i}", i, region="casing") for i in casing]
    out += [ref("th-guard-plate-near", "guard-plate-near", region="casing", sectionFaces=["+w"]), ref("th-guard-plate-far", "guard-plate-far", region="casing", sectionFaces=["-w"]), ref("th-guard-edge", "guard-edge", region="casing", sectionFaces=["-u"])]
    out += zones(50)
    out.append(revolve("th-hub", "rotor", G.STEEL, 0.4, [[-0.1, G.R_HUB], [0.1, G.R_HUB]], C_LOCAL, region="hub"))
    out += [ref(f"th-{i}", i, region=G.MOTOR_REGION) for i in ("gearhead", "motor", "motor-end-cap", "terminal-box")]
    return out


def structure50():
    """🏗️ 1:50 load path: rotor on hub and spokes, bearings, fixed shaft, two beams, casing ring, feet."""
    out = [ref(f"st-{i}", i, member="casing", part="member") for i in ("band-near", "band-far", "band-belt-strip", "beam-near", "beam-far", "shaft", "foot-plate-left", "foot-plate-right", "gusset-left", "gusset-right")]
    out += [ref(f"st-{i}", i, member="rotor", part="member") for i in ("hub", "spoke", "rim-shell")]
    out += [ref(f"st-{i}", i, member="seal", part="member") for i in ("sector-frame-near", "sector-frame-far")]
    out += [
        revolve("joint-bearing-near", "rotor", G.STEEL, 0.4, loop([[-0.1003, G.R_SHAFT], [-0.1003, 0.026], [-0.0885, 0.026], [-0.0885, G.R_SHAFT]]), C_LOCAL, member="rotor", part="joint"),
        revolve("joint-bearing-far", "rotor", G.STEEL, 0.4, loop([[0.1003, G.R_SHAFT], [0.0885, G.R_SHAFT], [0.0885, 0.026], [0.1003, 0.026]]), C_LOCAL, member="rotor", part="joint"),
        box("joint-sector-root", "seal", G.STEEL, 0.3, [0.044, 0.044, 0.0085], [0.0, 0.0, -0.105], member="seal", part="joint", repeat={"count": 2, "translation": [0.0, 0.0, 0.21]}),
    ]
    return out + masses(50) + [support(50)]


def body100():
    """🏭 1:100: merged casing drum, bar beams, block feet, one motor block, solid guard."""
    b, f, t, hd, rh = G.R_BAND, G.R_FLANGE, G.T_SHEET, G.HALF_DEPTH, 0.095
    my = G.motor_y(100)
    return [
        revolve("casing", "casing", G.STEEL, 0.35, loop([[-hd, f], [-hd, b], [hd, b], [hd, f], [hd - t, f], [hd - t, b - t], [-hd + t, b - t], [-hd + t, f]]), C_CASING),
        box("beam-near", "casing", G.STEEL, 0.3, [G.BEAM_W, 0.55, 0.015], [0.0, G.CY, -0.1025]),
        box("beam-far", "casing", G.STEEL, 0.3, [G.BEAM_W, 0.55, 0.015], [0.0, G.CY, 0.1025]),
        extrude("foot-left", "casing", G.STEEL, 0.3, G.foot_profile(-1, 0.015), -0.09, 0.09),
        extrude("foot-right", "casing", G.STEEL, 0.3, G.foot_profile(1, 0.015, G.PLATE_X1), -0.09, 0.09),
        extrude("belt-guard", "casing", G.STEEL, 0.4, G.guard_profile(100), G.GUARD_Z0, G.GUARD_Z1),
        box("motor", "casing", G.PAINT, 0.2, [0.07, 0.07, 0.1045], [G.MOTOR_X, my, -0.03775], cornerRadius=0.008),
        revolve("hub", "rotor", G.STEEL, 0.4, [[-rh, G.R_HUB], [rh, G.R_HUB]], C_LOCAL),
        revolve("matrix", "rotor", G.GEL, 0.6, loop([[-rh, G.R_HUB], [-rh, G.R_ROTOR], [rh, G.R_ROTOR], [rh, G.R_HUB]]), C_LOCAL),
        box("spoke", "rotor", G.STEEL, 0.3, [G.R_ROTOR - G.R_HUB - 0.004, G.SPOKE_T, 2 * rh + 0.0006], [(G.R_ROTOR + G.R_HUB) / 2, 0.0, 0.0], role="detail", texture=True, repeat=turn8()),
        extrude("sector-frame-near", "seal", G.STEEL, 0.3, G.v_profile(0.275), -0.11, -0.095),
        extrude("sector-frame-far", "seal", G.STEEL, 0.3, G.v_profile(0.275), 0.095, 0.11),
    ]


def thermal100():
    """🌡️ 1:100 thermal list."""
    out = quadrants(G.R_FLANGE, G.R_BAND)
    out += [ref(f"th-{i}", i, region="casing") for i in ("beam-near", "beam-far", "foot-left", "foot-right", "belt-guard", "sector-frame-near", "sector-frame-far")]
    out += zones(100)
    out.append(ref("th-hub", "hub", region="hub"))
    out.append(ref("th-motor", "motor", region=G.MOTOR_REGION))
    return out


def structure100():
    """🏗️ 1:100 load path."""
    rh = 0.095
    out = [ref(f"st-{i}", i, member="casing", part="member") for i in ("casing", "beam-near", "beam-far", "foot-left", "foot-right")]
    out += [ref("st-hub", "hub", member="rotor", part="member"), ref("st-spoke", "spoke", member="rotor", part="member")]
    out.append(revolve("st-rim-shell", "rotor", G.STEEL, 0.45, loop([[-rh, G.R_ROTOR - G.T_SHELL], [-rh, G.R_ROTOR], [rh, G.R_ROTOR], [rh, G.R_ROTOR - G.T_SHELL]]), C_LOCAL, member="rotor", part="member"))
    out += [ref(f"st-{i}", i, member="seal", part="member") for i in ("sector-frame-near", "sector-frame-far")]
    out.append(revolve("joint-bearings", "rotor", G.STEEL, 0.4, loop([[-0.0955, G.R_SHAFT], [-0.0955, 0.03], [-0.0805, 0.03], [-0.0805, G.R_SHAFT]]), C_LOCAL, member="rotor", part="joint", repeat={"count": 2, "translation": [0.0, 0.0, 0.176]}))
    out.append(box("joint-sector-root", "seal", G.STEEL, 0.3, [0.044, 0.044, 0.0155], [0.0, 0.0, -0.1025], member="seal", part="joint", repeat={"count": 2, "translation": [0.0, 0.0, 0.205]}))
    return out + masses(100) + [support(100)]


def body200():
    """🏭 1:200: a solid casing ring round the matrix disc, 30 mm bars, block feet, motor and guard blocks on the ground."""
    b, f, hd, rh = G.R_BAND, G.R_FLANGE, G.HALF_DEPTH, 0.08
    return [
        revolve("casing", "casing", G.STEEL, 0.35, loop([[-hd, f], [-hd, b], [hd, b], [hd, f]]), C_CASING),
        box("beam-near", "casing", G.STEEL, 0.3, [G.BEAM_W, 0.56, 0.03], [0.0, G.CY, -0.095]),
        box("beam-far", "casing", G.STEEL, 0.3, [G.BEAM_W, 0.56, 0.03], [0.0, G.CY, 0.095]),
        extrude("foot-left", "casing", G.STEEL, 0.3, G.foot_profile(-1, 0.0), -0.09, 0.09),
        extrude("foot-right", "casing", G.STEEL, 0.3, G.foot_profile(1, 0.0), -0.09, 0.09),
        extrude("belt-guard", "casing", G.STEEL, 0.4, G.guard_profile(200), G.GUARD_Z0, G.GUARD_Z1),
        box("motor", "casing", G.PAINT, 0.2, [0.07, 0.07, 0.1045], [G.MOTOR_X, G.MOTOR_HALF, -0.03775]),
        revolve("rotor", "rotor", G.GEL, 0.6, [[-rh, f], [rh, f]], C_LOCAL),
        extrude("sector-frame-near", "seal", G.STEEL, 0.3, G.v_profile(0.28), -0.11, -0.08),
        extrude("sector-frame-far", "seal", G.STEEL, 0.3, G.v_profile(0.28), 0.08, 0.11),
    ]


def thermal200():
    """🌡️ 1:200 thermal list."""
    out = quadrants(G.R_FLANGE, G.R_BAND)
    out += [ref(f"th-{i}", i, region="casing") for i in ("beam-near", "beam-far", "foot-left", "foot-right", "belt-guard", "sector-frame-near", "sector-frame-far")]
    out += zones(200)
    out.append(revolve("th-hub", "rotor", G.STEEL, 0.4, [[-0.08, G.R_HUB], [0.08, G.R_HUB]], C_LOCAL, region="hub"))
    out.append(ref("th-motor", "motor", region=G.MOTOR_REGION))
    return out


def structure200():
    """🏗️ 1:200 load path: the rotor as hub, four spoke plates and rim, not the matrix disc."""
    rh, f = 0.08, G.R_FLANGE
    out = [ref(f"st-{i}", i, member="casing", part="member") for i in ("casing", "beam-near", "beam-far", "foot-left", "foot-right")]
    out += [
        revolve("st-hub", "rotor", G.STEEL, 0.4, [[-rh, G.R_HUB], [rh, G.R_HUB]], C_LOCAL, member="rotor", part="member"),
        box("st-spoke", "rotor", G.STEEL, 0.3, [f - 0.03 - G.R_HUB, 0.03, 2 * rh], [(f - 0.03 + G.R_HUB) / 2, 0.0, 0.0], member="rotor", part="member", repeat={"count": 4, "rotationRad": round(math.pi / 2, 9), "rotationAxis": Z, "origin": C_LOCAL}),
        revolve("st-rim", "rotor", G.STEEL, 0.45, loop([[-rh, f - 0.03], [-rh, f], [rh, f], [rh, f - 0.03]]), C_LOCAL, member="rotor", part="member"),
    ]
    out += [ref(f"st-{i}", i, member="seal", part="member") for i in ("sector-frame-near", "sector-frame-far")]
    return out + masses(200) + [support(200)]


def main():
    """💾 Write the source."""
    volumes = {
        "50": {"status": "authored", "denominator": 50, "body": body50(), "thermal": thermal50(), "structure": structure50(), "decisions": X.VOLUME_DECISIONS[50]},
        "100": {"status": "authored", "denominator": 100, "body": body100(), "thermal": thermal100(), "structure": structure100(), "decisions": X.VOLUME_DECISIONS[100]},
        "200": {"status": "authored", "denominator": 200, "body": body200(), "thermal": thermal200(), "structure": structure200(), "decisions": X.VOLUME_DECISIONS[200]},
        "500": {"status": "unavailable", "denominator": 500},
    }
    path = os.path.join(HERE, "volumes", "machinery", "sorption-wheel.json")
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({"volumes": volumes}, handle, ensure_ascii=False, indent=1)
        handle.write("\n")
    for scale in ("50", "100", "200"):
        v = volumes[scale]
        print(scale, len(v["body"]), len(v["thermal"]), len(v["structure"]))


if __name__ == "__main__":
    main()
