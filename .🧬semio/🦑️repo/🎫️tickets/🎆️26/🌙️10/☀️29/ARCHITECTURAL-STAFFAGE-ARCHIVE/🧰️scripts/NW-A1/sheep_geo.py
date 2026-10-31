# [scratch] NW-A1 sheep: rig + geometry builders per scale
from quad import *
from gait import *
import sheep_rig

TITLES = {
    "pelvis": T("Pelvis and hip joint", "Becken und Hüftgelenk"), "lumbar": T("Lumbar spine", "Lendenwirbelsäule"),
    "thorax": T("Thoracic spine and ribcage", "Brustwirbelsäule und Brustkorb"), "neck": T("Neck", "Hals"), "head": T("Head", "Kopf"),
    "jaw": T("Lower jaw", "Unterkiefer"), "ear-n": T("Near ear", "Nahes Ohr"), "ear-f": T("Far ear", "Fernes Ohr"),
    "tail-1": T("Tail root", "Schwanzwurzel"), "tail-2": T("Tail tip", "Schwanzspitze"),
    "hind": [("thigh (femur)", "Oberschenkel"), ("shank (tibia)", "Unterschenkel"), ("metatarsus (hock to fetlock)", "Mittelfuß (Sprunggelenk bis Fesselgelenk)"), ("hind hoof with pastern", "Hinterklaue mit Fessel")],
    "fore": [("shoulder blade", "Schulterblatt"), ("upper arm (humerus)", "Oberarm"), ("forearm", "Unterarm"), ("metacarpus (knee to fetlock)", "Mittelhand (Vorderfusswurzel bis Fesselgelenk)"), ("fore hoof with pastern", "Vorderklaue mit Fessel")],
}
sheep_rig.P["titles"] = TITLES


def rig():
    sk = build_rig(sheep_rig.P)
    return sk


def anchors(sk):
    for sfx, nm in (("hn", "near hind"), ("hf", "far hind"), ("fn", "near fore"), ("ff", "far fore")):
        pass
