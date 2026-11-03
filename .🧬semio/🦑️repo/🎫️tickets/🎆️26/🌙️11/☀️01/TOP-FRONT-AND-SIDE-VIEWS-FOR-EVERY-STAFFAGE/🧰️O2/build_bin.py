import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "waste-bin"
doc = load(A)
HD, LD, FILM = "hdpe-plastic", "hdpe-plastic", "ldpe-film"
HINGE_X, HINGE_Y = -0.255, 0.98

bones = doc["asset"]["rig"]["bones"]
bones[:] = [x for x in bones if x["id"] != "lid-front"]
SOL = {"kind": "extruded", "z0": -0.26, "z1": 0.26}
LIDMAIN = "M 0 0 L 0.52 0 L 0.52 0.006 C 0.52 0.03 0.45 0.04 0.4 0.038 L 0.12 0.038 C 0.05 0.04 0 0.03 0 0.006 Z"
LID200 = "M 0 -0.02 L 0.52 -0.02 L 0.52 0.006 C 0.52 0.03 0.45 0.04 0.4 0.038 L 0.12 0.038 C 0.05 0.04 0 0.03 0 0.006 Z"


def lid_parts(scale):
    k = dict(bone="lid", role="body", region="lid-hdpe", material=LD, solid=SOL)
    if scale == 500:
        return [rect(0, 0, 0.52, 0.04, id="lid", z=5, tone=0.55, **k)]
    if scale == 200:
        return [path(LID200, id="lid", z=5, tone=0.55, **k)]
    out = [path(LIDMAIN, id="lid", z=5, tone=0.55, **k), rect(0, -0.028, 0.52, 0.03, rad=0.006, id="lid-skirt", z=4.9, tone=0.45, **k)]
    if scale == 50:
        out += [
            path("M 0.38 0.038 C 0.39 0.058 0.47 0.058 0.48 0.038 Z", id="lid-handle", bone="lid", role="detail", z=5.1, tone=0.35, region="lid-hdpe", material=LD, solid={"kind": "extruded", "z0": -0.05, "z1": 0.05}),
            ellipse(0.0, 0.004, 0.014, 0.014, id="hinge-barrel", bone="lid", role="detail", z=5.3, tone=0.25, region="lid-hdpe", material=LD, solid={"kind": "extruded", "z0": -0.13, "z1": 0.13}),
        ]
    return out


CORNER_R = 0.04
STEPS = {500: 2, 200: 3, 100: 4, 50: 6}


def corner_steps(scale):
    if scale == 50:
        return [(0.0, 0.014, 0.02), (0.014, 0.03, 0.0075)]
    if scale == 100:
        return [(0.0, 0.008, 0.016)]
    n = STEPS[scale]
    edges = [CORNER_R * i / n for i in range(n + 1)]
    return [(edges[i], edges[i + 1], CORNER_R - math.sqrt(CORNER_R**2 - (CORNER_R - (edges[i] + edges[i + 1]) / 2) ** 2)) for i in range(n)]


def lid_plan_parts(scale):
    k = dict(bone="lid", role="body", region="lid-hdpe", material=LD)
    out = []
    for tag, z, tone, y0, y1, pen in (("lid", 5, 0.55, 0.0, 0.038, 0.35), ("lid-skirt", 4.9, 0.45, -0.028, 0.002, 0.25)):
        if scale in (500, 200) and tag == "lid-skirt":
            continue
        pen = pen if scale == 50 else None
        if tag == "lid-skirt":
            out.append(rect(0.0, y0, 0.52, y1 - y0, id=tag, z=z, tone=tone, solid=SOL, pen=pen, **k))
            continue
        for i, (a, b, inset) in enumerate(corner_steps(scale)):
            half = 0.26 - inset
            for side, (x0, x1) in (("a", (a, b)), ("b", (0.52 - b, 0.52 - a))):
                out.append(rect(x0, y0, x1 - x0, y1 - y0, id=f"{tag}-corner-{side}{i}", z=z, tone=tone, solid={"kind": "extruded", "z0": r(-half), "z1": r(half)}, pen=pen, **k))
        last = corner_steps(scale)[-1][1]
        out.append(rect(last, y0, 0.52 - 2 * last, y1 - y0, id=f"{tag}-span", z=z, tone=tone, solid=SOL, pen=pen, **k))
    if scale == 50:
        handle, barrel = lid_parts(50)[2], lid_parts(50)[3]
        handle.update({"filled": False, "pen": 0.25, "penMm": 0.25})
        handle.pop("pen")
        out += [handle, barrel]
    return out


