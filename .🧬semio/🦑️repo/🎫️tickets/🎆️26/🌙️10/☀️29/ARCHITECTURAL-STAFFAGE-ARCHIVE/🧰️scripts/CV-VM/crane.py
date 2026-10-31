import sys
from build import *

REL = "🏗️machinery/tower-crane"
KEYS = [
    opkey("hoisting", "Hoist winch", "Hubwerk", [(0, "Off", "Aus"), (0.15, "Inching", "Schleichgang"), (0.5, "Part speed", "Teilgeschwindigkeit"), (1, "Rated lift", "Nennlast heben")]),
    opkey("slewing", "Slewing drive", "Drehwerk", [(0, "Off, weathervane", "Aus, Windfahne"), (0.3, "Slow slewing", "Langsames Schwenken"), (1, "Full speed", "Volle Geschwindigkeit")]),
]
steel = lambda mm, hc: layer("steel", "Steel %d mm" % mm, "Stahl %d mm" % mm, "metal", "steel-s355", mm / 1000, mm / 1000 / 50, hc, source="literature", ref="incropera-2011")
conc = lambda m, hc: layer("concrete", "Concrete %.2f m (thermally active part)" % m, "Beton %.2f m (thermisch wirksamer Teil)" % m, "masonry", "concrete", m, m / 1.7, hc, source="literature", ref="concrete-density")
REGIONS = [
    region("lattice", "Lattice steel of tower, jib and counter-jib", "Gitterstahl von Turm, Ausleger und Gegenausleger", "surface", "passive", "steel-s355", 180, 0.12, "cylinder", 0.9, 30000, [steel(8, 30000)], ref="ashrae-2021"),
    region("ballast", "Concrete ballast", "Betonballast", "surface", "passive", "concrete", 40, 3.5, "vertical-plate", 0.5, 300000, [conc(0.15, 300000)], ref="ashrae-2021"),
    region("counterweight", "Counterweight slabs", "Gegengewichtsplatten", "surface", "passive", "concrete", 25, 2.4, "vertical-plate", 0.7, 300000, [conc(0.15, 300000)], ref="ashrae-2021"),
    region("house", "Slewing platform", "Drehbühne", "surface", "passive", "machinery", 25, 3, "horizontal-plate-up", 0.9, 11000, [steel(3, 11000)], ref="ashrae-2021"),
    region("slew-drive", "Slewing drive housings", "Drehwerksgehäuse", "engine", "operating", "machinery", 1, 0.5, "cylinder", 0.5, 30000, [], ("slewing", 450)),
    region("cab", "Operator cab shell", "Kabinenhülle des Kranführers", "wall", "conditioned", "machinery", 12, 2, "vertical-plate", 0.6, 12000, [steel(3, 12000), layer("lining", "Insulated lining 30 mm", "Gedämmte Verkleidung 30 mm", "insulation", "machinery", 0.03, 0.03 / 0.04, 600, source="authored-estimate")], ref="ashrae-2021"),
    region("cab-glazing", "Cab glazing", "Kabinenverglasung", "glazing", "conditioned", "glass", 4, 1.2, "vertical-plate", 0.6, 15000,
           [layer("pane-out", "Glass 4 mm", "Glas 4 mm", "glass", "glass", 0.004, 0.004, 7500, source="literature", ref="incropera-2011"), layer("gap", "Air gap 12 mm", "Luftspalt 12 mm", "air-gap", "glass", 0.012, 0.15, None, source="literature", ref="incropera-2011"),
            layer("pane-in", "Glass 4 mm", "Glas 4 mm", "glass", "glass", 0.004, 0.004, 7500, source="literature", ref="incropera-2011")], ref="ashrae-2021"),
    region("winch-house", "Winch house skin", "Außenhaut des Windenhauses", "surface", "operating", "machinery", 38, 2, "vertical-plate", 0.6, 11000, [steel(3, 11000)], ("hoisting", 5700)),
    region("hoist-motor", "Hoist motor and drive surfaces", "Hubmotor- und Antriebsoberflächen", "engine", "operating", "machinery", 3, 0.8, "cylinder", 0.3, 40000, [], ("hoisting", 1600)),
    region("trolley", "Trolley", "Laufkatze", "surface", "passive", "machinery", 4, 1.4, "vertical-plate", 0.7, 20000, [steel(6, 20000)], ref="ashrae-2021"),
    region("skip", "Hook block and concrete skip", "Hakenflasche und Betonkübel", "surface", "passive", "skip-steel", 8, 1.2, "vertical-plate", 0.7, 40000, [steel(8, 40000)], ref="ashrae-2021"),
    region("pendants", "Pendant ties and slings", "Abspannungen und Anschlagseile", "surface", "passive", "wire-rope", 3, 0.05, "cylinder", 0.9, 3000, []),
    region("rope", "Hoist rope", "Hubseil", "surface", "passive", "wire-rope", 1, 0.02, "cylinder", 0.9, 2000, []),
]
for r in REGIONS:
    if r["model"] == "conditioned":
        r["conditioned"] = {"interiorTempC": 21, "interiorFilmM2KW": 0.13}
