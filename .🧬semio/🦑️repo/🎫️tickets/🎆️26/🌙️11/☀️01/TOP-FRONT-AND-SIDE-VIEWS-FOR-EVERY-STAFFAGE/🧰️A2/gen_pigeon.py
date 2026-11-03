import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = Asset("pigeon")
FAR, NEAR = 1, -1
BODY = profile([(-0.09, 0), (-0.08, 0.03), (-0.03, 0.058), (0.04, 0.062), (0.1, 0.05), (0.13, 0.025), (0.149, 0.0)])


def front_core(F, scale):
    F.ell("body", "trunk", 0, 0.132, 0.07, 0.082, like="body", z=3)
    F.ext("tail", "tail", F.bar("tail", -0.02, 0.1, 0.03), -0.04, 0.04, like="tail", z=2.8)
    if scale <= 100:
        F.ext("far-tail", "tail-f", F.bar("tail-f", -0.02, 0.1, 0.03), -0.045, 0.045, like="far-tail", z=1)
    F.ell("head", "head", 0, 0.224, 0.028, 0.026, like="head", z=4)


def build50():
    F, P = View(A, 50, "front"), View(A, 50, "plan")
    front_core(F, 50)
    F.rev("neck-1", "neck-1", lat=0.034, like="neck-1", z=3.3)
    F.rev("neck-2", "neck-2", lat=0.03, like="neck-2", z=3.3)
    F.rev("neck-3", "neck-3", lat=0.028, like="neck-3", z=3.3)
    F.ell("beak", "head", 0, 0.219, 0.009, 0.008, like="beak", z=4.1)
    F.ell("cere", "head", 0, 0.226, 0.01, 0.007, like="cere", z=4.2)
    F.ell("eye", "head", -0.022, 0.226, 0.005, 0.006, like="eye", z=5, role="detail", filled=True)
    F.ell("eye-f", "head", 0.022, 0.226, 0.005, 0.006, like="eye", z=5, role="detail", filled=True)
    for sd, t, tag in ((FAR, "f", "far"), (NEAR, "n", "near")):
        z = 1.0 if sd == FAR else 5.0
        F.ext(tag + "-coverts", "forearm-" + t, F.bar("forearm-" + t, -0.03, 0.08, 0.028), sd * 0.044 - 0.03 if False else min(sd * 0.045, sd * 0.075), max(sd * 0.045, sd * 0.075), like="near-coverts", z=z + 3.7 if sd == NEAR else z + 0.6)
        F.ext(tag + "-primaries", "hand-" + t, F.bar("hand-" + t, 0.0, 0.15, 0.013), min(sd * 0.05, sd * 0.075), max(sd * 0.05, sd * 0.075), like="near-primaries" if sd == NEAR else "far-primaries", z=z + 3.6 if sd == NEAR else z + 0.5)
        lat = sd * 0.026
        F.limb_front(tag + "-thigh", "thigh-" + t, lat, 0.03, 0.02, like=tag + "-thigh", z=z)
        F.limb_front(tag + "-shank", "shank-" + t, lat, 0.016, 0.012, like=tag + "-shank", z=z)
        F.limb_front(tag + "-tarsus", "tarsus-" + t, lat, 0.012, 0.012, like=tag + "-tarsus", z=z)
        F.ext(tag + "-toes", "toes-" + t, F.bar("toes-" + t, -0.004, 0.036, 0.011), lat - 0.032, lat + 0.032, keep_box=True, like=tag + "-toes", z=z)
        P.limb_plan(tag + "-thigh", "thigh-" + t, lat, 0.022, 0.012, like=tag + "-thigh", z=z)
        P.limb_plan(tag + "-shank", "shank-" + t, lat, 0.01, 0.008, like=tag + "-shank", z=z)
        P.limb_plan(tag + "-tarsus", "tarsus-" + t, lat, 0.008, 0.008, like=tag + "-tarsus", z=z)
        P.limb_plan(tag + "-toes", "toes-" + t, lat, 0.012, 0.009, like=tag + "-toes", z=z)
        wl = sd * 0.052
        P.ext(tag + "-coverts", "forearm-" + t, P.bar("forearm-" + t, -0.03, 0.08, 0.028), min(sd * 0.045, sd * 0.075), max(sd * 0.045, sd * 0.075), like="near-coverts", z=z + 3.7 if sd == NEAR else z + 0.6)
        P.ext(tag + "-primaries", "hand-" + t, P.bar("hand-" + t, 0.0, 0.15, 0.013), min(sd * 0.05, sd * 0.075), max(sd * 0.05, sd * 0.075), like="near-primaries" if sd == NEAR else "far-primaries", z=z + 3.6 if sd == NEAR else z + 0.5)
    P.cap("far-tail", "tail-f", -0.068, 0, -0.134, 0, 0.026, 0.044, follow=True, like="far-tail", z=1)
    P.cap("tail", "tail", -0.068, 0, -0.14, 0, 0.022, 0.038, follow=True, like="tail", z=2.8)
    P.blob("body", "trunk", BODY, like="body", z=3)
    P.rev("neck-1", "neck-1", lat=0.034, like="neck-1", z=3.3)
    P.rev("neck-2", "neck-2", lat=0.03, like="neck-2", z=3.3)
    P.rev("neck-3", "neck-3", lat=0.028, like="neck-3", z=3.3)
    P.cap("head", "head", 0.095, 0, 0.148, 0, 0.028, 0.016, follow=True, like="head", z=4)
    P.ell("beak", "head", 0.138, 0.0, 0.016, 0.009, like="beak", z=4.1)
    P.ell("cere", "head", 0.124, 0.0, 0.009, 0.01, like="cere", z=4.2)
    P.ell("eye", "head", 0.119, -0.023, 0.006, 0.005, like="eye", z=5, role="detail", filled=True)
    P.ell("eye-f", "head", 0.119, 0.023, 0.006, 0.005, like="eye", z=5, role="detail", filled=True)
    return F, P


