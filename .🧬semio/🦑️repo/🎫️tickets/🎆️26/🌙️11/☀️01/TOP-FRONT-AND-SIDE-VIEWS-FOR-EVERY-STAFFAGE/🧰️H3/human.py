"""The shared biped builder: front and plan of the anatomical human rig at four detail tiers (0 = 1:500 ... 3 = 1:50)."""
import math
from humanlib import *

FIST = {0: (.1, .1), 1: (.055, .05), 2: (0, 0), 3: (0, 0)}
SIDES = (("l", 1), ("r", -1))


def wbox(R, st, scale, bone, ids):
    x0, y0, x1, y1 = st.bbox(scale, ids)
    cs = [R.p(bone, x, y) for x in (x0, x1) for y in (y0, y1)]
    return min(c[0] for c in cs), min(c[1] for c in cs), max(c[0] for c in cs), max(c[1] for c in cs)


def arc(cx, cy, rx, ry, a0, a1, n):
    return [(cx + rx * math.cos(math.radians(a0 + (a1 - a0) * i / n)), cy + ry * math.sin(math.radians(a0 + (a1 - a0) * i / n))) for i in range(n + 1)]


class Human:
    """P holds the numbers of one person, KEY the elevation shape ids that give tone and region."""
    def __init__(h, asset, P, KEY, bones=None):
        h.a = asset; h.R = Rest(asset); h.st = Styles(asset); h.P = P; h.KEY = KEY
        h.bones = bones or {}

    def S(h, sc, key, side="l", **kw):
        ids = h.KEY[key]
        if isinstance(ids, dict): return dict(ids)
        ids = [i.replace("{s}", side) for i in (ids if isinstance(ids, list) else [ids])]
        return h.st.get(sc, ids, **kw)

    def head_box(h, sc):
        R = h.R; b = h.bones.get("head", "head")
        skull = wbox(R, h.st, sc, b, h.KEY["skull"])
        hair = wbox(R, h.st, sc, b, h.KEY["hair"])
        return skull, hair

    # ------------------------------------------------------------------ front
    def front(h, sc, o, skip=()):
        R, P, t = h.R, h.P, tier(sc)
        B = h.bones
        K = lambda n: B.get(n, n)
        hip_y = R.o(K("thigh-l"))[1]
        for sd, s in SIDES:
            lat = s * P["hl"]
            if f"leg-{sd}" not in skip:
                th, sk, ft, tb = K("thigh-" + sd), K("shank-" + sd), K("foot-" + sd), K("toe-" + sd)
                knee_y = R.t(th)[1]; ank_y = R.t(sk)[1]; tip_y = R.t(ft)[1]
                flat = abs(ank_y - tip_y) < 0.03
                if flat: tip_y = ank_y - P.get("drop", 0.09)
                limbsol(h, o, f"thigh-{sd}", th, P["t0"], P["t1"], lat, 1.0, h.S(sc, "thigh", sd))
                ell(o, f"knee-{sd}", sk, lat, knee_y, P["t1"] * 1.02, P["t1"] * 0.95, 1.03, h.S(sc, "thigh", sd)) if t >= 1 else None
                low = f"lower-{sd}" not in skip
                fw = P["fw"]
                if low: limbsol(h, o, f"shank-{sd}", sk, P["s0"], P["s1"], lat, 1.02, h.S(sc, "shank", sd))
                if low and flat:
                    fl_ = R.b[ft]["lengthM"]; dr_ = P.get("drop", 0.09)
                    poly(o, f"shoe-{sd}", ft, [(-0.05, -dr_), (-0.05, 0.03), (fl_ * 0.45, 0.03), (fl_, -dr_ * 0.5), (fl_, -dr_)], 1.1, h.S(sc, "shoe", sd)); sol(o, lat - fw, lat + fw)
                elif low: poly(o, f"shoe-{sd}", ft, [(lat - P["s1"] * 1.08, ank_y), (lat - fw, ank_y - (ank_y - tip_y) * 0.55), (lat - fw * 0.96, tip_y + 0.006), (lat + fw * 0.96, tip_y + 0.006), (lat + fw, ank_y - (ank_y - tip_y) * 0.55), (lat + P["s1"] * 1.08, ank_y)], 1.1, h.S(sc, "shoe", sd), follow=not flat)
                tb0 = tip_y if flat else R.o(tb)[1]
                if low and t >= 2 and "toe" not in skip:
                    ell(o, f"shoe-toe-{sd}", tb, lat, tb0 + 0.02, fw * 0.98, 0.021, 1.12, h.S(sc, "toe", sd))
                if low and t >= 3:
                    line(o, f"shoe-sole-{sd}", tb if "toe" not in skip else ft, [(lat - fw * 0.96, tb0 + 0.004), (lat + fw * 0.96, tb0 + 0.004)], 1.15, 0.9)
                    if "hem-leg" in h.KEY: line(o, f"trouser-hem-{sd}", sk, [(lat - P["s1"] * 1.1, ank_y + 0.045), (lat + P["s1"] * 1.1, ank_y + 0.045)], 1.05, 0.3)
        if "hips" not in skip:
            pv = K("pelvis"); y0 = R.o(pv)[1]; y1 = R.t(pv)[1]; hw_ = P["hl"] + P["t0"] * 0.98
            poly(o, "hips", pv, [(-hw_, y0 - 0.01), (-hw_ * 1.0, y1 - 0.01), (-P["ww"], y1 + 0.005), (P["ww"], y1 + 0.005), (hw_ * 1.0, y1 - 0.01), (hw_, y0 - 0.01)], 3.0, h.S(sc, "hips" if "hips" in h.KEY else "pelvis"), follow=True)
        if "torso" not in skip:
            tr = K("torso"); y0 = R.o(tr)[1]; y1 = R.t(tr)[1]; span = y1 - y0
            sh = y0 + span * 0.9
            poly(o, "torso", tr, [(-P["ww"], y0), (-P["cw"], y0 + span * 0.5), (-P["sw"] * 0.97, sh - 0.03), (-P["sw"] * 0.78, sh), (-P["nw"] * 1.5, y1 - 0.004), (P["nw"] * 1.5, y1 - 0.004), (P["sw"] * 0.78, sh), (P["sw"] * 0.97, sh - 0.03), (P["cw"], y0 + span * 0.5), (P["ww"], y0)], 4.0, h.S(sc, "torso"), follow=True)
            if t >= 3 and "hem" in h.KEY:
                line(o, "torso-hem", tr, [(-P["ww"] * 1.0, y0 + 0.03), (P["ww"] * 1.0, y0 + 0.03)], 4.1, 0.3)
        if "neck" not in skip and t >= 1:
            nk = K("neck"); y0 = R.o(nk)[1]; y1 = R.t(nk)[1]
            poly(o, "neck", nk, limb(0.0, y0 - 0.012, y1 + 0.01, P["nw"], P["nw"] * 0.95), 3.8, h.S(sc, "neck"), follow=True)
        if "head" not in skip:
            hb = K("head"); skull, hair = h.head_box(sc)
            cy = (skull[1] + skull[3]) / 2; ry = (skull[3] - skull[1]) / 2; hx = o_hw = P["hw"]
            if t == 0:
                sk_ = h.st.bbox(sc, h.KEY["skull"]); hr_ = h.st.bbox(sc, h.KEY["hair"])
                ellsol(o, "head", hb, sk_[0], max(sk_[2], hr_[2]), sk_[1], sk_[3], hx, 5.0, {"tone": .4, "regions": regs(h.S(sc, "skin"), h.S(sc, "hair"))})
            else:
                ell(o, "head", hb, 0.0, cy, hx, ry, 5.0, h.S(sc, "skin"))
            top = hair[3]; hcy = cy + (top - (cy + ry)) / 2 + 0.002
            cap_pts = arc(0.0, cy, hx * 1.06, top - cy, 188, -8, 10 if t >= 2 else 6)
            fringe = [(hx * 1.0, cy + ry * 0.38), (hx * 0.45, cy + ry * 0.52), (0.0, cy + ry * 0.42), (-hx * 0.5, cy + ry * 0.55), (-hx * 1.0, cy + ry * 0.38)]
            if t > 0: poly(o, "hair", hb, cap_pts + fringe[:0] + [(hx * 1.0, cy + ry * 0.35), (hx * 0.5, cy + ry * 0.55), (0.0, cy + ry * 0.45), (-hx * 0.5, cy + ry * 0.55), (-hx * 1.0, cy + ry * 0.35)], 5.5, h.S(sc, "hair"))
            if t >= 2:
                for sd, s in SIDES:
                    ell(o, f"ear-{sd}", hb, s * (hx + 0.004), cy - ry * 0.05, 0.011, 0.027, 5.2, h.S(sc, "skin"))
            if t >= 3:
                for sd, s in SIDES:
                    ell(o, f"eye-{sd}", hb, s * hx * 0.42, cy + ry * 0.12, 0.0075, 0.0045, 5.6, h.S(sc, "skin", tone=0.9), role="detail")
                    line(o, f"brow-{sd}", hb, [(s * hx * 0.2, cy + ry * 0.27), (s * hx * 0.62, cy + ry * 0.3)], 5.6, 0.2)
                line(o, "nose", hb, [(0.0, cy + ry * 0.05), (0.0, cy - ry * 0.28)], 5.6, 0.25)
                line(o, "mouth", hb, [(-hx * 0.3, cy - ry * 0.52), (hx * 0.3, cy - ry * 0.52)], 5.6, 0.2)
        for sd, s in SIDES:
            if f"arm-{sd}" in skip: continue
            arm, fa, hd = K("arm-" + sd), K("forearm-" + sd), K("hand-" + sd)
            lat = s * P["ac"]
            sh_y = R.o(arm)[1]; el_y = R.t(arm)[1]; wr_y = R.t(fa)[1]; tip_y = R.t(hd)[1]
            if t >= 1: ell(o, f"shoulder-{sd}", arm, lat, sh_y, P["a0"], P["a0"], 4.62, h.S(sc, "sleeve", sd))
            limbsol(h, o, f"upper-arm-{sd}", arm, P["a0"], P["a1"], lat, 4.6, h.S(sc, "sleeve", sd))
            if t >= 1: ell(o, f"elbow-{sd}", fa, lat, el_y, max(P["a1"], min(FIST[t][0], P["a0"])), max(P["a1"], min(FIST[t][0], P["a0"])), 4.65, h.S(sc, "sleeve", sd))
            limbsol(h, o, f"forearm-{sd}", fa, P["f0"], P["f1"], lat, 4.64, h.S(sc, "forearm", sd))
            limbsol(h, o, f"hand-{sd}", hd, P["hr"], P["hr"] * 0.82, lat, 4.7, h.S(sc, "hand", sd))
            ell(o, f"fist-{sd}", hd, lat, wr_y, max(P["hr"] * 1.02, min(FIST[t][1], P["a0"])), max(P["hr"], min(FIST[t][1], P["a0"])), 4.72, h.S(sc, "hand", sd))
            if t >= 3 and "cuff" in h.KEY:
                line(o, f"cuff-{sd}", fa, [(lat - P["f1"], wr_y + 0.02), (lat + P["f1"], wr_y + 0.02)], 4.66, 0.3)
        return o

    # ------------------------------------------------------------------ plan
    def plan(h, sc, o, skip=()):
        R, P, t = h.R, h.P, tier(sc)
        B = h.bones
        K = lambda n: B.get(n, n)
        for sd, s in SIDES:
            lat = s * P["hl"]
            if f"leg-{sd}" not in skip:
                th, sk, ft, tb = K("thigh-" + sd), K("shank-" + sd), K("foot-" + sd), K("toe-" + sd)
                low = f"lower-{sd}" not in skip
                fx0 = R.o(ft)[0]; fx1 = R.t(ft)[0]; tx1 = wbox(R, h.st, sc, tb, h.KEY["shoe-toe"].replace("{s}", sd))[2] if (low and "shoe-toe" in h.KEY) else R.t(tb)[0] + 0.02
                fw = P["fw"]
                nt = "toe" in skip
                if low: cap(o, f"shoe-heel-{sd}", ft, fx0 - 0.025, lat, fx1 - (fw * 0.8 - 0.012 if nt else (fw + 0.02 if sc == "500" else 0)), lat, fw * 0.85, fw * (0.8 if nt else 1), 1.2, h.S(sc, "shoe", sd), follow=True)
                if low and not nt: cap(o, f"shoe-toe-{sd}", tb, min(fx1, tx1 - fw * 0.8), lat, tx1 - fw * 0.8, lat, fw, fw * 0.8, 1.2, h.S(sc, "toe", sd), follow=True)
                if low and t >= 3: line(o, f"shoe-laces-{sd}", ft, [(fx0 + 0.04, lat - fw * 0.5), (fx1 - 0.03, lat - fw * 0.45)], 1.4, 0.6)
                if low: cap(o, f"shank-{sd}", sk, R.o(sk)[0], lat, R.t(sk)[0], lat, P["s0"], P["s1"], 1.1, h.S(sc, "shank", sd), follow=True)
                cap(o, f"thigh-{sd}", th, R.o(th)[0], lat, R.t(th)[0], lat, P["t0"], P["t1"], 1.05, h.S(sc, "thigh", sd), follow=True)
        for sd, s in SIDES:
            if f"arm-{sd}" in skip: continue
            arm, fa, hd = K("arm-" + sd), K("forearm-" + sd), K("hand-" + sd)
            lat = s * P["ac"]
            cap(o, f"upper-arm-{sd}", arm, R.o(arm)[0], lat, R.t(arm)[0], lat, P["a0"], P["a1"], 3.0, h.S(sc, "sleeve", sd), follow=True)
            cap(o, f"forearm-{sd}", fa, R.o(fa)[0], lat, R.t(fa)[0], lat, P["f0"], P["f1"], 3.05, h.S(sc, "forearm", sd), follow=True)
            cap(o, f"hand-{sd}", hd, R.o(hd)[0] - max(P["hr"] - 0.03, 0.0), lat, R.t(hd)[0] + 0.012 - P["hr"] * 0.8, lat, P["hr"], P["hr"] * 0.8, 3.1, h.S(sc, "hand", sd), follow=True)
        pv, tr, nk = K("pelvis"), K("torso"), K("neck")
        if "torso" not in skip:
            ell(o, "waist", pv, R.o(pv)[0] + P.get("waist_dx", 0.0), 0.0, P["pd"], P["ww"] * 1.02, 3.9, h.S(sc, "pelvis"))
            for sd, s in SIDES:
                cap(o, f"trunk-{sd}", tr, R.o(tr)[0], s * P["tl"], R.t(tr)[0], s * P["tl"], P["cd"], P["cd"], 3.95, h.S(sc, "torso"), follow=True)
            ell(o, "shoulders", nk, R.o(nk)[0] + P.get("shoulder_dx", 0.0), 0.0, P["sd"], P["sw"], 4.0, h.S(sc, "torso"))
            if t >= 3:
                for sd, s in SIDES:
                    line(o, f"shoulder-seam-{sd}", nk, [(R.o(nk)[0] + 0.05, s * P["sw"] * 0.4), (R.o(nk)[0] - 0.005, s * P["sw"] * 0.82)], 4.2, 0.55)
        if "head" not in skip:
            hb = K("head"); skull, hair = h.head_box(sc)
            if t >= 1: ell(o, "neck", nk, R.o(nk)[0] + 0.005, 0.0, P["nw"] * 1.25, P["nw"] * 1.3, 5.0, h.S(sc, "neck"))
            xf = skull[2]; xb = hair[0]
            hxc = (xf + xb) / 2; hrx = (xf - xb) / 2
            ell(o, "head", hb, hxc, 0.0, hrx, P["hw"], 6.0, h.S(sc, "hair"))
            if t >= 1: ell(o, "face", hb, xf - hrx * 0.3, 0.0, hrx * 0.3, P["hw"] * 0.7, 6.1, h.S(sc, "skin"))
            if t >= 3:
                cap(o, "nose", hb, xf - 0.004, 0.0, xf + 0.014, 0.0, 0.011, 0.008, 6.3, h.S(sc, "skin"), role="detail")
                for sd, s in SIDES:
                    ell(o, f"ear-{sd}", hb, hxc - hrx * 0.1, s * (P["hw"] + 0.002), 0.012, 0.007, 6.2, h.S(sc, "skin"), role="detail")
                line(o, "hair-parting", hb, [(hxc - hrx * 0.5, -0.03), (hxc - hrx * 0.1, 0.012), (hxc + hrx * 0.1, 0.03)], 6.3, 0.5)
        return o