EMITTERS = [emitter("winch-vent", "vent", "Winch house ventilation outlet", "Lüftungsauslass des Windenhauses", "winch-vent", 90, 0.4, "hoisting", 0.6, 25, 0.0)]
DEFAULTS = {"lattice": (1, 0.5), "ballast": (1, 0.4), "counterweight": (1, 0.6), "house": (1, 0.6), "slew-drive": (0.5, 0), "cab": (1, 0.5), "cab-glazing": (1, 0.5), "winch-house": (1, 0.5), "hoist-motor": (0.3, 0), "trolley": (1, 0.8), "skip": (1, 0.7), "pendants": (1, 0.8), "rope": (1, 0.8)}
CONTACTS = {"ballast": (0.15, 25)}
P = lambda h, s=0: {"operating": {"hoisting": h, "slewing": s}, "emitters": {"winch-vent": 1 if h > 0 else 0}}
POSES = {"parked-weathervane": P(0), "lifting-load": P(0.8), "max-reach": P(0.6), "skip-at-ground": P(0.2)}


def fix_clips(clips):
    for c in clips:
        c["tracks"] = [t for t in c["tracks"] if not t["target"].startswith("emitter:")]
        for t in c["tracks"]:
            if t["target"] == "operating:hoist":
                t["target"] = "operating:hoisting"
        if c["id"] == "lift-and-traverse":
            c["events"].insert(0, {"t": 0.3, "kind": "ignition", "title": tx("Hoist and slewing drives switched on", "Hub- und Drehwerk eingeschaltet"), "target": "winch-vent"})
            c["tracks"] += [{"target": "emitter:winch-vent", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 0.3, "v": 1}]},
                            {"target": "operating:slewing", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 9, "v": 0}, {"t": 11, "v": 0.3}, {"t": 17, "v": 0.3}, {"t": 19, "v": 0}, {"t": 22, "v": 0}]}]
        if c["id"] == "unload":
            for t in c["tracks"]:
                if t["target"] == "operating:hoisting":
                    t["keys"] = [{"t": 0, "v": 0.8}, {"t": 5, "v": 0.4}, {"t": 7, "v": 0.3}, {"t": 9, "v": 0.9}, {"t": 13, "v": 0.5}, {"t": 15, "v": 0.15}, {"t": 16, "v": 0}]
            c["events"].append({"t": 15.5, "kind": "shutdown", "title": tx("Drives switched off", "Antriebe ausgeschaltet"), "target": "winch-vent"})
            c["tracks"] += [{"target": "emitter:winch-vent", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 15.5, "v": 0}]},
                            {"target": "operating:slewing", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 16, "v": 0}]}]


