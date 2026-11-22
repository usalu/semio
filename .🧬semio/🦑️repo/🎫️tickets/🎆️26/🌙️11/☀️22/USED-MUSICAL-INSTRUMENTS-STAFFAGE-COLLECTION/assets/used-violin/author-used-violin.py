"""🎻️ The authoritative chisel of the asset `used-violin`: states every hand-chosen number of the used 4/4 violin, its bow, its oblong case and the shoulder strap, and writes `🔣️.json` next to this file.

Run: `python3 author-used-violin.py [areas.json]` — the optional file maps thermal region ids to the 1:50 outer areas measured by the validator.

@see ./author_lib.py — frames and parts
@see ../../brief.md — the binding brief of the wave
"""
import json
import math
import pathlib
import sys

from author_lib import (Frame, Part, add, band, columns, compose, cross, decompose, dot, mat_mul, mat_vec, offset_polyline, placement,
                        polygon_circle, r6, rect, scale, sub, transpose, unit)

HERE = pathlib.Path(__file__).parent
AREAS = json.loads(pathlib.Path(sys.argv[1]).read_text()) if len(sys.argv) > 1 else {}

T = lambda en, de: {"en": en, "de": de}

CASE_L, CASE_W, SHELL_H, LID_H, WALL = 0.80, 0.245, 0.080, 0.045, 0.006
HALF_W = CASE_W / 2
LID_LEAN = -8.0
BED_TOP = 0.014
BODY_L, VIOLIN_L = 0.356, 0.593
RIB_H, TOP_ARCH, BACK_ARCH = 0.031, 0.0155, 0.0145
VIOLIN_X0 = -0.293
VIOLIN_Y0 = BED_TOP + RIB_H + BACK_ARCH
BOW_L = 0.75
BOW_AT = (0.025, 0.198, 0.023)
STRAP_RING = (0.03, -0.012, -(HALF_W + 0.008))
STRAP_SPAN, STRAP_H, STRAP_HALF, STRAP_T = 0.74, 0.528, 0.019, 0.003
PAD = (0.14, 0.06, 0.010)
SHOULDER_Y, CHIN_Y = 1.45, 1.40
STRING_T = 235.0
BOW_FORCE = 0.8
CHIN_FORCE = 15.0

OUTLINE = [(0.0, 0.0), (0.002, 0.028), (0.006, 0.046), (0.013, 0.064), (0.025, 0.081), (0.042, 0.094), (0.062, 0.1015), (0.080, 0.104), (0.098, 0.103),
           (0.115, 0.0985), (0.129, 0.0925), (0.135, 0.089), (0.139, 0.080), (0.146, 0.068), (0.160, 0.059), (0.180, 0.056), (0.200, 0.0575), (0.213, 0.064),
           (0.220, 0.075), (0.224, 0.081), (0.230, 0.0815), (0.250, 0.082), (0.275, 0.084), (0.295, 0.0835), (0.315, 0.079), (0.332, 0.069), (0.344, 0.054),
           (0.351, 0.038), (0.3545, 0.022), (0.356, 0.0)]
U_C = 0.178


def outline(s=1.0, step=1):
    pts = OUTLINE[::step] if step > 1 else OUTLINE
    if pts[-1] != OUTLINE[-1]:
        pts = pts + [OUTLINE[-1]]
    side = [(U_C + s * (u - U_C), s * w) for u, w in pts]
    other = [(u, -w) for u, w in side[-2:0:-1]]
    return side + other


def hw(u):
    for (u0, w0), (u1, w1) in zip(OUTLINE, OUTLINE[1:]):
        if u0 <= u <= u1:
            return w0 + (w1 - w0) * (u - u0) / (u1 - u0)
    return 0.0


FB_TOP = lambda u: 0.0165 + 0.1333 * (0.356 - u)
FB_BOT = lambda u: 0.0065 + 0.1371 * (0.356 - u) if u <= 0.356 else 0.0065 - 0.106 * (u - 0.356)
NUT = (0.486, FB_TOP(0.486) + 0.001)
BRIDGE_U, BRIDGE_TOP = 0.1615, 0.0477
TAIL_HOLE = (0.118, 0.036)
STRINGS = [("g", 0.0083, 0.0170, 0.0004), ("d", 0.0028, 0.0057, 0.0004), ("a", -0.0028, -0.0057, 0.0003), ("e", -0.0083, -0.0170, 0.00013)]
ARC_R = 0.042
ARC_DROP = lambda w: ARC_R - math.sqrt(ARC_R * ARC_R - w * w)
BRIDGE_V = {name: BRIDGE_TOP - ARC_DROP(w_b) + (0.0008 if name == "g" else 0.0) for name, _, w_b, _ in STRINGS}
NUT_V = {name: NUT[1] - ARC_DROP(w_n) * 0.5 for name, w_n, _, _ in STRINGS}
STICK = [(0.0, 0.0238, 0.0043), (0.06, 0.0238, 0.0043), (0.15, 0.0205, 0.0043), (0.25, 0.0155, 0.0042), (0.35, 0.0115, 0.0039), (0.45, 0.0100, 0.0036),
         (0.55, 0.0108, 0.0032), (0.65, 0.0140, 0.0029), (0.725, 0.0180, 0.00275)]


def stick_v(u):
    for (u0, v0, _), (u1, v1, _) in zip(STICK, STICK[1:]):
        if u0 <= u <= u1:
            return v0 + (v1 - v0) * (u - u0) / (u1 - u0)
    return STICK[-1][1]


IDENTITY = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
REST = {
    "floor": (None, 0, 0, 0, placement()),
    "case": ("floor", -CASE_L / 2, SHELL_H, 0, placement()),
    "lid": ("case", 0, 0, HALF_W, placement(twist=LID_LEAN)),
    "bow-slide": ("lid", *BOW_AT, placement()),
    "bow-reach": ("bow-slide", 0, 0, 0, placement()),
    "bow-hold": ("bow-reach", 0, 0, 0, placement()),
    "bow": ("bow-hold", 0, 0, 0, placement()),
    "strap": ("case", *STRAP_RING, None),
    "violin-lift": ("case", VIOLIN_X0 + CASE_L / 2, VIOLIN_Y0 - SHELL_H, 0, placement()),
    "violin": ("violin-lift", 0, 0, 0, placement()),
}
PRISMATIC = {"bow-slide": 0, "bow-reach": 90, "bow": 0, "violin-lift": 90}
ORDER = list(REST)


def strap_rest_twist():
    pad_far = STRAP_H + PAD[1] / 2
    y_ring = SHELL_H + STRAP_RING[1]
    lo, hi = 90.0, 110.0
    for _ in range(80):
        mid = (lo + hi) / 2
        c, s = math.cos(math.radians(mid)), math.sin(math.radians(mid))
        y = y_ring + pad_far * c - (PAD[2] / 2) * s
        lo, hi = (mid, hi) if y > 0 else (lo, mid)
    return round((lo + hi) / 2, 3)


STRAP_TWIST = strap_rest_twist()
REST["strap"] = ("case", *STRAP_RING, placement(twist=STRAP_TWIST))


def frames(offsets=None, root=(0.0, 0.0)):
    offsets = offsets or {}
    out = {}
    for bone in ORDER:
        parent, x, y, z, rest = REST[bone]
        off = offsets.get(bone, {})
        r_off = placement(off.get("angleDeg", 0), off.get("abductionDeg", 0), off.get("twistDeg", 0))
        d = (0.0, 0.0)
        if bone in PRISMATIC:
            a = math.radians(PRISMATIC[bone])
            s = off.get("dx", 0) * math.cos(a) + off.get("dy", 0) * math.sin(a)
            d = (s * math.cos(a), s * math.sin(a))
            r_off = IDENTITY
        if parent is None:
            out[bone] = Frame([root[0] + x, root[1] + y, z], mat_mul(rest, r_off))
        else:
            out[bone] = compose(out[parent], x, y, z, rest, r_off, d)
    return out


F0 = frames()

MASS = {
    "case": (2.05, "authored-estimate"),
    "lid": (0.72, "authored-estimate"),
    "strap": (0.13, "authored-estimate"),
    "violin": (0.466, "literature"),
    "bow": (0.061, "literature"),
}
COM = {
    "case": (0.40, -0.046),
    "lid": (0.40, 0.1225),
    "strap": (0.37, 0.33),
    "violin": (0.234, -0.008),
    "bow": (0.245, 0.012),
}


def com_world(fr):
    total = 0.0
    acc = [0.0, 0.0, 0.0]
    for bone, (kg, _) in MASS.items():
        p = fr[bone].world([COM[bone][0], COM[bone][1], 0])
        acc = add(acc, scale(p, kg))
        total += kg
    return scale(acc, 1 / total), total


LID_CLOSED = 90.0 - LID_LEAN


def carried_offsets():
    base = {"lid": {"twistDeg": LID_CLOSED}}
    fr = frames(base)
    loc = [0.0, 0.0, 0.0]
    kg_sum = 0.0
    for bone, (kg, _) in MASS.items():
        if bone == "strap":
            continue
        p = fr["case"].local(fr[bone].world([COM[bone][0], COM[bone][1], 0]))
        loc = add(loc, scale(p, kg))
        kg_sum += kg
    loc = scale(loc, 1 / kg_sum)
    vr, wr = STRAP_RING[1], STRAP_RING[2]
    rho = math.degrees(math.atan2(loc[2] - wr, loc[1] - vr))
    if math.cos(math.radians(rho)) * (vr - loc[1]) + math.sin(math.radians(rho)) * (wr - loc[2]) < 0:
        rho = rho + 180 if rho < 0 else rho - 180
    rho = round(rho, 2)
    sigma = round(-(rho + STRAP_TWIST), 2)
    offs = {"case": {"twistDeg": rho}, "lid": {"twistDeg": LID_CLOSED}, "strap": {"twistDeg": sigma}}
    fr = frames(offs)
    com, _ = com_world(fr)
    apex_u = COM_APEX = 0.37
    apex = fr["strap"].world([apex_u, STRAP_H, 0])
    lift = round(SHOULDER_Y - apex[1], 4)
    return offs, lift, loc, com


CARRY, LIFT, CASE_COM, _ = carried_offsets()


def played_offsets():
    pitch, roll = 12.0, 35.0
    u = [-math.cos(math.radians(pitch)), math.sin(math.radians(pitch)), 0.0]
    v0 = [math.sin(math.radians(pitch)), math.cos(math.radians(pitch)), 0.0]
    w0 = cross(u, v0)
    c, s = math.cos(math.radians(roll)), math.sin(math.radians(roll))
    v = add(scale(v0, c), scale(w0, -s))
    w = add(scale(w0, c), scale(v0, s))
    rv = columns(u, v, w)
    a, b, cc = decompose(rv)
    lift = CHIN_Y - VIOLIN_Y0
    offs = {"violin-lift": {"dy": round(lift, 4)}, "violin": {"angleDeg": round(a, 3), "abductionDeg": round(b, 3), "twistDeg": round(cc, 3)}}
    fr = frames(offs)
    fv = fr["violin"]
    contact_u = BRIDGE_U + 0.024
    f = (NUT[0] - contact_u) / (NUT[0] - BRIDGE_U)
    a_w = STRINGS[2][1] + (STRINGS[2][2] - STRINGS[2][1]) * f
    a_v = NUT_V["a"] + (BRIDGE_V["a"] - NUT_V["a"]) * f + STRINGS[2][3]
    contact = fv.world([contact_u, a_v, a_w])
    tilt = math.asin(-a_w / ARC_R)
    uv, vv, wv = fv.axis(0), fv.axis(1), fv.axis(2)
    d = add(scale(wv, math.cos(tilt)), scale(vv, math.sin(tilt)))
    n = add(scale(vv, math.cos(tilt)), scale(wv, -math.sin(tilt)))
    d, n = unit(d), unit(n)
    bw = cross(d, n)
    rb = columns(d, n, bw)
    t_mid = 0.40
    o_b = sub(contact, scale(d, t_mid))
    fl = F0["lid"]
    o_s = fl.world(list(BOW_AT))
    w_l, u_l, v_l = fl.axis(2), fl.axis(0), fl.axis(1)
    dx_mid = dot(sub(o_b, o_s), w_l) / dot(d, w_l)
    p = sub(o_b, scale(d, dx_mid))
    slide = dot(sub(p, o_s), u_l)
    reach = dot(sub(p, o_s), v_l)
    hold = mat_mul(transpose(fl.r), rb)
    ha, hb, hc = decompose(hold)
    offs.update({"bow-slide": {"dx": round(slide, 4)}, "bow-reach": {"dy": round(reach, 4)}, "bow-hold": {"angleDeg": round(ha, 3), "abductionDeg": round(hb, 3), "twistDeg": round(hc, 3)}, "bow": {"dx": round(dx_mid, 4)}})
    strokes = {"frog": round(dx_mid + t_mid - 0.085, 4), "tip": round(dx_mid + t_mid - 0.655, 4)}
    return offs, contact, n, strokes, contact_u, a_v, a_w


PLAYED, CONTACT_W, BOW_NORMAL, STROKE, CONTACT_U, CONTACT_V, CONTACT_WL = played_offsets()


