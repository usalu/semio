import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

# World = rig rest pose, root at the origin on the ground. Front x = lateral (left +x); plan x = depth axis, y = lateral.
# Drops hang in a 3 x 3 grid below the trough: x in {-0.1, 0, 0.1} (side view) and lateral in {-0.1, 0, 0.1} (front view).


def drops_front(rx, ry, ids=(-0.1, 0, 0.1)):
    return [ellipse("drop-%d" % (i + 1), "drip", "body", 3.9, 0.85, lat, 0.5, rx, ry, region="pad", material="water") for i, lat in enumerate(ids)]


front50 = [
    ellipse("drop-1", "drip", "body", 3.9, 0.85, -0.1, 0.5, 0.01, 0.013, region="pad", material="water"),
    ellipse("drop-2", "drip", "body", 3.9, 0.85, 0, 0.5, 0.01, 0.013, region="pad", material="water"),
    ellipse("drop-3", "drip", "body", 3.9, 0.85, 0.1, 0.5, 0.01, 0.013, region="pad", material="water"),
    rect("trough", "trough", "body", 3.5, 0.4, -0.19, 0.48, 0.38, 0.05, region="trough", material="pp-plastic"),
    rect("pad", "pad", "body", 3.3, 0.7, -0.18, 0.08, 0.36, 0.4, region="pad", material="cellulose"),
    rect("water", "water", "body", 3.2, 0.45, -0.19, 0.02, 0.38, 0.05, region="sump", material="water"),
    rect("sump-tray", "casing", "body", 3.4, 0.3, -0.19, 0, 0.38, 0.02, region="casing", material="galvanised-steel"),
    rect("casing", "casing", "body", 3.1, 0.3, -0.2, 0, 0.4, 0.6, region="casing", material="galvanised-steel", filled=False),
]
plan50 = [
    ellipse("drop-1", "drip", "body", 3.9, 0.85, -0.1, -0.1, 0.01, 0.01, region="pad", material="water"),
    ellipse("drop-2", "drip", "body", 3.9, 0.85, 0, -0.1, 0.01, 0.01, region="pad", material="water"),
    ellipse("drop-3", "drip", "body", 3.9, 0.85, 0.1, -0.1, 0.01, 0.01, region="pad", material="water"),
    ellipse("drop-4", "drip", "body", 3.9, 0.85, -0.1, 0, 0.01, 0.01, region="pad", material="water"),
    ellipse("drop-5", "drip", "body", 3.9, 0.85, 0, 0, 0.01, 0.01, region="pad", material="water"),
    ellipse("drop-6", "drip", "body", 3.9, 0.85, 0.1, 0, 0.01, 0.01, region="pad", material="water"),
    ellipse("drop-7", "drip", "body", 3.9, 0.85, -0.1, 0.1, 0.01, 0.01, region="pad", material="water"),
    ellipse("drop-8", "drip", "body", 3.9, 0.85, 0, 0.1, 0.01, 0.01, region="pad", material="water"),
    ellipse("drop-9", "drip", "body", 3.9, 0.85, 0.1, 0.1, 0.01, 0.01, region="pad", material="water"),
    rect("film", "pad", "body", 3.6, 0.85, -0.19, -0.17, 0.38, 0.34, region="pad", material="water"),
    rect("trough", "trough", "body", 3.5, 0.4, -0.21, -0.19, 0.42, 0.38, region="trough", material="pp-plastic"),
    rect("casing", "casing", "body", 3.1, 0.3, -0.25, -0.2, 0.5, 0.4, region="casing", material="galvanised-steel", filled=False),
]
front100 = [
    ellipse("drop-1", "drip", "body", 3.9, 0.85, -0.1, 0.5, 0.014, 0.0182, region="pad", material="water"),
    ellipse("drop-2", "drip", "body", 3.9, 0.85, 0, 0.5, 0.014, 0.0182, region="pad", material="water"),
    ellipse("drop-3", "drip", "body", 3.9, 0.85, 0.1, 0.5, 0.014, 0.0182, region="pad", material="water"),
    rect("trough", "trough", "body", 3.5, 0.4, -0.19, 0.48, 0.38, 0.05, region="trough", material="pp-plastic"),
    rect("pad", "pad", "body", 3.3, 0.7, -0.18, 0.08, 0.36, 0.4, region="pad", material="cellulose"),
    rect("water", "water", "body", 3.2, 0.45, -0.19, 0.02, 0.38, 0.05, region="sump", material="water"),
    rect("casing", "casing", "body", 3.1, 0.3, -0.2, 0, 0.4, 0.6, region="casing", material="galvanised-steel", filled=False),
]
plan100 = [
    ellipse("drop-1", "drip", "body", 3.9, 0.85, -0.1, -0.1, 0.014, 0.014, region="pad", material="water"),
    ellipse("drop-2", "drip", "body", 3.9, 0.85, 0.1, -0.1, 0.014, 0.014, region="pad", material="water"),
    ellipse("drop-3", "drip", "body", 3.9, 0.85, 0, 0, 0.014, 0.014, region="pad", material="water"),
    ellipse("drop-4", "drip", "body", 3.9, 0.85, -0.1, 0.1, 0.014, 0.014, region="pad", material="water"),
    ellipse("drop-5", "drip", "body", 3.9, 0.85, 0.1, 0.1, 0.014, 0.014, region="pad", material="water"),
    rect("trough", "trough", "body", 3.5, 0.4, -0.21, -0.19, 0.42, 0.38, region="trough", material="pp-plastic"),
    rect("casing", "casing", "body", 3.1, 0.3, -0.25, -0.2, 0.5, 0.4, region="casing", material="galvanised-steel", filled=False),
]
front200 = [
    rect("trough", "trough", "body", 3.5, 0.4, -0.19, 0.48, 0.38, 0.06, region="trough", material="pp-plastic"),
    rect("pad", "pad", "body", 3.3, 0.7, -0.18, 0.1, 0.36, 0.37, regions=["pad"], material="cellulose"),
    rect("water", "water", "body", 3.2, 0.45, -0.19, 0.04, 0.38, 0.05, region="sump", material="water"),
    rect("casing", "casing", "body", 3.1, 0.3, -0.2, 0, 0.4, 0.6, region="casing", material="galvanised-steel", filled=False),
]
plan200 = [
    rect("trough", "trough", "body", 3.5, 0.4, -0.21, -0.19, 0.42, 0.38, regions=["trough", "pad"], material="pp-plastic"),
    rect("casing", "casing", "body", 3.1, 0.3, -0.25, -0.2, 0.5, 0.4, region="casing", material="galvanised-steel", filled=False),
]

