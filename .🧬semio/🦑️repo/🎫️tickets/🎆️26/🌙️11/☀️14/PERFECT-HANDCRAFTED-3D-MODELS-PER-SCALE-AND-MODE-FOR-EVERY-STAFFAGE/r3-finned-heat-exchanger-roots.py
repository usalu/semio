"""🌱 Round 3, machinery/finned-heat-exchanger: the root corrections outside `volumes` (dimensions, materials, rig, anchor, thermal regions, pose bindings, texts) and the 2D drawings of 1:50, 1:100 and 1:200 (elevation section A-A across the tubes, front seen from the air-leaving side, plan), drawn by hand from the measures of `r3-finned-heat-exchanger-geo.py`.

Usage: imported by `r3-finned-heat-exchanger-build.py` (`apply_roots(asset)`).
"""
import importlib.util
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("fhx_geo", os.path.join(HERE, "r3-finned-heat-exchanger-geo.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)

STEEL, ALU, CU = "galvanised-steel", "aluminium-fin", "copper"
TONE = {"casing": 0.5, "fins": 0.78, "tubes": 0.4, "headers": 0.4}
MAT = {"casing": STEEL, "fins": ALU, "tubes": CU, "headers": CU}
BONE = {"casing": "frame", "fins": "fins", "tubes": "tubes", "headers": "tubes"}


def shape(id, region, z, geom, role="body", filled=True, tone=None, **kw):
    """✏️ One 2D shape with the common members in the catalog's order; bone, tone and material follow the region."""
    out = {"id": id, "bone": BONE[region], "role": role, "z": z, "filled": filled, "tone": TONE[region] if tone is None else tone}
    out.update(geom)
    if "regions" in kw:
        out["regions"] = kw.pop("regions")
    else:
        out["region"] = region
    out["material"] = MAT[region]
    out.update(kw)
    return out


def poly(points, closed=True):
    """📐 Poly geometry."""
    return {"kind": "poly", "points": [[round(x, 6), round(y, 6)] for x, y in points], "closed": closed}


def rect(x0, y0, x1, y1, r=None):
    """▭ Rect geometry between two corners."""
    out = {"kind": "rect", "x": round(x0, 6), "y": round(y0, 6), "w": round(x1 - x0, 6), "h": round(y1 - y0, 6)}
    if r:
        out["r"] = r
    return out


def disc(cx, cy, rx, ry=None):
    """⭕ Ellipse geometry."""
    return {"kind": "ellipse", "cx": round(cx, 6), "cy": round(cy, 6), "rx": rx, "ry": rx if ry is None else ry}


def el(region, pts):
    """↘ World rest points (x ahead, y up) into the elevation frame of the region's vertical bone."""
    return [G.loc(BONE[region], x, y) for x, y in pts]


def box(x0, y0, x1, y1):
    """⬜ Corner points of a world rectangle."""
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


def elevation(s):
    """🖼️ Elevation drawn as section A-A across the tubes at mid length (z = 0), seen from the right: the left tube sheet beyond, a fin seen flat with the 24 cut tubes in their staggered pattern, the two channels cut."""
    x, t = G.CASE_X, G.SHEET
    if s == 200:
        return [
            shape("end-plate-left", "casing", 3.2, poly(el("casing", box(-x, 0.0, x, G.TOP))), regions=["casing", "headers"]),
            shape("coil-block", "fins", 3.3, poly(el("fins", box(-G.FIN_X, 0.03, G.FIN_X, 0.49))), regions=["fins", "tubes"]),
            shape("casing-bottom", "casing", 3.4, poly(el("casing", box(-x, 0.0, x, 0.03))), tone=0.35),
            shape("casing-top", "casing", 3.4, poly(el("casing", box(-x, 0.49, x, G.TOP))), tone=0.35),
        ]
    out = [
        shape("tube-sheet-left", "casing", 3.2, poly(el("casing", box(-x, 0.0, x, G.TOP))), regions=["casing", "headers"]),
        shape("fin-pack", "fins", 3.3, poly(el("fins", box(-G.FIN_X, G.FIN_Y[0], G.FIN_X, G.FIN_Y[1])))),
    ]
    n = 0
    for row in (0, 1):
        for k in range(G.TUBES_PER_ROW):
            n += 1
            c = G.loc("tubes", G.ROW_X[row], G.tube_y(row, k))
            out.append(shape(f"tube-{n}", "tubes", 3.6, disc(c[0], c[1], G.TUBE_R), penMm=0.13 if s == 50 else 0.1))
    out.append(shape("casing-bottom", "casing", 3.5, poly(el("casing", channel_cut(False))), tone=0.25))
    out.append(shape("casing-top", "casing", 3.5, poly(el("casing", channel_cut(True))), tone=0.25))
    return out


def channel_cut(up):
    """🛤️ The cut U profile of a channel at its real 1.5 mm sheet (world x, y)."""
    x, t = G.CASE_X, G.SHEET
    if up:
        y0, y1 = G.FIN_Y[1], G.TOP
        return [[-x, y1], [-x, y0], [x, y0], [x, y1], [x - t, y1], [x - t, y0 + t], [-x + t, y0 + t], [-x + t, y1]]
    y0, y1 = 0.0, G.FIN_Y[0]
    return [[-x, y0], [-x + t, y0], [-x + t, y1 - t], [x - t, y1 - t], [x - t, y0], [x, y0], [x, y1], [-x, y1]]


def hairpin_stack(n):
    """🔁 Front outline of the twelve hairpins as one scalloped strip: each hairpin an elliptical nose (n segments) beyond the right tube sheet, the 4 mm gaps between neighbours closed."""
    a, b, zc, zs = G.BEND_R + G.TUBE_R, 0.01 + G.TUBE_R, G.BEND_Z0, -G.FIN_Z
    pts = [[zs, G.tube_y(0, 0) - G.TUBE_R]]
    for k in range(G.TUBES_PER_ROW):
        yc = G.tube_y(0, k) + 0.01
        for i in range(n + 1):
            ang = -0.5 * math.pi + math.pi * i / n
            pts.append([zc - a * math.cos(ang), yc + b * math.sin(ang)])
    pts.append([zs, G.tube_y(1, G.TUBES_PER_ROW - 1) + G.TUBE_R])
    return pts


def stub_outline(y, s, z0):
    """🚰 Outline of a stub seen from the side (lateral coordinate first, then the coordinate across about y): pipe and, at 1:50, the threaded sleeve."""
    a, r = G.STUB_END, G.HEADER_R
    if s == 50:
        b, q = a - G.SLEEVE_L, G.SLEEVE_R
        return [[z0, y - r], [b, y - r], [b, y - q], [a, y - q], [a, y + q], [b, y + q], [b, y + r], [z0, y + r]]
    return [[z0, y - r], [a, y - r], [a, y + r], [z0, y + r]]


def front(s):
    """🖼️ Front seen from ahead (the air-leaving face): the fin face hatched with its vertical fin edges inside the frame of flanges and channel legs, the hairpins beyond the right flange, the two headers with their stubs beyond the left flange."""
    if s == 200:
        return [
            shape("outlet-stub", "headers", 3.1, rect(0.26, G.STUB_Y[0] - 0.015, G.STUB_END, G.STUB_Y[0] + 0.015)),
            shape("inlet-stub", "headers", 3.1, rect(0.26, G.STUB_Y[1] - 0.015, G.STUB_END, G.STUB_Y[1] + 0.015)),
            shape("header-block", "headers", 3.2, rect(0.23, 0.012, 0.26, 0.508)),
            shape("coil-block", "fins", 3.3, rect(-G.FIN_Z, 0.03, G.FIN_Z, 0.49), regions=["fins", "tubes"]),
            shape("casing-bottom", "casing", 3.4, rect(-G.FIN_Z, 0.0, G.FIN_Z, 0.03)),
            shape("casing-top", "casing", 3.4, rect(-G.FIN_Z, 0.49, G.FIN_Z, G.TOP)),
            shape("end-plate-left", "casing", 3.5, rect(G.FIN_Z, 0.0, 0.23, G.TOP)),
            shape("end-plate-right", "casing", 3.5, rect(G.RIGHT, 0.0, -G.FIN_Z, G.TOP)),
        ]
    out = [shape("hairpins", "tubes", 3.1, poly(hairpin_stack(8 if s == 50 else 4)))]
    z0, z1 = G.HEADER_Z - G.HEADER_R, G.HEADER_Z + G.HEADER_R
    cap = 0.007 if s == 50 else None
    out.append(shape("header-return", "headers", 3.12, rect(z0, G.HEADER_Y[0][0], z1, G.HEADER_Y[0][1], cap)))
    out.append(shape("outlet-stub", "headers", 3.14, poly(stub_outline(G.STUB_Y[0], s, G.HEADER_Z))))
    out.append(shape("header-supply", "headers", 3.2, rect(z0, G.HEADER_Y[1][0], z1, G.HEADER_Y[1][1], cap)))
    if s == 50:
        out.append(shape("drain-plug", "headers", 3.22, rect(G.HEADER_Z - G.PLUG_R, G.HEADER_Y[1][0] - G.PLUG_L, G.HEADER_Z + G.PLUG_R, G.HEADER_Y[1][0] + 0.002), tone=0.3))
    out.append(shape("inlet-stub", "headers", 3.25, poly(stub_outline(G.STUB_Y[1], s, G.HEADER_Z))))
    out.append(shape("fin-pack", "fins", 3.3, rect(-G.FIN_Z, G.FIN_Y[0], G.FIN_Z, G.FIN_Y[1]), hatch="fins"))
    out.append(shape("casing-bottom", "casing", 3.4, rect(-G.FIN_Z, 0.0, G.FIN_Z, G.CASE_H)))
    out.append(shape("casing-top", "casing", 3.4, rect(-G.FIN_Z, G.FIN_Y[1], G.FIN_Z, G.TOP)))
    out.append(shape("tube-sheet-left", "casing", 3.5, rect(G.FIN_Z, 0.0, G.FLANGE_Z, G.TOP)))
    out.append(shape("tube-sheet-right", "casing", 3.5, rect(-G.FLANGE_Z, 0.0, -G.FIN_Z, G.TOP)))
    return out


def hairpin_plan():
    """🔁 Plan outline of the stack of hairpins (all twelve project onto one U): outer and inner edge of the bent tube from the right tube sheet around the bend."""
    a, b, r, z0, zs = G.ROW_X[1], G.BEND_R, G.TUBE_R, G.BEND_Z0, -G.SHEET_Z
    n = 10
    outer = [[(a + r) * math.cos(math.pi * i / n), z0 - (b + r) * math.sin(math.pi * i / n)] for i in range(n + 1)]
    inner = [[(a - r) * math.cos(math.pi * i / n), z0 - (b - r) * math.sin(math.pi * i / n)] for i in range(n, -1, -1)]
    return [[a + r, zs]] + outer + [[-a - r, zs], [-a + r, zs]] + inner + [[a - r, zs]]


def swap(pts):
    """🔀 Lateral-first points into the plan frame (x ahead, y lateral)."""
    return [[b, a] for a, b in pts]


def plan(s):
    """🖼️ Plan seen from above: the top channel between the tube sheets, their flanges, the hairpins beyond the right tube sheet, the tube ends, the two headers and their stubs beyond the left one."""
    x = G.CASE_X
    if s == 200:
        return [
            shape("outlet-stub", "headers", 3.1, rect(G.ROW_X[0] - 0.015, 0.26, G.ROW_X[0] + 0.015, G.STUB_END)),
            shape("inlet-stub", "headers", 3.1, rect(G.ROW_X[1] - 0.015, 0.26, G.ROW_X[1] + 0.015, G.STUB_END)),
            shape("header-block", "headers", 3.2, rect(-0.035, 0.23, 0.035, 0.26)),
            shape("casing-top", "casing", 3.4, rect(-x, -G.FIN_Z, x, G.FIN_Z), regions=["casing", "fins", "tubes"]),
            shape("end-plate-left", "casing", 3.5, rect(-x, G.FIN_Z, x, 0.23)),
            shape("end-plate-right", "casing", 3.5, rect(-x, G.RIGHT, x, -G.FIN_Z)),
        ]
    out = [shape("hairpins", "tubes", 3.1, poly(hairpin_plan()))]
    if s == 50:
        for name, row in (("return", 0), ("supply", 1)):
            out.append(shape(f"tube-ends-{name}", "tubes", 3.1, rect(G.ROW_X[row] - G.TUBE_R, G.SHEET_Z, G.ROW_X[row] + G.TUBE_R, G.HEADER_Z)))
    else:
        out.append(shape("tube-ends", "tubes", 3.1, rect(G.ROW_X[0] - G.TUBE_R, G.SHEET_Z, G.ROW_X[1] + G.TUBE_R, G.HEADER_Z)))
    for name, row in (("outlet-stub", 0), ("inlet-stub", 1)):
        out.append(shape(name, "headers", 3.15, poly(swap(stub_outline(G.ROW_X[row], s, G.HEADER_Z)))))
    out.append(shape("header-return", "headers", 3.3, disc(G.ROW_X[0], G.HEADER_Z, G.HEADER_R)))
    out.append(shape("header-supply", "headers", 3.3, disc(G.ROW_X[1], G.HEADER_Z, G.HEADER_R)))
    if s == 50:
        out.append(shape("vent-plug", "headers", 3.35, disc(G.ROW_X[0], G.HEADER_Z, G.PLUG_R), role="detail", tone=0.3))
    out.append(shape("casing-top", "casing", 3.4, rect(-x, -G.FIN_Z, x, G.FIN_Z)))
    t = G.SHEET
    for name, side in (("left", 1), ("right", -1)):
        z0, z1, zf = side * G.FIN_Z, side * G.SHEET_Z, side * G.FLANGE_Z
        out.append(shape(f"tube-sheet-{name}", "casing", 3.5, poly([[-x, z0], [-x, zf], [-x + t, zf], [-x + t, z1], [x - t, z1], [x - t, zf], [x, zf], [x, z0]]), tone=0.25))
    return out


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


SECTIONS = {"elevation": {"plane": 0, "label": T("A-A", "A-A")}}

HATCH = {
    50: [{"id": "fins", "title": T("Fin edges on the air face", "Lamellenkanten auf der Luftseite"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.4, "penMm": 0.1}],
    100: [{"id": "fins", "title": T("Fin edges on the air face", "Lamellenkanten auf der Luftseite"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.4, "penMm": 0.1}],
    200: [],
}

DECISIONS = {
    50: {
        "silhouette": T("A two-row finned tube coil, 0.52 m high, 0.1 m deep and 0.535 m long over hairpins and stubs, 10.4 mm by 2 mm by 10.7 mm on paper. Section A-A (side) cuts the coil across its tubes at mid length: one fin seen flat, 69 mm by 480 mm, with the 24 cut tubes in two staggered rows, the bottom and the top channel cut as heavy lines and the left tube sheet beyond. Front (air-leaving face): the fin face hatched with its vertical fin edges inside the frame of the two flanges and the channel legs, the twelve hairpins beyond the right flange, the two headers with outlet stub (top) and inlet stub (bottom) beyond the left flange. Plan: the top channel between the tube sheets, the flanges, the U of the hairpins, the tube ends, the two header circles and the two stubs.", "Ein zweireihiges Lamellenrohr-Register, 0,52 m hoch, 0,1 m tief und über Bögen und Stutzen 0,535 m lang, auf Papier 10,4 mm mal 2 mm mal 10,7 mm. Schnitt A-A (Seite) schneidet das Register in halber Länge quer zu den Rohren: eine Lamelle in der Fläche, 69 mm mal 480 mm, mit den 24 geschnittenen Rohren in zwei versetzten Reihen, unteres und oberes Profil geschnitten als kräftige Linien und dahinter der linke Rohrboden. Vorderansicht (Luftaustrittsseite): die Lamellenfläche, schraffiert mit ihren senkrechten Lamellenkanten, im Rahmen der zwei Flansche und der Profilschenkel, die zwölf Haarnadelbögen hinter dem rechten Flansch, die zwei Sammler mit Austrittsstutzen (oben) und Eintrittsstutzen (unten) hinter dem linken Flansch. Grundriss: das obere Profil zwischen den Rohrböden, die Flansche, das U der Bögen, die Rohrenden, die zwei Sammlerkreise und die zwei Stutzen."),
        "retained": T("Fin pack, all 24 tubes, twelve hairpins, both headers, both stubs with their threaded sleeves, drain and vent plug, both tube sheets with flanges, both channels.", "Lamellenpaket, alle 24 Rohre, zwölf Haarnadelbögen, beide Sammler, beide Stutzen mit Gewindemuffe, Entleerungs- und Entlüftungsstopfen, beide Rohrböden mit Flanschen, beide Profile."),
        "merged": T("The 134 fins are one fin outline; nothing else is merged.", "Die 134 Lamellen sind ein Lamellenumriss; sonst ist nichts zusammengeführt."),
        "omitted": T("The fin collars, the louvres of the fins, the screws of the casing and the 9 mm mounting holes of the flanges; in the front view the tubes are hidden behind the fin edges and the vent plug behind the supply header; in the plan the fin pack and the tubes lie under the top channel.", "Die Lamellenkragen, die Schlitze der Lamellen, die Schrauben des Gehäuses und die 9-mm-Befestigungslöcher der Flansche; in der Vorderansicht sind die Rohre hinter den Lamellenkanten und der Entlüftungsstopfen hinter dem Vorlaufsammler verdeckt; im Grundriss liegen Lamellenpaket und Rohre unter dem oberen Profil."),
        "thinElements": T("The 1.5 mm casing sheet is drawn as a line where it is seen on edge: the cut channels in A-A with a 0.25 mm pen, the tube-sheet edges in the plan with the body pen. The 0.2 mm fins at 3 mm pitch are a hatch of 20 mm real pitch. The tubes are their real 15.88 mm (0.32 mm on paper).", "Das 1,5-mm-Gehäuseblech ist dort, wo es auf der Kante gesehen wird, als Linie gezeichnet: die geschnittenen Profile in A-A mit 0,25-mm-Stift, die Rohrbodenkanten im Grundriss mit dem Körperstift. Die 0,2-mm-Lamellen mit 3 mm Teilung sind eine Schraffur mit 20 mm realer Teilung. Die Rohre haben ihre echten 15,88 mm (0,32 mm auf Papier)."),
        "hatching": T("A vertical line hatch of 0.4 mm paper pitch (20 mm real) with a 0.1 mm pen fills the fin face in the front view: the fin edges. The fin seen flat in A-A is not hatched.", "Eine senkrechte Linienschraffur mit 0,4 mm Papierabstand (20 mm real) und 0,1-mm-Stift füllt die Lamellenfläche in der Vorderansicht: die Lamellenkanten. Die in A-A flach gesehene Lamelle ist nicht schraffiert."),
        "styles": T("The light tone carries the aluminium fins, the middle tone the galvanised casing, the dark tone the copper tubes, headers and stubs; body pen 0.18 mm, detail 0.13 mm, cut sheet 0.25 mm, ground 0.35 mm.", "Der helle Ton trägt die Aluminiumlamellen, der mittlere das verzinkte Gehäuse, der dunkle die Kupferrohre, Sammler und Stutzen; Körperstift 0,18 mm, Detail 0,13 mm, geschnittenes Blech 0,25 mm, Boden 0,35 mm."),
        "overlays": T("Fins, tubes, headers and casing are four regions: in the thermal view the tubes and headers warm or cool first, the fins follow and the casing stays near the air temperature. In A-A the left tube sheet lists casing and headers, because the two headers stand hidden right behind it.", "Lamellen, Rohre, Sammler und Gehäuse sind vier Regionen: in der Thermalansicht erwärmen oder kühlen sich Rohre und Sammler zuerst, die Lamellen folgen und das Gehäuse bleibt nahe der Lufttemperatur. In A-A führt der linke Rohrboden Gehäuse und Sammler, weil die zwei Sammler verdeckt unmittelbar dahinter stehen."),
    },
    100: {
        "silhouette": T("A 5.2 mm high coil in the same three views: in A-A the fin with 24 tube dots, the cut channels and the tube sheet beyond; in the front view the hatched fin face in its frame, the hairpins on the right, headers and plain stubs on the left; in the plan the top channel, the flanges, the hairpin U, the header circles and the stubs.", "Ein 5,2 mm hohes Register in denselben drei Ansichten: in A-A die Lamelle mit 24 Rohrpunkten, die geschnittenen Profile und dahinter der Rohrboden; in der Vorderansicht die schraffierte Lamellenfläche in ihrem Rahmen, rechts die Haarnadelbögen, links Sammler und glatte Stutzen; im Grundriss das obere Profil, die Flansche, das U der Bögen, die Sammlerkreise und die Stutzen."),
        "retained": T("Fin pack, all 24 tubes, twelve hairpins, both headers and stubs, tube sheets with flanges, channels.", "Lamellenpaket, alle 24 Rohre, zwölf Haarnadelbögen, beide Sammler und Stutzen, Rohrböden mit Flanschen, Profile."),
        "merged": T("The threaded sleeves are merged into plain stubs and the header caps into flat ends.", "Die Gewindemuffen gehen in glatten Stutzen und die Sammlerkappen in flachen Enden auf."),
        "omitted": T("Mounting holes, drain and vent plug, fin collars.", "Befestigungslöcher, Entleerungs- und Entlüftungsstopfen, Lamellenkragen."),
        "thinElements": T("The 1.5 mm sheet seen on edge is a body-pen line; the fins are a hatch of 40 mm real pitch; the tubes are their real 15.88 mm (0.16 mm on paper).", "Das auf der Kante gesehene 1,5-mm-Blech ist eine Linie im Körperstift; die Lamellen sind eine Schraffur mit 40 mm realer Teilung; die Rohre haben ihre echten 15,88 mm (0,16 mm auf Papier)."),
        "hatching": T("A vertical line hatch of 0.4 mm paper pitch (40 mm real) with a 0.1 mm pen fills the fin face in the front view.", "Eine senkrechte Linienschraffur mit 0,4 mm Papierabstand (40 mm real) und 0,1-mm-Stift füllt die Lamellenfläche in der Vorderansicht."),
        "styles": T("The light tone carries the aluminium fins, the middle tone the galvanised casing, the dark tone the copper parts; body pen 0.18 mm, detail none, ground 0.25 mm.", "Der helle Ton trägt die Aluminiumlamellen, der mittlere das verzinkte Gehäuse, der dunkle die Kupferteile; Körperstift 0,18 mm, Detail entfällt, Boden 0,25 mm."),
        "overlays": T("The same four regions as at 1:50; in A-A the left tube sheet lists casing and headers.", "Dieselben vier Regionen wie bei 1:50; in A-A führt der linke Rohrboden Gehäuse und Sammler."),
    },
    200: {
        "silhouette": T("A 2.6 mm high coil of a few masses: in A-A the coil block between two cut bars in front of the left end plate; in the front view the coil block in a frame of two bars and two end plates, the header block and the two stubs on the left; in the plan the top bar between the end plates, the header block and the stubs.", "Ein 2,6 mm hohes Register aus wenigen Massen: in A-A der Registerblock zwischen zwei geschnittenen Riegeln vor der linken Endplatte; in der Vorderansicht der Registerblock im Rahmen aus zwei Riegeln und zwei Endplatten, links der Sammlerblock und die zwei Stutzen; im Grundriss der obere Riegel zwischen den Endplatten, der Sammlerblock und die Stutzen."),
        "retained": T("Coil block, casing bars, end plates, header block, stubs.", "Registerblock, Gehäuseriegel, Endplatten, Sammlerblock, Stutzen."),
        "merged": T("Fins and the 24 tubes are one block (regions fins and tubes); the hairpins are merged into the right end plate, the two headers into one block; in the plan the top bar also lists fins and tubes, because the pack lies under it.", "Lamellen und die 24 Rohre sind ein Block (Regionen Lamellen und Rohre); die Haarnadelbögen gehen in der rechten Endplatte auf, die zwei Sammler in einem Block; im Grundriss führt der obere Riegel auch Lamellen und Rohre, weil das Paket darunter liegt."),
        "omitted": T("All tube circles, the hatch, flanges as separate parts, plugs and holes.", "Alle Rohrkreise, die Schraffur, Flansche als eigene Teile, Stopfen und Löcher."),
        "thinElements": T("The 20 mm channels are thickened to 30 mm bars, the 28 mm headers and stubs to 30 mm, the 1.5 mm tube sheet with its 25 mm flange to a 30 mm end plate.", "Die 20-mm-Profile sind zu 30-mm-Riegeln verdickt, die 28-mm-Sammler und -Stutzen auf 30 mm, der 1,5-mm-Rohrboden mit seinem 25-mm-Flansch zu einer 30-mm-Endplatte."),
        "hatching": T("None.", "Keine."),
        "styles": T("The light tone carries the coil block, the middle tone the casing, the dark tone the copper header block and stubs; body pen 0.13 mm, detail none, ground 0.25 mm.", "Der helle Ton trägt den Registerblock, der mittlere das Gehäuse, der dunkle Sammlerblock und Stutzen aus Kupfer; Körperstift 0,13 mm, Detail entfällt, Boden 0,25 mm."),
        "overlays": T("The coil block lists fins and tubes as merged regions; headers and casing keep their own shapes, except in A-A, where the left end plate lists casing and headers.", "Der Registerblock führt Lamellen und Rohre als zusammengeführte Regionen; Sammler und Gehäuse behalten eigene Formen, außer in A-A, wo die linke Endplatte Gehäuse und Sammler führt."),
    },
}

RIG_BONES = {
    "frame": {"x": 0, "y": 0, "angleDeg": 90, "lengthM": 0.52, "kg": 3.1, "comX": 0.26, "title": T("Casing: tube sheets and channels (floor to top)", "Gehäuse: Rohrböden und Profile (Boden bis Oberkante)")},
    "fins": {"x": 0.02, "y": 0, "angleDeg": 0, "lengthM": 0.48, "kg": 2.1, "comX": 0.24, "title": T("Aluminium fin pack (bottom to top)", "Aluminium-Lamellenpaket (unten bis oben)")},
    "tubes": {"x": 0.01, "y": 0, "angleDeg": 0, "lengthM": 0.46, "kg": 2.7, "comX": 0.23, "title": T("Copper tubes, hairpins and headers (lowest to highest tube)", "Kupferrohre, Bögen und Sammler (unterstes bis oberstes Rohr)")},
}

DIMENSIONS = {"heightM": 0.52, "widthM": 0.1, "depthM": 0.535, "massKg": 7.9}

STEEL_MATERIAL = {"id": STEEL, "title": T("Galvanised steel", "Verzinkter Stahl"), "emissivity": 0.25, "solarAbsorptivity": 0.5, "densityKgM3": 7800, "conductivityWmK": 50, "specificHeatJKgK": 480, "source": "literature", "reference": "incropera-2018"}

AREAS = {"tubes": 0.56, "headers": 0.1}
LENGTHS = {"fins": 0.07, "tubes": 0.016, "headers": 0.028}
HEAT = {"tubes": 168, "headers": 17}

CASING_REGION = {
    "id": "casing",
    "title": T("Galvanised casing", "Verzinktes Gehäuse"),
    "kind": "surface",
    "model": "passive",
    "material": STEEL,
    "areaM2": 0.54,
    "characteristicLengthM": 0.52,
    "geometry": "vertical-plate",
    "skyViewFactor": 0.5,
    "layers": [{"id": "casing-skin", "title": T("Galvanised sheet", "Verzinktes Blech"), "kind": "metal", "material": STEEL, "thicknessM": 0.0015, "resistanceM2KW": 3e-05, "heatCapacityJM2K": 5600, "source": "authored-estimate", "reference": "incropera-2018"}],
    "heatCapacityJM2K": 5600,
    "source": "authored-estimate",
    "reference": "incropera-2018",
}

SUMMARY = T(
    "A two-row finned tube coil of 0.4 m by 0.48 m face: aluminium plate fins on 24 copper tubes with hairpins on one end and two headers with stubs on the other, in a flanged galvanised casing; cold as evaporator or cooling coil, hot as condenser or heating coil; drawn in section across its tubes.",
    "Ein zweireihiges Lamellenrohr-Register mit 0,4 m mal 0,48 m Anströmfläche: Aluminiumlamellen auf 24 Kupferrohren mit Haarnadelbögen an einem Ende und zwei Sammlern mit Stutzen am anderen, in einem verzinkten Gehäuse mit Flanschen; kalt als Verdampfer oder Kühler, heiß als Verflüssiger oder Erhitzer; im Schnitt quer zu den Rohren gezeichnet.",
)

NOTES = T(
    "A finned tube coil of a small heat pump or air handler in the common P40 geometry: 24 copper tubes of 5/8 in (15.88 mm, EN 12735-2) in two staggered rows of 12 (40 mm tube pitch, 34.64 mm row pitch, an equilateral pattern) through 134 aluminium plate fins of 0.2 mm at 3 mm pitch (VDI 3803-1: 2 to 4 mm), each fin 69.3 mm deep and 480 mm high, over a finned length of 0.4 m. Circuiting: twelve full circuits; beyond the right tube sheet twelve hairpins of 20 mm bend radius join tube k of the air-entering row with tube k of the air-leaving row; beyond the left tube sheet every tube runs into the vertical 28 mm header (EN 1057) of its row, the supply header ahead on the air-leaving row (counterflow) with the inlet stub at the bottom and a drain plug, the return header behind with the outlet stub at the top and a vent plug; the stubs end in R 1 threaded sleeves (EN 10226-1). The casing is 1.5 mm galvanised sheet (EN 10346): two tube sheets with 25 mm flanges, and a bottom and a top channel of 100 mm by 20 mm; the coil stands on the tube-sheet feet and the legs of the bottom channel. The side elevation is section A-A across the tubes at mid length, so one fin appears flat with the 24 tubes as circles. Mass 7.9 kg dry: fins 2.1 kg (134 fins of 0.0284 m2 net), tubes, hairpins and headers 2.7 kg (11 m of 15.88 mm by 0.4 mm tube at 0.173 kg/m, 1.1 m of 28 mm by 1 mm pipe at 0.755 kg/m), casing 3.1 kg. The duty key runs from -0.7 (cold: evaporating refrigerant or chilled water) over 0 to +1 (hot: condensing refrigerant or hot water); the heatW values (fins 340 W, tubes 168 W, headers 17 W) are authored estimates of the heat the surfaces exchange with still room air; tubes and headers keep the flux densities first authored for them (300 and 167 W/m2) on their real outer areas (0.56 and 0.1 m2), so that the tubes are warmer than the fins they feed; the casing is passive. The same asset stands for the evaporator and the condenser of the heat pump and for the heating coil of an air handler. The origin is the centre of the casing footprint on the base line; x points with the air flow, the headers stand on the left (+z).",
    "Ein Lamellenrohr-Register einer kleinen Wärmepumpe oder eines Lüftungsgeräts in der verbreiteten Geometrie P40: 24 Kupferrohre von 5/8 Zoll (15,88 mm, EN 12735-2) in zwei versetzten Reihen zu je 12 (40 mm Rohrteilung, 34,64 mm Reihenteilung, gleichseitiges Muster) durch 134 Aluminiumlamellen von 0,2 mm bei 3 mm Teilung (VDI 3803-1: 2 bis 4 mm), jede Lamelle 69,3 mm tief und 480 mm hoch, über 0,4 m berippte Länge. Schaltung: zwölf volle Kreise; hinter dem rechten Rohrboden verbinden zwölf Haarnadelbögen mit 20 mm Biegeradius Rohr k der Lufteintrittsreihe mit Rohr k der Luftaustrittsreihe; hinter dem linken Rohrboden mündet jedes Rohr in den senkrechten 28-mm-Sammler (EN 1057) seiner Reihe, vorn der Vorlaufsammler an der Luftaustrittsreihe (Gegenstrom) mit dem Eintrittsstutzen unten und einem Entleerungsstopfen, hinten der Rücklaufsammler mit dem Austrittsstutzen oben und einem Entlüftungsstopfen; die Stutzen enden in Gewindemuffen R 1 (EN 10226-1). Das Gehäuse ist 1,5 mm verzinktes Blech (EN 10346): zwei Rohrböden mit 25-mm-Flanschen sowie ein unteres und ein oberes Profil von 100 mm mal 20 mm; das Register steht auf den Füßen der Rohrböden und den Schenkeln des unteren Profils. Die Seitenansicht ist der Schnitt A-A quer zu den Rohren in halber Länge, daher erscheint eine Lamelle in der Fläche mit den 24 Rohren als Kreisen. Masse 7,9 kg trocken: Lamellen 2,1 kg (134 Lamellen zu netto 0,0284 m2), Rohre, Bögen und Sammler 2,7 kg (11 m Rohr 15,88 mm mal 0,4 mm zu 0,173 kg/m, 1,1 m Rohr 28 mm mal 1 mm zu 0,755 kg/m), Gehäuse 3,1 kg. Der Schlüssel duty läuft von -0,7 (kalt: verdampfendes Kältemittel oder Kaltwasser) über 0 bis +1 (heiß: kondensierendes Kältemittel oder Heißwasser); die heatW-Werte (Lamellen 340 W, Rohre 168 W, Sammler 17 W) sind Schätzwerte der Wärme, die die Oberflächen mit ruhender Raumluft austauschen; Rohre und Sammler behalten die ursprünglich für sie angesetzten Wärmestromdichten (300 und 167 W/m2) auf ihren echten Außenflächen (0,56 und 0,1 m2), sodass die Rohre wärmer sind als die Lamellen, die sie speisen; das Gehäuse ist passiv. Dasselbe Asset steht für Verdampfer und Verflüssiger der Wärmepumpe und für den Erhitzer eines Lüftungsgeräts. Der Ursprung ist die Mitte der Gehäuse-Standfläche auf der Grundlinie; x zeigt mit dem Luftstrom, die Sammler stehen links (+z).",
)

REFERENCES = [
    {"id": "en-12735-2-2016", "citation": "DIN EN 12735-2:2016 Copper and copper alloys - Seamless, round tubes for air conditioning and refrigeration - Part 2: Tubes for equipment (5/8 in = 15.88 mm coil tube)."},
    {"id": "en-1057-2010", "citation": "DIN EN 1057:2010 Copper and copper alloys - Seamless, round copper tubes for water and gas in sanitary and heating applications (28 mm by 1 mm header pipe)."},
    {"id": "en-10346-2015", "citation": "DIN EN 10346:2015 Continuously hot-dip coated steel flat products for cold forming - Technical delivery conditions (galvanised casing sheet)."},
    {"id": "en-10226-1-2004", "citation": "DIN EN 10226-1:2004 Pipe threads where pressure tight joints are made on the threads - Part 1: Taper external threads and parallel internal threads (R 1 connection thread, 33.25 mm major diameter)."},
    {"id": "en-1216-1998", "citation": "DIN EN 1216:1998 Heat exchangers - Forced circulation air-cooling and air-heating coils - Test procedures for establishing the performance (terms: finned length, finned height, rows, tube and row pitch)."},
]

STRUCTURE_ASSUMPTION = T(
    "The 7.9 kg coil stands on the feet of its two tube sheets and the legs of the bottom channel: a unilateral contact of 0.1 m by 0.45 m carries the weight; the fins hang on the tubes, the tubes span between the tube sheets.",
    "Das 7,9 kg schwere Register steht auf den Füßen seiner zwei Rohrböden und den Schenkeln des unteren Profils: ein einseitiger Kontakt von 0,1 m mal 0,45 m trägt das Gewicht; die Lamellen hängen an den Rohren, die Rohre spannen zwischen den Rohrböden.",
)


POSE_TEXT = {
    "hot": T(
        "Condensing refrigerant or hot water at about 50 degC warms tubes and fins: in a 21 degC room the tubes reach about 50 degC, the fins about 44 degC and the headers about 42 degC, while the casing stays near the air temperature; the coil is a condenser or heating coil.",
        "Kondensierendes Kältemittel oder Heißwasser von etwa 50 degC erwärmt Rohre und Lamellen: in einem Raum von 21 degC erreichen die Rohre etwa 50 degC, die Lamellen etwa 44 degC und die Sammler etwa 42 degC, während das Gehäuse nahe der Lufttemperatur bleibt; das Register ist ein Verflüssiger oder Erhitzer.",
    ),
    "cold": T(
        "Evaporating refrigerant or chilled water about 20 K below the air cools tubes and fins: in a 21 degC room the tubes sit at about 0 degC, the fins at about 4 degC and the headers at about 6 degC, while the casing stays near the air temperature; the coil is an evaporator or cooling coil.",
        "Verdampfendes Kältemittel oder Kaltwasser etwa 20 K unter der Lufttemperatur kühlt Rohre und Lamellen: in einem Raum von 21 degC liegen die Rohre bei etwa 0 degC, die Lamellen bei etwa 4 degC und die Sammler bei etwa 6 degC, während das Gehäuse nahe der Lufttemperatur bleibt; das Register ist ein Verdampfer oder Kühler.",
    ),
}


def apply_roots(asset):
    """🌱 Writes every root correction into the asset dict in place."""
    asset["summary"] = SUMMARY
    asset["provenance"]["notes"] = NOTES
    have = {r["id"] for r in asset["provenance"]["references"]}
    asset["provenance"]["references"] += [r for r in REFERENCES if r["id"] not in have]
    asset["dimensions"] = DIMENSIONS
    if STEEL not in {m["id"] for m in asset["materials"]}:
        asset["materials"].append(STEEL_MATERIAL)
    for bone in asset["rig"]["bones"]:
        fix = RIG_BONES[bone["id"]]
        bone["x"], bone["y"], bone["angleDeg"], bone["lengthM"], bone["title"] = fix["x"], fix["y"], fix["angleDeg"], fix["lengthM"], fix["title"]
        bone["mass"]["kg"], bone["mass"]["comX"], bone["mass"]["comY"] = fix["kg"], fix["comX"], 0
    for anchor in asset["anchors"]:
        anchor["footprintLengthM"], anchor["footprintWidthM"] = 0.1, 0.45
        anchor["title"] = T("Tube-sheet feet and channel legs on the surface", "Rohrbodenfüße und Profilschenkel auf der Unterlage")
    regions = asset["thermal"]["regions"]
    for region in regions:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
        if region["id"] in HEAT:
            region["operating"]["heatW"] = HEAT[region["id"]]
        if region["id"] in LENGTHS:
            region["characteristicLengthM"] = LENGTHS[region["id"]]
    if "casing" not in {r["id"] for r in regions}:
        regions.append(CASING_REGION)
    for pose in asset["poses"]:
        if pose["id"] in POSE_TEXT:
            pose["description"] = POSE_TEXT[pose["id"]]
        pose["thermal"]["regions"].setdefault("casing", {"sunlitFraction": 0, "exposedFraction": 1})
    asset["structure"]["assumptions"] = [STRUCTURE_ASSUMPTION]
    for s in (50, 100, 200):
        rep = asset["representations"][str(s)]
        rep["hatches"] = HATCH[s]
        rep["geometry"] = {"elevation": elevation(s), "front": front(s), "plan": plan(s)}
        new = {}
        for key, value in rep.items():
            if key == "decisions":
                new["sections"] = SECTIONS
            if key != "sections":
                new[key] = value
        new["decisions"] = DECISIONS[s]
        asset["representations"][str(s)] = new
    rep = asset["representations"]["500"]
    rep["reason"] = T("At 1:500 the coil is only 1 mm high and 0.2 mm deep on paper and its smallest parts, the 0.2 mm fins, the 1.5 mm casing sheet and the 16 mm tubes, are far below every print profile.", "Bei 1:500 ist das Register auf Papier nur 1 mm hoch und 0,2 mm tief, und seine kleinsten Teile, die 0,2-mm-Lamellen, das 1,5-mm-Gehäuseblech und die 16-mm-Rohre, liegen weit unter jedem Druckprofil.")
    rep["limitingFeature"] = T("The 0.2 mm fins, the 1.5 mm casing sheet and the 16 mm tubes", "Die 0,2-mm-Lamellen, das 1,5-mm-Gehäuseblech und die 16-mm-Rohre")
