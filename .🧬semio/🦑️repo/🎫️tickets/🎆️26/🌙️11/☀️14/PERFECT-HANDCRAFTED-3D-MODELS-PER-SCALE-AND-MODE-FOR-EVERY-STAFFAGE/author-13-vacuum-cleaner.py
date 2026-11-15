import sys, json, math; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib")
globals().update({k: getattr(L, k) for k in dir(L) if not k.startswith("_")})
SEED = json.load(open("author-13-seed-vacuum.json"))
RED, GREY, HOSE, STEEL, RUB, BRI = "abs-red", "abs-grey", "hose-pvc", "tube-steel", "rubber-wheel", "nylon-bristle"
QY = [0, 1, 0, 0]
Q_XZ = [0.70710678, 0, 0, 0.70710678]
HOSES = [f"hose-{k}" for k in range(1, 11)]
ROT = [-0.5, -0.5, -0.5, 0.5]

def pick(solid, ids, new_id=None, **kw):
    s = json.loads(json.dumps(solid))
    s["sections"] = [s["sections"][i] for i in ids]
    if new_id: s["id"] = new_id
    s.update(kw); return s

def block(id, levels, mat, role, tone, n=6, capS=True, capE=True, **kw):
    secs = []
    for y, u0, u1, w in levels:
        secs.append({"at": [round((u0 + u1) / 2, 5), y, 0], "rx": w, "ry": round((u1 - u0) / 2, 5), "exponent": n, "rotation": ROT})
    return loft(id, "body", mat, role, tone, secs, capS, capE, **kw)

def wheel_pair(prefix, bone, z, r, w, hub_r=None, hub=True, rim=None, sides=("l", "r"), tone=0.2, extra_w=0.0):
    out = []
    for s in sides:
        sgn = 1 if s == "l" else -1
        prof = [(-w / 2, 0), (-w / 2, r * 0.83), (-w * 0.42, r * 0.97), (-w * 0.23, r), (w * 0.23, r), (w * 0.42, r * 0.97), (w / 2, r * 0.83)]
        if rim: prof += [(w / 2, rim[0]), (rim[1], rim[2]), (rim[1], 0)]
        else: prof += [(w / 2, 0)]
        out.append(rev(f"{prefix}-{s}", bone, RUB, "body", tone, prof, at=(0, 0, sgn * z), rot=QY if s == "r" else None))
    return out

def hose_set(r, with_ribs, role="body"):
    out = []
    for h in HOSES:
        out.append(cap(f"{h}-tube", h, HOSE, role, 0.35, (0, 0, 0), (0.11, 0, 0), r))
        if with_ribs: out.append(rev(f"{h}-rib", h, HOSE, "texture", 0.15, [(0.027, r + 0.0025), (0.033, r + 0.0025)], axis=(1, 0, 0), repeat={"count": 2, "translation": [0.05, 0, 0]}))
    return out

def build50():
    sh, lid, hd = SEED["shell"], SEED["lid"], SEED["handle"]
    grille = ext("exhaust-grille", "body", GREY, "body", 0.25, [(0.09, -0.09), (0.19, -0.09), (0.19, 0.09), (0.09, 0.09)], "yz", -0.262, -0.242, holes=[[(0.101 + 0.02 * k, -0.075), (0.109 + 0.02 * k, -0.075), (0.109 + 0.02 * k, 0.075), (0.101 + 0.02 * k, 0.075)] for k in range(5)])
    b = [sh, lid, hd,
         ell("rewind-button", "body", GREY, "detail", 0.8, (0.014, 0.0075, 0.014), (0.175, 0.2585, 0)),
         grille,
         box("front-bumper", "body", GREY, "body", 0.25, (0.02, 0.1, 0.1), (0.255, 0.11, 0), r=0.008)]
    b += wheel_pair("rear-wheel", "wheel", 0.135, 0.06, 0.03, rim=(0.034, 0.012, 0.03))
    for s, sg in (("l", 1), ("r", -1)):
        b.append(rev(f"rear-hub-{s}", "wheel", GREY, "body", 0.75, [(0.0085, 0.0), (0.0085, 0.028), (0.0165, 0.024), (0.0165, 0.0)], at=(0, 0, sg * 0.135), rot=QY if s == "r" else None))
    for s, sg in (("l", 1), ("r", -1)):
        b.append(rev(f"caster-wheel-{s}", "caster", RUB, "body", 0.2, [(-0.01, 0), (-0.01, 0.015), (-0.0075, 0.023), (-0.002, 0.025), (0.002, 0.025), (0.0075, 0.023), (0.01, 0.015), (0.01, 0)], at=(0, 0, sg * 0.05)))
    b.append(cap("caster-axle", "caster", GREY, "detail", 0.7, (0, 0, -0.056), (0, 0, 0.056), 0.0045))
    b += hose_set(0.019, True)
    b.append(cap("cuff", "hose-1", GREY, "body", 0.25, (-0.03, 0, 0), (0.012, 0, 0), 0.026))
    b.append(cap("wand-tube", "wand", STEEL, "body", 0.8, (0, 0, 0), (0.78, 0, 0), 0.0175))
    b.append(rev("wand-joint", "wand", STEEL, "detail", 0.3, [(0.395, 0.0195), (0.405, 0.0195)], axis=(1, 0, 0)))
    b.append(loft("grip-sleeve", "grip", GREY, "body", 0.3, [sec((u, 0, 0), r, r) for u, r in [(-0.026, 0.008), (-0.02, 0.016), (0.0, 0.0225), (0.03, 0.026), (0.1, 0.027), (0.13, 0.026), (0.15, 0.02), (0.166, 0.008)]]))
    b.append(ell("air-slide", "grip", GREY, "detail", 0.7, (0.014, 0.0075, 0.012), (0.06, 0.026, 0)))
    nz = SEED["nozzle"]
    b.append(nz)
    b.append(box("brush", "nozzle", BRI, "body", 0.15, (0.18, 0.0125, 0.28), (0.09, -0.03925, 0)))
    b.append(ell("swivel", "nozzle", GREY, "body", 0.5, (0.028, 0.028, 0.028)))
    return b


