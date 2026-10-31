# [scratch] NW-A1 fox builder (rig, masses, anchors, poses helpers)
import sys
from build import *
import fox_rig
from sheep_geo import TITLES
from sheep import set_world_angles

P = fox_rig.P
P["titles"] = dict(TITLES)
P["titles"]["tail-3"] = T("Brush tip", "Fuchsschwanzspitze")
P["titles"]["tail-1"] = T("Brush root", "Schwanzwurzel")
P["titles"]["tail-2"] = T("Brush middle", "Schwanzmitte")
P["titles"]["hind"] = [("thigh (femur)", "Oberschenkel"), ("shank (tibia)", "Unterschenkel"), ("metatarsus (hock to toes)", "Mittelfuß (Sprunggelenk bis Zehen)"), ("hind paw", "Hinterpfote")]
P["titles"]["fore"] = [("shoulder blade", "Schulterblatt"), ("upper arm (humerus)", "Oberarm"), ("forearm", "Unterarm"), ("metacarpus (wrist)", "Mittelhand (Handwurzel)"), ("fore paw", "Vorderpfote")]
FOOT = "paw"
MASS = 6.0
REF_M = "brody-1945"


def make_skel():
    sk = build_rig(P)
    shares = {"pelvis": 0.07, "lumbar": 0.14, "thorax": 0.30, "neck": 0.06, "head": 0.075, "jaw": 0.01, "ear-n": 0.003, "ear-f": 0.003, "tail-1": 0.012, "tail-2": 0.012, "tail-3": 0.008}
    for sfx in ("hn", "hf"):
        shares.update({"thigh-" + sfx: 0.036, "shank-" + sfx: 0.016, "metatarsus-" + sfx: 0.007, FOOT + "-" + sfx: 0.007})
    for sfx in ("fn", "ff"):
        shares.update({"scapula-" + sfx: 0.02, "humerus-" + sfx: 0.016, "forearm-" + sfx: 0.012, "metacarpus-" + sfx: 0.006, FOOT + "-" + sfx: 0.006})
    coms = {"pelvis": (0.03, -0.02, "uniform"), "lumbar": (0.11, -0.06, "uniform"), "thorax": (0.1, -0.07, "uniform"), "neck": (0.07, 0.0, "uniform"), "head": (0.08, 0.0, "point")}
    set_masses(sk, MASS, shares, coms, REF_M)
    for sfx, nm in (("hn", ("near hind", "nahe Hinter")), ("hf", ("far hind", "ferne Hinter")), ("fn", ("near fore", "nahe Vorder")), ("ff", ("far fore", "ferne Vorder"))):
        hb = FOOT + "-" + sfx
        sk.anchor("contact-" + sfx, hb, sk.end(hb), "contact", T("Pads of the %s paw" % nm[0], "Ballen der %spfote" % nm[1]), support="contact", normalDeg=90, footprintLengthM=0.05, footprintWidthM=0.035, frictionCoefficient=0.6)
    sk.anchor("nose", "head", add(sk.w0["head"], R((0.172, 0.0), sk.aw["head"])), "emitter", T("Nostrils", "Nasenlöcher"))
    sk.anchor("mouth", "head", add(sk.w0["head"], R((0.12, -0.02), sk.aw["head"])), "emitter", T("Mouth corner (open-mouth panting)", "Maulspalte (Hecheln mit offenem Maul)"))
    sk.anchor("back-plume", "thorax", (0.25, 0.5), "emitter", T("Coat surface above the back (plume source)", "Fellfläche über dem Rücken (Fahnenquelle)"))
    return sk


def hoof_x(sk, key):
    return sk.end("paw-" + key)[0]


def leg_by_key(sk, key):
    return Leg(sk, key, FOOT, 0.0, 0.0)


def place_leg(sk, offs, key, x, y, ac, ah, scap=None, root=None):
    leg = leg_by_key(sk, key)
    fixed = {leg.chain[-1]: ah, leg.chain[-2]: ac}
    if not leg.hind:
        fixed[leg.chain[0]] = sk.aw[leg.chain[0]] if scap is None else scap
    return solve_leg(sk, offs, leg.chain, leg.pair, fixed, leg.anchor, (x, y), leg.sign, root)
