import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = Asset("red-fox")


def rows(scale):
    big = scale <= 50
    k = 1.0 if scale <= 100 else 1.25
    kc = 1.0 if scale <= 100 else 1.6
    pad = 0.0 if scale <= 100 else 0.022
    return [
        dict(id="gaskin-{s}", bone="shank-h{s}", flat=0.058, fw=(0.05 * k, 0.04 * k), fz=1.2, plat=0.05, pr=(0.026 * k, 0.018 * k), pz=1.2),
        dict(id="cannon-h-{s}", bone="metatarsus-h{s}", flat=0.058, fw=(0.036 * kc, 0.032 * kc), over=pad, under=pad, fz=1.04, plat=0.05, pr=(0.016 * kc, 0.014 * kc), pz=1.04),
        dict(id="paw-h-{s}", bone="paw-h{s}", flat=0.058, fw=(0.04 * k, 0.046 * k), over=pad, fz=1.06, plat=0.05, pr=(0.02 * k, 0.017 * k), pz=1.06),
        dict(id="forearm-{s}", bone="forearm-f{s}", flat=0.04, fw=(0.05 * k, 0.034 * k), fz=5.05, plat=0.04, pr=(0.022 * k, 0.016 * k), pz=1.05),
        dict(id="cannon-f-{s}", bone="metacarpus-f{s}", flat=0.04, fw=(0.034 * kc, 0.03 * kc), over=pad, under=pad, fz=5.04, plat=0.04, pr=(0.014 * kc, 0.013 * kc), pz=1.04),
        dict(id="paw-f-{s}", bone="paw-f{s}", flat=0.04, fw=(0.038 * k, 0.044 * k), over=pad, fz=5.06, plat=0.04, pr=(0.019 * k, 0.016 * k), pz=1.06, pext=(0.0 if scale <= 100 else 0.03)),
    ]


def build50():
    F, P = View(A, 50, "front"), View(A, 50, "plan")
    limbs(F, P, rows(50))
    F.ell("hindbody", "lumbar", 0, 0.359, 0.08, 0.114, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.356, 0.07, 0.119, like="chest", z=3.2)
    F.ell("neck", "neck", 0, 0.452, 0.048, 0.12, like="neck", z=2.9)
    F.ell("jaw", "jaw", 0, 0.44, 0.022, 0.034, like="jaw", z=3.9)
    F.ell("head", "head", 0, 0.495, 0.043, 0.075, like="head", z=4)
    F.ell("ear-f", "ear-f", 0.033, 0.575, 0.027, 0.055, rot=-8, like="ear-f", z=4.5)
    F.ell("ear-n", "ear-n", -0.033, 0.575, 0.027, 0.055, rot=8, like="ear-n", z=4.5)
    F.ell("nose", "head", 0, 0.432, 0.012, 0.01, like="nose", z=4.6, role="detail", filled=True)
    F.ell("eye", "head", -0.026, 0.503, 0.009, 0.009, like="eye", z=4.6, role="detail", filled=True)
    F.ell("eye-f", "head", 0.026, 0.503, 0.009, 0.009, like="eye", z=4.6, role="detail", filled=True)
    F.rev("brush-root", "tail-1", lat=0.07, like="brush-root", z=2.8)
    F.rev("brush-mid", "tail-2", lat=0.078, like="brush-mid", z=2.7)
    F.rev("brush-tip", "tail-3", lat=0.05, like="brush-tip", z=2.6)
    P.blob("hindbody", "lumbar", profile([(-0.076, 0), (-0.066, 0.045), (-0.03, 0.074), (0.05, 0.08), (0.15, 0.076), (0.3, 0.07), (0.4, 0.066), (0.49, 0.066)]), like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.312, 0.066), (0.4, 0.072), (0.48, 0.07), (0.54, 0.05), (0.563, 0)]), like="chest", z=3.2)
    P.cap("neck", "neck", 0.469, 0, 0.579, 0, 0.058, 0.046, follow=True, like="neck", z=2.9)
    P.cap("head", "head", 0.565, 0, 0.71, 0, 0.044, 0.016, follow=True, like="head", z=4)
    P.cap("jaw", "jaw", 0.606, 0, 0.676, 0, 0.02, 0.012, follow=True, like="jaw", z=3.9)
    P.cap("ear-f", "ear-f", 0.628, 0.036, 0.642, 0.036, 0.022, 0.014, follow=True, like="ear-f", z=4.5)
    P.cap("ear-n", "ear-n", 0.628, -0.036, 0.642, -0.036, 0.022, 0.014, follow=True, like="ear-n", z=4.5)
    P.ell("nose", "head", 0.724, 0.0, 0.011, 0.01, like="nose", z=4.6, role="detail", filled=True)
    P.ell("eye", "head", 0.652, -0.028, 0.009, 0.009, like="eye", z=4.6, role="detail", filled=True)
    P.ell("eye-f", "head", 0.652, 0.028, 0.009, 0.009, like="eye", z=4.6, role="detail", filled=True)
    P.rev("brush-root", "tail-1", lat=0.07, like="brush-root", z=2.8)
    P.rev("brush-mid", "tail-2", lat=0.078, like="brush-mid", z=2.7)
    P.rev("brush-tip", "tail-3", lat=0.05, like="brush-tip", z=2.6)
    return F, P