def tube_ring(id, bone, mat, ro, ri, length, member, part="member", tone=0.5):
    return st(ext(id, bone, mat, "body", tone, circle(ro, 24), "yz", 0, length, holes=[circle(ri, 24)]), member, part)

def open_tube(id, bone, mat, r, length, region, layer, tone=0.35):
    d = rev(id, bone, mat, "body", tone, [(0, r), (length, r)], axis=(1, 0, 0))
    d["region"] = region; d["layer"] = layer; return d

def struct50():
    s = [st(box("member-body", "body", RED, "body", 0.5, (0.4, 0.1, 0.2), (0, 0.115, 0), r=0.01), "body", "member"),
         st(cap("member-wheel", "wheel", GREY, "body", 0.5, (0, 0, -0.135), (0, 0, 0.135), 0.0075), "wheel", "member")]
    for h in HOSES:
        s.append(tube_ring(f"member-{h}", h, HOSE, 0.019, 0.0178, 0.11, h))
        s.append(st(ell(f"joint-{h}", h, HOSE, "detail", 0.35, (0.01, 0.021, 0.021)), h, "joint"))
    s += [st(cap("member-grip", "grip", GREY, "body", 0.5, (0.01, 0, 0), (0.13, 0, 0), 0.0145), "grip", "member"),
          st(ell("joint-grip", "grip", GREY, "detail", 0.35, (0.012, 0.024, 0.024)), "grip", "joint"),
          tube_ring("member-wand", "wand", STEEL, 0.0175, 0.0165, 0.78, "wand"),
          st(box("member-nozzle", "nozzle", GREY, "body", 0.5, (0.14, 0.03, 0.12), (0.07, -0.005, 0), r=0.005), "nozzle", "member"),
          st(ell("joint-nozzle", "nozzle", GREY, "detail", 0.35, (0.026, 0.026, 0.026)), "nozzle", "joint"),
          st(ell("joint-wheel", "wheel", GREY, "detail", 0.35, (0.02, 0.02, 0.02)), "wheel", "joint"),
          st(ell("joint-caster", "caster", GREY, "detail", 0.35, (0.012, 0.012, 0.03)), "caster", "joint")]
    s.append(st(box("support-wheel-contact", "wheel", RUB, "ground", 0.1, (0.03, 0.008, 0.3), (0, -0.056, 0)), "wheel", "support"))
    s.append(st(box("support-caster-contact", "caster", RUB, "ground", 0.1, (0.02, 0.008, 0.12), (0, -0.021, 0)), "caster", "support"))
    s.append(st(box("support-nozzle-contact", "nozzle", BRI, "ground", 0.1, (0.2, 0.008, 0.28), (0.05, -0.041, 0)), "nozzle", "support"))
    s += [st(ell("mass-body", "body", RED, "detail", 0.15, (0.1, 0.045, 0.08), (0, 0.14, 0)), "body", "mass"),
          st(ell("mass-nozzle", "nozzle", GREY, "detail", 0.15, (0.07, 0.02, 0.09), (0.05, -0.01, 0)), "nozzle", "mass"),
          st(ell("mass-grip", "grip", GREY, "detail", 0.15, (0.05, 0.015, 0.015), (0.07, 0, 0)), "grip", "mass")]
    for n, sg in (("l", 1), ("r", -1)):
        s.append(st(ell(f"mass-wheel-{n}", "wheel", RUB, "detail", 0.15, (0.02, 0.02, 0.012), (0, 0, sg * 0.135)), "wheel", "mass"))
    return s

