from author_15_lib import *
OAK, STEEL = "oak-timber", "steel-slides"
HALF = (("l", "top-left", 0.0, 0.1, 0.8), ("r", "top-right", 0.8, 0.8, 1.5))
SUP = [("leg-left-l", 0.135, 0.375), ("leg-left-r", 0.135, -0.375), ("leg-right-l", 0.665 + 0.8, 0.375), ("leg-right-r", 0.665 + 0.8, -0.375)]

def legs(c, secs, lo=None):
    for side, x0, bone in (("l", 0.135, "top-left"), ("r", 0.665 + 0.8, "top-right")):
        c.loft("leg-" + side, bone, OAK, "body", 0.6, [([x0, y, 0.375], rx, ry, 8) for y, rx, ry in secs], X, Z, repeat={"count": 2, "translation": [0, 0, -0.75]})

def thermal_refs(c, table):
    c.to("thermal")
    for region, items in table.items():
        for sid in items:
            c.ref("th-%s-%s" % (region, sid), sid, region=region)

def supports(c, pad, fx, fz):
    c.to("structure")
    for sid, x, z in SUP:
        bone = "top-left" if "left" in sid else "top-right"
        c.box("support-" + sid, bone, OAK, "ground", 0.5, [x, pad / 2, z], [fx, pad, fz], member=bone, part="support", anchor=sid)

def s50():
    c = Ctx("dining-table", 50)
    c.to("body")
    c.ext("top-l", "top-left", OAK, "body", 0.7, [[0.0, -0.442], [0.008, -0.45], [0.8, -0.45], [0.8, 0.45], [0.008, 0.45], [0.0, 0.442]], "xz", 0.72, 0.75)
    c.ext("top-r", "top-right", OAK, "body", 0.7, [[0.8, -0.45], [1.592, -0.45], [1.6, -0.442], [1.6, 0.442], [1.592, 0.45], [0.8, 0.45]], "xz", 0.72, 0.75)
    c.box("apron-l-side", "top-left", OAK, "body", 0.55, [0.45, 0.68, 0.37], [0.7, 0.08, 0.02], repeat={"count": 2, "translation": [0, 0, -0.74]})
    c.box("apron-l-end", "top-left", OAK, "body", 0.55, [0.11, 0.68, 0], [0.02, 0.08, 0.72], repeat={"count": 2, "translation": [0.68, 0, 0]})
    c.box("apron-r-side", "top-right", OAK, "body", 0.55, [1.15, 0.68, 0.37], [0.7, 0.08, 0.02], repeat={"count": 2, "translation": [0, 0, -0.74]})
    c.box("apron-r-end", "top-right", OAK, "body", 0.55, [0.81, 0.68, 0], [0.02, 0.08, 0.72], repeat={"count": 2, "translation": [0.68, 0, 0]})
    legs(c, [(0, 0.0225, 0.025), (0.56, 0.035, 0.035), (0.64, 0.035, 0.035)])
    c.bw("corner-l", "top-left", OAK, "body", 0.55, (0.1, 0.17), (0.64, 0.72), (0.34, 0.38), repeat={"count": 2, "translation": [0, 0, -0.72]})
    c.bw("corner-r", "top-right", OAK, "body", 0.55, (1.43, 1.5), (0.64, 0.72), (0.34, 0.38), repeat={"count": 2, "translation": [0, 0, -0.72]})
    c.bw("leaf", "leaf", OAK, "body", 0.65, (0.5, 1.1), (0.69, 0.72), (-0.44, 0.44), corner=0.004)
    c.bw("rail", "frame", STEEL, "body", 0.35, (0.3, 1.3), (0.67, 0.69), (0.2962, 0.3038), repeat={"count": 2, "translation": [0, 0, -0.6]})
    c.bw("cleat-l", "top-left", STEEL, "body", 0.35, (0.35, 0.45), (0.66, 0.72), (0.2962, 0.3038), repeat={"count": 2, "translation": [0, 0, -0.6]})
    c.bw("cleat-r", "top-right", STEEL, "body", 0.35, (1.15, 1.25), (0.66, 0.72), (0.2962, 0.3038), repeat={"count": 2, "translation": [0, 0, -0.6]})
    thermal_refs(c, {"table-top": ["top-l", "top-r"], "leaf-top": ["leaf"], "apron": ["apron-l-side", "apron-l-end", "apron-r-side", "apron-r-end"], "legs": ["leg-l", "leg-r", "corner-l", "corner-r"], "slide-rails": ["rail", "cleat-l", "cleat-r"]})
    c.to("structure")
    for sid, m in (("top-l", "top-left"), ("apron-l-side", "top-left"), ("apron-l-end", "top-left"), ("top-r", "top-right"), ("apron-r-side", "top-right"), ("apron-r-end", "top-right"), ("leaf", "leaf"), ("rail", "frame"), ("leg-l", "ground"), ("leg-r", "ground"), ("corner-l", "ground"), ("corner-r", "ground")):
        c.ref("m-" + sid, sid, member=m, part="member")
    c.ref("j-cleat-l", "cleat-l", member="top-left", part="joint")
    c.ref("j-cleat-r", "cleat-r", member="top-right", part="joint")
    supports(c, 0.008, 0.05, 0.045)
    c.dec("Each half is a 30 mm oak plate with chamfered outer edges on a rectangular 20 mm apron frame, held by four legs with a tapered 560 mm shaft and a square 70 mm shoulder block; the leaf is a rounded 30 mm plank stored on two 7.5 mm steel runners under the plates; four steel cleats bridge runner and plate underside and are the prismatic joints.", "Jede Hälfte ist eine 30 mm Eichenplatte mit gefasten Aussenkanten auf einem rechteckigen 20 mm Zargenrahmen, getragen von vier Beinen mit konischem 560 mm Schaft und quadratischem 70 mm Schulterblock; die Einlegeplatte ist eine gerundete 30 mm Bohle auf zwei 7,5 mm Stahlschienen unter den Platten; vier Stahlwinkel verbinden Schiene und Plattenunterseite und bilden die Schiebegelenke.")
    c.dec("Thermal: every region is a reference to the body solids it names (single-layer regions are one surface solid each, B2); no hidden regions, no emitters. Structure: plates and aprons are the bending members of their half, the legs are the columns carried by the ground bone, the runners are the frame member, the cleats are the joints, and four 8 mm contact pads sit under the legs.", "Thermik: jede Region verweist auf die Körper, die sie nennt (einschichtige Regionen sind je ein Oberflächenkörper, B2); keine verdeckten Regionen, keine Emitter. Tragwerk: Platten und Zargen sind die Biegeglieder ihrer Hälfte, die Beine sind die Stützen des Bodenbezugs, die Schienen das Rahmenglied, die Winkel die Gelenke, vier 8 mm Auflager unter den Beinen.")
    return c.out()

