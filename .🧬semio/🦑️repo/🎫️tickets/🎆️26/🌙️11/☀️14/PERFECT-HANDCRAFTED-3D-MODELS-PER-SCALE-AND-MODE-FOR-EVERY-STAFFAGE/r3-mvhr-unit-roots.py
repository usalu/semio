"""🌱 Round 3, appliances/mvhr-unit: the root corrections outside `volumes` (rig, anchors, materials, references, notes, thermal layers and regions, poses, structure assumptions) and the 2D shapes of 1:50, 1:100 and 1:200 drawn from the same measures as the solids (ruling E1).

`apply_roots(asset)` changes the asset dictionary in place.
"""
import importlib.util
import math
import os

_spec = importlib.util.spec_from_file_location("geo", os.path.join(os.path.dirname(os.path.abspath(__file__)), "r3-mvhr-unit-geo.py"))
g = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(g)
R5 = g.R5


def T(en, de):
    """🌐 Bilingual text."""
    return {"en": en, "de": de}


def pts(points):
    """📌 Rounded [x, y] list."""
    return [[R5(x), R5(y)] for x, y in points]


def shape(id, bone, role, z, tone, geom, filled=True, **kw):
    """✏️ One 2D shape with the member order of the catalog."""
    out = {"id": id, "bone": bone, "role": role, "z": z, "filled": filled, "tone": tone}
    out.update(geom)
    out.update(kw)
    return out


def poly(points, closed=True):
    """✏️ Poly geometry."""
    return {"kind": "poly", "points": pts(points), "closed": closed}


def ring_path(rings):
    """✏️ Path geometry with one closed subpath per ring (even-odd)."""
    d = []
    for ring in rings:
        d.append("M " + " L ".join(f"{R5(x)} {R5(y)}" for x, y in ring) + " Z")
    return {"kind": "path", "d": " ".join(d)}


def ellipse(cx, cy, r):
    """⭕ Ellipse geometry."""
    return {"kind": "ellipse", "cx": R5(cx), "cy": R5(cy), "rx": R5(r), "ry": R5(r)}


def rect(x, y, w, h, r=None):
    """⬜ Rect geometry."""
    out = {"kind": "rect", "x": R5(x), "y": R5(y), "w": R5(w), "h": R5(h)}
    if r:
        out["r"] = R5(r)
    return out


def cas_rect(x0, x1, y0, y1, r=None):
    """⬜ World rectangle as a rect of the `casing` elevation frame (u = Y, v = -X)."""
    return rect(y0, -x1, y1 - y0, x1 - x0, r)


def spig_rect(x0, x1, y0, y1, r=None):
    """⬜ World rectangle as a rect of the `spigots` elevation frame (u = Y - 0.6, v = -X)."""
    return rect(y0 - 0.6, -x1, y1 - y0, x1 - x0, r)


CAS = dict(region="casing", material="painted-steel")
DUCT = dict(region="ducts", material="galvanised-steel")
SEAL = dict(region="ducts", material="epdm-seal")
IMP = dict(region="impeller", material="pp-impeller")


def elevation(scale):
    """🖼️ Side elevation (the front face of the unit, seen from -Z): bone-local shapes."""
    out = [shape("casing", "casing", "body", 3, 0.82, cas_rect(-0.3, 0.3, 0, 0.6, g.EDGE_R), **CAS)]
    for i, (_, x, _) in enumerate(g.SPIGOTS, 1):
        ro = g.TUBE_RO if scale < 200 else 0.045
        out.append(shape(f"spigot-{i}", "spigots", "body", 3.1, 0.66 if scale < 200 else 0.62, spig_rect(x - ro, x + ro, 0.6, 0.6 + g.TUBE_H), **DUCT))
        if scale == 50:
            out.append(shape(f"bead-{i}", "spigots", "body", 3.2, 0.3, spig_rect(x - g.SEAL_RO, x + g.SEAL_RO, 0.6 + g.SEAL_Y0, 0.6 + g.TUBE_H, 0.003), **SEAL))
        if scale == 100:
            out.append(shape(f"bead-{i}", "spigots", "body", 3.2, 0.3, spig_rect(x - g.SEAL_RO, x + g.SEAL_RO, 0.647, 0.662), **SEAL))
    w = [0.3, 0.0]
    if scale == 50:
        out.append(shape("cover-edge", "casing", "detail", 3.3, 0.4, poly([g.casing_uv(x, y) for x, y in g.rr(0.292, 0.292, 0.0243, 0, 0.3, n=4)]), filled=False))
        out.append(shape("window-ring", "casing", "body", 3.4, 0.6, ellipse(*w, g.BEZEL_R), **CAS))
        out.append(shape("window", "casing", "glazing", 3.45, 0.14, ellipse(*w, g.GLASS_R), region="window", material="polycarbonate"))
        out.append(shape("back-plate", "impeller", "body", 3.47, 0.55, ellipse(0, 0, 0.095), **IMP))
        for k in range(7):
            line = g.rot([p for p in g.blade_line(n=8) if math.hypot(*p) <= 0.0625], 2 * math.pi * k / 7)
            out.append(shape(f"blade-{k + 1}", "impeller", "detail", 3.48, 0.8, poly(line, closed=False), filled=False))
        out.append(shape("shroud", "impeller", "body", 3.5, 0.85, ring_path([g.circle(0.095, n=48), g.circle(0.062, n=40)[::-1]]), **IMP))
        out.append(shape("hub", "impeller", "body", 3.6, 0.35, ellipse(0, 0, 0.035), **IMP))
        out.append(shape("led", "casing", "detail", 3.5, 0.95, ellipse(*g.casing_uv(g.LED[0], g.LED[1]), g.LED[2]), **CAS))
        x0, x1, y0, y1 = g.GRIP
        out.append(shape("handle", "casing", "detail", 3.5, 0.45, cas_rect(x0, x1, y0, y1, 0.012), **CAS))
    elif scale == 100:
        out.append(shape("window", "casing", "body", 3.4, 0.16, ellipse(*w, g.BEZEL_R), regions=["casing", "window"], material="polycarbonate"))
        out.append(shape("shroud", "impeller", "body", 3.5, 0.85, ellipse(0, 0, 0.095), **IMP))
        out.append(shape("back-plate", "impeller", "body", 3.55, 0.55, ellipse(0, 0, 0.062), **IMP))
        out.append(shape("hub", "impeller", "body", 3.6, 0.35, ellipse(0, 0, 0.035), **IMP))
    else:
        out.append(shape("window", "casing", "body", 3.4, 0.16, ellipse(*w, g.BEZEL_R), regions=["casing", "window"], material="polycarbonate"))
        out.append(shape("shroud", "impeller", "body", 3.5, 0.8, ellipse(0, 0, 0.095), **IMP))
    return out