def thermal50(b):
    ids = {x["id"]: x for x in b}
    lid = json.loads(json.dumps(ids["lid"])); lid["id"] = "th-lid"; lid["capStart"] = False; lid["region"] = "lid"; lid["layer"] = "lid"
    t = [th("th-shell", "shell", "body-shell", "shell"), lid]
    gr = loft("th-grille", "body", GREY, "body", 0.25, [sec((-0.262, 0.14, 0), 0.09, 0.05, 20), sec((-0.254, 0.14, 0), 0.09, 0.05, 20)], True, False, region="exhaust", layer="grille")
    em = rev("th-exhaust-air", "body", GREY, "detail", 0.5, [(-0.25, 0.045), (-0.22, 0.045), (-0.22, 0)], axis=(1, 0, 0), at=(0, 0.14, 0), region="body-shell", tags=["exhaust-air"])
    t += [gr, em]
    for n, sg in (("l", 1), ("r", -1)):
        tyre = rev(f"th-tyre-{n}", "wheel", RUB, "body", 0.2, [(0.012, 0.045), (0.012, 0.052), (0.0105, 0.058), (0.006, 0.06), (-0.006, 0.06), (-0.0105, 0.058), (-0.012, 0.052), (-0.012, 0.045)], at=(0, 0, sg * 0.135), region="wheels", layer="tyre")
        t.append(tyre)
    for h in HOSES: t.append(open_tube(f"th-{h}", h, HOSE, 0.0196, 0.11, "hose", "hose"))
    t.append(open_tube("th-wand", "wand", STEEL, 0.0182, 0.78, "wand", "tube", 0.8))
    t.append(loft("th-grip", "grip", GREY, "body", 0.3, [sec((u, 0, 0), r, r) for u, r in [(-0.02, 0.01), (0.0, 0.0178), (0.03, 0.0203), (0.1, 0.0212), (0.13, 0.0203), (0.15, 0.0155)]], False, False, region="grip", layer="sleeve"))
    t.append(loft("th-nozzle", "nozzle", GREY, "body", 0.45, [sec((0.045, 0.03, 0), 0.13, 0.14, 12, Q_XZ), sec((0.045, 0.05, 0), 0.13, 0.14, 12, Q_XZ)], False, True, region="nozzle", layer="noz"))
    t.append(box("th-brush", "nozzle", BRI, "texture", 0.15, (0.005, 0.0145, 0.28), (0.0025, -0.0405, 0), region="brush", layer="bristle"))
    return t


def disc(id, bone, mat, tone, r, w, z, side, hub=False):
    prof = [(-w / 2, 0), (-w / 2, r * 0.85), (-w * 0.25, r), (w * 0.25, r), (w / 2, r * 0.85), (w / 2, 0)]
    return rev(id, bone, mat, "body", tone, prof, at=(0, 0, side * z))

def common_hoses(r):
    return [cap(f"{h}-tube", h, HOSE, "body", 0.35, (0, 0, 0), (0.11, 0, 0), r) for h in HOSES]

def build100():
    sh = pick(SEED["shell"], [0, 2, 4, 5, 8, 10, 12])
    lid = pick(SEED["lid"], [0, 2, 4, 5, 6, 8])
    hd = json.loads(json.dumps(SEED["handle"])); hd.pop("holes")
    b = [sh, lid, hd,
         box("exhaust-grille", "body", GREY, "body", 0.25, (0.022, 0.1, 0.18), (-0.251, 0.14, 0), r=0.005),
         disc("rear-wheel-l", "wheel", RUB, 0.2, 0.06, 0.03, 0.135, 1), disc("rear-wheel-r", "wheel", RUB, 0.2, 0.06, 0.03, 0.135, -1)]
    for n, sg in (("l", 1), ("r", -1)):
        b.append(rev(f"rear-hub-{n}", "wheel", GREY, "body", 0.75, [(0.0, 0.0), (0.0, 0.025), (0.0165, 0.025), (0.0165, 0.0)], at=(0, 0, sg * 0.135), rot=QY if sg < 0 else None))
        b.append(rev(f"caster-wheel-{n}", "caster", RUB, "body", 0.2, [(-0.01, 0), (-0.01, 0.022), (-0.004, 0.025), (0.004, 0.025), (0.01, 0.022), (0.01, 0)], at=(0, 0, sg * 0.05)))
    b += common_hoses(0.019)
    b += [cap("cuff", "hose-1", GREY, "body", 0.25, (-0.03, 0, 0), (0.012, 0, 0), 0.026),
          cap("wand-tube", "wand", STEEL, "body", 0.8, (0, 0, 0), (0.78, 0, 0), 0.0175),
          cap("grip-sleeve", "grip", GREY, "body", 0.3, (0, 0, 0), (0.14, 0, 0), 0.026),
          SEED["nozzle"], box("brush", "nozzle", BRI, "body", 0.15, (0.18, 0.015, 0.28), (0.09, -0.0375, 0))]
    return b

