import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

# World = rig rest pose, root on the ground. Front x = lateral (left +x), plan x = depth axis, plan y = lateral (left +y).
# Casing 0.45 m deep x 1.10 m wide x 1.41 m high; fan disc (d 0.70) 0.12 m left of the centre line at 0.80 m.
# The lid (hinged at the back) and the discharge vane are extruded solids: their side-view profile, placed with the full bone
# transform, spans the lateral width, so the lid plate stands up in the front view and shrinks in the plan when it opens.
CF = 0.12


def lid(t, z=2.5):
    return X(poly("lid", "lid", "body", z, 0.35, [(0, -t), (0.47, -t), (0.47, 0), (0, 0)], region="top-lid", material="powder-coat"), -0.56, 0.56)


def vane(h, z=2.6):
    return X(poly("vane", "vane", "body", z, 0.3, [(-0.02, -h), (0.22, -h), (0.22, h), (-0.02, h)], region="discharge-vane", material="powder-coat"), CF - 0.36, CF + 0.36)


def spokes(prefix, z, bone="unit"):
    out = []
    for name, a in (("v", 90), ("d1", 45), ("d2", 135)):
        c, s = math.cos(math.radians(a)), math.sin(math.radians(a))
        out.append(poly("%s-%s" % (prefix, name), bone, "detail", z, 0.3, [(CF - 0.34 * c, 0.8 - 0.34 * s), (CF + 0.34 * c, 0.8 + 0.34 * s)], closed=False, region="fan-guard", material="galvanised-steel"))
    return out


def blades(n, z=2.33, tone=0.5, sweep=0.55, half=0.2, r0=0.075, r1=0.31, steps=6):
    """Static swept fan blades: blade k leaves the hub at angle 2*pi*k/n, its centre line sweeps by `sweep` rad towards the tip, its half width in angle widens to the middle and narrows to the tip."""
    out = []
    for k in range(n):
        a0 = 2 * math.pi * k / n + 0.35
        lead, trail = [], []
        for i in range(steps + 1):
            t = i / steps
            rr = r0 + (r1 - r0) * t
            w = half * math.sin(math.pi * (0.25 + 0.75 * t)) * (1 - 0.25 * t)
            c = a0 + sweep * t
            lead.append((CF + rr * math.cos(c + w), 0.8 + rr * math.sin(c + w)))
            trail.append((CF + rr * math.cos(c - w), 0.8 + rr * math.sin(c - w)))
        out.append(poly("fan-blade-%d" % (k + 1), "unit", "detail", z, tone, lead + list(reversed(trail)), region="fan-guard", material="aluminium", filled=False))
    return out


