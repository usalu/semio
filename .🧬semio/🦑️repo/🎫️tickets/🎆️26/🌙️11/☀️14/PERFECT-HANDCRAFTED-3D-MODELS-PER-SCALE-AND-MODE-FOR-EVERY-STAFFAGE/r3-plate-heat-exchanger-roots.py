"""🌱 Round 3, machinery/plate-heat-exchanger: the root corrections outside `volumes` (rig, anchors, emitter titles, texts) and the 2D drawings of 1:50, 1:100 and 1:200 (elevation section A-A, front section B-B, plan), drawn by hand from the measures of `r3-plate-heat-exchanger-geo.py`.

Usage: imported by `r3-plate-heat-exchanger-build.py` (`apply_roots(asset)`).
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("phx_geo", os.path.join(HERE, "r3-plate-heat-exchanger-geo.py"))
G = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(G)

STEEL = "galvanised-steel"
ALU = "aluminium-plate"


def shape(id, bone, role, z, tone, region, material, geom, filled=True, **kw):
    """✏️ One 2D shape with the common members in the catalog's order."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    out["region"] = region
    out["material"] = material
    out.update(kw)
    return out


def poly(points, closed=True):
    """📐 Poly geometry."""
    return {"kind": "poly", "points": [[round(x, 6), round(y, 6)] for x, y in points], "closed": closed}


def rect(x, y, w, h):
    """▭ Rect geometry."""
    return {"kind": "rect", "x": round(x, 6), "y": round(y, 6), "w": round(w, 6), "h": round(h, 6)}


def el(bone, pts):
    """↘ World rest points into the bone-local elevation frame."""
    return [G.loc(bone, x, y) for x, y in pts]


def spigot_outline(side, cy, s):
    """🕳️ Elevation and plan outline of a spigot seen side-on (world x, second coordinate about cy): tube and, at 1:50, its flange."""
    h, o, r = G.HALF, G.HALF + G.SPIGOT_OUT, G.SPIGOT_R
    if s == 50:
        f, fr = 0.0075, G.FLANGE_R
        pts = [[o, cy - r], [h + f, cy - r], [h + f, cy - fr], [h, cy - fr], [h, cy + fr], [h + f, cy + fr], [h + f, cy + r], [o, cy + r]]
    else:
        pts = [[o, cy - r], [h, cy - r], [h, cy + r], [o, cy + r]]
    pts = [[side * x, y] for x, y in pts]
    return pts if side > 0 else pts[::-1]


def elevation(s):
    """🖼️ Elevation drawn as section A-A on the lateral mid plane (z = 0), seen from the right: the back panel behind the four chambers, the diamond core, its seals, the spigots cut through their axes, base rails, flap."""
    t = G.walls(s)
    a, b = G.core(s)
    out = [shape("frame", "frame", "body", 3.3, 0.35, "frame", STEEL, poly(el("frame", G.rect(-G.HALF, G.BASE, G.HALF, G.TOP))))]
    for name, x in (("rear", -G.RAIL_X), ("front", G.RAIL_X)):
        out.append(shape(f"base-rail-{name}", "frame", "body", 3.25, 0.25, "frame", STEEL, poly(el("frame", G.rect(x - G.RAIL_W / 2, 0.0, x + G.RAIL_W / 2, G.RAIL_H)))))
    if s == 50:
        out.append(shape("condensate-drain", "frame", "detail", 3.25, 0.3, "frame", STEEL, poly(el("frame", G.rect(-0.131, 0.0075, -0.109, G.BASE)))))
    out.append(shape("plate-pack", "plates", "body", 3.4, 0.7, "plates", ALU, poly(el("plates", G.diamond(a, b)))))
    if s != 200:
        out.append(shape("core-cradle", "frame", "body", 3.45, 0.3, "frame", STEEL, poly(el("frame", G.cradle(s)))))
        out.append(shape("core-top-rail", "frame", "body", 3.45, 0.3, "frame", STEEL, poly(el("frame", G.top_rail(s)))))
    if s == 50:
        out.append(shape("seal-rail-rear", "frame", "body", 3.45, 0.3, "frame", STEEL, poly(el("frame", G.side_rail(-1)))))
        out.append(shape("seal-rail-front", "frame", "body", 3.45, 0.3, "frame", STEEL, poly(el("frame", G.side_rail(1)))))
    for id, side, up in (("collar-upper-left", -1, 1), ("collar-lower-left", -1, 0), ("collar-upper-right", 1, 1), ("collar-lower-right", 1, 0)):
        out.append(shape(id, "ports", "body", 3.2, 0.4, "ports", STEEL, poly(el("ports", spigot_outline(side, G.SPIGOT_Y[up], s)))))
    out += damper_elevation(s)
    return out