def reps(a, old):
    R = {}
    O = {s["id"]: s for s in old}
    take = lambda ids: convert_v1_shapes([O[i] for i in ids])
    ln = lambda id, bone, d, region, z=5, pen=0.13, mat="steel-s355", regions=None, role="detail": path(id, bone, role, z, d, 0.3, False, region=region, regions=regions, material=mat, pen=pen)
    ropes = lambda role, pen: [ln("rope-%d-line" % i, "rope-%d" % i, "M 0,0 L 0,-5.75", "rope", 3, pen, "wire-rope", role=role) for i in range(1, 9)]
    ballast = rect("ballast-slab", "base", "body", 1, -3.5, 0, 7, 1, tone=0.45, region="ballast", material="concrete")
    foot = poly("mast-foot", "base", "body", 1.5, [(-1.5, 1), (1.5, 1), (1.15, 1.5), (-1.15, 1.5)], True, 0.8, True, region="lattice", material="steel-s355")
    cw = rect("counterweight-slabs", "counterweight", "body", 2, -1.2, 0, 2.4, 2.2, tone=0.4, region="counterweight", material="concrete")
    cab = rect("cab", "slewing", "body", 2, 1.4, 1.2, 1.9, 2.2, 0.1, tone=0.55, region="cab", material="machinery")
    glass = path("cab-glass", "slewing", "glazing", 3, "M 2.0,1.7 L 3.2,1.7 L 3.2,3.2 L 2.0,3.2 Z", 0.85, True, region="cab-glazing", material="glass")
    winch = lambda regs: rect("winch-house", "winch", "body", 2, -2, 0, 4, 1.8, tone=0.55, regions=regs, material="machinery")
    trolley = rect("trolley-frame", "trolley", "body", 2, -0.7, -0.55, 1.4, 0.55, tone=0.55, region="trolley", material="machinery")
    block = rect("block-body", "hoist-block", "body", 2, -0.28, -0.5, 0.56, 0.5, tone=0.45, region="skip", material="skip-steel")
    skip = path("skip-body", "skip", "body", 2, O["skip-body"]["d"], 0.45, True, region="skip", material="skip-steel")
    hook = path("hook", "hoist-block", "body", 2.5, O["hook"]["d"], 0.4, False, region="skip", material="skip-steel", pen=0.13)
    pend = lambda role, pen: [ln("pendant-back", "slewing", "M 0,8.6 L -15.6,5.1", "pendants", 4, pen, "wire-rope", role=role), ln("pendant-fore-1", "slewing", "M 0,8.6 L 22,5.2", "pendants", 4, pen, "wire-rope", role=role), ln("pendant-fore-2", "slewing", "M 0,8.6 L 50,5.2", "pendants", 4, pen, "wire-rope", role=role),
                              ln("slings", "skip", "M 0,1.0 L -0.7,0 M 0,1.0 L 0.7,0", "pendants", 4, pen, "wire-rope", role=role)]
    # 500: composed silhouettes
    sil = lambda id, bone, x, y, w, h: rect(id, bone, "body", 1.8, x, y, w, h, tone=0.8, region="lattice", material="steel-s355")
    e500 = [ballast, foot, sil("mast-silhouette", "mast", 0, -1, 39, 2), rect("slew-platform", "slewing", "body", 2, -2.4, 0, 5.8, 1.2, tone=0.55, regions=["house", "slew-drive"], material="machinery"),
            poly("tower-head", "slewing", "body", 1.8, [(-1.2, 1.2), (0, 8.6), (1.2, 1.2)], True, 0.8, True, region="lattice", material="steel-s355"),
            cab, glass, sil("jib-silhouette", "jib", 0, -0.8, 55, 1.6), sil("counter-silhouette", "counter-jib", 0, -0.7, 16.5, 1.4), cw, winch(["winch-house", "hoist-motor"]), trolley, block, skip] + ropes("body", 0.13) + pend("body", 0.13) + [hook]
    R["500"] = rep(500, *RANGES[500], PRINT_PENS[500], [], e500, decisions(
        tx("Composed silhouette: the lattice tower, the jib, the counter-jib and the tower head are closed, lightly toned outlines; machinery, cab, counterweight, ballast, trolley, hook block and skip are solid masses.", "Zusammengesetzte Silhouette: Gitterturm, Ausleger, Gegenausleger und Turmspitze sind geschlossene, hell getönte Umrisse; Maschinenhaus, Kabine, Gegengewicht, Ballast, Laufkatze, Hakenflasche und Kübel sind volle Massen."),
        tx("All moving parts: trolley, hook block, skip and the eight rope pieces keep their bones and move exactly as in the finer scales.", "Alle beweglichen Teile: Laufkatze, Hakenflasche, Kübel und die acht Seilstücke behalten ihre Knochen und bewegen sich wie in den feineren Maßstäben."),
        tx("The lattice (chords 0.1 m, diagonals 0.06 m = 0.2 and 0.12 mm) is not resolved as members: tower, jib and counter-jib are single toned silhouettes. Slew ring and platform, winch house and hoist motor merge into one shape each (area-weighted overlay).", "Das Gitter (Gurte 0,1 m, Diagonalen 0,06 m = 0,2 und 0,12 mm) wird nicht als Stäbe aufgelöst: Turm, Ausleger und Gegenausleger sind einzelne getönte Silhouetten. Drehkranz und Plattform, Windenhaus und Hubmotor gehen je in einer Form auf (flächengewichtete Überlagerung)."),
        tx("Diagonals, ties, ladder, sheaves, trolley wheels, ballast joints, louvres, cab details.", "Diagonalen, Querriegel, Leiter, Seilscheiben, Laufkatzenräder, Ballastfugen, Lamellen, Kabinendetails."),
        tx("Pendant ties, slings and ropes (0.02 m) are kept as lines at the minimum stroke of 0.13 mm (0.065 m), thicker than real, because they carry the structural reading of the crane; the hook is a line too. This exceeds the outline tolerance and is stated here.", "Abspannungen, Anschlagseile und Hubseile (0,02 m) bleiben als Linien mit dem Mindeststrich von 0,13 mm (0,065 m), dicker als real, weil sie die statische Lesart des Krans tragen; der Haken ist ebenfalls eine Linie. Das überschreitet die Konturtoleranz und wird hier ausgewiesen."),
        tx("None.", "Keine."), tx("Lattice silhouettes light, machinery and cab mid-tone, counterweight and ballast dark.", "Gittersilhouetten hell, Maschinen und Kabine mittlerer Ton, Gegengewicht und Ballast dunkel."),
        tx("The lattice regions colour the three toned silhouettes uniformly (area-weighted over members and gaps is not modelled); all other regions have their own shapes.", "Die Gitterregionen färben die drei getönten Silhouetten einheitlich (eine flächengewichtete Mittelung über Stäbe und Lücken wird nicht modelliert); alle anderen Regionen haben eigene Formen.")))
    # 200: individual members as strokes at real width
    mem = lambda id, bone, d, pen, z=1.8: ln(id, bone, d, "lattice", z, pen, role="body" if pen >= 0.3 else "detail")
    chords = lambda id, bone, x1, hh, pen: mem(id, bone, "M 0,%s L %s,%s M 0,%s L %s,%s" % (-hh, x1, -hh, hh, x1, hh), pen)
    e200 = [ballast, foot,
            chords("mast-chords", "mast", 39, 1, 0.5), mem("mast-diagonals", "mast", O["mast-diagonals"]["d"], 0.3), mem("mast-end-ties", "mast", "M 0,-1 L 0,1 M 39,-1 L 39,1", 0.3),
            rect("slew-platform", "slewing", "body", 2, -2.4, 0.5, 5.8, 0.7, tone=0.55, region="house", material="machinery"), rect("slew-ring", "slewing", "detail", 2.1, -1.4, 0, 2.8, 0.5, tone=0.35, region="slew-drive", material="machinery"),
            mem("tower-head-legs", "slewing", "M -1.2,1.2 L 0,8.6 L 1.2,1.2", 0.5), mem("tower-head-diagonals", "slewing", O["tower-head-diagonals"]["d"], 0.25), cab, glass,
            chords("jib-chords", "jib", 55, 0.8, 0.4), mem("jib-diagonals", "jib", O["jib-diagonals"]["d"], 0.3), mem("jib-end", "jib", "M 0,-0.8 L 0,0.8 M 55,-0.8 L 55,0.8", 0.3),
            chords("counter-chords", "counter-jib", 16.5, 0.7, 0.4), mem("counter-diagonals", "counter-jib", O["counter-diagonals"]["d"], 0.3), cw, winch(["winch-house"]),
            rect("winch-motor-grille", "winch", "detail", 2.1, -1.6, 0.4, 1.2, 0.8, tone=0.25, region="hoist-motor", material="machinery"), trolley, block, skip] + ropes("detail", 0.13) + pend("detail", 0.13) + [hook]
    R["200"] = rep(200, *RANGES[200], PRINT_PENS[200], [], e200, decisions(
        tx("Individual members: tower and jib are drawn as two chords with a zigzag of diagonals, the tower head as two legs with diagonals, at real member widths (strokes of 0.25 to 0.5 mm = 0.05 to 0.1 m).", "Einzelne Stäbe: Turm und Ausleger bestehen aus zwei Gurten mit einem Zickzack aus Diagonalen, die Turmspitze aus zwei Beinen mit Diagonalen, in realen Stabbreiten (Striche von 0,25 bis 0,5 mm = 0,05 bis 0,1 m)."),
        tx("Chords, diagonals, end ties, slew platform and ring, cab and glass, winch house with motor grille, counterweight, ballast, trolley, hook block, skip, ropes, pendants.", "Gurte, Diagonalen, Endriegel, Drehbühne und Drehkranz, Kabine und Glas, Windenhaus mit Motorgitter, Gegengewicht, Ballast, Laufkatze, Hakenflasche, Kübel, Seile, Abspannungen."),
        tx("The horizontal ties of the tower (every 2 m, 0.04 m) are omitted; slings merge with the skip region lines.", "Die Querriegel des Turms (alle 2 m, 0,04 m) entfallen; die Anschlagseile werden mit den Linien der Abspannungen zusammengefasst."),
        tx("Ladder, sheaves, trolley wheels, ballast joints, louvres.", "Leiter, Seilscheiben, Laufkatzenräder, Ballastfugen, Lamellen."),
        tx("Members at 0.25 to 0.5 mm are real width; ropes and pendants at 0.13 mm (0.026 m) approach the real 0.02 m.", "Stäbe mit 0,25 bis 0,5 mm entsprechen der realen Breite; Seile und Abspannungen mit 0,13 mm (0,026 m) kommen den realen 0,02 m nahe."),
        tx("None.", "Keine."), tx("Members dark, machinery mid-tone, counterweight and ballast dark, glass light.", "Stäbe dunkel, Maschinen mittlerer Ton, Gegengewicht und Ballast dunkel, Glas hell."),
        tx("The lattice region colours the member strokes; solid regions colour their fills.", "Die Gitterregion färbt die Stabstriche; die vollen Regionen färben ihre Flächen.")))
    # 100
    e100 = [s for s in e200 if s["id"] not in ("mast-chords", "mast-diagonals", "mast-end-ties", "jib-chords", "jib-diagonals", "jib-end", "counter-chords", "counter-diagonals", "tower-head-legs", "tower-head-diagonals")]
    e100 += [chords("mast-chords", "mast", 39, 1, 0.8), mem("mast-diagonals", "mast", O["mast-diagonals"]["d"], 0.5), mem("mast-horizontals", "mast", O["mast-horizontals"]["d"], 0.3),
             chords("jib-chords", "jib", 55, 0.8, 0.8), mem("jib-diagonals", "jib", O["jib-diagonals"]["d"], 0.5), mem("jib-end", "jib", "M 0,-0.8 L 0,0.8 M 55,-0.8 L 55,0.8", 0.4),
             mem("jib-rail", "jib", "M 0,-0.8 L 55,-0.8", 0.8, 1.9), chords("counter-chords", "counter-jib", 16.5, 0.7, 0.8), mem("counter-diagonals", "counter-jib", O["counter-diagonals"]["d"], 0.5),
             mem("tower-head-legs", "slewing", "M -1.2,1.2 L 0,8.6 L 1.2,1.2", 0.8), mem("tower-head-diagonals", "slewing", O["tower-head-diagonals"]["d"], 0.4), mem("tower-head-ties", "slewing", O["tower-head-ties"]["d"], 0.25)]
    e100 += take(["head-sheave", "jib-tip-sheave", "trolley-sheave", "block-sheave", "trolley-wheels", "skip-gate", "ballast-joints"])
    R["100"] = rep(100, *RANGES[100], PRINT_PENS[100], [], e100, decisions(
        tx("The 1:200 member drawing with thicker real-width members (0.8 mm chords, 0.5 mm diagonals), the horizontal ties and the tower-head ties added, sheaves and trolley wheels visible.", "Die Stabzeichnung von 1:200 mit dickeren Stäben in realer Breite (Gurte 0,8 mm, Diagonalen 0,5 mm), zusätzlich Querriegeln und Turmspitzenriegeln, Seilscheiben und Laufkatzenräder sichtbar."),
        tx("All members including the horizontals, rail, sheaves, trolley wheels, skip gate and ballast joints.", "Alle Stäbe einschließlich der Querriegel, Schiene, Seilscheiben, Laufkatzenräder, Kübelklappe und Ballastfugen."),
        tx("Ladder merges into the tower ties.", "Die Leiter geht in den Turmriegeln auf."), tx("Ladder, louvres.", "Leiter, Lamellen."),
        tx("Members at real width; ropes at 0.13 mm (0.013 m real 0.02 m).", "Stäbe in realer Breite; Seile mit 0,13 mm (0,013 m, real 0,02 m)."), tx("None.", "Keine."), tx("As 1:200.", "Wie 1:200."), tx("As 1:200.", "Wie 1:200.")))
    e50 = convert_v1_shapes(old)
    for s_ in e50:
        if s_["id"] == "slew-ring":
            s_["region"] = "slew-drive"
    R["50"] = rep(50, *RANGES[50], PRINT_PENS[50], [], e50, decisions(
        tx("Individual members as authored in the first version: chords as outlines, diagonals, horizontals, ties and ladder.", "Einzelne Stäbe wie in der ersten Fassung: Gurte als Umrisse, Diagonalen, Querriegel, Riegel und Leiter."),
        tx("Everything including ladder, louvres and ballast joints.", "Alles einschließlich Leiter, Lamellen und Ballastfugen."), tx("Nothing merged.", "Nichts zusammengefasst."), tx("Cables of the ladder cage (not modelled).", "Absturzsicherung der Leiter (nicht modelliert)."),
        tx("Ropes 0.02 m = 0.4 mm at the detail pen 0.25 mm.", "Seile 0,02 m = 0,4 mm mit Detailstift 0,25 mm."), tx("None.", "Keine."), tx("Three line weights 0.35 / 0.25 / 0.13 mm.", "Drei Linienstärken 0,35 / 0,25 / 0,13 mm."), tx("As 1:200.", "Wie 1:200.")))
    return R