def front(scale):
    """🖼️ Front elevation (the right side of the unit, seen from +X): x = world Z, y = world Y."""
    back = g.Z_BACK if scale < 200 else g.Z_WALL
    out = [shape("casing", "casing", "body", 3, 0.82, poly(g.side_outline(g.Z_FRONT, back, 0, 0.6, g.FILLET if scale == 50 else 0)), **CAS)]
    if scale == 50:
        out.append(shape("cover-seam", "casing", "detail", 3.3, 0.4, poly([(g.Z_COVER, 0.0), (g.Z_COVER, 0.6)], closed=False), filled=False))
        out.append(shape("hook", "casing", "body", 3.15, 0.5, poly([(z, y) for y, z in g.HOOK]), **CAS))
        out.append(shape("rail", "casing", "body", 3.1, 0.6, poly([(z, y) for y, z in g.RAIL]), material="galvanised-steel"))
        out.append(shape("rest", "casing", "body", 3.1, 0.25, rect(g.REST[2], g.REST[0], g.REST[3] - g.REST[2], g.REST[1] - g.REST[0]), material="epdm-seal"))
    elif scale == 100:
        out.append(shape("bracket", "casing", "body", 3.1, 0.55, rect(g.Z_BACK, 0.52, 0.015, 0.08), material="galvanised-steel"))
        out.append(shape("rest", "casing", "body", 3.1, 0.25, rect(g.REST[2], g.REST[0], g.REST[3] - g.REST[2], g.REST[1] - g.REST[0]), material="epdm-seal"))
    for i, (_, x, z) in enumerate(g.SPIGOTS[:2], 1):
        ro = g.TUBE_RO if scale < 200 else 0.045
        out.append(shape(f"spigot-{i}", "spigots", "body", 3.1, 0.66 if scale < 200 else 0.62, rect(z - ro, 0.6, 2 * ro, g.TUBE_H), **DUCT))
        if scale == 50:
            out.append(shape(f"bead-{i}", "spigots", "body", 3.2, 0.3, rect(z - g.SEAL_RO, 0.6 + g.SEAL_Y0, 2 * g.SEAL_RO, g.TUBE_H - g.SEAL_Y0, 0.003), **SEAL))
    return out


def plan(scale):
    """🖼️ Plan (seen from above): x = world X, y = world Z."""
    back = g.Z_BACK if scale < 200 else g.Z_WALL
    outline = [(x, z) for z, x in g.side_outline(g.Z_FRONT, back, -0.3, 0.3, g.FILLET if scale == 50 else 0)]
    out = [shape("casing", "casing", "body", 3, 0.82, poly(outline[::-1] if g.poly_area(outline) < 0 else outline), **CAS)]
    if scale == 50:
        out.append(shape("cover-seam", "casing", "detail", 3.3, 0.4, poly([(-0.3, g.Z_COVER), (0.3, g.Z_COVER)], closed=False), filled=False))
        out.append(shape("rail", "casing", "body", 3.1, 0.6, rect(-g.SPAN, 0.1475, 2 * g.SPAN, 0.0025), material="galvanised-steel"))
        out.append(shape("hook", "casing", "body", 3.15, 0.5, rect(-g.SPAN, g.Z_BACK, 2 * g.SPAN, 0.1465 - g.Z_BACK), **CAS))
    elif scale == 100:
        out.append(shape("bracket", "casing", "body", 3.1, 0.55, rect(-g.SPAN, g.Z_BACK, 2 * g.SPAN, 0.015), material="galvanised-steel"))
    for i, (_, x, z) in enumerate(g.SPIGOTS, 1):
        r = g.SEAL_RO if scale == 100 else (g.TUBE_RO if scale == 50 else 0.045)
        out.append(shape(f"spigot-{i}", "spigots", "body", 3.1, 0.66 if scale < 200 else 0.62, poly(g.circle(r, x, z, n=40)), **DUCT))
        if scale == 50:
            out.append(shape(f"bead-{i}", "spigots", "detail", 3.2, 0.3, poly(g.circle(g.SEAL_RO, x, z, n=40)), filled=False))
            out.append(shape(f"bore-{i}", "spigots", "detail", 3.25, 0.4, poly(g.circle(g.TUBE_RI, x, z, n=40)), filled=False))
    return out