def damper_profile(s):
    """🚪 The flap profile in its bone frame, as the 3D body has it."""
    return G.damper_profile(s)


def damper_elevation(s):
    """🚪 Elevation shapes of the flap and (1:50) its hinge barrel."""
    out = [shape("damper", "damper", "body", 3.5, 0.3, "damper", STEEL, poly(damper_profile(s)))]
    if s == 50:
        out.append(shape("damper-hinge", "damper", "detail", 3.6, 0.15, "damper", STEEL, {"kind": "ellipse", "cx": 0, "cy": 0, "rx": G.BARREL_R, "ry": G.BARREL_R}))
    return out


def damper_solid(s, z):
    """🚪 Front and plan shape of the flap: its elevation profile extruded across the flap width (the product generates the view)."""
    xs = [p[0] for p in damper_profile(s)]
    ys = [p[1] for p in damper_profile(s)]
    return shape("damper", "damper", "body", z, 0.3, "damper", STEEL, rect(min(xs), min(ys), max(xs) - min(xs), max(ys) - min(ys)), solid={"kind": "extruded", "sweep": "w", "z0": -G.FLAP_HALF_Z, "z1": G.FLAP_HALF_Z})


def front(s):
    """🖼️ Front drawn as section B-B on the forward mid plane (x = 0), seen from ahead: the casing with the rear end wall beyond, the plate pack cut across its plates (hatched), cradle and top rail cut, the rear base rail and the drain beyond, flap and hinge."""
    t = G.walls(s)
    a, b = G.core(s)
    inner = G.HALF - t
    out = [
        shape("frame", "frame", "body", 3.3, 0.35, "frame", STEEL, rect(-G.HALF, G.BASE, 2 * G.HALF, G.TOP - G.BASE)),
        shape("base-rail-rear", "frame", "body", 3.25, 0.25, "frame", STEEL, rect(-G.HALF, 0.0, 2 * G.HALF, G.RAIL_H)),
    ]
    if s == 50:
        out.append(shape("condensate-drain", "frame", "detail", 3.25, 0.3, "frame", STEEL, rect(-0.011, 0.0075, 0.022, G.BASE - 0.0075)))
    if s == 50:
        lo, hi = G.CORE_Y - b - 0.015, G.CORE_Y + b + 0.015
    else:
        lo, hi = G.CORE_Y - b, G.CORE_Y + b
    pack = shape("plate-pack", "plates", "body", 3.4, 0.7, "plates", ALU, rect(-inner, lo, 2 * inner, hi - lo))
    if s != 200:
        pack["hatch"] = "plates"
    out.append(pack)
    if s != 200:
        cr = G.cradle(s)[3][1]
        tr = G.top_rail(s)[2][1]
        out.append(shape("core-cradle", "frame", "body", 3.45, 0.3, "frame", STEEL, rect(-inner, G.BASE + t, 2 * inner, cr - G.BASE - t)))
        out.append(shape("core-top-rail", "frame", "body", 3.45, 0.3, "frame", STEEL, rect(-inner, tr, 2 * inner, G.TOP - t - tr)))
    out.append(damper_solid(s, 3.6))
    if s == 50:
        out.append(shape("damper-hinge", "damper", "detail", 3.55, 0.15, "damper", STEEL, rect(-G.FLAP_HALF_Z, G.RIG["damper"][1] - G.BARREL_R, 2 * G.FLAP_HALF_Z, 2 * G.BARREL_R)))
    return out