def build100():
    F, P = View(A, 100, "front"), View(A, 100, "plan")
    front_core(F, 100)
    F.rev("neck-1", "neck-1", lat=0.034, like="neck-1", z=3.3)
    F.rev("neck-2", "neck-2", lat=0.03, like="neck-2", z=3.3)
    F.rev("neck-3", "neck-3", lat=0.028, like="neck-3", z=3.3)
    F.ell("beak", "head", 0, 0.219, 0.009, 0.008, like="beak", z=4.1)
    for sd, t, tag in ((FAR, "f", "far"), (NEAR, "n", "near")):
        z = 1.0 if sd == FAR else 5.0
        F.ext(tag + "-coverts", "forearm-" + t, F.bar("forearm-" + t, -0.03, 0.08, 0.028), min(sd * 0.045, sd * 0.075), max(sd * 0.045, sd * 0.075), like=tag + "-coverts", z=z + 3.7 if sd == NEAR else z + 0.6)
        F.ext(tag + "-primaries", "hand-" + t, F.bar("hand-" + t, 0.0, 0.15, 0.013), min(sd * 0.05, sd * 0.075), max(sd * 0.05, sd * 0.075), like=tag + "-primaries", z=z + 3.6 if sd == NEAR else z + 0.5)
        lat = sd * 0.026
        F.limb_front(tag + "-thigh", "thigh-" + t, lat, 0.03, 0.02, like=tag + "-thigh", z=z)
        F.limb_front(tag + "-shank", "shank-" + t, lat, 0.016, 0.012, like=tag + "-shank", z=z)
        F.limb_front(tag + "-tarsus", "tarsus-" + t, lat, 0.012, 0.012, like=tag + "-tarsus", z=z)
        F.ext(tag + "-toes", "toes-" + t, F.bar("toes-" + t, -0.004, 0.036, 0.011), lat - 0.032, lat + 0.032, keep_box=True, like=tag + "-toes", z=z)
        P.limb_plan(tag + "-shank", "shank-" + t, lat, 0.012, 0.009, like=tag + "-shank", z=z)
        P.limb_plan(tag + "-tarsus", "tarsus-" + t, lat, 0.009, 0.009, like=tag + "-tarsus", z=z)
        P.limb_plan(tag + "-toes", "toes-" + t, lat, 0.012, 0.009, like=tag + "-toes", z=z)
        wl = sd * 0.052
        P.ext(tag + "-coverts", "forearm-" + t, P.bar("forearm-" + t, -0.03, 0.08, 0.028), min(sd * 0.045, sd * 0.075), max(sd * 0.045, sd * 0.075), like=tag + "-coverts", z=z + 3.7 if sd == NEAR else z + 0.6)
        P.ext(tag + "-primaries", "hand-" + t, P.bar("hand-" + t, 0.0, 0.15, 0.013), min(sd * 0.05, sd * 0.075), max(sd * 0.05, sd * 0.075), like=tag + "-primaries", z=z + 3.6 if sd == NEAR else z + 0.5)
    P.cap("far-tail", "tail-f", -0.068, 0, -0.134, 0, 0.026, 0.044, follow=True, like="far-tail", z=1)
    P.cap("tail", "tail", -0.068, 0, -0.14, 0, 0.022, 0.038, follow=True, like="tail", z=2.8)
    P.blob("body", "trunk", BODY, like="body", z=3)
    P.rev("neck-1", "neck-1", lat=0.034, like="neck-1", z=3.3)
    P.rev("neck-2", "neck-2", lat=0.03, like="neck-2", z=3.3)
    P.rev("neck-3", "neck-3", lat=0.028, like="neck-3", z=3.3)
    P.cap("head", "head", 0.095, 0, 0.148, 0, 0.028, 0.016, follow=True, like="head", z=4)
    P.ell("beak", "head", 0.138, 0.0, 0.016, 0.009, like="beak", z=4.1)
    return F, P