def violin_parts(scale_id):
    parts = []
    reg = {"top": "violin-top", "back": "violin-back", "neck": "violin-neck", "worn": "worn-varnish", "fit": "fingerboard-fittings", "chin": "chinrest", "bridge": "bridge", "str": "strings"}
    if scale_id == 50:
        parts.append(Part("violin-ribs", "violin", "maple-flamed", "pv", {"profile": outline(), "v0": -RIB_H, "v1": 0.0}, reg["back"], tone=0.42))
        parts.append(Part("violin-top-lower", "violin", "spruce-top-aged", "pv", {"profile": outline(0.90), "v0": 0.0, "v1": 0.0075}, reg["top"], tone=0.62))
        parts.append(Part("violin-top-upper", "violin", "spruce-top-aged", "pv", {"profile": outline(0.62), "v0": 0.0075, "v1": TOP_ARCH}, reg["top"], tone=0.66))
        parts.append(Part("violin-back-lower", "violin", "maple-flamed", "pv", {"profile": outline(0.90), "v0": -RIB_H - 0.0075, "v1": -RIB_H}, reg["back"], tone=0.38))
        parts.append(Part("violin-back-upper", "violin", "maple-flamed", "pv", {"profile": outline(0.62), "v0": -RIB_H - BACK_ARCH, "v1": -RIB_H - 0.0075}, reg["back"], tone=0.36))
        neck = [(0.350, -0.0345), (0.358, -0.0345), (0.366, -0.028), (0.372, -0.018), (0.380, -0.0105), (0.400, -0.0095), (0.486, -0.0198), (0.486, FB_BOT(0.486)), (0.356, 0.0065), (0.350, 0.0065)]
        parts.append(Part("violin-neck", "violin", "maple-bare-worn", "pw", {"profile": neck, "w0": -0.0135, "w1": 0.0135}, reg["neck"], tone=0.55))
        pegbox = [(0.486, -0.0198), (0.552, -0.030), (0.552, -0.010), (0.486, FB_BOT(0.486) + 0.002)]
        parts.append(Part("violin-pegbox", "violin", "maple-flamed", "pw", {"profile": pegbox, "w0": -0.0115, "w1": 0.0115}, reg["neck"], tone=0.4))
        scroll = polygon_circle(0.5725, -0.021, 0.0205, 14)
        parts.append(Part("violin-scroll", "violin", "maple-flamed", "pw", {"profile": scroll, "w0": -0.0205, "w1": 0.0205}, reg["neck"], tone=0.4))
        fb_u = [0.216, 0.356, 0.486]
        fb_w = [0.021, 0.021 - 0.009 * 0.14 / 0.27, 0.012]
        secs = [{"at": [u, (FB_TOP(u) + FB_BOT(u)) / 2, 0.0], "rx": (FB_TOP(u) - FB_BOT(u)) / 2, "ry": w, "exponent": 8} for u, w in zip(fb_u, fb_w)]
        fb_profile = [(0.216, FB_BOT(0.216)), (0.216, FB_TOP(0.216)), (0.486, FB_TOP(0.486)), (0.486, FB_BOT(0.486)), (0.356, FB_BOT(0.356))]
        k = (fb_w[2] - fb_w[0]) / (0.486 - 0.216)
        w_at0, w_tip = fb_w[0] - k * 0.216, fb_w[0] + k * (VIOLIN_L - 0.216)
        parts.append(Part("fingerboard", "violin", "ebony", "loft", {"sections": secs}, reg["fit"], tone=0.12,
                          two_d=[("pw", {"profile": fb_profile, "w0": -w_at0, "w1": w_at0, "wtip": (-w_tip, w_tip)}, "fingerboard")]))
        for i, (pu, side) in enumerate([(0.500, 1), (0.513, -1), (0.527, 1), (0.540, -1)]):
            w_in, w_out = (0.0115, 0.042) if side > 0 else (-0.042, -0.0115)
            parts.append(Part(f"peg-{i + 1}", "violin", "ebony", "pv", {"profile": rect(pu - 0.00375, pu + 0.00375, w_in, w_out), "v0": -0.025, "v1": -0.007}, reg["fit"], tone=0.12))
        parts.append(Part("bridge", "violin", "maple-bridge", "pw", {"profile": [(BRIDGE_U - 0.0022, TOP_ARCH), (BRIDGE_U + 0.0022, TOP_ARCH), (BRIDGE_U + 0.0008, BRIDGE_TOP), (BRIDGE_U - 0.0008, BRIDGE_TOP)], "w0": -0.0207, "w1": 0.0207}, reg["bridge"], tone=0.78))
        tail = [(0.018, -0.009), (0.112, -0.020), (0.124, -0.019), (0.130, -0.012), (0.130, 0.012), (0.124, 0.019), (0.112, 0.020), (0.018, 0.009)]
        parts.append(Part("tailpiece", "violin", "ebony", "pv", {"profile": tail, "v0": 0.020, "v1": 0.030}, reg["fit"], tone=0.14))
        parts.append(Part("fine-tuners", "violin", "string-steel", "pv", {"profile": rect(0.098, 0.120, -0.019, 0.019), "v0": 0.030, "v1": 0.037}, reg["str"], tone=0.7))
        parts.append(Part("tailgut", "violin", "ebony", "pw", {"profile": [(0.0, 0.0), (0.006, 0.0), (0.024, 0.020), (0.018, 0.026), (0.0, 0.006)], "w0": -0.006, "w1": 0.006}, reg["fit"], tone=0.14))
        parts.append(Part("endpin", "violin", "ebony", "pu", {"profile": polygon_circle(-0.0155, 0.0, 0.006, 10), "u0": -0.009, "u1": 0.001}, reg["fit"], tone=0.12))
        cup = [(-0.004, -0.010), (0.010, -0.012), (0.040, -0.010), (0.058, 0.004), (0.060, 0.030), (0.054, 0.058), (0.036, 0.072), (0.012, 0.074), (-0.004, 0.064), (-0.010, 0.040), (-0.008, 0.010)]
        parts.append(Part("chinrest-cup", "violin", "ebony", "pv", {"profile": cup, "v0": 0.0075, "v1": 0.030}, reg["chin"], tone=0.15))
        parts.append(Part("chinrest-bracket", "violin", "string-steel", "pv", {"profile": rect(-0.010, 0.004, 0.020, 0.050), "v0": -0.025, "v1": 0.0075}, reg["chin"], tone=0.7))
        for name, w_n, w_b, r in STRINGS:
            p0 = [BRIDGE_U, BRIDGE_V[name] + r, w_b]
            p1 = [NUT[0], NUT_V[name] + r, w_n]
            p2 = [TAIL_HOLE[0], TAIL_HOLE[1] + r, w_b * 0.85]
            k1 = (w_n - w_b) / (NUT[0] - BRIDGE_U)
            z0 = w_b - k1 * BRIDGE_U
            ztip = w_b + k1 * (VIOLIN_L - BRIDGE_U)
            parts.append(Part(f"string-{name}", "violin", "string-steel", "capsule", {"p0": p0, "p1": p1, "r": r}, reg["str"], role="detail", tone=0.8, flags={"slender": True},
                              two_d=[("pw", {"profile": band((p0[0], p0[1]), (p1[0], p1[1]), r), "w0": z0 - r, "w1": z0 + r, "wtip": (ztip - r, ztip + r)}, f"string-{name}")]))
            ka = (p2[2] - p0[2]) / (p2[0] - p0[0])
            za = p0[2] - ka * p0[0]
            ztipa = p0[2] + ka * (VIOLIN_L - p0[0])
            parts.append(Part(f"afterlength-{name}", "violin", "string-steel", "capsule", {"p0": p2, "p1": p0, "r": r}, reg["str"], role="detail", tone=0.8, flags={"slender": True},
                              two_d=[("pw", {"profile": band((p2[0], p2[1]), (p0[0], p0[1]), r), "w0": za - r, "w1": za + r, "wtip": (ztipa - r, ztipa + r)}, f"afterlength-{name}")]))
        parts.append(Part("worn-shoulder-back", "violin", "maple-bare-worn", "pv", {"profile": rect(0.040, 0.085, 0.055, 0.080), "v0": -RIB_H - 0.0085, "v1": -RIB_H - 0.0075}, reg["worn"], role="texture", tone=0.7, flags={"texture": True}, filled=False))
        parts.append(Part("worn-bout-top", "violin", "maple-bare-worn", "pv", {"profile": rect(0.300, 0.335, -0.066, -0.050), "v0": 0.0075, "v1": 0.0085}, reg["worn"], role="texture", tone=0.75, flags={"texture": True}, filled=False))
        parts.append(Part("rosin-dust", "violin", "rosin", "pv", {"profile": rect(0.166, 0.214, -0.028, 0.028), "v0": TOP_ARCH, "v1": TOP_ARCH + 0.0008}, None, role="texture", tone=0.9, flags={"texture": True}, filled=False))
        parts.append(Part("fingerboard-grooves", "violin", "ebony", "pw", {"profile": [(0.430, FB_TOP(0.430) - 0.0004), (0.470, FB_TOP(0.470) - 0.0004), (0.470, FB_TOP(0.470) + 0.0004), (0.430, FB_TOP(0.430) + 0.0004)], "w0": -0.008, "w1": -0.002}, None, role="texture", tone=0.3, flags={"texture": True}, filled=False))
    elif scale_id == 100:
        parts.append(Part("violin-body", "violin", "maple-flamed", "pv", {"profile": outline(1.0, 2), "v0": -RIB_H - 0.0075, "v1": 0.0}, reg["back"], tone=0.42))
        parts.append(Part("violin-arch", "violin", "spruce-top-aged", "pv", {"profile": outline(0.80, 2), "v0": 0.0, "v1": TOP_ARCH}, reg["top"], tone=0.64))
        parts.append(Part("violin-back-arch", "violin", "maple-flamed", "pv", {"profile": outline(0.62, 2), "v0": -RIB_H - BACK_ARCH, "v1": -RIB_H - 0.0075}, reg["back"], tone=0.36))
        head = [(0.350, -0.0345), (0.362, -0.0345), (0.372, -0.018), (0.400, -0.0105), (0.486, -0.0198), (0.552, -0.031), (0.560, -0.0415), (0.585, -0.0415), (0.593, -0.021), (0.585, -0.003), (0.552, -0.006), (0.486, FB_BOT(0.486)), (0.356, 0.0065), (0.350, 0.0065)]
        parts.append(Part("violin-head", "violin", "maple-bare-worn", "pw", {"profile": head, "w0": -0.0155, "w1": 0.0155}, reg["neck"], tone=0.5))
        fb_profile = [(0.216, FB_BOT(0.216)), (0.216, FB_TOP(0.216)), (0.486, FB_TOP(0.486)), (0.486, FB_BOT(0.486)), (0.356, FB_BOT(0.356))]
        parts.append(Part("fingerboard", "violin", "ebony", "pw", {"profile": fb_profile, "w0": -0.0165, "w1": 0.0165}, reg["fit"], tone=0.12))
        parts.append(Part("pegs", "violin", "ebony", "pv", {"profile": rect(0.495, 0.545, -0.042, 0.042), "v0": -0.025, "v1": -0.010}, reg["fit"], tone=0.12))
        parts.append(Part("bridge", "violin", "maple-bridge", "pw", {"profile": [(BRIDGE_U - 0.0022, TOP_ARCH), (BRIDGE_U + 0.0022, TOP_ARCH), (BRIDGE_U + 0.0008, BRIDGE_TOP), (BRIDGE_U - 0.0008, BRIDGE_TOP)], "w0": -0.0207, "w1": 0.0207}, reg["bridge"], tone=0.78))
        parts.append(Part("tailpiece", "violin", "ebony", "pv", {"profile": [(0.0, -0.008), (0.128, -0.020), (0.128, 0.020), (0.0, 0.008)], "v0": TOP_ARCH, "v1": 0.031}, reg["fit"], tone=0.14))
        parts.append(Part("fine-tuners", "violin", "string-steel", "pv", {"profile": rect(0.098, 0.120, -0.019, 0.019), "v0": 0.031, "v1": 0.037}, reg["str"], role="texture", tone=0.7, flags={"texture": True}, filled=False))
        parts.append(Part("chinrest", "violin", "ebony", "pv", {"profile": [(-0.008, -0.010), (0.058, -0.010), (0.060, 0.050), (0.040, 0.072), (-0.008, 0.070)], "v0": -0.016, "v1": 0.030}, reg["chin"], tone=0.15))
        p0, p1 = [BRIDGE_U, BRIDGE_TOP + 0.0004, 0.0], [NUT[0], NUT[1] + 0.0004, 0.0]
        parts.append(Part("strings", "violin", "string-steel", "capsule", {"p0": p0, "p1": p1, "r": 0.0006}, reg["str"], role="detail", tone=0.8, flags={"slender": True},
                          two_d=[("pw", {"profile": band((p0[0], p0[1]), (p1[0], p1[1]), 0.0006), "w0": -0.0127, "w1": 0.0127, "wtip": (-0.0045, 0.0045)}, "strings")]))
        parts.append(Part("worn-shoulder-back", "violin", "maple-bare-worn", "pv", {"profile": rect(0.036, 0.090, 0.050, 0.082), "v0": -RIB_H - 0.0085, "v1": -RIB_H - 0.0075}, reg["worn"], role="texture", tone=0.7, flags={"texture": True}, filled=False))
        parts.append(Part("rosin-dust", "violin", "rosin", "pv", {"profile": rect(0.166, 0.214, -0.028, 0.028), "v0": TOP_ARCH, "v1": TOP_ARCH + 0.0008}, None, role="texture", tone=0.9, flags={"texture": True}, filled=False))
    else:
        parts.append(Part("violin-body", "violin", "maple-flamed", "pv", {"profile": outline(1.0, 3), "v0": -RIB_H - BACK_ARCH, "v1": TOP_ARCH}, reg["back"], tone=0.45))
        head = [(0.350, -0.036), (0.400, -0.017), (0.486, -0.024), (0.560, -0.0415), (0.593, -0.0415), (0.593, -0.003), (0.486, 0.006), (0.356, 0.024), (0.216, 0.036), (0.216, 0.006), (0.350, 0.006)]
        parts.append(Part("violin-neck", "violin", "maple-bare-worn", "pw", {"profile": head, "w0": -0.015, "w1": 0.015}, reg["neck"], tone=0.4))
        parts.append(Part("top-face", "violin", "spruce-top-aged", "pv", {"profile": outline(0.80, 3), "v0": TOP_ARCH, "v1": TOP_ARCH + 0.001}, reg["top"], role="texture", tone=0.66, flags={"texture": True}, filled=False))
        parts.append(Part("fingerboard-face", "violin", "ebony", "pw", {"profile": [(0.216, 0.035), (0.486, 0.005), (0.486, 0.006), (0.216, 0.036)], "w0": -0.015, "w1": 0.015}, reg["fit"], role="texture", tone=0.12, flags={"texture": True}, filled=False))
        parts.append(Part("chinrest-face", "violin", "ebony", "pv", {"profile": [(0.0, 0.0), (0.058, 0.0), (0.058, 0.07), (0.0, 0.07)], "v0": TOP_ARCH, "v1": TOP_ARCH + 0.001}, reg["chin"], role="texture", tone=0.15, flags={"texture": True}, filled=False))
        parts.append(Part("bridge-face", "violin", "maple-bridge", "pw", {"profile": [(BRIDGE_U - 0.001, TOP_ARCH), (BRIDGE_U + 0.001, TOP_ARCH), (BRIDGE_U + 0.001, BRIDGE_TOP), (BRIDGE_U - 0.001, BRIDGE_TOP)], "w0": -0.0207, "w1": 0.0207}, reg["bridge"], role="texture", tone=0.78, flags={"texture": True}, filled=False))
        p0, p1 = [BRIDGE_U, BRIDGE_TOP + 0.0004, 0.0], [NUT[0], NUT[1] + 0.0004, 0.0]
        parts.append(Part("strings", "violin", "string-steel", "capsule", {"p0": p0, "p1": p1, "r": 0.0006}, reg["str"], role="detail", tone=0.8, flags={"slender": True},
                          two_d=[("pw", {"profile": band((p0[0], p0[1]), (p1[0], p1[1]), 0.0006), "w0": -0.0127, "w1": 0.0127, "wtip": (-0.0045, 0.0045)}, "strings")]))
        parts.append(Part("worn-shoulder-back", "violin", "maple-bare-worn", "pv", {"profile": rect(0.030, 0.095, 0.045, 0.085), "v0": -RIB_H - BACK_ARCH - 0.001, "v1": -RIB_H - BACK_ARCH}, reg["worn"], role="texture", tone=0.7, flags={"texture": True}, filled=False))
    return parts