REP_DECISIONS = {
    50: {
        "silhouette": T("A 0.6 m by 0.665 m wall unit, 12 mm by 13.3 mm on paper: the front cover with its rounded edge, a round window with a bezel showing the shrouded supply impeller (inlet eye, blade edges, motor hub), a status light, a pull grip, and four round duct spigots with a sealing ring at the top. Front view: the 285 mm casing with the 8 mm rounded front edge, the cover joint, the hook strip on the wall rail and the rubber rest strip that hold it 15 mm off the wall, two of the four spigots. Top view: the casing with the cover joint, the hook strip over the rail, and the four spigots with seal ring and bore in a zigzag 0.07 m either side of the middle.", "Ein Wandgerät von 0,6 m mal 0,665 m, auf Papier 12 mm mal 13,3 mm: der Frontdeckel mit gerundeter Kante, ein rundes Fenster mit Ring, das das Zuluftlaufrad mit Deckscheibe zeigt (Einlaufauge, Schaufelkanten, Motornabe), eine Statusleuchte, eine Griffleiste und vier runde Kanalstutzen mit Dichtring oben. Vorderansicht: das 285 mm tiefe Gehäuse mit der um 8 mm gerundeten Vorderkante, die Deckelfuge, die Hakenleiste auf der Wandschiene und die Gummi-Auflageleiste, die es 15 mm von der Wand halten, zwei der vier Stutzen. Draufsicht: das Gehäuse mit Deckelfuge, die Hakenleiste über der Schiene und die vier Stutzen mit Dichtring und Bohrung im Zickzack 0,07 m beidseits der Mitte."),
        "retained": T("Casing 0.6 m with 25 mm bent edges, cover edge line 8 mm inside the outline, bezel 256 mm, window 224 mm, impeller shroud 190 mm with its 124 mm eye, back plate, seven blade edges in the eye, hub 70 mm, light 28 mm, grip 160 x 24 mm, four spigots of 98 mm with seal rings of 114 mm, cover joint, hook strip, rail and rest strip.", "Gehäuse 0,6 m mit 25-mm-Biegekanten, Deckelkante 8 mm innerhalb des Umrisses, Ring 256 mm, Fenster 224 mm, Deckscheibe des Laufrads 190 mm mit 124-mm-Auge, Tragscheibe, sieben Schaufelkanten im Auge, Nabe 70 mm, Leuchte 28 mm, Griff 160 x 24 mm, vier Stutzen von 98 mm mit Dichtringen von 114 mm, Deckelfuge, Hakenleiste, Schiene und Auflageleiste."),
        "merged": T("The plate heat exchanger, the extract fan and the filter cassettes behind the cover are their own hidden thermal regions and are not drawn; only the supply impeller shows through the window.", "Plattenwärmeübertrager, Abluftventilator und Filterkassetten hinter dem Deckel sind eigene verborgene Thermikregionen und werden nicht gezeichnet; nur das Zuluftlaufrad zeigt sich durch das Fenster."),
        "omitted": T("Condensate drain, power cable, screws, the label and the duct labels; the blades behind the shroud. Front and top view omit window, light and grip, which are seen edge-on there (2.5 to 6.5 mm).", "Kondensatablauf, Netzkabel, Schrauben, Typenschild und Stutzenbeschriftung; die Schaufeln hinter der Deckscheibe. Vorder- und Draufsicht lassen Fenster, Leuchte und Griff weg, die dort von der Kante zu sehen wären (2,5 bis 6,5 mm)."),
        "thinElements": T("The seal rings are 14 mm (0.28 mm), the rail web and the hook strip 2.5 to 3 mm and drawn at their true size in the body pen; the blade edges, the cover edge, the joint and the bores are 0.13 mm detail strokes.", "Die Dichtringe sind 14 mm (0,28 mm), Schienensteg und Hakenleiste 2,5 bis 3 mm und in wahrer Größe im Körperstift gezeichnet; Schaufelkanten, Deckelkante, Fuge und Bohrungen sind 0,13-mm-Detailstriche."),
        "hatching": T("None; the window is a dark tone.", "Keine; das Fenster ist ein dunkler Ton."),
        "styles": T("Casing 0.82, bezel 0.6, window 0.14, shroud 0.85, back plate 0.55, hub 0.35, spigots 0.66, seals 0.3, rail 0.6, hook 0.5, rest strip 0.25; body pen 0.18 mm, detail and glazing 0.13 mm, ground 0.35 mm.", "Gehäuse 0,82, Ring 0,6, Fenster 0,14, Deckscheibe 0,85, Tragscheibe 0,55, Nabe 0,35, Stutzen 0,66, Dichtungen 0,3, Schiene 0,6, Hakenleiste 0,5, Auflageleiste 0,25; Körperstift 0,18 mm, Detail und Verglasung 0,13 mm, Boden 0,35 mm."),
        "overlays": T("Casing, window, impeller and ducts are drawn regions: in the thermal view the casing and the impeller warm, the spigots follow the air they carry; the plate pack, the extract fan and the filters are hidden regions that only the 3D thermal model shows.", "Gehäuse, Fenster, Laufrad und Kanäle sind gezeichnete Regionen: in der Thermalansicht erwärmen sich Gehäuse und Laufrad, die Stutzen folgen der Luft, die sie tragen; Plattenpaket, Abluftventilator und Filter sind verborgene Regionen, die nur das 3D-Thermikmodell zeigt."),
    },
    100: {
        "silhouette": T("A 6 mm by 6.7 mm box with a round window holding the impeller disc with its eye and hub, and four spigots with their seal bands on top. Front view: the 285 mm casing with bracket and rest strip at the wall, two spigots without their seal bands (the 42 mm gap between the two rows would close to 0.08 mm). Top view: the casing with the bracket and four round spigots in a zigzag.", "Ein Kasten von 6 mm mal 6,7 mm mit rundem Fenster, das die Laufradscheibe mit Auge und Nabe enthält, und vier Stutzen mit Dichtbändern oben. Vorderansicht: das 285 mm tiefe Gehäuse mit Konsole und Auflageleiste an der Wand, zwei Stutzen ohne Dichtbänder (die 42-mm-Lücke zwischen den Reihen schlösse sich auf 0,08 mm). Draufsicht: das Gehäuse mit der Konsole und vier runden Stutzen im Zickzack."),
        "retained": T("Casing with 25 mm rounded corners, window 256 mm, impeller shroud, eye and hub, four spigots with 15 mm seal bands, bracket and rest strip in front and top view.", "Gehäuse mit 25-mm-Rundungen, Fenster 256 mm, Deckscheibe, Auge und Nabe des Laufrads, vier Stutzen mit 15-mm-Dichtbändern, Konsole und Auflageleiste in Vorder- und Draufsicht."),
        "merged": T("Bezel and window are one disc (regions casing and window merged); hook strip and rail are one bracket; the blade edges join the back plate in the eye.", "Ring und Fenster sind eine Scheibe (Regionen Gehäuse und Fenster zusammengeführt); Hakenleiste und Schiene sind eine Konsole; die Schaufelkanten gehen im Auge in der Tragscheibe auf."),
        "omitted": T("Cover edge and joint, light and grip (below 15 mm). Front and top view omit the window as in the side view and the seal bands in the top view.", "Deckelkante und -fuge, Leuchte und Griff (unter 15 mm). Vorder- und Draufsicht lassen wie die Seitenansicht das Fenster weg, die Draufsicht die Dichtbänder."),
        "thinElements": T("The seal bands are 15 mm (0.15 mm) and the bracket 15 mm deep; nothing is widened.", "Die Dichtbänder sind 15 mm (0,15 mm) und die Konsole 15 mm tief; nichts ist verbreitert."),
        "hatching": T("None.", "Keine."),
        "styles": T("Casing 0.82, window 0.16, shroud 0.85, back plate 0.55, hub 0.35, spigots 0.66, seals 0.3, bracket 0.55, rest strip 0.25; one body pen of 0.18 mm.", "Gehäuse 0,82, Fenster 0,16, Deckscheibe 0,85, Tragscheibe 0,55, Nabe 0,35, Stutzen 0,66, Dichtungen 0,3, Konsole 0,55, Auflageleiste 0,25; ein Körperstift von 0,18 mm."),
        "overlays": T("The window disc lists casing and window as merged regions; impeller and ducts stay their own.", "Die Fensterscheibe führt Gehäuse und Fenster als zusammengeführte Regionen; Laufrad und Kanäle bleiben eigene."),
    },
    200: {
        "silhouette": T("A 3 mm by 3.3 mm box with a dark window disc holding the light impeller disc and four stubs on top. Front view: a 0.3 m by 0.6 m block to the wall with two spigots on top. Top view: a 0.6 m by 0.3 m block with four round spigots.", "Ein Kasten von 3 mm mal 3,3 mm mit dunkler Fensterscheibe, die die helle Laufradscheibe enthält, und vier Stummeln oben. Vorderansicht: ein Block von 0,3 m mal 0,6 m bis zur Wand mit zwei Stutzen oben. Draufsicht: ein Block von 0,6 m mal 0,3 m mit vier runden Stutzen."),
        "retained": T("Box, window disc, impeller disc, four spigots generalised to 90 mm.", "Kasten, Fensterscheibe, Laufradscheibe, vier auf 90 mm verallgemeinerte Stutzen."),
        "merged": T("Bezel and window are one disc; shroud, eye and hub are one disc; bracket and rest strip are within the 0.3 m depth of the block; the seals join the spigots.", "Ring und Fenster sind eine Scheibe; Deckscheibe, Auge und Nabe sind eine Scheibe; Konsole und Auflageleiste liegen in der Tiefe von 0,3 m des Blocks; die Dichtungen gehen in den Stutzen auf."),
        "omitted": T("Cover edge and joint, light, grip, seals and the eye of the impeller (below 30 mm). Front and top view omit window and impeller as in the side view.", "Deckelkante und -fuge, Leuchte, Griff, Dichtungen und das Auge des Laufrads (unter 30 mm). Vorder- und Draufsicht lassen wie die Seitenansicht Fenster und Laufrad weg."),
        "thinElements": T("Nothing below 30 mm is drawn; the 98 mm spigots are drawn 90 mm wide so that the 42 mm gap between the two rows in the front view stays printable (60 mm, 0.3 mm on paper less the pen).", "Nichts unter 30 mm wird gezeichnet; die 98-mm-Stutzen sind 90 mm breit gezeichnet, damit die 42-mm-Lücke zwischen den Reihen in der Vorderansicht druckbar bleibt (60 mm, auf Papier 0,3 mm abzüglich Stift)."),
        "hatching": T("None.", "Keine."),
        "styles": T("Casing 0.82, window 0.16, impeller 0.8, spigots 0.62; one body pen of 0.13 mm.", "Gehäuse 0,82, Fenster 0,16, Laufrad 0,8, Stutzen 0,62; ein Körperstift von 0,13 mm."),
        "overlays": T("As at 1:100.", "Wie bei 1:100."),
    },
}


