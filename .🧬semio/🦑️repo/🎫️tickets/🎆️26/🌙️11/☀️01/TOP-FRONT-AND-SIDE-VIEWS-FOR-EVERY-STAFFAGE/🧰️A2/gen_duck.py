import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = Asset("mallard-duck")
NEAR, FAR = -1, 1


def build50():
    F = View(A, 50, "front")
    P = View(A, 50, "plan")
    # ---------------- front (looking at the drake from ahead) ----------------
    for sd, tag, wing, leg in ((FAR, "far", "wing-f", "f"), (NEAR, "near", "wing-n", "n")):
        z0 = 1 if sd == FAR else 4
        F.ext("%s-wing" % tag, wing, F.bar(wing, 0.0, 0.3, 0.045), min(sd * 0.04, sd * 0.078), max(sd * 0.04, sd * 0.078), like="%s-wing" % tag, z=z0 + (0.2 if sd == FAR else 0))
    F.ell("tail", "tail", 0, 0.165, 0.028, 0.03, like="tail", z=2)
    F.rev("body", "trunk", lat=0.098, like="body", z=3)
    F.ell("breast", "trunk", 0, 0.2, 0.062, 0.078, like="breast", z=3.1)
    for sd, tag, leg in ((FAR, "far", "f"), (NEAR, "near", "n")):
        z = 1.2 if sd == FAR else 4.5
        F.rect("%s-thigh" % tag, "thigh-%s" % leg, sd * 0.052 - 0.014, 0.08, 0.028, 0.07, r=0.012, like="near-thigh", z=z - 0.2 if sd == FAR else 3.5, follow=True)
        F.rect("%s-shank" % tag, "shank-%s" % leg, sd * 0.046 - 0.0085, 0.053, 0.017, 0.07, r=0.006, like="%s-shank" % tag, z=z, follow=True)
        F.rect("%s-tarsus" % tag, "tarsus-%s" % leg, sd * 0.046 - 0.0085, 0.01, 0.017, 0.045, r=0.006, like="%s-tarsus" % tag, z=z, follow=True)
        F.poly("%s-foot" % tag, "foot-%s" % leg, [(sd * 0.046 - 0.013, 0.025), (sd * 0.046 + 0.013, 0.025), (sd * 0.046 + 0.046, -0.003), (sd * 0.046 - 0.046, -0.003)], like="%s-foot" % tag, z=z, filled=True)
    F.rev("neck-1", "neck-1", lat=0.043, like="neck-1", z=3.2)
    F.rev("neck-2", "neck-2", lat=0.034, like="neck-2", z=3.3)
    F.rev("neck-3", "neck-3", lat=0.031, like="neck-3", z=3.4)
    F.ell("neck-ring", "neck-2", 0, 0.34, 0.037, 0.012, like="neck-ring", z=3.6)
    F.ell("head", "head", 0, 0.421, 0.034, 0.032, like="head", z=4)
    F.ell("bill", "head", 0, 0.411, 0.026, 0.013, like="bill", z=4.1)
    # ---------------- plan (seen from above, left = +y) ----------------
    for sd, tag, leg in ((FAR, "far", "f"), (NEAR, "near", "n")):
        y = sd * 0.045
        z = 1.2 if sd == FAR else 1.3
        P.cap("%s-thigh" % tag, "thigh-%s" % leg, 0.0, y, 0.03, y, 0.026, 0.02, follow=True, like="near-thigh", z=z - 0.1)
        P.cap("%s-shank" % tag, "shank-%s" % leg, 0.03, y, -0.01, y, 0.017, 0.011, follow=True, like="%s-shank" % tag, z=z)
        P.cap("%s-tarsus" % tag, "tarsus-%s" % leg, -0.01, y, 0.0, y, 0.011, 0.01, follow=True, like="%s-tarsus" % tag, z=z)
        P.poly("%s-foot" % tag, "foot-%s" % leg, [(-0.012, y - 0.01), (0.075, y - 0.036), (0.082, y), (0.075, y + 0.036), (-0.012, y + 0.01)], like="%s-foot" % tag, z=z)
    P.poly("tail", "tail", [(-0.14, 0.0), (-0.16, 0.04), (-0.2, 0.034), (-0.275, 0.01), (-0.28, 0), (-0.275, -0.01), (-0.2, -0.034), (-0.16, -0.04)], like="tail", z=2)
    P.rev("body", "trunk", lat=0.098, like="body", z=3)
    P.blob("breast", "trunk", [(0.06, 0.0), (0.09, 0.055), (0.13, 0.05), (0.165, 0), (0.13, -0.05), (0.09, -0.055)], like="breast", z=3.1)
    for sd, tag, wing in ((FAR, "far", "wing-f"), (NEAR, "near", "wing-n")):
        P.ext("%s-wing" % tag, wing, P.bar(wing, 0.0, 0.3, 0.045), min(sd * 0.04, sd * 0.078), max(sd * 0.04, sd * 0.078), like="%s-wing" % tag, z=4 if sd == NEAR else 4.0)
    P.rev("neck-1", "neck-1", lat=0.043, like="neck-1", z=3.2)
    P.rev("neck-2", "neck-2", lat=0.034, like="neck-2", z=3.3)
    P.rev("neck-3", "neck-3", lat=0.031, like="neck-3", z=3.4)
    P.cap("head", "head", 0.205, 0, 0.255, 0, 0.034, 0.029, follow=True, like="head", z=4)
    P.poly("bill", "head", [(0.25, 0.019), (0.292, 0.024), (0.312, 0.014), (0.312, -0.014), (0.292, -0.024), (0.25, -0.019)], like="bill", z=4.1)
    P.cap("neck-ring", "neck-2", 0.15, 0.0, 0.155, 0.0, 0.037, 0.037, follow=True, like="neck-ring", z=3.6)
    return F, P