def thermal_common(sc, shell, lid_src, gr_thick, tyre, hose_r, wand_r, grip_scale, nozzle_h, brush_w):
    lid = json.loads(json.dumps(lid_src)); lid["id"] = "th-lid"; lid["capStart"] = False; lid["region"] = "lid"; lid["layer"] = "lid"
    sh = json.loads(json.dumps(shell)); sh["id"] = "th-shell"; sh["region"] = "body-shell"; sh["layer"] = "shell"
    t = [sh, lid,
         loft("th-grille", "body", GREY, "texture", 0.25, [sec((-0.262, 0.14, 0), 0.09, 0.05, 20), sec((-0.262 + gr_thick, 0.14, 0), 0.09, 0.05, 20)], True, False, region="exhaust", layer="grille"),
         rev("th-exhaust-air", "body", GREY, "detail", 0.5, [(-0.25, 0.045), (-0.22, 0.045), (-0.22, 0)], axis=(1, 0, 0), at=(0, 0.14, 0), region="body-shell", tags=["exhaust-air"])]
    for n, sg in (("l", 1), ("r", -1)):
        t.append(rev(f"th-tyre-{n}", "wheel", RUB, "body", 0.2, tyre, at=(0, 0, sg * 0.135), region="wheels", layer="tyre"))
    for h in HOSES: t.append(open_tube(f"th-{h}", h, HOSE, hose_r + 0.0006, 0.11, "hose", "hose"))
    t.append(open_tube("th-wand", "wand", STEEL, wand_r + 0.0006, 0.78, "wand", "tube", 0.8))
    t.append(loft("th-grip", "grip", GREY, "body", 0.3, [sec((u, 0, 0), r * 1.02, r * 1.02) for u, r in [(-0.02, 0.012), (0.0, 0.0205), (0.03, 0.0235), (0.1, 0.0245), (0.13, 0.0235), (0.15, 0.018)]], False, False, region="grip", layer="sleeve"))
    t.append(loft("th-nozzle", "nozzle", GREY, "body", 0.45, [sec((0.045, 0.05 - nozzle_h, 0), 0.13, 0.14, 12, Q_XZ), sec((0.045, 0.05, 0), 0.13, 0.14, 12, Q_XZ)], False, True, region="nozzle", layer="noz"))
    t.append(box("th-brush", "nozzle", BRI, "texture", 0.15, (brush_w, 0.0145, 0.28), (brush_w / 2, -0.0405, 0), region="brush", layer="bristle"))
    return t

def struct_common(pad, joints, hose_joints, masses_wheels, section_ring=True):
    s = [st(box("member-body", "body", RED, "body", 0.5, (0.4, 0.1, 0.2), (0, 0.115, 0), r=0.01), "body", "member"),
         st(cap("member-wheel", "wheel", GREY, "body", 0.5, (0, 0, -0.135), (0, 0, 0.135), max(0.0075, pad / 2)), "wheel", "member")]
    for h in HOSES:
        s.append(tube_ring(f"member-{h}", h, HOSE, 0.019, 0.0178, 0.11, h))
        if hose_joints: s.append(st(ell(f"joint-{h}", h, HOSE, "detail", 0.35, (0.01, 0.021, 0.021)), h, "joint"))
    s += [st(cap("member-grip", "grip", GREY, "body", 0.5, (0.01, 0, 0), (0.13, 0, 0), max(0.0145, pad / 2)), "grip", "member"),
          tube_ring("member-wand", "wand", STEEL, 0.0175, 0.0165, 0.78, "wand"),
          st(box("member-nozzle", "nozzle", GREY, "body", 0.5, (0.14, 0.03, 0.12), (0.07, -0.005, 0), r=0.005), "nozzle", "member")]
    if joints:
        s += [st(ell("joint-grip", "grip", GREY, "detail", 0.35, (0.012, 0.024, 0.024)), "grip", "joint"),
              st(ell("joint-nozzle", "nozzle", GREY, "detail", 0.35, (0.026, 0.026, 0.026)), "nozzle", "joint")]
    return s

def supports(th, caster):
    s = []
    s.append(st(box("support-wheel-contact", "wheel", RUB, "ground", 0.1, (0.03, th, 0.3), (0, -0.06 + th / 2, 0)), "wheel", "support"))
    s.append(st(box("support-caster-contact", "caster", RUB, "ground", 0.1, (0.02 if th < 0.02 else th, th, 0.12), (0, -0.025 + th / 2, 0)), "caster", "support"))
    s.append(st(box("support-nozzle-contact", "nozzle", BRI, "ground", 0.1, (0.2, th, 0.28), (0.05, -0.045 + th / 2, 0)), "nozzle", "support"))
    return s

def masses(wheels=True, grip=True):
    s = [st(ell("mass-body", "body", RED, "detail", 0.15, (0.1, 0.045, 0.08), (0, 0.14, 0)), "body", "mass"),
         st(ell("mass-nozzle", "nozzle", GREY, "detail", 0.15, (0.07, 0.02, 0.09), (0.05, -0.01, 0)), "nozzle", "mass")]
    if grip: s.append(st(ell("mass-grip", "grip", GREY, "detail", 0.15, (0.05, 0.015, 0.015), (0.07, 0, 0)), "grip", "mass"))
    if wheels:
        for n, sg in (("l", 1), ("r", -1)): s.append(st(ell(f"mass-wheel-{n}", "wheel", RUB, "detail", 0.15, (0.02, 0.02, 0.012), (0, 0, sg * 0.135)), "wheel", "mass"))
    return s

TYRE50 = [(0.012, 0.045), (0.012, 0.052), (0.0105, 0.058), (0.006, 0.06), (-0.006, 0.06), (-0.0105, 0.058), (-0.012, 0.052), (-0.012, 0.045)]

NOZ_C = [(-0.09, -0.045), (0.19, -0.045), (0.19, 0.01), (0.06, 0.03), (0.03, 0.05), (-0.04, 0.05), (-0.09, 0)]
CAN200 = [(0.045, -0.215, 0.215, 0.08), (0.0613, -0.2423, 0.2456, 0.117), (0.1, -0.252, 0.262, 0.13), (0.188, -0.252, 0.258, 0.13), (0.2022, -0.2515, 0.2479, 0.1295),
          (0.2297, -0.239, 0.225, 0.1115), (0.2517, -0.217, 0.19, 0.0925), (0.2637, -0.1875, 0.15, 0.0811), (0.268, -0.15, 0.11, 0.05)]
