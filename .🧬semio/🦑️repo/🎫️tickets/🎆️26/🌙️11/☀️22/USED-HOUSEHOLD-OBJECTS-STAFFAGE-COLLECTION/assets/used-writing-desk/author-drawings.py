"""✏️ The hand-drawn 2D representations (elevation from the user's side, front from the pedestal end, plan) of `used-writing-desk` at 1:50, 1:100, 1:200 and 1:500, and the bilingual decisions of the volumes.

All numbers are world rest coordinates of the stored pose, the same numbers the solids of `author-used-writing-desk.py` use.

@see ./author-used-writing-desk.py
"""

T = lambda en, de: {"en": en, "de": de}
API = {}
VEN, TEAK, CHIP, CHROME, BRASS, FELT, LABEL = "teak-veneered-chipboard", "solid-teak", "chipboard-p2", "chrome-steel", "brass", "moving-blanket", "paper-label"
TOPR, CAR, FRO, BLA = "desk-top", "carcass", "drawer-fronts", "blanket"
FRONT_SPANS = {"drawer-bottom": (0.083, 0.4045), "drawer-middle": (0.4045, 0.5905), "drawer-top": (0.5905, 0.735)}
BOX = {"drawer-bottom": (0.095, 0.365), "drawer-middle": (0.420, 0.570), "drawer-top": (0.605, 0.710)}
GRIPS = {"drawer-bottom": (0.376, 0.394), "drawer-top": (0.705, 0.723)}
STAGE = {"50": 4, "100": 3, "200": 2, "500": 1}


def elevation(sc):
    """🖼️ Side elevation seen from the user (from -z): blanket, panel end edge, modesty panel through the knee space, pedestal with its fronts, top edge."""
    E, P, L = API["rectE"], API["polyE"], STAGE[sc]
    side = (0.170, 0.189, 0.551, 0.570) if L >= 3 else (0.170, 0.200, 0.540, 0.570)
    s = [
        E("blanket", "blanket", -0.62, 0.62, 0.0, 0.02, 1, 0.30, BLA, FELT, **({"r": 0.008} if L == 4 else {})),
        E("modesty-panel", "top", -0.545, 0.170, 0.30, 0.735, 2, 0.45, CAR, VEN),
        E("leg-panel-edge", "leg-panel", -0.570, -0.545, 0.02, 0.735, 5, 0.55, CAR, VEN),
    ]
    if L == 1:
        s.append(E("pedestal", "carcass", 0.170, 0.570, 0.02, 0.735, 4, 0.5, CAR, VEN))
        xs = (0.1905, 0.5495)
    else:
        s.append(E("pedestal-side-inner", "carcass", side[0], side[1], 0.02, 0.735, 5, 0.55, CAR, VEN))
        s.append(E("pedestal-side-outer", "carcass", side[2], side[3], 0.02, 0.735, 5, 0.55, CAR, VEN))
        s.append(E("plinth", "carcass", side[1], side[2], 0.02, 0.083, 3, 0.3 if L >= 3 else 0.4, CAR, VEN))
        xs = (side[1], side[2])
    for d, (y0, y1) in FRONT_SPANS.items():
        s.append(E(f"{d}-front", d, xs[0], xs[1], y0, y1, 6, 0.5, FRO, VEN))
    s.append(E("top-edge", "top", -0.60, 0.60, 0.735, 0.76, 9, 0.62, TOPR, VEN))
    if L == 4:
        for d, (g0, g1) in GRIPS.items():
            s.append(E(f"{d}-grip", d, 0.220, 0.520, g0, g1, 8, 0.3, FRO, TEAK))
        s.append(P("drawer-middle-handle", "drawer-middle", [(0.306, 0.560), (0.434, 0.560)], 8, 0.15, FRO, CHROME, penMm=0.2))
        s.append(API["ellE"]("drawer-top-escutcheon", "drawer-top", 0.370, 0.660, 0.010, 0.014, 8, 0.25, FRO, BRASS))
        s.append(P("drawer-top-keyhole", "drawer-top", [(0.370, 0.664), (0.370, 0.652)], 8, 0.1, FRO, BRASS))
        s.append(P("top-chip", "top", [(-0.572, 0.735), (-0.572, 0.751), (-0.578, 0.76)], 10, 0.4, TOPR, VEN))
        s.append(P("scuff-bottom-front", "drawer-bottom", [(0.25, 0.11), (0.29, 0.125)], 7, 0.3, FRO, VEN, role="texture"))
        s.append(P("scuff-bottom-front-2", "drawer-bottom", [(0.41, 0.10), (0.47, 0.115), (0.49, 0.112)], 7, 0.3, FRO, VEN, role="texture"))
        s.append(P("scuff-leg-edge", "leg-panel", [(-0.565, 0.05), (-0.55, 0.09)], 7, 0.3, CAR, VEN, role="texture"))
    if L >= 3:
        s.append(P("shadow-gap-top", "top", [(xs[0], 0.7335), (xs[1], 0.7335)], 10, 0.2, TOPR, VEN))
    return s


