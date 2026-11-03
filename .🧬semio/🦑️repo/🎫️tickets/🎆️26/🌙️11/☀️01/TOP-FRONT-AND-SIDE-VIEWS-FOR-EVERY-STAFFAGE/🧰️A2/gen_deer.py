import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = Asset("roe-deer")

HIND = profile([(-0.145, 0), (-0.135, 0.05), (-0.08, 0.09), (0.0, 0.1), (0.15, 0.095), (0.3, 0.085), (0.5, 0.08)])
CHEST = profile([(0.45, 0.08), (0.55, 0.085), (0.7, 0.085), (0.82, 0.07), (0.88, 0.04), (0.906, 0.0)])


def rows(scale):
    k = 1.0 if scale <= 100 else 1.4
    return [
        dict(id="{side}-shank", bone="shank-h{s}", flat=0.065, fw=(0.05, 0.034), fz=1.0, plat=0.06, pr=(0.03 * k, 0.02 * k), pz=1.0),
        dict(id="{side}-hind-cannon", bone="cannon-h{s}", flat=0.065, fw=(0.03, 0.026), fz=1.0, plat=0.06, pr=(0.014 * k, 0.013 * k), pz=1.0),
        dict(id="{side}-hind-hoof", bone="hoof-h{s}", flat=0.065, fw=(0.034, 0.042), fz=1.0, plat=0.06, pr=(0.019 * k, 0.015 * k), pz=1.0),
        dict(id="{side}-forearm", bone="forearm-f{s}", flat=0.05, fw=(0.05, 0.032), fz=5.0, plat=0.05, pr=(0.027 * k, 0.019 * k), pz=1.0),
        dict(id="{side}-fore-cannon", bone="cannon-f{s}", flat=0.05, fw=(0.03, 0.026), fz=5.0, plat=0.05, pr=(0.014 * k, 0.013 * k), pz=1.0),
        dict(id="{side}-fore-hoof", bone="hoof-f{s}", flat=0.05, fw=(0.034, 0.042), fz=5.0, plat=0.05, pr=(0.019 * k, 0.015 * k), pz=1.0),
    ]


def rows500():
    return [
        dict(id="{side}-hind-leg", bone="shank-h{s}", like="{side}-hind-leg", flat=0.065, fz=1.0, plat=0.06, pz=1.0, under=0.3),
        dict(id="{side}-fore-leg", bone="forearm-f{s}", like="{side}-fore-leg", flat=0.05, fz=5.0, plat=0.05, pz=1.0, under=0.265),
    ]


def trunk(F, P, scale):
    n = 5 if scale <= 50 else 3
    F.ell("hindbody", "lumbar", 0, 0.652, 0.095, 0.133, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.62, 0.085, 0.181, like="chest", z=3.2)
    F.rev("neck", "neck", lat=0.045, like="neck", z=3.3)
    P.blob("hindbody", "lumbar", HIND, n=n, like="hindbody", z=3)
    P.blob("chest", "thorax", CHEST, n=n, like="chest", z=3.2)
    P.rev("neck", "neck", lat=0.045, like="neck", z=3.3)


def build50():
    F, P = View(A, 50, "front"), View(A, 50, "plan")
    limbs(F, P, rows(50))
    trunk(F, P, 50)
    F.ell("rump-patch", "lumbar", 0, 0.66, 0.05, 0.065, like="rump-patch", z=3.1)
    F.ell("jaw", "jaw", 0, 0.83, 0.02, 0.035, like="jaw", z=3.9)
    F.ell("head", "head", 0, 0.888, 0.042, 0.089, like="head", z=4)
    F.ell("far-ear", "ear-f", 0.07, 1.01, 0.034, 0.066, rot=-18, like="far-ear", z=2)
    F.ell("near-ear", "ear-n", -0.07, 1.01, 0.034, 0.066, rot=18, like="near-ear", z=4.5)
    F.ell("nose-leather", "head", 0, 0.815, 0.016, 0.011, like="nose-leather", z=4.2)
    F.ell("eye", "head", -0.036, 0.904, 0.007, 0.007, like="eye", z=5, role="detail", filled=True)
    F.ell("eye-f", "head", 0.036, 0.904, 0.007, 0.007, like="eye", z=5, role="detail", filled=True)
    P.ell("rump-patch", "lumbar", -0.095, 0.0, 0.035, 0.04, like="rump-patch", z=3.1)
    P.cap("jaw", "jaw", 1.112, 0, 1.228, 0, 0.022, 0.012, follow=True, like="jaw", z=3.9)
    P.cap("head", "head", 1.09, 0, 1.286, 0, 0.046, 0.016, follow=True, like="head", z=4)
    P.cap("far-ear", "ear-f", 1.15, 0.062, 1.24, 0.062, 0.034, 0.018, follow=True, like="far-ear", z=2)
    P.cap("near-ear", "ear-n", 1.15, -0.062, 1.24, -0.062, 0.034, 0.018, follow=True, like="near-ear", z=4.5)
    P.ell("nose-leather", "head", 1.304, 0.0, 0.012, 0.014, like="nose-leather", z=4.2)
    P.ell("eye", "head", 1.21, -0.04, 0.007, 0.007, like="eye", z=5, role="detail", filled=True)
    P.ell("eye-f", "head", 1.21, 0.04, 0.007, 0.007, like="eye", z=5, role="detail", filled=True)
    return F, P


