# [scratch] NW-A1 sheep builder
import sys
from build import *
import sheep_rig
from sheep_geo import TITLES

P = sheep_rig.P
P["ear"] = ((0.03, 0.04), 0.09, 195)
P["jaw"] = ((0.07, -0.045), 0.12, -49)
P["titles"] = TITLES
FOOT = "hoof"
MASS = 70.0
REF_M = "brody-1945"


def make_skel():
    sk = build_rig(P)
    shares = {"pelvis": 0.075, "lumbar": 0.17, "thorax": 0.37, "neck": 0.06, "head": 0.042, "jaw": 0.006, "ear-n": 0.0015, "ear-f": 0.0015, "tail-1": 0.004, "tail-2": 0.003}
    for sfx in ("hn", "hf"):
        shares.update({"thigh-" + sfx: 0.036, "shank-" + sfx: 0.016, "metatarsus-" + sfx: 0.006, FOOT + "-" + sfx: 0.005})
    for sfx in ("fn", "ff"):
        shares.update({"scapula-" + sfx: 0.024, "humerus-" + sfx: 0.02, "forearm-" + sfx: 0.012, "metacarpus-" + sfx: 0.006, FOOT + "-" + sfx: 0.005})
    coms = {"pelvis": (0.06, -0.02, "uniform"), "lumbar": (0.13, -0.09, "uniform"), "thorax": (0.14, -0.10, "uniform"), "neck": (0.12, 0.0, "uniform"), "head": (0.1, 0.0, "point")}
    set_masses(sk, MASS, shares, coms, REF_M)
    for sfx, nm in (("hn", ("near hind", "nahe Hinter")), ("hf", ("far hind", "ferne Hinter")), ("fn", ("near fore", "nahe Vorder")), ("ff", ("far fore", "ferne Vorder"))):
        hb = FOOT + "-" + sfx
        L = sk.by[hb]["lengthM"]
        sk.anchor("contact-" + sfx, hb, sk.end(hb), "contact", T("Sole of the %shoof" % (nm[0] + " "), "Sohle der %sklaue" % (nm[1])), support="contact", normalDeg=90, footprintLengthM=0.055, footprintWidthM=0.04, frictionCoefficient=0.7)
    sk.anchor("contact-belly", "thorax", (0.34, 0.302), "contact", T("Belly fleece rests on the ground when lying", "Bauchvlies liegt beim Liegen am Boden"), support="contact", normalDeg=90, footprintLengthM=0.3, footprintWidthM=0.3, frictionCoefficient=0.5)
    sk.anchor("contact-brisket", "thorax", (0.55, 0.304), "contact", T("Brisket rests on the ground when lying", "Brustbein liegt beim Liegen am Boden"), support="contact", normalDeg=90, footprintLengthM=0.25, footprintWidthM=0.25, frictionCoefficient=0.5)
    ho = sk.w0["head"]
    sk.anchor("nose", "head", add(sk.w0["head"], R((0.228, 0.0), sk.aw["head"])), "emitter", T("Nostrils", "Nasenlöcher"))
    sk.anchor("mouth", "head", add(sk.w0["head"], R((0.2, -0.025), sk.aw["head"])), "emitter", T("Mouth corner (open-mouth panting)", "Maulspalte (Hecheln mit offenem Maul)"))
    sk.anchor("back-plume", "thorax", (0.45, sk.spine_y + 0.08), "emitter", T("Fleece surface above the back (plume source)", "Vliesoberfläche über dem Rücken (Fahnenquelle)"))
    return sk


def leg_by_key(sk, key):
    return Leg(sk, key, FOOT, 0.0, 0.0)


def place_leg(sk, offs, key, x, y, ac, ah, scap=None, root=None):
    leg = leg_by_key(sk, key)
    fixed = {leg.chain[-1]: ah, leg.chain[-2]: ac}
    if not leg.hind:
        fixed[leg.chain[0]] = sk.aw[leg.chain[0]] if scap is None else scap
    return solve_leg(sk, offs, leg.chain, leg.pair, fixed, leg.anchor, (x, y), leg.sign, root)


STG = 0.2
STG_H = 0.2
STG_F = 0.2
STG_FG = -0.2


def standing_offsets(sk):
    offs = {}
    place_leg(sk, offs, "hf", hoof_x(sk, "hn") + STG_H, 0.0, -97, -52)
    place_leg(sk, offs, "ff", hoof_x(sk, "fn") + STG_F, 0.0, -82, -53)
    return offs


def hoof_x(sk, key):
    return sk.end("hoof-" + key)[0]


def set_world_angles(sk, offs, world):
    """offsets so that the named bones reach the given world angles, parents first (others keep their offsets)."""
    for b in sk.bones:
        if b["id"] in world:
            fr = sk.frames(offs)
            pa = fr[b["parent"]][1] if b["parent"] else 0.0
            offs[b["id"]] = r4(wrap(world[b["id"]] - pa - b["angleDeg"]))
    return offs


def grazing_offsets(sk, drop=0.08):
    offs = {"pelvis": -10.0}
    set_world_angles(sk, offs, {"neck": -38.0, "head": -62.0})
    root = (sk.hip[0], sk.hip[1] - drop)
    for key, x, ac, ah in (("hn", hoof_x(sk, "hn"), -88, -52), ("hf", hoof_x(sk, "hn") + STG_H, -97, -52), ("fn", hoof_x(sk, "fn"), -90, -53), ("ff", hoof_x(sk, "fn") + STG_FG, -100, -53)):
        place_leg(sk, offs, key, x, 0.0, ac, ah, scap=None, root=root)
    return offs, root


def lying_offsets(sk, drop=0.303):
    offs = {"neck": 42.0, "head": 14.0}
    root = (sk.hip[0], sk.hip[1] - drop)
    place_leg(sk, offs, "hn", 0.16, 0.14, 2, -4, root=root)
    place_leg(sk, offs, "hf", 0.16 + STG_H, 0.14, 2, -4, root=root)
    place_leg(sk, offs, "fn", 0.46, 0.17, 178, 170, scap=-62, root=root)
    place_leg(sk, offs, "ff", 0.46 + STG_F, 0.17, 178, 170, scap=-62, root=root)
    return offs, root
