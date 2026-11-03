import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = Asset("red-squirrel")
FAR, NEAR = 1, -1
RUMP = profile([(-0.048, 0), (-0.044, 0.03), (-0.01, 0.056), (0.03, 0.06), (0.065, 0.056), (0.08, 0.04)])
CHEST = profile([(0.0, 0.04), (0.04, 0.05), (0.075, 0.048), (0.09, 0.03), (0.092, 0.0)])


def common(F, P, scale):
    n = 5 if scale <= 50 else 3
    F.rev("rump", "spine-r", lat=0.06, like="rump", z=3)
    F.rev("chest", "spine-f", lat=0.05, like="chest", z=3.1)
    F.rev("neck", "neck", lat=0.032, like="neck", z=3.3)
    F.rev("head", "head", lat=0.032, like="head", z=4.5)
    F.ell("ear", "head", -0.027, 0.256, 0.009, 0.022, like="ear", z=4.4)
    F.ell("ear-far", "head", 0.027, 0.256, 0.009, 0.022, like="ear", z=4.4)
    F.rev("tail-1", "tail-1", lat=0.045, like="tail-1", z=0.5)
    F.rev("tail-2", "tail-2", lat=0.05, like="tail-2", z=0.4)
    P.rev("tail-1", "tail-1", lat=0.045, like="tail-1", z=3.5)
    P.rev("tail-2", "tail-2", lat=0.05, like="tail-2", z=3.6)
    P.rev("rump", "spine-r", lat=0.06, like="rump", z=3)
    P.rev("chest", "spine-f", lat=0.05, like="chest", z=3.1)
    P.rev("neck", "neck", lat=0.032, like="neck", z=3.3)
    P.rev("head", "head", lat=0.032, like="head", z=4.5)
    P.ell("ear", "head", 0.062, -0.027, 0.011, 0.008, like="ear", z=4.4)
    P.ell("ear-far", "head", 0.062, 0.027, 0.011, 0.008, like="ear", z=4.4)


def build50():
    F, P = View(A, 50, "front"), View(A, 50, "plan")
    common(F, P, 50)
    F.ell("belly-patch", "spine-f", 0, 0.14, 0.024, 0.055, like="belly-patch", z=3.2)
    for sd, t, tag, z in ((FAR, "f", "far", 1.0), (NEAR, "n", "near", 4.0)):
        F.limb_front(tag + "-arm", "arm-" + t, sd * 0.034, 0.028, 0.022, like=tag + "-arm", z=z)
        F.limb_front(tag + "-forearm", "forearm-" + t, sd * 0.032, 0.02, 0.02, like=tag + "-forearm", z=z)
        F.limb_front(tag + "-hand", "hand-" + t, sd * 0.03, 0.018, 0.022, like=tag + "-hand", z=z + 0.1)
        F.ell(tag + "-thigh", "thigh-" + t, sd * 0.04, 0.045, 0.02, 0.03, like=tag + "-thigh", z=z + 0.2)
        F.limb_front(tag + "-shank", "shank-" + t, sd * 0.04, 0.03, 0.02, like=tag + "-shank", z=z + 0.2)
        F.ell(tag + "-foot", "foot-" + t, sd * 0.04, 0.01, 0.022, 0.01, like=tag + "-foot", z=z + 0.2)
        P.limb_plan(tag + "-arm", "arm-" + t, sd * 0.036, 0.012, 0.011, like=tag + "-arm", z=z)
        P.limb_plan(tag + "-forearm", "forearm-" + t, sd * 0.036, 0.011, 0.01, like=tag + "-forearm", z=z)
        P.limb_plan(tag + "-hand", "hand-" + t, sd * 0.036, 0.011, 0.008, like=tag + "-hand", z=z + 0.1)
        P.limb_plan(tag + "-thigh", "thigh-" + t, sd * 0.038, 0.024, 0.02, like=tag + "-thigh", z=z + 0.2)
        P.limb_plan(tag + "-shank", "shank-" + t, sd * 0.04, 0.02, 0.014, like=tag + "-shank", z=z + 0.2)
        P.limb_plan(tag + "-foot", "foot-" + t, sd * 0.04, 0.014, 0.012, like=tag + "-foot", z=z + 0.2)
    P.ell("belly-patch", "spine-f", 0.072, 0.0, 0.02, 0.02, like="belly-patch", z=3.2)
    return F, P


def build100():
    F, P = View(A, 100, "front"), View(A, 100, "plan")
    common(F, P, 100)
    return F, P


BUILD = {50: build50, 100: build100}

NOTES = {
    50: ("Front view: the squirrel head-on at full articulation: oval haunch and chest with the pale belly patch, neck, head with two ear tufts, the tail in two extruded masses behind the body that follow the tail bones, and each arm, forearm, hand, thigh, shank and foot, bound with followTip so reaching, climbing and bounding limbs read correctly. Plan view: a spindle back in two masses, neck, head wedge, ears, the tail in two links over the rump, and all limbs as capsules that stretch along their bones.",
         "Vorderansicht: das Eichhörnchen von vorn in voller Gliederung: ovale Keule und Brust mit dem hellen Bauchfleck, Hals, Kopf mit zwei Ohrpinseln, der Schwanz in zwei extrudierten Massen hinter dem Körper, die den Schwanzknochen folgen, sowie Arm, Unterarm, Hand, Schenkel, Unterschenkel und Fuß, mit followTip gebunden, damit greifende, kletternde und springende Gliedmaßen richtig lesbar sind. Draufsicht: ein Spindelrücken in zwei Massen, Hals, Kopfkeil, Ohren, der Schwanz in zwei Gliedern über der Kruppe und alle Gliedmaßen als Kapseln, die sich entlang ihrer Knochen dehnen."),
    100: ("Front view: articulated head-on in the same bone split as the side view: haunch and chest ovals (limbs merged into them), neck, head with two ears and the tail in two extruded masses. Plan view: spindle back in two masses, neck, head wedge, ears and the tail in two links.",
          "Vorderansicht: gegliedert von vorn mit derselben Knochenaufteilung wie die Seitenansicht: Keulen- und Brustoval (Gliedmaßen darin zusammengefasst), Hals, Kopf mit zwei Ohren und der Schwanz in zwei extrudierten Massen. Draufsicht: Spindelrücken in zwei Massen, Hals, Kopfkeil, Ohren und der Schwanz in zwei Gliedern."),
}