def s100():
    c = Ctx("dining-table", 100)
    c.to("body")
    c.bw("top-l", "top-left", OAK, "body", 0.7, (0, 0.8), (0.72, 0.75), (-0.45, 0.45))
    c.bw("top-r", "top-right", OAK, "body", 0.7, (0.8, 1.6), (0.72, 0.75), (-0.45, 0.45))
    for side, bone, a, b in (("l", "top-left", 0.1, 0.8), ("r", "top-right", 0.8, 1.5)):
        c.ext("apron-" + side, bone, OAK, "body", 0.55, [[a, -0.38], [b, -0.38], [b, 0.38], [a, 0.38]], "xz", 0.64, 0.72, holes=[[[a + 0.02, -0.36], [b - 0.02, -0.36], [b - 0.02, 0.36], [a + 0.02, 0.36]]])
    legs(c, [(0, 0.0225, 0.025), (0.64, 0.035, 0.035)])
    c.bw("leaf", "leaf", OAK, "body", 0.65, (0.5, 1.1), (0.69, 0.72), (-0.44, 0.44))
    c.bw("rail", "frame", STEEL, "body", 0.35, (0.3, 1.3), (0.665, 0.69), (0.2925, 0.3075), repeat={"count": 2, "translation": [0, 0, -0.6]})
    thermal_refs(c, {"table-top": ["top-l", "top-r"], "leaf-top": ["leaf"], "apron": ["apron-l", "apron-r"], "legs": ["leg-l", "leg-r"], "slide-rails": ["rail"]})
    c.to("structure")
    for sid, m in (("top-l", "top-left"), ("apron-l", "top-left"), ("top-r", "top-right"), ("apron-r", "top-right"), ("leaf", "leaf"), ("rail", "frame"), ("leg-l", "ground"), ("leg-r", "ground")):
        c.ref("m-" + sid, sid, member=m, part="member")
    c.bw("j-slide-l", "top-left", STEEL, "detail", 0.35, (0.35, 0.45), (0.665, 0.72), (0.2925, 0.3075), member="top-left", part="joint", repeat={"count": 2, "translation": [0, 0, -0.6]})
    c.bw("j-slide-r", "top-right", STEEL, "detail", 0.35, (1.15, 1.25), (0.665, 0.72), (0.2925, 0.3075), member="top-right", part="joint", repeat={"count": 2, "translation": [0, 0, -0.6]})
    supports(c, 0.015, 0.05, 0.05)
    c.dec("1:100 merges the four apron boards of each half into one 20 mm hollow frame, drops the chamfers and the leg shoulder step (the leg is one taper to the apron bottom plus the square block under the plate), and widens the runners to the 15 mm feature; plate, leaf and legs keep their real positions and the same proportions as 1:50.", "1:100 fasst die vier Zargenbretter jeder Hälfte zu einem 20 mm Hohlrahmen zusammen, lässt die Fasen und die Schulterstufe der Beine weg (das Bein ist ein Konus bis zur Zargenunterkante plus Vierkantblock unter der Platte) und verbreitert die Schienen auf das 15 mm Mindestmass; Platte, Einlegeplatte und Beine behalten ihre wahren Lagen und die Proportionen von 1:50.")
    c.dec("Thermal and structure reference the body solids; the cleats are merged into the runners in the body and kept as 15 mm joint solids in the structure list, hidden inside the apron zone.", "Thermik und Tragwerk verweisen auf die Körper; die Winkel sind im Körper in die Schienen eingegangen und bleiben als 15 mm Gelenkkörper in der Tragwerksliste, verborgen in der Zargenzone.")
    return c.out()