def front(sc):
    """🪟️ Front elevation from the pedestal end (from +x): blanket, the outer pedestal side, the end of the top, grips and handle standing proud on the user's side; drawer boxes behind the side for the drawer poses."""
    W, P, L = API["rectW"], API["polyW"], STAGE[sc]
    s = [
        W("blanket", "blanket", -0.32, 0.32, 0.0, 0.02, 1, 0.30, BLA, FELT),
        W("pedestal-side-outer", "carcass", -0.28, 0.28, 0.02, 0.735, 7, 0.55, CAR, VEN),
        W("top-end", "top", -0.30, 0.30, 0.735, 0.76, 9, 0.62, TOPR, VEN),
    ]
    for d, (y0, y1) in FRONT_SPANS.items():
        b0, b1 = BOX[d] if L >= 3 else (y0 + 0.002, y1 - 0.002)
        back = 0.26 if L >= 3 else (0.25 if L == 2 else 0.20)
        if L >= 3:
            s.append(W(f"{d}-box", d, -0.261, back, b0, b1, 4, 0.6, None, CHIP))
            s.append(W(f"{d}-front", d, -0.280, -0.261, y0, y1, 5, 0.5, FRO, VEN))
        else:
            s.append(W(f"{d}-block", d, -0.280, back, y0, y1, 4, 0.5, FRO, VEN))
    if L == 4:
        for d, (g0, g1) in GRIPS.items():
            s.append(W(f"{d}-grip", d, -0.298, -0.280, g0, g1, 8, 0.3, FRO, TEAK))
        s.append(P("drawer-middle-handle", "drawer-middle", [(-0.280, 0.560), (-0.305, 0.560)], 8, 0.15, FRO, CHROME, penMm=0.2))
        s.append(W("price-sticker", "carcass", 0.02, 0.08, 0.50, 0.54, 8, 0.9, CAR, LABEL, role="texture"))
        s.append(P("scuff-side", "carcass", [(-0.20, 0.06), (-0.12, 0.075)], 8, 0.3, CAR, VEN, role="texture"))
        s.append(P("scuff-side-2", "carcass", [(0.05, 0.035), (0.11, 0.05), (0.14, 0.048)], 8, 0.3, CAR, VEN, role="texture"))
    return s