def build100():
    F, P = View(A, 100, "front"), View(A, 100, "plan")
    limbs(F, P, rows(100))
    F.ell("hindbody", "lumbar", 0, 0.359, 0.08, 0.114, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.356, 0.07, 0.119, like="chest", z=3.2)
    F.ell("neck", "neck", 0, 0.452, 0.048, 0.12, like="neck", z=2.9)
    F.ell("head", "head", 0, 0.495, 0.043, 0.075, like="head", z=4)
    F.ell("ear-f", "ear-f", 0.033, 0.575, 0.027, 0.055, rot=-8, like="ear-f", z=4.5)
    F.ell("ear-n", "ear-n", -0.033, 0.575, 0.027, 0.055, rot=8, like="ear-n", z=4.5)
    F.rev("brush", "tail-1", lat=0.07, like="brush", z=2.8)
    F.rev("brush-2", "tail-2", lat=0.078, like="brush-2", z=2.7)
    F.rev("brush-tip", "tail-3", lat=0.05, like="brush-tip", z=2.6)
    P.blob("hindbody", "lumbar", profile([(-0.076, 0), (-0.066, 0.045), (-0.03, 0.074), (0.05, 0.08), (0.15, 0.076), (0.3, 0.07), (0.4, 0.066), (0.49, 0.066)]), like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.312, 0.066), (0.4, 0.072), (0.48, 0.07), (0.54, 0.05), (0.563, 0)]), like="chest", z=3.2)
    P.cap("neck", "neck", 0.469, 0, 0.579, 0, 0.058, 0.046, follow=True, like="neck", z=2.9)
    P.cap("head", "head", 0.565, 0, 0.71, 0, 0.044, 0.016, follow=True, like="head", z=4)
    P.cap("ear-f", "ear-f", 0.628, 0.036, 0.642, 0.036, 0.022, 0.014, follow=True, like="ear-f", z=4.5)
    P.cap("ear-n", "ear-n", 0.628, -0.036, 0.642, -0.036, 0.022, 0.014, follow=True, like="ear-n", z=4.5)
    P.rev("brush", "tail-1", lat=0.07, like="brush", z=2.8)
    P.rev("brush-2", "tail-2", lat=0.078, like="brush-2", z=2.7)
    P.rev("brush-tip", "tail-3", lat=0.05, like="brush-tip", z=2.6)
    return F, P