# ---- 1:50
front50 = blades(5, half=0.24, sweep=0.6) + [lid(0.04), vane(0.025),
    ellipse("fan-hub", "unit", "detail", 2.55, 0.15, CF, 0.8, 0.07, 0.07, region="fan-guard", material="galvanised-steel", filled=False),
] + [
    ellipse("fan-ring-inner", "unit", "detail", 2.35, 0.3, CF, 0.8, 0.19, 0.19, region="fan-guard", material="galvanised-steel", filled=False),
    ellipse("fan-ring-outer", "unit", "detail", 2.35, 0.3, CF, 0.8, 0.27, 0.27, region="fan-guard", material="galvanised-steel", filled=False),
    ellipse("fan-guard", "unit", "body", 2.3, 0.3, CF, 0.8, 0.35, 0.35, region="fan-guard", material="galvanised-steel"),
    rect("compressor-panel", "unit", "body", 2.2, 0.4, -0.55, 0.15, 1.1, 0.17, region="compressor-casing", material="powder-coat"),
    rect("base-tray", "unit", "body", 2.1, 0.3, -0.55, 0.08, 1.1, 0.07, region="base-tray", material="galvanised-steel"),
    rect("casing", "unit", "body", 2.0, 0.55, -0.55, 0.08, 1.1, 1.33, region="casing", material="powder-coat"),
    rect("foot-left", "unit", "body", 1.5, 0.2, 0.34, 0, 0.18, 0.09, region="feet", material="rubber"),
    rect("foot-right", "unit", "body", 1.5, 0.2, -0.52, 0, 0.18, 0.09, region="feet", material="rubber"),
]
plan50 = [lid(0.04), vane(0.025),
    rect("fan-guard", "unit", "body", 2.2, 0.3, 0.225, CF - 0.35, 0.035, 0.7, region="fan-guard", material="galvanised-steel"),
    rect("casing", "unit", "body", 2.0, 0.55, -0.225, -0.55, 0.45, 1.1, region="casing", material="powder-coat"),
]
# ---- 1:100
front100 = blades(5, half=0.26, steps=4) + [lid(0.04), vane(0.025),
    ellipse("fan-hub", "unit", "detail", 2.55, 0.15, CF, 0.8, 0.08, 0.08, region="fan-guard", material="galvanised-steel", filled=False),
    ellipse("fan-ring", "unit", "detail", 2.35, 0.3, CF, 0.8, 0.24, 0.24, region="fan-guard", material="galvanised-steel", filled=False),
    ellipse("fan-guard", "unit", "body", 2.3, 0.3, CF, 0.8, 0.35, 0.35, region="fan-guard", material="galvanised-steel"),
    rect("compressor-panel", "unit", "body", 2.2, 0.4, -0.55, 0.15, 1.1, 0.17, region="compressor-casing", material="powder-coat"),
    rect("base-tray", "unit", "body", 2.1, 0.3, -0.55, 0.08, 1.1, 0.07, region="base-tray", material="galvanised-steel"),
    rect("casing", "unit", "body", 2.0, 0.55, -0.55, 0.08, 1.1, 1.33, region="casing", material="powder-coat"),
    rect("foot-left", "unit", "body", 1.5, 0.2, 0.34, 0, 0.18, 0.09, region="feet", material="rubber"),
    rect("foot-right", "unit", "body", 1.5, 0.2, -0.52, 0, 0.18, 0.09, region="feet", material="rubber"),
]
plan100 = [lid(0.04), vane(0.025),
    rect("fan-guard", "unit", "body", 2.2, 0.3, 0.225, CF - 0.35, 0.035, 0.7, region="fan-guard", material="galvanised-steel"),
    rect("casing", "unit", "body", 2.0, 0.55, -0.225, -0.55, 0.45, 1.1, region="casing", material="powder-coat"),
]
# ---- 1:200
front200 = [lid(0.05), vane(0.03),
    ellipse("fan-guard", "unit", "body", 2.3, 0.3, CF, 0.8, 0.35, 0.35, region="fan-guard", material="galvanised-steel"),
    rect("compressor-panel", "unit", "body", 2.2, 0.4, -0.55, 0.08, 1.1, 0.24, region="compressor-casing", material="powder-coat"),
    rect("base-tray", "unit", "body", 2.1, 0.3, -0.55, 0.08, 1.1, 0.07, region="base-tray", material="galvanised-steel"),
    rect("casing", "unit", "body", 2.0, 0.55, -0.55, 0, 1.1, 1.41, regions=["casing", "feet"], material="powder-coat"),
]
plan200 = [lid(0.05), vane(0.03),
    rect("fan-guard", "unit", "body", 2.2, 0.3, 0.225, CF - 0.35, 0.035, 0.7, region="fan-guard", material="galvanised-steel"),
    rect("casing", "unit", "body", 2.0, 0.55, -0.225, -0.55, 0.45, 1.1, regions=["casing", "feet", "compressor-casing", "base-tray"], material="powder-coat"),
]
# ---- 1:500
front500 = [lid(0.1), vane(0.06),
    ellipse("fan-disc", "unit", "body", 2.3, 0.3, CF, 0.8, 0.35, 0.35, region="fan-guard", material="galvanised-steel"),
    rect("casing", "unit", "body", 2.0, 0.5, -0.55, 0, 1.1, 1.35, regions=["casing", "coil-guard", "compressor-casing", "base-tray", "feet"], material="powder-coat"),
]
plan500 = [lid(0.1), vane(0.06),
    rect("casing", "unit", "body", 2.0, 0.5, -0.225, -0.55, 0.485, 1.1, regions=["casing", "coil-guard", "compressor-casing", "base-tray", "fan-guard", "feet"], material="powder-coat"),
]