SYMBOLS = [{
    "id": "site-plan-reach", "title": tx("Site-plan crane reach symbol", "Baustellenplan-Kransymbol mit Reichweite"),
    "description": tx("Plan symbol for a site plan: the mast square, the jib and counter-jib lines and the circle swept by the trolley at maximum reach.", "Planzeichen für den Baustelleneinrichtungsplan: Turmquadrat, Ausleger- und Gegenauslegerlinie und der vom Katzfahrwerk bei größter Ausladung überstrichene Kreis."),
    "semantics": "true-extent", "units": "model-m", "denominators": [500, 200], "pens": pens(0.25, 0.18, None, None, 0.25), "hatches": [],
    "geometry": {"elevation": [], "plan": [
        {"kind": "rect", "x": -1, "y": -1, "w": 2, "h": 2, **{"id": "mast-square", "bone": "base", "role": "body", "z": 2, "filled": True, "tone": 0.45}},
        {"kind": "poly", "points": [[-16.5, 0], [55, 0]], "closed": False, **{"id": "jib-line", "bone": "base", "role": "body", "z": 3, "filled": False, "tone": 0.3}},
        {"kind": "ellipse", "cx": 0, "cy": 0, "rx": 50, "ry": 50, **{"id": "trolley-reach-circle", "bone": "base", "role": "detail", "z": 1, "filled": False, "tone": 0.3}}]},
    "dimensionalNote": tx("True extent in model metres: mast 2 x 2 m, jib 55 m, counter-jib 16.5 m, circle of the maximum trolley position 50 m from the mast axis; the pictogram is drawn at real size at every scale it lists.", "Wahre Ausdehnung in Modellmetern: Turm 2 x 2 m, Ausleger 55 m, Gegenausleger 16,5 m, Kreis der größten Katzposition 50 m von der Turmachse; das Piktogramm ist in jedem gelisteten Maßstab in Realgröße gezeichnet.")}]