CAN500 = [CAN200[i] for i in (0, 1, 2, 3, 5, 6, 8)]
LID500 = [(0.19, -0.252, 0.258, 0.13), (0.2022, -0.2515, 0.2479, 0.1295), (0.2297, -0.239, 0.225, 0.1115), (0.2517, -0.217, 0.19, 0.0925), (0.268, -0.15, 0.11, 0.05)]
LID200 = [CAN200[i] for i in (4, 5, 6, 7, 8)]

def open_ring(id, bone, mat, r, w, zc, side, region, layer, tone=0.2):
    d = loft(id, bone, mat, "body", tone, [sec((0, 0, side * (zc - w / 2)), r, r), sec((0, 0, side * (zc + w / 2)), r, r)], side > 0, side < 0, region=region, layer=layer)
    return d

def build_coarse(sc):
    big = sc == 500
    wz, ww = (0.1118, 0.0764) if big else (0.135, 0.031)
    hr = 0.0375 if big else 0.0255
    gr = 0.0375 if big else 0.031
    blk = block("canister", CAN500 if big else CAN200, RED, "body", 0.5, 6)
    b = [blk, disc("rear-wheel-l", "wheel", RUB, 0.2, 0.0603, ww, wz, 1), disc("rear-wheel-r", "wheel", RUB, 0.2, 0.0603, ww, wz + (0.0007 if big else 0), -1)]
    b += common_hoses(hr)
    b += [cap("wand-tube", "wand", STEEL, "body", 0.8, (0, 0, 0), (0.78, 0, 0), hr), cap("grip-sleeve", "grip", GREY, "body", 0.3, (0, 0, 0), (0.14, 0, 0), gr),
          ext("nozzle", "nozzle", GREY, "body", 0.45, NOZ_C, "xy", -0.14, 0.14)]
    return b

def collar(id, bone, mat, r, u0, u1, region, layer, tone=0.35):
    return loft(id, bone, mat, "body", tone, [sec((u0, 0, 0), r, r), sec((u1, 0, 0), r, r)], False, False, region=region, layer=layer)

def thermal_coarse(sc):
    big = sc == 500
    wz, ww = (0.1118, 0.0764) if big else (0.135, 0.031)
    sh = pick(SEED["shell"], [0, 3, 6, 9, 12] if big else [0, 2, 4, 5, 8, 10, 12], "th-shell", region="body-shell", layer="shell")
    lvl = [(0.19, -0.252, 0.258, 0.13), (0.2297, -0.239, 0.225, 0.1115), (0.268, -0.15, 0.11, 0.05)] if big else LID200
    lid = block("th-lid", lvl, RED, "body", 0.62, 5, False, True, region="lid", layer="lid")
    em_len = 0.075 if big else 0.03
    t = [sh, lid,
         loft("th-grille", "body", GREY, "texture", 0.25, [sec((-0.262, 0.14, 0), 0.09, 0.05, 20), sec((-0.254, 0.14, 0), 0.09, 0.05, 20)], True, False, region="exhaust", layer="grille"),
         rev("th-exhaust-air", "body", GREY, "detail", 0.5, [(-0.25, 0.045), (-0.25 + em_len, 0.045), (-0.25 + em_len, 0)], axis=(1, 0, 0), at=(0, 0.14, 0), region="body-shell", tags=["exhaust-air"])]
    for n, sg in (("l", 1), ("r", -1)):
        t.append(loft(f"th-tyre-{n}", "wheel", RUB, "body", 0.2, [sec((0, 0, sg * (wz - ww / 2)), 0.056, 0.056), sec((0, 0, sg * (wz + ww / 2)), 0.056, 0.056)], True, True, region="wheels", layer="tyre") if not big else open_ring(f"th-tyre-{n}", "wheel", RUB, 0.06, ww, wz, 1 if n == "l" else -1, "wheels", "tyre"))
    for h in HOSES:
        t.append(collar(f"th-{h}", h, HOSE, 0.0375, 0.0173, 0.0928, "hose", "hose") if big else open_tube(f"th-{h}", h, HOSE, 0.0196, 0.11, "hose", "hose"))
    t.append(collar("th-wand", "wand", STEEL, 0.0375, 0.195, 0.585, "wand", "tube", 0.8) if big else open_tube("th-wand", "wand", STEEL, 0.0182, 0.78, "wand", "tube", 0.8))
    if big: t.append(collar("th-grip", "grip", GREY, 0.0378, 0.02, 0.105, "grip", "sleeve", 0.3))
    else: t.append(loft("th-grip", "grip", GREY, "body", 0.3, [sec((u, 0, 0), r * 1.2, r * 1.2) for u, r in [(-0.02, 0.012), (0.0, 0.0178), (0.1, 0.0212), (0.15, 0.0155)]], False, False, region="grip", layer="sleeve"))
    nz = (0.1, 0.11, 0.075) if big else (0.13, 0.14, 0.03)
    t.append(loft("th-nozzle", "nozzle", GREY, "body", 0.45, [sec((0.045, 0.05 - nz[2], 0), nz[0], nz[1], 12, Q_XZ), sec((0.045, 0.05, 0), nz[0], nz[1], 12, Q_XZ)], False, True, region="nozzle", layer="noz"))
    t.append(box("th-brush", "nozzle", BRI, "texture", 0.15, (0.005, 0.0145, 0.28), (0.0025, -0.0405, 0), region="brush", layer="bristle"))
    return t