def build200():
    F, P = View(A, 200, "front"), View(A, 200, "plan")
    front_core(F, 200)
    F.rev("neck", "neck-2", lat=0.032, like="neck", z=3.3)
    F.ext("near-primaries", "hand-n", F.bar("hand-n", 0.0, 0.15, 0.013), -0.075, -0.035, like="near-primaries", z=4.6)
    F.ext("near-coverts", "forearm-n", F.bar("forearm-n", -0.03, 0.08, 0.028), -0.075, -0.045, like="near-coverts", z=4.7)
    F.ext("far-wing", "hand-f", F.bar("hand-f", 0.0, 0.15, 0.025), 0.035, 0.075, like="far-wing", z=1.5)
    for sd, t, tag in ((FAR, "f", "far"), (NEAR, "n", "near")):
        z = 1.0 if sd == FAR else 5.0
        lat = sd * 0.026
        F.poly(tag + "-leg", "shank-" + t, [(lat, 0.09), (lat, 0.01)], closed=False, follow=True, filled=False, like=tag + "-leg", z=z)
        F.poly(tag + "-foot", "tarsus-" + t, [(lat - 0.03, -0.002), (lat + 0.03, -0.002)], closed=False, filled=False, like=tag + "-foot", z=z)
        P.cap(tag + "-leg", "shank-" + t, 0.041, lat, 0.02, lat, 0.001, 0.001, follow=True, filled=False, like=tag + "-leg", z=z)
        P.cap(tag + "-foot", "tarsus-" + t, 0.02, lat, 0.06, lat, 0.001, 0.001, follow=True, filled=False, like=tag + "-foot", z=z)
    P.ext("near-primaries", "hand-n", P.bar("hand-n", 0.0, 0.15, 0.013), -0.075, -0.035, like="near-primaries", z=4.6)
    P.ext("near-coverts", "forearm-n", P.bar("forearm-n", -0.03, 0.08, 0.028), -0.075, -0.045, like="near-coverts", z=4.7)
    P.limb_plan("near-shoulder", "humerus-n", -0.052, 0.02, 0.02, like="near-shoulder", z=4.8)
    P.ext("far-wing", "hand-f", P.bar("hand-f", 0.0, 0.15, 0.025), 0.035, 0.075, like="far-wing", z=1.5)
    P.cap("far-tail", "tail-f", -0.068, 0, -0.134, 0, 0.026, 0.044, follow=True, like="far-tail", z=1)
    P.cap("tail", "tail", -0.068, 0, -0.14, 0, 0.022, 0.038, follow=True, like="tail", z=2.8)
    P.blob("body", "trunk", BODY, n=3, like="body", z=3)
    P.rev("neck", "neck-2", lat=0.032, like="neck", z=3.3)
    P.cap("head", "head", 0.095, 0, 0.148, 0, 0.03, 0.018, follow=True, like="head", z=4)
    return F, P


BUILD = {50: build50, 100: build100, 200: build200}

NOTES = {
    50: ("Front view: the pigeon head-on at full articulation: round breast, a three-piece tapered neck that bends with the bones, head with beak, cere and both eyes, folded wings as coverts and primaries at the flanks that rise when the wings lift, the tail fan behind and each leg as thigh, shank, tarsus and a splayed toe foot. Plan view: spindle back, both folded wings as coverts and primaries strokes, the tail fan, three neck links, head with beak, cere and eyes and the legs under the body.",
         "Vorderansicht: die Taube von vorn in voller Gliederung: runde Brust, ein dreiteiliger, sich verjüngender Hals, der den Knochen folgt, Kopf mit Schnabel, Wachshaut und beiden Augen, angelegte Flügel als Decken und Handschwingen an den Flanken, die beim Flügelheben steigen, dahinter der Schwanzfächer und jedes Bein aus Schenkel, Lauf und gespreiztem Zehenfuß. Draufsicht: Spindelrücken, beide angelegten Flügel als Decken- und Handschwingenstriche, der Schwanzfächer, drei Halsglieder, Kopf mit Schnabel, Wachshaut und Augen und die Beine unter dem Körper."),
    100: ("Front view: articulated head-on: breast oval, tapered neck in three links, head with beak, wing coverts and primaries at the flanks, the tail and the legs with toe foot. Plan view: spindle back, folded wings, tail fan, neck links, head with beak and legs under the body.",
          "Vorderansicht: gegliedert von vorn: Brustoval, sich verjüngender Hals in drei Gliedern, Kopf mit Schnabel, Flügeldecken und Handschwingen an den Flanken, der Schwanz und die Beine mit Zehenfuß. Draufsicht: Spindelrücken, angelegte Flügel, Schwanzfächer, Halsglieder, Kopf mit Schnabel und Beine unter dem Körper."),
    200: ("Front view: main masses head-on: breast oval, one tapered neck, head, the near and far wing as extruded strips, the tail behind and each leg as one stroke with a foot bar. Plan view: spindle back, the near wing and the far wing, tail fan, neck, head and hairline legs under the body.",
          "Vorderansicht: Hauptmassen von vorn: Brustoval, ein sich verjüngender Hals, Kopf, nahe und ferne Flügel als extrudierte Streifen, dahinter der Schwanz und jedes Bein als ein Strich mit Fußbalken. Draufsicht: Spindelrücken, nahe und ferner Flügel, Schwanzfächer, Hals, Kopf und haarfeine Beine unter dem Körper."),
}
