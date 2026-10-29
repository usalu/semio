"""Shared anatomical rig and base silhouettes for the humans-b assets (hand-entered width tables and paths, see lib.py)."""
from lib import *

SIDES = (("l", "far"), ("r", "near"))
Z = {"far": {"arm": 0.40, "leg": 1.0, "hand": 0.45}, "near": {"arm": 6.0, "leg": 2.0, "hand": 6.2}}

# profile paths in (u, v) of the head bone: v up from C7 (neutral 1.76 m), u forward
HEAD_SKIN = [("M", -0.045, 0.0), ("L", -0.040, 0.075), ("C", -0.066, 0.100, -0.100, 0.130, -0.098, 0.190), ("C", -0.095, 0.245, -0.040, 0.276, 0.015, 0.272), ("C", 0.065, 0.268, 0.088, 0.238, 0.089, 0.205),
             ("L", 0.092, 0.188), ("C", 0.096, 0.178, 0.108, 0.162, 0.118, 0.149), ("C", 0.112, 0.141, 0.100, 0.138, 0.094, 0.134), ("C", 0.098, 0.124, 0.094, 0.116, 0.088, 0.110), ("C", 0.092, 0.102, 0.088, 0.092, 0.080, 0.088),
             ("C", 0.062, 0.080, 0.045, 0.076, 0.034, 0.072), ("C", 0.042, 0.050, 0.050, 0.022, 0.052, 0.0), ("Z",)]
HAIR_SHORT = [("M", 0.090, 0.208), ("C", 0.094, 0.248, 0.052, 0.283, 0.005, 0.281), ("C", -0.062, 0.279, -0.108, 0.242, -0.105, 0.186), ("C", -0.103, 0.150, -0.088, 0.122, -0.060, 0.108),
              ("L", -0.052, 0.150), ("C", -0.046, 0.196, -0.008, 0.226, 0.040, 0.230), ("C", 0.066, 0.231, 0.084, 0.226, 0.090, 0.208), ("Z",)]
HAIR_TIED = [("M", 0.090, 0.208), ("C", 0.094, 0.248, 0.052, 0.283, 0.005, 0.281), ("C", -0.062, 0.279, -0.108, 0.242, -0.105, 0.186), ("C", -0.112, 0.150, -0.130, 0.118, -0.140, 0.085),
             ("C", -0.150, 0.060, -0.120, 0.052, -0.100, 0.075), ("C", -0.085, 0.095, -0.060, 0.112, -0.052, 0.150), ("C", -0.046, 0.196, -0.008, 0.226, 0.040, 0.230), ("C", 0.066, 0.231, 0.084, 0.226, 0.090, 0.208), ("Z",)]
HAND = [("M", 0.026, -0.005), ("C", 0.034, 0.040, 0.040, 0.100, 0.034, 0.150), ("C", 0.030, 0.185, 0.010, 0.196, 0.000, 0.180), ("C", -0.010, 0.170, -0.022, 0.120, -0.024, 0.070), ("C", -0.024, 0.030, -0.022, 0.005, -0.020, -0.005), ("Z",)]
SHOES = {
    "sneaker": ([("M", -0.060, -0.080), ("C", -0.066, -0.045, -0.062, -0.005, -0.052, 0.030), ("L", 0.035, 0.030), ("C", 0.060, 0.012, 0.100, -0.008, 0.150, -0.034), ("C", 0.190, -0.050, 0.218, -0.058, 0.218, -0.070), ("C", 0.218, -0.078, 0.212, -0.080, 0.200, -0.080), ("Z",)],
                [(-0.061, -0.060), (0.216, -0.060)]),
    "boot": ([("M", -0.062, -0.080), ("L", -0.066, 0.000), ("C", -0.066, 0.050, -0.060, 0.100, -0.052, 0.125), ("L", 0.052, 0.125), ("L", 0.054, 0.045), ("C", 0.080, 0.020, 0.130, -0.012, 0.170, -0.034), ("C", 0.205, -0.048, 0.228, -0.056, 0.228, -0.070), ("C", 0.228, -0.078, 0.222, -0.080, 0.212, -0.080), ("Z",)],
             [(-0.064, -0.052), (0.226, -0.052)]),
    "runner": ([("M", -0.066, -0.080), ("C", -0.074, -0.050, -0.068, -0.008, -0.054, 0.032), ("L", 0.030, 0.034), ("C", 0.060, 0.020, 0.105, 0.000, 0.150, -0.026), ("C", 0.188, -0.044, 0.222, -0.052, 0.222, -0.068), ("C", 0.222, -0.077, 0.216, -0.080, 0.205, -0.080), ("Z",)],
                [(-0.068, -0.056), (0.220, -0.056)]),
    "shoe": ([("M", -0.058, -0.080), ("C", -0.064, -0.045, -0.060, -0.005, -0.050, 0.028), ("L", 0.040, 0.030), ("C", 0.065, 0.012, 0.105, -0.010, 0.145, -0.032), ("C", 0.185, -0.046, 0.212, -0.056, 0.212, -0.068), ("C", 0.212, -0.077, 0.206, -0.080, 0.196, -0.080), ("Z",)],
              [(-0.059, -0.066), (0.210, -0.066)]),
}

