from kit import *

LO = {1: (0.88, 1.42), 2: (0.77, 1.42), 3: (0.75, 1.42), 4: (0.81, 1.41)}
HI = {1: (1.17, 1.92), 2: (1.17, 1.97), 3: (1.17, 1.97), 4: (1.17, 1.86)}
MATX = {1: (-0.9, -0.2), 2: (-0.57, 0.2), 3: (-0.16, 0.62), 4: (0.27, 0.98)}
STEMX = {1: -0.62, 2: -0.2, 3: 0.22, 4: 0.64}
HL = 0.125


def front_mat(scale):
    out = []
    for n in (1, 2, 3, 4):
        s = 1 if n % 2 else -1
        for part, rng, tone, reg, zb in (("lo", LO[n], 0.65, "leaves-lower", 6.0), ("hi", HI[n], 0.5, "leaves-upper", 6.1)):
            cy, ry = (rng[0] + rng[1]) / 2, (rng[1] - rng[0]) / 2
            b = f"fol-{n}-{part}"
            out.append(lobe(f"mat-{n}-{part}-0", b, s * 0.06, cy, 0.065, ry, 7 if scale <= 100 else 5, 0.16, tone, zb, phase=n, seed=n * 3 + (part == "hi"), region=reg))
            out.append(lobe(f"mat-{n}-{part}-1", b, -s * 0.05, cy - ry * 0.12, 0.06, ry * 0.85, 6 if scale <= 100 else 5, 0.16, tone, zb - 0.02, phase=n + 2, seed=n * 5 + (part == "hi"), region=reg))
    return out


def plan_mat(scale):
    out = []
    for n in (1, 2, 3, 4):
        x0, x1 = MATX[n]
        cx, rx = (x0 + x1) / 2, (x1 - x0) / 2
        for part, tone, reg, zb, ry in (("lo", 0.65, "leaves-lower", 6.0, 0.1), ("hi", 0.5, "leaves-upper", 6.1, HL)):
            b = f"fol-{n}-{part}"
            out.append(lobe(f"mat-{n}-{part}-0", b, cx, 0.0, rx * (0.95 if part == "lo" else 1.0), ry, 9 if scale <= 100 else 6, 0.16, tone, zb, phase=n, seed=n * 7 + (part == "hi"), region=reg))
    return out


