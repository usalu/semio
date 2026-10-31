"""NW-H1: parametric human rig (anatomical, 2-D elevation), masses (de Leva 1996), standard anchors, thermal/respiration helpers, pose/clip plumbing."""
import math, copy
from nwlib import *

DELEVA = {
    # head+neck, pelvis (lower trunk), trunk (mid+upper), upper arm, forearm, hand, thigh, shank, foot  (fractions of body mass, de Leva 1996 Tables 4/5)
    "m": dict(head=0.0694, pelvis=0.1117, torso=0.3229, arm=0.0271, forearm=0.0162, hand=0.0061, thigh=0.1416, shank=0.0433, foot=0.0137),
    "f": dict(head=0.0668, pelvis=0.1247, torso=0.3010, arm=0.0255, forearm=0.0138, hand=0.0056, thigh=0.1478, shank=0.0481, foot=0.0129),
    # child: authored estimate (larger head, shorter limbs) guided by adult de Leva fractions
    "c": dict(head=0.0900, pelvis=0.1200, torso=0.3200, arm=0.0250, forearm=0.0150, hand=0.0060, thigh=0.1300, shank=0.0440, foot=0.0150),
}
COM = dict(head=0.5, pelvis=0.5, torso=0.45, arm=0.5772, forearm=0.4574, hand=0.79, thigh=0.4095, shank=0.4459, foot=0.44)


