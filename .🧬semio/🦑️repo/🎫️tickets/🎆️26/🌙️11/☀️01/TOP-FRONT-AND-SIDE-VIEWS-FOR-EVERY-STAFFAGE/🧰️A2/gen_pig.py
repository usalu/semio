import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = Asset("pig")
N, Fr = -1, 1  # near = right = -lateral, far = left = +lateral


def legs_front(F, scale, detail):
    pad = 0.0 if scale <= 100 else 0.03
    for sd, t in ((Fr, "f"), (N, "n")):
        zh, zf = 1.0, 5.0
        hl, fl = sd * 0.13, sd * 0.1
        if detail >= 2:
            F.limb_front("gaskin-" + t, "shank-h" + t, hl, 0.07, 0.056, like="gaskin-" + t, z=zh + 0.2)
            F.limb_front("cannon-h-" + t, "metatarsus-h" + t, hl, 0.05, 0.044, over=pad, under=pad, like="cannon-h-" + t, z=zh + 0.04)
            F.limb_front("forearm-" + t, "forearm-f" + t, fl, 0.08, 0.06, over=pad, under=pad, like="forearm-" + t, z=zf + 0.05)
            F.limb_front("cannon-f-" + t, "metacarpus-f" + t, fl, 0.052, 0.046, over=pad, under=pad, like="cannon-f-" + t, z=zf + 0.04)
            F.limb_front("hoof-h-" + t, "hoof-h" + t, hl, 0.058, 0.075, over=pad, like="hoof-h-" + t, z=zh + 0.06)
            F.limb_front("hoof-f-" + t, "hoof-f" + t, fl, 0.058, 0.075, over=pad, like="hoof-f-" + t, z=zf + 0.06)
        else:
            F.limb_front("leg-hind-upper-" + t, "shank-h" + t, hl, 0.07, 0.05, like="leg-hind-upper-n", z=zh)
            F.limb_front("leg-hind-lower-" + t, "metatarsus-h" + t, hl, 0.05, 0.062, like="leg-hind-lower-n", z=zh + 0.04, under=0.055)
            F.limb_front("leg-fore-upper-" + t, "forearm-f" + t, fl, 0.08, 0.056, like="leg-fore-upper-n", z=zf + 0.05)
            F.limb_front("leg-fore-lower-" + t, "metacarpus-f" + t, fl, 0.052, 0.064, like="leg-fore-lower-n", z=zf + 0.06, under=0.056)


