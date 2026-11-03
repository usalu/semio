import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

# Front: seen from the air side; tubes run across the lateral width of 0.5 m. Plan: the top header covers the fin pack.
# Inlet stub at the left (+) end of the top header, outlet stub at the right (-) end of the bottom header.
NEAR = [0.06 + 0.04 * i for i in range(11)]

front50 = [
    ellipse("inlet-stub", "frame", "body", 3.8, 0.4, 0.2, 0.51, 0.006, 0.006, region="headers", material="copper"),
    ellipse("outlet-stub", "frame", "body", 3.8, 0.4, -0.2, 0.01, 0.006, 0.006, region="headers", material="copper"),
    rect("header-top", "frame", "body", 3.5, 0.4, -0.25, 0.5, 0.5, 0.02, region="headers", material="copper", rr=0.01),
    rect("header-bottom", "frame", "body", 3.5, 0.4, -0.25, 0, 0.5, 0.02, region="headers", material="copper", rr=0.01),
] + [
    rect("tube-%d" % (i + 1), "tubes", "body", 3.6, 0.5, -0.23, cy - 0.009, 0.46, 0.018, region="tubes", material="copper") for i, cy in enumerate(NEAR)
] + [
    rect("end-plate-left", "fins", "body", 3.4, 0.6, 0.23, 0.02, 0.02, 0.48, region="fins", material="aluminium-fin"),
    rect("end-plate-right", "fins", "body", 3.4, 0.6, -0.25, 0.02, 0.02, 0.48, region="fins", material="aluminium-fin"),
    rect("fin-pack", "fins", "body", 3.3, 0.7, -0.23, 0.02, 0.46, 0.48, region="fins", material="aluminium-fin"),
]
plan50 = [
    rect("inlet-stub", "frame", "body", 3.4, 0.4, 0.075, 0.194, 0.055, 0.012, region="headers", material="copper"),
    rect("outlet-stub", "frame", "body", 3.4, 0.4, 0.075, -0.206, 0.055, 0.012, region="headers", material="copper"),
    rect("header-top", "frame", "body", 3.5, 0.4, -0.075, -0.25, 0.15, 0.5, region="headers", material="copper", rr=0.01),
]
front100 = [
    ellipse("inlet-stub", "frame", "body", 3.8, 0.4, 0.2, 0.51, 0.01, 0.01, region="headers", material="copper"),
    ellipse("outlet-stub", "frame", "body", 3.8, 0.4, -0.2, 0.01, 0.01, 0.01, region="headers", material="copper"),
    rect("header-top", "frame", "body", 3.5, 0.4, -0.25, 0.495, 0.5, 0.025, region="headers", material="copper"),
    rect("header-bottom", "frame", "body", 3.5, 0.4, -0.25, 0, 0.5, 0.025, region="headers", material="copper"),
] + [
    rect("tube-%d" % (i + 1), "tubes", "body", 3.6, 0.5, -0.23, cy - 0.011, 0.46, 0.022, region="tubes", material="copper") for i, cy in enumerate(NEAR)
] + [
    rect("fin-pack", "fins", "body", 3.3, 0.7, -0.25, 0.02, 0.5, 0.48, region="fins", material="aluminium-fin"),
]
plan100 = [
    rect("inlet-stub", "frame", "body", 3.4, 0.4, 0.075, 0.19, 0.055, 0.02, region="headers", material="copper"),
    rect("outlet-stub", "frame", "body", 3.4, 0.4, 0.075, -0.21, 0.055, 0.02, region="headers", material="copper"),
    rect("header-top", "frame", "body", 3.5, 0.4, -0.075, -0.25, 0.15, 0.5, region="headers", material="copper"),
]
front200 = [
    ellipse("inlet-stub", "frame", "body", 3.8, 0.4, 0.2, 0.505, 0.015, 0.015, region="headers", material="copper"),
    ellipse("outlet-stub", "frame", "body", 3.8, 0.4, -0.2, 0.015, 0.015, 0.015, region="headers", material="copper"),
    rect("header-top", "frame", "body", 3.5, 0.4, -0.25, 0.48, 0.5, 0.04, region="headers", material="copper"),
    rect("header-bottom", "frame", "body", 3.5, 0.4, -0.25, 0, 0.5, 0.04, region="headers", material="copper"),
    rect("fin-pack", "fins", "body", 3.3, 0.6, -0.25, 0.03, 0.5, 0.46, regions=["fins", "tubes"], material="aluminium-fin"),
]
plan200 = [
    rect("inlet-stub", "frame", "body", 3.4, 0.4, 0.075, 0.185, 0.06, 0.03, region="headers", material="copper"),
    rect("outlet-stub", "frame", "body", 3.4, 0.4, 0.075, -0.215, 0.06, 0.03, region="headers", material="copper"),
    rect("header-top", "frame", "body", 3.5, 0.4, -0.075, -0.25, 0.15, 0.5, regions=["headers", "fins", "tubes"], material="copper"),
]

apply("finned-heat-exchanger", {
    "50": {"front": front50, "plan": plan50, "dec": {
        "retained": (
            "Front view from the air side: both headers with the two stubs seen end-on (inlet left, outlet right), the fin face between two end plates and the near column of 11 tube runs across the face, drawn as if the first fin were lifted. Plan: the top header and the two stubs; the fin pack lies below the header.",
            "Vorderansicht von der Luftseite: beide Sammler mit den beiden Stutzen in der Stirnansicht (Eintritt links, Austritt rechts), die Lamellenfläche zwischen zwei Endplatten und die nahe Reihe aus 11 Rohrläufen über der Fläche, gezeichnet, als sei die erste Lamelle abgehoben. Grundriss: der obere Sammler und die beiden Stutzen; das Lamellenpaket liegt unter dem Sammler."),
        "omitted": (
            "The second tube column is hidden behind the near one; in the plan the tubes and fins are hidden below the top header, and so is the outlet header.",
            "Die zweite Rohrreihe ist hinter der nahen verdeckt; im Grundriss sind Rohre und Lamellen unter dem oberen Sammler verdeckt, ebenso der untere Sammler."),
    }},
    "100": {"front": front100, "plan": plan100, "dec": {
        "retained": (
            "Front view: fin block with end plates merged into it, 11 tube runs of the near column, both headers and the two stubs end-on. Plan: top header and the two stubs.",
            "Vorderansicht: Lamellenblock mit eingeschmolzenen Endplatten, 11 Rohrläufe der nahen Reihe, beide Sammler und die zwei Stutzen in der Stirnansicht. Grundriss: oberer Sammler und die zwei Stutzen."),
        "merged": (
            "The end plates join the fin block in the front view.",
            "Die Endplatten gehen in der Vorderansicht im Lamellenblock auf."),
    }},
    "200": {"front": front200, "plan": plan200, "dec": {
        "retained": (
            "Front view: one fin block between two thick headers with the stubs end-on. Plan: one header block with the two stubs.",
            "Vorderansicht: ein Lamellenblock zwischen zwei kräftigen Sammlern mit den Stutzen in der Stirnansicht. Grundriss: ein Sammlerblock mit den zwei Stutzen."),
        "merged": (
            "In the plan the header block lists headers, fins and tubes, because the fin pack lies below it; in the front view the block carries fins and tubes.",
            "Im Grundriss führt der Sammlerblock Sammler, Lamellen und Rohre, weil das Lamellenpaket darunter liegt; in der Vorderansicht trägt der Block Lamellen und Rohre."),
    }},
})