def struct_coarse(sc):
    big = sc == 500
    pad = 0.075 if big else 0.03
    ro = 0.0375 if big else 0.019
    s = [st(box("member-body", "body", RED, "body", 0.5, (0.4, 0.1, 0.2), (0, 0.115, 0), r=0.01), "body", "member"),
         st(cap("member-wheel", "wheel", GREY, "body", 0.5, (0, 0, -0.12), (0, 0, 0.12), pad / 2), "wheel", "member")]
    for h in HOSES: s.append(tube_ring(f"member-{h}", h, HOSE, ro, ro - 0.0012, 0.11, h))
    s += [st(cap("member-grip", "grip", GREY, "body", 0.5, (0.02, 0, 0), (0.12, 0, 0), pad / 2 if big else 0.015), "grip", "member"),
          tube_ring("member-wand", "wand", STEEL, ro, ro - 0.0012, 0.78, "wand"),
          st(box("member-nozzle", "nozzle", GREY, "body", 0.5, (0.14, pad if big else 0.03, 0.12), (0.07, -0.005, 0), r=0.005), "nozzle", "member")]
    s.append(st(box("support-wheel-contact", "wheel", RUB, "ground", 0.1, (0.03 if pad < 0.03 else pad, pad, 0.3), (0, -0.06 + pad / 2, 0)), "wheel", "support"))
    s.append(st(box("support-caster-contact", "caster", RUB, "ground", 0.1, (pad, pad, 0.12), (0, -0.025 + pad / 2, 0)), "caster", "support"))
    s.append(st(box("support-nozzle-contact", "nozzle", BRI, "ground", 0.1, (0.2, pad, 0.28), (0.05, -0.045 + pad / 2, 0)), "nozzle", "support"))
    m = [st(ell("mass-body", "body", RED, "detail", 0.15, (0.1, 0.045, 0.08), (0, 0.14, 0)), "body", "mass"),
         st(ell("mass-nozzle", "nozzle", GREY, "detail", 0.15, (0.07, max(0.02, pad / 2), 0.09), (0.05, -0.01, 0)), "nozzle", "mass")]
    return s + (m if big else m + [st(ell("mass-grip", "grip", GREY, "detail", 0.15, (0.05, 0.015, 0.02), (0.07, 0, 0)), "grip", "mass")])