apply("heat-pump-outdoor-unit", {
    "50": {"front": front50, "plan": plan50, "dec": {
        "retained": (
            "Front view: casing with base tray and compressor panel, two rubber feet, the round fan guard with two rings and hub in front of five swept fan blades, the discharge louvre as a bar that rises and falls with its vane, and the lid plate that stands up when the lid opens. Plan: casing top, the lid plate that shortens towards the hinge as it opens, the fan bulge and the louvre. Lid and louvre are extruded solids of their side-view profile.",
            "Vorderansicht: Gehäuse mit Bodenwanne und Verdichterblende, zwei Gummifüße, das runde Ventilatorgitter mit zwei Ringen und Nabe vor fünf gepfeilten Ventilatorflügeln, die Ausblaslamelle als Balken, der mit der Lamelle steigt und fällt, und die Deckelplatte, die sich beim Öffnen aufstellt. Grundriss: Gehäusedeckfläche, die Deckelplatte, die beim Öffnen zum Scharnier hin kürzer wird, die Ventilatorwölbung und die Lamelle. Deckel und Lamelle sind extrudierte Körper ihres Seitenansichtsprofils."),
        "omitted": (
            "The fan blades are drawn static (they turn in operation); the coil guard on the side panels is edge-on in front and plan.",
            "Die Ventilatorflügel sind statisch gezeichnet (im Betrieb drehen sie sich); der Tauscherschutz an den Seitenwänden steht in Vorderansicht und Grundriss auf der Kante."),
    }},
    "100": {"front": front100, "plan": plan100, "dec": {
        "retained": (
            "Front view: casing, tray, compressor panel, two feet, fan guard with one ring and hub in front of five swept blades, louvre bar and the lid plate. Plan: casing top, lid plate, fan bulge and louvre.",
            "Vorderansicht: Gehäuse, Wanne, Verdichterblende, zwei Füße, Ventilatorgitter mit einem Ring und Nabe vor fünf gepfeilten Flügeln, Lamellenbalken und die Deckelplatte. Grundriss: Gehäusedeckfläche, Deckelplatte, Ventilatorwölbung und Lamelle."),
        "omitted": (
            "Fan spokes in the front view; the coil guard is edge-on.",
            "Ventilatorstreben in der Vorderansicht; der Tauscherschutz steht auf der Kante."),
    }},
    "200": {"front": front200, "plan": plan200, "dec": {
        "retained": (
            "Front view: casing down to the ground, compressor panel, tray band, fan disc, louvre bar and lid plate. Plan: casing top, lid plate, fan bulge and louvre.",
            "Vorderansicht: Gehäuse bis zum Boden, Verdichterblende, Wannenband, Ventilatorscheibe, Lamellenbalken und Deckelplatte. Grundriss: Gehäusedeckfläche, Deckelplatte, Ventilatorwölbung und Lamelle."),
        "merged": (
            "Feet merge into the casing in the front view; in the plan the casing top lists casing, feet, compressor panel and tray.",
            "Die Füße gehen in der Vorderansicht im Gehäuse auf; im Grundriss führt die Gehäusedeckfläche Gehäuse, Füße, Verdichterblende und Wanne."),
    }},
    "500": {"front": front500, "plan": plan500, "dec": {
        "retained": (
            "Front view: one casing form, the fan disc, the louvre bar (0.12 m, 0.24 mm) and the lid bar, both of which move. Plan: one casing form with the fan bulge, the lid plate and the louvre.",
            "Vorderansicht: eine Gehäuseform, die Ventilatorscheibe, der Lamellenbalken (0,12 m, 0,24 mm) und der Deckelbalken, die sich beide bewegen. Grundriss: eine Gehäuseform mit der Ventilatorwölbung, die Deckelplatte und die Lamelle."),
        "merged": (
            "Casing, coil guard, compressor panel, tray and feet are one form in both views; the fan bulge is part of the plan form.",
            "Gehäuse, Tauscherschutz, Verdichterblende, Wanne und Füße sind in beiden Ansichten eine Form; die Ventilatorwölbung gehört im Grundriss zur Form."),
    }},
})
