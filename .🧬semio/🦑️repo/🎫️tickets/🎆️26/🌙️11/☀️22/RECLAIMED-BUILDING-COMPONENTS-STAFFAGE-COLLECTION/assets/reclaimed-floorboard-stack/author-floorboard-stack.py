"""🪵 Authoritative author script of the staffage asset `reclaimed-floorboard-stack`: every hand-chosen number of the stack, its rig, poses, clips, thermal and structural models, the four drawings and the four volumes, written to `🔣️.json` next to this file.

Run: python3 author-floorboard-stack.py

See ../../brief.md (asset contract) and ../../reports/reclaimed-floorboard-stack.md (reference table and decisions).
"""
import json
import math
import random
from pathlib import Path

HERE = Path(__file__).resolve().parent


def T(en, de):
    """🌐 A bilingual text, English first."""
    return {"en": en, "de": de}


def r6(x):
    """🎯 Rounds a coordinate to a micrometre."""
    return round(x + 0.0, 6)


BT = 0.028
BW = 0.137
TG = 0.008
TT = 0.009
GD = 0.010
GW = 0.010
LW = 6 * BW + TG
HL = LW / 2
NL = 19
STK_T = 0.024
STK_B = 0.048
STK_L = 0.85
P = BT + STK_T
XS = [0.125 + 0.75 * j for j in range(6)]
BR = 0.1
BR_L = 1.0
STACK_Y = BR
TOP19 = (NL - 1) * P + BT
STRIP_V = NL * P
PT_V = STRIP_V + BT
SL_B = 0.06
SL_H = 0.08
SL_L = 1.0
XSL = [0.05 + 0.65 * i for i in range(7)]
Z_SL = 1.3
XST = [0.5, 3.5]
ST_W = 0.019
ST_T = 0.0008
XSG = [1.25, 2.75]
RS = 0.016
RSU = 0.0165
ZSG = HL + RS
LEG = 0.75 * math.sqrt(2)
L1 = 0.40
L2 = LEG - L1
LEG_V = 0.0168
LIFT = 0.30
DROP = P
STRIP_DZ = Z_SL
ZOFF = -Z_SL / 2
STRIP_DY = SL_H - (STACK_Y + STRIP_V)

RHO_PINE = 510.0
RHO_SW = 420.0
RHO_PET = 1380.0
RHO_SLING = 250.0

PROFILE_AREA = BW * BT + TG * TT - GD * GW

rng = random.Random(5)
LAYERS = [[round(rng.uniform(0.0, 0.024), 3) for _ in range(6)] for _ in range(NL)]
STRIP = [round(rng.uniform(0.0, 0.006), 3) for _ in range(6)]
DMAX = max(max(l) for l in LAYERS)
XMIN = -DMAX
XMAX = 4.0 + DMAX


def board_profile(w0):
    """🪚 The tongue-and-groove section of one board in (v, w): body from w0 over 137 mm, tongue 9 x 8 mm on +w, groove 10 x 10 mm on -w."""
    a = w0 + BW
    return [[0.0, w0], [0.0, a], [0.0095, a], [0.0095, a + TG], [0.0185, a + TG], [0.0185, a], [BT, a], [BT, w0], [0.019, w0], [0.019, w0 + GD], [0.009, w0 + GD], [0.009, w0]]


def layer_front(y0):
    """🪚 The end view of one engaged layer in world (z, y): six boards, groove open at -z, tongue proud at +z."""
    zl = -HL
    zr = HL - TG
    return [[zl, y0], [zr, y0], [zr, y0 + 0.0095], [HL, y0 + 0.0095], [HL, y0 + 0.0185], [zr, y0 + 0.0185], [zr, y0 + BT], [zl, y0 + BT], [zl, y0 + 0.019], [zl + GD, y0 + 0.019], [zl + GD, y0 + 0.009], [zl, y0 + 0.009]]


def solid(kind, sid, bone, material, tone, role="body", **kw):
    """🧊 A volume solid."""
    s = {"id": sid, "kind": kind, "bone": bone, "material": material, "role": role, "tone": tone}
    s.update(kw)
    return s


def box(sid, bone, material, tone, centre, size, **kw):
    """📦 A box solid centred at `centre` (u, v, w) of size (u, v, w)."""
    s = solid("box", sid, bone, material, tone, size=[r6(v) for v in size], transform={"translation": [r6(v) for v in centre]})
    s.update(kw)
    return s


def capsule(sid, bone, material, tone, p0, p1, r, **kw):
    """💊 A capsule solid between p0 and p1."""
    s = solid("capsule", sid, bone, material, tone, p0=[r6(v) for v in p0], p1=[r6(v) for v in p1], r0=r, r1=r)
    s.update(kw)
    return s


def ref(sid, of, **kw):
    """🔗 A thermal or structural reference to a body solid."""
    s = {"id": sid, "of": of}
    s.update(kw)
    return s


def shape(kind, sid, bone, role, z, tone, filled=True, **kw):
    """✏️ A 2D shape."""
    s = {"id": sid, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone, "kind": kind}
    s.update(kw)
    return s


def rect(sid, bone, x, y, w, h, role="body", z=1, tone=0.5, **kw):
    """⬛ A rect shape, lower-left at (x, y)."""
    return shape("rect", sid, bone, role, z, tone, x=r6(x), y=r6(y), w=r6(w), h=r6(h), **kw)


def cap2(sid, bone, a, b, r, role="body", z=1, tone=0.5, **kw):
    """💊 A capsule shape between a and b."""
    return shape("capsule", sid, bone, role, z, tone, x0=r6(a[0]), y0=r6(a[1]), x1=r6(b[0]), y1=r6(b[1]), r0=r, r1=r, **kw)


def line(sid, bone, pts, role="texture", z=5, tone=0.25, **kw):
    """〰️ An open stroke."""
    return shape("poly", sid, bone, role, z, tone, filled=False, points=[[r6(x), r6(y)] for x, y in pts], closed=False, **kw)


def poly(sid, bone, pts, role="body", z=1, tone=0.5, **kw):
    """🔷 A closed filled polygon."""
    return shape("poly", sid, bone, role, z, tone, points=[[r6(x), r6(y)] for x, y in pts], closed=True, **kw)


def lv(k):
    """📐 Stack-local bottom of layer k (0-based, 0..18)."""
    return k * P


def wy_stack(v):
    """🌍 World height of a stack-local v."""
    return STACK_Y + v


def wy_pt(v):
    """🌍 World height of a pack-top-local v at rest."""
    return STACK_Y + PT_V + v


MAT_BOARD = "pine-floorboard"
MAT_WEATHERED = "pine-weathered"
MAT_SW = "softwood-weathered"
MAT_NEW = "softwood-new"
MAT_PET = "pet-strap"
MAT_SLING = "round-sling"

TONE_LAYER = [0.52, 0.6, 0.56, 0.64, 0.5, 0.58]
TONE_STRIP = 0.42
TONE_STK = 0.74
TONE_BR = 0.36
TONE_SL = 0.82
TONE_ST = 0.18
TONE_SG = 0.3

R_STRIP = "strip-boards"
R_STACK = "stack-boards"
R_BR = "bearers"
R_SL = "sleepers"
R_LASH = "lashing"


def lashing_parts(scale):
    """🪢 The body solids of the two PET straps and two round slings of a scale: 1:50 and 1:100 capsules and sheets, 1:200 boxes and no straps, 1:500 none."""
    body = []
    if scale in (50, 100):
        zs = HL + ST_T / 2
        zu = HL + ST_T * 1.5
        for i, x in enumerate(XST, 1):
            body += [
                box(f"strap-{i}-bottom", "stack", MAT_PET, TONE_ST, [x, -ST_T / 2, 0], [ST_W, ST_T, 2 * HL + 2 * ST_T], sheet=True),
                box(f"strap-{i}-side-r", "stack", MAT_PET, TONE_ST, [x, (TOP19 - ST_T) / 2, -zs], [ST_W, TOP19 + ST_T, ST_T], sheet=True),
                box(f"strap-{i}-side-l", "stack", MAT_PET, TONE_ST, [x, (TOP19 - ST_T) / 2, zs], [ST_W, TOP19 + ST_T, ST_T], sheet=True),
                box(f"strap-{i}-upper-r", "pack-top", MAT_PET, TONE_ST, [x, (ST_T - DROP) / 2, -zu], [ST_W, DROP + ST_T, ST_T], sheet=True),
                box(f"strap-{i}-upper-l", "pack-top", MAT_PET, TONE_ST, [x, (ST_T - DROP) / 2, zu], [ST_W, DROP + ST_T, ST_T], sheet=True),
                box(f"strap-{i}-top", "pack-top", MAT_PET, TONE_ST, [x, ST_T / 2, 0], [ST_W, ST_T, 2 * HL + 4 * ST_T], sheet=True),
            ]
        for i, x in enumerate(XSG, 1):
            body += [
                capsule(f"sling-{i}-bottom", "stack", MAT_SLING, TONE_SG, [x, -RS, -ZSG], [x, -RS, ZSG], RS),
                capsule(f"sling-{i}-side-r", "stack", MAT_SLING, TONE_SG, [x, -RS, -ZSG], [x, TOP19, -ZSG], RS),
                capsule(f"sling-{i}-side-l", "stack", MAT_SLING, TONE_SG, [x, -RS, ZSG], [x, TOP19, ZSG], RS),
                capsule(f"sling-{i}-upper-r", "pack-top", MAT_SLING, TONE_SG, [x, -DROP, -ZSG], [x, RS, -ZSG], RSU),
                capsule(f"sling-{i}-upper-l", "pack-top", MAT_SLING, TONE_SG, [x, -DROP, ZSG], [x, RS, ZSG], RSU),
                capsule(f"sling-{i}-top", "pack-top", MAT_SLING, TONE_SG, [x, RS, -ZSG], [x, RS, ZSG], RS),
            ]
        for leg, length in (("leg-a1", L1), ("leg-a2", L2), ("leg-b1", L1), ("leg-b2", L2)):
            body.append(capsule(leg, leg, MAT_SLING, TONE_SG - 0.04, [0, 0, 0], [length, 0, 0], RS))
    elif scale == 200:
        a = 2 * RS
        for i, x in enumerate(XSG, 1):
            body += [
                box(f"sling-{i}-bottom", "stack", MAT_SLING, TONE_SG, [x, -RS, 0], [a, a, 2 * ZSG + a]),
                box(f"sling-{i}-side-r", "stack", MAT_SLING, TONE_SG, [x, (TOP19 - a) / 2, -ZSG], [a, TOP19 + a, a]),
                box(f"sling-{i}-side-l", "stack", MAT_SLING, TONE_SG, [x, (TOP19 - a) / 2, ZSG], [a, TOP19 + a, a]),
                box(f"sling-{i}-upper-r", "pack-top", MAT_SLING, TONE_SG, [x, (a - DROP) / 2, -ZSG], [a, DROP + a, a]),
                box(f"sling-{i}-upper-l", "pack-top", MAT_SLING, TONE_SG, [x, (a - DROP) / 2, ZSG], [a, DROP + a, a]),
                box(f"sling-{i}-top", "pack-top", MAT_SLING, TONE_SG, [x, RS, 0], [a, a, 2 * ZSG + a]),
            ]
        for leg, length in (("leg-a1", L1), ("leg-a2", L2), ("leg-b1", L1), ("leg-b2", L2)):
            body.append(box(leg, leg, MAT_SLING, TONE_SG - 0.04, [length / 2, 0, 0], [length, a, a]))
    return body