def build50():
    F, P = View(A, 50, "front"), View(A, 50, "plan")
    legs_front(F, 50, 2)
    F.ell("hindbody", "lumbar", 0, 0.468, 0.208, 0.262, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.469, 0.196, 0.266, like="chest", z=3.2)
    F.ell("neck", "neck", 0, 0.62, 0.15, 0.14, like="neck", z=2.9)
    F.ell("jaw", "jaw", 0, 0.47, 0.065, 0.075, like="jaw", z=3.9)
    F.ell("head", "head", 0, 0.585, 0.095, 0.175, like="head", z=4)
    F.ell("snout", "head", 0, 0.428, 0.048, 0.04, like="snout", z=4.2)
    F.ell("ear-f", "ear-f", 0.118, 0.655, 0.058, 0.08, rot=-20, like="ear-f", z=4.5)
    F.ell("ear-n", "ear-n", -0.118, 0.655, 0.058, 0.08, rot=20, like="ear-n", z=4.5)
    F.ell("eye", "head", -0.07, 0.603, 0.012, 0.012, like="eye", z=4.6, role="detail", filled=True)
    F.ell("eye-f", "head", 0.07, 0.603, 0.012, 0.012, like="eye", z=4.6, role="detail", filled=True)
    F.ell("tail-curl", "tail-2", 0.0, 0.72, 0.025, 0.035, like="tail-curl", z=2.8)
    # plan
    for sd, t in ((Fr, "f"), (N, "n")):
        z = 1.0 if sd == Fr else 1.1
        P.limb_plan("gaskin-" + t, "shank-h" + t, sd * 0.11, 0.046, 0.036, like="gaskin-" + t, z=z + 0.2)
        P.limb_plan("cannon-h-" + t, "metatarsus-h" + t, sd * 0.11, 0.034, 0.03, like="cannon-h-" + t, z=z + 0.04)
        P.limb_plan("hoof-h-" + t, "hoof-h" + t, sd * 0.11, 0.04, 0.036, like="hoof-h-" + t, z=z + 0.06)
        P.limb_plan("forearm-" + t, "forearm-f" + t, sd * 0.1, 0.05, 0.04, like="forearm-" + t, z=z + 0.05)
        P.limb_plan("cannon-f-" + t, "metacarpus-f" + t, sd * 0.1, 0.034, 0.03, like="cannon-f-" + t, z=z + 0.04)
        P.limb_plan("hoof-f-" + t, "hoof-f" + t, sd * 0.1, 0.04, 0.036, like="hoof-f-" + t, z=z + 0.06)
    P.blob("hindbody", "lumbar", profile([(-0.138, 0), (-0.12, 0.1), (-0.06, 0.18), (0.02, 0.209), (0.15, 0.21), (0.3, 0.2), (0.45, 0.195), (0.56, 0.2)]), like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.5, 0.195), (0.6, 0.208), (0.7, 0.2), (0.77, 0.15), (0.795, 0.09), (0.8, 0)]), like="chest", z=3.2)
    P.cap("neck", "neck", 0.672, 0, 0.82, 0, 0.14, 0.12, follow=True, like="neck", z=2.9)
    P.cap("head", "head", 0.8, 0, 1.021, 0, 0.1, 0.045, follow=True, like="head", z=4)
    P.cap("jaw", "jaw", 0.85, 0, 0.978, 0, 0.06, 0.04, follow=True, like="jaw", z=3.9)
    P.cap("ear-f", "ear-f", 0.9, 0.105, 1.03, 0.115, 0.05, 0.035, follow=True, like="ear-f", z=4.5)
    P.cap("ear-n", "ear-n", 0.9, -0.105, 1.03, -0.115, 0.05, 0.035, follow=True, like="ear-n", z=4.5)
    P.ell("eye", "head", 0.9, -0.075, 0.012, 0.012, like="eye", z=4.6, role="detail", filled=True)
    P.ell("eye-f", "head", 0.9, 0.075, 0.012, 0.012, like="eye", z=4.6, role="detail", filled=True)
    P.ell("tail-root", "tail-1", -0.045, 0.0, 0.012, 0.016, like="tail-root", z=2.8)
    P.ell("tail-curl", "tail-2", 0.0, 0.0, 0.045, 0.02, like="tail-curl", z=2.8)
    return F, P


def build100():
    F, P = View(A, 100, "front"), View(A, 100, "plan")
    legs_front(F, 100, 2)
    F.ell("hindbody", "lumbar", 0, 0.468, 0.208, 0.262, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.469, 0.196, 0.266, like="chest", z=3.2)
    F.ell("neck", "neck", 0, 0.62, 0.15, 0.14, like="neck", z=2.9)
    F.ell("head", "head", 0, 0.585, 0.095, 0.175, like="head", z=4)
    F.ell("snout", "head", 0, 0.428, 0.048, 0.04, like="snout", z=4.2)
    F.ell("ear-f", "ear-f", 0.118, 0.655, 0.058, 0.08, rot=-20, like="ear-f", z=4.5)
    F.ell("ear-n", "ear-n", -0.118, 0.655, 0.058, 0.08, rot=20, like="ear-n", z=4.5)
    P.ell("tail", "tail-1", -0.02, 0.0, 0.05, 0.02, like="tail", z=2.8)
    for sd, t in ((Fr, "f"), (N, "n")):
        z = 1.0 if sd == Fr else 1.1
        P.limb_plan("gaskin-" + t, "shank-h" + t, sd * 0.11, 0.046, 0.036, like="gaskin-" + t, z=z + 0.2)
        P.limb_plan("cannon-h-" + t, "metatarsus-h" + t, sd * 0.11, 0.034, 0.03, like="cannon-h-" + t, z=z + 0.04)
        P.limb_plan("hoof-h-" + t, "hoof-h" + t, sd * 0.11, 0.04, 0.036, like="hoof-h-" + t, z=z + 0.06)
        P.limb_plan("forearm-" + t, "forearm-f" + t, sd * 0.1, 0.05, 0.04, like="forearm-" + t, z=z + 0.05)
        P.limb_plan("cannon-f-" + t, "metacarpus-f" + t, sd * 0.1, 0.034, 0.03, like="cannon-f-" + t, z=z + 0.04)
        P.limb_plan("hoof-f-" + t, "hoof-f" + t, sd * 0.1, 0.04, 0.036, like="hoof-f-" + t, z=z + 0.06)
    P.blob("hindbody", "lumbar", profile([(-0.138, 0), (-0.12, 0.1), (-0.06, 0.18), (0.02, 0.209), (0.15, 0.21), (0.3, 0.2), (0.45, 0.195), (0.56, 0.2)]), like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.5, 0.195), (0.6, 0.208), (0.7, 0.2), (0.77, 0.15), (0.795, 0.09), (0.8, 0)]), like="chest", z=3.2)
    P.cap("neck", "neck", 0.672, 0, 0.82, 0, 0.14, 0.12, follow=True, like="neck", z=2.9)
    P.cap("head", "head", 0.8, 0, 1.021, 0, 0.1, 0.045, follow=True, like="head", z=4)
    P.cap("ear-f", "ear-f", 0.9, 0.105, 1.03, 0.115, 0.05, 0.035, follow=True, like="ear-f", z=4.5)
    P.cap("ear-n", "ear-n", 0.9, -0.105, 1.03, -0.115, 0.05, 0.035, follow=True, like="ear-n", z=4.5)
    return F, P


