# [scratch] NW-A1 goat builder (rig, masses, anchors, poses helpers)
import sys
from build import *
import goat_rig
from sheep_geo import TITLES
from sheep import hoof_x, set_world_angles, leg_by_key, place_leg

P = goat_rig.P
P["titles"] = TITLES
FOOT = "hoof"
MASS = 55.0
REF_M = "silanikove-2000"


def make_skel():
    sk = build_rig(P)
    shares = {"pelvis": 0.08, "lumbar": 0.17, "thorax": 0.33, "neck": 0.065, "head": 0.045, "jaw": 0.006, "ear-n": 0.002, "ear-f": 0.002, "tail-1": 0.003, "tail-2": 0.002}
    for sfx in ("hn", "hf"):
        shares.update({"thigh-" + sfx: 0.042, "shank-" + sfx: 0.019, "metatarsus-" + sfx: 0.007, FOOT + "-" + sfx: 0.006})
    for sfx in ("fn", "ff"):
        shares.update({"scapula-" + sfx: 0.026, "humerus-" + sfx: 0.022, "forearm-" + sfx: 0.014, "metacarpus-" + sfx: 0.007, FOOT + "-" + sfx: 0.006})
    coms = {"pelvis": (0.06, -0.02, "uniform"), "lumbar": (0.15, -0.08, "uniform"), "thorax": (0.17, -0.09, "uniform"), "neck": (0.14, 0.0, "uniform"), "head": (0.1, 0.0, "point")}
    set_masses(sk, MASS, shares, coms, REF_M)
    for sfx, nm in (("hn", ("near hind", "nahe Hinter")), ("hf", ("far hind", "ferne Hinter")), ("fn", ("near fore", "nahe Vorder")), ("ff", ("far fore", "ferne Vorder"))):
        hb = FOOT + "-" + sfx
        sk.anchor("contact-" + sfx, hb, sk.end(hb), "contact", T("Sole of the %s hoof" % nm[0], "Sohle der %sklaue" % nm[1]), support="contact", normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.035, frictionCoefficient=0.8)
    sk.anchor("nose", "head", add(sk.w0["head"], R((0.228, 0.0), sk.aw["head"])), "emitter", T("Nostrils", "Nasenlöcher"))
    sk.anchor("mouth", "head", add(sk.w0["head"], R((0.2, -0.022), sk.aw["head"])), "emitter", T("Mouth corner (open-mouth panting)", "Maulspalte (Hecheln mit offenem Maul)"))
    sk.anchor("back-plume", "thorax", (0.5, sk.spine_y + 0.07), "emitter", T("Coat surface above the back (plume source)", "Fellfläche über dem Rücken (Fahnenquelle)"))
    return sk