def build100():
    F = View(A, 100, "front")
    P = View(A, 100, "plan")
    for sd, tag, wing in ((FAR, "far", "wing-f"), (NEAR, "near", "wing-n")):
        F.ext("%s-wing" % tag, wing, F.bar(wing, 0.0, 0.3, 0.045), min(sd * 0.04, sd * 0.078), max(sd * 0.04, sd * 0.078), like="%s-wing" % tag, z=1 if sd == FAR else 4)
    F.ell("tail", "tail", 0, 0.165, 0.028, 0.03, like="tail", z=2)
    F.rev("body", "trunk", lat=0.098, like="body", z=3)
    for sd, tag, leg in ((FAR, "far", "f"), (NEAR, "near", "n")):
        z = 1.0 if sd == FAR else 4.5
        F.poly("%s-leg" % tag, "shank-%s" % leg, [(sd * 0.046, 0.115), (sd * 0.046, 0.01), (sd * 0.046 + 0.04, 0.008), (sd * 0.046 - 0.04, 0.008)], closed=False, follow=True, filled=False, like="near-leg", z=z, penMm=0.2)
    F.rev("neck-lower", "neck-1", lat=0.043, like="neck-lower", z=3.2)
    F.rev("neck-upper", "neck-3", lat=0.032, like="neck-upper", z=3.4)
    F.ell("head", "head", 0, 0.421, 0.034, 0.032, like="head", z=4)
    F.ell("bill", "head", 0, 0.411, 0.026, 0.013, like="bill", z=4.1, filled=True, penMm=0.2)
    P.poly("tail", "tail", [(-0.14, 0.0), (-0.16, 0.04), (-0.2, 0.034), (-0.28, 0.0), (-0.2, -0.034), (-0.16, -0.04)], like="tail", z=2)
    P.rev("body", "trunk", lat=0.098, like="body", z=3)
    for sd, tag, wing in ((FAR, "far", "wing-f"), (NEAR, "near", "wing-n")):
        P.ext("%s-wing" % tag, wing, P.bar(wing, 0.0, 0.3, 0.045), min(sd * 0.04, sd * 0.078), max(sd * 0.04, sd * 0.078), like="%s-wing" % tag, z=4)
    for sd, tag, leg in ((FAR, "far", "f"), (NEAR, "near", "n")):
        y = sd * 0.045
        P.poly("%s-leg" % tag, "shank-%s" % leg, [(0.0, y), (-0.07, y)], closed=False, filled=False, like="near-leg", z=1, penMm=0.2)
    P.rev("neck-lower", "neck-1", lat=0.043, like="neck-lower", z=3.2)
    P.rev("neck-upper", "neck-3", lat=0.032, like="neck-upper", z=3.4)
    P.cap("head", "head", 0.205, 0, 0.255, 0, 0.034, 0.029, follow=True, like="head", z=4)
    P.poly("bill", "head", [(0.25, 0.019), (0.292, 0.024), (0.312, 0.014), (0.312, -0.014), (0.292, -0.024), (0.25, -0.019)], like="bill", z=4.1, filled=True, penMm=0.2)
    return F, P