def lashing_shapes(scale):
    """🪢 The 2D of straps, slings and legs at 1:50 and 1:100 (same parts as the solids of `lashing_parts`)."""
    el, fr, pl = [], [], []
    pen = 0.13
    thin = {} if scale == 50 else {"penMm": 0.1}
    for i, x in enumerate(XST, 1):
        el.append(rect(f"strap-{i}", "stack", x - ST_W / 2, -ST_T, ST_W, TOP19 + ST_T, role="detail", z=40, tone=TONE_ST, region=R_LASH, material=MAT_PET, penMm=pen))
        if scale == 50:
            el.append(rect(f"strap-{i}-top", "pack-top", x - ST_W / 2, BT - P, ST_W, P - BT + ST_T, role="detail", z=41, tone=TONE_ST, region=R_LASH, material=MAT_PET, penMm=pen))
        pl += [rect(f"strap-{i}-top", "pack-top", x - ST_W / 2, -(HL + 2 * ST_T), ST_W, 2 * HL + 4 * ST_T, role="detail", z=40, tone=TONE_ST, region=R_LASH, material=MAT_PET, penMm=pen)]
    for i, x in enumerate(XSG, 1):
        el += [cap2(f"sling-{i}", "stack", [x, -RS], [x, TOP19], RS, z=42, tone=TONE_SG, region=R_LASH, material=MAT_SLING, **thin),
               cap2(f"sling-{i}-top", "pack-top", [x, BT - P + RSU if scale == 50 else RS], [x, RS], RSU, z=43, tone=TONE_SG, region=R_LASH, material=MAT_SLING, **thin)]
        for side, zz in (("r", -ZSG), ("l", ZSG)):
            fr += [cap2(f"sling-{i}-side-{side}", "stack", [zz, wy_stack(-RS)], [zz, wy_stack(TOP19)], RS, z=42 + i * 0.1, tone=TONE_SG, region=R_LASH, material=MAT_SLING, **thin),
                   cap2(f"sling-{i}-upper-{side}", "pack-top", [zz, wy_pt(-DROP)], [zz, wy_pt(RS)], RSU, z=43 + i * 0.1, tone=TONE_SG, region=R_LASH, material=MAT_SLING, **thin)]
        fr += [cap2(f"sling-{i}-top", "pack-top", [-ZSG, wy_pt(RS)], [ZSG, wy_pt(RS)], RS, z=44 + i * 0.1, tone=TONE_SG, region=R_LASH, material=MAT_SLING, **thin)]
        pl += [cap2(f"sling-{i}-top", "pack-top", [x, -ZSG], [x, ZSG], RS, z=45, tone=TONE_SG, region=R_LASH, material=MAT_SLING, **thin)]
    for leg, length in (("leg-a1", L1), ("leg-a2", L2), ("leg-b1", L1), ("leg-b2", L2)):
        el.append(cap2(leg, leg, [0, 0], [length, 0], RS, z=46, tone=TONE_SG - 0.04, region=R_LASH, material=MAT_SLING, **thin))
        sol = {"kind": "extruded", "sweep": "w", "z0": -RS, "z1": RS}
        fr.append(rect(leg, leg, 0, -RS, length, 2 * RS, z=46, tone=TONE_SG - 0.04, region=R_LASH, material=MAT_SLING, solid=sol, **thin))
        pl.append(rect(leg, leg, 0, -RS, length, 2 * RS, z=46, tone=TONE_SG - 0.04, region=R_LASH, material=MAT_SLING, solid=sol, **thin))
    return el, fr, pl


def bearers_sleepers(scale):
    """🪵 Bearers and sleepers of a scale: solids and the three views."""
    body, el, fr, pl = [], [], [], []
    bw = 0.075 if scale == 500 else SL_B
    rad = {"cornerRadius": 0.005} if scale == 50 else {}
    body.append(box("bearers", "bearers", MAT_SW, TONE_BR, [XS[0], BR / 2, 0], [BR, BR, BR_L], repeat={"count": 6, "translation": [0.75, 0, 0]}, **rad))
    body.append(box("sleepers", "sleepers", MAT_NEW, TONE_SL, [XSL[0], SL_H / 2, 0], [bw, SL_H, SL_L], repeat={"count": 7, "translation": [0.65, 0, 0]}))
    rr = {"r": 0.005} if scale == 50 else {}
    for j, x in enumerate(XS, 1):
        el.append(rect(f"bearer-{j}", "bearers", x - BR / 2, 0, BR, BR, z=2, tone=TONE_BR, region=R_BR, material=MAT_SW, **rr))
        pl.append(rect(f"bearer-{j}", "bearers", x - BR / 2, -BR_L / 2, BR, BR_L, z=2, tone=TONE_BR, region=R_BR, material=MAT_SW))
    fr.append(rect("bearer-6", "bearers", -BR_L / 2, 0, BR_L, BR, z=2, tone=TONE_BR, region=R_BR, material=MAT_SW))
    for i, x in enumerate(XSL, 1):
        el.append(rect(f"sleeper-{i}", "sleepers", x - bw / 2, 0, bw, SL_H, z=1, tone=TONE_SL, region=R_SL, material=MAT_NEW))
        pl.append(rect(f"sleeper-{i}", "sleepers", x - bw / 2, Z_SL - SL_L / 2, bw, SL_L, z=1, tone=TONE_SL, region=R_SL, material=MAT_NEW))
    fr.append(rect("sleeper-7", "sleepers", Z_SL - SL_L / 2, 0, SL_L, SL_H, z=1, tone=TONE_SL, region=R_SL, material=MAT_NEW))
    return body, el, fr, pl


def nail_marks():
    """🔩 Old nail holes of the first floor along both edges of every strip board at the former joist spacing of 0.80 m."""
    out = []
    for b in range(6):
        w0 = -HL + b * BW
        for k, x in enumerate([0.2 + 0.8 * n for n in range(5)]):
            for e, zz in (("g", w0 + 0.022), ("t", w0 + BW - 0.022)):
                out.append(line(f"nail-{b + 1}-{k + 1}-{e}", "strip", [[x - 0.0075, zz], [x + 0.0075, zz]], role="texture", z=30, tone=0.1, region=R_STRIP))
    return out


def stack_scale(scale):
    """🧱 Body, thermal, structure and the three views of the board stack at 1:50 or 1:100."""
    body, el, fr, pl = [], [], [], []
    pen = 0.13 if scale == 50 else 0.1
    for k in range(NL):
        v0 = lv(k)
        ds = LAYERS[k]
        if scale == 50:
            for b in range(6):
                body.append(solid("extrude", f"board-{k + 1:02d}-{b + 1}", "stack", MAT_BOARD, TONE_LAYER[b], profile=[[r6(p), r6(q)] for p, q in [[v0 + a, w] for a, w in board_profile(-HL + b * BW)]], plane="yz", **{"from": r6(-ds[b]), "to": r6(4.0 + ds[b])}))
        else:
            body.append(box(f"layer-{k + 1:02d}", "stack", MAT_BOARD, TONE_LAYER[k % 6], [2.0, v0 + BT / 2, 0], [4.0 + 2 * max(ds), BT, LW]))
        el.append(rect(f"layer-{k + 1:02d}", "stack", -max(ds), v0, 4.0 + 2 * max(ds), BT, z=10 + k * 0.01, tone=TONE_LAYER[k % 6], regions=[R_STACK], material=MAT_BOARD, penMm=pen))
        if scale == 50:
            fr.append(poly(f"layer-{k + 1:02d}", "stack", layer_front(wy_stack(v0)), z=10 + k * 0.01, tone=0.66, regions=[R_STACK], material=MAT_BOARD, penMm=pen))
        else:
            fr.append(rect(f"layer-{k + 1:02d}", "stack", -HL, wy_stack(v0), LW, BT, z=10 + k * 0.01, tone=0.66, regions=[R_STACK], material=MAT_BOARD, penMm=pen))
        for b in range(1, 6):
            zz = -HL + b * BW
            fr.append(line(f"joint-{k + 1:02d}-{b}", "stack", [[zz, wy_stack(v0)], [zz, wy_stack(v0 + BT)]], role="texture", z=12, tone=0.3, region=R_STACK))
        pl.append(rect(f"layer-{k + 1:02d}", "stack", -max(ds), -HL, 4.0 + 2 * max(ds), LW, z=10 + k * 0.01, tone=TONE_LAYER[k % 6], regions=[R_STACK], material=MAT_BOARD, penMm=pen))
        sv = v0 + BT
        for j, x in enumerate(XS, 1):
            el.append(rect(f"sticker-{j}-{k + 1:02d}", "stack", x - STK_B / 2, sv, STK_B, STK_T, z=20 + k * 0.01, tone=TONE_STK, regions=[R_STACK], material=MAT_SW, penMm=pen))
        fr.append(rect(f"sticker-6-{k + 1:02d}", "stack", -STK_L / 2, wy_stack(sv), STK_L, STK_T, z=20 + k * 0.01, tone=0.48, regions=[R_STACK], material=MAT_SW, penMm=pen))
    for j, x in enumerate(XS, 1):
        body.append(box(f"sticker-row-{j}", "stack", MAT_SW, TONE_STK, [x, BT + STK_T / 2, 0], [STK_B, STK_T, STK_L], repeat={"count": NL, "translation": [0, P, 0]}))
        pl.append(rect(f"sticker-{j}-{NL:02d}", "stack", x - STK_B / 2, -STK_L / 2, STK_B, STK_L, z=9, tone=TONE_STK, regions=[R_STACK], material=MAT_SW, penMm=pen))
    if scale == 50:
        for b in range(6):
            d = STRIP[b]
            body.append(solid("extrude", f"strip-board-{b + 1}", "strip", MAT_BOARD, TONE_STRIP + 0.02 * (b % 2), profile=[[r6(p), r6(q)] for p, q in board_profile(-HL + b * BW)], plane="yz", **{"from": r6(-d), "to": r6(4.0 + d)}))
            w0 = -HL + b * BW
            pl.append(rect(f"strip-board-{b + 1}", "strip", -d, w0, 4.0 + 2 * d, BW + (TG if b == 5 else 0), z=31 + b * 0.01, tone=TONE_STRIP + 0.02 * (b % 2), region=R_STRIP, material=MAT_BOARD, penMm=pen))
        fr.append(poly("strip", "strip", layer_front(wy_stack(STRIP_V)), z=31, tone=0.5, region=R_STRIP, material=MAT_BOARD, penMm=pen))
        pl += nail_marks()
    else:
        sd = max(STRIP)
        body.append(box("strip", "strip", MAT_BOARD, TONE_STRIP, [2.0, BT / 2, 0], [4.0 + 2 * sd, BT, LW]))
        fr.append(rect("strip", "strip", -HL, wy_stack(STRIP_V), LW, BT, z=31, tone=0.5, region=R_STRIP, material=MAT_BOARD, penMm=pen))
        for b in range(6):
            w0 = -HL + b * BW
            d = STRIP[b]
            pl.append(rect(f"strip-board-{b + 1}", "strip", -d, w0, 4.0 + 2 * d, BW + (TG if b == 5 else 0), z=31 + b * 0.01, tone=TONE_STRIP + 0.02 * (b % 2), region=R_STRIP, material=MAT_BOARD, penMm=pen))
    sd = max(STRIP)
    el.append(rect("strip", "strip", -sd, 0, 4.0 + 2 * sd, BT, z=31, tone=TONE_STRIP, region=R_STRIP, material=MAT_BOARD, penMm=pen))
    for b in range(1, 6):
        zz = -HL + b * BW
        fr.append(line(f"strip-joint-{b}", "strip", [[zz, wy_stack(STRIP_V)], [zz, wy_stack(STRIP_V + BT)]], role="texture", z=32, tone=0.3, region=R_STRIP))
    return body, el, fr, pl


