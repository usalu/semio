"""Human rig and anchor builders (rest pose: standing, arms hanging, pelvis along +y, facing +x). Conventions:
pelvis/torso/head local +y = back, -y = front; arms and legs at rest point down, local +y = front; feet point forward, local +y = up."""
from lib import tx

MALE = dict(pelvis=11.17, torso=32.29, head=6.94, upper=2.71, fore=1.62, hand=0.61, thigh=14.16, shank=4.33, foot=1.37)
FEMALE = dict(pelvis=12.47, torso=30.10, head=6.68, upper=2.55, fore=1.38, hand=0.56, thigh=14.78, shank=4.81, foot=1.29)
COM = dict(pelvis=0.5, torso=0.4566, head=0.5, upper=0.5772, fore=0.4574, hand=0.5, thigh=0.4095, shank=0.4459, foot=0.5)
RATIO = dict(pelvis=0.0568, torso=0.2614, head=0.1534, upper=0.1762, fore=0.1534, hand=0.1023, thigh=0.2443, shank=0.2386, foot=0.125, ankle=0.0454, toe=0.0523, ball=0.0738)

NAMES = {
    "pelvis": ("Pelvis", "Becken"), "torso": ("Trunk", "Rumpf"), "head": ("Head and neck", "Kopf und Hals"),
    "upper-arm": ("upper arm", "Oberarm"), "forearm": ("forearm", "Unterarm"), "hand": ("hand", "Hand"),
    "thigh": ("thigh", "Oberschenkel"), "shank": ("shank", "Unterschenkel"), "foot": ("foot", "Fuß"), "toe": ("forefoot", "Vorfuß"),
}
SIDE = {"l": ("Left", "Linker", "Linke", "Linkes"), "r": ("Right", "Rechter", "Rechte", "Rechtes")}
DE_GENDER = {"upper-arm": 0, "forearm": 0, "hand": 2, "thigh": 0, "shank": 0, "foot": 0, "toe": 0}


def seg_lengths(H, **mod):
    d = {k: v * H for k, v in RATIO.items()}
    for k, v in mod.items():
        d[k] *= v
    return d


def bone(i, parent, en, de, x, y, ang, L, joint, mass=None):
    b = {"id": i, "parent": parent, "title": tx(en, de), "x": round(x, 4), "y": round(y, 4), "angleDeg": ang, "lengthM": round(L, 4), "joint": joint}
    if mass:
        b["mass"] = mass
    return b


def rev(lo, hi):
    return {"type": "revolute", "minDeg": lo, "maxDeg": hi}