def build200():
    F, P = View(A, 200, "front"), View(A, 200, "plan")
    legs_front(F, 200, 2)
    F.ell("hindbody", "lumbar", 0, 0.468, 0.21, 0.262, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.469, 0.198, 0.266, like="chest", z=3.2)
    F.ell("neck", "neck", 0, 0.62, 0.15, 0.14, like="neck", z=2.9)
    F.ell("head", "head", 0, 0.52, 0.1, 0.24, like="head", z=4)
    F.ell("ear", "ear-n", -0.118, 0.655, 0.058, 0.08, rot=20, like="ear", z=4.5)
    F.ell("ear-far", "ear-f", 0.118, 0.655, 0.058, 0.08, rot=-20, like="ear", z=0.5)
    for sd, t in ((Fr, "f"), (N, "n")):
        z = 1.0 if sd == Fr else 1.1
        P.limb_plan("gaskin-" + t, "shank-h" + t, sd * 0.11, 0.05, 0.04, like="gaskin-" + t, z=z + 0.2)
        P.limb_plan("cannon-h-" + t, "metatarsus-h" + t, sd * 0.11, 0.04, 0.036, like="cannon-h-" + t, z=z + 0.04)
        P.limb_plan("hoof-h-" + t, "hoof-h" + t, sd * 0.11, 0.045, 0.04, like="hoof-h-" + t, z=z + 0.06)
        P.limb_plan("forearm-" + t, "forearm-f" + t, sd * 0.1, 0.055, 0.045, like="forearm-" + t, z=z + 0.05)
        P.limb_plan("cannon-f-" + t, "metacarpus-f" + t, sd * 0.1, 0.04, 0.036, like="cannon-f-" + t, z=z + 0.04)
        P.limb_plan("hoof-f-" + t, "hoof-f" + t, sd * 0.1, 0.045, 0.04, like="hoof-f-" + t, z=z + 0.06)
    P.blob("hindbody", "lumbar", profile([(-0.138, 0), (-0.12, 0.1), (-0.06, 0.18), (0.02, 0.209), (0.15, 0.21), (0.3, 0.2), (0.45, 0.195), (0.56, 0.2)]), n=3, like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.5, 0.195), (0.6, 0.208), (0.7, 0.2), (0.77, 0.15), (0.795, 0.09), (0.8, 0)]), n=3, like="chest", z=3.2)
    P.cap("neck", "neck", 0.672, 0, 0.82, 0, 0.14, 0.12, follow=True, like="neck", z=2.9)
    P.cap("head", "head", 0.8, 0, 1.021, 0, 0.1, 0.045, follow=True, like="head", z=4)
    P.cap("ear", "ear-n", 0.9, -0.105, 1.03, -0.115, 0.05, 0.035, follow=True, like="ear", z=4.5)
    P.cap("ear-far", "ear-f", 0.9, 0.105, 1.03, 0.115, 0.05, 0.035, follow=True, like="ear", z=4.5)
    return F, P