def bow_parts(scale_id):
    parts = []
    secs = [{"at": [u, v, 0.0], "rx": r, "ry": r, "exponent": 2} for u, v, r in STICK]
    upper = [(u, v + r) for u, v, r in STICK]
    lower = [(u, v - r) for u, v, r in STICK][::-1]
    if scale_id == 50:
        parts.append(Part("bow-stick", "bow", "brazilwood", "loft", {"sections": secs}, "bow", tone=0.3, flags={"slender": True},
                          two_d=[("pw", {"profile": upper + lower, "w0": -0.0043, "w1": 0.0043}, "bow-stick")]))
        parts.append(Part("bow-grip", "bow", "winding-silver", "capsule", {"p0": [0.060, stick_v(0.060), 0.0], "p1": [0.125, stick_v(0.125), 0.0], "r": 0.0049}, "bow", tone=0.6,
                          two_d=None))
        frog = [(0.014, 0.0195), (0.058, 0.0195), (0.058, 0.004), (0.052, -0.001), (0.030, -0.001), (0.022, 0.004), (0.014, 0.012)]
        parts.append(Part("bow-frog", "bow", "ebony", "pw", {"profile": frog, "w0": -0.008, "w1": 0.008}, "bow", tone=0.12))
        parts.append(Part("bow-button", "bow", "winding-silver", "pu", {"profile": polygon_circle(0.0238, 0.0, 0.004, 8), "u0": -0.006, "u1": 0.014}, "bow", tone=0.6))
        head = [(0.716, 0.0215), (0.738, 0.0225), (0.748, 0.020), (0.750, 0.012), (0.744, 0.002), (0.722, -0.0015), (0.716, -0.0015)]
        parts.append(Part("bow-head", "bow", "brazilwood", "pw", {"profile": head, "w0": -0.0045, "w1": 0.0045}, "bow", tone=0.3))
        parts.append(Part("bow-hair", "bow", "horsehair", "pw", {"profile": rect(0.058, 0.722, -0.0005, 0.0005), "w0": -0.0055, "w1": 0.0055}, "bow", role="detail", tone=0.88, flags={"slender": True}))
    elif scale_id == 100:
        parts.append(Part("bow-stick", "bow", "brazilwood", "loft", {"sections": secs[::2]}, "bow", tone=0.3, flags={"slender": True},
                          two_d=[("pw", {"profile": upper[::2] + lower[::2], "w0": -0.0043, "w1": 0.0043}, "bow-stick")]))
        parts.append(Part("bow-frog", "bow", "ebony", "pw", {"profile": [(0.0, 0.0195), (0.058, 0.0195), (0.058, -0.001), (0.022, -0.001), (0.0, 0.012)], "w0": -0.0075, "w1": 0.0075}, "bow", tone=0.12))
        parts.append(Part("bow-head", "bow", "brazilwood", "pw", {"profile": [(0.712, 0.0225), (0.750, 0.0225), (0.750, -0.0015), (0.712, -0.0015)], "w0": -0.0075, "w1": 0.0075}, "bow", tone=0.3))
        parts.append(Part("bow-hair", "bow", "horsehair", "pw", {"profile": rect(0.058, 0.712, -0.0005, 0.0005), "w0": -0.0055, "w1": 0.0055}, "bow", role="detail", tone=0.88, flags={"slender": True}))
    else:
        parts.append(Part("bow-stick", "bow", "brazilwood", "loft", {"sections": [secs[0], secs[4], secs[-1]]}, "bow", tone=0.3, flags={"slender": True},
                          two_d=[("pw", {"profile": [upper[0], upper[4], upper[-1], lower[0], lower[4], lower[-1]], "w0": -0.0043, "w1": 0.0043}, "bow-stick")]))
        parts.append(Part("bow-hair", "bow", "horsehair", "pw", {"profile": rect(0.030, 0.740, -0.0005, 0.0005), "w0": -0.0055, "w1": 0.0055}, "bow", role="detail", tone=0.88, flags={"slender": True}))
    return parts


def case_parts(scale_id):
    parts = []
    L, Wh, H, wall = CASE_L, HALF_W, SHELL_H, WALL
    v_bot = -H
    walls2d = [("pv", {"profile": rect(0, L, -Wh, -Wh + wall), "v0": v_bot + wall, "v1": 0.0}, "shell-wall-front"),
               ("pv", {"profile": rect(0, L, Wh - wall, Wh), "v0": v_bot + wall, "v1": 0.0}, "shell-wall-back"),
               ("pv", {"profile": rect(0, wall, -Wh + wall, Wh - wall), "v0": v_bot + wall, "v1": 0.0}, "shell-wall-end-button"),
               ("pv", {"profile": rect(L - wall, L, -Wh + wall, Wh - wall), "v0": v_bot + wall, "v1": 0.0}, "shell-wall-end-scroll")]
    parts.append(Part("shell-walls", "case", "covered-plywood", "ring_zx", {"outer": rect(0, L, -Wh, Wh), "holes": [rect(wall, L - wall, -Wh + wall, Wh - wall)], "v0": v_bot + wall, "v1": 0.0}, "case-cover", tone=0.18, two_d=walls2d))
    parts.append(Part("shell-bottom", "case", "covered-plywood", "pv", {"profile": rect(0, L, -Wh, Wh), "v0": v_bot, "v1": v_bot + wall}, "case-cover", tone=0.16, flags={"sheet": True}))
    parts.append(Part("lining-bed", "case", "velvet-lining", "pv", {"profile": rect(wall, L - wall, -Wh + wall, Wh - wall), "v0": v_bot + wall, "v1": BED_TOP - H}, "case-lining", tone=0.32, flags={"sheet": True}))
    if scale_id in (50, 100):
        parts.append(Part("compartment", "case", "velvet-lining", "pv", {"profile": rect(0.705, L - wall, -Wh + wall, Wh - wall), "v0": BED_TOP - H, "v1": -0.040}, "case-lining", tone=0.36))
        parts.append(Part("end-block", "case", "velvet-lining", "pv", {"profile": rect(wall, 0.090, -0.060, 0.060), "v0": BED_TOP - H, "v1": -0.040}, "case-lining", tone=0.36))
        neck_back = VIOLIN_Y0 - H - 0.0149
        parts.append(Part("neck-rest", "case", "velvet-lining", "pv", {"profile": rect(0.505, 0.545, -0.018, 0.018), "v0": BED_TOP - H, "v1": neck_back}, "case-lining", tone=0.36))
    else:
        parts.append(Part("compartment", "case", "velvet-lining", "pv", {"profile": rect(0.700, L - wall, -Wh + wall, Wh - wall), "v0": BED_TOP - H, "v1": -0.038}, "case-lining", tone=0.36))
    if scale_id == 50:
        handle = [(0.34, -Wh), (0.355, -Wh), (0.355, -Wh - 0.018), (0.445, -Wh - 0.018), (0.445, -Wh), (0.46, -Wh), (0.46, -Wh - 0.030), (0.34, -Wh - 0.030)]
        parts.append(Part("handle", "case", "covered-plywood", "pv", {"profile": handle, "v0": -0.032, "v1": -0.012}, "case-cover", tone=0.2))
        for i, u in enumerate((0.20, 0.60)):
            parts.append(Part(f"latch-{i + 1}", "case", "nickel-steel", "pv", {"profile": rect(u - 0.015, u + 0.015, -Wh - 0.008, -Wh), "v0": -0.028, "v1": 0.0}, "case-hardware", tone=0.7))
        for i, u in enumerate((STRAP_RING[0], L - STRAP_RING[0])):
            parts.append(Part(f"strap-ring-{i + 1}", "case", "nickel-steel", "pv", {"profile": rect(u - 0.0125, u + 0.0125, -Wh - 0.008, -Wh), "v0": -0.020, "v1": -0.004}, "case-hardware", tone=0.7))
        for i, u in enumerate((0.15, 0.65)):
            parts.append(Part(f"hinge-{i + 1}", "case", "nickel-steel", "pv", {"profile": rect(u - 0.02, u + 0.02, Wh, Wh + 0.008), "v0": -0.016, "v1": 0.0}, "case-hardware", tone=0.7))
        for i, (u0, u1, w0, w1) in enumerate(((-0.0006, 0.0, -Wh + 0.004, -Wh + 0.050), (L, L + 0.0006, -Wh + 0.004, -Wh + 0.050), (-0.0006, 0.0, Wh - 0.050, Wh - 0.004), (L, L + 0.0006, Wh - 0.050, Wh - 0.004))):
            parts.append(Part(f"scuff-{i + 1}", "case", "leathercloth-scuffed", "pv", {"profile": rect(u0, u1, w0, w1), "v0": v_bot + 0.004, "v1": -0.030}, None, role="texture", tone=0.55, flags={"texture": True}, filled=False))
    elif scale_id == 100:
        handle = [(0.34, -Wh), (0.355, -Wh), (0.355, -Wh - 0.015), (0.445, -Wh - 0.015), (0.445, -Wh), (0.46, -Wh), (0.46, -Wh - 0.030), (0.34, -Wh - 0.030)]
        parts.append(Part("handle", "case", "covered-plywood", "pv", {"profile": handle, "v0": -0.032, "v1": -0.012}, "case-cover", tone=0.2))
        tx = {"role": "texture", "tone": 0.7, "flags": {"texture": True}, "filled": False}
        for i, u in enumerate((0.20, 0.60)):
            parts.append(Part(f"latch-{i + 1}-tx", "case", "nickel-steel", "pv", {"profile": rect(u - 0.015, u + 0.015, -Wh - 0.008, -Wh), "v0": -0.028, "v1": 0.0}, "case-hardware", **tx))
        for i, u in enumerate((STRAP_RING[0], L - STRAP_RING[0])):
            parts.append(Part(f"strap-ring-{i + 1}-tx", "case", "nickel-steel", "pv", {"profile": rect(u - 0.0125, u + 0.0125, -Wh - 0.008, -Wh), "v0": -0.020, "v1": -0.004}, "case-hardware", **tx))
        for i, u in enumerate((0.15, 0.65)):
            parts.append(Part(f"hinge-{i + 1}-tx", "case", "nickel-steel", "pv", {"profile": rect(u - 0.02, u + 0.02, Wh, Wh + 0.008), "v0": -0.016, "v1": 0.0}, "case-hardware", **tx))
    else:
        parts.append(Part("hardware-front", "case", "nickel-steel", "pv", {"profile": rect(STRAP_RING[0] - 0.0125, L - STRAP_RING[0] + 0.0125, -Wh - 0.008, -Wh), "v0": -0.032, "v1": -0.004}, "case-hardware", role="texture", tone=0.7, flags={"texture": True}, filled=False))
    return parts


