# [scratch] NW-A1 generic quadruped rig builder (ground-up leg chains) and gait/pose helpers
import math
from lib import *

SIDES = (("hn", "near", "nah", "Near", "Nahes"), ("hf", "far", "fern", "Far", "Fernes"))


def up(p, ang, L):
    return (p[0] - L * math.cos(ang * D), p[1] - L * math.sin(ang * D))


def build_rig(P):
    sk = Skel()
    foot = P["foot"]
    hl, fl = P["hind"], P["fore"]
    q = (P["hind_sole_x"], 0.0)
    hpts = [q]
    for L, a in reversed(hl):
        q = up(q, a, L)
        hpts.append(q)
    hip = hpts[-1]
    q = (P["fore_sole_x"], 0.0)
    for L, a in reversed(fl):
        q = up(q, a, L)
    withers_top = q
    sy = withers_top[1]
    sk.hip, sk.spine_y = hip, sy
    sacrum = (hip[0] + P["sacrum_dx"], sy)
    lim = P["lim"]
    T_ = P["titles"]
    sk.add("pelvis", None, hip, sacrum, lim["pelvis"], T_["pelvis"])
    lumbar_end = (P["thorax_x"], sy)
    sk.add("lumbar", "pelvis", sacrum, lumbar_end, lim["lumbar"], T_["lumbar"])
    thorax_end = (P["neck_x"], sy)
    sk.add("thorax", "lumbar", lumbar_end, thorax_end, lim["thorax"], T_["thorax"])
    L, a = P["neck"]
    neck_end = (thorax_end[0] + L * math.cos(a * D), thorax_end[1] + L * math.sin(a * D))
    sk.add("neck", "thorax", thorax_end, neck_end, lim["neck"], T_["neck"])
    L, a = P["head"]
    head_end = (neck_end[0] + L * math.cos(a * D), neck_end[1] + L * math.sin(a * D))
    sk.add("head", "neck", neck_end, head_end, lim["head"], T_["head"])
    jp, jl, ja = P["jaw"]
    ho = sk.w0["head"]
    j0 = add(ho, R(jp, sk.aw["head"]))
    j1 = add(j0, R((jl, 0), ja))
    sk.add("jaw", "head", j0, j1, lim["jaw"], T_["jaw"])
    for sfx, key in (("n", "ear-n"), ("f", "ear-f")):
        ep, el, ea = P["ear"]
        e0 = add(ho, R(ep, sk.aw["head"]))
        e1 = add(e0, R((el, 0), ea))
        sk.add("ear-" + sfx, "head", e0, e1, lim["ear"], T_["ear-" + sfx])
    tail_root = add(hip, P["tail_root"])
    prev, pe = "pelvis", tail_root
    for i, (L, a) in enumerate(P["tail"], 1):
        e = (pe[0] + L * math.cos(a * D), pe[1] + L * math.sin(a * D))
        sk.add("tail-%d" % i, prev if i == 1 else "tail-%d" % (i - 1), pe, e, lim["tail"], T_["tail-%d" % i])
        prev, pe = "tail-%d" % i, e
    names_h = ["thigh", "shank", "metatarsus", foot]
    names_f = ["scapula", "humerus", "forearm", "metacarpus", foot]
    for sfx, side, sd, Side, Sd in SIDES:
        parent = "pelvis"
        p0 = hip
        for (L, a), nm, ti in zip(hl, names_h, T_["hind"]):
            p1 = (p0[0] + L * math.cos(a * D), p0[1] + L * math.sin(a * D))
            idn = "%s-%s" % (nm, sfx)
            sk.add(idn, parent, p0, p1, lim[nm if nm != foot else "foot"], (Side + " " + ti[0], Sd + " " + ti[1]) if False else T("%s %s" % (Side, ti[0]), "%s %s" % (Sd, ti[1])))
            parent, p0 = idn, p1
    fidx = {"hn": "fn", "hf": "ff"}
    for sfx, side, sd, Side, Sd in SIDES:
        f = fidx[sfx]
        parent = "thorax"
        p0 = withers_top
        for (L, a), nm, ti in zip(fl, names_f, T_["fore"]):
            p1 = (p0[0] + L * math.cos(a * D), p0[1] + L * math.sin(a * D))
            idn = "%s-%s" % (nm, f)
            sk.add(idn, parent, p0, p1, lim[nm if nm != foot else "foot"], T("%s %s" % (Side, ti[0]), "%s %s" % (Sd, ti[1])))
            parent, p0 = idn, p1
    return sk


def set_masses(sk, kg_total, shares, coms, refid):
    """shares: bone -> fraction ; coms: bone -> (comX, comY, dist). Normalised to kg_total (rounded to 3 digits, remainder on pelvis? no: on thorax)."""
    tot = sum(shares.values())
    kgs = {b: round(kg_total * s / tot, 3) for b, s in shares.items()}
    diff = round(kg_total - sum(kgs.values()), 3)
    kgs["thorax"] = round(kgs["thorax"] + diff, 3)
    for b in sk.bones:
        kg = kgs.get(b["id"], 0.0)
        cx, cy, dist = coms.get(b["id"], (b["lengthM"] / 2, 0.0, "uniform"))
        b["_kg"] = kg
        b["_com"] = (cx, cy)
        b["mass"] = {"kg": kg, "comX": r4(cx), "comY": r4(cy), "distribution": dist, "source": "authored-estimate", "reference": refid}
    return kgs


def finish_bones(sk):
    out = []
    for b in sk.bones:
        c = {k: v for k, v in b.items() if not k.startswith("_")}
        out.append(c)
    return out


def sample_foot_traj(P, u, stance):
    pass