def build200():
    F, P = View(A, 200, "front"), View(A, 200, "plan")
    limbs(F, P, rows(200))
    F.ell("hindbody", "lumbar", 0, 0.359, 0.08, 0.114, like="hindbody", z=3)
    F.ell("chest", "thorax", 0, 0.356, 0.072, 0.119, like="chest", z=3.2)
    F.ell("neck", "neck", 0, 0.452, 0.05, 0.12, like="neck", z=2.9)
    F.ell("head", "head", 0, 0.495, 0.046, 0.075, like="head", z=4)
    F.ell("ear", "ear-n", -0.033, 0.575, 0.03, 0.055, rot=8, like="ear", z=4.5)
    F.ell("ear-far", "ear-f", 0.033, 0.575, 0.03, 0.055, rot=-8, like="ear", z=4.5)
    F.rev("brush", "tail-1", lat=0.07, like="brush", z=2.8)
    F.rev("brush-2", "tail-2", lat=0.078, like="brush-2", z=2.7)
    F.rev("brush-tip", "tail-3", lat=0.05, like="brush-tip", z=2.6)
    P.blob("hindbody", "lumbar", profile([(-0.076, 0), (-0.066, 0.045), (-0.03, 0.074), (0.05, 0.08), (0.15, 0.076), (0.3, 0.07), (0.4, 0.066), (0.49, 0.066)]), n=3, like="hindbody", z=3)
    P.blob("chest", "thorax", profile([(0.312, 0.066), (0.4, 0.072), (0.48, 0.07), (0.54, 0.05), (0.563, 0)]), n=3, like="chest", z=3.2)
    P.cap("neck", "neck", 0.469, 0, 0.579, 0, 0.058, 0.046, follow=True, like="neck", z=2.9)
    P.cap("head", "head", 0.565, 0, 0.71, 0, 0.046, 0.018, follow=True, like="head", z=4)
    P.ell("nose", "head", 0.716, 0.0, 0.02, 0.02, like="nose", z=4.6, role="body", filled=True)
    P.cap("ear", "ear-n", 0.628, -0.036, 0.642, -0.036, 0.024, 0.016, follow=True, like="ear", z=4.5)
    P.cap("ear-far", "ear-f", 0.628, 0.036, 0.642, 0.036, 0.024, 0.016, follow=True, like="ear", z=4.5)
    P.rev("brush", "tail-1", lat=0.07, like="brush", z=2.8)
    P.rev("brush-2", "tail-2", lat=0.078, like="brush-2", z=2.7)
    P.rev("brush-tip", "tail-3", lat=0.05, like="brush-tip", z=2.6)
    return F, P


BUILD = {50: build50, 100: build100, 200: build200}

NOTES = {
    50: ("Front view: the fox head-on at full articulation: hind mass and chest as two ovals, neck, head with muzzle, nose and both eyes, two upright ears and the jaw; the brush as three extruded masses behind the body that follow the tail bones when it rises or swings; each foreleg and hind leg as forearm or gaskin, cannon and paw wedge bound with followTip. Plan view: the narrow back in two masses, neck, head wedge with nose, ears, eyes and jaw, the brush in three tapering links that follow the tail bones, legs as strokes under the body that show only when a stride swings them out.",
         "Vorderansicht: der Fuchs von vorn in voller Gliederung: Hinterkörper und Brust als zwei Ovale, Hals, Kopf mit Fang, Nase und beiden Augen, zwei aufrechte Ohren und der Kiefer; die Lunte als drei extrudierte Massen hinter dem Körper, die den Schwanzknochen beim Heben und Schwingen folgen; Vorder- und Hinterbeine je als Unterarm oder Unterschenkel, Röhre und Pfotenkeil mit followTip gebunden. Draufsicht: der schmale Rücken in zwei Massen, Hals, Kopfkeil mit Nase, Ohren, Augen und Kiefer, die Lunte in drei sich verjüngenden Gliedern, die den Schwanzknochen folgen, Beine als Striche unter dem Körper, die nur erscheinen, wenn ein Schritt sie herausschwingt."),
    100: ("Front view: articulated head-on: two body ovals, neck, head, two ears, the brush in three extruded masses behind the body and the legs in forearm or gaskin, cannon and paw. Plan view: back in two masses, neck, head wedge, ears, the brush in three links, legs as strokes under the body.",
          "Vorderansicht: gegliedert von vorn: zwei Körperovale, Hals, Kopf, zwei Ohren, die Lunte in drei extrudierten Massen hinter dem Körper und die Beine aus Unterarm oder Unterschenkel, Röhre und Pfote. Draufsicht: Rücken in zwei Massen, Hals, Kopfkeil, Ohren, die Lunte in drei Gliedern, Beine als Striche unter dem Körper."),
    200: ("Front view: main masses head-on: two body ovals, neck, head, ears, the three brush masses and the legs as lower leg masses. Plan view: back in two masses, neck, head wedge, ear strokes, the brush in three links and the leg masses under the body.",
          "Vorderansicht: Hauptmassen von vorn: zwei Körperovale, Hals, Kopf, Ohren, die drei Luntenmassen und die Beine als untere Beinmassen. Draufsicht: Rücken in zwei Massen, Hals, Kopfkeil, Ohrstriche, die Lunte in drei Gliedern und die Beinmassen unter dem Körper."),
}