def lid_parts(scale_id):
    parts = []
    L, W, H, wall = CASE_L, CASE_W, LID_H, WALL
    walls2d = [("pu", {"profile": rect(0, wall, 0, H - wall), "u0": 0.0, "u1": L}, "lid-wall-hinge"),
               ("pu", {"profile": rect(W - wall, W, 0, H - wall), "u0": 0.0, "u1": L}, "lid-wall-front"),
               ("pu", {"profile": rect(wall, W - wall, 0, H - wall), "u0": 0.0, "u1": wall}, "lid-wall-end-button"),
               ("pu", {"profile": rect(wall, W - wall, 0, H - wall), "u0": L - wall, "u1": L}, "lid-wall-end-scroll")]
    parts.append(Part("lid-walls", "lid", "covered-plywood", "ring_xy", {"outer": rect(0, L, 0, W), "holes": [rect(wall, L - wall, wall, W - wall)], "w0": 0.0, "w1": H - wall}, "case-cover", tone=0.2, two_d=walls2d))
    parts.append(Part("lid-top", "lid", "covered-plywood", "pu", {"profile": rect(0, W, H - wall, H), "u0": 0.0, "u1": L}, "case-cover", tone=0.16, flags={"sheet": True}))
    parts.append(Part("lid-lining", "lid", "velvet-lining", "pu", {"profile": rect(wall, W - wall, H - wall - 0.008, H - wall), "u0": wall, "u1": L - wall}, "case-lining", tone=0.32, flags={"sheet": True}))
    if scale_id == 50:
        for i, bu in enumerate((0.10, 0.68)):
            v = BOW_AT[1] + stick_v(bu)
            parts.append(Part(f"bow-clip-{i + 1}", "lid", "velvet-lining", "pu", {"profile": rect(v - 0.008, v + 0.008, 0.019, H - wall), "u0": BOW_AT[0] + bu - 0.008, "u1": BOW_AT[0] + bu + 0.008}, "case-lining", tone=0.36))
        for i, u in enumerate((0.20, 0.60)):
            parts.append(Part(f"lid-catch-{i + 1}", "lid", "nickel-steel", "pu", {"profile": rect(W, W + 0.008, 0.0, 0.014), "u0": u - 0.012, "u1": u + 0.012}, "case-hardware", tone=0.7))
        parts.append(Part("dealer-sticker", "lid", "sticker-paper", "pu", {"profile": rect(0.030, 0.100, H - wall - 0.008 - 0.0006, H - wall - 0.008), "u0": 0.50, "u1": 0.60}, None, role="texture", tone=0.85, flags={"texture": True}, filled=False))
    elif scale_id == 100:
        for i, u in enumerate((0.20, 0.60)):
            parts.append(Part(f"lid-catch-{i + 1}-tx", "lid", "nickel-steel", "pu", {"profile": rect(W, W + 0.008, 0.0, 0.014), "u0": u - 0.012, "u1": u + 0.012}, "case-hardware", role="texture", tone=0.7, flags={"texture": True}, filled=False))
    return parts


def strap_parts(scale_id):
    parts = []
    apex = (STRAP_SPAN / 2, STRAP_H)
    centre = [(0.0, 0.0), apex, (STRAP_SPAN, 0.0)]
    poly = offset_polyline(centre, STRAP_HALF if scale_id < 200 else 0.019)
    parts.append(Part("strap-webbing", "strap", "pp-webbing", "pw", {"profile": poly, "w0": -STRAP_T / 2, "w1": STRAP_T / 2}, "strap", tone=0.15, flags={"sheet": True}))
    if scale_id in (50, 100):
        parts.append(Part("shoulder-pad", "strap", "pp-webbing", "pw", {"profile": rect(apex[0] - PAD[0] / 2, apex[0] + PAD[0] / 2, apex[1] - PAD[1] / 2, apex[1] + PAD[1] / 2), "w0": -PAD[2] / 2, "w1": PAD[2] / 2}, "strap", tone=0.22))
    if scale_id == 50:
        for i, u in enumerate((0.0, STRAP_SPAN)):
            parts.append(Part(f"strap-hook-{i + 1}", "strap", "nickel-steel", "pw", {"profile": rect(u - 0.012, u + 0.012, 0.0, 0.035), "w0": -0.004, "w1": 0.004}, "case-hardware", tone=0.7))
    elif scale_id == 100:
        for i, u in enumerate((0.0, STRAP_SPAN)):
            parts.append(Part(f"strap-hook-{i + 1}-tx", "strap", "nickel-steel", "pw", {"profile": rect(u - 0.012, u + 0.012, 0.0, 0.035), "w0": -0.004, "w1": 0.004}, "case-hardware", role="texture", tone=0.7, flags={"texture": True}, filled=False))
    else:
        parts.append(Part("strap-hooks", "strap", "nickel-steel", "pw", {"profile": rect(-0.012, STRAP_SPAN + 0.012, 0.0, 0.0008), "w0": -0.004, "w1": 0.004}, "case-hardware", role="texture", tone=0.7, flags={"texture": True}, filled=False))
    return parts


def all_parts(scale_id):
    return case_parts(scale_id) + lid_parts(scale_id) + violin_parts(scale_id) + bow_parts(scale_id) + strap_parts(scale_id)


def structure_list(scale_id, parts):
    ids = {p.id for p in parts}
    out = []
    member = lambda sid, of, bone, part="member", members=None: out.append({"id": sid, "of": of, "member": bone, "part": part, **({"members": members} if members else {})})
    member("st-shell-walls", "shell-walls", "case")
    member("st-shell-bottom", "shell-bottom", "case")
    member("st-lid-walls", "lid-walls", "lid")
    member("st-lid-top", "lid-top", "lid")
    for of in ("violin-ribs", "violin-back-lower", "violin-back-upper", "violin-neck", "violin-body", "violin-back-arch", "violin-head"):
        if of in ids:
            member(f"st-{of}", of, "violin")
    member("st-bow-stick", "bow-stick", "bow")
    member("st-strap-webbing", "strap-webbing", "strap")
    member("st-bed", "lining-bed", "violin", "joint")
    for of in ("hinge-1", "hinge-2", "strap-ring-1", "strap-ring-2", "strap-hook-1", "strap-hook-2", "bow-clip-1", "bow-clip-2"):
        if of in ids:
            bone = {"hinge": "lid", "strap": "strap", "bow": "bow"}[of.split("-")[0]]
            member(f"st-{of}", of, bone, "joint")
    if scale_id != 50:
        r = 0.006 if scale_id == 100 else 0.012
        out.append({"id": "st-hooks", "kind": "box", "bone": "strap", "material": "nickel-steel", "role": "body", "tone": 0.6, "size": [2 * r + 0.002, 0.016, 0.016], "transform": {"translation": [0.0, 0.004, 0.0]}, "repeat": {"count": 2, "translation": [STRAP_SPAN, 0, 0]}, "member": "strap", "part": "joint"})
        out.append({"id": "st-rings", "kind": "box", "bone": "case", "material": "nickel-steel", "role": "body", "tone": 0.6, "size": [0.025, 0.016, 0.016], "transform": {"translation": [STRAP_RING[0], STRAP_RING[1], STRAP_RING[2] + 0.004]}, "repeat": {"count": 2, "translation": [STRAP_SPAN, 0, 0]}, "member": "case", "part": "joint"})
        out.append({"id": "st-bow-clips", "kind": "box", "bone": "lid", "material": "velvet-lining", "role": "body", "tone": 0.6, "size": [0.03, 0.03, 0.022], "transform": {"translation": [BOW_AT[0] + 0.10, BOW_AT[1] + stick_v(0.10), 0.028]}, "repeat": {"count": 2, "translation": [0.58, stick_v(0.68) - stick_v(0.10), 0]}, "member": "bow", "part": "joint"})
    r = {50: 0.008, 100: 0.008, 200: 0.016}[scale_id]
    for bone in MASS:
        if bone == "strap":
            continue
        cx, cy = COM[bone]
        out.append({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": {"case": "covered-plywood", "lid": "covered-plywood", "strap": "pp-webbing", "violin": "maple-flamed", "bow": "brazilwood"}[bone], "role": "body", "tone": 0.5,
                    "radii": [r, r, r], "transform": {"translation": [cx, cy, 0.0]}, "member": bone, "part": "mass"})
    return out


def thermal_list(parts):
    return [{"id": f"th-{p.id}", "of": p.id, "region": p.region} for p in parts if p.region]


def shapes_of(parts):
    out = []
    z = 1.0
    for p in parts:
        oz = F0[p.bone].o[2]
        for s in p.shapes(oz):
            s["z"] = round(z, 2)
            z += 0.05
            out.append(s)
    return out


def bbox(parts):
    lo, hi = [9.0] * 3, [-9.0] * 3
    for p in parts:
        if p.flags.get("texture"):
            continue
        f = F0[p.bone]
        g = p.geo
        cloud = []
        if p.kind == "pv":
            cloud = [[u, v, w] for u, w in g["profile"] for v in (g["v0"], g["v1"])]
        elif p.kind == "pw":
            cloud = [[u, v, w] for u, v in g["profile"] for w in (g["w0"], g["w1"])]
        elif p.kind == "pu":
            cloud = [[u, v, w] for v, w in g["profile"] for u in (g["u0"], g["u1"])]
        elif p.kind == "ring_zx":
            cloud = [[u, v, w] for u, w in g["outer"] for v in (g["v0"], g["v1"])]
        elif p.kind == "ring_xy":
            cloud = [[u, v, w] for u, v in g["outer"] for w in (g["w0"], g["w1"])]
        elif p.kind == "capsule":
            cloud = [add(q, [a, b, c]) for q in (g["p0"], g["p1"]) for a in (-g["r"], g["r"]) for b in (-g["r"], g["r"]) for c in (-g["r"], g["r"])]
        elif p.kind == "loft":
            cloud = [[s["at"][0], s["at"][1] + a * s["rx"], s["at"][2] + b * s["ry"]] for s in g["sections"] for a in (-1, 1) for b in (-1, 1)]
        for q in cloud:
            wq = f.world(q)
            for i in range(3):
                lo[i], hi[i] = min(lo[i], wq[i]), max(hi[i], wq[i])
    return lo, hi


REFERENCES = [
    {"id": "hornbostel-sachs-1914", "citation": "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift fuer Ethnologie 46, 553-590 (class 321.322-71: necked box lute sounded by a bow, the violin family)."},
    {"id": "johnson-courtnall-1999", "citation": "Johnson, C., Courtnall, R. (1999). The Art of Violin Making. Robert Hale, London (4/4 violin: body length 356 mm, bouts 168 / 112 / 208 mm, ribs 30 to 32 mm, arching 15 to 16 mm, neck 130 mm, string length 328 mm, stop 195 mm, projection 27 mm, overstand 6 to 7 mm, bridge 33 mm, fingerboard 270 mm, 24 to 42 mm wide)."},
    {"id": "heron-allen-1884", "citation": "Heron-Allen, E. (1884). Violin-Making, as It Was and Is. Ward, Lock and Co., London (outline, scroll, pegbox, button and the fittings of the classical violin)."},
    {"id": "wegst-2006", "citation": "Wegst, U. G. K. (2006). Wood for sound. American Journal of Botany 93(10), 1439-1448 (density and stiffness of spruce, maple, ebony and pernambuco or brazilwood for instruments and bows)."},
    {"id": "string-set-tension", "citation": "Catalogue-typical tension of a synthetic-core 4/4 violin string set with a steel E (string maker data sheets consulted 2026-11): E about 78 N, A 57 N, D 50 N, G 46 N, together about 230 to 240 N; gauges E 0.26 mm, A 0.6 mm, D and G about 0.8 mm."},
    {"id": "jansson-2002", "citation": "Jansson, E. V. (2002). Acoustics for Violin and Guitar Makers, 4th ed. KTH, Stockholm (string break angle over the bridge and down-bearing of about 80 to 100 N on the top)."},
    {"id": "askenfelt-1989", "citation": "Askenfelt, A. (1989). Measurement of the bowing parameters in violin playing. II: Bow-bridge distance, dynamic range, and limits of bow force. Journal of the Acoustical Society of America 86(2), 503-516 (bow force 0.5 to 1.5 N, bow-bridge distance 10 to 40 mm, bow speed 0.1 to 0.5 m/s)."},
    {"id": "obata-kinoshita-2012", "citation": "Obata, S., Kinoshita, H. (2012). Chin force in violin playing. European Journal of Applied Physiology 112, 2085-2095 (chin force on the chinrest of about 10 to 30 N while playing)."},
    {"id": "bow-catalogue", "citation": "Catalogue-typical data of 4/4 student and orchestra violin bows of brazilwood with ebony frog (dealer and maker catalogues consulted 2026-11): length 74 to 75 cm, 58 to 62 g, stick 8.6 mm at the frog tapering to about 5.5 mm, balance point about 24 cm from the end of the screw, hair ribbon about 11 mm."},
    {"id": "oblong-case-catalogue", "citation": "Catalogue-typical data of oblong 4/4 violin cases of the plywood shell type with leathercloth cover, velvet lining, two bow holders in the lid, accessory box, two latches, handle and a shoulder strap clipped to two rings (dealer catalogues consulted 2026-11): outside 79 to 81 x 24 to 26 x 12 to 13 cm, 2.5 to 3.2 kg."},
    {"id": "din-33402-2", "citation": "DIN 33402-2:2020 Ergonomics - Human body dimensions - Part 2: Values (shoulder height about 1.43 m, chin height about 1.53 m of the adult median, used for the shoulder and chin heights of the carried and played poses)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values (timber, plywood, polymer foams)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities and thermal properties of wood, steel, polymers and textiles)."},
]