def sc(v, s): return v * s

class Human:
    def __init__(self, height_m, mass_kg, wf=1.0, ref_masses="de-leva-1996"):
        self.H, self.M, self.s, self.wf = height_m, mass_kg, height_m / 1.76, wf
        s = self.s
        self.L = {"pelvis": 0.10 * s, "torso": 0.46 * s, "head": 0.27 * s, "upper-arm": 0.31 * s, "forearm": 0.27 * s, "hand": 0.18 * s, "thigh": 0.43 * s, "shank": 0.42 * s, "foot": 0.22 * s}
        self.up, self.down, self.fwd = Frame("up", s, wf), Frame("down", s, wf), Frame("fwd", s, 1.0)
        self.ankle = 0.08 * s

    # ------------------------------------------------------------------------------------------------------------------- rig
    def m(self, kind, comX, comY=0.0, src="literature", ref="de-leva-1996"):
        return {"kg": seg_mass(kind, self.M), "comX": r(comX), "comY": r(comY), "distribution": "point", "source": src, "reference": ref}

    def bone(self, id, parent, en, de, x, y, ang, length, joint, mass=None):
        b = {"id": id, "parent": parent, "title": T(en, de), "x": r(x), "y": r(y), "angleDeg": ang, "lengthM": r(length), "joint": joint}
        if mass: b["mass"] = mass
        return b

    def bones(self, extra_head_kg=0.0):
        L, s = self.L, self.s
        rv = lambda lo, hi: {"type": "revolute", "minDeg": lo, "maxDeg": hi}
        out = [
            self.bone("pelvis", None, "Pelvis", "Becken", 0, 0, 90, L["pelvis"], rv(-60, 30), self.m("pelvis", 0.05 * s)),
            self.bone("torso", "pelvis", "Trunk", "Rumpf", L["pelvis"], 0, 0, L["torso"], rv(-95, 30), self.m("torso", 0.21 * s)),
            self.bone("head", "torso", "Head and neck", "Kopf und Hals", L["torso"], 0, 0, L["head"], rv(-50, 45), self.m("head", 0.135 * s)),
        ]
        for side, title_en, title_de in (("l", "Left", "Linker"), ("r", "Right", "Rechter")):
            ar = "Arm" if side else ""
            out += [
                self.bone(f"upper-arm-{side}", "torso", f"{title_en} upper arm", f"{title_de} Oberarm", L["torso"] - 0.03 * s, 0, -180, L["upper-arm"], rv(-60, 180), self.m("upper-arm", 0.5772 * L["upper-arm"])),
                self.bone(f"forearm-{side}", f"upper-arm-{side}", f"{title_en} forearm", f"{title_de} Unterarm", L["upper-arm"], 0, 0, L["forearm"], rv(0, 150), self.m("forearm", 0.4574 * L["forearm"])),
                self.bone(f"hand-{side}", f"forearm-{side}", f"{title_en} hand", f"{title_de} Hand", L["forearm"], 0, 0, L["hand"], rv(-65, 75), self.m("hand", 0.09 * s, src="authored-estimate")),
                self.bone(f"thigh-{side}", "pelvis", f"{title_en} thigh", f"{title_de} Oberschenkel", 0, 0, -180, L["thigh"], rv(-30, 125), self.m("thigh", 0.4095 * L["thigh"])),
                self.bone(f"shank-{side}", f"thigh-{side}", f"{title_en} shank", f"{title_de} Unterschenkel", L["thigh"], 0, 0, L["shank"], rv(-150, 0), self.m("shank", 0.4459 * L["shank"])),
                self.bone(f"foot-{side}", f"shank-{side}", f"{title_en} foot", f"{title_de} Fuß", L["shank"], 0, 90, L["foot"], rv(-50, 38), self.m("foot", 0.11 * s, -0.04 * s)),
            ]
        return out

    def anchors(self):
        s = self.s
        out = []
        for side, en, de in (("l", "Left", "Linke"), ("r", "Right", "Rechte")):
            out.append({"id": f"contact-{side}", "bone": f"foot-{side}", "x": r(0.13 * s), "y": r(-0.08 * s), "kind": "contact", "title": T(f"{en} sole", f"{de} Sohle"), "support": "contact", "normalDeg": 90, "footprintLengthM": r(0.20 * s), "footprintWidthM": r(0.10 * s), "frictionCoefficient": 0.6})
        for side, en, de in (("l", "Left", "Linke"), ("r", "Right", "Rechte")):
            out.append({"id": f"heel-{side}", "bone": f"foot-{side}", "x": r(-0.03 * s), "y": r(-0.08 * s), "kind": "contact", "title": T(f"{en} heel", f"{de} Ferse"), "support": "contact", "normalDeg": 90, "footprintLengthM": r(0.08 * s), "footprintWidthM": r(0.08 * s), "frictionCoefficient": 0.6})
        out.append({"id": "mouth", "bone": "head", "x": r(0.114 * s), "y": r(-0.095 * s), "kind": "emitter", "title": T("Mouth and nose", "Mund und Nase")})
        for side, en, de in (("l", "Left", "Linke"), ("r", "Right", "Rechte")):
            out.append({"id": f"grip-{side}", "bone": f"hand-{side}", "x": r(0.075 * s), "y": 0, "kind": "attachment", "title": T(f"{en} grip", f"{de} Griffstelle")})
        return out

    # ------------------------------------------------------------------------------------------------------------------- geometry
    def sh(self, id, bone, role, lod, z, tone, geom, region=None, material=None, filled=True):
        return shape(id, bone, role, lod, z, tone, geom, region, material, filled)

    def head_shapes(self, skin_tone=0.86, hair="short", hair_tone=0.18, skin_region="head-skin", hair_region="hair", skin_mat="skin", hair_mat="hair", face=True, hair_lod=0):
        f = self.up
        out = [self.sh("head", "head", "body", 0, 5.0, skin_tone, g_path(f, HEAD_SKIN, True), skin_region, skin_mat)]
        if hair:
            cmds = HAIR_SHORT if hair == "short" else HAIR_TIED
            out.append(self.sh("hair", "head", "body", hair_lod, 5.4, hair_tone, g_path(f, cmds, True), hair_region, hair_mat))
            out.append(self.sh("hair-strands", "head", "texture", 3, 5.5, hair_tone, g_poly(f, [(-0.075, 0.235), (-0.03, 0.262), (0.02, 0.268)], False), hair_region, hair_mat, False))
        if face:
            out += [
                self.sh("eye", "head", "detail", 3, 5.6, 0.1, g_ellipse(f, 0.060, 0.178, 0.0075, 0.0055), skin_region, skin_mat),
                self.sh("brow", "head", "detail", 3, 5.6, 0.2, g_poly(f, [(0.046, 0.194), (0.080, 0.197)], False), skin_region, skin_mat, False),
                self.sh("ear", "head", "detail", 3, 5.6, 0.7, g_ellipse(f, -0.020, 0.158, 0.012, 0.024), skin_region, skin_mat, False),
                self.sh("mouth-line", "head", "detail", 3, 5.6, 0.3, g_poly(f, [(0.075, 0.112), (0.090, 0.110)], False), skin_region, skin_mat, False),
            ]
        return out

    def torso_shape(self, id="torso", role="body", lod=0, z=3.2, tone=0.5, region="torso-clothing", material="cotton-clothing", grow=0.0, v0=None, v1=None):
        return self.sh(id, "torso", role, lod, z, tone, g_path(self.up, table_cmds(TORSO, v0, v1, grow=grow), True), region, material)

    def pelvis_shape(self, tone=0.3, region="leg-clothing", material="cotton-clothing", z=3.05):
        return self.sh("pelvis", "pelvis", "body", 0, z, tone, g_path(self.up, table_cmds(PELVIS, grow=0.0), True), region, material)

    def limb(self, id, bone, tab, frame, role, lod, z, tone, region, material, d0=None, d1=None, cap0=0.0, cap1=0.0, grow=0.0):
        return self.sh(id, bone, role, lod, z, tone, g_path(frame, table_cmds(tab, d0, d1, cap0, cap1, grow), True), region, material)

    def hand_shape(self, side, tone=0.86, region="hand-skin", material="skin"):
        z = Z["near" if side == "r" else "far"]["hand"]
        return self.sh(f"hand-{side}", f"hand-{side}", "body", 0, z, tone, g_path(self.down, HAND, True), region, material)

    def shoe(self, side, kind="sneaker", tone=0.15, region="shoes", material="leather-shoe"):
        z = Z["near" if side == "r" else "far"]["leg"] + 0.2
        cmds, sole = SHOES[kind]
        out = [self.sh(f"shoe-{side}", f"foot-{side}", "body", 0, z, tone, g_path(self.fwd, cmds), region, material),
               self.sh(f"sole-{side}", f"foot-{side}", "detail", 2, z + 0.01, 0.1, g_poly(self.fwd, sole, False), region, material, False)]
        return out

    def standard(self, arm_sleeve="short", legs="long", shoe="sneaker", tones=None, hair="short", leg_hem=0.30, face=True, hair_tone=0.18):
        """Base set: head, torso shirt, pelvis/leg clothing, skin arms/hands, shoes. Returns a list; assets replace or extend."""
        t = {"skin": 0.86, "shirt": 0.55, "trousers": 0.30, "shoes": 0.15, **(tones or {})}
        s, up, down = self.s, self.up, self.down
        out = self.head_shapes(t["skin"], hair, hair_tone, face=face)
        out.append(self.torso_shape(tone=t["shirt"]))
        out.append(self.pelvis_shape(tone=t["trousers"]))
        for side, near in SIDES:
            z = Z[near]
            sl = 0.14 if arm_sleeve == "short" else 0.0
            # arms
            out.append(self.limb(f"upper-arm-{side}", f"upper-arm-{side}", UPPER_ARM, down, "body", 0, z["arm"], t["shirt"], "arm-clothing", "cotton-clothing", d0=-0.01, d1=(0.14 if arm_sleeve == "short" else 0.31), cap0=0.035))
            if arm_sleeve == "short":
                out.append(self.limb(f"upper-arm-skin-{side}", f"upper-arm-{side}", UPPER_ARM, down, "body", 0, z["arm"] - 0.02, t["skin"], "arm-skin", "skin", d0=0.12, d1=0.31))
                out.append(self.sh(f"sleeve-hem-{side}", f"upper-arm-{side}", "detail", 2, z["arm"] + 0.01, 0.3, g_poly(down, [(0.048, 0.14), (-0.044, 0.14)], False), "arm-clothing", "cotton-clothing", False))
            out.append(self.limb(f"forearm-{side}", f"forearm-{side}", FOREARM, down, "body", 0, z["arm"] + 0.02, t["shirt"] if arm_sleeve == "long" else t["skin"], "arm-clothing" if arm_sleeve == "long" else "arm-skin", "cotton-clothing" if arm_sleeve == "long" else "skin", d0=-0.005, d1=0.27, cap1=0.01))
            if arm_sleeve == "long":
                out.append(self.sh(f"cuff-{side}", f"forearm-{side}", "detail", 2, z["arm"] + 0.03, 0.3, g_poly(down, [(0.028, 0.245), (-0.026, 0.245)], False), "arm-clothing", "cotton-clothing", False))
            out.append(self.hand_shape(side, t["skin"]))
            # legs
            lz = z["leg"]
            if legs == "long":
                out.append(self.limb(f"thigh-{side}", f"thigh-{side}", THIGH, down, "body", 0, lz, t["trousers"], "leg-clothing", "cotton-clothing", cap0=0.0, cap1=0.012))
                out.append(self.limb(f"shank-{side}", f"shank-{side}", SHANK, down, "body", 0, lz + 0.02, t["trousers"], "leg-clothing", "cotton-clothing", d0=-0.01, d1=0.40, cap1=0.01))
                out.append(self.sh(f"trouser-hem-{side}", f"shank-{side}", "detail", 2, lz + 0.03, 0.2, g_poly(down, [(0.040, 0.385), (-0.044, 0.385)], False), "leg-clothing", "cotton-clothing", False))
            else:
                out.append(self.limb(f"thigh-{side}", f"thigh-{side}", THIGH, down, "body", 0, lz, t["trousers"], "leg-clothing", "cotton-clothing", d1=leg_hem))
                out.append(self.limb(f"thigh-skin-{side}", f"thigh-{side}", THIGH, down, "body", 0, lz - 0.02, t["skin"], "leg-skin", "skin", d0=leg_hem - 0.04))
                out.append(self.limb(f"shank-{side}", f"shank-{side}", SHANK, down, "body", 0, lz + 0.02, t["skin"], "leg-skin", "skin", d0=-0.01, d1=0.40, cap1=0.01))
            out += self.shoe(side, shoe, t["shoes"])
        return out