def coarse_stack(scale):
    """🧱 The stack at 1:200 and 1:500: the nineteen stickered layers merged into one block, the strip as its own thin board layer."""
    body, el, fr, pl = [], [], [], []
    pen = 0.18 if scale == 200 else 0.13
    sd = max(STRIP)
    body.append(box("stack-block", "stack", MAT_BOARD, 0.58, [2.0, STRIP_V / 2, 0], [XMAX - XMIN, STRIP_V, LW], envelope=True))
    body.append(box("strip", "strip", MAT_BOARD, TONE_STRIP, [2.0, BT / 2, 0], [4.0 + 2 * sd, BT, LW], sheet=True))
    if scale == 200:
        top = STRIP_V + BT + LEG_V + RS
        regs = [R_STACK, R_STRIP, R_LASH]
        el.append(rect("stack-block", "stack", XMIN, 0, XMAX - XMIN, top, z=10, tone=0.58, regions=regs, material=MAT_BOARD, penMm=pen))
        for k in range(1, NL + 1):
            el.append(line(f"layer-line-{k:02d}", "stack", [[XMIN, lv(k) - STK_T / 2], [XMAX, lv(k) - STK_T / 2]], z=11, tone=0.35, region=R_STACK))
        fr.append(rect("stack-block", "stack", -HL, wy_stack(0), LW, top, z=10, tone=0.62, regions=regs, material=MAT_BOARD, penMm=pen))
        for b in range(1, 6):
            zz = -HL + b * BW
            fr.append(line(f"board-joint-{b}", "stack", [[zz, wy_stack(0)], [zz, wy_stack(STRIP_V + BT)]], role="texture", z=32, tone=0.3, region=R_STACK))
        pl.append(rect("stack-block", "stack", XMIN, -HL, XMAX - XMIN, LW, z=10, tone=0.58, regions=[R_STACK, R_LASH], material=MAT_BOARD, penMm=pen))
        pl.append(rect("strip", "strip", -sd, -HL, 4.0 + 2 * sd, LW, z=31, tone=TONE_STRIP, region=R_STRIP, material=MAT_BOARD, penMm=pen))
        for b in range(1, 6):
            zz = -HL + b * BW
            pl.append(line(f"strip-joint-{b}", "strip", [[-sd, zz], [4.0 + sd, zz]], role="texture", z=32, tone=0.3, region=R_STRIP))
    else:
        regs = [R_STACK, R_STRIP, R_LASH]
        top = STRIP_V + BT + LEG_V + RS
        el.append(rect("stack-block", "stack", XMIN, 0, XMAX - XMIN, top, z=10, tone=0.5, regions=regs, material=MAT_BOARD, penMm=pen))
        fr.append(rect("stack-block", "stack", -HL, wy_stack(0), LW, top, z=10, tone=0.55, regions=regs, material=MAT_BOARD, penMm=pen))
        pl.append(rect("stack-block", "stack", XMIN, -HL, XMAX - XMIN, LW, z=10, tone=0.45, regions=regs, material=MAT_BOARD, penMm=pen))
    return body, el, fr, pl


def masses_and_joints(scale):
    """⚖️ Mass markers at the centres of mass and joint solids at the leg pivots, sized for the scale."""
    rm = {50: 0.03, 100: 0.035, 200: 0.04, 500: 0.045}[scale]
    rj = {50: 0.02, 100: 0.022, 200: 0.025, 500: 0.04}[scale]
    out = [
        solid("ellipsoid", "mass-bearers", "bearers", MAT_SW, 0.5, radii=[rm, rm, rm], transform={"translation": [2.0, BR / 2, 0]}, member="bearers", part="mass"),
        solid("ellipsoid", "mass-sleepers", "sleepers", MAT_NEW, 0.5, radii=[rm, rm, rm], transform={"translation": [2.0, SL_H / 2, 0]}, member="sleepers", part="mass"),
        solid("ellipsoid", "mass-stack", "stack", MAT_BOARD, 0.5, radii=[rm, 0.012, rm] if scale == 50 else [rm, rm, rm], transform={"translation": [2.0, r6(lv(round((STACK_COM_V - BT / 2) / P)) + BT / 2 if scale == 50 else STACK_COM_V), 0]}, member="stack", part="mass"),
        solid("ellipsoid", "mass-strip", "strip", MAT_BOARD, 0.5, radii=[rm, 0.013 if scale in (50, 100) else rm, rm], transform={"translation": [2.0, BT / 2, 0]}, member="strip", part="mass"),
    ]
    for leg in ("leg-a1", "leg-b1"):
        out.append(solid("ellipsoid", f"joint-{leg}", leg, MAT_SLING, 0.7, radii=[rj, rj, rj], member=leg, part="joint"))
    for leg in ("leg-a2", "leg-b2"):
        out.append(solid("ellipsoid", f"joint-{leg}", leg, MAT_SLING, 0.7, radii=[rj, rj, rj], member=leg, part="joint"))
    out.append(solid("ellipsoid", "joint-load-carrier", "load-carrier", MAT_SW, 0.3, radii=[rj, rj, rj], member="load-carrier", part="joint"))
    return out


def lashing_area():
    """📏 Outer area of the straps and slings at 1:50, from their parts (cylinders of the slings, both faces of the straps)."""
    straps = 2 * (2 * ST_W * ((2 * HL + 2 * ST_T) + 2 * (TOP19 + ST_T) + 2 * (DROP + ST_T) + (2 * HL + 4 * ST_T)))
    loop = 2 * ZSG + 2 * (TOP19 + RS) + (2 * ZSG)
    slings = 2 * (2 * math.pi * RS * (loop + LEG) + 4 * math.pi * RS * RS)
    return straps + slings


def strip_area():
    """📏 Outer area of the six engaged strip boards (top, underside, the two long edges with tongue and groove, the ends)."""
    lens = [4.0 + 2 * d for d in STRIP]
    top = sum(lens) * BW * 2 + lens[5] * TG * 2
    edges = lens[0] * (BT + 2 * GD) + lens[5] * BT
    ends = 2 * 6 * PROFILE_AREA
    steps = sum(abs(lens[b] - lens[b + 1]) for b in range(5)) * BT
    return top + edges + ends + steps


def stack_area():
    """📏 Exchange area of the nineteen stickered layers and the 114 stickers (every face the air reaches through the 24 mm gaps)."""
    layers = 0.0
    for ds in LAYERS:
        lens = [4.0 + 2 * d for d in ds]
        layers += sum(lens) * BW * 2 + lens[5] * TG * 2 + lens[0] * (BT + 2 * GD) + lens[5] * BT + 12 * PROFILE_AREA + sum(abs(lens[b] - lens[b + 1]) for b in range(5)) * BT
    stickers = NL * 6 * 2 * (STK_L * STK_B + STK_L * STK_T + STK_B * STK_T)
    return layers + stickers


V_BOARD = sum(PROFILE_AREA * (4.0 + 2 * d) for ds in LAYERS for d in ds)
V_STRIP = sum(PROFILE_AREA * (4.0 + 2 * d) for d in STRIP)
V_STK = NL * 6 * STK_L * STK_B * STK_T
M_BOARDS = V_BOARD * RHO_PINE
M_STRIP = V_STRIP * RHO_PINE
M_STK = V_STK * RHO_SW
M_BR = 6 * BR * BR * BR_L * RHO_SW
M_SL = 7 * SL_B * SL_H * SL_L * RHO_SW
A_SL = math.pi * RS * RS
M_STRAP_LOW = 2 * ST_W * ST_T * ((2 * HL + 2 * ST_T) + 2 * (TOP19 + ST_T)) * RHO_PET
M_STRAP_UP = 2 * ST_W * ST_T * (2 * (DROP + ST_T) + (2 * HL + 4 * ST_T)) * RHO_PET
M_SLING_LOW = 2 * A_SL * (2 * ZSG + 2 * (TOP19 + RS)) * RHO_SLING
M_SLING_UP = 2 * (math.pi * RSU * RSU * 2 * (DROP + RS) + A_SL * 2 * ZSG) * RHO_SLING
M_LEG1 = A_SL * L1 * RHO_SLING
M_LEG2 = A_SL * L2 * RHO_SLING
M_STACK = M_BOARDS + M_STK + M_STRAP_LOW + M_SLING_LOW
M_PT = M_STRAP_UP + M_SLING_UP
STACK_COM_V = (M_BOARDS * (sum(lv(k) for k in range(NL)) / NL + BT / 2) + M_STK * (sum(lv(k) for k in range(NL)) / NL + BT + STK_T / 2) + (M_STRAP_LOW + M_SLING_LOW) * (TOP19 / 2)) / M_STACK
M_TOTAL = M_STACK + M_PT + M_STRIP + M_BR + M_SL + 2 * (M_LEG1 + M_LEG2)
HEIGHT = STACK_Y + PT_V + LEG_V + RS
SECTION_LAYER_I = LW * BT ** 3 / 12

A_STRIP = strip_area()
A_STACK = stack_area()
A_BR = 6 * (4 * BR * BR_L + 2 * BR * BR)
A_SLEEP = 7 * (2 * (SL_B + SL_H) * SL_L + 2 * SL_B * SL_H)
A_LASH = lashing_area()


def kg(x, com_x, com_y, dist="point", ref_id="wagenfuehr-holzatlas"):
    """⚖️ A segment mass."""
    return {"kg": round(x, 3), "comX": r6(com_x), "comY": r6(com_y), "distribution": dist, "source": "literature", "reference": ref_id}