NOTES = tx("Hammerhead tower crane, 55 m jib. The lattice is drawn as composed silhouettes at 1:500 and as individual members at real width from 1:200 (chords 0.1 m, diagonals 0.06 m: 0.2 and 0.12 mm at 1:500, below the printable feature). Slewing is a vertical-axis motion and not an elevation joint. Electrical drives: no exhaust; waste heat leaves through the winch house skin, hoist motor surfaces, slewing drive housings and the ventilation outlet. Cab conditioned to 21 C.",
           "Obendreher-Turmkran, 55 m Ausleger. Das Gitter ist bei 1:500 als zusammengesetzte Silhouette gezeichnet und ab 1:200 als Einzelstäbe in realer Breite (Gurte 0,1 m, Diagonalen 0,06 m: 0,2 und 0,12 mm bei 1:500, unter dem druckbaren Merkmal). Das Schwenken ist eine Bewegung um die Hochachse und kein Gelenk der Ansicht. Elektrische Antriebe: kein Abgas; Abwärme verlässt den Kran über die Außenhaut des Windenhauses, die Hubmotoroberflächen, die Drehwerksgehäuse und den Lüftungsauslass. Kabine auf 21 C konditioniert.")
ASSUMPTIONS = [
    tx("Heat release at level 1: winch house skin 5.7 kW (about +15 K; a 55 kW winch loses about 8 %), hoist motor surfaces 1.6 kW (+45 K), slewing drives 0.45 kW. Heat capacities are active shell masses: sheet steel 3 mm 11 kJ/(m2 K), lattice members 30 kJ/(m2 K), concrete 0.15 m 300 kJ/(m2 K).",
       "Wärmefreisetzung bei Stufe 1: Außenhaut des Windenhauses 5,7 kW (etwa +15 K; eine 55-kW-Winde verliert etwa 8 %), Hubmotoroberflächen 1,6 kW (+45 K), Drehwerke 0,45 kW. Wärmekapazitäten sind aktive Schalenmassen: Stahlblech 3 mm 11 kJ/(m2 K), Gitterstäbe 30 kJ/(m2 K), Beton 0,15 m 300 kJ/(m2 K)."),
    tx("Winch house ventilation at level 1: 0.6 m3/s through a 0.4 m outlet, 25 K above ambient, dry; flow and temperature rise scale with the hoisting level.", "Lüftung des Windenhauses bei Stufe 1: 0,6 m3/s durch einen 0,4-m-Auslass, 25 K über der Umgebung, trocken; Volumenstrom und Temperaturerhöhung skalieren mit der Hubstufe."),
    tx("The lattice is a cylinder-like member array with sky view 0.9; 15 % of the ballast area rests on ground at 25 C.", "Das Gitter ist eine zylinderartige Stabanordnung mit Himmelssichtfaktor 0,9; 15 % der Ballastfläche liegen auf 25 C warmem Boden.")]
SPEC = dict(sky=8, ground=25, operating=KEYS, regions=REGIONS, emitters=EMITTERS, assumptions=ASSUMPTIONS, defaults=DEFAULTS, contacts=CONTACTS, poses=POSES, fix_clips=fix_clips, reps=reps, notes=NOTES, category="machinery", symbols=SYMBOLS)
if __name__ == "__main__":
    a = assemble("tower-crane", REL, SPEC)
    if len(sys.argv) > 1:
        previews(a, "crane", poses=sys.argv[2:] or None, dens=[int(x) for x in sys.argv[1].split(",")], zoom=(1,), print_ppm=int(sys.argv[3]) if len(sys.argv) > 3 else 24)