def build(scale):
    f, p = [], []
    if scale == 500:
        f += [
            poly([(-0.2, 0), (0.2, 0), (0.2, 0.025), (0.255, 0.94), (0.255, 0.98), (-0.255, 0.98), (-0.255, 0.94), (-0.2, 0.025)], id="body", bone="body", role="body", z=3, tone=0.42, regions=["bin-hdpe", "bin-ribs", "liner-film"], material=HD),
        ]
        p += [
            rect(-0.255, -0.255, 0.51, 0.51, rad=0.04, id="body", bone="body", role="body", z=3, tone=0.42, regions=["bin-hdpe", "bin-ribs", "liner-film"], material=HD),
        ]
    elif scale == 200:
        f += [
            poly([(-0.2, 0), (0.2, 0), (0.2, 0.025), (0.248, 0.94), (-0.248, 0.94), (-0.2, 0.025)], id="body-silhouette", bone="body", role="body", z=3, tone=0.42, region="bin-hdpe", material=HD),
            poly([(-0.206, 0.15), (0.206, 0.15)], closed=False, id="band-0", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.222, 0.45), (0.222, 0.45)], closed=False, id="band-1", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.236, 0.72), (0.236, 0.72)], closed=False, id="band-2", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            rect(-0.255, 0.94, 0.51, 0.04, id="rim", bone="body", role="body", z=4, tone=0.5, region="bin-hdpe", material=HD),
            poly([(-0.2, 0.925), (0.2, 0.925)], closed=False, id="liner-collar", bone="body", role="detail", z=4.2, tone=0.5, filled=False, region="liner-film", material=FILM),
        ]
        p += [
            rect(-0.255, -0.255, 0.51, 0.51, rad=0.04, id="body-silhouette", bone="body", role="body", z=3, tone=0.42, region="bin-hdpe", material=HD),
            rect(-0.2, -0.2, 0.4, 0.4, rad=0.03, id="opening", bone="body", role="detail", z=3.3, tone=0.15, region="liner-film", material=FILM),
        ]
    elif scale == 100:
        f += [
            rect(-0.2, 0.0, 0.4, 0.025, id="pedestal", bone="body", role="body", z=1, tone=0.3, region="bin-hdpe", material=HD),
            poly([(-0.2, 0.025), (0.2, 0.025), (0.248, 0.94), (-0.248, 0.94)], id="shell", bone="body", role="body", z=3, tone=0.42, region="bin-hdpe", material=HD),
            poly([(-0.206, 0.15), (0.206, 0.15)], closed=False, id="band-0", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.222, 0.45), (0.222, 0.45)], closed=False, id="band-1", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.236, 0.72), (0.236, 0.72)], closed=False, id="band-2", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            rect(-0.255, 0.94, 0.51, 0.04, rad=0.008, id="rim", bone="body", role="body", z=4, tone=0.5, region="bin-hdpe", material=HD),
            poly([(-0.24, 0.925), (0.24, 0.925)], closed=False, id="liner-collar", bone="body", role="detail", z=4.2, tone=0.5, filled=False, region="liner-film", material=FILM),
            rect(-0.2, 0.925, 0.026, 0.065, id="hinge-bracket-r", bone="body", role="body", z=2.9, tone=0.3, region="bin-hdpe", material=HD),
            rect(0.174, 0.925, 0.026, 0.065, id="hinge-bracket-l", bone="body", role="body", z=2.9, tone=0.3, region="bin-hdpe", material=HD),
        ]
        p += [
            rect(-0.255, -0.255, 0.51, 0.51, rad=0.04, id="shell", bone="body", role="body", z=3, tone=0.42, region="bin-hdpe", material=HD),
            rect(-0.235, -0.235, 0.47, 0.47, rad=0.035, id="rim-inner", bone="body", role="detail", z=3.1, tone=0.3, filled=False, region="bin-ribs", material=HD),
            rect(-0.2, -0.2, 0.4, 0.4, rad=0.03, id="opening", bone="body", role="detail", z=3.3, tone=0.15, region="liner-film", material=FILM),
            rect(-0.268, 0.16, 0.026, 0.06, id="hinge-bracket-l", bone="body", role="body", z=4.5, tone=0.3, region="bin-hdpe", material=HD),
            rect(-0.268, -0.22, 0.026, 0.06, id="hinge-bracket-r", bone="body", role="body", z=4.5, tone=0.3, region="bin-hdpe", material=HD),
        ]
    else:
        f += [
            path("M -0.2 0 L 0.2 0 L 0.2 0.012 C 0.2 0.02 0.196 0.025 0.19 0.025 L -0.19 0.025 C -0.196 0.025 -0.2 0.02 -0.2 0.012 Z", id="pedestal", bone="body", role="body", z=1, tone=0.3, region="bin-hdpe", material=HD, pen=0.35),
            rect(-0.2, 0.0, 0.05, 0.008, rad=0.002, id="pedestal-foot-l", bone="body", role="detail", z=1.1, tone=0.15, region="bin-hdpe", material=HD),
            rect(0.15, 0.0, 0.05, 0.008, rad=0.002, id="pedestal-foot-r", bone="body", role="detail", z=1.1, tone=0.15, region="bin-hdpe", material=HD),
            path("M -0.2 0.025 L 0.2 0.025 C 0.215 0.3 0.235 0.7 0.248 0.955 L -0.248 0.955 C -0.235 0.7 -0.215 0.3 -0.2 0.025 Z", id="shell", bone="body", role="body", z=3, tone=0.42, region="bin-hdpe", material=HD, pen=0.422),
            poly([(-0.2085, 0.15), (0.2085, 0.15)], closed=False, id="band-0", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.2097, 0.168), (0.2097, 0.168)], closed=False, id="band-0-bead", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.2267, 0.45), (0.2267, 0.45)], closed=False, id="band-1", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.2277, 0.468), (0.2277, 0.468)], closed=False, id="band-1-bead", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.2402, 0.72), (0.2402, 0.72)], closed=False, id="band-2", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            poly([(-0.241, 0.738), (0.241, 0.738)], closed=False, id="band-2-bead", bone="body", role="detail", z=3.5, tone=0.3, filled=False, region="bin-ribs", material=HD),
            path("M -0.15 0.30 C -0.05 0.31 0.05 0.29 0.15 0.30 M -0.14 0.58 C -0.05 0.59 0.06 0.57 0.14 0.58 M -0.12 0.85 C -0.04 0.86 0.05 0.84 0.12 0.85", id="shell-texture", bone="body", role="texture", z=3.6, tone=0.3, filled=False, region="bin-ribs", material=HD),
            rect(-0.255, 0.94, 0.51, 0.04, rad=0.008, id="rim", bone="body", role="body", z=4, tone=0.5, region="bin-hdpe", material=HD, pen=0.35),
            poly([(-0.235, 0.955), (0.235, 0.955)], closed=False, id="rim-inner", bone="body", role="detail", z=4.1, tone=0.3, filled=False, region="bin-ribs", material=HD),
            path("M -0.245 0.945 C -0.20 0.915 -0.10 0.930 0.0 0.918 C 0.10 0.930 0.20 0.915 0.245 0.945", id="liner-collar", bone="body", role="detail", z=4.2, tone=0.5, filled=False, region="liner-film", material=FILM),
            rect(-0.2458, 0.895, 0.4915, 0.012, id="liner-ring", bone="body", role="detail", z=3.8, tone=0.3, filled=False, region="bin-ribs", material=HD),
            rect(-0.2, 0.925, 0.026, 0.065, rad=0.004, id="hinge-bracket-r", bone="body", role="body", z=2.9, tone=0.3, region="bin-hdpe", material=HD),
            rect(0.174, 0.925, 0.026, 0.065, rad=0.004, id="hinge-bracket-l", bone="body", role="body", z=2.9, tone=0.3, region="bin-hdpe", material=HD),
        ]
        p += [
            rect(-0.255, -0.255, 0.51, 0.51, rad=0.04, id="shell", bone="body", role="body", z=3, tone=0.42, region="bin-hdpe", material=HD, pen=0.18),
            rect(-0.235, -0.235, 0.47, 0.47, rad=0.035, id="rim-inner", bone="body", role="detail", z=3.1, tone=0.3, filled=False, region="bin-ribs", material=HD),
            rect(-0.22, -0.22, 0.44, 0.44, rad=0.032, id="liner-collar", bone="body", role="detail", z=3.2, tone=0.5, filled=False, region="liner-film", material=FILM),
            rect(-0.2, -0.2, 0.4, 0.4, rad=0.03, id="opening", bone="body", role="detail", z=3.3, tone=0.15, region="liner-film", material=FILM),
            rect(-0.268, 0.16, 0.026, 0.06, rad=0.004, id="hinge-bracket-l", bone="body", role="body", z=4.5, tone=0.3, region="bin-hdpe", material=HD),
            rect(-0.268, -0.22, 0.026, 0.06, rad=0.004, id="hinge-bracket-r", bone="body", role="body", z=4.5, tone=0.3, region="bin-hdpe", material=HD),
        ]
    f += lid_parts(scale)
    p += lid_plan_parts(scale)
    return f, p