class Human:
    def __init__(self, H, kg, sex, ankle=None, foot_k=1.0, head_k=1.0, limb_k=1.0, torso_k=1.0, lim_over=None):
        self.lim_over = lim_over or {}
        self.H, self.kg, self.sex = H, kg, sex
        self.ah = ankle if ankle is not None else 0.039 * H + 0.012
        self.Lth = 0.245 * H * limb_k
        self.Lsh = 0.246 * H * limb_k
        self.hh = self.Lth + self.Lsh + self.ah
        self.Lp = 0.050 * H
        self.Lt = 0.255 * H * torso_k
        self.Ln = 0.047 * H
        self.Lh = 0.118 * H * head_k
        self.hd = 0.13 * H * head_k
        self.Lua = 0.186 * H * limb_k
        self.Lfa = 0.146 * H * limb_k
        self.Lha = 0.108 * H * limb_k
        self.xs = 0.93 * self.Lt
        self.Lball = 0.082 * H * foot_k
        self.theta = math.degrees(math.atan2(self.ah, self.Lball))
        self.Lf = math.hypot(self.Lball, self.ah)
        self.Ltoe = 0.036 * H * foot_k
        self.heel_back = 0.05 * H * foot_k
        self.extra_bones = []
        self.extra_anchors = []

    # ---- rig ----
    def bones(self):
        H = self
        lim = dict(
            pelvis=(-15, 15), torso=(-70, 30), neck=(-35, 40), head=(-35, 35), arm=(-70, 175), forearm=(0, 150), hand=(-70, 70),
            thigh=(-35, 120), shank=(-150, 5), foot=(-65, 42), toe=(-5, 65))
        lim.update(self.lim_over)
        m = DELEVA[self.sex]
        kg = self.kg
        src = "literature"
        ref = "de-leva-1996"

        def mass(seg, length, comk=None, srcx=src):
            k = COM[seg] if comk is None else comk
            return {"kg": round(kg * m[seg], 3), "comX": r4(k * length), "comY": 0, "distribution": "point", "source": srcx, "reference": ref}

        def revolute(seg):
            return {"type": "revolute", "minDeg": lim[seg][0], "maxDeg": lim[seg][1]}
        L = []

        def bone(id_, parent, en, de, x, y, ang, length, joint, ms=None):
            b = {"id": id_, "parent": parent, "title": tx(en, de), "x": r4(x), "y": r4(y), "angleDeg": r4(ang), "lengthM": r4(length), "joint": joint}
            if ms: b["mass"] = ms
            L.append(b)
        bone("pelvis", None, "Pelvis", "Becken", 0, self.hh, 90, self.Lp, revolute("pelvis"), mass("pelvis", self.Lp))
        bone("torso", "pelvis", "Torso", "Rumpf", self.Lp, 0, 0, self.Lt, revolute("torso"), mass("torso", self.Lt))
        bone("neck", "torso", "Neck", "Hals", self.Lt, 0, 0, self.Ln, revolute("neck"))
        bone("head", "neck", "Head", "Kopf", self.Ln, 0, 0, self.Lh, revolute("head"), mass("head", self.Lh))
        for s, (en, de) in (("l", ("Left", "Linker")), ("r", ("Right", "Rechter"))):
            bone("arm-" + s, "torso", en + " upper arm", de + " Oberarm", self.xs, 0, -180, self.Lua, revolute("arm"), mass("arm", self.Lua))
            bone("forearm-" + s, "arm-" + s, en + " forearm", de + " Unterarm", self.Lua, 0, 0, self.Lfa, revolute("forearm"), mass("forearm", self.Lfa))
            bone("hand-" + s, "forearm-" + s, en + " hand", de + " Hand", self.Lfa, 0, 0, self.Lha, revolute("hand"), mass("hand", self.Lha))
            bone("thigh-" + s, "pelvis", en + " thigh", de + " Oberschenkel", 0, 0, 180, self.Lth, revolute("thigh"), mass("thigh", self.Lth))
            bone("shank-" + s, "thigh-" + s, en + " shank", de + " Unterschenkel", self.Lth, 0, 0, self.Lsh, revolute("shank"), mass("shank", self.Lsh))
            fm = mass("foot", self.Lf)
            bone("foot-" + s, "shank-" + s, en + " foot and shoe", de + " Fuß mit Schuh", self.Lsh, 0, 90 - self.theta, self.Lf, revolute("foot"), fm)
            bone("toe-" + s, "foot-" + s, en + " forefoot", de + " Vorfuß", self.Lf, 0, self.theta, self.Ltoe, revolute("toe"))
        return L + self.extra_bones

    def build_rig(self):
        self.rig = Rig(self.bones(), "pelvis")
        self.rest = self.rig.rest()
        return self.rig

    # ---- rest landmarks (world, rest pose) ----
    def P(self, bone, lx, ly):
        return self.rest[bone].ap(lx, ly)

    def head_pt(self, u, v):
        """normalised head coordinates (x forward, y up, unit = head height) with origin at the head-bone origin -> rest world"""
        o = self.rest["head"]
        return (o.x + u * self.hd, o.y + v * self.hd)

    def head_world(self, u, v, pose_angle=0):
        return self.head_pt(u, v)

    def anchors(self):
        out = []
        hd = self.heel_back
        for s, nm in (("l", ("Left", "Linker")), ("r", ("Right", "Rechter"))):
            fa = self.rest["foot-" + s]
            # heel contact point world (ankle.x - heel_back, 0), ball (ankle.x + Lball, 0)
            ank = (fa.x, fa.y)
            heel = fa.inv(ank[0] - hd, 0.0)
            ball = fa.inv(ank[0] + self.Lball, 0.0)
            out.append({"id": "heel-" + s, "bone": "foot-" + s, "x": r4(heel[0]), "y": r4(heel[1]), "kind": "contact", "title": tx(nm[0] + " heel", nm[1] + " Ferse"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.07, "footprintWidthM": 0.07, "frictionCoefficient": 0.8})
            out.append({"id": "ball-" + s, "bone": "foot-" + s, "x": r4(ball[0]), "y": r4(ball[1]), "kind": "contact", "title": tx(nm[0] + " ball of foot", nm[1] + " Fußballen"), "support": "contact", "normalDeg": 90, "footprintLengthM": 0.11, "footprintWidthM": 0.09, "frictionCoefficient": 0.8})
        nose = self.head_pt(0.47, 0.33)
        mouth = self.head_pt(0.38, 0.17)
        crown = self.head_pt(0.05, 0.92)
        for id_, p, en, de in (("nose", nose, "Nostrils", "Nasenlöcher"), ("mouth", mouth, "Mouth", "Mund"), ("crown", crown, "Crown of the head", "Scheitel")):
            q = self.rest["head"].inv(*p)
            out.append({"id": id_, "bone": "head", "x": r4(q[0]), "y": r4(q[1]), "kind": "emitter", "title": tx(en, de)})
        return out

    # ---- pose solving ----
    def shoulder(self, pelvis, torso_ang, pelvis_ang=90.0):
        p = pelvis
        t0 = (p[0] + self.Lp * math.cos(math.radians(pelvis_ang)), p[1] + self.Lp * math.sin(math.radians(pelvis_ang)))
        return (t0[0] + self.xs * math.cos(math.radians(torso_ang)), t0[1] + self.xs * math.sin(math.radians(torso_ang))), t0

    def foot_pose(self, pivot, kind, ang):
        """ankle position (world) of a foot with world angle `ang` (the bone points from ankle to ball) such that `kind` ('heel'|'ball') sits at pivot."""
        a = math.radians(ang)
        rest_a = math.radians(-self.theta)
        # local anchor positions relative to ankle in the foot frame
        if kind == "ball":
            lx, ly = self.Lf, 0.0
        else:
            # heel anchor local = inverse-rest of (-heel_back, -ah) relative to ankle (world offsets at rest angle)
            wx, wy = -self.heel_back, -self.ah
            c, s = math.cos(-rest_a), math.sin(-rest_a)
            lx, ly = c * wx - s * wy, s * wx + c * wy
        ox = math.cos(a) * lx - math.sin(a) * ly
        oy = math.sin(a) * lx + math.cos(a) * ly
        return (pivot[0] - ox, pivot[1] - oy)

    def anchor_world(self, anchors, world, id_):
        an = next(x for x in anchors if x["id"] == id_)
        return world[an["bone"]].ap(an["x"], an["y"])

    def solve(self, pelvis, torso, neck, head, legs, arms, pelvis_ang=90.0, extra=None, prismatic=None, knee_side=+1):
        """world angles for every body bone. legs[s]=dict(ankle=(x,y), foot=world deg, toe=world deg); arms[s]=dict(wrist=(x,y), hand=world deg, elbow=side(+1/-1)) or dict(ang=(a1,a2,a3))"""
        wa = {"pelvis": pelvis_ang, "torso": torso, "neck": neck, "head": head}
        flags = {}
        sh, t0 = self.shoulder(pelvis, torso, pelvis_ang)
        for s in ("l", "r"):
            lg = legs[s]
            a1, a2, ok = ik2(pelvis, lg["ankle"], self.Lth, self.Lsh, knee_side)
            wa["thigh-" + s] = a1
            wa["shank-" + s] = a2
            wa["foot-" + s] = lg["foot"]
            wa["toe-" + s] = lg["toe"]
            flags["leg-" + s] = ok
            ar = arms[s]
            if "ang" in ar:
                wa["arm-" + s], wa["forearm-" + s], wa["hand-" + s] = ar["ang"]
            else:
                shs = sh
                b1, b2, ok = ik2(shs, ar["wrist"], self.Lua, self.Lfa, ar.get("elbow", -1))
                wa["arm-" + s] = b1
                wa["forearm-" + s] = b2
                wa["hand-" + s] = ar["hand"]
                flags["arm-" + s] = ok
        if extra: wa.update(extra)
        return wa, flags

    def finalize_offsets(self, wa, prismatic=None):
        return offsets_from_world(self.rig, wa, prismatic or {}, 0.0)


# ---------- thermal helpers ----------
CLO = 0.155
AIR0 = 0.161 * CLO


def vap(R):
    return round(R / (16.5e-3 * 0.38), 2)


def layer(id_, kind, material, R, cap, en, de, thick=None, src="authored-estimate", ref="iso-9920-2007"):
    d = {"id": id_, "title": tx(en, de), "kind": kind, "material": material}
    if thick is not None: d["thicknessM"] = thick
    d["resistanceM2KW"] = round(R, 4)
    d["vapourResistanceM2PaW"] = vap(R)
    d["heatCapacityJM2K"] = cap
    d["source"] = src
    if ref: d["reference"] = ref
    return d


def clothed(prefix, garments):
    ls = [layer(prefix + "-air", "air-gap", "still-air", AIR0, 0, "Air enclosed under the clothing", "Unter der Kleidung eingeschlossene Luft", 0.004, "literature", "iso-9920-2007")]
    for gid, kind, m, iclu, cap, en, de, th in garments:
        ls.append(layer(prefix + "-" + gid, kind, m, iclu * 0.835 * CLO, cap, en, de, th))
    return ls


def body_region(id_, en, de, kind, material, area, L, geom, svf, seg, ms, sf, layers, cap_extra=0, ref="iso-9920-2007", hidden=None):
    cap = 13600 + sum(l.get("heatCapacityJM2K", 0) for l in layers) + cap_extra
    d = {"id": id_, "title": tx(en, de), "kind": kind, "model": "body", "material": material, "areaM2": round(area, 4), "characteristicLengthM": L,
         "geometry": geom, "skyViewFactor": svf, "layers": layers, "heatCapacityJM2K": cap,
         "body": {"segment": seg, "metabolicShare": ms, "skinFraction": sf}}
    if hidden: d["hidden"] = hidden
    d["source"] = "authored-estimate"
    d["reference"] = ref
    return d


def passive_region(id_, en, de, kind, material, area, L, geom, svf, cap, src="authored-estimate", ref=None, layers=None, hidden=None):
    d = {"id": id_, "title": tx(en, de), "kind": kind, "model": "passive", "material": material, "areaM2": round(area, 4), "characteristicLengthM": L,
         "geometry": geom, "skyViewFactor": svf, "layers": layers or [], "heatCapacityJM2K": cap}
    if hidden: d["hidden"] = hidden
    d["source"] = src
    if ref: d["reference"] = ref
    return d


def dubois(kg, H):
    return 0.007184 * kg ** 0.425 * (H * 100) ** 0.725


def pattern(id_, en, de, rate, vt, fr, route, src="authored-estimate", ref="gupta-2010"):
    assert abs(sum(fr) - 1) < 1e-9, id_
    return {"id": id_, "title": tx(en, de), "rateBpm": rate, "tidalVolumeL": vt, "inhaleFraction": fr[0], "pauseAfterInhaleFraction": fr[1], "exhaleFraction": fr[2],
            "pauseAfterExhaleFraction": fr[3], "route": route, "source": src, "reference": ref}


def std_emitters(plume_region, plume_d=0.22, child=False):
    s = 0.8 if child else 1.0
    return [
        {"id": "breath-nose", "kind": "breath", "title": tx("Exhaled breath from the nostrils", "Ausatemluft aus den Nasenlöchern"), "anchor": "nose", "directionDeg": -150, "diameterM": round(0.0095 * s, 4), "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "breath-mouth", "kind": "breath", "title": tx("Exhaled breath from the mouth", "Ausatemluft aus dem Mund"), "anchor": "mouth", "directionDeg": -100, "diameterM": round(0.0124 * s, 4), "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "plume", "kind": "plume", "title": tx("Thermal plume above the body", "Thermische Fahne über dem Körper"), "anchor": "crown", "directionDeg": 0, "diameterM": plume_d, "region": plume_region, "source": "authored-estimate", "reference": "ashrae-2021"}]


GAGGE = ("gagge-1971", "Gagge, A. P., Stolwijk, J. A. J. and Nishi, Y. (1971). An effective temperature scale based on a simple model of human physiological regulatory response. ASHRAE Transactions 77(1), 247-262.")
GUPTA = ("gupta-2010", "Gupta, J. K., Lin, C.-H. and Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39. Used as guidance only: opening areas, jet angles and the flow shape are authored estimates.")
DELEVA_REF = ("de-leva-1996", "de Leva, P. (1996). Adjustments to Zatsiorsky-Seluyanov's segment inertia parameters. Journal of Biomechanics 29(9), 1223-1230. Tables 4 (male) and 5 (female): segment mass fractions and centre-of-mass positions.")
DUBOIS_REF = ("dubois-1916", "DuBois, D. and DuBois, E. F. (1916). A formula to estimate the approximate surface area if height and weight be known. Archives of Internal Medicine 17, 863-871.")
DRILLIS_REF = ("drillis-contini-1966", "Drillis, R., Contini, R. and Bluestein, M. (1964). Body segment parameters: a survey of measurement techniques. Artificial Limbs 8(1), 44-66; segment lengths as fractions of stature.")
ISO9920 = ("iso-9920-2007", "ISO 9920:2007 Ergonomics of the thermal environment: estimation of thermal insulation and water vapour resistance of a clothing ensemble (garment values, I_cl = 0.161 + 0.835 sum I_clu in m2K/W per clo of 0.155, enclosed air layer).")
ISO8996 = ("iso-8996-2021", "ISO 8996:2021 Ergonomics of the thermal environment: determination of metabolic rate (activity levels in met, 1 met = 58.15 W/m2).")
ASHRAE = ("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort: surface properties of skin and fabrics, clothing, metabolic rates, respiratory heat loss, Lewis relation.")
WINTER = ("winter-2009", "Winter, D. A. (2009). Biomechanics and Motor Control of Human Movement, 4th ed., Wiley. Table 4.1 anthropometric segment data; gait cycle phases and walking kinematics.")
PERRY = ("perry-burnfield-2010", "Perry, J. and Burnfield, J. M. (2010). Gait Analysis: Normal and Pathological Function, 2nd ed., SLACK: gait cycle phases, stance and swing fractions, joint angle ranges.")


def physiology(kg, H, ref="gagge-1971", sex="m", age_factor=1.0):
    A = dubois(kg, H)
    bm = (10 * kg + 6.25 * H * 100 - 5 * 30 + (5 if sex == "m" else -161))
    return A


# ---------- pose / clip plumbing ----------
def binding(regions, emitters, met, pat, work=0, passengers=None):
    d = {"activityMet": met, "externalWorkW": work, "breathing": pat, "regions": regions, "emitters": emitters}
    if passengers: d["passengers"] = passengers
    return d


def make_pose(id_, en, de, den, dede, offsets, root, contacts, thermal, loads=None, key=False, ground="auto"):
    p = {"id": id_, "title": tx(en, de), "description": tx(den, dede)}
    if key: p["key"] = True
    p["bones"] = {k: {kk: r4(vv) for kk, vv in v.items()} for k, v in offsets.items()}
    p["root"] = {"x": r4(root[0]), "y": r4(root[1])}
    p["ground"] = ground
    p["contacts"] = contacts
    p["thermal"] = thermal
    p["loads"] = loads or []
    return p
