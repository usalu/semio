import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = Asset("sheep")

HIND = profile([(-0.152, 0), (-0.14, 0.1), (-0.08, 0.19), (0.0, 0.218), (0.15, 0.22), (0.3, 0.215), (0.45, 0.21), (0.56, 0.21)])
CHEST = profile([(0.5, 0.205), (0.56, 0.212), (0.65, 0.212), (0.72, 0.17), (0.755, 0.09), (0.763, 0.0)])


def rows(scale):
    k = 1.0 if scale <= 100 else 1.2
    return [
        dict(id="gaskin-{s}", bone="shank-h{s}", flat=0.118, fw=(0.09, 0.07), over=(0.0 if scale <= 100 else 0.02), under=(0.0 if scale <= 100 else 0.02), fz=1.2, plat=0.115, pr=(0.055 * k, 0.04 * k), pz=1.2),
        dict(id="cannon-h-{s}", bone="metatarsus-h{s}", flat=0.118, fw=(0.04, 0.036), fz=1.04, plat=0.115, pr=(0.026 * k, 0.024 * k), pz=1.04),
        dict(id="hoof-h-{s}", bone="hoof-h{s}", flat=0.118, fw=(0.048, 0.062), fz=1.06, plat=0.115, pr=(0.03 * k, 0.026 * k), pz=1.06),
        dict(id="forearm-{s}", bone="forearm-f{s}", flat=0.1, fw=(0.1, 0.06), fz=5.05, plat=0.1, pr=(0.05 * k, 0.034 * k), pz=1.05),
        dict(id="cannon-f-{s}", bone="metacarpus-f{s}", flat=0.1, fw=(0.042, 0.036), fz=5.04, plat=0.1, pr=(0.026 * k, 0.024 * k), pz=1.04),
        dict(id="hoof-f-{s}", bone="hoof-f{s}", flat=0.1, fw=(0.048, 0.062), fz=5.06, plat=0.1, pr=(0.03 * k, 0.026 * k), pz=1.06),
    ]


def rows500():
    return [
        dict(id="leg-hind-upper-{s}", like="leg-hind-upper-n", bone="shank-h{s}", flat=0.118, fz=1.0, plat=0.115, pz=1.0, over=0.03),
        dict(id="leg-hind-lower-{s}", like="leg-hind-lower-n", bone="metatarsus-h{s}", flat=0.118, fz=1.04, plat=0.115, pz=1.04, under=0.049),
        dict(id="leg-fore-upper-{s}", like="leg-fore-upper-n", bone="forearm-f{s}", flat=0.1, fz=5.0, plat=0.1, pz=1.05, over=0.03),
        dict(id="leg-fore-lower-{s}", like="leg-fore-lower-n", bone="metacarpus-f{s}", flat=0.1, fz=5.04, plat=0.1, pz=1.06, under=0.048),
    ]


def trunk(F, P, scale):
    n = 5 if scale <= 50 else 3
    F.ell("hindbody", "lumbar", 0, 0.542, 0.22, 0.234, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.54, 0.21, 0.243, like="chest", z=3.2)
    F.rev("neck", "neck", lat=0.085, like="neck", z=2.9)
    P.blob("hindbody", "lumbar", HIND, n=n, like="hindbody", z=3)
    P.blob("chest", "thorax", CHEST, n=n, like="chest", z=3.2)
    P.rev("neck", "neck", lat=0.085, like="neck", z=2.9)