DEC = {
"50": (["Body: the canister is a 13-section lofted shell with a lofted lid and a carry handle with its opening, a louvred exhaust grille (five slots cut through an extrude), a bumper, two solid rear tyres with dished rims and hub caps, two caster discs on an axle; the hose is ten capsule segments with corrugation rings, the telescopic wand a steel capsule with a joint ring, the grip a lofted sleeve with air slide, the nozzle an extrude on a swivel ball with a bristle strip. Each part keeps the bone of the drawings, so the pose chain moves all of them.",
        "Thermal: every region has its own skin, each with its single layer: the canister shell is the body shell itself, the lid an open-based dome, the exhaust a front-capped 8 mm grille plate, tyres open bands, hose and wand open tubes, the grip a thinned sleeve, the nozzle a hood skin of 20 mm and the bristle strip a 5 mm texture row (areas follow areaM2); the exhaust-air emitter is a 90 mm duct solid behind the grille inside the shell.",
        "Structure: a 400 x 100 x 200 mm chassis core in the canister, the wheel axle, ten thin-walled hose rings with a joint ball at each hose bone, the grip core, the wand tube, the nozzle spine and the swivel joint; supports are rubber pads under both tyres, both caster wheels and the brush strip; masses are the motor and dust-bin block, the nozzle, the grip and both wheels."],
       ["Körper: der Kessel ist eine Schale aus 13 Loft-Schnitten mit gelofteten Deckel und Tragegriff samt Öffnung, einem Abluftgitter mit fünf durchgeschnittenen Schlitzen, einer Stoßstange, zwei vollen Hinterreifen mit Felgenmulde und Radkappe und zwei Rollenscheiben auf einer Achse; der Schlauch besteht aus zehn Kapselsegmenten mit Wellringen, das Teleskoprohr aus einer Stahlkapsel mit Stoßring, der Griff aus einer gelofteten Hülse mit Nebenluftschieber, die Düse aus einem Extrusionskörper auf einer Gelenkkugel mit Borstenleiste. Jedes Teil behält den Knochen der Zeichnungen, die Kette bewegt alle mit.",
        "Thermik: jede Region hat ihre eigene Haut mit je einer Schicht: die Kesselschale ist der Körper selbst, der Deckel eine unten offene Kuppel, die Abluft eine vorne geschlossene 8-mm-Gitterplatte, die Reifen offene Bänder, Schlauch und Rohr offene Röhren, der Griff eine verdünnte Hülse, die Düse eine Haubenhaut von 20 mm und die Borstenleiste eine 5-mm-Texturreihe (die Flächen folgen areaM2); der Emitter Abluft ist ein 90-mm-Kanal hinter dem Gitter innerhalb der Schale.",
        "Tragwerk: ein Gehäusekern von 400 x 100 x 200 mm im Kessel, die Radachse, zehn dünnwandige Schlauchringe mit einer Gelenkkugel an jedem Schlauchknochen, der Griffkern, das Rohr, die Düsenwirbelsäule und das Schwenkgelenk; Auflager sind Gummipads unter beiden Reifen, beiden Rollen und der Borstenleiste; Massen sind Motor- und Staubbehälterblock, Düse, Griff und beide Räder."]),
"100": (["Body: modelled anew against the 1:100 drawings with seven shell and six lid sections, a handle without opening (15 mm smallest feature), one grille block for the louvres, plain tyre discs and 17 mm hub caps, caster discs; the ten hose segments stay separate capsules (they are bones) but the ribs, the bumper (merged into the shell), the wand joint ring and the swivel are gone and the brush is 15 mm thick.",
         "Thermal: same regions as at 1:50, one solid each; shell and lid reuse the reduced sections, tyres are four-point bands, hose and wand are open tubes, the nozzle hood is 20 mm; the exhaust duct stays inside the shell.",
         "Structure: the same members as at 1:50 (chassis core, axle, ten hose rings, grip, wand, nozzle), but joint balls of the hose chain are merged into the overlapping rings and only grip and nozzle keep a joint; pads and masses are 15 mm thick."],
        ["Körper: neu gegen die Zeichnungen 1:100 gebaut mit sieben Schalen- und sechs Deckelschnitten, einem Griff ohne Öffnung (Mindestmaß 15 mm), einem Gitterblock statt der Lamellen, einfachen Reifenscheiben mit 17-mm-Radkappen und Rollenscheiben; die zehn Schlauchsegmente bleiben einzelne Kapseln (sie sind Knochen), aber Rippen, Stoßstange (in der Schale aufgegangen), Rohrstoßring und Schwenkkugel entfallen und die Bürste ist 15 mm dick.",
         "Thermik: dieselben Regionen wie bei 1:50 mit je einem Körper; Schale und Deckel verwenden die reduzierten Schnitte, die Reifen sind Bänder aus vier Punkten, Schlauch und Rohr offene Röhren, die Düsenhaube 20 mm; der Abluftkanal bleibt in der Schale.",
         "Tragwerk: dieselben Glieder wie bei 1:50 (Gehäusekern, Achse, zehn Schlauchringe, Griff, Rohr, Düse), die Gelenkkugeln der Schlauchkette sind aber in den überlappenden Ringen aufgegangen, nur Griff und Düse behalten ein Gelenk; Pads und Massen sind 15 mm dick."]),
"200": (["Body: shell, lid, handle and grille merge into one nine-section canister block following the 1:200 outline, the bristle strip and the swivel into the nozzle wedge; the caster joins the shell. Two 31 mm tyre discs remain, ten hose capsules and the wand with the grip are drawn 51 and 62 mm wide (30 mm smallest feature), a little thicker than the 48 mm of the drawing so that they stay close to both neighbouring scales.",
         "Thermal: each of the nine regions keeps one solid at the 30 mm limit: the shell is the canister's lower part, the lid a dome skin of 4 sections, the grille an 8 mm texture plate, tyres capped bands of radius 56 mm, hose and wand open tubes, grip sleeve, a 30 mm nozzle hood and a texture row for the bristles.",
         "Structure: members as before (chassis, axle, ten hose rings, grip, wand, nozzle) with 30 mm pads under the tyres, the caster (which is not drawn any more but still carries load) and the brush; hose joints are merged into the ring overlaps and masses are the motor, the nozzle and the grip."],
        ["Körper: Schale, Deckel, Griff und Gitter gehen in einem Kesselblock aus neun Schnitten nach dem Umriss 1:200 auf, Borstenleiste und Schwenkkugel in der Düsenspitze; die Rolle verschwindet in der Schale. Zwei Reifenscheiben von 31 mm bleiben, die zehn Schlauchkapseln und das Rohr mit dem Griff sind 51 und 62 mm breit gezeichnet (Mindestmaß 30 mm), etwas dicker als die 48 mm der Zeichnung, damit sie nahe an beiden Nachbarmaßstäben bleiben.",
         "Thermik: jede der neun Regionen behält einen Körper an der 30-mm-Grenze: die Schale ist der untere Teil des Kessels, der Deckel eine Kuppelhaut aus 4 Schnitten, das Gitter eine 8-mm-Texturplatte, die Reifen geschlossene Bänder mit Radius 56 mm, Schlauch und Rohr offene Röhren, der Griff eine Hülse, die Düse eine Haube von 30 mm und die Borsten eine Texturreihe.",
         "Tragwerk: Glieder wie zuvor (Gehäuse, Achse, zehn Schlauchringe, Griff, Rohr, Düse) mit 30-mm-Pads unter den Reifen, der Rolle (nicht mehr gezeichnet, trägt aber weiter) und der Bürste; die Schlauchgelenke sind in den Ringüberlappungen aufgegangen, Massen sind Motor, Düse und Griff."]),
"500": (["Body: one seven-section canister block, two 76 mm wide wheel blocks, ten hose capsules, wand and grip all 75 mm thick (the 75 mm feature is above the 60 mm of the drawing: this is the thinnest solid the scale allows) and the nozzle wedge; the 1:500 silhouette therefore stays slightly outside the drawn hose strokes (IoU 0.91, see author-requests).",
         "Thermal: nine regions, one solid each, 75 mm at least: hose collars 75 mm long in the middle of every segment and a 390 mm wand sleeve and 85 mm grip sleeve (the full tubes would be twice areaM2 at this diameter), shell and lid reduced to 5 and 3 sections, the nozzle hood 75 mm deep over a smaller footprint.",
         "Structure: members scaled to 75 mm (chassis, axle, ten hose rings, grip, wand, nozzle spine), 75 mm pads under tyres, caster and brush and masses for motor and nozzle; joints are all merged."],
        ["Körper: ein Kesselblock aus sieben Schnitten, zwei 76 mm breite Radblöcke, zehn Schlauchkapseln, Rohr und Griff mit je 75 mm Dicke (das Mindestmaß 75 mm liegt über den 60 mm der Zeichnung: dünner geht es bei diesem Maßstab nicht) und der Düsenkeil; die Silhouette 1:500 liegt deshalb knapp außerhalb der gezeichneten Schlauchstriche (IoU 0,91, siehe author-requests).",
         "Thermik: neun Regionen mit je einem Körper von mindestens 75 mm: 75 mm lange Schlauchmanschetten in der Mitte jedes Segments, eine 390 mm lange Rohrhülse und eine 85 mm lange Griffhülse (die vollen Röhren hätten bei diesem Durchmesser die doppelte Fläche von areaM2), Schale und Deckel auf 5 und 3 Schnitte reduziert, die Düsenhaube 75 mm tief über kleinerer Grundfläche.",
         "Tragwerk: Glieder auf 75 mm vergrößert (Gehäuse, Achse, zehn Schlauchringe, Griff, Rohr, Düsenwirbelsäule), 75-mm-Pads unter Reifen, Rolle und Bürste sowie Massen für Motor und Düse; alle Gelenke sind zusammengefasst."]),
}