def build(scale):
    front, plan = [], []
    if scale == 500:
        front.append(poly("screen", "panel", "body", 2.0, 0.5, [(-0.125, 0.0), (0.125, 0.0), (0.125, 0.45), (0.11, 1.0), (0.12, 1.5), (0.1, 2.05), (-0.1, 2.05), (-0.12, 1.5), (-0.11, 1.0), (-0.125, 0.45)], regions=["trellis-timber", "post-shoes", "leaves-lower", "leaves-upper"]))
        front.append(lobe("crown-L", "crown-L", 0.055, 2.04, 0.075, 0.09, 6, 0.16, 0.5, 3.0, seed=1, region="leaves-upper"))
        front.append(lobe("crown-R", "crown-R", -0.05, 2.04, 0.075, 0.09, 6, 0.16, 0.5, 3.0, seed=2, region="leaves-upper"))
        plan.append(poly("screen", "panel", "body", 2.0, 0.5, [(-0.96, -0.06), (-0.9, -0.125), (-0.3, -0.12), (0.3, -0.125), (0.9, -0.12), (0.96, -0.06), (0.96, 0.06), (0.9, 0.125), (0.3, 0.12), (-0.3, 0.125), (-0.9, 0.12), (-0.96, 0.06)], regions=["trellis-timber", "post-shoes", "leaves-lower", "leaves-upper"]))
        return front, plan
    # posts and shoes, rails
    front.append(rect("post-L", "post-L", "body", 1.9, 0.35, -0.05, 0.0, 0.1, 2.1, region="trellis-timber"))
    front.append(rect("post-R", "post-R", "body", 2.0, 0.35, -0.05, 0.0, 0.1, 2.1, region="trellis-timber"))
    front.append(rect("shoe-L", "post-L", "body", 3.9, 0.15, -0.065, 0.0, 0.13, 0.28, region="post-shoes"))
    front.append(rect("shoe-R", "post-R", "body", 4.0, 0.15, -0.065, 0.0, 0.13, 0.28, region="post-shoes"))
    front.append(rect("rail-top", "panel", "body", 3.0, 0.35, -0.035, 1.93, 0.07, 0.07, region="trellis-timber"))
    front.append(rect("rail-bottom", "panel", "body", 3.0, 0.35, -0.035, 0.35, 0.07, 0.07, region="trellis-timber"))
    plan.append(rect("post-L", "post-L", "body", 2.0, 0.35, -0.95, -0.05, 0.1, 0.1, region="trellis-timber"))
    plan.append(rect("post-R", "post-R", "body", 2.0, 0.35, 0.85, -0.05, 0.1, 0.1, region="trellis-timber"))
    plan.append(rect("shoe-L", "post-L", "body", 4.0, 0.15, -0.96, -0.065, 0.12, 0.13, region="post-shoes"))
    plan.append(rect("shoe-R", "post-R", "body", 4.0, 0.15, 0.84, -0.065, 0.12, 0.13, region="post-shoes"))
    plan.append(rect("rail-top", "panel", "body", 3.0, 0.35, -0.9, -0.035, 1.8, 0.07, region="trellis-timber"))
    front += front_mat(scale)
    plan += plan_mat(scale)
    if scale <= 100:
        front.append(poly("batten-h-0", "panel", "detail", 2.0, 0.4, [(-0.03, 0.6), (0.03, 0.6)], closed=False, region="trellis-timber"))
        for k in range(5):
            plan.append(poly(f"batten-v-{k}", "panel", "detail", 2.5, 0.4, [(-0.6 + 0.3 * k, -0.025), (-0.6 + 0.3 * k, 0.025)], closed=False, region="trellis-timber"))
        for n in (1, 2, 3, 4):
            x0, x1 = MATX[n]
            plan += arcs(f"leaf-{n}", f"fol-{n}-hi", (x0 + x1) / 2, 0.0, (x1 - x0) / 2, 0.1, 2 if scale == 100 else 4, 300 + n, 0.25, 7.0, role="detail", size=(0.08, 0.14))
            s = 1 if n % 2 else -1
            front += arcs(f"leaf-{n}", f"fol-{n}-hi", s * 0.06, 1.55, 0.065, 0.45, 2 if scale == 100 else 4, 310 + n, 0.25, 7.0, role="detail", size=(0.2, 0.35))
    if scale == 50:
        front.append(poly("batten-h-1", "panel", "detail", 2.0, 0.4, [(-0.03, 0.65), (0.03, 0.65)], closed=False, region="trellis-timber"))
        for n in (1, 2, 3, 4):
            s = 1 if n % 2 else -1
            front.append(poly(f"stem-{n}", f"stem-{n}", "detail", 5.0, 0.5, [(-s * 0.03, 0.05), (-s * 0.035, 1.0), (-s * 0.03, 1.9 + 0.02 * n)], closed=False, region="stems"))
            plan.append(poly(f"stem-{n}", f"stem-{n}", "detail", 5.0, 0.5, [(STEMX[n] - 0.02, -0.03), (STEMX[n] + 0.02, 0.03)], closed=False, region="stems"))
    return front, plan


NOTES = {
    500: ("Front: the screen seen end-on as one thin tall section with two ivy tufts on top. Plan: the screen as one 0.25 m band.",
          "Vorderansicht: die Wand im Querschnitt von der Stirnseite als eine schmale hohe Fläche mit zwei Efeubüscheln oben. Draufsicht: die Wand als ein 0,25 m breites Band."),
    200: ("Front: the screen seen end-on: near post, hidden far post, shoes, rail ends and the eight leaf masses on both faces, alternating left and right. Plan: posts, shoes, top rail and four leaf bands from above.",
          "Vorderansicht: die Wand von der Stirnseite: naher Pfosten, verdeckter ferner Pfosten, Schuhe, Riegelenden und die acht Blattmassen auf beiden Seiten, abwechselnd links und rechts. Draufsicht: Pfosten, Schuhe, oberer Riegel und vier Blattbänder von oben."),
    100: ("Front adds a batten cut and leaf arcs; plan adds five batten ends under the leaves and leaf arcs.",
          "Die Vorderansicht ergänzt einen Lattenschnitt und Blattbögen; die Draufsicht ergänzt fünf Lattenenden unter dem Laub und Blattbögen."),
    50: ("Front and plan add four leaf arcs per band, the second batten cut and the four stems as strokes through the leaf gaps.",
         "Vorder- und Draufsicht ergänzen vier Blattbögen je Band, den zweiten Lattenschnitt und die vier Triebe als Striche durch die Blattlücken."),
}

if __name__ == "__main__":
    views = {sc: build(sc) for sc in (500, 200, 100, 50)}
    install("ivy-on-trellis", views, NOTES)
    extents("ivy-on-trellis")