def material(mid, en, de, eps, alpha, rho, k, cp, source, ref, e=None, strength=None):
    m = {"id": mid, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": cp}
    if e:
        m["elasticModulusPa"] = e
    if strength:
        m["strengthPa"] = strength
    m.update({"source": source, "reference": ref})
    return m


MATERIALS = [
    material("spruce-top-aged", "Spruce top under aged amber oil varnish, rosin-dusted", "Fichtendecke unter gealtertem bernsteinfarbenem Öllack, mit Kolophoniumstaub", 0.9, 0.6, 420, 0.11, 1700, "authored-estimate", "wegst-2006", 10e9, 40e6),
    material("maple-flamed", "Flamed maple under aged varnish (back, ribs, scroll)", "Geflammter Ahorn unter gealtertem Lack (Boden, Zargen, Schnecke)", 0.9, 0.62, 640, 0.16, 1600, "literature", "wegst-2006", 10.5e9, 60e6),
    material("maple-bare-worn", "Maple worn bare by the hand and the shoulder, darkened by grease", "Von Hand und Schulter blank gespielter, von Fett nachgedunkelter Ahorn", 0.92, 0.5, 640, 0.16, 1600, "authored-estimate", "incropera-2018", 10.5e9, 60e6),
    material("maple-bridge", "Unvarnished bridge maple", "Unlackiertes Stegahorn", 0.9, 0.4, 650, 0.16, 1600, "literature", "wegst-2006", 10e9),
    material("ebony", "Ebony (fingerboard, pegs, tailpiece, chinrest, frog)", "Ebenholz (Griffbrett, Wirbel, Saitenhalter, Kinnhalter, Frosch)", 0.9, 0.9, 1100, 0.2, 1600, "literature", "wegst-2006", 17e9, 100e6),
    material("string-steel", "Steel strings and nickel-plated fine tuners, tarnished", "Stahlsaiten und vernickelte Feinstimmer, angelaufen", 0.3, 0.5, 7850, 50, 470, "authored-estimate", "incropera-2018", 200e9, 2000e6),
    material("brazilwood", "Brazilwood bow stick under shellac", "Fernambuk-Ersatzholz (Brasilholz) des Bogens unter Schellack", 0.9, 0.7, 950, 0.2, 1600, "literature", "wegst-2006", 20e9, 150e6),
    material("winding-silver", "Nickel-silver winding, button and leather thumb grip, worn", "Neusilberwicklung, Beinchen und Lederdaumenauflage, abgegriffen", 0.4, 0.5, 8600, 30, 400, "authored-estimate", "incropera-2018"),
    material("horsehair", "Horsehair, greyed by rosin and use", "Rosshaar, von Kolophonium und Gebrauch vergraut", 0.9, 0.35, 1300, 0.2, 1500, "authored-estimate", "incropera-2018"),
    material("rosin", "Rosin dust (colophony)", "Kolophoniumstaub", 0.9, 0.3, 1070, 0.13, 1300, "authored-estimate", "incropera-2018"),
    material("covered-plywood", "Birch plywood 5 mm under black PVC leathercloth", "Birkensperrholz 5 mm unter schwarzem PVC-Kunstleder", 0.92, 0.9, 700, 0.15, 1600, "literature", "iso-10456-2007", 7e9, 30e6),
    material("leathercloth-scuffed", "Leathercloth scuffed through to the plywood at the corners", "An den Ecken bis aufs Sperrholz durchgescheuertes Kunstleder", 0.9, 0.7, 700, 0.15, 1600, "authored-estimate", "iso-10456-2007"),
    material("velvet-lining", "Polyester velvet over polyurethane foam, pressed flat by use", "Polyestersamt über Polyurethanschaum, vom Gebrauch plattgedrückt", 0.95, 0.8, 80, 0.04, 1500, "literature", "iso-10456-2007"),
    material("nickel-steel", "Nickel-plated steel hardware, tarnished", "Vernickelte Stahlbeschläge, angelaufen", 0.25, 0.5, 7850, 45, 470, "authored-estimate", "incropera-2018", 200e9, 400e6),
    material("pp-webbing", "Polypropylene webbing and padded shoulder pad", "Polypropylen-Gurtband und gepolsterte Schulterauflage", 0.9, 0.9, 910, 0.2, 1900, "literature", "iso-10456-2007", 1.5e9, 25e6),
    material("sticker-paper", "Paper label, yellowed", "Papieretikett, vergilbt", 0.9, 0.4, 800, 0.1, 1300, "authored-estimate", "iso-10456-2007"),
]
DENSITY = {m["id"]: m["densityKgM3"] for m in MATERIALS}

REGIONS = [
    ("violin-top", "Spruce top with aged varnish and rosin dust", "Fichtendecke mit gealtertem Lack und Kolophoniumstaub", "surface", "spruce-top-aged", 0.2, "horizontal-plate-up", 2000, 0.12),
    ("violin-back", "Maple back and ribs", "Ahornboden und Zargen", "surface", "maple-flamed", 0.2, "horizontal-plate-down", 3070, 0.14),
    ("violin-neck", "Neck, pegbox and scroll, the neck worn bare", "Hals, Wirbelkasten und Schnecke, der Hals blank gespielt", "surface", "maple-bare-worn", 0.025, "cylinder", 10240, 0.03),
    ("worn-varnish", "Varnish worn through at the shoulder and the upper bout", "An Schulterauflage und Oberbug durchgescheuerter Lack", "surface", "maple-bare-worn", 0.05, "horizontal-plate-down", 3070, 0.004),
    ("fingerboard-fittings", "Ebony fingerboard, pegs, tailpiece, tailgut and endpin", "Griffbrett, Wirbel, Saitenhalter, Saitenhalterschlinge und Endknopf aus Ebenholz", "surface", "ebony", 0.03, "cylinder", 10560, 0.03),
    ("chinrest", "Ebony chinrest with nickel bracket", "Kinnhalter aus Ebenholz mit Nickelbügel", "surface", "ebony", 0.06, "horizontal-plate-up", 14080, 0.008),
    ("bridge", "Maple bridge", "Ahornsteg", "surface", "maple-bridge", 0.033, "vertical-plate", 2080, 0.003),
    ("strings", "Four strings with fine tuners", "Vier Saiten mit Feinstimmern", "surface", "string-steel", 0.0007, "cylinder", 553, 0.004),
    ("bow", "Bow: stick, frog, head and hair", "Bogen: Stange, Frosch, Kopf und Haar", "surface", "brazilwood", 0.0086, "cylinder", 3268, 0.02),
    ("case-cover", "Leathercloth-covered plywood shell, lid and handle", "Kunstlederbezogene Sperrholzschale, Deckel und Griff", "surface", "covered-plywood", 0.25, "horizontal-plate-up", 6720, 1.0),
    ("case-lining", "Velvet lining, bed, blocks and bow holders", "Samtfutter, Bett, Klötze und Bogenhalter", "surface", "velvet-lining", 0.2, "horizontal-plate-up", 960, 0.6),
    ("case-hardware", "Latches, hinges, strap rings and snap hooks", "Verschlüsse, Scharniere, Gurtringe und Karabiner", "surface", "nickel-steel", 0.02, "vertical-plate", 3690, 0.01),
    ("strap", "Shoulder strap with pad", "Schultergurt mit Polster", "surface", "pp-webbing", 0.038, "vertical-plate", 5187, 0.06),
]
PASSIVE = {"violin-top", "bow", "case-cover"}


def regions():
    out = []
    for rid, en, de, kind, mat, length, geom, cap, area in REGIONS:
        r = {"id": rid, "title": T(en, de), "kind": kind, "model": "passive", "material": mat, "areaM2": round(AREAS.get(rid, area), 4), "characteristicLengthM": length, "geometry": geom, "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": cap}
        if rid in PASSIVE:
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        r["source"] = "authored-estimate"
        r["reference"] = "incropera-2018" if rid.startswith("case") or rid in ("strap", "strings") else "wegst-2006"
        out.append(r)
    return out


def bone(bid, parent, en, de, x, y, z, length, joint, angle=0.0, abduction=0.0, twist=0.0, mass=None, section=None):
    b = {"id": bid, "parent": parent, "title": T(en, de), "x": r6(x), "y": r6(y)}
    if z:
        b["z"] = r6(z)
    b["angleDeg"] = angle
    if abduction:
        b["abductionDeg"] = abduction
    if twist:
        b["twistDeg"] = twist
    b["lengthM"] = length
    b["joint"] = joint
    if mass:
        b["mass"] = mass
    if section:
        b["section"] = section
    return b


def seg(bid, ref):
    kg, src = MASS[bid]
    return {"kg": kg, "comX": COM[bid][0], "comY": COM[bid][1], "distribution": "point", "source": src, "reference": ref}


def lim(values, margin=0.02):
    return round(min(values + [0.0]) - margin, 3), round(max(values + [0.0]) + margin, 3)


def rig():
    bs_lo, bs_hi = lim([PLAYED["bow-slide"]["dx"]])
    br_lo, br_hi = lim([PLAYED["bow-reach"]["dy"]])
    bw_lo, bw_hi = lim([PLAYED["bow"]["dx"], STROKE["frog"], STROKE["tip"]])
    vl_lo, vl_hi = lim([PLAYED["violin-lift"]["dy"]])
    hold = PLAYED["bow-hold"]
    vio = PLAYED["violin"]
    return {"kind": "mechanical", "root": "floor", "bones": [
        bone("floor", None, "Shop floor (massless reference)", "Ladenboden (masselose Bezugslinie)", 0, 0, 0, 0, {"type": "fixed"}),
        bone("case", "floor", "Lower shell of the oblong case with lining, handle, latches and strap rings, rolled about its rim line when hung from the strap", "Unterschale des Rechteckkoffers mit Futter, Griff, Verschlüssen und Gurtringen, am Gurt hängend um die Randlinie gerollt",
             -CASE_L / 2, SHELL_H, 0, CASE_L, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -100, "twistMaxDeg": 0}, mass=seg("case", "oblong-case-catalogue")),
        bone("lid", "case", "Lid on its rear hinge line, resting 8 degrees past vertical against its cloth stop", "Deckel an seiner hinteren Scharnierlinie, 8 Grad über die Senkrechte am Stoffanschlag ruhend",
             0, 0, HALF_W, CASE_L, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": 0, "twistMaxDeg": LID_CLOSED}, twist=LID_LEAN, mass=seg("lid", "oblong-case-catalogue")),
        bone("bow-slide", "lid", "Bow carrier along the lid (massless)", "Bogenträger längs des Deckels (masselos)", *BOW_AT, 0, {"type": "prismatic", "axisDeg": 0, "minM": bs_lo, "maxM": bs_hi}),
        bone("bow-reach", "bow-slide", "Bow carrier across the lid, the reach of the bow arm (massless)", "Bogenträger quer zum Deckel, die Reichweite des Bogenarms (masselos)", 0, 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": br_lo, "maxM": br_hi}),
        bone("bow-hold", "bow-reach", "Bow hold: the orientation the right hand gives the bow (massless)", "Bogenhaltung: die Lage, die die rechte Hand dem Bogen gibt (masselos)", 0, 0, 0, 0,
             {"type": "revolute", "minDeg": -180, "maxDeg": 180, "twistMinDeg": -180, "twistMaxDeg": 180}),
        bone("bow", "bow-hold", "Bow from the screw button to the tip, sliding along its hair in the stroke", "Bogen vom Beinchen bis zur Spitze, im Strich längs seiner Haare gleitend", 0, 0, 0, BOW_L,
             {"type": "prismatic", "axisDeg": 0, "minM": bw_lo, "maxM": bw_hi}, mass=seg("bow", "bow-catalogue"),
             section={"areaM2": 5.81e-5, "secondMomentM4": 2.69e-10, "extremeFibreM": 0.0043, "material": "brazilwood", "title": T("Round stick 8.6 mm at the frog", "Runde Stange 8,6 mm am Frosch")}),
        bone("strap", "case", "Shoulder strap between the two front rings, lying on the floor or hanging from the shoulder", "Schultergurt zwischen den beiden vorderen Ringen, am Boden liegend oder von der Schulter hängend",
             *STRAP_RING, STRAP_SPAN, {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -100, "twistMaxDeg": 0}, twist=STRAP_TWIST, mass=seg("strap", "oblong-case-catalogue"),
             section={"areaM2": 9.5e-5, "secondMomentM4": 4.95e-11, "extremeFibreM": 0.00125, "material": "pp-webbing", "title": T("Webbing 38 x 2.5 mm", "Gurtband 38 x 2,5 mm")}),
        bone("violin-lift", "case", "Violin carrier, lifted out of the bed to the collarbone (massless)", "Geigenträger, aus dem Bett bis zum Schlüsselbein gehoben (masselos)", VIOLIN_X0 + CASE_L / 2, VIOLIN_Y0 - SHELL_H, 0, 0,
             {"type": "prismatic", "axisDeg": 90, "minM": vl_lo, "maxM": vl_hi}),
        bone("violin", "violin-lift", "Violin from the endpin to the scroll, turned into the playing position under the chin", "Geige vom Endknopf bis zur Schnecke, unter dem Kinn in die Spielhaltung gedreht", 0, 0, 0, VIOLIN_L,
             {"type": "revolute", "minDeg": -180, "maxDeg": 180, "twistMinDeg": -180, "twistMaxDeg": 180}, mass=seg("violin", "johnson-courtnall-1999"),
             section={"areaM2": 4.5e-4, "secondMomentM4": 1.67e-8, "extremeFibreM": 0.0105, "material": "maple-flamed", "title": T("Neck at the heel, D section 27 x 21 mm", "Hals am Fuß, D-Querschnitt 27 x 21 mm")}),
    ]}


APEX_U = STRAP_SPAN / 2


def anchors():
    vio_back = [0.06, -RIB_H - BACK_ARCH, 0.05]
    return [
        {"id": "case-floor", "bone": "case", "x": CASE_L / 2, "y": -SHELL_H, "kind": "contact", "title": T("Bottom of the shell on the shop floor", "Schalenboden auf dem Ladenboden"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.78, "footprintWidthM": 0.235, "frictionCoefficient": 0.5},
        {"id": "shoulder", "bone": "strap", "x": APEX_U, "y": STRAP_H, "kind": "support", "title": T("Shoulder under the strap pad", "Schulter unter dem Gurtpolster"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.14, "footprintWidthM": 0.06},
        {"id": "collarbone", "bone": "violin", "x": vio_back[0], "y": vio_back[1], "z": vio_back[2], "kind": "contact", "title": T("Left collarbone under the lower bout of the back", "Linkes Schlüsselbein unter dem Unterbug des Bodens"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.05, "footprintWidthM": 0.04, "frictionCoefficient": 0.6},
        {"id": "left-hand", "bone": "violin", "x": 0.47, "y": -0.016, "kind": "support", "title": T("Left hand at the neck in first position", "Linke Hand am Hals in der ersten Lage"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.03, "footprintWidthM": 0.025},
        {"id": "right-hand", "bone": "bow", "x": 0.075, "y": stick_v(0.075), "kind": "support", "title": T("Right hand at the frog: thumb and fingers on the stick", "Rechte Hand am Frosch: Daumen und Finger an der Stange"), "support": "pin", "normalDeg": 90, "footprintLengthM": 0.04, "footprintWidthM": 0.02},
        {"id": "chin", "bone": "violin", "x": 0.03, "y": 0.030, "z": 0.035, "kind": "load", "title": T("Chin on the chinrest", "Kinn auf dem Kinnhalter")},
    ]


def string_loads(fr):
    fv = fr["violin"]
    nut = [NUT[0], NUT_V["d"], 0.0]
    top = [BRIDGE_U, BRIDGE_V["d"], 0.0]
    hole = [TAIL_HOLE[0], TAIL_HOLE[1], 0.0]
    d1 = unit(mat_vec(fv.r, sub(top, nut)))
    d2 = unit(mat_vec(fv.r, sub(top, hole)))
    f1, f2 = scale(d1, STRING_T), scale(d2, STRING_T)
    f3 = scale(add(f1, f2), -1)
    def load(lid, en, de, p, f):
        return {"id": lid, "kind": "imposed", "title": T(en, de), "bone": "violin", "x": r6(p[0]), "y": r6(p[1]), "fxN": round(f[0], 2), "fyN": round(f[1], 2), "fzN": round(f[2], 2), "source": "literature", "reference": "string-set-tension"}
    return [load("strings-at-nut", "Pull of the four strings on the nut, 235 N", "Zug der vier Saiten am Sattel, 235 N", nut, f1),
            load("strings-at-tailpiece", "Pull of the four strings through the tailpiece and its gut on the saddle, 235 N", "Zug der vier Saiten über Saitenhalter und Schlinge am Untersattel, 235 N", hole, f2),
            load("strings-on-bridge", f"Down-bearing of the strings on the bridge, {round(math.sqrt(dot(f3, f3)))} N", f"Saitendruck auf den Steg, {round(math.sqrt(dot(f3, f3)))} N", top, f3)]


def played_loads(fr):
    fv, fb = fr["violin"], fr["bow"]
    chin = scale(fv.axis(1), -CHIN_FORCE)
    n = BOW_NORMAL
    on_bow_local = fb.local(CONTACT_W)
    out = string_loads(fr)
    out.append({"id": "chin-force", "kind": "occupant", "title": T("Chin pressing the violin onto the collarbone, 15 N", "Kinn drückt die Geige auf das Schlüsselbein, 15 N"), "anchor": "chin", "fxN": round(chin[0], 2), "fyN": round(chin[1], 2), "fzN": round(chin[2], 2), "source": "literature", "reference": "obata-kinoshita-2012"})
    out.append({"id": "bow-pressure", "kind": "occupant", "title": T("Bow force of the hair on the A string, 0.8 N", "Bogendruck der Haare auf die A-Saite, 0,8 N"), "bone": "violin", "x": r6(CONTACT_U), "y": r6(CONTACT_V), "z": r6(CONTACT_WL), "fxN": round(-n[0] * BOW_FORCE, 3), "fyN": round(-n[1] * BOW_FORCE, 3), "fzN": round(-n[2] * BOW_FORCE, 3), "source": "literature", "reference": "askenfelt-1989"})
    out.append({"id": "string-on-hair", "kind": "occupant", "title": T("Reaction of the A string on the bow hair, 0.8 N", "Gegenkraft der A-Saite auf die Bogenhaare, 0,8 N"), "bone": "bow", "x": r6(on_bow_local[0]), "y": r6(on_bow_local[1]), "z": r6(on_bow_local[2]), "fxN": round(n[0] * BOW_FORCE, 3), "fyN": round(n[1] * BOW_FORCE, 3), "fzN": round(n[2] * BOW_FORCE, 3), "source": "literature", "reference": "askenfelt-1989"})
    return out


ALL_REGIONS = [r[0] for r in REGIONS]


def binding(kind):
    rb = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in ALL_REGIONS}
    inside = ["violin-top", "violin-back", "violin-neck", "worn-varnish", "fingerboard-fittings", "chinrest", "bridge", "strings", "bow", "case-lining"]
    if kind in ("stored", "played"):
        rb["case-cover"] = {"exposedFraction": 0.85, "sunlitFraction": 0, "contact": {"fraction": 0.15, "tempC": 19}}
        rb["strap"] = {"exposedFraction": 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.4, "tempC": 19}}
        rb["case-lining"] = {"exposedFraction": 0.8, "sunlitFraction": 0}
    if kind == "stored":
        rb["violin-back"] = {"exposedFraction": 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.2, "tempC": 20}}
        rb["worn-varnish"] = {"exposedFraction": 0.6, "sunlitFraction": 0}
        rb["bow"] = {"exposedFraction": 0.7, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 20}}
    if kind in ("closed", "carried"):
        for r in inside:
            rb[r] = {"exposedFraction": 0.2, "sunlitFraction": 0}
        rb["case-cover"] = {"exposedFraction": 0.85, "sunlitFraction": 0, "contact": {"fraction": 0.15, "tempC": 19 if kind == "closed" else 31}}
        rb["strap"] = {"exposedFraction": 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.4, "tempC": 19}} if kind == "closed" else {"exposedFraction": 0.85, "sunlitFraction": 0, "contact": {"fraction": 0.15, "tempC": 32}}
    if kind == "played":
        rb["chinrest"] = {"exposedFraction": 0.6, "sunlitFraction": 0, "contact": {"fraction": 0.4, "tempC": 34}}
        rb["violin-neck"] = {"exposedFraction": 0.7, "sunlitFraction": 0, "contact": {"fraction": 0.3, "tempC": 33}}
        rb["worn-varnish"] = {"exposedFraction": 0.5, "sunlitFraction": 0, "contact": {"fraction": 0.5, "tempC": 33}}
        rb["violin-back"] = {"exposedFraction": 0.92, "sunlitFraction": 0, "contact": {"fraction": 0.08, "tempC": 33}}
        rb["bow"] = {"exposedFraction": 0.92, "sunlitFraction": 0, "contact": {"fraction": 0.08, "tempC": 33}}
        rb["strings"] = {"exposedFraction": 0.95, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 33}}
    return {"regions": rb, "emitters": {}}