def material(id, en, de, eps, alpha, rho, lam, c, source, ref):
    """🧪 A material entry."""
    return {"id": id, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": lam, "specificHeatJKgK": c, "source": source, "reference": ref}


def layer(id, en, de, kind, mat, t, r, c, ref):
    """🧅 A thermal layer."""
    return {"id": id, "title": T(en, de), "kind": kind, "material": mat, "thicknessM": t, "resistanceM2KW": r, "heatCapacityJM2K": c, "source": "authored-estimate", "reference": ref}


def apply_roots(a, areas):
    """🌱 Applies every root correction to the asset dictionary `a`; `areas` maps region ids to the hand-checked areaM2."""
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones["casing"].update({"angleDeg": 90})
    bones["casing"]["title"] = T("Casing on the wall rail (vertical centre line)", "Gehäuse an der Wandschiene (senkrechte Mittellinie)")
    bones["casing"]["mass"].update({"comX": 0.3, "comY": 0})
    bones["spigots"].update({"x": 0.6, "y": 0})
    bones["spigots"]["mass"].update({"comX": 0.03, "comY": 0})
    imp = bones["impeller"]
    new_imp = {}
    for k, v in imp.items():
        new_imp[k] = v
        if k == "y":
            new_imp["z"] = g.IMP_Z
    new_imp.update({"x": 0.3, "y": 0, "angleDeg": -90})
    a["rig"]["bones"] = [new_imp if b["id"] == "impeller" else b for b in a["rig"]["bones"]]
    anchors = {x["id"]: x for x in a["anchors"]}
    anchors["wall-hook"].update({"x": 0.586, "y": 0})
    anchors["wall-hook"]["title"] = T("Hook strip on the wall rail", "Hakenleiste auf der Wandschiene")
    anchors["wall-rest"].update({"x": 0.03, "y": 0})
    anchors["wall-rest"]["title"] = T("Rubber rest strip against the wall", "Gummi-Auflageleiste an der Wand")
    anchors["supply-spigot"].update({"x": 0.065, "y": -0.075})
    anchors["exhaust-spigot"].update({"x": 0.065, "y": 0.225})

    def with_z(anchor, z, extra=None):
        out = {}
        for k, v in anchor.items():
            if k == "z":
                continue
            out[k] = v
            if k == "y":
                out["z"] = z
        for k, v in (extra or {}).items():
            out[k] = v
        return out

    a["anchors"] = [
        with_z(anchors["wall-hook"], 0.145, None),
        with_z(anchors["wall-rest"], 0.15, None),
        with_z(anchors["supply-spigot"], 0.07),
        with_z(anchors["exhaust-spigot"], 0.07),
    ]
    rest = a["anchors"][1]
    order = list(rest.keys())
    rest["tiltDeg"] = -90
    a["anchors"][1] = {k: rest[k] for k in order[: order.index("normalDeg") + 1]} | {"tiltDeg": -90} | {k: rest[k] for k in order[order.index("normalDeg") + 1 :] if k != "tiltDeg"}
    a["anchors"][0]["normalDeg"] = 90

    have = {m["id"] for m in a["materials"]}
    for m in [
        material("epp-foam", "Black expanded polypropylene (EPP) 35 kg/m3", "Schwarzes expandiertes Polypropylen (EPP) 35 kg/m3", 0.9, 0.9, 35, 0.039, 1800, "authored-estimate", "iso-10456-2007"),
        material("polystyrene-plate", "Polystyrene heat-exchanger plates 0.25 mm", "Polystyrol-Wärmeübertragerplatten 0,25 mm", 0.9, 0.5, 1050, 0.16, 1300, "literature", "iso-10456-2007"),
        material("filter-media", "Pleated synthetic filter media in a plastic frame", "Plissiertes Kunstfaser-Filtermedium im Kunststoffrahmen", 0.9, 0.5, 60, 0.04, 1300, "authored-estimate", "iso-10456-2007"),
        material("epdm-seal", "EPDM double-lip seal", "EPDM-Doppellippendichtung", 0.9, 0.9, 1150, 0.25, 1000, "literature", "iso-10456-2007"),
    ]:
        if m["id"] not in have:
            a["materials"].append(m)

    prov = a["provenance"]
    refs = {r["id"] for r in prov["references"]}
    for r in [
        {"id": "iso-10456-2007", "citation": "EN ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values (polystyrene, polypropylene, EPDM)."},
        {"id": "vent-axia-kinetic", "citation": "Vent-Axia Kinetic whole-house MVHR wall unit, specification: 550 x 550 x 285 mm, 15 kg, four 125 mm spigots on top, composite plastic counter-flow heat exchanger, EC fans (product class reference for size and layout)."},
        {"id": "envirovent-energisava-260", "citation": "EnviroVent energiSava 260 technical data sheet ENV627 issue B: wall-hung MVHR unit 560 x 635 x 260 mm, 14 kg, 125 mm spigots (product class reference)."},
        {"id": "ebm-papst-r3g190", "citation": "ebm-papst RadiCal R3G190 EC centrifugal fan: backward-curved impeller of 190 mm with front shroud and inlet ring, external-rotor motor in the hub (fan class of residential MVHR units)."},
    ]:
        if r["id"] not in refs:
            prov["references"].append(r)
    prov["notes"] = T(
        "A compact wall unit of the 250 m3/h class (Vent-Axia Kinetic 550 x 550 x 285 mm, EnviroVent energiSava 260 560 x 635 x 260 mm; this one 0.6 x 0.6 x 0.3 m in powder-coated steel, 24 kg). Construction: a 1 mm steel wrapper with 25 mm bent edges and a back tray, lined with 19 mm black EPP that forms the air channels; a removable front cover 30 mm deep with an 8 mm rounded edge, a round inspection window of 224 mm in a 256 mm bezel (a didactic feature: real units are closed), a 28 mm status light and a pull grip. Behind the window the supply fan of the R3G190 class: a 190 mm backward-curved impeller with front shroud and 124 mm inlet eye, seven blades and the external rotor of its EC motor in the hub. Inside: a counter-flow plate pack of polystyrene plates (hexagonal, 0.53 x 0.26 m, 0.195 m deep, 88 plates at 2.2 mm), the extract fan in its scroll rising to the exhaust spigot, and two filter cassettes under the extract and outdoor spigots. Four DN100 spigots of 98 mm with double-lip EPDM seals on top, from the left: exhaust, extract, supply, outdoor air (exhaust and supply in the back row, the two intakes in the front row). The unit hangs with a folded hook strip on a galvanised wall rail and rests with a rubber strip at its lower edge, 15 mm off the wall. The overall height with the spigots is 0.665 m; the origin is the lower edge of the casing, x = 0 on the centreline. At level 1 each stream is 0.061 m3/s (220 m3/h) and the two fans take 75 W (about 33 W warm the air streams, 12 W show at the casing behind its lining, which runs about 1.5 K above the room, and 30 W at the supply impeller); at level 0.5 it is 110 m3/h and 38 W, linearised from a fan law with an SFP of 0.34 to 0.38 Wh/m3. The heat recovery is 85 percent (temperature ratio of the heat exchanger, EN 308 test point 0 degC outdoors, 21 degC extract): the supply air leaves at 0.85 x 21 = 17.9 degC plus 0.5 K of fan heat, 2.6 K below the room, and the exhaust air at 3.2 degC, 17.9 K below the room.",
        "Ein kompaktes Wandgerät der Klasse 250 m3/h (Vent-Axia Kinetic 550 x 550 x 285 mm, EnviroVent energiSava 260 560 x 635 x 260 mm; dieses 0,6 x 0,6 x 0,3 m in pulverbeschichtetem Stahl, 24 kg). Aufbau: ein 1-mm-Stahlblechmantel mit 25-mm-Biegekanten und einer Rückwandwanne, ausgekleidet mit 19 mm schwarzem EPP, das die Luftkanäle bildet; ein abnehmbarer Frontdeckel von 30 mm Tiefe mit um 8 mm gerundeter Kante, ein rundes Schauglas von 224 mm in einem Ring von 256 mm (ein didaktisches Merkmal: reale Geräte sind geschlossen), eine Statusleuchte von 28 mm und eine Griffleiste. Hinter dem Fenster der Zuluftventilator der Klasse R3G190: ein rückwärts gekrümmtes Laufrad von 190 mm mit Deckscheibe und 124-mm-Einlaufauge, sieben Schaufeln und der Außenläufer seines EC-Motors in der Nabe. Innen: ein Gegenstrom-Plattenpaket aus Polystyrolplatten (sechseckig, 0,53 x 0,26 m, 0,195 m tief, 88 Platten im Abstand von 2,2 mm), der Abluftventilator in seiner Spirale, die zum Fortluftstutzen steigt, und zwei Filterkassetten unter Abluft- und Außenluftstutzen. Oben vier DN100-Stutzen von 98 mm mit EPDM-Doppellippendichtungen, von links: Fortluft, Abluft, Zuluft, Außenluft (Fortluft und Zuluft in der hinteren Reihe, die beiden Ansaugungen in der vorderen). Das Gerät hängt mit einer gekanteten Hakenleiste an einer verzinkten Wandschiene und liegt unten mit einer Gummileiste an, 15 mm von der Wand. Die Gesamthöhe mit den Stutzen beträgt 0,665 m; der Ursprung ist die Unterkante des Gehäuses, x = 0 auf der Mittellinie. Auf Stufe 1 beträgt jeder Strom 0,061 m3/s (220 m3/h) und die beiden Ventilatoren nehmen 75 W auf (etwa 33 W erwärmen die Luftströme, 12 W zeigen sich am Gehäuse hinter seiner Auskleidung, das etwa 1,5 K über dem Raum liegt, und 30 W am Zuluftlaufrad); bei Stufe 0,5 sind es 110 m3/h und 38 W, linearisiert nach einem Ventilatorgesetz mit einer SFP von 0,34 bis 0,38 Wh/m3. Die Wärmerückgewinnung beträgt 85 Prozent (Temperaturverhältnis des Wärmeübertragers, Prüfpunkt nach EN 308 mit 0 degC Außenluft und 21 degC Abluft): die Zuluft verlässt das Gerät mit 0,85 x 21 = 17,9 degC plus 0,5 K Ventilatorwärme, 2,6 K unter der Raumluft, und die Fortluft mit 3,2 degC, 17,9 K unter der Raumluft.",
    )
    a["summary"] = T(
        "A wall-mounted mechanical ventilation unit with heat recovery of 0.6 m by 0.6 m: a steel box hung on a wall rail, with an inspection window onto the turning shrouded supply impeller and four duct spigots on top; inside a counter-flow plate heat exchanger, the extract fan and two filters; it moves 110 to 220 m3/h and returns 85 percent of the heat of the extract air for 38 to 75 W.",
        "Ein wandmontiertes Lüftungsgerät mit Wärmerückgewinnung von 0,6 m mal 0,6 m: ein Stahlkasten an einer Wandschiene, mit Schauglas auf das drehende Zuluftlaufrad mit Deckscheibe und vier Kanalstutzen oben; innen ein Gegenstrom-Plattenwärmeübertrager, der Abluftventilator und zwei Filter; es bewegt 110 bis 220 m3/h und gibt 85 Prozent der Abluftwärme zurück bei 38 bis 75 W.",
    )

    regions = {r["id"]: r for r in a["thermal"]["regions"]}
    cas = regions["casing"]
    cas["layers"] = [
        layer("liner", "Black EPP lining 19 mm", "Schwarze EPP-Auskleidung 19 mm", "insulation", "epp-foam", 0.019, 0.487, 1197, "iso-10456-2007"),
        layer("sheet", "Powder-coated steel sheet 1 mm", "Pulverbeschichtetes Stahlblech 1 mm", "metal", "painted-steel", 0.001, 2e-05, 3822, "incropera-2018"),
    ]
    cas["heatCapacityJM2K"] = 5019
    imp_region = regions["impeller"]
    imp_region["title"] = T("Supply impeller and its rotor", "Zuluftlaufrad mit Läufer")
    new_imp_region = {}
    for k, v in imp_region.items():
        new_imp_region[k] = v
        if k == "areaM2":
            new_imp_region["areaBasis"] = "exchange"
    imp_region.clear()
    imp_region.update(new_imp_region)
    imp_region["layers"][0]["title"] = T("Polypropylene shroud, blades and back plate 2.5 mm", "Polypropylen-Deckscheibe, Schaufeln und Tragscheibe 2,5 mm")
    regions["ducts"]["layers"][0]["title"] = T("Galvanised steel tube 0.7 mm with EPDM seal", "Verzinktes Stahlrohr 0,7 mm mit EPDM-Dichtung")
    hidden = lambda en, de: T(en, de)
    new = [
        {"id": "core", "title": T("Counter-flow plate heat exchanger", "Gegenstrom-Plattenwärmeübertrager"), "kind": "surface", "model": "passive", "material": "polystyrene-plate", "areaM2": 18.3, "areaBasis": "exchange", "characteristicLengthM": 0.0022, "geometry": "vertical-plate", "skyViewFactor": 0,
         "layers": [layer("plates", "Polystyrene plate 0.25 mm", "Polystyrolplatte 0,25 mm", "polymer", "polystyrene-plate", 0.00025, 0.0016, 341, "iso-10456-2007")], "heatCapacityJM2K": 341, "source": "authored-estimate", "reference": "iso-10456-2007",
         "hidden": hidden("The plate pack sits in the EPP lining behind the cover; no drawing shows it. Its area is the wetted area of 88 plates of 0.104 m2 on both sides (18.3 m2), the exchange area of the 85 percent heat recovery, not an envelope.", "Das Plattenpaket sitzt in der EPP-Auskleidung hinter dem Deckel; keine Zeichnung zeigt es. Seine Fläche ist die benetzte Fläche von 88 Platten zu 0,104 m2 auf beiden Seiten (18,3 m2), die Austauschfläche der 85-prozentigen Wärmerückgewinnung, keine Hülle.")},
        {"id": "extract-fan", "title": T("Extract fan in its scroll", "Abluftventilator in seiner Spirale"), "kind": "surface", "model": "passive", "material": "pp-impeller", "areaM2": areas.get("extract-fan", 0.15), "characteristicLengthM": 0.144, "geometry": "cylinder", "skyViewFactor": 0,
         "layers": [layer("scroll", "Polypropylene scroll 2.5 mm", "Polypropylen-Spirale 2,5 mm", "polymer", "pp-impeller", 0.0025, 0.0114, 4275, "incropera-2018")], "heatCapacityJM2K": 4275, "source": "authored-estimate", "reference": "incropera-2018",
         "hidden": hidden("The extract fan and its scroll sit in the upper left of the lining under the exhaust spigot; no drawing shows them. Its electrical input is part of the 75 W of the supply impeller region and its heat part of the 12 W of the casing.", "Der Abluftventilator und seine Spirale sitzen oben links in der Auskleidung unter dem Fortluftstutzen; keine Zeichnung zeigt sie. Seine Leistungsaufnahme ist Teil der 75 W der Region des Zuluftlaufrads, seine Wärme Teil der 12 W des Gehäuses.")},
        {"id": "filters", "title": T("Filter cassettes (extract ISO coarse, outdoor ePM1)", "Filterkassetten (Abluft ISO Grob, Außenluft ePM1)"), "kind": "surface", "model": "passive", "material": "filter-media", "areaM2": areas.get("filters", 0.24), "characteristicLengthM": 0.035, "geometry": "horizontal-plate-up", "skyViewFactor": 0,
         "layers": [layer("media", "Pleated filter media 35 mm", "Plissiertes Filtermedium 35 mm", "insulation", "filter-media", 0.035, 0.875, 2730, "iso-10456-2007")], "heatCapacityJM2K": 2730, "source": "authored-estimate", "reference": "iso-10456-2007",
         "hidden": hidden("The two cassettes lie under the extract and outdoor spigots behind the cover and are changed through it; no drawing shows them.", "Die zwei Kassetten liegen unter Abluft- und Außenluftstutzen hinter dem Deckel und werden durch ihn gewechselt; keine Zeichnung zeigt sie.")},
    ]
    ids = {r["id"] for r in a["thermal"]["regions"]}
    a["thermal"]["regions"] += [r for r in new if r["id"] not in ids]
    for r in a["thermal"]["regions"]:
        if r["id"] in areas:
            r["areaM2"] = areas[r["id"]]
    a["thermal"]["assumptions"] = a["thermal"]["assumptions"][:2] + [T(
        "The plate pack, the extract fan and the filter cassettes are hidden passive regions inside the lining: they give the 3D thermal model its real internals, while the solver keeps them at the reference air of the room (their temperatures between the two streams are not solved); the casing area is the outer area of its steel shell, 1.37 m2 for the 0.6 x 0.6 x 0.285 m box with the window opening, the back 15 mm off the wall included.",
        "Plattenpaket, Abluftventilator und Filterkassetten sind verborgene passive Regionen in der Auskleidung: sie geben dem 3D-Thermikmodell sein wirkliches Inneres, während der Löser sie auf der Bezugsluft des Raums hält (ihre Temperaturen zwischen den beiden Strömen werden nicht gelöst); die Gehäusefläche ist die Außenfläche seiner Stahlschale, 1,37 m2 für den Kasten von 0,6 x 0,6 x 0,285 m mit der Fensteröffnung, die Rückseite 15 mm vor der Wand eingeschlossen.")]
    for pose in a["poses"]:
        regs = pose["thermal"]["regions"]
        for rid in ["core", "extract-fan", "filters"]:
            regs.setdefault(rid, {"exposedFraction": 1, "sunlitFraction": 0})
    a["structure"]["assumptions"] = [T(
        "The 24 kg unit hangs with the folded hook strip on its back over the wall rail at 0.586 m (pin) and leans with the rubber strip at its lower edge against the wall (roller pressed by the wall): the rail carries the weight of 235 N, and the pair resists the overturning moment of the centre of mass 0.15 m in front of the wall with about 63 N of pull at the rail and of push at the strip (235 N x 0.15 m / 0.556 m).",
        "Das 24 kg schwere Gerät hängt mit der gekanteten Hakenleiste auf seiner Rückseite über der Wandschiene auf 0,586 m (Festlager) und lehnt unten mit der Gummileiste an der Wand (Rollenlager, von der Wand gedrückt): die Schiene trägt das Gewicht von 235 N, und das Paar nimmt das Kippmoment des Schwerpunkts 0,15 m vor der Wand mit etwa 63 N Zug an der Schiene und Druck an der Leiste auf (235 N x 0,15 m / 0,556 m).")]

    for d in (50, 100, 200):
        rep = a["representations"][str(d)]
        rep["geometry"] = {"elevation": elevation(d), "plan": plan(d), "front": front(d)}
        rep["decisions"] = REP_DECISIONS[d]
    return a