apply("evaporative-humidifier", {
    "50": {"front": front50, "plan": plan50, "dec": {
        "retained": (
            "Front view: casing outline, sump tray and water, the pad face, the trough and the three drops of the front row. Plan: casing rim, trough with its water film and the nine drops in a 3 by 3 grid, which fall with the drip bone.",
            "Vorderansicht: Gehäuseumriss, Wannenblech und Wasser, die Füllkörperfläche, die Rinne und die drei Tropfen der vorderen Reihe. Grundriss: Gehäuserand, Rinne mit Wasserfilm und die neun Tropfen im 3-mal-3-Raster, die mit dem Tropfenknochen fallen."),
        "omitted": (
            "In the plan the pad and the sump lie below the trough and stay hidden.",
            "Im Grundriss liegen Füllkörper und Wanne unter der Rinne und bleiben verdeckt."),
    }},
    "100": {"front": front100, "plan": plan100, "dec": {
        "retained": (
            "Front view: casing, pad, trough, sump water and three drops. Plan: casing rim, trough and five drops that fall with the drip bone.",
            "Vorderansicht: Gehäuse, Füllkörper, Rinne, Wannenwasser und drei Tropfen. Grundriss: Gehäuserand, Rinne und fünf Tropfen, die mit dem Tropfenknochen fallen."),
        "omitted": (
            "In the plan the water film, the pad and the sump are hidden below the trough.",
            "Im Grundriss sind Wasserfilm, Füllkörper und Wanne unter der Rinne verdeckt."),
    }},
    "200": {"front": front200, "plan": plan200, "dec": {
        "retained": (
            "Front view: casing outline, pad, trough and sump water. Plan: casing outline and one trough block.",
            "Vorderansicht: Gehäuseumriss, Füllkörper, Rinne und Wannenwasser. Grundriss: Gehäuseumriss und ein Rinnenblock."),
        "merged": (
            "In the plan the trough block carries the regions trough and pad (the wet film).",
            "Im Grundriss trägt der Rinnenblock die Regionen Rinne und Füllkörper (der nasse Film)."),
    }},
})