def poses():
    f_stored = frames()
    f_closed = frames({"lid": {"twistDeg": LID_CLOSED}})
    f_carried = frames(CARRY, (0.0, LIFT))
    f_played = frames(PLAYED)
    return [
        {"id": "stored", "title": T("Stored open at the dealer", "Offen beim Händler gelagert"),
         "description": T("The case lies open on the shop floor: the violin in its velvet bed with the scroll on the neck rest, the bow in its two holders in the lid, which rests 8 degrees past vertical, the shoulder strap laid out on the floor in front.",
                          "Der Koffer liegt offen auf dem Ladenboden: die Geige in ihrem Samtbett mit der Schnecke auf der Halsstütze, der Bogen in seinen beiden Haltern im Deckel, der 8 Grad über die Senkrechte geöffnet ruht, der Schultergurt davor auf dem Boden ausgelegt."),
         "bones": {}, "ground": "authored", "contacts": ["case-floor"], "thermal": binding("stored"), "loads": string_loads(f_stored)},
        {"id": "case-closed", "title": T("Case closed", "Koffer geschlossen"),
         "description": T("The lid is closed and latched over the violin; the bow lies in the lid above the treble side, 4 mm clear of the bridge; the strap still lies on the floor.",
                          "Der Deckel ist über der Geige geschlossen und verriegelt; der Bogen liegt im Deckel über der Diskantseite, 4 mm über dem Steg; der Gurt liegt noch am Boden."),
         "bones": {"lid": {"twistDeg": LID_CLOSED}}, "ground": "authored", "contacts": ["case-floor"], "thermal": binding("closed"), "loads": string_loads(f_closed)},
        {"id": "carried", "title": T("Carried on the shoulder strap", "Am Schultergurt getragen"),
         "description": T(f"The closed case hangs from the strap pad on the shoulder at 1.45 m; it rolls {abs(CARRY['case']['twistDeg']):.0f} degrees onto its back so that its centre of mass hangs below the strap rings, handle side up, against the hip.",
                          f"Der geschlossene Koffer hängt am Gurtpolster auf der Schulter in 1,45 m Höhe; er rollt {abs(CARRY['case']['twistDeg']):.0f} Grad auf seine Rückseite, sodass sein Schwerpunkt unter den Gurtringen hängt, Griffseite oben, an der Hüfte."),
         "bones": CARRY, "root": {"y": LIFT}, "ground": "authored", "contacts": ["shoulder"], "thermal": binding("carried"), "loads": string_loads(f_carried)},
        {"id": "played", "title": T("Played standing", "Im Stehen gespielt"),
         "description": T("The violin rests with its back on the left collarbone at 1.40 m under the chin, scroll raised 12 degrees and rolled 35 degrees towards the bow, the left hand at the neck in first position; the bow lies on the A string 24 mm from the bridge with its middle; the open case stays on the floor.",
                          "Die Geige liegt mit dem Boden auf dem linken Schlüsselbein in 1,40 m Höhe unter dem Kinn, die Schnecke 12 Grad gehoben und 35 Grad zum Bogen gerollt, die linke Hand am Hals in der ersten Lage; der Bogen liegt mit seiner Mitte 24 mm vor dem Steg auf der A-Saite; der offene Koffer bleibt am Boden."),
         "bones": PLAYED, "ground": "authored", "contacts": ["case-floor", "collarbone", "left-hand", "right-hand"], "thermal": binding("played"), "loads": played_loads(f_played)},
    ]


def clips():
    return [
        {"id": "open-case", "title": T("Open the case", "Koffer öffnen"), "description": T("The two latches spring open, the lid swings back on its hinge line in 1.6 seconds and settles against its cloth stop 8 degrees past vertical.", "Die beiden Verschlüsse springen auf, der Deckel schwingt in 1,6 Sekunden auf seiner Scharnierlinie zurück und legt sich 8 Grad über der Senkrechten an seinen Stoffanschlag."),
         "durationS": 2.4, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "case-closed", "ease": "step"}, {"t": 0.4, "pose": "case-closed", "ease": "smooth"}, {"t": 2.0, "pose": "stored", "ease": "smooth"}, {"t": 2.4, "pose": "stored", "ease": "smooth"}],
         "contacts": [{"anchor": "case-floor", "fromS": 0, "toS": 2.4}],
         "phases": [{"id": "unlatch", "fromS": 0, "toS": 0.4, "title": T("Latches open", "Verschlüsse öffnen")}, {"id": "lift-lid", "fromS": 0.4, "toS": 2.0, "title": T("Lid swings open", "Deckel schwingt auf")}, {"id": "rest-open", "fromS": 2.0, "toS": 2.4, "title": T("Lid at its stop", "Deckel am Anschlag")}],
         "events": [{"t": 0.4, "kind": "motion", "title": T("Lid leaves the rim", "Deckel hebt vom Rand ab"), "target": "lid"}, {"t": 2.0, "kind": "motion", "title": T("Lid reaches its cloth stop", "Deckel erreicht seinen Stoffanschlag"), "target": "lid"}],
         "tracks": []},
        {"id": "bowing", "title": T("Whole bows on the A string", "Ganze Bögen auf der A-Saite"), "description": T("A down-bow from the frog to the tip in two seconds and an up-bow back, the hair parallel to the bridge 24 mm from it; the bow force eases from 1.0 N at the frog to 0.7 N at the tip, where the arm presses less weight into the string.", "Ein Abstrich vom Frosch zur Spitze in zwei Sekunden und ein Aufstrich zurück, die Haare parallel zum Steg 24 mm vor ihm; der Bogendruck lässt von 1,0 N am Frosch auf 0,7 N an der Spitze nach, wo der Arm weniger Gewicht in die Saite legt."),
         "durationS": 4, "fps": 25, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "played", "ease": "smooth", "bones": {"bow": {"dx": STROKE["frog"]}}}, {"t": 2, "pose": "played", "ease": "smooth", "bones": {"bow": {"dx": STROKE["tip"]}}}, {"t": 4, "pose": "played", "ease": "smooth", "bones": {"bow": {"dx": STROKE["frog"]}}}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 4} for a in ("case-floor", "collarbone", "left-hand", "right-hand")],
         "phases": [{"id": "down-bow", "fromS": 0, "toS": 2, "title": T("Down-bow", "Abstrich")}, {"id": "up-bow", "fromS": 2, "toS": 4, "title": T("Up-bow", "Aufstrich")}],
         "events": [{"t": 0, "kind": "motion", "title": T("Bow change at the frog", "Bogenwechsel am Frosch"), "target": "bow"}, {"t": 2, "kind": "motion", "title": T("Bow change at the tip", "Bogenwechsel an der Spitze"), "target": "bow"}],
         "tracks": [{"target": "load:bow-pressure", "ease": "smooth", "keys": [{"t": 0, "v": 1.25}, {"t": 2, "v": 0.875}, {"t": 4, "v": 1.25}]}, {"target": "load:string-on-hair", "ease": "smooth", "keys": [{"t": 0, "v": 1.25}, {"t": 2, "v": 0.875}, {"t": 4, "v": 1.25}]}]},
        {"id": "carry", "title": T("Close the case and shoulder it", "Koffer schließen und schultern"), "description": T("The lid closes in 1.5 seconds and is latched; the player takes the strap pad, lifts it onto the shoulder and the case rolls onto its back as it leaves the floor.", "Der Deckel schließt sich in 1,5 Sekunden und wird verriegelt; der Spieler greift das Gurtpolster, hebt es auf die Schulter, und der Koffer rollt auf seine Rückseite, während er den Boden verlässt."),
         "durationS": 6, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.5, "pose": "case-closed", "ease": "smooth"}, {"t": 2.5, "pose": "case-closed", "ease": "smooth"}, {"t": 5.0, "pose": "carried", "ease": "smooth"}, {"t": 6.0, "pose": "carried", "ease": "smooth"}],
         "contacts": [{"anchor": "case-floor", "fromS": 0, "toS": 2.5}, {"anchor": "shoulder", "fromS": 2.5, "toS": 6}],
         "phases": [{"id": "close-lid", "fromS": 0, "toS": 1.5, "title": T("Lid closes", "Deckel schließt")}, {"id": "latch", "fromS": 1.5, "toS": 2.5, "title": T("Latching", "Verriegeln")}, {"id": "lift", "fromS": 2.5, "toS": 5.0, "title": T("Lifted onto the shoulder", "Auf die Schulter gehoben")}, {"id": "carry", "fromS": 5.0, "toS": 6.0, "title": T("Carried", "Getragen")}],
         "events": [{"t": 1.5, "kind": "motion", "title": T("Lid closed", "Deckel geschlossen"), "target": "lid"}, {"t": 2.5, "kind": "contact-off", "title": T("Case leaves the floor", "Koffer verlässt den Boden"), "target": "case-floor"}, {"t": 2.5, "kind": "contact-on", "title": T("Strap pad in the hand, then on the shoulder", "Gurtpolster in der Hand, dann auf der Schulter"), "target": "shoulder"}],
         "tracks": []},
    ]