def build500():
    F, P = View(A, 500, "front"), View(A, 500, "plan")
    legs_front(F, 500, 1)
    F.ell("hindbody", "lumbar", 0, 0.468, 0.21, 0.262, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.469, 0.2, 0.266, like="chest", z=3.2)
    F.ell("neck", "neck", 0, 0.62, 0.15, 0.14, like="neck", z=2.9)
    F.ell("head", "head", 0, 0.52, 0.11, 0.24, like="head", z=4)
    P.blob("hindbody", "lumbar", profile([(-0.138, 0), (-0.1, 0.12), (0.02, 0.208), (0.3, 0.2), (0.56, 0.2)]), n=2, like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.5, 0.195), (0.62, 0.208), (0.75, 0.18), (0.8, 0)]), n=2, like="chest", z=3.2)
    P.cap("neck", "neck", 0.672, 0, 0.82, 0, 0.14, 0.12, follow=True, like="neck", z=2.9)
    P.cap("head", "head", 0.8, 0, 1.021, 0, 0.11, 0.05, follow=True, like="head", z=4)
    for sd, t in ((Fr, "f"), (N, "n")):
        z = 1.0 if sd == Fr else 1.1
        P.limb_plan("leg-hind-upper-" + t, "shank-h" + t, sd * 0.11, 0.06, 0.05, like="leg-hind-upper-n", z=z)
        P.limb_plan("leg-hind-lower-" + t, "metatarsus-h" + t, sd * 0.11, 0.05, 0.05, like="leg-hind-lower-n", z=z + 0.04)
        P.limb_plan("leg-fore-upper-" + t, "forearm-f" + t, sd * 0.1, 0.06, 0.05, like="leg-fore-upper-n", z=z + 0.05)
        P.limb_plan("leg-fore-lower-" + t, "metacarpus-f" + t, sd * 0.1, 0.05, 0.05, like="leg-fore-lower-n", z=z + 0.06)
    return F, P


BUILD = {50: build50, 100: build100, 200: build200, 500: build500}

NOTES = {
    50: ("Front view: head-on at full articulation: barrel in two masses (ham mass behind, chest in front), short neck, broad head with the snout disc, jaw, two drooping ears and both eyes; each foreleg and hind leg as forearm or gaskin, cannon and hoof wedge, bound with followTip so bent knees shorten and lifted legs rise. Plan view: back seen from above in two masses with the rump curve, neck and head wedge with the snout end, jaw, both ears and eyes, the curled tail as a small loop; the legs lie under the barrel and appear as strokes only when a stride swings them out.",
         "Vorderansicht: von vorn in voller Gliederung: Tonne in zwei Massen (Schinken hinten, Brust vorn), kurzer Hals, breiter Kopf mit Rüsselscheibe, Kiefer, zwei hängende Ohren und beide Augen; Vorder- und Hinterbeine je als Unterarm oder Unterschenkel, Röhre und Klauenkeil, mit followTip gebunden, sodass gebeugte Gelenke kürzer werden und gehobene Beine steigen. Draufsicht: der Rücken von oben in zwei Massen mit Kruppenrundung, Hals und Kopfkeil mit Rüsselende, Kiefer, beide Ohren und Augen, der Ringelschwanz als kleine Schlaufe; die Beine liegen unter der Tonne und erscheinen nur als Striche, wenn ein Schritt sie herausschwingt."),
    100: ("Front view: articulated head-on: two barrel masses, neck, head with snout disc and both ears, and each leg in forearm or gaskin, cannon and hoof wedge. Plan view: back in two masses, neck and head wedge, both ears, the tail as one loop; legs as strokes under the barrel.",
          "Vorderansicht: gegliedert von vorn: zwei Tonnenmassen, Hals, Kopf mit Rüsselscheibe und beiden Ohren, jedes Bein aus Unterarm oder Unterschenkel, Röhre und Klauenkeil. Draufsicht: Rücken in zwei Massen, Hals und Kopfkeil, beide Ohren, der Schwanz als eine Schlaufe; Beine als Striche unter der Tonne."),
    200: ("Front view: main masses head-on: barrel in two parts, neck, one head mass, the ears as one edge stroke and legs as lower leg masses. Plan view: back in two masses, neck, head wedge, one ear stroke and the leg masses under the barrel.",
          "Vorderansicht: Hauptmassen von vorn: Tonne in zwei Teilen, Hals, eine Kopfmasse, die Ohren als eine Randlinie und Beine als untere Beinmassen. Draufsicht: Rücken in zwei Massen, Hals, Kopfkeil, ein Ohrstrich und die Beinmassen unter der Tonne."),
    500: ("Front view: one composed silhouette head-on: barrel, neck and head merged into three smooth masses, each leg as two strokes. Plan view: back and chest as two masses, neck and head wedge; the leg strokes lie below them.",
          "Vorderansicht: eine zusammengesetzte Silhouette von vorn: Tonne, Hals und Kopf in drei glatten Massen, jedes Bein als zwei Striche. Draufsicht: Rücken und Brust als zwei Massen, Hals und Kopfkeil; die Beinstriche liegen darunter."),
}