def build50():
    F, P = View(A, 50, "front"), View(A, 50, "plan")
    limbs(F, P, rows(50))
    trunk(F, P, 50)
    F.ell("jaw", "jaw", 0, 0.65, 0.032, 0.055, like="jaw", z=3.9)
    F.ell("head", "head", 0, 0.703, 0.056, 0.121, like="head", z=4)
    F.ell("ear-f", "ear-f", 0.098, 0.768, 0.05, 0.022, rot=8, like="ear-f", z=4.5)
    F.ell("ear-n", "ear-n", -0.098, 0.768, 0.05, 0.022, rot=-8, like="ear-n", z=4.5)
    F.ell("eye", "head", -0.042, 0.734, 0.01, 0.01, like="eye", z=4.6, role="detail", filled=True)
    F.ell("eye-f", "head", 0.042, 0.734, 0.01, 0.01, like="eye", z=4.6, role="detail", filled=True)
    F.ell("tail-root", "tail-1", 0.0, 0.63, 0.04, 0.05, like="tail-root", z=2.8)
    P.cap("head", "head", 0.96, 0, 1.1, 0, 0.06, 0.02, follow=True, like="head", z=4)
    P.cap("jaw", "jaw", 0.967, 0, 1.046, 0, 0.035, 0.02, follow=True, like="jaw", z=3.9)
    P.cap("ear-f", "ear-f", 1.0, 0.055, 0.925, 0.13, 0.03, 0.02, follow=True, like="ear-f", z=4.5)
    P.cap("ear-n", "ear-n", 1.0, -0.055, 0.925, -0.13, 0.03, 0.02, follow=True, like="ear-n", z=4.5)
    P.ell("eye", "head", 1.023, -0.04, 0.01, 0.01, like="eye", z=4.6, role="detail", filled=True)
    P.ell("eye-f", "head", 1.023, 0.04, 0.01, 0.01, like="eye", z=4.6, role="detail", filled=True)
    P.ell("tail-root", "tail-1", -0.04, 0.0, 0.04, 0.032, like="tail-root", z=2.8)
    P.ell("tail-tip", "tail-2", -0.045, 0.0, 0.022, 0.018, like="tail-tip", z=2.8)
    return F, P


def build100():
    F, P = View(A, 100, "front"), View(A, 100, "plan")
    limbs(F, P, rows(100))
    trunk(F, P, 100)
    F.ell("head", "head", 0, 0.703, 0.056, 0.121, like="head", z=4)
    F.ell("ear-f", "ear-f", 0.098, 0.768, 0.05, 0.022, rot=8, like="ear-f", z=4.5)
    F.ell("ear-n", "ear-n", -0.098, 0.768, 0.05, 0.022, rot=-8, like="ear-n", z=4.5)
    F.ell("tail", "tail-1", 0.0, 0.63, 0.04, 0.05, like="tail", z=2.8)
    P.cap("head", "head", 0.96, 0, 1.1, 0, 0.06, 0.02, follow=True, like="head", z=4)
    P.cap("ear-f", "ear-f", 1.0, 0.055, 0.925, 0.13, 0.03, 0.02, follow=True, like="ear-f", z=4.5)
    P.cap("ear-n", "ear-n", 1.0, -0.055, 0.925, -0.13, 0.03, 0.02, follow=True, like="ear-n", z=4.5)
    P.ell("tail", "tail-1", -0.04, 0.0, 0.05, 0.034, like="tail", z=2.8)
    return F, P


def build200():
    F, P = View(A, 200, "front"), View(A, 200, "plan")
    limbs(F, P, rows(200))
    trunk(F, P, 200)
    F.ell("head", "head", 0, 0.703, 0.06, 0.121, like="head", z=4)
    F.ell("ear", "ear-n", -0.1, 0.768, 0.052, 0.024, rot=-8, like="ear", z=4.5)
    F.ell("ear-far", "ear-f", 0.1, 0.768, 0.052, 0.024, rot=8, like="ear", z=4.5)
    P.cap("head", "head", 0.96, 0, 1.1, 0, 0.062, 0.022, follow=True, like="head", z=4)
    P.cap("ear", "ear-n", 1.0, -0.055, 0.925, -0.13, 0.03, 0.02, follow=True, like="ear", z=4.5)
    P.cap("ear-far", "ear-f", 1.0, 0.055, 0.925, 0.13, 0.03, 0.02, follow=True, like="ear", z=4.5)
    P.cap("tail", "tail-1", -0.015, 0, -0.04, 0, 0.03, 0.03, follow=True, like="tail", z=2.8, filled=False)
    return F, P