def s200():
    c = Ctx("dining-table", 200)
    c.to("body")
    c.bw("top-l", "top-left", OAK, "body", 0.7, (0, 0.8), (0.72, 0.75), (-0.45, 0.45))
    c.bw("top-r", "top-right", OAK, "body", 0.7, (0.8, 1.6), (0.72, 0.75), (-0.45, 0.45))
    for side, bone, a, b in (("l", "top-left", 0.05, 0.8), ("r", "top-right", 0.8, 1.55)):
        c.ext("apron-" + side, bone, OAK, "body", 0.55, [[a, -0.45], [b, -0.45], [b, 0.45], [a, 0.45]], "xz", 0.64, 0.72, holes=[[[a + 0.03, -0.42], [b - 0.03, -0.42], [b - 0.03, 0.42], [a + 0.03, 0.42]]])
    for side, x0, bone in (("l", 0.135, "top-left"), ("r", 1.465, "top-right")):
        c.bw("leg-" + side, bone, OAK, "body", 0.6, (x0 - 0.03, x0 + 0.03), (0, 0.64), (0.345, 0.405), repeat={"count": 2, "translation": [0, 0, -0.75]})
    c.bw("leaf-bar", "leaf", OAK, "body", 0.6, (0.5, 1.1), (0.69, 0.72), (-0.44, 0.44))
    c.bw("rail", "frame", STEEL, "body", 0.35, (0.3, 1.3), (0.63, 0.69), (-0.015, 0.015))
    thermal_refs(c, {"table-top": ["top-l", "top-r"], "leaf-top": ["leaf-bar"], "apron": ["apron-l", "apron-r"], "legs": ["leg-l", "leg-r"], "slide-rails": ["rail"]})
    c.to("structure")
    for sid, m in (("top-l", "top-left"), ("apron-l", "top-left"), ("top-r", "top-right"), ("apron-r", "top-right"), ("leaf-bar", "leaf"), ("rail", "frame"), ("leg-l", "ground"), ("leg-r", "ground")):
        c.ref("m-" + sid, sid, member=m, part="member")
    supports(c, 0.03, 0.05, 0.05)
    c.dec("1:200 draws each half as plate plus a 30 mm perimeter apron flush with the plate edge, the two runners are one 30 mm centre beam, and the legs are plain two-section tapers.", "1:200 zeichnet jede Hälfte als Platte mit einer 30 mm Randzarge bündig zur Plattenkante, die zwei Schienen sind ein 30 mm Mittelbalken, die Beine einfache Zweischnitt-Konen.")
    return c.out()