def build200():
    F = View(A, 200, "front")
    P = View(A, 200, "plan")
    F.ell("tail", "tail", 0, 0.165, 0.03, 0.03, like="tail", z=2)
    F.rev("body", "trunk", lat=0.098, like="body", z=3)
    for sd, tag, leg in ((FAR, "far", "f"), (NEAR, "near", "n")):
        F.poly("%s-leg" % tag, "shank-%s" % leg, [(sd * 0.046, 0.1), (sd * 0.046, 0.01)], closed=False, follow=True, filled=False, like="near-leg", z=4.5, penMm=0.16)
    F.rev("neck-lower", "neck-1", lat=0.043, like="neck-lower", z=3.2)
    F.rev("neck-upper", "neck-3", lat=0.032, like="neck-upper", z=3.4)
    F.ell("head", "head", 0, 0.421, 0.036, 0.033, like="head", z=4)
    for sd, tag, wing in ((FAR, "wing-far", "wing-f"), (NEAR, "wing", "wing-n")):
        F.ext(tag, wing, F.bar(wing, 0.0, 0.3, 0.045), min(sd * 0.04, sd * 0.078), max(sd * 0.04, sd * 0.078), filled=False, like="wing", z=4, penMm=0.15)
    P.poly("tail", "tail", [(-0.14, 0.0), (-0.16, 0.042), (-0.2, 0.036), (-0.28, 0.0), (-0.2, -0.036), (-0.16, -0.042)], like="tail", z=2)
    P.rev("body", "trunk", lat=0.098, like="body", z=3)
    P.ext("wing", "wing-n", P.bar("wing-n", 0.0, 0.3, 0.045), -0.078, -0.04, filled=False, like="wing", z=4, penMm=0.15)
    P.ext("wing-far", "wing-f", P.bar("wing-f", 0.0, 0.3, 0.045), 0.04, 0.078, filled=False, like="wing", z=4, penMm=0.15)
    P.rev("neck-lower", "neck-1", lat=0.043, like="neck-lower", z=3.2)
    P.rev("neck-upper", "neck-3", lat=0.032, like="neck-upper", z=3.4)
    P.cap("head", "head", 0.205, 0, 0.255, 0, 0.035, 0.03, follow=True, like="head", z=4)
    P.poly("bill", "head", [(0.25, 0.019), (0.312, 0.012), (0.312, -0.012), (0.25, -0.019)], like="bill", z=4.1, filled=True, penMm=0.2)
    return F, P


NOTES = {
    50: ("Front view: the drake seen head-on, full articulation: oval breast and body, folded wings as flank bulges, three-piece neck column with the white ring, head with the bill disc, near and far leg as thigh, shank and tarsus strips with a flat webbed foot; legs follow the knee and heel travel of the poses. Plan view: spindle-shaped back with pointed tail, both folded wings as tapering strokes, three neck links and head with the spatulate bill; legs and webbed feet are drawn beneath the body and show only when a step swings them out.",
         "Vorderansicht: der Erpel von vorn, volle Gliederung: ovale Brust und Rumpf, angelegte Flügel als Flankenwölbungen, dreiteilige Halssäule mit weißem Ring, Kopf mit Schnabelscheibe, nahes und fernes Bein als Schenkel-, Unterschenkel- und Laufstreifen mit flachem Schwimmfuß; die Beine folgen dem Knie- und Fersenweg der Posen. Draufsicht: spindelförmiger Rücken mit spitzem Schwanz, beide angelegten Flügel als spitz zulaufende Striche, drei Halsglieder und Kopf mit löffelförmigem Schnabel; Beine und Schwimmfüße liegen unter dem Körper und zeigen sich nur, wenn ein Schritt sie herausschwingt."),
    100: ("Front view: articulated, head-on: body disc, two wing bulges, a two-piece neck, head with bill disc, and each leg as one polyline with a foot stroke following the shank. Plan view: spindle body with tail wedge, two tapering wings, two neck links, head and the spatulate bill; the legs are hidden strokes under the body.",
          "Vorderansicht: gegliedert, von vorn: Rumpfscheibe, zwei Flügelwölbungen, zweiteiliger Hals, Kopf mit Schnabelscheibe, jedes Bein als ein Linienzug mit Fußstrich, der dem Unterschenkel folgt. Draufsicht: Spindelrumpf mit Schwanzkeil, zwei spitze Flügel, zwei Halsglieder, Kopf und löffelförmiger Schnabel; die Beine sind versteckte Striche unter dem Körper."),
    200: ("Front view: composed from main masses: body disc, tail disc behind it, a two-piece neck, head and a leg stroke per side. Plan view: spindle body with tail wedge, the near wing as one edge stroke, neck links, head and a blunt bill.",
          "Vorderansicht: aus Hauptmassen zusammengesetzt: Rumpfscheibe, Schwanzscheibe dahinter, zweiteiliger Hals, Kopf und ein Beinstrich je Seite. Draufsicht: Spindelrumpf mit Schwanzkeil, der nahe Flügel als eine Randlinie, Halsglieder, Kopf und stumpfer Schnabel."),
}

BUILD = {50: build50, 100: build100, 200: build200}

if __name__ == "__main__":
    which = [int(x) for x in sys.argv[1:]] or [50, 100, 200]
    for sc, fn in ((50, build50), (100, build100), (200, build200)):
        if sc in which:
            F, P = fn()
            commit(A, sc, F.shapes, P.shapes, NOTES[sc][0], NOTES[sc][1])
            print("committed", sc, len(F.shapes), len(P.shapes))