def plan(s):
    """🖼️ Plan seen from above: the top panel with the bypass opening under the flap, the spigots of both end walls (upper and lower spigots coincide), flap and hinge."""
    o = G.OPENING
    out = [
        shape("frame", "frame", "body", 3.3, 0.35, "frame", STEEL, rect(-G.HALF, -G.HALF, 2 * G.HALF, 2 * G.HALF)),
        shape("bypass-opening", "frame", "body", 3.35, 0.15, "frame", STEEL, rect(-o, -o, 2 * o, 2 * o)),
        shape("collar-left", "ports", "body", 3.2, 0.4, "ports", STEEL, poly(spigot_outline(-1, 0.0, s))),
        shape("collar-right", "ports", "body", 3.2, 0.4, "ports", STEEL, poly(spigot_outline(1, 0.0, s))),
    ]
    if s == 50:
        out.append(shape("damper-hinge", "damper", "detail", 3.55, 0.15, "damper", STEEL, rect(G.RIG["damper"][0] - G.BARREL_R, -G.FLAP_HALF_Z, 2 * G.BARREL_R, 2 * G.FLAP_HALF_Z)))
    out.append(damper_solid(s, 3.6))
    return out


def T(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


SECTIONS = {"elevation": {"plane": 0, "label": T("A-A", "A-A")}, "front": {"plane": 0, "label": T("B-B", "B-B")}}

HATCH = {
    50: [{"id": "plates", "title": T("Plate pack cut across its plates", "Plattenpaket quer zu den Platten geschnitten"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.5, "penMm": 0.1}],
    100: [{"id": "plates", "title": T("Plate pack cut across its plates", "Plattenpaket quer zu den Platten geschnitten"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.4, "penMm": 0.1}],
    200: [],
}

DECISIONS = {
    50: {
        "silhouette": T("A 0.5 m by 0.6 m cross-flow heat-recovery section, 12.4 mm by 12.4 mm on paper, drawn in two sections. Section A-A (side) cuts the casing on its lateral mid plane: the back panel shows behind the four chambers, the diamond plate core stands on its lower corner in a V cradle, held by a top rail and two side seal rails, the four DN 200 spigots are cut through their axes with their flanges, the base rails and the condensate drain stand under the casing and the bypass flap lies on the lid on its hinge. Section B-B (front) cuts across the plates, so the pack shows as a vertical hatch between cradle and top rail inside the casing, with the rear rail and the drain beyond. The plan shows the lid, the flap over the bypass opening, the hinge and the spigots of both end walls.", "Ein 0,5 m mal 0,6 m großer Kreuzstrom-Wärmerückgewinnungsteil, auf Papier 12,4 mm mal 12,4 mm, in zwei Schnitten gezeichnet. Schnitt A-A (Seite) schneidet das Gehäuse in seiner seitlichen Mittelebene: die Rückwand erscheint hinter den vier Kammern, der rautenförmige Plattenkern steht auf seiner unteren Ecke in einer V-Wiege, gehalten von einer Deckschiene und zwei seitlichen Dichtschienen, die vier DN-200-Stutzen sind in ihrer Achse mit ihren Flanschen geschnitten, Fußschienen und Kondensatablauf stehen unter dem Gehäuse und die Bypass-Klappe liegt an ihrem Scharnier auf dem Deckel. Schnitt B-B (vorn) schneidet quer durch die Platten, das Paket erscheint daher als senkrechte Schraffur zwischen Wiege und Deckschiene im Gehäuse, dahinter die hintere Schiene und der Ablauf. Die Draufsicht zeigt den Deckel, die Klappe über der Bypassöffnung, das Scharnier und die Stutzen beider Stirnwände."),
        "retained": T("Casing, base rails, condensate drain, plate core with cradle, top rail and side seal rails, four spigots with flanges, flap with hinge barrel, bypass opening.", "Gehäuse, Fußschienen, Kondensatablauf, Plattenkern mit Wiege, Deckschiene und seitlichen Dichtschienen, vier Stutzen mit Flansch, Klappe mit Scharnierrohr, Bypassöffnung."),
        "merged": T("The core end plates and corner profiles are merged into the core outline and its seal rails; the 1 mm casing sheet is drawn as the casing outline.", "Endplatten und Eckprofile des Kerns gehen in Kernumriss und Dichtschienen auf; das 1-mm-Gehäuseblech ist als Gehäuseumriss gezeichnet."),
        "omitted": T("The embossing of the plates, the cover fasteners (3.5 mm proud) and the hinge leaf.", "Die Prägung der Platten, die Deckelverschlüsse (3,5 mm vorstehend) und der Scharnierlappen."),
        "thinElements": T("The 0.3 mm plates at 2.5 mm pitch are drawn as a vertical hatch of 25 mm real pitch in section B-B; the 1 mm spigot wall is drawn as the spigot outline; the flap blade is 7.5 mm.", "Die 0,3 mm dicken Platten mit 2,5 mm Teilung sind in Schnitt B-B als senkrechte Schraffur mit 25 mm realer Teilung gezeichnet; die 1-mm-Stutzenwand ist als Stutzenumriss gezeichnet; das Klappenblatt hat 7,5 mm."),
        "hatching": T("A vertical line hatch of 0.5 mm paper pitch (25 mm real) with a 0.1 mm pen fills the plate pack cut across its plates; nothing else is hatched.", "Eine senkrechte Linienschraffur mit 0,5 mm Papierabstand (25 mm real) und 0,1-mm-Stift füllt das quer zu den Platten geschnittene Plattenpaket; sonst ist nichts schraffiert."),
        "styles": T("Darker tones carry the steel parts, the light tone the aluminium core; the bypass opening is dark; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Dunklere Töne tragen die Stahlteile, der helle Ton den Aluminiumkern; die Bypassöffnung ist dunkel; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."),
        "overlays": T("Casing, plates, spigots and flap are four passive regions without a heat source of their own; the supply and exhaust streams leave the lower spigots as vent emitters that scale with the exchange level.", "Gehäuse, Platten, Stutzen und Klappe sind vier passive Regionen ohne eigene Wärmequelle; Zuluft- und Fortluftstrom verlassen die unteren Stutzen als Lüftungsemitter, die mit der Austauschstufe skalieren."),
    },
    100: {
        "silhouette": T("A 6.2 mm by 6.2 mm exchanger section in the same two sections: in A-A the casing, the diamond core between cradle and top rail with its side corners at the end walls, four plain spigots, rails and flap; in B-B the hatched plate pack between cradle and top rail; in plan the lid, the flap over the opening and the spigots.", "Ein 6,2 mm mal 6,2 mm großer Übertragerteil in denselben zwei Schnitten: in A-A das Gehäuse, der Rautenkern zwischen Wiege und Deckschiene mit den Seitenecken an den Stirnwänden, vier glatte Stutzen, Schienen und Klappe; in B-B das schraffierte Plattenpaket zwischen Wiege und Deckschiene; in der Draufsicht der Deckel, die Klappe über der Öffnung und die Stutzen."),
        "retained": T("Casing, base rails, core with cradle and top rail, spigots, flap, bypass opening.", "Gehäuse, Fußschienen, Kern mit Wiege und Deckschiene, Stutzen, Klappe, Bypassöffnung."),
        "merged": T("The side seal rails are merged into the core, which reaches the end walls; flange and hinge barrel are merged into spigot and flap.", "Die seitlichen Dichtschienen gehen im Kern auf, der die Stirnwände erreicht; Flansch und Scharnierrohr gehen in Stutzen und Klappe auf."),
        "omitted": T("The condensate drain, the fasteners, the hinge leaf and the embossing.", "Kondensatablauf, Verschlüsse, Scharnierlappen und Prägung."),
        "thinElements": T("The plates are a vertical hatch of 40 mm real pitch; the flap with its hinge is 15 mm.", "Die Platten sind eine senkrechte Schraffur mit 40 mm realer Teilung; die Klappe mit Scharnier hat 15 mm."),
        "hatching": T("A vertical line hatch of 0.4 mm paper pitch (40 mm real) with a 0.1 mm pen fills the plate pack in B-B.", "Eine senkrechte Linienschraffur mit 0,4 mm Papierabstand (40 mm real) und 0,1-mm-Stift füllt das Plattenpaket in B-B."),
        "styles": T("Darker tones carry the steel parts, the light tone the aluminium core; body pen 0.18 mm, detail none, ground 0.25 mm.", "Dunklere Töne tragen die Stahlteile, der helle Ton den Aluminiumkern; Körperstift 0,18 mm, Detail entfällt, Boden 0,25 mm."),
        "overlays": T("Same four regions and two emitters as at 1:50.", "Dieselben vier Regionen und zwei Emitter wie bei 1:50."),
    },
    200: {
        "silhouette": T("A 3.1 mm by 3.1 mm exchanger of a few masses in the same two sections: the casing, the core as a rhombus from floor to lid and wall to wall, four spigots, two rails and the flap; in B-B the core as a plain block, in plan lid, opening, flap and spigots.", "Ein 3,1 mm mal 3,1 mm großer Übertrager aus wenigen Massen in denselben zwei Schnitten: das Gehäuse, der Kern als Raute von Boden bis Deckel und Wand bis Wand, vier Stutzen, zwei Schienen und die Klappe; in B-B der Kern als einfacher Block, in der Draufsicht Deckel, Öffnung, Klappe und Stutzen."),
        "retained": T("Casing, base rails, core, spigots, flap, bypass opening.", "Gehäuse, Fußschienen, Kern, Stutzen, Klappe, Bypassöffnung."),
        "merged": T("Cradle, top rail and side seals are merged into the core rhombus; the hinge into the flap.", "Wiege, Deckschiene und Seitendichtungen gehen in der Kernraute auf; das Scharnier in der Klappe."),
        "omitted": T("Hatch, drain, fasteners, flanges and embossing.", "Schraffur, Ablauf, Verschlüsse, Flansche und Prägung."),
        "thinElements": T("The flap with its hinge is 20 mm (0.1 mm on paper).", "Die Klappe mit Scharnier hat 20 mm (0,1 mm auf Papier)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Darker tones carry the steel parts, the light tone the aluminium core; body pen 0.13 mm, detail none, ground 0.25 mm.", "Dunklere Töne tragen die Stahlteile, der helle Ton den Aluminiumkern; Körperstift 0,13 mm, Detail entfällt, Boden 0,25 mm."),
        "overlays": T("Same four regions and two emitters as at 1:50.", "Dieselben vier Regionen und zwei Emitter wie bei 1:50."),
    },
}

RIG_BONES = {
    "frame": {"x": -0.25, "y": 0.0, "lengthM": 0.5, "mass": {"comX": 0.25, "comY": 0.3}},
    "plates": {"x": 0.025, "y": 0.315, "lengthM": 0.45, "mass": {"comX": 0.225, "comY": 0.0}},
    "ports": {"x": -0.06, "y": 0.315, "lengthM": 0.62, "mass": {"comX": 0.31, "comY": 0.0}},
    "damper": {"x": 0.05, "y": 0.61, "lengthM": 0.4, "mass": {"comX": 0.2, "comY": -0.006}},
}

BONE_TITLES = {
    "frame": T("Casing on its base rails (base edge, behind to ahead)", "Gehäuse auf seinen Fußschienen (Grundkante, hinten nach vorn)"),
    "plates": T("Cross-flow plate core (horizontal diagonal, corner to corner)", "Kreuzstrom-Plattenkern (waagrechte Diagonale, Ecke zu Ecke)"),
    "ports": T("Four DN 200 spigots (mouth to mouth)", "Vier DN-200-Stutzen (Mündung zu Mündung)"),
}

ANCHORS = {
    "base-contact": {"x": 0.25, "y": 0.0},
    "supply-collar": {"x": 0.62, "y": -0.155, "title": T("Supply air leaving the lower-right spigot", "Zuluft am rechten unteren Stutzen")},
    "exhaust-collar": {"x": 0.0, "y": -0.155, "title": T("Exhaust air leaving the lower-left spigot", "Fortluft am linken unteren Stutzen")},
}

SUMMARY = T(
    "A 0.5 m by 0.6 m heat-recovery section of an air handler: a galvanised casing on two base rails around an aluminium cross-flow plate core standing on one corner, four DN 200 spigots on the end walls and a bypass flap on the lid; drawn in section through the core.",
    "Ein 0,5 m mal 0,6 m großer Wärmerückgewinnungsteil eines Lüftungsgeräts: ein verzinktes Gehäuse auf zwei Fußschienen um einen auf einer Ecke stehenden Kreuzstrom-Plattenkern aus Aluminium, vier DN-200-Stutzen an den Stirnwänden und eine Bypass-Klappe auf dem Deckel; im Schnitt durch den Kern gezeichnet.",
)

NOTES = T(
    "A cross-flow plate heat exchanger section of the 1000 m3/h class. The core is about 192 embossed aluminium plates of 0.3 mm, 0.318 m square, at 2.5 mm pitch, 0.485 m long between two end plates and four corner profiles; it stands on one corner (diamond) in a V cradle and is sealed by a top rail and two side rails, which divide the 0.5 m by 0.57 m by 0.5 m galvanised casing into four chambers. Each chamber has a DN 200 spigot (EN 1506) on its end wall: outdoor air enters upper left and leaves as supply lower right, extract air enters upper right and leaves as exhaust lower left, so the two streams cross in the core. The front panel is the inspection cover; a condensate drain leaves the exhaust chamber; the casing stands on two U-channel base rails 30 mm high; a 0.4 m bypass flap on a hinge closes a 0.36 m opening in the lid. The mass is 18 kg (casing 5, core 12, spigots 0.4, flap 0.6). The side and front views are sections through the core (A-A, B-B). At full exchange the supply air leaves 3.1 K below the room and the exhaust air 17.9 K below it (EN 308 test point, 85 percent); casing, core, spigots and flap are passive and carry no heat of their own, and the flap reduces the exchange. The origin is the centre of the base on the floor.",
    "Ein Kreuzstrom-Plattenwärmeübertrager-Teil der Klasse 1000 m3/h. Der Kern besteht aus etwa 192 geprägten Aluminiumplatten von 0,3 mm, 0,318 m im Quadrat, mit 2,5 mm Teilung, 0,485 m lang zwischen zwei Endplatten und vier Eckprofilen; er steht auf einer Ecke (Raute) in einer V-Wiege und ist durch eine Deckschiene und zwei Seitenschienen abgedichtet, die das 0,5 m mal 0,57 m mal 0,5 m große verzinkte Gehäuse in vier Kammern teilen. Jede Kammer hat einen DN-200-Stutzen (EN 1506) an ihrer Stirnwand: Außenluft tritt oben links ein und unten rechts als Zuluft aus, Abluft tritt oben rechts ein und unten links als Fortluft aus, so dass sich die beiden Ströme im Kern kreuzen. Die vordere Wand ist der Revisionsdeckel; ein Kondensatablauf verlässt die Fortluftkammer; das Gehäuse steht auf zwei 30 mm hohen U-Profil-Fußschienen; eine 0,4 m lange Bypass-Klappe an einem Scharnier verschließt eine 0,36 m große Öffnung im Deckel. Die Masse beträgt 18 kg (Gehäuse 5, Kern 12, Stutzen 0,4, Klappe 0,6). Seiten- und Vorderansicht sind Schnitte durch den Kern (A-A, B-B). Bei vollem Austausch verlässt die Zuluft das Gerät 3,1 K unter Raumluft und die Fortluft 17,9 K darunter (EN-308-Prüfpunkt, 85 Prozent); Gehäuse, Kern, Stutzen und Klappe sind passiv und haben keine eigene Wärme, die Klappe verringert den Austausch. Der Ursprung ist die Mitte des Fußes auf der Unterlage.",
)

EMITTER_TITLES = {
    "supply-air": T("Supply air leaving the lower-right spigot", "Zuluft am rechten unteren Stutzen"),
    "exhaust-air": T("Exhaust air leaving the lower-left spigot", "Fortluft am linken unteren Stutzen"),
}

AREAS = {"frame": 1.54, "ports": 0.25}

REFERENCES = [
    {"id": "en-1506-2007", "citation": "DIN EN 1506:2007 Ventilation for buildings - Sheet metal air ducts and fittings with circular cross-section - Dimensions."},
    {"id": "en-13141-7-2021", "citation": "DIN EN 13141-7:2021 Ventilation for buildings - Performance testing of components/products for residential ventilation - Part 7: Mechanical supply and exhaust ventilation units (including heat recovery)."},
]


def apply_roots(asset):
    """🌱 Writes every root correction into the asset dict in place."""
    asset["summary"] = SUMMARY
    asset["provenance"]["notes"] = NOTES
    have = {r["id"] for r in asset["provenance"]["references"]}
    asset["provenance"]["references"] += [r for r in REFERENCES if r["id"] not in have]
    for bone in asset["rig"]["bones"]:
        fix = RIG_BONES[bone["id"]]
        bone["x"], bone["y"], bone["lengthM"] = fix["x"], fix["y"], fix["lengthM"]
        bone["mass"]["comX"], bone["mass"]["comY"] = fix["mass"]["comX"], fix["mass"]["comY"]
        if bone["id"] in BONE_TITLES:
            bone["title"] = BONE_TITLES[bone["id"]]
    for anchor in asset["anchors"]:
        fix = ANCHORS[anchor["id"]]
        anchor["x"], anchor["y"] = fix["x"], fix["y"]
        if "title" in fix:
            anchor["title"] = fix["title"]
    for region in asset["thermal"]["regions"]:
        if region["id"] in AREAS:
            region["areaM2"] = AREAS[region["id"]]
    for emitter in asset["thermal"]["emitters"]:
        emitter["title"] = EMITTER_TITLES[emitter["id"]]
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
    rep["reason"] = T("At 1:500 the exchanger is only 1.2 mm wide on paper and its smallest parts, the 0.3 mm plates and the 1 mm spigot and flap sheet, are far below every print profile.", "Bei 1:500 ist der Übertrager auf Papier nur 1,2 mm breit, und seine kleinsten Teile, die 0,3 mm dicken Platten und das 1-mm-Blech von Stutzen und Klappe, liegen weit unter jedem Druckprofil.")
    rep["limitingFeature"] = T("The 0.3 mm plates and the 1 mm sheet of spigots and flap", "Die 0,3 mm dicken Platten und das 1-mm-Blech von Stutzen und Klappe")
    import runpy
    runpy.run_path(os.path.join(os.path.dirname(__file__), "equipment-fine-ink.py"))["apply"](asset)