def build100():
    F, P = View(A, 100, "front"), View(A, 100, "plan")
    limbs(F, P, rows(100))
    trunk(F, P, 100)
    F.ell("rump-patch", "lumbar", 0, 0.66, 0.05, 0.065, like="rump-patch", z=3.1)
    F.ell("jaw", "jaw", 0, 0.83, 0.02, 0.035, like="jaw", z=3.9)
    F.ell("head", "head", 0, 0.888, 0.042, 0.089, like="head", z=4)
    F.ell("far-ear", "ear-f", 0.07, 1.01, 0.034, 0.066, rot=-18, like="far-ear", z=2)
    F.ell("near-ear", "ear-n", -0.07, 1.01, 0.034, 0.066, rot=18, like="near-ear", z=4.5)
    F.ell("nose-leather", "head", 0, 0.815, 0.016, 0.011, like="nose-leather", z=4.2)
    P.ell("rump-patch", "lumbar", -0.095, 0.0, 0.035, 0.04, like="rump-patch", z=3.1)
    P.cap("jaw", "jaw", 1.112, 0, 1.228, 0, 0.022, 0.012, follow=True, like="jaw", z=3.9)
    P.cap("head", "head", 1.09, 0, 1.286, 0, 0.046, 0.016, follow=True, like="head", z=4)
    P.cap("far-ear", "ear-f", 1.15, 0.062, 1.24, 0.062, 0.034, 0.018, follow=True, like="far-ear", z=2)
    P.cap("near-ear", "ear-n", 1.15, -0.062, 1.24, -0.062, 0.034, 0.018, follow=True, like="near-ear", z=4.5)
    P.ell("nose-leather", "head", 1.304, 0.0, 0.012, 0.014, like="nose-leather", z=4.2)
    return F, P


def build200():
    F, P = View(A, 200, "front"), View(A, 200, "plan")
    limbs(F, P, rows(200))
    trunk(F, P, 200)
    F.ell("rump-patch", "lumbar", 0, 0.66, 0.05, 0.065, like="rump-patch", z=3.1)
    F.ell("jaw", "jaw", 0, 0.83, 0.024, 0.035, like="jaw", z=3.9)
    F.ell("head", "head", 0, 0.888, 0.046, 0.089, like="head", z=4)
    F.ell("far-ear", "ear-f", 0.07, 1.01, 0.036, 0.066, rot=-18, like="far-ear", z=2)
    F.ell("near-ear", "ear-n", -0.07, 1.01, 0.036, 0.066, rot=18, like="near-ear", z=4.5)
    P.cap("jaw", "jaw", 1.112, 0, 1.228, 0, 0.024, 0.014, follow=True, like="jaw", z=3.9)
    P.cap("head", "head", 1.09, 0, 1.286, 0, 0.048, 0.02, follow=True, like="head", z=4)
    P.cap("far-ear", "ear-f", 1.15, 0.062, 1.24, 0.062, 0.036, 0.02, follow=True, like="far-ear", z=2)
    P.cap("near-ear", "ear-n", 1.15, -0.062, 1.24, -0.062, 0.036, 0.02, follow=True, like="near-ear", z=4.5)
    P.ell("rump-patch", "lumbar", -0.095, 0.0, 0.035, 0.04, like="rump-patch", z=3.1)
    return F, P