PENS = {50: {"body": 0.25, "detail": 0.2, "texture": 0.1, "glazing": None, "ground": 0.35},
        100: {"body": 0.25, "detail": 0.2, "texture": 0.1, "glazing": None, "ground": 0.35},
        200: {"body": 0.25, "detail": 0.2, "texture": 0.1, "glazing": None, "ground": 0.25}}
RANGES = {50: (20, 75), 100: (75, 150), 200: (150, 350)}

REP_DECISIONS = {
    50: {
        "silhouette": T("At 1:50 the open case is 16 x 6.6 mm on paper: in side elevation the front wall with handle, latches and strap rings, above it the fingerboard, bridge and chinrest of the violin and the standing lid with the bow; in plan the whole violin in its bed, the narrow lid edge behind it and the strap laid out in front; in front elevation the case end, the scroll, the pegs and the lid edge.",
                        "Bei 1:50 ist der offene Koffer 16 x 6,6 mm groß: in der Seitenansicht die Vorderwand mit Griff, Verschlüssen und Gurtringen, darüber Griffbrett, Steg und Kinnhalter der Geige und der stehende Deckel mit dem Bogen; im Grundriss die ganze Geige in ihrem Bett, dahinter die schmale Deckelkante und davor der ausgelegte Gurt; in der Vorderansicht die Kofferstirn, Schnecke, Wirbel und Deckelkante."),
        "retained": T("The violin outline with its bouts, corners and C-bouts, ribs and two-step arching of top and back, neck, pegbox, scroll, four pegs, the tilted fingerboard, bridge, tailpiece with fine tuners, tailgut, endpin, chinrest with bracket and four strings with their afterlengths; the bow with stick, grip, frog, button, head and hair; the case with shell, bottom, bed, compartment, end block, neck rest, handle, two latches, two hinges, two strap rings, lid walls, top, lining, two bow holders, two catches; the strap with webbing, pad and two hooks.",
                      "Der Geigenumriss mit Bügen, Ecken und C-Bügen, Zargen und zweistufiger Wölbung von Decke und Boden, Hals, Wirbelkasten, Schnecke, vier Wirbel, das geneigte Griffbrett, Steg, Saitenhalter mit Feinstimmern, Saitenhalterschlinge, Endknopf, Kinnhalter mit Bügel und vier Saiten mit ihren Nachlängen; der Bogen mit Stange, Griff, Frosch, Beinchen, Kopf und Haar; der Koffer mit Schale, Boden, Bett, Fach, Endklotz, Halsstütze, Griff, zwei Verschlüssen, zwei Scharnieren, zwei Gurtringen, Deckelwänden, Deckelplatte, Futter, zwei Bogenhaltern, zwei Schließblechen; der Gurt mit Band, Polster und zwei Karabinern."),
        "merged": T("Plates and ribs share one outline (the 2.5 mm overhang is below the 7.5 mm feature); each arching is two steps; the nut is part of the fingerboard; peg shaft and thumb piece are one paddle; the chinrest cork and the inner blocks are not drawn apart.",
                    "Platten und Zargen teilen einen Umriss (der Überstand von 2,5 mm liegt unter dem Merkmal von 7,5 mm); jede Wölbung ist zweistufig; der Sattel gehört zum Griffbrett; Wirbelschaft und Griffplatte sind ein Paddel; Kinnhalterkork und Innenklötze werden nicht getrennt gezeichnet."),
        "omitted": T("f-holes, purfling, soundpost, bass bar, linings, the rosin cake inside the compartment, the screws of the hinges and the stitching of the strap.",
                     "f-Löcher, Einlage, Stimmstock, Bassbalken, Reifchen, der Kolophoniumblock im Fach, die Schrauben der Scharniere und die Nähte des Gurts."),
        "thinElements": T("Strings (0.26 to 0.8 mm), hair ribbon (1 mm), bridge (4.4 to 1.6 mm) and the 3 mm webbing keep their true thickness and are drawn with the 0.2 mm detail pen; the 0.6 mm sheets of the scuffs, the worn patches, the rosin dust and the label are texture outlines.",
                          "Saiten (0,26 bis 0,8 mm), Haarband (1 mm), Steg (4,4 bis 1,6 mm) und das 3-mm-Gurtband behalten ihre wahre Dicke und werden mit dem Detailstift 0,2 mm gezeichnet; die 0,6-mm-Flächen der Scheuerstellen, der blanken Stellen, des Kolophoniumstaubs und des Etiketts sind Texturumrisse."),
        "hatching": T("No hatch: the tones separate the dark leathercloth and ebony from the amber spruce, the maple and the velvet.", "Keine Schraffur: die Töne trennen dunkles Kunstleder und Ebenholz von bernsteinfarbener Fichte, Ahorn und Samt."),
        "styles": T("Body pen 0.25 mm for the outlines, detail pen 0.2 mm for strings and hair, texture pen 0.1 mm for traces of use.", "Körperstift 0,25 mm für die Umrisse, Detailstift 0,2 mm für Saiten und Haar, Texturstift 0,1 mm für Gebrauchsspuren."),
        "overlays": T("Thermal regions per part; the structural view draws the case, lid, violin, bow and strap members with their joints and mass marks.", "Thermische Regionen je Teil; die Tragwerksansicht zeigt Stäbe von Koffer, Deckel, Geige, Bogen und Gurt mit Gelenken und Massenmarken."),
    },
    100: {
        "silhouette": T("At 1:100 the open case is 8 x 3.3 mm on paper: the same three views with the violin reduced to body, arch, back arch, head and fingerboard and the case to shell, bed, blocks, handle and lid.",
                        "Bei 1:100 ist der offene Koffer 8 x 3,3 mm groß: dieselben drei Ansichten mit der Geige als Korpus, Wölbung, Bodenwölbung, Kopf und Griffbrett und dem Koffer als Schale, Bett, Klötze, Griff und Deckel."),
        "retained": T("Outline of the violin, one arching step for the top and the back, the head from the heel to the scroll, the tilted fingerboard, the pegs as one cross bar 84 mm wide, the bridge, tailpiece, chinrest and the strings as one band; bow stick, frog, head and hair; case shell, bottom, bed, compartment, end block, neck rest, handle, lid walls, top and lining; strap webbing and pad.",
                      "Umriss der Geige, eine Wölbungsstufe für Decke und Boden, der Kopf vom Halsfuß bis zur Schnecke, das geneigte Griffbrett, die Wirbel als ein 84 mm breiter Querriegel, Steg, Saitenhalter, Kinnhalter und die Saiten als ein Band; Bogenstange, Frosch, Kopf und Haar; Kofferschale, Boden, Bett, Fach, Endklotz, Halsstütze, Griff, Deckelwände, Deckelplatte und Futter; Gurtband und Polster."),
        "merged": T("Ribs and the lower back step are one prism; neck, pegbox and scroll are one head 31 mm wide; the four pegs are one bar; four strings are one band; latches, rings, hooks, catches and fine tuners are texture outlines.",
                    "Zargen und untere Bodenstufe sind ein Prisma; Hals, Wirbelkasten und Schnecke sind ein Kopf von 31 mm Breite; die vier Wirbel sind ein Riegel; vier Saiten sind ein Band; Verschlüsse, Ringe, Karabiner, Schließbleche und Feinstimmer sind Texturumrisse."),
        "omitted": T("Bow holders, hinges, endpin, tailgut, bow button and grip, the label and the corner scuffs.", "Bogenhalter, Scharniere, Endknopf, Saitenhalterschlinge, Beinchen und Griff des Bogens, das Etikett und die Eckscheuerstellen."),
        "thinElements": T("Strings, hair, bridge and webbing keep their true thickness, drawn with the 0.2 mm detail pen and the 0.25 mm body pen.", "Saiten, Haar, Steg und Gurtband behalten ihre wahre Dicke und werden mit dem Detailstift 0,2 mm und dem Körperstift 0,25 mm gezeichnet."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Body pen 0.25 mm, detail pen 0.2 mm, texture pen 0.1 mm.", "Körperstift 0,25 mm, Detailstift 0,2 mm, Texturstift 0,1 mm."),
        "overlays": T("Thermal regions per part, merged hardware as texture; structural members as at 1:50.", "Thermische Regionen je Teil, zusammengefasste Beschläge als Textur; Tragwerksstäbe wie bei 1:50."),
    },
    200: {
        "silhouette": T("At 1:200 the open case is 4 x 1.6 mm on paper: a dark box with the standing lid, the violin as one body with its neck rising above the front wall, the bow as a line in the lid and the strap as a thin V on the floor.",
                        "Bei 1:200 ist der offene Koffer 4 x 1,6 mm groß: ein dunkler Kasten mit dem stehenden Deckel, die Geige als ein Korpus mit dem über die Vorderwand ragenden Hals, der Bogen als Linie im Deckel und der Gurt als dünnes V am Boden."),
        "retained": T("Overall 0.80 x 0.33 x 0.89 m, the shell and its lid with their walls, the violin body 356 x 208 x 61 mm and its neck, the bow and the strap.", "Gesamt 0,80 x 0,33 x 0,89 m, Schale und Deckel mit ihren Wänden, der Geigenkorpus 356 x 208 x 61 mm mit seinem Hals, der Bogen und der Gurt."),
        "merged": T("Top, ribs and back are one prism of 61 mm; neck, fingerboard, pegs and scroll are one head 30 mm wide; frog and head of the bow join the stick; fittings, chinrest, bridge and hardware are texture outlines.",
                    "Decke, Zargen und Boden sind ein Prisma von 61 mm; Hals, Griffbrett, Wirbel und Schnecke sind ein Kopf von 30 mm Breite; Frosch und Kopf des Bogens gehen in der Stange auf; Garnitur, Kinnhalter, Steg und Beschläge sind Texturumrisse."),
        "omitted": T("Handle, end block, neck rest, lid lining detail, pad, hooks, every trace of use except the worn patch of the back.", "Griff, Endklotz, Halsstütze, Futterdetails des Deckels, Polster, Karabiner, jede Gebrauchsspur außer der blanken Stelle des Bodens."),
        "thinElements": T("Strings, bow stick, hair and the 3 mm webbing keep their true size and print with the 0.2 and 0.25 mm pens.", "Saiten, Bogenstange, Haar und das 3-mm-Gurtband behalten ihre wahre Größe und drucken mit den Stiften 0,2 und 0,25 mm."),
        "hatching": T("No hatch.", "Keine Schraffur."),
        "styles": T("Body pen 0.25 mm, detail pen 0.2 mm, texture pen 0.1 mm.", "Körperstift 0,25 mm, Detailstift 0,2 mm, Texturstift 0,1 mm."),
        "overlays": T("Every region keeps one shape; merged parts are drawn as texture outlines on their region.", "Jede Region behält eine Form; zusammengefasste Teile werden als Texturumrisse auf ihrer Region gezeichnet."),
    },
}

VOL_DECISIONS = {
    50: [T("Body: every part of the 2D drawing as the same extruded profile: the violin outline (bouts 208 / 112 / 168 mm, 356 mm) as ribs of 31 mm with top and back arching in two steps each (outline scaled 0.90 and 0.62, 15.5 and 14.5 mm), the neck, pegbox and 41 mm scroll as side profiles, the fingerboard as a loft tilted 7.6 degrees from 42 to 24 mm, four peg paddles, the 4.4 mm bridge, tailpiece, fine tuners, tailgut, endpin, chinrest and bracket, and the four strings and afterlengths as slender capsules of their gauges.",
           "Körper: jedes Teil der 2D-Zeichnung als dasselbe extrudierte Profil: der Geigenumriss (Büge 208 / 112 / 168 mm, 356 mm) als Zargen von 31 mm mit je zweistufiger Wölbung von Decke und Boden (Umriss 0,90 und 0,62 skaliert, 15,5 und 14,5 mm), Hals, Wirbelkasten und 41-mm-Schnecke als Seitenprofile, das Griffbrett als um 7,6 Grad geneigter Loft von 42 auf 24 mm, vier Wirbelpaddel, der 4,4-mm-Steg, Saitenhalter, Feinstimmer, Schlinge, Endknopf, Kinnhalter und Bügel und die vier Saiten mit Nachlängen als schlanke Kapseln ihrer Stärken."),
         T("Case: the shell as one ring of 6 mm walls with a 6 mm bottom sheet, the 8 mm velvet bed, compartment, end block and neck rest, handle, latches, rings and hinges; the lid as a ring with a 6 mm top sheet and 8 mm lining, two bow holders screwed through the lining and two catches; the strap as one V-shaped 3 mm webbing sheet with pad and hooks. Traces of use as texture sheets: worn varnish on the back's lower bout and the treble upper bout, rosin dust between bridge and fingerboard, grooves in the fingerboard, four scuffed case corners and a yellowed dealer price label on the lid lining.",
           "Koffer: die Schale als ein Ring aus 6-mm-Wänden mit 6-mm-Bodenplatte, das 8-mm-Samtbett, Fach, Endklotz und Halsstütze, Griff, Verschlüsse, Ringe und Scharniere; der Deckel als Ring mit 6-mm-Deckelplatte und 8-mm-Futter, zwei durch das Futter geschraubten Bogenhaltern und zwei Schließblechen; der Gurt als ein V-förmiges 3-mm-Gurtband mit Polster und Karabinern. Gebrauchsspuren als Texturflächen: blanker Lack am Unterbug des Bodens und am Diskant-Oberbug, Kolophoniumstaub zwischen Steg und Griffbrett, Rillen im Griffbrett, vier abgestoßene Kofferecken und ein vergilbtes Preisetikett des Händlers auf dem Deckelfutter."),
         T("Thermal: every region by reference to its body solids. Structure: case shell and bottom, lid ring and top, violin ribs, back steps and neck, bow stick and strap webbing as members; the bed, hinges, rings, hooks and bow holders as joints; one mass mark per bone at its centre of mass.",
           "Thermik: jede Region über Verweise auf ihre Körper. Tragwerk: Kofferschale und Boden, Deckelring und Platte, Geigenzargen, Bodenstufen und Hals, Bogenstange und Gurtband als Stäbe; Bett, Scharniere, Ringe, Karabiner und Bogenhalter als Gelenke; eine Massenmarke je Knochen im Schwerpunkt.")],
    100: [T("Body for the 15 mm feature: ribs and the lower back step are one prism, the top arch one step (outline 0.80), the back arch one step; neck, pegbox and scroll one head 31 mm wide; the fingerboard a tilted prism 33 mm wide; the four pegs a cross bar; bridge, tailpiece, chinrest; the strings one slender band; the bow stick with fewer sections, frog and head as 15 mm blocks, the hair; the case without hinges and holders, latches, rings and catches as texture sheets; strap webbing and pad as sheets, hooks as texture.",
            "Körper für das Merkmal von 15 mm: Zargen und untere Bodenstufe sind ein Prisma, die Deckenwölbung eine Stufe (Umriss 0,80), die Bodenwölbung eine Stufe; Hals, Wirbelkasten und Schnecke ein Kopf von 31 mm Breite; das Griffbrett ein geneigtes Prisma von 33 mm Breite; die vier Wirbel ein Querriegel; Steg, Saitenhalter, Kinnhalter; die Saiten ein schlankes Band; die Bogenstange mit weniger Schnitten, Frosch und Kopf als 15-mm-Blöcke, das Haar; der Koffer ohne Scharniere und Halter, Verschlüsse, Ringe und Schließbleche als Texturflächen; Gurtband und Polster als Flächen, Karabiner als Textur."),
          T("Thermal by reference; structure as at 1:50 with the hinges replaced by the touching hinge edge and boxes for rings, hooks and bow holders as joints.", "Thermik über Verweise; Tragwerk wie bei 1:50, die Scharniere durch die anliegende Scharnierkante ersetzt und Blöcke für Ringe, Karabiner und Bogenhalter als Gelenke.")],
    200: [T("Body for the 30 mm feature: the violin as one 61 mm prism of its outline and one head 30 mm wide from the heel over the fingerboard to the scroll; fingerboard, chinrest and bridge as texture faces on it; the bow as its slender stick and hair; the case as shell ring, bottom, bed and compartment, lid ring, top and lining; the strap as one webbing sheet; hardware and hooks as texture.",
            "Körper für das Merkmal von 30 mm: die Geige als ein 61-mm-Prisma ihres Umrisses und ein Kopf von 30 mm Breite vom Halsfuß über das Griffbrett bis zur Schnecke; Griffbrett, Kinnhalter und Steg als Texturflächen darauf; der Bogen als seine schlanke Stange und das Haar; der Koffer als Schalenring, Boden, Bett und Fach, Deckelring, Platte und Futter; der Gurt als ein Gurtband; Beschläge und Karabiner als Textur."),
          T("Thermal by reference, every region keeps a solid; structure: shell, lid, violin body and head, bow stick and strap members with boxes for the joints and one mass mark per bone.", "Thermik über Verweise, jede Region behält einen Körper; Tragwerk: Stäbe von Schale, Deckel, Geigenkorpus und Kopf, Bogenstange und Gurt mit Blöcken für die Gelenke und einer Massenmarke je Knochen.")],
}


def representation(s, parts):
    sh = shapes_of(parts)
    lo, hi = RANGES[s]
    return {"status": "authored", "denominator": s, "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": PENS[s], "hatches": [], "geometry": {"elevation": sh, "front": sh, "plan": sh}, "decisions": REP_DECISIONS[s]}


def volume(s, parts):
    return {"status": "authored", "denominator": s, "body": [p.solid() for p in parts], "thermal": thermal_list(parts), "structure": structure_list(s, parts), "decisions": VOL_DECISIONS[s]}


def document():
    p50, p100, p200 = all_parts(50), all_parts(100), all_parts(200)
    lo, hi = bbox(p50)
    dims = {"heightM": round(hi[1] - lo[1], 3), "widthM": round(hi[0] - lo[0], 3), "depthM": round(hi[2] - lo[2], 3), "massKg": round(sum(kg for kg, _ in MASS.values()), 3)}
    unavailable = {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
                   "reason": T("At 1:500 the case is 1.6 x 0.5 mm and the violin 1.2 x 0.4 mm on paper, under the 3 mm a recognisable drawing needs; neck, bow and strap fall below the 0.2 mm minimum feature. Draw it at 1:200 or larger.",
                               "Bei 1:500 ist der Koffer 1,6 x 0,5 mm und die Geige 1,2 x 0,4 mm auf Papier groß, unter den 3 mm, die eine erkennbare Zeichnung braucht; Hals, Bogen und Gurt fallen unter das Mindestmerkmal von 0,2 mm. Bei 1:200 oder größer zeichnen."),
                   "limitingFeature": T("The whole case, 0.80 m long", "Der ganze Koffer, 0,80 m lang"), "limitingSizeM": 0.8,
                   "alternatives": [{"kind": "scale", "denominator": 200}, {"kind": "scale", "denominator": 100}]}
    asset = {
        "id": "used-violin", "category": "objects",
        "title": T("Used Violin with Case", "Gebrauchte Geige mit Koffer"),
        "summary": T("A second-hand 4/4 student violin of 593 mm with its 750 mm brazilwood bow in an oblong plywood case 0.80 x 0.245 x 0.125 m with shoulder strap, as a used-instrument dealer shows it: the case lies open on the floor, the bow in the lid; its varnish is worn through on the back where the shoulder rests and on the treble upper bout, the neck is worn bare, rosin dust lies between bridge and fingerboard, the case corners are scuffed. It opens, is carried on the shoulder strap and is played standing with whole bows on the A string.",
                     "Eine gebrauchte 4/4-Schülergeige von 593 mm mit ihrem 750 mm langen Brasilholzbogen in einem Rechteckkoffer aus Sperrholz von 0,80 x 0,245 x 0,125 m mit Schultergurt, wie ein Gebrauchtinstrumentenhändler sie zeigt: der Koffer liegt offen am Boden, der Bogen im Deckel; ihr Lack ist am Boden, wo die Schulter aufliegt, und am Diskant-Oberbug durchgescheuert, der Hals ist blank gespielt, zwischen Steg und Griffbrett liegt Kolophoniumstaub, die Kofferecken sind abgestoßen. Sie wird geöffnet, am Schultergurt getragen und im Stehen mit ganzen Bögen auf der A-Saite gespielt."),
        "tags": ["used", "instrument", "music", "second-hand", "strings", "violin", "bowed", "violin-case", "spruce", "maple", "ebony", "plywood", "object"],
        "provenance": {"author": "Semio staffage content (used-instruments wave, row 04)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES,
                       "notes": T("Type reference: a 4/4 workshop violin of the classical Stradivari pattern as student and orchestra instruments were built in the Vogtland and Markneukirchen workshops of both German states (no maker, no label text), body 356 mm, string length 328 mm, with a brazilwood bow and an oblong plywood case. Dimensions, masses and loads are authored estimates and literature values from the cited sources; nothing is measured. The violin is modelled as its outer envelope (its air volume is closed by f-holes narrower than the scale can show), so its mass comes from the plate model of the reference table, not from envelope volume times density.",
                                  "Typreferenz: eine 4/4-Werkstattgeige nach dem klassischen Stradivari-Modell, wie Schüler- und Orchesterinstrumente in den Werkstätten des Vogtlands und von Markneukirchen in beiden deutschen Staaten gebaut wurden (ohne Hersteller, ohne Zetteltext), Korpus 356 mm, Mensur 328 mm, mit einem Brasilholzbogen und einem Rechteckkoffer aus Sperrholz. Maße, Massen und Lasten sind Schätzungen und Literaturwerte der genannten Quellen; nichts ist gemessen. Die Geige ist als ihre äußere Hülle modelliert (ihr Luftraum ist durch f-Löcher geschlossen, die schmaler sind, als der Maßstab zeigen kann), ihre Masse folgt daher dem Plattenmodell der Bezugstabelle, nicht dem Hüllvolumen mal Dichte.")},
        "dimensions": dims,
        "materials": MATERIALS,
        "rig": rig(),
        "anchors": anchors(),
        "representations": {"50": representation(50, p50), "100": representation(100, p100), "200": representation(200, p200), "500": unavailable},
        "symbols": [],
        "poses": poses(),
        "clips": clips(),
        "thermal": {
            "environment": {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
            "passengers": [], "operating": [], "regions": regions(), "emitters": [],
            "assumptions": [T("An acoustic instrument in a heated shop: every surface is passive, the violin, the bow and the case draw no power (passive, 0 W). In the closed case the violin and the bow exchange heat only with the trapped air (20 percent of their surface counted as exposed).",
                              "Ein akustisches Instrument in einem beheizten Laden: jede Fläche ist passiv, Geige, Bogen und Koffer nehmen keine Leistung auf (passiv, 0 W). Im geschlossenen Koffer tauschen Geige und Bogen nur mit der eingeschlossenen Luft Wärme (20 Prozent ihrer Fläche als ausgesetzt gerechnet)."),
                            T("Played, the chin (34 degrees C) on 40 percent of the chinrest, the left hand (33 degrees C) on 30 percent of the neck, the collarbone through the shirt on the worn patch of the back and the right hand on 8 percent of the bow warm the wood; there is no breath stream through a string instrument.",
                              "Beim Spiel erwärmen das Kinn (34 Grad C) auf 40 Prozent des Kinnhalters, die linke Hand (33 Grad C) auf 30 Prozent des Halses, das Schlüsselbein durch das Hemd auf der blanken Stelle des Bodens und die rechte Hand auf 8 Prozent des Bogens das Holz; durch ein Streichinstrument geht kein Atemstrom."),
                            T("Aged surfaces: bare maple worn by hand and shoulder absorbs less (0.5) and emits slightly more (0.92) than the varnished wood (0.62, 0.9); tarnished steel strings and hardware emit 0.25 to 0.3 instead of about 0.1 when new (authored estimates after Incropera).",
                              "Gealterte Flächen: von Hand und Schulter blank gescheuerter Ahorn absorbiert weniger (0,5) und strahlt etwas mehr ab (0,92) als lackiertes Holz (0,62; 0,9); angelaufene Stahlsaiten und Beschläge strahlen 0,25 bis 0,3 statt etwa 0,1 im Neuzustand ab (Schätzungen nach Incropera).")],
        },
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [],
                      "assumptions": [T("Stored and closed, the 3.43 kg rest with the shell bottom on the floor (one contact of 0.78 x 0.235 m); carried, the strap pad on the shoulder is the only support (pin) and the case hangs rolled so that its centre of mass lies below it.",
                                        "Gelagert und geschlossen ruhen die 3,43 kg mit dem Schalenboden auf dem Boden (ein Kontakt von 0,78 x 0,235 m); getragen ist das Gurtpolster auf der Schulter das einzige Auflager (Gelenk), und der Koffer hängt so gerollt, dass sein Schwerpunkt darunter liegt."),
                                      T("The four strings pull 235 N on the nut and through the tailpiece on the saddle and press about 95 N down on the bridge; these self-equilibrated loads load the violin's neck and body in every pose. Played, the 0.47 kg violin rests on the collarbone (contact) and in the left hand (pin), the chin presses 15 N, the bow of 61 g is held by the right hand (pin) and presses 0.7 to 1.0 N on the A string.",
                                        "Die vier Saiten ziehen mit 235 N am Sattel und über den Saitenhalter am Untersattel und drücken etwa 95 N auf den Steg; diese in sich ausgeglichenen Lasten beanspruchen Hals und Korpus der Geige in jeder Haltung. Beim Spiel liegt die 0,47 kg schwere Geige auf dem Schlüsselbein (Kontakt) und in der linken Hand (Gelenk), das Kinn drückt mit 15 N, der 61 g schwere Bogen wird von der rechten Hand gehalten (Gelenk) und drückt mit 0,7 bis 1,0 N auf die A-Saite."),
                                      T("The solver takes the asset as one rigid body: in the played pose the split of the weights between the floor under the open case and the hands follows its equal-stiffness assumption, not the separation of case, violin and bow.",
                                        "Der Löser behandelt das Objekt als einen starren Körper: in der Spielhaltung folgt die Aufteilung der Gewichte zwischen dem Boden unter dem offenen Koffer und den Händen seiner Annahme gleicher Steifigkeit, nicht der Trennung von Koffer, Geige und Bogen.")],
                      "clinical": False},
        "volumes": {"50": volume(50, p50), "100": volume(100, p100), "200": volume(200, p200), "500": {"status": "unavailable", "denominator": 500}},
        "presentation": {"defaultPose": "stored", "defaultClip": "bowing", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}


if __name__ == "__main__":
    doc = document()
    (HERE / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n")
    print(f"[DEBUG] wrote {HERE / '🔣️.json'} dims {doc['asset']['dimensions']}")