def coarse(P, sc):
    """Coarse scales need thicker limbs (print minimum feature); the outer arm edge and the lateral extent stay where they are."""
    if sc not in ("500", "200"): return P
    P = dict(P, **P.get("p500", {})) if sc == "500" else dict(P)
    fl = .08 if sc == "500" else .04
    fl5 = P.get("fl5", (.08, .09, .09))
    q = dict(P)
    for k in ("t0", "t1", "s0", "s1", "fw", "a0", "a1", "f0", "f1", "hr", "nw"): q[k] = max(P[k], (fl5[2] if k == "fw" else fl5[1] if k in ("a0", "a1", "f0", "f1", "hr") else fl5[0]) if sc == "500" else fl)
    q["ac"] = P["ac"] + P["a0"] - q["a0"]
    return q


def over(h, sc, of, op, length, key="thigh-over"):
    """A garment on the upper thigh (shorts): front follows the thigh, plan sits on the hip."""
    R, P = h.R, h.P
    for sd, s in SIDES:
        th = h.bones.get("thigh-" + sd, "thigh-" + sd)
        y0 = R.o(th)[1]; lat = s * P["hl"]
        poly(of, f"shorts-{sd}", th, limb(lat, y0, y0 - length, P["t0"] + 0.008, P["t0"] * 0.92 + 0.008), 1.03, h.S(sc, key, sd), follow=True)
        ell(op, f"shorts-{sd}", th, R.o(th)[0], lat, P["t0"] * 1.06, P["t0"] * 1.06, 1.06, h.S(sc, key, sd))


def limbsol(h, o, id, bone, ra, rb, lat, z, style, zr=None):
    """A limb segment as an extruded solid: the tapered outline in the bone frame, swept across the limb thickness, so every pose projects it exactly (an arm pointing at the viewer shows its end-on width)."""
    L = h.R.b[bone]["lengthM"]
    poly(o, id, bone, [(0.0, -ra), (L, -rb), (L, rb), (0.0, ra)], z, style)
    r = zr if zr is not None else (ra + rb) / 2
    sol(o, lat - r, lat + r)


def ellsol(o, id, bone, x0, x1, y0, y1, half, z, style):
    """An ellipse given by its bone-local box, swept laterally to +-half: a solid with its depth built in."""
    cx, cy, rx, ry = (x0 + x1) / 2, (y0 + y1) / 2, (x1 - x0) / 2, (y1 - y0) / 2
    poly(o, id, bone, [(cx + rx * math.cos(a * math.pi / 12), cy + ry * math.sin(a * math.pi / 12)) for a in range(24)], z, style)
    sol(o, -half, half)


def regs(*styles):
    out = []
    for d in styles:
        for r in ([d["region"]] if "region" in d else d.get("regions", [])):
            if r not in out: out.append(r)
    return out