def grow(x, d):
    x = json.loads(json.dumps(x)); k = x.get("kind")
    if k == "loft": x["sections"] = [dict(q, rx=round(q["rx"] + d, 6), ry=round(q["ry"] + d, 6)) for q in x["sections"]]
    elif k == "capsule": x["r0"] = round(x["r0"] + d, 6); x["r1"] = round(x["r1"] + d, 6)
    elif k == "revolve": x["profile"] = [[a, round(r + d, 6) if r > 0 else r] for a, r in x["profile"]]
    elif k == "box": x["size"] = [round(v + 2 * d, 6) for v in x["size"]]
    elif k == "ellipsoid": x["radii"] = [round(v + d, 6) for v in x["radii"]]
    return x
def skin_pass(v):
    for sc, vol in v.items():
        if vol.get("status") != "authored": continue
        body = {b["id"]: b for b in vol["body"]}
        out = []
        for t in vol["thermal"]:
            if "of" in t and t["of"] in body:
                g = grow(body[t["of"]], 0.001 if int(sc) <= 100 else 0.002)
                g["id"] = t["id"]; g["region"] = t["region"]
                if "layer" in t: g["layer"] = t["layer"]
                for kk in ("repeat",):
                    pass
                out.append(g)
            else: out.append(t)
        for i, t in enumerate(out):
            if t["id"] == "th-grip" and "grip-sleeve" in body and body["grip-sleeve"]["kind"] in ("loft", "capsule"):
                g = grow(body["grip-sleeve"], 0.0006); g["id"] = "th-grip"; g["region"] = "grip"; g["layer"] = "sleeve"
                if g["kind"] == "loft": g["capStart"] = False; g["capEnd"] = False
                out[i] = g
        vol["thermal"] = out
    return v

def volumes():
    return skin_pass(volumes0())
def volumes0():
    b50 = build50()
    v = {"50": scale_vol(50, b50, thermal50(b50), struct50(), *DEC["50"])}
    b100 = build100()
    t100 = thermal_common(100, pick(SEED["shell"], [0, 2, 4, 5, 8, 10, 12]), pick(SEED["lid"], [0, 2, 4, 5, 6, 8]), 0.01, [(0.012, 0.045), (0.012, 0.058), (0.006, 0.06), (-0.006, 0.06), (-0.012, 0.058), (-0.012, 0.045)], 0.019, 0.0175, 0.87, 0.02, 0.005)
    s100 = struct_common(0.015, True, False, True) + supports(0.015, True) + masses()
    v["100"] = scale_vol(100, b100, t100, s100, *DEC["100"])
    for sc in (200, 500):
        v[str(sc)] = scale_vol(sc, build_coarse(sc), thermal_coarse(sc), struct_coarse(sc), *DEC[str(sc)])
    return v

if __name__ == "__main__":
    v = volumes()
    write("appliances/vacuum-cleaner", v)
    print(run("check", "appliances/vacuum-cleaner"))