def build500():
    F, P = View(A, 500, "front"), View(A, 500, "plan")
    limbs_line(F, P, rows500())
    F.ell("hindbody", "lumbar", 0, 0.652, 0.095, 0.133, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.62, 0.088, 0.181, like="chest", z=3.2)
    F.rev("neck", "neck", lat=0.045, like="neck", z=3.3)
    F.ell("head", "head", 0, 0.888, 0.056, 0.089, like="head", z=4, filled=False)
    F.ell("ears", "ear-n", 0.0, 1.01, 0.1, 0.066, like="ears", z=4.5, filled=False)
    P.blob("hindbody", "lumbar", profile([(-0.145, 0), (-0.08, 0.09), (0.0, 0.1), (0.3, 0.085), (0.5, 0.08)]), n=2, like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.45, 0.08), (0.7, 0.088), (0.85, 0.06), (0.906, 0.0)]), n=2, like="chest", z=3.2)
    P.rev("neck", "neck", lat=0.045, like="neck", z=3.3)
    P.cap("head", "head", 1.09, 0, 1.286, 0, 0.056, 0.046, follow=True, like="head", z=4)
    P.ell("ears", "ear-n", 1.2, 0.0, 0.05, 0.095, like="ears", z=4.5, filled=False)
    return F, P


BUILD = {50: build50, 100: build100, 200: build200, 500: build500}

NOTES = {
    50: ("Front view: the roe deer head-on at full articulation: slim ovals for hindquarters and chest, the white rump patch, tapered neck that follows the neck bone when the head is raised or lowered, head with muzzle, jaw and eyes, two broad ears; each foreleg and hind leg as forearm or shank, cannon and hoof, bound with followTip so lifted and bounding legs read correctly. Plan view: the narrow back in two masses with the rump patch, neck, head wedge with muzzle, jaw, eyes and spread ears, legs as thin strokes under the body that show only when a stride swings them out.",
         "Vorderansicht: das Reh von vorn in voller Gliederung: schlanke Ovale für Hinterhand und Brust, der weiße Spiegel, sich verjüngender Hals, der dem Halsknochen beim Heben und Senken des Kopfes folgt, Kopf mit Äser, Kiefer und Augen, zwei breite Lauscher; Vorder- und Hinterläufe je als Unterarm oder Unterschenkel, Röhre und Schalen, mit followTip gebunden, damit gehobene und flüchtige Läufe richtig lesbar sind. Draufsicht: der schmale Rücken in zwei Massen mit Spiegel, Hals, Kopfkeil mit Äser, Kiefer, Augen und gespreizten Lauschern, Läufe als dünne Striche unter dem Körper, die nur erscheinen, wenn ein Schritt sie herausschwingt."),
    100: ("Front view: articulated head-on: two body ovals, rump patch, tapered neck, head with jaw, two ears and the legs as shank or forearm, cannon and hoof. Plan view: back in two masses, rump patch, neck, head wedge, jaw, ears; legs as strokes under the body.",
          "Vorderansicht: gegliedert von vorn: zwei Körperovale, Spiegel, sich verjüngender Hals, Kopf mit Kiefer, zwei Lauscher und die Läufe aus Unterschenkel oder Unterarm, Röhre und Schalen. Draufsicht: Rücken in zwei Massen, Spiegel, Hals, Kopfkeil, Kiefer, Lauscher; Läufe als Striche unter dem Körper."),
    200: ("Front view: main masses head-on: two body ovals, rump patch, neck, head with jaw, two ears and the legs as lower leg masses. Plan view: back in two masses, neck, head wedge, jaw, ears and the leg masses under the body.",
          "Vorderansicht: Hauptmassen von vorn: zwei Körperovale, Spiegel, Hals, Kopf mit Kiefer, zwei Lauscher und die Läufe als untere Beinmassen. Draufsicht: Rücken in zwei Massen, Hals, Kopfkeil, Kiefer, Lauscher und die Beinmassen unter dem Körper."),
    500: ("Front view: one composed silhouette head-on: body ovals, neck and head as smooth masses, the ears as one band and each leg as one stroke. Plan view: back and chest as two masses, neck and head wedge, ears as one oval; leg strokes below.",
          "Vorderansicht: eine zusammengesetzte Silhouette von vorn: Körperovale, Hals und Kopf als glatte Massen, die Lauscher als ein Band und jeder Lauf als ein Strich. Draufsicht: Rücken und Brust als zwei Massen, Hals und Kopfkeil, Lauscher als ein Oval; Laufstriche darunter."),
}