def human_bones(H, male, mass, parent_of_pelvis=None, pelvis_xy=(0.0, 0.0), mod=None, foot_min=-65, lengths=None):
    """Return the body bones (pelvis ... toe-r). parent_of_pelvis None makes the pelvis the rig root."""
    L = lengths or seg_lengths(H, **(mod or {}))
    F = MALE if male else FEMALE
    ref = "de-leva-1996"

    def m(k, kg_frac, com_key=None, src="literature"):
        kg = round(mass * kg_frac / 100, 3)
        c = COM[com_key or k] * Ldict[com_key or k]
        return {"kg": kg, "comX": round(c, 4), "comY": 0.0 if k != "foot" else round(-0.045 * H / 1.82, 4), "distribution": "point", "source": src, "reference": ref}

    Ldict = {"pelvis": L["pelvis"], "torso": L["torso"], "head": L["head"], "upper": L["upper-arm"] if "upper-arm" in L else L["upper"], "fore": L["fore"], "hand": L["hand"], "thigh": L["thigh"], "shank": L["shank"], "foot": L["foot"]}
    sh = 0.935 * L["torso"]
    bones = []
    bones.append(bone("pelvis", parent_of_pelvis, "Pelvis", "Becken", pelvis_xy[0], pelvis_xy[1], 90, L["pelvis"], rev(-60, 30), m("pelvis", F["pelvis"])))
    bones.append(bone("torso", "pelvis", "Trunk", "Rumpf", L["pelvis"], 0, 0, L["torso"], rev(-95, 30), m("torso", F["torso"])))
    bones.append(bone("head", "torso", "Head and neck", "Kopf und Hals", L["torso"], 0, 0, L["head"], rev(-50, 45), m("head", F["head"])))
    for s in ("l", "r"):
        e, g1, g2, g3 = SIDE[s]
        bones.append(bone(f"upper-arm-{s}", "torso", f"{e} upper arm", f"{g1} Oberarm", sh, 0, -180, L["upper"], rev(-60, 180), m("upper", F["upper"])))
        bones.append(bone(f"forearm-{s}", f"upper-arm-{s}", f"{e} forearm", f"{g1} Unterarm", L["upper"], 0, 0, L["fore"], rev(0, 150), m("fore", F["fore"])))
        bones.append(bone(f"hand-{s}", f"forearm-{s}", f"{e} hand", f"{g2} Hand".replace("Linker Hand", "Linke Hand").replace("Rechter Hand", "Rechte Hand"), L["fore"], 0, 0, L["hand"], rev(-65, 75), m("hand", F["hand"], "hand")))
    for s in ("l", "r"):
        e, g1, g2, g3 = SIDE[s]
        bones.append(bone(f"thigh-{s}", "pelvis", f"{e} thigh", f"{g1} Oberschenkel", 0, 0, -180, L["thigh"], rev(-60, 125), m("thigh", F["thigh"])))
        bones.append(bone(f"shank-{s}", f"thigh-{s}", f"{e} shank", f"{g1} Unterschenkel", L["thigh"], 0, 0, L["shank"], rev(-150, 0), m("shank", F["shank"])))
        bones.append(bone(f"foot-{s}", f"shank-{s}", f"{e} foot", f"{g1} Fuß", L["shank"], 0, 90, L["foot"], rev(foot_min, 60), m("foot", F["foot"])))
        bones.append(bone(f"toe-{s}", f"foot-{s}", f"{e} forefoot", f"{g1} Vorfuß", L["ball"], -L["ankle"], 0, L["toe"], rev(0, 70)))
    return bones, L


def human_anchors(L, H, kinds=("heel", "ball", "mouth", "nostrils", "crown", "grip", "lumbar")):
    f = H / 1.82
    a = []

    def add(i, bone_, x, y, kind, en, de, **kw):
        d = {"id": i, "bone": bone_, "x": round(x, 4), "y": round(y, 4), "kind": kind, "title": tx(en, de)}
        d.update(kw)
        a.append(d)

    for s in ("l", "r"):
        e = "Left" if s == "l" else "Right"
        g = "Linke" if s == "l" else "Rechte"
        add(f"contact-{s}", f"foot-{s}", L["ball"], -L["ankle"], "contact", f"{e} ball of foot", f"{g} Ballen", support="contact", normalDeg=90, footprintLengthM=round(0.1 * f, 3), footprintWidthM=round(0.09 * f, 3), frictionCoefficient=0.8)
        add(f"heel-{s}", f"foot-{s}", round(-0.031 * f, 4), -L["ankle"], "contact", f"{e} heel", f"{g} Ferse", support="contact", normalDeg=90, footprintLengthM=round(0.07 * f, 3), footprintWidthM=round(0.07 * f, 3), frictionCoefficient=0.8)
    for s in ("l", "r"):
        e = "Left" if s == "l" else "Right"
        g = "Linke" if s == "l" else "Rechte"
        add(f"grip-{s}", f"hand-{s}", round(0.0776 * f, 4), 0.0, "attachment", f"{e} grip", f"{g} Griffstelle")
    add("lumbar", "torso", 0, 0, "load", "Lumbar joint L5/S1", "Lendengelenk L5/S1")
    add("mouth", "head", round(0.1179 * f, 4), round(-0.0982 * f, 4), "emitter", "Mouth", "Mund")
    add("nostrils", "head", round(0.148 * f, 4), round(-0.1033 * f, 4), "emitter", "Nostrils", "Nasenlöcher")
    add("crown", "head", round(0.2876 * f, 4), 0.0, "emitter", "Crown of the head (plume source)", "Scheitel (Fahnenquelle)")
    return a
