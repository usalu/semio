import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "storage-crate-stack"
doc = load(A)
HC = "hdpe-crate"
TONES = {1: (0.45, 0.55), 2: (0.55, 0.65), 3: (0.45, 0.55), 4: (0.55, 0.65), 5: (0.45, 0.55)}


def z_of(i, off=0.0):
    return round(4.91 - 0.1 * (i - 1) + off, 3)


def build(scale):
    f, p = [], []
    for i in range(1, 6):
        b = f"crate-{i}"
        y0 = 0.3 * (i - 1)
        tb, tr = TONES[i]
        reg = dict(region=b, material=HC)
        if scale == 200:
            f.append(poly([(-0.17, y0), (0.17, y0), (0.2, y0 + 0.3), (-0.2, y0 + 0.3)], id=f"c{i}", bone=b, role="body", z=z_of(i), tone=tb, **reg))
            p.append(rect(-0.31, -0.2, 0.62, 0.4, rad=0.02, id=f"c{i}", bone=b, role="body", z=round(4.5 + 0.1 * (i - 1) + 0.01, 3), tone=tb, **reg))
        elif scale == 100:
            f += [
                poly([(-0.17, y0), (0.17, y0), (0.19, y0 + 0.275), (-0.19, y0 + 0.275)], id=f"c{i}-body", bone=b, role="body", z=z_of(i), tone=tb, **reg),
                rect(-0.2, y0 + 0.275, 0.4, 0.025, id=f"c{i}-rim", bone=b, role="body", z=z_of(i, 0.01), tone=tr, **reg),
            ]
            z = round(4.5 + 0.1 * (i - 1), 3)
            p += [
                rect(-0.31, -0.2, 0.62, 0.4, rad=0.02, id=f"c{i}-rim", bone=b, role="body", z=z, tone=tr, **reg),
                rect(-0.28, -0.17, 0.56, 0.34, rad=0.015, id=f"c{i}-body", bone=b, role="body", z=round(z + 0.01, 3), tone=tb, **reg),
            ]
        else:
            f += [
                rect(-0.15, y0, 0.3, 0.02, id=f"c{i}-foot", bone=b, role="body", z=z_of(i, -0.01), tone=round(tb - 0.1, 2), **reg),
                poly([(-0.17, y0 + 0.02), (0.17, y0 + 0.02), (0.19, y0 + 0.28), (-0.19, y0 + 0.28)], id=f"c{i}-body", bone=b, role="body", z=z_of(i), tone=tb, **reg),
                rect(-0.2, y0 + 0.28, 0.4, 0.02, id=f"c{i}-rim", bone=b, role="body", z=z_of(i, 0.01), tone=tr, **reg),
                rect(-0.07, y0 + 0.12, 0.14, 0.04, rad=0.01, id=f"c{i}-slot", bone=b, role="detail", z=z_of(i, 0.02), tone=0.3, filled=False, region=b),
                poly([(-0.16, y0 + 0.07), (0.16, y0 + 0.07)], closed=False, id=f"c{i}-rib", bone=b, role="detail", z=z_of(i, 0.02), tone=0.3, filled=False, region=b),
            ]
            z = round(4.5 + 0.1 * (i - 1), 3)
            p += [
                rect(-0.31, -0.2, 0.62, 0.4, rad=0.02, id=f"c{i}-rim", bone=b, role="body", z=z, tone=tr, **reg),
                rect(-0.28, -0.17, 0.56, 0.34, rad=0.015, id=f"c{i}-body", bone=b, role="body", z=round(z + 0.01, 3), tone=tb, **reg),
                rect(-0.25, -0.14, 0.5, 0.28, rad=0.01, id=f"c{i}-floor", bone=b, role="detail", z=round(z + 0.02, 3), tone=round(tb - 0.15, 2), filled=True, region=b, material=HC),
                poly([(-0.25, 0.0), (0.25, 0.0)], closed=False, id=f"c{i}-floor-rib-x", bone=b, role="detail", z=round(z + 0.03, 3), tone=0.3, filled=False, region=b),
                poly([(0.0, -0.14), (0.0, 0.14)], closed=False, id=f"c{i}-floor-rib-y", bone=b, role="detail", z=round(z + 0.03, 3), tone=0.3, filled=False, region=b),
            ]
    return f, p


T = {
    200: (
        ("Front view: five stacked trapezoids, each 0.4 m wide at its rim, which sink into the one below as the crates nest. Plan view: one rounded rectangle per crate; the crates lie exactly above each other, so only the top crate shows and the nesting reads in the front and side views.",
         "Vorderansicht: fünf gestapelte Trapeze, am Rand je 0,4 m breit, die sich beim Ineinanderschieben in die darunter liegende Kiste senken. Draufsicht: pro Kiste ein abgerundetes Rechteck; die Kisten liegen genau übereinander, nur die oberste ist zu sehen, das Ineinanderschieben liest sich in Vorder- und Seitenansicht."),
        ("Front and plan view: five crates, each its own region.",
         "Vorder- und Draufsicht: fünf Kisten, jede mit eigener Region."),
        ("Front and plan view: no rim, no foot, no handle slots.",
         "Vorder- und Draufsicht: kein Rand, kein Fuß, keine Griffschlitze."),
    ),
    100: (
        ("Front view: five crates as a tapering body with a separate rim band, alternating light and dark tones so that the nested crates stay apart. Plan view: per crate the rim ring and the inner wall outline; the lower crates are covered by the top one and show in the thermal views only.",
         "Vorderansicht: fünf Kisten als konischer Körper mit eigenem Randband, abwechselnd hell und dunkel, damit die ineinander geschobenen Kisten getrennt bleiben. Draufsicht: pro Kiste Randring und innere Wandkontur; die unteren Kisten verdeckt die oberste, sie erscheinen nur in den thermischen Ansichten."),
        ("Front and plan view: body, rim band and inner wall of every crate.",
         "Vorder- und Draufsicht: Körper, Randband und Innenwand jeder Kiste."),
        ("Front and plan view: foot, handle slot and floor ribs are not drawn.",
         "Vorder- und Draufsicht: Fuß, Griffschlitz und Bodenrippen sind nicht gezeichnet."),
    ),
    50: (
        ("Front view: five crates with foot, tapering end wall, rim band, one central handle slot and a moulded rib, each nesting into the one below when the stack is lowered. Plan view: per crate the rim ring, the inner wall, the floor with its two ribs; the stack is vertical, so only the top crate is seen from above.",
         "Vorderansicht: fünf Kisten mit Fuß, konischer Stirnwand, Randband, einem mittigen Griffschlitz und einer Formrippe; jede schiebt sich beim Absenken des Stapels in die darunter liegende. Draufsicht: pro Kiste Randring, Innenwand und Boden mit zwei Rippen; der Stapel steht senkrecht, von oben ist nur die oberste Kiste zu sehen."),
        ("Front view: handle slot, rib, foot. Plan view: floor with ribs.",
         "Vorderansicht: Griffschlitz, Rippe, Fuß. Draufsicht: Boden mit Rippen."),
        ("Front and plan view: stacking lugs and the hand-hole texture are not drawn.",
         "Vorder- und Draufsicht: Stapelnocken und die Griffmuldentextur sind nicht gezeichnet."),
    ),
}

for scale in (200, 100, 50):
    f, p = build(scale)
    s, ret, omi = T[scale]
    install(doc, scale, f, p, {"silhouette": s, "retained": ret, "omitted": omi})
finish(doc)
save(A, doc)
print("[DEBUG] crates written")