def rig():
    """🦴 The structural rig: bearers (root, stay on the ground), sleepers, stack (lifts), pack-top (re-tensioned), the strip on a lateral carrier and the four sling-leg segments."""
    rev = lambda lo, hi: {"type": "revolute", "minDeg": lo, "maxDeg": hi}
    return {"kind": "structural", "root": "bearers", "bones": [
        {"id": "bearers", "parent": None, "title": T("Squared-timber bearers", "Kantholz-Unterleger"), "x": 0, "y": 0, "z": ZOFF, "angleDeg": 0, "lengthM": 4.0, "joint": {"type": "fixed"}, "mass": kg(M_BR, 2.0, BR / 2, ref_id="en-338")},
        {"id": "sleepers", "parent": "bearers", "title": T("Sleepers of the reinstalled strip", "Lagerhölzer des wiederverlegten Streifens"), "x": 0, "y": 0, "z": Z_SL, "angleDeg": 0, "lengthM": 4.0, "joint": {"type": "fixed"}, "mass": kg(M_SL, 2.0, SL_H / 2, ref_id="en-338")},
        {"id": "stack", "parent": "bearers", "title": T("Board stack, layers 1 to 19 with stickers", "Dielenstapel, Lagen 1 bis 19 mit Stapellatten"), "x": 0, "y": STACK_Y, "angleDeg": 0, "lengthM": 4.0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": 0, "maxM": 0.6}, "mass": kg(M_STACK, 2.0, STACK_COM_V, dist="uniform"), "section": {"areaM2": r6(NL * LW * BT), "secondMomentM4": float(f"{NL * SECTION_LAYER_I:.6e}"), "extremeFibreM": BT / 2, "material": MAT_BOARD, "title": T("Nineteen unconnected layers of six boards 28 x 137 mm", "Neunzehn unverbundene Lagen aus sechs Dielen 28 x 137 mm")}},
        {"id": "pack-top", "parent": "stack", "title": T("Top runs of straps and slings", "Obere Läufe von Bändern und Rundschlingen"), "x": 0, "y": PT_V, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": -DROP, "maxM": 0}, "mass": kg(M_PT, 2.0, 0.0, ref_id="en-1492-2")},
        {"id": "leg-a1", "parent": "pack-top", "title": T("Sling leg A, lower run", "Schlingenstrang A, unterer Lauf"), "x": XSG[0], "y": LEG_V, "angleDeg": 180, "lengthM": L1, "joint": rev(-135, 0), "mass": kg(M_LEG1, L1 / 2, 0, dist="uniform", ref_id="en-1492-2")},
        {"id": "leg-a2", "parent": "leg-a1", "title": T("Sling leg A, upper run to the hook", "Schlingenstrang A, oberer Lauf zum Haken"), "x": L1, "y": 0, "angleDeg": 0, "lengthM": r6(L2), "joint": rev(0, 10), "mass": kg(M_LEG2, L2 / 2, 0, dist="uniform", ref_id="en-1492-2")},
        {"id": "leg-b1", "parent": "pack-top", "title": T("Sling leg B, lower run", "Schlingenstrang B, unterer Lauf"), "x": XSG[1], "y": LEG_V, "angleDeg": 0, "lengthM": L1, "joint": rev(0, 135), "mass": kg(M_LEG1, L1 / 2, 0, dist="uniform", ref_id="en-1492-2")},
        {"id": "leg-b2", "parent": "leg-b1", "title": T("Sling leg B, upper run to the hook", "Schlingenstrang B, oberer Lauf zum Haken"), "x": L1, "y": 0, "angleDeg": 0, "lengthM": r6(L2), "joint": rev(-10, 0), "mass": kg(M_LEG2, L2 / 2, 0, dist="uniform", ref_id="en-1492-2")},
        {"id": "strip-carrier", "parent": "stack", "title": T("Carrier of the strip (lateral frame)", "Träger des Streifens (seitlicher Rahmen)"), "x": 0, "y": STRIP_V, "angleDeg": 0, "abductionDeg": 90, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "strip-shift", "parent": "strip-carrier", "title": T("Lateral travel of the strip", "Seitlicher Weg des Streifens"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": STRIP_DZ}},
        {"id": "strip", "parent": "strip-shift", "title": T("Top layer: the strip of six boards", "Oberste Lage: der Streifen aus sechs Dielen"), "x": 0, "y": 0, "angleDeg": 0, "abductionDeg": -90, "lengthM": 4.0, "joint": {"type": "prismatic", "axisDeg": 90, "minM": r6(STRIP_DY), "maxM": 0}, "mass": kg(M_STRIP, 2.0, BT / 2, dist="uniform"), "section": {"areaM2": r6(LW * BT), "secondMomentM4": float(f"{SECTION_LAYER_I:.6e}"), "extremeFibreM": BT / 2, "material": MAT_BOARD, "title": T("Six boards 28 x 137 mm side by side, 830 mm", "Sechs Dielen 28 x 137 mm nebeneinander, 830 mm")}},
        {"id": "load-carrier", "parent": "strip", "title": T("Load carrier (massless, moves the walking load)", "Lastträger (masselos, bewegt die Gehlast)"), "x": 0.2, "y": BT, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 3.6}},
    ]}


def anchors():
    """📍 Contacts of the stack on its bearers, of the sleepers and of the reinstalled strip on the sleepers, the two crane-hook pins and the load point."""
    out = []
    for j, x in enumerate(XS, 1):
        out.append({"id": f"stack-on-bearer-{j}", "bone": "stack", "x": r6(x), "y": 0, "kind": "contact", "title": T(f"Bottom layer on bearer {j}", f"Unterste Lage auf Unterleger {j}"), "support": "contact", "normalDeg": 90, "footprintLengthM": BR, "footprintWidthM": r6(LW), "frictionCoefficient": 0.5})
    for i, x in enumerate(XSL, 1):
        out.append({"id": f"sleeper-on-ground-{i}", "bone": "sleepers", "x": r6(x), "y": 0, "kind": "contact", "title": T(f"Sleeper {i} on the yard slab", f"Lagerholz {i} auf der Hofplatte"), "support": "contact", "normalDeg": 90, "footprintLengthM": SL_B, "footprintWidthM": SL_L, "frictionCoefficient": 0.5})
    for i, x in enumerate(XSL, 1):
        out.append({"id": f"strip-on-sleeper-{i}", "bone": "strip", "x": r6(x), "y": 0, "kind": "contact", "title": T(f"Reinstalled strip on sleeper {i}", f"Wiederverlegter Streifen auf Lagerholz {i}"), "support": "contact", "normalDeg": 90, "footprintLengthM": SL_B, "footprintWidthM": r6(LW), "frictionCoefficient": 0.4})
    out.append({"id": "hook-a", "bone": "leg-a2", "x": r6(L2), "y": 0, "kind": "support", "title": T("Crane hook, eye of sling A", "Kranhaken, Auge der Schlinge A"), "support": "pin"})
    out.append({"id": "hook-b", "bone": "leg-b2", "x": r6(L2), "y": 0, "kind": "support", "title": T("Crane hook, eye of sling B", "Kranhaken, Auge der Schlinge B"), "support": "pin"})
    out.append({"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": T("Walking load on the strip", "Gehlast auf dem Streifen"), "z": 0})
    return out


BEARER_C = [f"stack-on-bearer-{j}" for j in range(1, 7)]
SLEEPER_C = [f"sleeper-on-ground-{i}" for i in range(1, 8)]
STRIP_C = [f"strip-on-sleeper-{i}" for i in range(1, 8)]
LIFTED = {"stack": {"dy": LIFT}, "leg-a1": {"angleDeg": -135}, "leg-b1": {"angleDeg": 135}}
TENSIONED = {"leg-a1": {"angleDeg": -135}, "leg-b1": {"angleDeg": 135}}
DRAPE1 = 3.8
DRAPE2 = 6.17
REINSTALLED = {"pack-top": {"dy": -DROP}, "leg-a1": {"angleDeg": -DRAPE1}, "leg-a2": {"angleDeg": DRAPE2}, "leg-b1": {"angleDeg": DRAPE1}, "leg-b2": {"angleDeg": -DRAPE2}, "strip-shift": {"dx": STRIP_DZ}, "strip": {"dy": r6(STRIP_DY)}}


def binding(strip_sun, sleeper_sun, sleeper_exposed):
    """🌤 The thermal binding of a pose (every region)."""
    return {"regions": {R_STRIP: {"exposedFraction": 1, "sunlitFraction": strip_sun}, R_STACK: {"exposedFraction": 1, "sunlitFraction": 0.04}, R_BR: {"exposedFraction": 1, "sunlitFraction": 0.1}, R_SL: {"exposedFraction": sleeper_exposed, "sunlitFraction": sleeper_sun}, R_LASH: {"exposedFraction": 1, "sunlitFraction": 0.4}}, "emitters": {}}


IMPOSED = {"id": "imposed-floor", "kind": "imposed", "title": T("Imposed floor load 2 kN/m² over the 0.83 m strip, 1.66 kN/m", "Nutzlast 2 kN/m² über den 0,83 m breiten Streifen, 1,66 kN/m"), "fxN": 0, "fyN": 0, "distributed": {"bone": "strip", "fromM": 0, "toM": 4.0, "wxNPerM": 0, "wyNPerM": -round(2000 * LW, 1)}, "source": "literature", "reference": "en-1991-1-1"}
POINT = {"id": "walking-load", "kind": "live", "title": T("Concentrated load 2 kN (a person walking)", "Einzellast 2 kN (eine gehende Person)"), "anchor": "load-point", "fxN": 0, "fyN": -2000, "source": "literature", "reference": "en-1991-1-1"}


def poses():
    """🧍 Stored (default), slings tensioned (key), lifted, reinstalled and reinstalled under the walking load."""
    return [
        {"id": "stored", "title": T("Stored on bearers", "Gelagert auf Unterlegern"), "description": T("Twenty stickered layers on six squared-timber bearers in the yard, two PET straps and two round slings already slung under the pack, their legs lying folded on top; the sleepers of a trial floor wait beside it.", "Zwanzig Lagen mit Stapellatten auf sechs Kantholz-Unterlegern im Hof, zwei PET-Bänder und zwei Rundschlingen schon unter dem Paket angeschlagen, ihre Stränge liegen gefaltet obenauf; daneben warten die Lagerhölzer eines Probebodens."), "bones": {}, "ground": "authored", "contacts": BEARER_C + SLEEPER_C, "thermal": binding(0.47, 0.5, 1), "loads": []},
        {"id": "slings-tensioned", "key": True, "title": T("Slings hooked and tensioned", "Schlingen eingehängt und gespannt"), "description": T("Both sling legs raised to the crane hook at 45 degrees, the pack still resting on its bearers.", "Beide Schlingenstränge unter 45 Grad zum Kranhaken hochgezogen, das Paket liegt noch auf den Unterlegern."), "bones": TENSIONED, "ground": "authored", "contacts": BEARER_C + SLEEPER_C, "thermal": binding(0.47, 0.5, 1), "loads": []},
        {"id": "lifted", "title": T("Lifted on two round slings", "Angehoben an zwei Rundschlingen"), "description": T("The crane holds the pack 300 mm above the bearers on two choked round slings (two-leg, 45 degrees); bearers and sleepers stay on the ground.", "Der Kran hält das Paket 300 mm über den Unterlegern an zwei geschnürten Rundschlingen (zweisträngig, 45 Grad); Unterleger und Lagerhölzer bleiben am Boden."), "bones": LIFTED, "ground": "authored", "contacts": ["hook-a", "hook-b"] + SLEEPER_C, "thermal": binding(0.47, 0.5, 1), "loads": []},
        {"id": "reinstalled", "title": T("Strip reinstalled on sleepers", "Streifen auf Lagerhölzern wiederverlegt"), "description": T("The top layer of six boards is laid tongue in groove on seven sleepers at 650 mm and carries the imposed floor load of 2 kN/m²; straps and slings are re-tensioned on the nineteen remaining layers.", "Die oberste Lage aus sechs Dielen ist Feder in Nut auf sieben Lagerhölzern im Abstand von 650 mm verlegt und trägt die Nutzlast von 2 kN/m²; Bänder und Schlingen sind auf den neunzehn übrigen Lagen nachgespannt."), "bones": REINSTALLED, "ground": "authored", "contacts": BEARER_C + STRIP_C, "thermal": binding(0.5, 0.1, 0.6), "loads": [IMPOSED]},
        {"id": "reinstalled-point-load", "title": T("Reinstalled strip, person at midspan", "Wiederverlegter Streifen, Person in der Mitte"), "description": T("A concentrated load of 2 kN stands on the reinstalled strip at 2.0 m, between the sleepers at 1.35 and 2.0 m.", "Eine Einzellast von 2 kN steht bei 2,0 m auf dem wiederverlegten Streifen über dem Lagerholz bei 2,0 m."), "bones": {**REINSTALLED, "load-carrier": {"dx": 1.8}}, "ground": "authored", "contacts": BEARER_C + STRIP_C, "thermal": binding(0.5, 0.1, 0.6), "loads": [POINT]},
    ]


def clips():
    """🎞 The lift (stored, slings tensioned, lifted) and the walking load on the reinstalled strip."""
    return [
        {"id": "lift", "title": T("Lift off the bearers", "Abheben von den Unterlegern"), "description": T("The crane takes up the two sling legs (0 to 3 s) and lifts the pack 300 mm off its bearers (3 to 8 s); bearers and sleepers stay behind.", "Der Kran nimmt die beiden Schlingenstränge auf (0 bis 3 s) und hebt das Paket 300 mm von seinen Unterlegern ab (3 bis 8 s); Unterleger und Lagerhölzer bleiben liegen."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 3, "pose": "slings-tensioned", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 3} for a in BEARER_C] + [{"anchor": a, "fromS": 3, "toS": 8} for a in ("hook-a", "hook-b")] + [{"anchor": a, "fromS": 0, "toS": 8} for a in SLEEPER_C],
         "phases": [{"id": "take-up", "fromS": 0, "toS": 3, "title": T("Take up the slings", "Schlingen aufnehmen")}, {"id": "hoist", "fromS": 3, "toS": 8, "title": T("Hoist", "Heben")}],
         "events": [{"t": 3, "kind": "contact-off", "title": T("The pack lifts off the bearers", "Das Paket hebt von den Unterlegern ab")}, {"t": 3, "kind": "contact-on", "title": T("The hook carries the pack", "Der Haken trägt das Paket")}], "tracks": []},
        {"id": "walking-load", "title": T("Walking over the reinstalled strip", "Gang über den wiederverlegten Streifen"), "description": T("A person (2 kN) walks from 0.2 m to 3.8 m along the reinstalled strip, over the seven sleepers.", "Eine Person (2 kN) geht von 0,2 m bis 3,8 m über den wiederverlegten Streifen und die sieben Lagerhölzer."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-point-load", "ease": "linear", "bones": {"load-carrier": {"dx": 0}}}, {"t": 6, "pose": "reinstalled-point-load", "ease": "linear", "bones": {"load-carrier": {"dx": 3.6}}}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 6} for a in BEARER_C + STRIP_C],
         "phases": [{"id": "walk", "fromS": 0, "toS": 6, "title": T("Walk", "Gehen")}],
         "events": [{"t": 0, "kind": "motion", "title": T("The person steps onto the strip", "Die Person betritt den Streifen")}], "tracks": []},
    ]


def thermal():
    """🌡 Passive regions: painted strip, the ventilated stack (exchange area), bearers, sleepers and the polyester lashing."""
    hc = round(PROFILE_AREA * RHO_PINE * 1600 * 6 / (2 * BW * 6), -2)
    return {
        "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
        "operating": [],
        "regions": [
            {"id": R_STRIP, "title": T("Strip: top layer of six boards, old floor paint on the top face", "Streifen: oberste Lage aus sechs Dielen, alte Fußbodenfarbe auf der Oberseite"), "kind": "surface", "model": "passive", "material": MAT_BOARD, "areaM2": round(A_STRIP, 3), "characteristicLengthM": round(LW, 3), "geometry": "horizontal-plate-up", "skyViewFactor": 0.9, "layers": [], "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "en-iso-13786"},
            {"id": R_STACK, "title": T("Nineteen stickered layers and their stickers", "Neunzehn Lagen mit Stapellatten"), "kind": "surface", "model": "passive", "material": MAT_WEATHERED, "areaM2": round(A_STACK, 2), "areaBasis": "exchange", "characteristicLengthM": round(LW, 3), "geometry": "horizontal-plate-up", "skyViewFactor": 0.1, "layers": [], "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "en-iso-13786"},
            {"id": R_BR, "title": T("Squared-timber bearers", "Kantholz-Unterleger"), "kind": "surface", "model": "passive", "material": MAT_SW, "areaM2": round(A_BR, 3), "characteristicLengthM": BR, "geometry": "horizontal-plate-down", "skyViewFactor": 0.1, "layers": [], "heatCapacityJM2K": round(0.025 * RHO_SW * 1600, -2), "source": "authored-estimate", "reference": "en-iso-13786"},
            {"id": R_SL, "title": T("Sleepers of the trial floor", "Lagerhölzer des Probebodens"), "kind": "surface", "model": "passive", "material": MAT_NEW, "areaM2": round(A_SLEEP, 3), "characteristicLengthM": SL_B, "geometry": "horizontal-plate-up", "skyViewFactor": 0.8, "layers": [], "heatCapacityJM2K": round(0.015 * RHO_SW * 1600, -2), "source": "authored-estimate", "reference": "en-iso-13786"},
            {"id": R_LASH, "title": T("PET straps and polyester round slings", "PET-Umreifungsbänder und Polyester-Rundschlingen"), "kind": "surface", "model": "passive", "material": MAT_SLING, "areaM2": round(A_LASH, 3), "characteristicLengthM": 2 * RS, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 1300, "source": "authored-estimate", "reference": "en-iso-10456"},
        ],
        "emitters": [],
        "assumptions": [
            T("All parts are passive timber or polymer in the open yard; nothing on the stack draws power, so the total power is 0 W.", "Alle Teile sind passives Holz oder Kunststoff im offenen Hof; nichts am Stapel nimmt Leistung auf, die Gesamtleistung ist daher 0 W."),
            T("The 24 mm sticker gaps ventilate every layer: the 19 lower layers and their stickers exchange heat over their whole board surface (exchange area), which the drawn envelope of the stack cannot show.", "Die 24-mm-Lattenfugen belüften jede Lage: die 19 unteren Lagen und ihre Stapellatten tauschen Wärme über ihre ganze Dielenoberfläche aus (Austauschfläche), die die gezeichnete Hülle des Stapels nicht zeigen kann."),
            T("Old ox-blood floor paint on the top faces absorbs more sun (0.75) than the grey weathered undersides and edges (0.65) and than new planed softwood (0.5); emissivities stay at 0.9 (authored estimates after ASHRAE Fundamentals).", "Alte ochsenblutrote Fußbodenfarbe auf den Oberseiten absorbiert mehr Sonne (0,75) als die grau verwitterten Unterseiten und Kanten (0,65) und als neues gehobeltes Nadelholz (0,5); die Emissionsgrade bleiben bei 0,9 (Schätzungen nach ASHRAE Fundamentals)."),
            T("Heat capacity per area from half the board thickness (14 mm of pine at 510 kg/m3 and 1600 J/kgK), the effective storage depth of a board exposed on both faces (EN ISO 13786).", "Wärmekapazität je Fläche aus der halben Dielendicke (14 mm Kiefer mit 510 kg/m3 und 1600 J/kgK), der wirksamen Speichertiefe einer beidseitig umströmten Diele (EN ISO 13786)."),
        ],
        "passengers": [],
    }


def structure():
    """🏗 Quasi-static; supports are the contacts of each pose, so the support set changes from bearers to hook to sleepers."""
    return {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
        T("The supports change with the pose and are its contacts: stored, the bottom layer bears on six bearers and the sleepers on the slab; lifted, the two sling eyes hang on the crane hook (pins) at 45 degrees; reinstalled, the stack bears on its bearers and the strip on its seven sleepers.", "Die Auflager wechseln mit der Pose und sind ihre Kontakte: gelagert liegt die unterste Lage auf sechs Unterlegern und die Lagerhölzer auf der Platte; angehoben hängen die beiden Schlingenaugen unter 45 Grad am Kranhaken (Gelenke); wiederverlegt liegt der Stapel auf seinen Unterlegern und der Streifen auf seinen sieben Lagerhölzern."),
        T("The product solves the whole asset as one rigid body. The stack and the trial floor lie on two lines 1.3 m apart, so the load on one line is carried by the contacts of that line alone; the 25 kg of the bearers are carried with the pack when it is lifted (2.4 % of the hook load), because bearers on the ground and the hook lie on one line.", "Das Produkt löst das ganze Asset als einen starren Körper. Stapel und Probeboden liegen auf zwei Linien im Abstand von 1,3 m, daher tragen die Kontakte einer Linie die Last dieser Linie allein; die 25 kg der Unterleger werden beim Anheben mit dem Paket getragen (2,4 % der Hakenlast), weil Unterleger am Boden und Haken auf einer Linie liegen."),
        T("The stickered layers act as nineteen unconnected plates of six boards (no shear transfer through the stickers): section 19 x 830 x 28 mm, second moment 19 x 1.52e-6 m4, extreme fibre 14 mm; stickers sit over the bearers so each layer bears straight down.", "Die Lagen mit Stapellatten wirken als neunzehn unverbundene Platten aus sechs Dielen (kein Schub über die Latten): Querschnitt 19 x 830 x 28 mm, Flächenmoment 19 x 1,52e-6 m4, Randfaser 14 mm; die Latten liegen über den Unterlegern, jede Lage trägt also senkrecht ab."),
        T("Reclaimed pine taken as strength class C24 (EN 338) after visual grading; nail holes along the edges reduce the section by less than 1 % and are neglected.", "Gebrauchtes Kiefernholz nach Sichtsortierung als Festigkeitsklasse C24 (EN 338) angenommen; die Nagellöcher an den Kanten mindern den Querschnitt um weniger als 1 % und werden vernachlässigt."),
    ], "clinical": False}


def materials():
    """🧱 The six materials with weathered and painted surface properties."""
    return [
        {"id": MAT_BOARD, "title": T("Reclaimed pine floorboard, painted top face", "Gebrauchte Kiefernholzdiele, gestrichene Oberseite"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "densityKgM3": RHO_PINE, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "wagenfuehr-holzatlas"},
        {"id": MAT_WEATHERED, "title": T("Weathered pine, raw undersides and edges", "Verwittertes Kiefernholz, rohe Unterseiten und Kanten"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_PINE, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "source": "authored-estimate", "reference": "emissivity-literature"},
        {"id": MAT_SW, "title": T("Weathered softwood C24, bearers and stickers", "Verwittertes Nadelholz C24, Unterleger und Stapellatten"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": RHO_SW, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
        {"id": MAT_NEW, "title": T("New planed softwood C24, sleepers", "Neues gehobeltes Nadelholz C24, Lagerhölzer"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": RHO_SW, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
        {"id": MAT_PET, "title": T("PET strapping 19 x 0.8 mm", "PET-Umreifungsband 19 x 0,8 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": RHO_PET, "conductivityWmK": 0.2, "specificHeatJKgK": 1200, "source": "authored-estimate", "reference": "en-iso-10456"},
        {"id": MAT_SLING, "title": T("Polyester round sling, violet sleeve (WLL 1 t)", "Polyester-Rundschlinge, violetter Schlauch (WLL 1 t)"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": RHO_SLING, "conductivityWmK": 0.06, "specificHeatJKgK": 1300, "source": "authored-estimate", "reference": "en-1492-2"},
    ]


def provenance():
    """📚 References."""
    return {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": [
        {"id": "din-4072", "citation": "DIN 4072:1977-08 Gespundete Bretter aus Nadelholz (tongued and grooved softwood boards): profile of tongue and groove; the 28 mm board with about 8 mm tongue and a cover width near 135 mm is the common historic floorboard (dimensions of the modelled board are authored estimates within this series)."},
        {"id": "en-338", "citation": "EN 338:2016 Structural timber, strength classes, Table 1: C24 f_m,k 24 MPa, E_0,mean 11 GPa, rho_mean 420 kg/m3."},
        {"id": "wagenfuehr-holzatlas", "citation": "Wagenführ, R.; Wagenführ, A.: Holzatlas, 7th ed., Carl Hanser, Munich 2022, entry Kiefer (Pinus sylvestris): mean density at 12 % moisture about 510 kg/m3, bending modulus about 11 GPa."},
        {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1, Part 1-1, Tables 6.1 and 6.2: floor category A, imposed load q_k 2.0 kN/m2 and concentrated load Q_k 2.0 kN."},
        {"id": "en-1492-2", "citation": "EN 1492-2:2000+A1:2008 Textile slings, roundslings made of man-made fibres: colour code violet for a working load limit of 1000 kg, mode factors for choke hitch (0.8) and two-leg slings up to 45 degrees (1.4)."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties: softwood at 500 kg/m3 0.13 W/mK and 1600 J/kgK; polymers 0.2 W/mK."},
        {"id": "en-iso-13786", "citation": "EN ISO 13786:2017 Thermal performance of building components, dynamic thermal characteristics: periodic penetration depth delta = sqrt(lambda T / (pi rho c)); a 28 mm board exposed on both faces stores in half its thickness."},
        {"id": "emissivity-literature", "citation": "ASHRAE Handbook Fundamentals (2021), thermal radiation properties of building materials (typical emissivity and solar absorptance of wood, painted and weathered surfaces; rounded values guided by this chapter, not table look-ups)."},
        {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on sections and cuts."},
    ], "notes": T(
        f"One hundred and twenty reclaimed pine floorboards 28 x 145 mm (137 mm cover width, 8 mm tongue), about 4.0 m long, in twenty layers of six boards engaged tongue in groove (830 mm), separated by 24 x 48 mm stickers over six 100 x 100 mm bearers at 750 mm; the pack weighs {M_TOTAL:.0f} kg with straps, slings and the sleepers of the trial floor.",
        f"Hundertzwanzig gebrauchte Kiefernholzdielen 28 x 145 mm (137 mm Deckbreite, 8 mm Feder), etwa 4,0 m lang, in zwanzig Lagen zu sechs Dielen Feder in Nut (830 mm), getrennt durch Stapellatten 24 x 48 mm über sechs Unterlegern 100 x 100 mm im Abstand von 750 mm; das Paket wiegt mit Bändern, Schlingen und den Lagerhölzern des Probebodens {M_TOTAL:.0f} kg.")}


def decisions(scale):
    """📝 The representation decisions of a scale."""
    if scale == 50:
        return {
            "silhouette": T("Side elevation of the stack as it stands: nineteen 28 mm layers (0.56 mm) with the 24 mm sticker gaps open between the six sticker rows, the 100 mm bearers under the sticker rows, the round slings and PET straps over the side and the sling legs lying folded on top; the sleepers of the trial floor show between the bearers behind. Front view: the end grain of twenty layers with the open groove at the left and the proud tongue at the right, sticker strips in the gaps. Plan: the painted top layer of six boards with the old nail holes, the strap and sling runs and the bearer ends beside the pack; the sleepers lie at 1.3 m to the left.", "Seitenansicht des Stapels, wie er steht: neunzehn Lagen zu 28 mm (0,56 mm) mit offenen 24-mm-Lattenfugen zwischen den sechs Lattenreihen, die 100-mm-Unterleger unter den Lattenreihen, Rundschlingen und PET-Bänder über die Seite und die gefalteten Schlingenstränge obenauf; die Lagerhölzer des Probebodens zeigen sich dahinter zwischen den Unterlegern. Vorderansicht: das Hirnholz der zwanzig Lagen mit offener Nut links und vorstehender Feder rechts, Lattenstreifen in den Fugen. Draufsicht: die gestrichene oberste Lage aus sechs Dielen mit den alten Nagellöchern, Band- und Schlingenläufe und die Enden der Unterleger neben dem Paket; die Lagerhölzer liegen 1,3 m links."),
            "retained": T("Every board with its true tongue (9 x 8 mm) and groove (10 x 10 mm) in the front view, the ragged ends of the reclaimed layers (up to 24 mm proud), all 114 sticker ends, six bearers, seven sleepers, both straps and slings with their choke points and the four leg runs.", "Jede Diele mit ihrer wahren Feder (9 x 8 mm) und Nut (10 x 10 mm) in der Vorderansicht, die unregelmäßigen Enden der gebrauchten Lagen (bis 24 mm vorstehend), alle 114 Lattenenden, sechs Unterleger, sieben Lagerhölzer, beide Bänder und Schlingen mit ihren Schnürpunkten und die vier Strangläufe."),
            "merged": T("In the side elevation the six boards of a layer are one strip (the boards behind the outer one are hidden); in the front view only the sticker row nearest to the viewer and the front bearer and sleeper are drawn, the others lie behind them.", "In der Seitenansicht sind die sechs Dielen einer Lage ein Streifen (die Dielen hinter der äußeren sind verdeckt); in der Vorderansicht sind nur die vorderste Lattenreihe sowie vorderer Unterleger und vorderes Lagerholz gezeichnet, die übrigen liegen dahinter."),
            "omitted": T("The relief grooves on the board undersides (3 mm), the 4 mm nail holes as real holes and the steel seals of the straps; the 0.8 mm strap runs over the stack sides in the front view.", "Die Entlastungsnuten der Dielenunterseiten (3 mm), die 4-mm-Nagellöcher als echte Löcher und die Stahlplomben der Bänder; die 0,8-mm-Bandläufe über die Stapelseiten in der Vorderansicht."),
            "thinElements": T("The PET straps (19 x 0.8 mm) are filled bands of 0.38 mm with a 0.13 mm pen; nail holes are 15 mm texture marks at the old joist spacing of 0.80 m along both edges of every board of the top layer.", "Die PET-Bänder (19 x 0,8 mm) sind gefüllte Bänder von 0,38 mm mit 0,13-mm-Feder; Nagellöcher sind 15-mm-Texturmarken im alten Balkenabstand von 0,80 m an beiden Kanten jeder Diele der obersten Lage."),
            "hatching": T("No hatch: the layer strips and sticker ends are the reading of the stack.", "Keine Schraffur: Lagenstreifen und Lattenenden sind die Lesart des Stapels."),
            "styles": T("Layers and stickers in 0.13 mm outlines so the 24 mm gaps stay white to 1:75, bearers and slings 0.35 mm, board joints and nail marks 0.1 mm; the painted top layer darker than the weathered layers.", "Lagen und Latten mit 0,13-mm-Konturen, damit die 24-mm-Fugen bis 1:75 weiß bleiben, Unterleger und Schlingen 0,35 mm, Dielenfugen und Nagelmarken 0,1 mm; die gestrichene oberste Lage dunkler als die verwitterten Lagen."),
            "overlays": T("Layer and sticker cells carry the stack region, the top layer the strip region, bearers, sleepers and lashing their own.", "Lagen- und Lattenzellen tragen die Region des Stapels, die oberste Lage die des Streifens, Unterleger, Lagerhölzer und Anschlagmittel ihre eigenen."),
        }
    if scale == 100:
        return {
            "silhouette": T("Side elevation with the nineteen layers (0.28 mm) and their 24 mm gaps (0.24 mm), all sticker ends, bearers, sleepers, straps, slings and the folded legs; front view of layer strips and sticker strips with the six board joints per layer; plan of the six top boards and the lashing runs.", "Seitenansicht mit den neunzehn Lagen (0,28 mm) und ihren 24-mm-Fugen (0,24 mm), allen Lattenenden, Unterlegern, Lagerhölzern, Bändern, Schlingen und den gefalteten Strängen; Vorderansicht aus Lagen- und Lattenstreifen mit den sechs Dielenfugen je Lage; Draufsicht der sechs obersten Dielen und der Anschlagläufe."),
            "retained": T("Layer count and gaps, ragged layer ends, sticker rows over the bearers, straps, slings and legs.", "Lagenzahl und Fugen, unregelmäßige Lagenenden, Lattenreihen über den Unterlegern, Bänder, Schlingen und Stränge."),
            "merged": T("The tongue and groove (8 to 10 mm, 0.1 mm) are merged into a plain layer strip; each layer is one strip in every view.", "Feder und Nut (8 bis 10 mm, 0,1 mm) gehen in einem glatten Lagenstreifen auf; jede Lage ist in jeder Ansicht ein Streifen."),
            "omitted": T("Nail holes, relief grooves, the 5 mm rounding of the old bearers and the strap seals.", "Nagellöcher, Entlastungsnuten, die 5-mm-Rundung der alten Unterleger und die Bandplomben."),
            "thinElements": T("Straps are 0.19 mm bands with a 0.13 mm pen, slings 0.32 mm; slings and legs in 0.1 mm outlines; in the side elevation the top runs of straps and slings show only end-on (the sling run as its 32 mm circle), so nothing of the lashing covers the top layer when the runs are re-tensioned onto layer 19.", "Bänder sind 0,19-mm-Bänder mit 0,13-mm-Feder, Schlingen 0,32 mm; Schlingen und Stränge mit 0,1-mm-Konturen; in der Seitenansicht erscheinen die oberen Läufe von Bändern und Schlingen nur im Schnitt (der Schlingenlauf als 32-mm-Kreis), damit beim Nachspannen auf Lage 19 nichts von den Anschlagmitteln die oberste Lage überdeckt."),
            "hatching": T("No hatch.", "Keine Schraffur."),
            "styles": T("Layers and stickers 0.1 mm outlines, bearers and slings 0.25 mm.", "Lagen und Latten mit 0,1-mm-Konturen, Unterleger und Schlingen 0,25 mm."),
            "overlays": T("As at 1:50: stack, strip, bearers, sleepers and lashing each their own cells.", "Wie bei 1:50: Stapel, Streifen, Unterleger, Lagerhölzer und Anschlagmittel je eigene Zellen."),
        }
    if scale == 200:
        return {
            "silhouette": T("The pack as one block 4.05 x 1.15 m (20.2 x 5.7 mm) with the twenty layer joints as texture lines, on six bearers, the sleepers beside it; plan of the top layer with its six boards.", "Das Paket als ein Block 4,05 x 1,15 m (20,2 x 5,7 mm) mit den zwanzig Lagenfugen als Texturlinien, auf sechs Unterlegern, daneben die Lagerhölzer; Draufsicht der obersten Lage mit ihren sechs Dielen."),
            "retained": T("Overall length and height, the layer joints, the board joints of the top layer in plan, bearer and sleeper spacing.", "Gesamtlänge und -höhe, die Lagenfugen, die Dielenfugen der obersten Lage in der Draufsicht, Abstand der Unterleger und Lagerhölzer."),
            "merged": T("All layers with their stickers, the top layer and the folded slings are one block in elevation and front view: the 24 mm gaps are 0.12 mm on paper and close in print, and the 28 mm top layer and 32 mm slings stay under 0.2 mm towards 1:350; the 3D model keeps the top layer and slings as their own parts.", "Alle Lagen mit ihren Latten, die oberste Lage und die gefalteten Schlingen sind in Seiten- und Vorderansicht ein Block: die 24-mm-Fugen messen 0,12 mm auf Papier und schließen sich im Druck, oberste Lage (28 mm) und Schlingen (32 mm) bleiben gegen 1:350 unter 0,2 mm; das 3D-Modell behält oberste Lage und Schlingen als eigene Teile."),
            "omitted": T("Sticker ends, PET straps (0.1 mm), ragged ends (under 30 mm), nail holes.", "Lattenenden, PET-Bänder (0,1 mm), unregelmäßige Enden (unter 30 mm), Nagellöcher."),
            "thinElements": T("Layer joints as 0.1 mm texture lines at the middle of each sticker gap; the slings are drawn in no view.", "Lagenfugen als 0,1-mm-Texturlinien in der Mitte jeder Lattenfuge; die Schlingen sind in keiner Ansicht gezeichnet."),
            "hatching": T("No hatch; layer joints as 0.1 mm texture lines.", "Keine Schraffur; Lagenfugen als 0,1-mm-Texturlinien."),
            "styles": T("Outline 0.18 mm, texture 0.1 mm.", "Kontur 0,18 mm, Textur 0,1 mm."),
            "overlays": T("The block carries the stack, strip and lashing regions merged; in plan the top layer carries the strip region.", "Der Block trägt die Regionen Stapel, Streifen und Anschlagmittel zusammengefasst; in der Draufsicht trägt die oberste Lage die Region des Streifens."),
        }
    return {
        "silhouette": T("The stack as one block 4.05 x 1.12 m (8.1 x 2.2 mm) on six bearers, beside it seven sleepers.", "Der Stapel als ein Block 4,05 x 1,12 m (8,1 x 2,2 mm) auf sechs Unterlegern, daneben sieben Lagerhölzer."),
        "retained": T("Length, height including the top layer, bearer spacing and the trial floor beside.", "Länge, Höhe einschließlich der obersten Lage, Abstand der Unterleger und der Probeboden daneben."),
        "merged": T("All twenty layers, stickers, straps and slings are one block; the strip, stack and lashing regions are merged in its cell.", "Alle zwanzig Lagen, Latten, Bänder und Schlingen sind ein Block; die Regionen Streifen, Stapel und Anschlagmittel sind in seiner Zelle zusammengefasst."),
        "omitted": T("Sling legs (0.06 mm), straps, sticker ends and board joints.", "Schlingenstränge (0,06 mm), Bänder, Lattenenden und Dielenfugen."),
        "thinElements": T("Sleepers drawn at 75 mm, the smallest feature of the scale (true 60 mm).", "Lagerhölzer mit 75 mm gezeichnet, der kleinsten Einzelheit des Maßstabs (wahr 60 mm)."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": T("Overlays colour the block with the area-weighted value of strip, stack and lashing (labelled merged).", "Überlagerungen färben den Block mit dem flächengewichteten Wert von Streifen, Stapel und Anschlagmitteln (als zusammengefasst gekennzeichnet)."),
    }


def representation(scale, el, fr, pl):
    """🗺 An authored representation."""
    pens = {50: {"body": 0.35, "detail": 0.18, "texture": 0.1, "glazing": 0.13, "ground": 0.5}, 100: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": 0.1, "ground": 0.35}, 200: {"body": 0.18, "detail": 0.1, "texture": 0.1, "glazing": 0.1, "ground": 0.25}, 500: {"body": 0.13, "detail": None, "texture": None, "glazing": None, "ground": 0.18}}[scale]
    rng_ = {50: (20, 75), 100: (75, 115), 200: (115, 350), 500: (350, 1000)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": rng_[0], "maxDenominator": rng_[1]}, "pens": pens, "hatches": [], "geometry": {"elevation": el, "front": fr, "plan": pl}, "decisions": decisions(scale)}


def shift_lateral(shapes, view):
    """↔️ Moves front (paper x) or plan (paper y) shapes from stack-centred lateral positions to asset positions, the stack centre lying at ZOFF so the asset is laterally centred; a `solid` shape keeps its bone-local profile and moves only its span `z0`..`z1`."""
    for sh in shapes:
        if "solid" in sh:
            sh["solid"] = {**sh["solid"], "z0": r6(sh["solid"]["z0"] + ZOFF), "z1": r6(sh["solid"]["z1"] + ZOFF)}
            continue
        if view == "front":
            if sh["kind"] == "rect":
                sh["x"] = r6(sh["x"] + ZOFF)
            elif sh["kind"] == "capsule":
                sh["x0"] = r6(sh["x0"] + ZOFF)
                sh["x1"] = r6(sh["x1"] + ZOFF)
            elif sh["kind"] == "poly":
                sh["points"] = [[r6(a + ZOFF), b] for a, b in sh["points"]]
        else:
            if sh["kind"] == "rect":
                sh["y"] = r6(sh["y"] + ZOFF)
            elif sh["kind"] == "capsule":
                sh["y0"] = r6(sh["y0"] + ZOFF)
                sh["y1"] = r6(sh["y1"] + ZOFF)
            elif sh["kind"] == "poly":
                sh["points"] = [[a, r6(b + ZOFF)] for a, b in sh["points"]]
    return shapes


def ids_of(lst):
    """🧾 The ids of a solid list."""
    return [s["id"] for s in lst]


def build_scale(scale):
    """🧊 One scale: body, thermal, structure, decisions and the three drawings."""
    br_body, br_el, br_fr, br_pl = bearers_sleepers(scale)
    if scale in (50, 100):
        st_body, st_el, st_fr, st_pl = stack_scale(scale)
        la_body = lashing_parts(scale)
        la_el, la_fr, la_pl = lashing_shapes(scale)
    else:
        st_body, st_el, st_fr, st_pl = coarse_stack(scale)
        la_body = lashing_parts(scale)
        la_el, la_fr, la_pl = [], [], []
    body = br_body + st_body + la_body
    el = br_el + st_el + la_el
    fr = shift_lateral(br_fr + st_fr + la_fr, "front")
    pl = shift_lateral(br_pl + st_pl + la_pl, "plan")
    thermal_ = [ref(f"th-{s['id']}", s["id"], region=R_BR if s["bone"] == "bearers" else R_SL if s["bone"] == "sleepers" else R_STRIP if s["bone"] == "strip" else R_LASH if s["material"] in (MAT_PET, MAT_SLING) else R_STACK) for s in body]
    if scale == 500:
        a = 2 * RS
        hs = STRIP_V + BT + RS
        for i, x in enumerate(XSG, 1):
            thermal_ += [
                box(f"th-sling-{i}-bottom", "stack", MAT_SLING, TONE_SG, [x, -RS, 0], [a, a, 2 * ZSG + a], slender=True, region=R_LASH),
                box(f"th-sling-{i}-side-r", "stack", MAT_SLING, TONE_SG, [x, hs / 2, -ZSG], [a, hs, a], slender=True, region=R_LASH),
                box(f"th-sling-{i}-side-l", "stack", MAT_SLING, TONE_SG, [x, hs / 2, ZSG], [a, hs, a], slender=True, region=R_LASH),
                box(f"th-sling-{i}-top", "pack-top", MAT_SLING, TONE_SG, [x, RS, 0], [a, a, 2 * ZSG + a], slender=True, region=R_LASH),
            ]
    struct = []
    for s in body:
        b = s["bone"]
        if b in ("bearers", "sleepers"):
            struct.append(ref(f"m-{s['id']}", s["id"], member=b, part="member"))
    if scale == 50:
        for k in range(NL):
            ds = LAYERS[k]
            struct.append(box(f"m-layer-{k + 1:02d}", "stack", MAT_BOARD, 0.55, [2.0, lv(k) + BT / 2, 0], [4.0 + 2 * max(ds), BT, LW], member="stack", part="member"))
        struct += [ref(f"m-sticker-row-{j}", f"sticker-row-{j}", member="stack", part="member") for j in range(1, 7)]
        struct.append(box("m-strip", "strip", MAT_BOARD, 0.5, [2.0, BT / 2, 0], [4.0 + 2 * max(STRIP), BT, LW], member="strip", part="member"))
    elif scale == 100:
        struct += [ref(f"m-layer-{k + 1:02d}", f"layer-{k + 1:02d}", member="stack", part="member") for k in range(NL)]
        struct += [ref(f"m-sticker-row-{j}", f"sticker-row-{j}", member="stack", part="member") for j in range(1, 7)]
        struct.append(ref("m-strip", "strip", member="strip", part="member"))
    elif scale == 200:
        struct.append(box("m-layers", "stack", MAT_BOARD, 0.55, [2.0, BT / 2, 0], [XMAX - XMIN, BT, LW], sheet=True, member="stack", part="member", repeat={"count": NL, "translation": [0, P, 0]}))
        struct.append(box("m-sticker-columns", "stack", MAT_SW, 0.7, [XS[0], STRIP_V / 2, 0], [STK_B, STRIP_V, STK_L], member="stack", part="member", repeat={"count": 6, "translation": [0.75, 0, 0]}))
        struct.append(ref("m-strip", "strip", member="strip", part="member"))
    else:
        core = NL * LW * BT / STRIP_V
        struct.append(box("m-layers-core", "stack", MAT_BOARD, 0.55, [2.0, STRIP_V / 2, 0], [XMAX - XMIN, STRIP_V, core], member="stack", part="member"))
        struct.append(ref("m-strip", "strip", member="strip", part="member"))
    for s in la_body:
        if s["bone"].startswith("leg-"):
            struct.append(ref(f"m-{s['id']}", s["id"], member=s["bone"], part="member"))
        elif s["material"] == MAT_SLING:
            struct.append(ref(f"m-{s['id']}", s["id"], member=s["bone"], part="member"))
    if scale == 500:
        for leg, length in (("leg-a1", L1), ("leg-a2", L2), ("leg-b1", L1), ("leg-b2", L2)):
            struct.append(box(f"m-{leg}", leg, MAT_SLING, TONE_SG, [length / 2, 0, 0], [length, 2 * RS, 2 * RS], slender=True, member=leg, part="member"))
    struct += masses_and_joints(scale)
    vol_dec = volume_decisions(scale)
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal_, "structure": struct, "decisions": vol_dec}, representation(scale, el, fr, pl)


def volume_decisions(scale):
    """📝 What each volume keeps, merges and leaves out."""
    if scale == 50:
        return [T("Body: every one of the 120 boards is a tongue-and-groove extrusion (28 x 137 mm, tongue 9 x 8 mm, groove 10 x 10 mm) at its own reclaimed length (up to 24 mm proud at either end), six per layer engaged tongue in groove; 114 stickers 24 x 48 x 850 mm in six rows over the six bearers (100 x 100 x 1000 mm with 5 mm worn arrises); two PET straps (19 x 0.8 mm sheets) and two round slings (32 mm) around the pack, their top runs on the pack-top bone so they can be re-tensioned 52 mm lower, and the four leg runs as capsules; seven sleepers 60 x 80 x 1000 mm at 650 mm beside the pack.", "Body: jede der 120 Dielen ist ein Feder-Nut-Extrusionskörper (28 x 137 mm, Feder 9 x 8 mm, Nut 10 x 10 mm) in ihrer eigenen gebrauchten Länge (bis 24 mm an jedem Ende vorstehend), sechs je Lage Feder in Nut; 114 Stapellatten 24 x 48 x 850 mm in sechs Reihen über den sechs Unterlegern (100 x 100 x 1000 mm mit 5 mm abgenutzten Kanten); zwei PET-Bänder (Flächen 19 x 0,8 mm) und zwei Rundschlingen (32 mm) um das Paket, ihre oberen Läufe am Knochen pack-top, damit sie 52 mm tiefer nachgespannt werden können, und die vier Strangläufe als Kapseln; sieben Lagerhölzer 60 x 80 x 1000 mm im Abstand von 650 mm neben dem Paket."),
                T("Thermal: references to the body solids, by region; the stack region counts its exchange area. Structure: nineteen layer plates of 830 x 28 mm, the sticker rows, the strip plate, bearers, sleepers, slings and legs as members, joint spheres at the leg pivots, mass markers at the centres of mass of bearers, sleepers, stack and strip.", "Thermik: Verweise auf die Body-Körper nach Region; die Stapelregion zählt ihre Austauschfläche. Tragwerk: neunzehn Lagenplatten 830 x 28 mm, die Lattenreihen, die Streifenplatte, Unterleger, Lagerhölzer, Schlingen und Stränge als Stäbe, Gelenkkugeln an den Strangdrehpunkten, Massenmarken in den Schwerpunkten von Unterlegern, Lagerhölzern, Stapel und Streifen.")]
    if scale == 100:
        return [T("Body: the six boards of a layer are one 830 x 28 mm slab at the layer's longest board (tongue and groove of 8 to 10 mm are below the 15 mm feature); sticker rows, straps, slings, legs, sleepers as at 1:50; bearers without the 5 mm rounding.", "Body: die sechs Dielen einer Lage sind eine Platte 830 x 28 mm in der Länge der längsten Diele der Lage (Feder und Nut mit 8 bis 10 mm liegen unter der Einzelheit von 15 mm); Lattenreihen, Bänder, Schlingen, Stränge und Lagerhölzer wie bei 1:50; Unterleger ohne die 5-mm-Rundung."),
                T("Thermal and structure reference the slabs; masses and joints sized for the scale.", "Thermik und Tragwerk verweisen auf die Platten; Massen und Gelenke für den Maßstab bemessen.")]
    if scale == 200:
        return [T("Body: layers 1 to 19 with their stickers are one block 4.05 x 0.99 x 0.83 m, declared the envelope of the typified layer and sticker rows (gaps of 24 mm are under the 30 mm feature); the top layer stays a 28 mm board layer of its own (declared sheet: it is the strip that is reinstalled); slings and legs as 32 mm boxes; the PET straps (19 mm) are left out of the body and kept in no list at this scale.", "Body: die Lagen 1 bis 19 mit ihren Latten sind ein Block 4,05 x 0,99 x 0,83 m, deklariert als Hülle der typisierten Lagen- und Lattenreihen (Fugen von 24 mm liegen unter der Einzelheit von 30 mm); die oberste Lage bleibt eine eigene 28-mm-Dielenlage (als Fläche deklariert: sie ist der Streifen, der wiederverlegt wird); Schlingen und Stränge als 32-mm-Quader; die PET-Bänder (19 mm) entfallen in diesem Maßstab."),
                T("Structure: the nineteen layers as declared sheets at their true 28 mm with the six sticker rows merged into sticker columns that carry them down to the bearers.", "Tragwerk: die neunzehn Lagen als deklarierte Flächen in ihren wahren 28 mm, die sechs Lattenreihen zu Lattensäulen zusammengefasst, die sie auf die Unterleger abtragen.")]
    return [T("Body: one block for layers 1 to 19 (the envelope of the typified layer rows), the top layer as a declared 28 mm sheet on its own bone, six bearers and seven sleepers widened to the 75 mm feature; slings, legs and straps (32 mm and less) are left out of the body.", "Body: ein Block für die Lagen 1 bis 19 (die Hülle der typisierten Lagenreihen), die oberste Lage als deklarierte 28-mm-Fläche an ihrem eigenen Knochen, sechs Unterleger und sieben Lagerhölzer auf die Einzelheit von 75 mm verbreitert; Schlingen, Stränge und Bänder (32 mm und weniger) entfallen im Body."),
            T("Thermal keeps the sling loops as slender solids for the lashing region; structure carries the stack as an area-equivalent core (19 x 830 x 28 mm = 0.44 m2 over the full stack height), the strip sheet, bearers, sleepers and the four leg runs as slender members.", "Die Thermik behält die Schlingenschlaufen als schlanke Körper für die Region der Anschlagmittel; das Tragwerk führt den Stapel als flächengleichen Kern (19 x 830 x 28 mm = 0,44 m2 über die ganze Stapelhöhe), die Streifenfläche, Unterleger, Lagerhölzer und die vier Strangläufe als schlanke Stäbe.")]


def main():
    """🏁 Writes the asset document."""
    vols, reps = {}, {}
    for scale in (50, 100, 200, 500):
        v, r = build_scale(scale)
        vols[str(scale)] = v
        reps[str(scale)] = r
    asset = {
        "id": "reclaimed-floorboard-stack",
        "category": "components",
        "title": T("Reclaimed Floorboard Stack", "Gebrauchter Dielenstapel"),
        "summary": T(f"A stack of 120 reclaimed pine tongue-and-groove floorboards 28 x 145 mm, about 4 m long, in twenty layers of six with 24 mm stickers on six squared-timber bearers, held by two PET straps and slung with two round slings; old nail holes along the board edges and old floor paint on the faces show the first floor. It is lifted 300 mm by a crane on the slings, and its top layer of six boards is reinstalled on seven sleepers under a floor load of 2 kN/m² and a walking person. {M_TOTAL:.0f} kg.", f"Ein Stapel aus 120 gebrauchten Kiefernholz-Nut-und-Feder-Dielen 28 x 145 mm, etwa 4 m lang, in zwanzig Lagen zu sechs mit 24-mm-Stapellatten auf sechs Kantholz-Unterlegern, gehalten von zwei PET-Bändern und angeschlagen mit zwei Rundschlingen; alte Nagellöcher an den Dielenkanten und alte Fußbodenfarbe auf den Oberseiten zeigen den ersten Boden. Der Kran hebt ihn an den Schlingen 300 mm an, und seine oberste Lage aus sechs Dielen wird auf sieben Lagerhölzern unter 2 kN/m² Nutzlast und einer gehenden Person wiederverlegt. {M_TOTAL:.0f} kg."),
        "tags": ["component", "reclaimed", "timber", "pine", "floorboard", "tongue-and-groove", "flooring", "stack", "stickered"],
        "provenance": provenance(),
        "dimensions": {"heightM": round(HEIGHT, 4), "widthM": round(XMAX - XMIN, 3), "depthM": round(Z_SL + SL_L / 2 + BR_L / 2, 3), "massKg": round(M_TOTAL, 2)},
        "materials": materials(),
        "rig": rig(),
        "anchors": anchors(),
        "representations": reps,
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": thermal(),
        "structure": structure(),
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    doc = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    bone_mass = sum(b.get("mass", {}).get("kg", 0) for b in asset["rig"]["bones"])
    print(f"[DEBUG] mass {M_TOTAL:.2f} kg (bones {bone_mass:.2f}), height {HEIGHT:.4f}, width {XMAX - XMIN:.3f}, areas strip {A_STRIP:.3f} stack {A_STACK:.1f} bearers {A_BR:.3f} sleepers {A_SLEEP:.3f} lashing {A_LASH:.3f}")
    for s in (50, 100, 200, 500):
        v = vols[str(s)]
        n = lambda l: sum(x.get("repeat", {}).get("count", 1) for x in l)
        print(f"[DEBUG] 1:{s} body {len(v['body'])}/{n(v['body'])} thermal {len(v['thermal'])} structure {len(v['structure'])} shapes e/f/p {len(reps[str(s)]['geometry']['elevation'])}/{len(reps[str(s)]['geometry']['front'])}/{len(reps[str(s)]['geometry']['plan'])}")


if __name__ == "__main__":
    main()