def plan(sc):
    """🗺️ Plan from above: blanket margin, the top with its knocked-off corner, the drawers under it for the drawer poses, traces of use on the top."""
    W, P, L = API["rectW"], API["polyW"], STAGE[sc]
    s = [W("blanket", "blanket", -0.62, 0.62, -0.32, 0.32, 1, 0.30, BLA, FELT, **({"hatch": "quilt"} if L == 4 else {}))]
    for d in FRONT_SPANS:
        if L >= 3:
            s.append(W(f"{d}-box", d, 0.1915, 0.5485, -0.261, 0.26, 3, 0.6, None, CHIP))
            s.append(W(f"{d}-front", d, 0.1905, 0.5495, -0.280, -0.261, 4, 0.5, FRO, VEN))
        else:
            x0, x1 = (0.200, 0.540) if L == 2 else (0.1905, 0.5495)
            s.append(W(f"{d}-block", d, x0, x1, -0.280, 0.25 if L == 2 else 0.20, 3, 0.5, FRO, VEN))
    if L == 4:
        for d in GRIPS:
            s.append(W(f"{d}-grip", d, 0.220, 0.520, -0.298, -0.280, 5, 0.3, FRO, TEAK))
        s.append(P("drawer-middle-handle", "drawer-middle", [(0.306, -0.280), (0.306, -0.300), (0.434, -0.300), (0.434, -0.280)], 6, 0.15, FRO, CHROME, penMm=0.2))
    pts = API["CHIP_XZ"] if L >= 3 else [(-0.60, -0.30), (0.60, -0.30), (0.60, 0.30), (-0.60, 0.30)]
    top = P("top", "top", pts, 9, 0.62, TOPR, VEN, role="body", closed=True, filled=True)
    if L == 4:
        top["hatch"] = "teak-grain"
    s.append(top)
    if L >= 3:
        E = API["ellW"]
        s.append(E("ring-mark", "top", -0.30, 0.08, 0.04, 0.04, 10, 0.45, TOPR, VEN))
        if L == 4:
            s.append(E("ring-mark-2", "top", 0.12, -0.05, 0.035, 0.035, 10, 0.45, TOPR, VEN))
            s.append(P("worn-forearm-zone", "top", [(-0.35, -0.30), (-0.33, -0.235), (-0.20, -0.205), (0.0, -0.20), (0.10, -0.225), (0.12, -0.30)], 10, 0.75, TOPR, VEN, role="texture"))
            for i, pl in enumerate([[(-0.48, -0.12), (-0.36, -0.10)], [(-0.20, 0.02), (-0.05, 0.05)], [(0.02, -0.18), (0.16, -0.21)], [(0.30, 0.12), (0.44, 0.10), (0.47, 0.11)], [(-0.10, 0.20), (0.02, 0.18)]]):
                s.append(P(f"scratch-{i + 1}", "top", pl, 10, 0.4, TOPR, VEN, role="texture"))
            s.append(P("top-chip-edge", "top", [(-0.572, -0.30), (-0.588, -0.291), (-0.60, -0.284)], 10, 0.4, TOPR, VEN))
    return s