def s500():
    c = Ctx("dining-table", 500)
    c.to("body")
    c.bw("slab-l", "top-left", OAK, "body", 0.7, (0, 0.8), (0.64, 0.75), (-0.45, 0.45))
    c.bw("slab-r", "top-right", OAK, "body", 0.7, (0.8, 1.6), (0.64, 0.75), (-0.45, 0.45))
    c.bw("leaf-bar", "leaf", OAK, "body", 0.6, (0.5, 1.1), (0.63, 0.735), (-0.44, 0.44))
    for side, x0, bone in (("l", 0.135, "top-left"), ("r", 1.465, "top-right")):
        c.bw("leg-" + side, bone, OAK, "body", 0.6, (x0 - 0.03, x0 + 0.03), (0, 0.64), (0.345, 0.405), repeat={"count": 2, "translation": [0, 0, -0.75]})
    c.to("thermal")
    for sid in ("slab-l", "slab-r"):
        c.ref("th-top-" + sid, sid, region="table-top")
    for side, bone, a, b in (("l", "top-left", 0.05, 0.8), ("r", "top-right", 0.8, 1.55)):
        c.ext("th-apron-" + side, bone, OAK, "body", 0.55, [[a, -0.45], [b, -0.45], [b, 0.45], [a, 0.45]], "xz", 0.65, 0.71, holes=[[[a + 0.03, -0.42], [b - 0.03, -0.42], [b - 0.03, 0.42], [a + 0.03, 0.42]]], region="apron")
    c.ref("th-leaf-bar", "leaf-bar", region="leaf-top")
    for sid in ("leg-l", "leg-r"): c.ref("th-legs-" + sid, sid, region="legs")
    c.to("thermal")
    c.bw("th-slide-rails", "frame", STEEL, "detail", 0.35, (0.3, 1.3), (0.665, 0.74), (-0.0375, 0.0375), region="slide-rails")
    c.to("structure")
    for sid, m in (("slab-l", "top-left"), ("slab-r", "top-right"), ("leaf-bar", "leaf"), ("leg-l", "ground"), ("leg-r", "ground")):
        c.ref("m-" + sid, sid, member=m, part="member")
    c.bw("m-rail", "frame", STEEL, "detail", 0.35, (0.3, 1.3), (0.665, 0.74), (-0.0375, 0.0375), member="frame", part="member")
    supports(c, 0.075, 0.075, 0.075)
    c.dec("Minimal: 1:500 draws each half as one solid 110 mm slab (plate and apron merged, as drawn), the leaf as a bar inside the slab zone, and four 60 mm legs (the mean of the 45 to 70 mm taper, drawn as filled 60 mm outlines so C13 allows them); the 75 mm smallest feature forbids a 30 mm plate or 20 mm apron boards, so the slab is solid and heavier than the real table. The thermal list counts the slab surface once, under table-top (top, underside and edge: the plate and apron are merged here); the apron region keeps its coverage as a 30 mm wall ring buried inside the slab, so it adds no second surface.", "Minimal: 1:500 zeichnet jede Hälfte als einen 110 mm Plattenkörper (Platte und Zarge verschmolzen, wie gezeichnet), die Einlegeplatte als Balken in der Plattenzone und vier 60 mm Beine (der Mittelwert des 45 bis 70 mm Konus, als gefüllte 60 mm Umrisse gezeichnet, damit C13 sie erlaubt); das 75 mm Mindestmass verbietet eine 30 mm Platte oder 20 mm Zargenbretter, daher ist der Körper massiv und schwerer als der echte Tisch. Die Thermikliste zählt die Plattenfläche einmal unter Tischplatte (Oberseite, Unterseite und Kante: Platte und Zarge sind hier verschmolzen); die Zargenregion behält ihre Abdeckung als 30 mm Wandring, vergraben im Plattenkörper, und fügt so keine zweite Fläche hinzu.")
    c.dec("The two runners are merged into one 75 mm centre beam hidden inside the slab, existing only as thermal and structure solid; supports are 75 mm contact blocks under the legs.", "Die zwei Schienen sind zu einem 75 mm Mittelbalken verborgen im Plattenkörper verschmolzen und bestehen nur als Thermik- und Tragwerkskörper; Auflager sind 75 mm Kontaktblöcke unter den Beinen.")
    return c.out()

if __name__ == "__main__":
    save("dining-table", {50: s50(), 100: s100(), 200: s200(), 500: s500()})