def build500():
    F, P = View(A, 500, "front"), View(A, 500, "plan")
    limbs_line(F, P, rows500())
    F.ell("hindbody", "lumbar", 0, 0.542, 0.22, 0.234, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.54, 0.21, 0.243, like="chest", z=3.2)
    F.rev("neck", "neck", lat=0.11, like="neck", z=2.9)
    F.ell("head", "head", 0, 0.703, 0.07, 0.121, like="head", z=4)
    P.blob("hindbody", "lumbar", profile([(-0.152, 0), (-0.1, 0.15), (0.0, 0.218), (0.3, 0.215), (0.56, 0.21)]), n=2, like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.5, 0.205), (0.65, 0.212), (0.74, 0.14), (0.763, 0.0)]), n=2, like="chest", z=3.2)
    P.rev("neck", "neck", lat=0.085, like="neck", z=2.9)
    P.cap("head", "head", 0.96, 0, 1.1, 0, 0.06, 0.022, follow=True, like="head", z=4)
    return F, P


BUILD = {50: build50, 100: build100, 200: build200, 500: build500}

NOTES = {
    50: ("Front view: the sheep head-on at full articulation: fleece barrel as two ovals, extruded neck that follows the bone when the head is lowered, narrow face with jaw and eyes, two horizontal ears, the tail root behind; each foreleg and hind leg as woolly forearm or gaskin, cannon and hoof, bound with followTip. Plan view: the back in two fleece masses, neck, head wedge with jaw and eyes, ears spread sideways, the tail as two small ovals, legs as strokes under the fleece that show when a stride swings them out.",
         "Vorderansicht: das Schaf von vorn in voller Gliederung: Vlieststonne als zwei Ovale, extrudierter Hals, der dem Knochen folgt, wenn der Kopf gesenkt wird, schmales Gesicht mit Kiefer und Augen, zwei waagerechte Ohren, dahinter der Schwanzansatz; Vorder- und Hinterbeine je als wolliger Unterarm oder Unterschenkel, Röhre und Klaue, mit followTip gebunden. Draufsicht: der Rücken in zwei Vliesmassen, Hals, Kopfkeil mit Kiefer und Augen, seitlich abstehende Ohren, der Schwanz als zwei kleine Ovale, Beine als Striche unter dem Vlies, die erscheinen, wenn ein Schritt sie herausschwingt."),
    100: ("Front view: articulated head-on: two fleece ovals, extruded neck, head, two ears, tail and legs in forearm or gaskin, cannon and hoof. Plan view: back in two masses, neck, head wedge, ears, tail oval; legs as strokes under the fleece.",
          "Vorderansicht: gegliedert von vorn: zwei Vliesovale, extrudierter Hals, Kopf, zwei Ohren, Schwanz und Beine aus Unterarm oder Unterschenkel, Röhre und Klaue. Draufsicht: Rücken in zwei Massen, Hals, Kopfkeil, Ohren, Schwanzoval; Beine als Striche unter dem Vlies."),
    200: ("Front view: main masses head-on: two fleece ovals, neck, head, ears and the legs as lower leg masses. Plan view: back in two masses, neck, head wedge, ear strokes, tail stroke and the leg masses under the fleece.",
          "Vorderansicht: Hauptmassen von vorn: zwei Vliesovale, Hals, Kopf, Ohren und die Beine als untere Beinmassen. Draufsicht: Rücken in zwei Massen, Hals, Kopfkeil, Ohrstriche, Schwanzstrich und die Beinmassen unter dem Vlies."),
    500: ("Front view: one composed silhouette head-on: fleece ovals, neck and head as smooth masses, each leg as two strokes. Plan view: back and chest as two masses, neck and head wedge; leg strokes below.",
          "Vorderansicht: eine zusammengesetzte Silhouette von vorn: Vliesovale, Hals und Kopf als glatte Massen, jedes Bein als zwei Striche. Draufsicht: Rücken und Brust als zwei Massen, Hals und Kopfkeil; Beinstriche darunter."),
}