PENS = {
    "50": {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": 0.18, "ground": 0.7},
    "100": {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": 0.13, "ground": 0.5},
    "200": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": 0.13, "ground": 0.35},
    "500": {"body": 0.18, "detail": None, "texture": None, "glazing": None, "ground": 0.25},
}
RANGES = {"50": (20, 75), "100": (75, 150), "200": (150, 350), "500": (350, 1000)}
HATCHES = {
    "50": [{"id": "teak-grain", "title": T("Teak veneer grain", "Teakfurniermaserung"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.6, "penMm": 0.13}, {"id": "quilt", "title": T("Quilting of the moving blanket", "Steppung der Möbeldecke"), "pattern": "cross", "angleDeg": 45, "spacingMm": 1.6, "penMm": 0.13}],
    "100": [], "200": [], "500": [],
}

DECISIONS = {
    "50": {
        "silhouette": T("Elevation 24.8 x 15.2 mm from the user's side: the folded blanket, the 25 mm edge of the panel end, the modesty panel seen through the knee space, the pedestal with its 19 mm side edges, the plinth recess and the three fronts (140, 183, 320 mm) with their shadow gaps, the teak grip strips of the bottom and top drawers, the chromed bow handle that replaced the middle grip, the brass escutcheon of the central lock, and the 25 mm top. Front from the pedestal end: the outer side with an old price sticker, the end of the top, the blanket and the grips and handle standing proud on the user's side. Plan: the top with its knocked-off user-side corner over the blanket margin.", "Ansicht 24,8 x 15,2 mm von der Nutzerseite: die gefaltete Decke, die 25-mm-Kante der Wange, die Knieblende durch den Knieraum, der Unterschrank mit seinen 19-mm-Seitenkanten, dem zurückgesetzten Sockel und den drei Fronten (140, 183, 320 mm) mit Schattenfugen, die Teak-Griffleisten des unteren und oberen Schubkastens, der verchromte Bügelgriff anstelle der mittleren Leiste, das Messing-Schlüsselschild des Zentralverschlusses und die 25-mm-Platte. Vorderansicht vom Unterschrankende: die äußere Seite mit einem alten Preisaufkleber, das Plattenende, die Decke und die auf der Nutzerseite vorstehenden Griffe. Grundriss: die Platte mit ihrer abgeschlagenen nutzerseitigen Ecke über dem Deckenrand."),
        "retained": T("Top 1200 x 600 x 25 mm, panel end 25 mm, pedestal 400 mm with 19 mm sides, plinth 60 mm recessed 40 mm, fronts 19 mm, grip strips 300 x 18 x 18 mm, bow handle 128 mm, escutcheon 20 x 28 mm, blanket 1240 x 640 x 20 mm, corner chip 28 x 16 mm.", "Platte 1200 x 600 x 25 mm, Wange 25 mm, Unterschrank 400 mm mit 19-mm-Seiten, Sockel 60 mm, 40 mm zurückgesetzt, Fronten 19 mm, Griffleisten 300 x 18 x 18 mm, Bügelgriff 128 mm, Schlüsselschild 20 x 28 mm, Decke 1240 x 640 x 20 mm, Eckabbruch 28 x 16 mm."),
        "merged": T("The 3 mm shadow gaps between the fronts are the outlines of the touching front shapes; the gap under the top is a detail line.", "Die 3-mm-Schattenfugen zwischen den Fronten sind die Umrisse der aneinanderstoßenden Frontflächen; die Fuge unter der Platte ist eine Detaillinie."),
        "omitted": T("Screws, connector fittings, the lock bolt and key, the felt glides and the drawer runners inside the closed pedestal.", "Schrauben, Verbinderbeschläge, Riegel und Schlüssel, Filzgleiter und die Laufleisten im geschlossenen Unterschrank."),
        "thinElements": T("The 10 mm rod of the bow handle and the keyhole are 0.2 and 0.25 mm strokes; the panel and side edges are filled strips with the 0.5 mm outline.", "Der 10-mm-Stab des Bügelgriffs und das Schlüsselloch sind Striche von 0,2 und 0,25 mm; Wangen- und Seitenkanten sind gefüllte Streifen mit dem 0,5-mm-Umriss."),
        "hatching": T("Teak grain lines on the top and the quilting of the blanket in plan.", "Teakmaserung auf der Platte und Steppung der Decke im Grundriss."),
        "styles": T("Top 0.62, panels 0.55, fronts 0.5, recessed plinth 0.3, grips 0.3, blanket 0.3, worn forearm zone 0.75.", "Platte 0,62, Wangen 0,55, Fronten 0,5, zurückgesetzter Sockel 0,3, Griffe 0,3, Decke 0,3, Umriss der abgenutzten Zone 0,13-mm-Textur."),
        "overlays": T("Traces of use drawn as texture: five scratches, two ring marks and the outline of the worn, sun-bleached zone where the forearms rest, scuffs on the bottom front, the panel edge and the pedestal side, the price sticker.", "Gebrauchsspuren als Textur: fünf Kratzer, zwei Glasränder und der Umriss der abgenutzten, sonnengebleichten Zone der Unterarme, Schrammen auf der unteren Front, der Wangenkante und der Unterschrankseite, der Preisaufkleber."),
    },
    "100": {
        "silhouette": T("Elevation 12.4 x 7.6 mm: blanket, panel edge, modesty panel, pedestal with side edges, plinth recess and the three fronts, top. Front: the outer pedestal side with the end of the top over the blanket. Plan: the top with its knocked-off corner.", "Ansicht 12,4 x 7,6 mm: Decke, Wangenkante, Knieblende, Unterschrank mit Seitenkanten, Sockelrücksprung und den drei Fronten, Platte. Vorderansicht: die äußere Unterschrankseite mit dem Plattenende über der Decke. Grundriss: die Platte mit abgeschlagener Ecke."),
        "retained": T("Top 25 mm, panel end 25 mm, sides 19 mm, plinth recess 60 x 40 mm, fronts 140, 183 and 320 mm, corner chip, one ring mark.", "Platte 25 mm, Wange 25 mm, Seiten 19 mm, Sockelrücksprung 60 x 40 mm, Fronten 140, 183 und 320 mm, Eckabbruch, ein Glasrand."),
        "merged": T("Grips, handle and escutcheon are not drawn (18 mm and less, under 0.2 mm at 1:150).", "Griffe, Bügel und Schlüsselschild entfallen (18 mm und weniger, unter 0,2 mm bei 1:150)."),
        "omitted": T("Scratches, scuffs, sticker, worn zone and the blanket quilting.", "Kratzer, Schrammen, Aufkleber, abgenutzte Zone und die Steppung der Decke."),
        "thinElements": T("The 25 mm panel edge and top and the 19 mm sides are filled strips carried by the 0.35 mm outline.", "Die 25-mm-Wangenkante und -Platte und die 19-mm-Seiten sind gefüllte Streifen, getragen vom 0,35-mm-Umriss."),
        "hatching": T("None.", "Keine."),
        "styles": T("As at 1:50.", "Wie bei 1:50."),
        "overlays": T("One ring mark remains on the top as texture.", "Ein Glasrand bleibt als Textur auf der Platte."),
    },
    "200": {
        "silhouette": T("Elevation 6.2 x 3.8 mm: the blanket band, panel edge, modesty panel, the pedestal with generalised 30 mm side edges, a flush plinth and three fronts, the top.", "Ansicht 6,2 x 3,8 mm: das Deckenband, die Wangenkante, die Knieblende, der Unterschrank mit verallgemeinerten 30-mm-Seitenkanten, bündigem Sockel und drei Fronten, die Platte."),
        "retained": T("Top 25 mm, panel end 25 mm, pedestal 400 mm, the three fronts, knee space 0.715 x 0.70 m.", "Platte 25 mm, Wange 25 mm, Unterschrank 400 mm, die drei Fronten, Knieraum 0,715 x 0,70 m."),
        "merged": T("The plinth recess (40 mm, under 0.15 mm gap at 1:350) is drawn flush; the sides are widened to 30 mm; each drawer is one block.", "Der Sockelrücksprung (40 mm, unter 0,15 mm Lücke bei 1:350) ist bündig gezeichnet; die Seiten sind auf 30 mm verbreitert; jeder Schubkasten ist ein Block."),
        "omitted": T("Corner chip, grips, handle, escutcheon and every trace of use.", "Eckabbruch, Griffe, Bügel, Schlüsselschild und alle Gebrauchsspuren."),
        "thinElements": T("Top, panel and blanket keep their true 25 and 20 mm, carried by the 0.25 mm outline.", "Platte, Wange und Decke behalten ihre wahren 25 und 20 mm, getragen vom 0,25-mm-Umriss."),
        "hatching": T("None.", "Keine."),
        "styles": T("Top 0.62, panels 0.55, fronts 0.5, plinth 0.4, blanket 0.3.", "Platte 0,62, Wangen 0,55, Fronten 0,5, Sockel 0,4, Decke 0,3."),
        "overlays": T("None.", "Keine."),
    },
    "500": {
        "silhouette": T("Elevation 2.5 x 1.5 mm: the blanket line, the top over the knee space between the panel edge and the pedestal block with its three fronts.", "Ansicht 2,5 x 1,5 mm: die Deckenlinie, die Platte über dem Knieraum zwischen Wangenkante und Unterschrankblock mit seinen drei Fronten."),
        "retained": T("Overall 1.24 x 0.76 m, knee space, pedestal 0.40 m, the three fronts.", "Gesamt 1,24 x 0,76 m, Knieraum, Unterschrank 0,40 m, die drei Fronten."),
        "merged": T("The pedestal is one block; the three drawers are blocks that slide out of it.", "Der Unterschrank ist ein Block; die drei Schubkästen sind Blöcke, die aus ihm herausgleiten."),
        "omitted": T("Plinth, shadow gaps, grips, handle, chip and traces of use.", "Sockel, Schattenfugen, Griffe, Bügel, Abbruch und Gebrauchsspuren."),
        "thinElements": T("Top, panel edge, modesty panel and blanket are drawn at their true 20-25 mm and carried by the 0.18 mm outline.", "Platte, Wangenkante, Knieblende und Decke sind mit ihren wahren 20-25 mm gezeichnet und vom 0,18-mm-Umriss getragen."),
        "hatching": T("None.", "Keine."),
        "styles": T("Top 0.62, pedestal 0.5, fronts 0.5, panels 0.55, blanket 0.3.", "Platte 0,62, Unterschrank 0,5, Fronten 0,5, Wangen 0,55, Decke 0,3."),
        "overlays": T("None.", "Keine."),
    },
}

VOLUME_DECISIONS = {
    "50": [
        T("Body, authored for 7.5 mm: the folded felt blanket 1240 x 640 x 20 mm with 8 mm rounded folds; the 25 mm top as an extrusion of its plan with the knocked-off user-side corner (28 x 16 mm); the 25 mm panel end with its 12 mm foot recess between two 80 mm feet; the pedestal of two 19 mm sides, an 8 mm back, a 16 mm bottom and a plinth recessed 40 mm; the 16 mm modesty panel; per drawer a 19 mm front, two 12 mm sides on 12 x 12 mm runners, a 12 mm back and a declared 4 mm hardboard bottom sheet; the solid teak grip strips on the bottom and top drawers, the chromed bow handle (two posts and a bar of 10 mm rod) on the middle one, the brass escutcheon and the price sticker as texture.", "Körper, für 7,5 mm gebaut: die gefaltete Filzdecke 1240 x 640 x 20 mm mit 8 mm gerundeten Falten; die 25-mm-Platte als Extrusion ihres Grundrisses mit abgeschlagener nutzerseitiger Ecke (28 x 16 mm); die 25-mm-Wange mit 12-mm-Fußausschnitt zwischen zwei 80-mm-Füßen; der Unterschrank aus zwei 19-mm-Seiten, 8-mm-Rückwand, 16-mm-Boden und einem 40 mm zurückgesetzten Sockel; die 16-mm-Knieblende; je Schubkasten eine 19-mm-Front, zwei 12-mm-Seiten auf 12 x 12 mm Laufleisten, eine 12-mm-Rückwand und ein als Blatt deklarierter 4-mm-Hartfaserboden; die Teak-Griffleisten am unteren und oberen Schubkasten, der verchromte Bügelgriff (zwei Stützen und ein Bügel aus 10-mm-Stab) am mittleren, Schlüsselschild und Preisaufkleber als Textur."),
        T("Thermal: references for the top, the panels and the blanket; the drawer fronts are own solids that close the 3 mm shadow gaps, so the pedestal is a closed cavity and the boxes inside are no thermal surface. Structure: the panels and boards are the members (they are the load path of a panel desk), runners and connector blocks are joints, one support per anchor, one mass marker per bone.", "Wärme: Verweise für Platte, Wangen und Decke; die Schubkastenfronten sind eigene Körper, die die 3-mm-Schattenfugen schließen, so dass der Unterschrank ein geschlossener Hohlraum ist und die Kästen darin keine Wärmefläche sind. Tragwerk: Platten und Wangen sind die Stäbe (sie sind der Lastweg eines Wangentischs), Laufleisten und Verbinderklötze sind Gelenke, je Anker ein Lager, je Knochen eine Massenmarke."),
    ],
    "100": [
        T("Body, authored for 15 mm: blanket, top with its chip (16 mm notch), the panel end without the 12 mm foot recess, the two pedestal sides as one repeated solid, a back thickened to 15 mm, bottom, plinth and modesty panel; each drawer a 19 mm front on one solid box that touches the sides within 1.5 mm, so the runners are not needed. Grips, bow handle, escutcheon and sticker (18 mm and less) are left out as in the 2D.", "Körper, für 15 mm gebaut: Decke, Platte mit Abbruch (16-mm-Kerbe), die Wange ohne den 12-mm-Fußausschnitt, die zwei Unterschrankseiten als ein wiederholter Körper, eine auf 15 mm verstärkte Rückwand, Boden, Sockel und Knieblende; je Schubkasten eine 19-mm-Front auf einem massiven Kasten, der die Seiten auf 1,5 mm berührt, daher entfallen die Laufleisten. Griffe, Bügel, Schlüsselschild und Aufkleber (18 mm und weniger) entfallen wie im 2D."),
        T("Thermal and structure as at 1:50 on these solids; the runner joints are small blocks at the side of each drawer box.", "Wärme und Tragwerk wie bei 1:50 auf diesen Körpern; die Laufgelenke sind kleine Klötze an der Seite jedes Kastens."),
    ],
    "200": [
        T("Body, authored for 30 mm: blanket, square top (no chip), panel end, pedestal shell with the sides generalised to 30 mm, a 30 mm back and a flush plinth block, a 30 mm modesty panel, and each drawer as one block between the sides. The 25 mm top and panel and the 20 mm blanket keep their true thickness because the 2D draws them so.", "Körper, für 30 mm gebaut: Decke, rechteckige Platte (ohne Abbruch), Wange, Unterschrankschale mit auf 30 mm verallgemeinerten Seiten, 30-mm-Rückwand und bündigem Sockelblock, eine 30-mm-Knieblende und jeder Schubkasten als ein Block zwischen den Seiten. Die 25-mm-Platte und -Wange und die 20-mm-Decke behalten ihre wahre Stärke, weil das 2D sie so zeichnet."),
        T("Thermal: the drawer fronts are 30 mm front slabs on the drawer bones; structure members on the blocks and panels.", "Wärme: die Schubkastenfronten sind 30-mm-Frontplatten an den Schubkastenknochen; Tragwerksstäbe auf Blöcken und Wangen."),
    ],
    "500": [
        T("Body, authored for 75 mm: blanket, top, panel end, modesty panel, the pedestal as one block and the three drawers as blocks lying in it, so they slide out of the block in the drawer poses; the knee space (0.715 x 0.70 m) is the decisive feature of the silhouette.", "Körper, für 75 mm gebaut: Decke, Platte, Wange, Knieblende, der Unterschrank als ein Block und die drei Schubkästen als darin liegende Blöcke, die in den Schubkastenposen aus dem Block herausgleiten; der Knieraum (0,715 x 0,70 m) ist das entscheidende Merkmal der Silhouette."),
        T("Thermal: 75 mm front slabs on the drawer bones inside the drawer blocks; structure members on the blocks and panels.", "Wärme: 75-mm-Frontplatten an den Schubkastenknochen in den Schubkastenblöcken; Tragwerksstäbe auf Blöcken und Wangen."),
    ],
}


def representations():
    """🗂️ The four authored representations."""
    out = {}
    for sc in ("500", "200", "100", "50"):
        a, b = RANGES[sc]
        out[sc] = {"status": "authored", "denominator": int(sc), "validRange": {"minDenominator": a, "maxDenominator": b}, "pens": PENS[sc], "hatches": HATCHES[sc], "geometry": {"elevation": elevation(sc), "front": front(sc), "plan": plan(sc)}, "decisions": DECISIONS[sc]}
    return out