T = {
    500: (
        ("Front view: one body silhouette 1.0 mm wide and the lid as a single thin plate. Plan view: the bin top as one rounded square and the lid as a rectangle with the rim's rounded corners, stepped from extruded strips. The lid is an extruded solid on its hinge bone: an open lid stands up as a tall plate in the front view and collapses to a thin strip at the hinge in the plan.",
         "Vorderansicht: eine einzige Körperkontur von 1,0 mm Breite und der Deckel als dünne Platte. Draufsicht: die Behälteröffnung als ein abgerundetes Quadrat und der Deckel als Rechteck mit den gerundeten Ecken des Rands, aus extrudierten Streifen gestuft. Der Deckel ist ein extrudierter Körper am Scharnierknochen: ein offener Deckel steht in der Vorderansicht als hohe Platte und fällt in der Draufsicht zu einem dünnen Streifen am Scharnier zusammen."),
        ("Front view: body outline, lid. Plan view: top outline, lid.",
         "Vorderansicht: Körperkontur, Deckel. Draufsicht: Öffnungskontur, Deckel."),
        ("Front and plan view: shell, rim and liner share one outline.",
         "Vorder- und Draufsicht: Schale, Rand und Beutel teilen sich eine Kontur."),
        ("Front and plan view: bands, hinge and handle are not drawn.",
         "Vorder- und Draufsicht: Bänder, Scharnier und Griff sind nicht gezeichnet."),
    ),
    200: (
        ("Front view: the tapering shell with three ribs, rim, liner collar and the lid plate. Plan view: the square rim, the dark bin opening and the lid as a rectangle with the rim's rounded corners, stepped from extruded strips. The lid is an extruded solid on its hinge bone, so an open lid rises as a tall plate in the front view and leaves the opening free in the plan.",
         "Vorderansicht: die konische Schale mit drei Rippen, Rand, Beutelkragen und die Deckelplatte. Draufsicht: der quadratische Rand, die dunkle Behälteröffnung und der Deckel als Rechteck mit den gerundeten Ecken des Rands, aus extrudierten Streifen gestuft. Der Deckel ist ein extrudierter Körper am Scharnierknochen, ein offener Deckel steigt in der Vorderansicht als hohe Platte auf und gibt in der Draufsicht die Öffnung frei."),
        ("Front view: three ribs, liner collar, lid. Plan view: opening, lid.",
         "Vorderansicht: drei Rippen, Beutelkragen, Deckel. Draufsicht: Öffnung, Deckel."),
        ("Plan view: the lid corners follow the 0.04 m rim radius in steps no finer than the print profile allows.",
         "Draufsicht: die Deckelecken folgen dem Randradius von 0,04 m in Stufen, die nicht feiner sind als das Druckprofil erlaubt."),
        ("Front and plan view: no hinge brackets, no handle.",
         "Vorder- und Draufsicht: keine Scharnierwinkel, kein Griff."),
    ),
    100: (
        ("Front view: pedestal, tapering shell with three ribs, rim, liner collar, both hinge brackets behind the rim, the lid skirt and the lid plate. Plan view: rim with inner rim line, the dark opening, both hinge brackets, and the lid with its skirt as two stacked rectangles whose corners follow the rim radius in one step. Lid plate and skirt are extruded solids on the hinge bone: open, the plate stands tall in the front view and shrinks to a narrow band behind the hinge in the plan.",
         "Vorderansicht: Sockel, konische Schale mit drei Rippen, Rand, Beutelkragen, beide Scharnierwinkel hinter dem Rand, die Deckelschürze und die Deckelplatte. Draufsicht: Rand mit innerer Randlinie, die dunkle Öffnung, beide Scharnierwinkel und der Deckel mit Schürze als zwei übereinanderliegende Rechtecke, deren Ecken dem Randradius in einer Stufe folgen. Deckelplatte und Schürze sind extrudierte Körper am Scharnierknochen: offen steht die Platte in der Vorderansicht hoch und schrumpft in der Draufsicht zu einem schmalen Band hinter dem Scharnier."),
        ("Front view: bands, liner collar, brackets, skirt. Plan view: inner rim line, opening, brackets, skirt.",
         "Vorderansicht: Bänder, Beutelkragen, Winkel, Schürze. Draufsicht: innere Randlinie, Öffnung, Winkel, Schürze."),
        ("Plan view: lid and skirt overlap as one footprint.",
         "Draufsicht: Deckel und Schürze überlagern sich zu einer Grundfläche."),
        ("Front and plan view: handle, bead lines and shell texture are not drawn.",
         "Vorder- und Draufsicht: Griff, Wulstlinien und Schalentextur sind nicht gezeichnet."),
    ),
    50: (
        ("Front view: pedestal with feet, tapering shell with ribs, beads and texture, rim, liner collar and ring, hinge brackets, lid skirt, lid plate, handle and hinge barrel. Plan view: shell, rim line, liner collar, dark opening, hinge brackets, lid with skirt (corners following the rim radius in two steps), the 0.10 m handle and the 0.26 m hinge barrel. Plate, skirt, handle and barrel are extruded solids on the lid bone, so an open lid stands up as a tall plate in the front view and folds onto the hinge in the plan.",
         "Vorderansicht: Sockel mit Füßen, konische Schale mit Rippen, Wülsten und Textur, Rand, Beutelkragen und -ring, Scharnierwinkel, Deckelschürze, Deckelplatte, Griff und Scharnierwalze. Draufsicht: Schale, Randlinie, Beutelkragen, dunkle Öffnung, Scharnierwinkel, Deckel mit Schürze (Ecken dem Randradius in zwei Stufen folgend), der 0,10 m breite Griff und die 0,26 m lange Scharnierwalze. Platte, Schürze, Griff und Walze sind extrudierte Körper am Deckelknochen: ein offener Deckel steht in der Vorderansicht als hohe Platte und klappt in der Draufsicht auf das Scharnier."),
        ("Front view: feet, beads, texture, liner ring, hinge barrel, handle. Plan view: rim line, liner collar, opening, handle, hinge barrel.",
         "Vorderansicht: Füße, Wülste, Textur, Beutelring, Scharnierwalze, Griff. Draufsicht: Randlinie, Beutelkragen, Öffnung, Griff, Scharnierwalze."),
        ("Plan view: lid plate and skirt overlap as one footprint.",
         "Draufsicht: Deckelplatte und Schürze überlagern sich zu einer Grundfläche."),
        ("Plan view: the feet below the shell and the skirt line are not drawn.",
         "Draufsicht: die Füße unter der Schale und die Schürzenlinie sind nicht gezeichnet."),
    ),
}

for scale in (500, 200, 100, 50):
    f, p = build(scale)
    s, ret, mer, omi = T[scale]
    install(doc, scale, f, p, {"silhouette": s, "retained": ret, "merged": mer, "omitted": omi})
finish(doc)
save(A, doc)
print("[DEBUG] bin written")
