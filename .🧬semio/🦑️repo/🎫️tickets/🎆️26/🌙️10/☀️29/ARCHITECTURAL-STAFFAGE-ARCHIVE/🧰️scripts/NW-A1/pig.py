# [scratch] NW-A1 pig builder (rig, masses, anchors, poses helpers)
import sys
from build import *
import pig_rig
from sheep_geo import TITLES
from sheep import hoof_x, set_world_angles, leg_by_key, place_leg

P = pig_rig.P
P["titles"] = TITLES
FOOT = "hoof"
MASS = 100.0
REF_M = "brody-1945"


def make_skel():
    sk = build_rig(P)
    shares = {"pelvis": 0.09, "lumbar": 0.19, "thorax": 0.36, "neck": 0.06, "head": 0.065, "jaw": 0.008, "ear-n": 0.004, "ear-f": 0.004, "tail-1": 0.002, "tail-2": 0.001}
    for sfx in ("hn", "hf"):
        shares.update({"thigh-" + sfx: 0.036, "shank-" + sfx: 0.014, "metatarsus-" + sfx: 0.005, FOOT + "-" + sfx: 0.005})
    for sfx in ("fn", "ff"):
        shares.update({"scapula-" + sfx: 0.022, "humerus-" + sfx: 0.018, "forearm-" + sfx: 0.01, "metacarpus-" + sfx: 0.005, FOOT + "-" + sfx: 0.005})
    coms = {"pelvis": (0.06, -0.03, "uniform"), "lumbar": (0.15, -0.16, "uniform"), "thorax": (0.14, -0.17, "uniform"), "neck": (0.08, -0.04, "uniform"), "head": (0.15, -0.02, "point")}
    set_masses(sk, MASS, shares, coms, REF_M)
    for sfx, nm in (("hn", ("near hind", "nahe Hinter")), ("hf", ("far hind", "ferne Hinter")), ("fn", ("near fore", "nahe Vorder")), ("ff", ("far fore", "ferne Vorder"))):
        hb = FOOT + "-" + sfx
        sk.anchor("contact-" + sfx, hb, sk.end(hb), "contact", T("Sole of the %s hoof" % nm[0], "Sohle der %sklaue" % nm[1]), support="contact", normalDeg=90, footprintLengthM=0.06, footprintWidthM=0.045, frictionCoefficient=0.6)
    sk.anchor("nose", "head", add(sk.w0["head"], R((0.31, 0.0), sk.aw["head"])), "emitter", T("Nostrils", "Nasenlöcher"))
    sk.anchor("mouth", "head", add(sk.w0["head"], R((0.24, -0.04), sk.aw["head"])), "emitter", T("Mouth corner (open-mouth panting)", "Maulspalte (Hecheln mit offenem Maul)"))
    sk.anchor("back-plume", "thorax", (0.5, 0.78), "emitter", T("Skin surface above the back (plume source)", "Hautfläche über dem Rücken (Fahnenquelle)"))
    sk.anchor("contact-belly", "thorax", (0.36, 0.205), "contact", T("Belly rests on the ground when lying", "Bauch liegt beim Liegen am Boden"), support="contact", normalDeg=90, footprintLengthM=0.5, footprintWidthM=0.4, frictionCoefficient=0.4)
    sk.anchor("contact-brisket", "thorax", (0.55, 0.205), "contact", T("Brisket rests on the ground when lying", "Brustbein liegt beim Liegen am Boden"), support="contact", normalDeg=90, footprintLengthM=0.3, footprintWidthM=0.3, frictionCoefficient=0.4)
    return sk
