"""🪑 Round 3 models of `furniture/folding-deck-chair`, one hand-stated list of parts per scale (1:50, 1:100, 1:200, 1:500)."""
from r3_folding_deck_chair_parts import *

L = STRUT_LEN
TIMBER = (0.55, 0.55, 0.55)
CROSS = (0.45, 0.45, 0.45)
STEEL = (0.2, 0.2, 0.2)
MASS = {"back": 2.07, "seat": 1.72, "strut": 0.82, "canvas-a": 0.09, "canvas-a2": 0.08, "canvas-b2": 0.035, "canvas-b": 0.035}
PANEL = {"canvas-a": ("canvas-back", R_BACK, (2, 2, 2.3)), "canvas-a2": ("canvas-back-low", R_BACK, (2, 2.1, 2.2)), "canvas-b2": ("canvas-seat-low", R_SEAT, (2, 2.4, 2.1)), "canvas-b": ("canvas-seat", R_SEAT, (2, 2.5, 2))}
COM = {"back": 0.30, "seat": 0.52, "strut": 0.29}
FEET = (("foot-rear-left", "seat", 1), ("foot-rear-right", "seat", -1), ("foot-front-left", "back", 1), ("foot-front-right", "back", -1))


def mass_length(bone):
    return rnd(0.03 * (MASS[bone] / 2.0) ** (1 / 3), 4)


def sling(sc, thickness, segments, half, stripes, tones, tone2d, lead, lap):
    """🏖️ The four canvas panels of the hanging sling, one per canvas bone, run on by `lead` into the rail and by `lap` over every joint so the sling stays closed in every pose."""
    for bone, n in zip(SLING_BONES, segments):
        ident, region, zs = PANEL[bone]
        first = lead if bone in ("canvas-a", "canvas-b") else lap
        sc.sheet(ident, bone, band(sling_curve(bone, n), thickness, first, lap), half, stripes, region, zs, tones, tone2d)


def structure_common(sc, z_seat, z_back, z_strut, canvas_half, pad, joint_r, rod_r, sling_t, sling_n, mass_v, mass_w, strut_v=0.016):
    for bone, n in zip(SLING_BONES, sling_n):
        sc.own("m-" + bone, bone, CA, {"kind": "extrude", "profile": pts(band(sling_curve(bone, n), sling_t)), "plane": "xy", "from": rnd(-canvas_half), "to": rnd(canvas_half)}, bone, "member")
    sc.sleeve_joint("j-rear-foot", "seat", (END_R, 0.0), joint_r, (z_seat[0] - 0.004, z_seat[1] + 0.004), "seat")
    sc.sleeve_joint("j-notch", "seat", (NOTCHES[2], BAR_R), joint_r, (z_seat[0] - 0.004, z_seat[1] + 0.004), "strut")
    sc.rod_joint("j-canvas-top", "canvas-a", (0.0, 0.0), rod_r, canvas_half, "canvas-a")
    sc.rod_joint("j-canvas-back", "canvas-a2", (0.0, 0.0), rod_r, canvas_half, "canvas-a2")
    sc.rod_joint("j-canvas-sag", "canvas-a2", (CHORD[1], 0.0), rod_r, canvas_half, "canvas-a2")
    sc.rod_joint("j-canvas-seat", "canvas-b2", (0.0, 0.0), rod_r, canvas_half, "canvas-b2")
    sc.rod_joint("j-canvas-front", "canvas-b", (0.0, 0.0), rod_r, canvas_half, "canvas-b")
    for bone, span in (("back", z_back), ("seat", z_seat), ("strut", z_strut)):
        sc.mass("mass-" + bone, bone, (COM[bone], 0.0), (max(mass_length(bone), mass_w), mass_v if bone != "strut" else strut_v, mass_w), span)
    for anchor, bone, side in FEET:
        z = (z_seat if bone == "seat" else z_back)
        sc.support(anchor, bone, side * (z[0] + z[1]) / 2, *pad)


def scale_50():
    sc = Scale(50, 0.25, 0.13, 0.13)
    sc.prism("back-rail", "back", stadium(BACK_FOOT, BACK_TOP, END_R, 10), pairs(Z_BACK), BE, 0.55, R_TIMBER, (4, 4, 4), TIMBER, {"kind": "capsule", "x0": BACK_FOOT + END_R, "y0": 0.0, "x1": BACK_TOP - END_R, "y1": 0.0, "r0": END_R, "r1": END_R}, member="back")
    sc.prism("top-rail", "back", rounded_rect(TOP_RAIL_U, 0.0, CROSS_HALF_U, CROSS_HALF_V, CROSS_R), [(-0.253, 0.253)], BE, 0.5, R_TIMBER, (2.5, 2.2, 5.6), CROSS, {"kind": "rect", "x": TOP_RAIL_U - CROSS_HALF_U, "y": -CROSS_HALF_V, "w": 0.04, "h": 0.02, "r": CROSS_R}, small=True, member="back")
    sc.prism("foot-rail", "back", rounded_rect(FOOT_RAIL_U, 0.03, CROSS_HALF_U, CROSS_HALF_V, CROSS_R), [(-0.268, 0.268)], BE, 0.5, R_TIMBER, (2.6, 2.8, 1.6), CROSS, {"kind": "rect", "x": FOOT_RAIL_U - CROSS_HALF_U, "y": 0.02, "w": 0.04, "h": 0.02, "r": CROSS_R}, small=True, member="back")
    sc.prism("seat-rail", "seat", notched_rail(8, True), pairs(Z_SEAT), BE, 0.55, R_TIMBER, (3, 3.5, 3), TIMBER, member="seat")
    sc.prism("front-rail", "seat", rounded_rect(FRONT_RAIL_U, 0.0, CROSS_HALF_U, CROSS_HALF_V, CROSS_R), [(-0.231, 0.231)], BE, 0.5, R_TIMBER, (2.5, 2.9, 5.2), CROSS, {"kind": "rect", "x": FRONT_RAIL_U - CROSS_HALF_U, "y": -CROSS_HALF_V, "w": 0.04, "h": 0.02, "r": CROSS_R}, small=True, member="seat")
    sc.prism("rear-rail", "seat", rounded_rect(REAR_RAIL_U, 0.0, CROSS_HALF_U, CROSS_HALF_V, CROSS_R), [(-0.231, 0.231)], BE, 0.5, R_TIMBER, (2.5, 1.5, 1.4), CROSS, {"kind": "rect", "x": REAR_RAIL_U - CROSS_HALF_U, "y": -CROSS_HALF_V, "w": 0.04, "h": 0.02, "r": CROSS_R}, small=True, member="seat")
    sc.prism("strut-arm", "strut", stadium(-STRUT_H / 2, L + STRUT_H / 2, STRUT_H / 2, 8), pairs(Z_STRUT), BE, 0.55, R_TIMBER, (5, 4.5, 5), TIMBER, {"kind": "capsule", "x0": 0.0, "y0": 0.0, "x1": rnd(L), "y1": 0.0, "r0": STRUT_H / 2, "r1": STRUT_H / 2}, member="strut")
    g, span = sc.turned("strut-bar", "strut", (L, 0.0), [(0.0, 0.0095), (0.0015, BAR_R), (0.5785, BAR_R), (0.58, 0.0095)], -0.29, 1, BE, 0.45, R_TIMBER, None, None, BAR_R, member="strut")
    sc.drawn("strut-bar", "strut", g, [span], BE, R_TIMBER, (6, 1.4, 1.5), CROSS, small=True)
    pivot = [(0.0, 0.0045), (0.0015, 0.007), (0.0015, 0.003), (0.0215, 0.003), (0.0215, 0.009), (0.0235, 0.009), (0.0235, 0.003), (0.0435, 0.003), (0.0435, 0.008), (0.045, 0.008), (0.045, 0.0055), (0.05, 0.0055), (0.053, 0.004), (0.054, 0.002)]
    prop = [(0.0, 0.002), (0.001, 0.004), (0.003, 0.0055), (0.007, 0.0055), (0.007, 0.008), (0.0085, 0.008), (0.0085, 0.003), (0.0285, 0.003), (0.0285, 0.009), (0.0305, 0.009), (0.0305, 0.003), (0.0505, 0.003), (0.0505, 0.007), (0.052, 0.007)]
    for side, name in ((1, "l"), (-1, "r")):
        g, span = sc.turned("pivot-bolt-" + name, "back", (0.0, 0.0), pivot, side * 0.2695, -side, SH, 0.2, R_TIMBER, None, None, 0.007, member="back", part="joint")
        sc.drawn("pivot-bolt-" + name, "back", g, [span], SH, R_TIMBER, (6, 6, 6), STEEL, small=True, elevation=side < 0)
        g, span = sc.turned("prop-bolt-" + name, "strut", (0.0, 0.0), prop, side * 0.2985, -side, SH, 0.2, R_TIMBER, None, None, 0.008, member="strut", part="joint")
        sc.drawn("prop-bolt-" + name, "strut", g, [span], SH, R_TIMBER, (6.5, 6, 6), STEEL, small=True, elevation=side < 0)
    sling(sc, 0.0075, (9, 6, 5, 5), Z_CANVAS, 9, (0.85, 0.5), (0.3, 0.85, 0.85), 0.006, 0.004)
    sc.sheet("canvas-top-lap", "back", sleeve(TOP_RAIL_U, 0.0, CROSS_HALF_U, CROSS_HALF_V, CROSS_R, 0.0075), Z_CANVAS, 9, R_BACK, (2.2, 2.3, 5.8), (0.85, 0.5), (0.3, 0.85, 0.85))
    sc.sheet("canvas-front-lap", "seat", sleeve(FRONT_RAIL_U, 0.0, CROSS_HALF_U, CROSS_HALF_V, CROSS_R, 0.0075), Z_CANVAS, 9, R_SEAT, (2.2, 3.0, 5.4), (0.85, 0.5), (0.3, 0.85, 0.85))
    structure_common(sc, Z_SEAT, Z_BACK, Z_STRUT, Z_CANVAS, (0.03, 0.008, 0.02), 0.014, 0.005, 0.0075, (9, 6, 5, 5), 0.02, 0.012)
    sc.decide(
        "1:50 is the joiner's model of the real chair: three beech frames of 20 x 40 mm rails on edge with half-round ends, nested sideways with 2 mm washer gaps (prop arms outermost at 270 to 290 mm, back frame 248 to 268 mm, seat frame 226 to 246 mm). The back frame runs from the front feet to the head rail, the seat frame from the rear feet to the knee rail and carries four round-bottomed notches of 24 mm in its upper edge; the 32 mm prop arms hang from the back frame and their 22 mm dowel sits in the third notch. The cross rails are 40 x 20 mm with 4 mm arrises, tenoned 5 mm into the rails; the foot rail of the back frame is planted on the rear edges of the legs so the seat frame can fold through. Four turned bolts (low heads, washers in the gaps, dome nuts) are the pivots.",
        "1:50 ist das Schreinermodell des echten Stuhls: drei Buchenrahmen aus hochkant stehenden Holmen 20 x 40 mm mit halbrunden Enden, seitlich mit 2 mm Scheibenfugen ineinander gesteckt (Stützarme aussen bei 270 bis 290 mm, Rückenrahmen 248 bis 268 mm, Sitzrahmen 226 bis 246 mm). Der Rückenrahmen läuft von den Vorderfüssen zur Kopfleiste, der Sitzrahmen von den Hinterfüssen zur Knieleiste und trägt vier rund ausgestochene Rasten von 24 mm in der Oberkante; die 32 mm Stützarme hängen am Rückenrahmen, ihr 22 mm Rundstab liegt in der dritten Raste. Die Querleisten messen 40 x 20 mm mit 4 mm gebrochenen Kanten und sind 5 mm in die Holme gezapft; die Fussleiste des Rückenrahmens sitzt auf den Hinterkanten der Beine, damit der Sitzrahmen durchklappen kann. Vier gedrehte Schrauben (flache Köpfe, Scheiben in den Fugen, Hutmuttern) sind die Gelenke.",
    )
    sc.decide(
        "The sling is the unloaded canvas as it really hangs: a catenary of 1.28 m between the head rail and the knee rail, carried by four canvas bones over equal forward steps (two back panels, two seat panels, 18 mm of sag under each chord), 420 mm wide in nine woven stripes (five light, four dark), lapped round both rails as a U and drawn at the 7.5 mm smallest feature instead of its true 0.7 mm. Thermal: every timber and steel part belongs to the frame region, the two back panels with their lap to the back canvas and the two seat panels with their lap to the seat canvas, all by reference to the body. Structure: rails, cross rails, prop arms and dowel are the members themselves, the four canvas panels are tension members of full width, the four bolts, the notch seat, the rear foot rounds and the five canvas hinges (head rail, back, sag, seat, knee rail) are the joints, the mass of each frame sits as two half masses inside its rails at the centre of mass and four 8 mm pads lie under the foot rounds.",
        "Das Tuch ist das unbelastete Segeltuch, wie es wirklich hängt: eine Kettenlinie von 1,28 m zwischen Kopf- und Knieleiste, getragen von vier Tuchknochen über gleiche Schritte nach vorn (zwei Rückenbahnen, zwei Sitzbahnen, 18 mm Stich unter jeder Sehne), 420 mm breit in neun gewebten Streifen (fünf helle, vier dunkle), als U um beide Leisten geschlagen und im Mindestmass von 7,5 mm statt der wahren 0,7 mm gezeichnet. Thermik: alle Holz- und Stahlteile gehören zur Rahmenregion, die zwei Rückenbahnen samt Umschlag zum Rückentuch, die zwei Sitzbahnen samt Umschlag zum Sitztuch, alle als Verweis auf den Körper. Tragwerk: Holme, Querleisten, Stützarme und Rundstab sind selbst die Stäbe, die vier Tuchbahnen Zugglieder in voller Breite, die vier Schrauben, der Rastensitz, die hinteren Fussrundungen und die fünf Tuchgelenke (Kopfleiste, Rücken, Durchhang, Sitz, Knieleiste) die Gelenke, die Masse jedes Rahmens sitzt als zwei Halbmassen in seinen Holmen auf Höhe des Schwerpunkts und vier 8 mm Platten liegen unter den Fussrundungen.",
    )
    return sc


def scale_100():
    sc = Scale(100, 0.25, 0.13, 0.18)
    sc.prism("back-rail", "back", stadium(BACK_FOOT, BACK_TOP, END_R, 4), pairs(Z_BACK), BE, 0.55, R_TIMBER, (4, 4, 4), TIMBER, {"kind": "capsule", "x0": BACK_FOOT + END_R, "y0": 0.0, "x1": BACK_TOP - END_R, "y1": 0.0, "r0": END_R, "r1": END_R}, member="back")
    sc.prism("top-rail", "back", rect(0.845, 0.885, -0.01, 0.01), [(-0.248, 0.248)], BE, 0.5, R_TIMBER, (2.5, 2.2, 5.6), CROSS, small=True, member="back")
    sc.prism("foot-rail", "back", rect(-0.22, -0.18, 0.02, 0.04), [(-0.268, 0.268)], BE, 0.5, R_TIMBER, (2.6, 2.8, 1.6), CROSS, small=True, member="back")
    sc.prism("seat-rail", "seat", notched_rail(4, False), pairs(Z_SEAT), BE, 0.55, R_TIMBER, (3, 3.5, 3), TIMBER, member="seat")
    sc.prism("front-rail", "seat", rect(0.985, 1.025, -0.01, 0.01), [(-0.226, 0.226)], BE, 0.5, R_TIMBER, (2.5, 2.9, 5.2), CROSS, small=True, member="seat")
    sc.prism("rear-rail", "seat", rect(0.025, 0.065, -0.01, 0.01), [(-0.226, 0.226)], BE, 0.5, R_TIMBER, (2.5, 1.5, 1.4), CROSS, small=True, member="seat")
    sc.prism("strut-arm", "strut", stadium(-STRUT_H / 2, L + STRUT_H / 2, STRUT_H / 2, 3), pairs(Z_STRUT), BE, 0.55, R_TIMBER, (5, 4.5, 5), TIMBER, {"kind": "capsule", "x0": 0.0, "y0": 0.0, "x1": rnd(L), "y1": 0.0, "r0": STRUT_H / 2, "r1": STRUT_H / 2}, member="strut")
    g, span = sc.turned("strut-bar", "strut", (L, 0.0), [(0.0, BAR_R), (0.58, BAR_R)], -0.29, 1, BE, 0.45, R_TIMBER, None, None, BAR_R, member="strut")
    sc.drawn("strut-bar", "strut", g, [span], BE, R_TIMBER, (6, 1.4, 1.5), CROSS, small=True)
    for side, name in ((1, "l"), (-1, "r")):
        g, span = sc.turned("pivot-bolt-" + name, "back", (0.0, 0.0), [(0.0, 0.0075), (0.046, 0.0075)], side * 0.27, -side, SH, 0.2, R_TIMBER, None, None, 0.0075, member="back", part="joint")
        sc.drawn("pivot-bolt-" + name, "back", g, [span], SH, R_TIMBER, (6, 6, 6), STEEL, small=True, elevation=side < 0)
        g, span = sc.turned("prop-bolt-" + name, "strut", (0.0, 0.0), [(0.0, 0.0075), (0.0515, 0.0075)], side * 0.2975, -side, SH, 0.2, R_TIMBER, None, None, 0.0075, member="strut", part="joint")
        sc.drawn("prop-bolt-" + name, "strut", g, [span], SH, R_TIMBER, (6.5, 6, 6), STEEL, small=True, elevation=side < 0)
    sling(sc, 0.015, (4, 3, 2, 2), Z_CANVAS, 9, (0.85, 0.5), (0.3, 0.85, 0.85), 0.02, 0.006)
    structure_common(sc, Z_SEAT, Z_BACK, Z_STRUT, Z_CANVAS, (0.04, 0.015, 0.02), 0.016, 0.0075, 0.015, (4, 3, 2, 2), 0.02, 0.012)
    sc.decide(
        "1:100 keeps the three nested frames at their true 20 x 40 mm and true lateral positions, but as plain joinery: rail ends as coarse half rounds, the four notches as square 24 mm housings, cross rails as sharp 40 x 20 mm bars butted between the rails, the prop dowel as a plain 22 mm cylinder and the four pivots as plain 15 mm pins without heads, washers or nuts. The sling stays the hanging catenary in nine stripes, drawn 15 mm thick with eleven facets over its four panels; the laps round the rails are left out and the panels run into the rails instead.",
        "1:100 behält die drei ineinander gesteckten Rahmen in wahren 20 x 40 mm und wahren Seitenlagen, aber als schlichte Schreinerarbeit: Holmenden als grobe Halbrundungen, die vier Rasten als eckige 24 mm Ausklinkungen, Querleisten als scharfkantige Stäbe 40 x 20 mm stumpf zwischen den Holmen, der Stützstab als glatter 22 mm Zylinder und die vier Gelenke als glatte 15 mm Bolzen ohne Köpfe, Scheiben und Muttern. Das Tuch bleibt die hängende Kettenlinie in neun Streifen, 15 mm dick mit elf Facetten über seine vier Bahnen; die Umschläge um die Leisten entfallen, die Bahnen laufen stattdessen in die Leisten.",
    )
    sc.decide(
        "Thermal references the body part by part (frame region, back canvas, seat canvas). Structure: rails, cross rails, prop arms and dowel are the members, the canvas panels full-width tension members, the pins, notch seat, rear foot rounds and five canvas hinges the joints, two half masses per frame inside the rails at the centre of mass, four 15 mm pads under the feet.",
        "Die Thermik verweist Teil für Teil auf den Körper (Rahmenregion, Rückentuch, Sitztuch). Tragwerk: Holme, Querleisten, Stützarme und Rundstab sind die Stäbe, die Tuchbahnen Zugglieder in voller Breite, die Bolzen, der Rastensitz, die hinteren Fussrundungen und die fünf Tuchgelenke die Gelenke, zwei Halbmassen je Rahmen in den Holmen auf Höhe des Schwerpunkts, vier 15 mm Platten unter den Füssen.",
    )
    return sc


def scale_200():
    sc = Scale(200, 0.25, 0.13, 0.13)
    z_strut, z_back, z_seat, half = (0.26, 0.29), (0.23, 0.26), (0.2, 0.23), 0.19
    dark = (0.3, 0.3, 0.3)
    sc.prism("back-rail", "back", stadium(BACK_FOOT, BACK_TOP, END_R, 2), pairs(z_back), BE, 0.55, R_TIMBER, (4, 4, 4), dark, member="back")
    sc.prism("top-rail", "back", rect(0.845, 0.885, -0.015, 0.015), [(-0.23, 0.23)], BE, 0.5, R_TIMBER, (2.5, 2.2, 5.6), dark, member="back")
    sc.prism("foot-rail", "back", rect(-0.22, -0.18, 0.02, 0.05), [(-0.26, 0.26)], BE, 0.5, R_TIMBER, (2.6, 2.8, 1.6), dark, member="back")
    sc.prism("seat-rail", "seat", stadium(0.0, SEAT_LEN, END_R, 2), pairs(z_seat), BE, 0.55, R_TIMBER, (3, 3.5, 3), dark, member="seat")
    sc.prism("front-rail", "seat", rect(0.985, 1.025, -0.015, 0.015), [(-0.2, 0.2)], BE, 0.5, R_TIMBER, (2.5, 2.9, 5.2), dark, member="seat")
    sc.prism("rear-rail", "seat", rect(0.025, 0.065, -0.015, 0.015), [(-0.2, 0.2)], BE, 0.5, R_TIMBER, (2.5, 1.5, 1.4), dark, member="seat")
    sc.prism("strut-arm", "strut", stadium(-0.016, L + 0.016, 0.016, 2), pairs(z_strut), BE, 0.55, R_TIMBER, (5, 4.5, 5), dark, member="strut")
    sc.prism("strut-bar", "strut", rect(L - 0.015, L + 0.015, -0.015, 0.015), [(-0.26, 0.26)], BE, 0.45, R_TIMBER, (6, 1.4, 1.5), dark, member="strut", elevation=False)
    sling(sc, 0.03, (2, 1, 1, 1), half, 0, (0.8,), (0.3, 0.8, 0.8), 0.02, 0.01)
    sc.sleeve_joint("j-pivot", "back", (0.0, 0.0), 0.02, (0.2, 0.26), "back")
    sc.sleeve_joint("j-prop", "strut", (0.0, 0.0), 0.02, (0.23, 0.29), "strut")
    structure_common(sc, z_seat, z_back, z_strut, half, (0.045, 0.03, 0.03), 0.022, 0.015, 0.03, (2, 1, 1, 1), 0.02, 0.015)
    sc.decide(
        "1:200 generalises the chair to what 30 mm can carry: every rail is a bar with chamfered ends, 40 mm deep and 30 mm thick (the true 20 mm is below the feature), so the three frames are redrawn side by side at 260 to 290 mm (prop), 230 to 260 mm (back) and 200 to 230 mm (seat) and the sling narrows to 380 mm; notches, rounded ends, bolts, arrises and the canvas stripes are left out, the cross rails and the prop bar are 40 x 30 mm and 30 x 30 mm bars. The sling is four 30 mm sheets, one per canvas bone and only the uppermost bent once, along the same catenary.",
        "1:200 verallgemeinert den Stuhl auf das, was 30 mm tragen: jeder Holm ist ein Stab mit gefasten Enden, 40 mm hoch und 30 mm dick (die wahren 20 mm liegen unter dem Mindestmass), darum liegen die drei Rahmen neu nebeneinander bei 260 bis 290 mm (Stütze), 230 bis 260 mm (Rücken) und 200 bis 230 mm (Sitz), und das Tuch wird 380 mm schmal; Rasten, runde Enden, Schrauben, gebrochene Kanten und Tuchstreifen entfallen, Querleisten und Stützstab sind Stäbe 40 x 30 mm und 30 x 30 mm. Das Tuch besteht aus vier 30 mm Bahnen, eine je Tuchknochen und nur die oberste einmal geknickt, entlang derselben Kettenlinie.",
    )
    sc.decide(
        "Thermal references the body (frame, back canvas, seat canvas). Structure: the bars are the members, the canvas sheets tension members, two 40 mm sleeves per side stand for the frame pivot and the prop pivot, further sleeves for the notch seat and the rear feet, 30 mm rods for the canvas hinges, two half masses per frame inside the bars and four 30 mm pads.",
        "Die Thermik verweist auf den Körper (Rahmen, Rückentuch, Sitztuch). Tragwerk: die Stäbe sind die Glieder, die Tuchbahnen Zugglieder, je Seite zwei 40 mm Hülsen stehen für Rahmengelenk und Stützgelenk, weitere Hülsen für Rastensitz und Hinterfüsse, 30 mm Stäbe für die Tuchgelenke, zwei Halbmassen je Rahmen in den Stäben und vier 30 mm Platten.",
    )
    return sc


def scale_500():
    sc = Scale(500, 0.18, 0.13, 0.2)
    z_strut, z_back, z_seat, half = (0.25, 0.29), (0.21, 0.25), (0.17, 0.21), 0.17
    dark = (0.3, 0.3, 0.3)
    sc.prism("back-rail", "back", stadium(BACK_FOOT, BACK_TOP, END_R, 2), pairs(z_back), BE, 0.55, R_TIMBER, (4, 4, 4), dark, member="back")
    sc.prism("top-rail", "back", rect(0.84, 0.88, -0.02, 0.02), [(-0.21, 0.21)], BE, 0.5, R_TIMBER, (2.5, 2.2, 5.6), dark, member="back")
    sc.prism("seat-rail", "seat", stadium(0.0, SEAT_LEN, END_R, 2), pairs(z_seat), BE, 0.55, R_TIMBER, (3, 3.5, 3), dark, member="seat")
    sc.prism("front-rail", "seat", rect(0.985, 1.025, -0.02, 0.02), [(-0.17, 0.17)], BE, 0.5, R_TIMBER, (2.5, 2.9, 5.2), dark, member="seat")
    sc.prism("strut-arm", "strut", stadium(-0.02, L + 0.02, 0.02, 2), pairs(z_strut), BE, 0.55, R_TIMBER, (5, 4.5, 5), dark, member="strut")
    sc.prism("strut-bar", "strut", rect(L - 0.02, L + 0.02, -0.02, 0.02), [(-0.25, 0.25)], BE, 0.45, R_TIMBER, (6, 1.4, 1.5), dark, member="strut", elevation=False)
    sling(sc, 0.04, (1, 1, 1, 1), half, 0, (0.8,), (0.3, 0.75, 0.75), 0.02, 0.012)
    sc.sleeve_joint("j-pivot", "back", (0.0, 0.0), 0.024, (0.17, 0.25), "back")
    sc.sleeve_joint("j-prop", "strut", (0.0, 0.0), 0.024, (0.21, 0.29), "strut")
    structure_common(sc, z_seat, z_back, z_strut, half, (0.05, 0.04, 0.04), 0.024, 0.02, 0.04, (1, 1, 1, 1), 0.02, 0.02, 0.02)
    sc.decide(
        "Minimal: at 1:500 the chair is a sign of 2 mm, so only its three frames and the sling remain: every rail is a square 40 x 40 mm bar with chamfered ends (the drawing shows 40 mm outlines, below the 75 mm feature), the frames sit side by side at 250 to 290 mm (prop), 210 to 250 mm (back) and 170 to 210 mm (seat), each with the one cross bar that makes it a frame (head rail, knee rail, prop bar); foot rail, rear rail, notches, bolts and stripes are left out. The sling is four straight 40 mm sheets 340 mm wide, one per canvas bone, the chords of the hanging curve.",
        "Minimal: bei 1:500 ist der Stuhl ein Zeichen von 2 mm, es bleiben nur seine drei Rahmen und das Tuch: jeder Holm ist ein quadratischer Stab 40 x 40 mm mit gefasten Enden (die Zeichnung zeigt 40 mm Umrisse, unter dem Mindestmass von 75 mm), die Rahmen liegen nebeneinander bei 250 bis 290 mm (Stütze), 210 bis 250 mm (Rücken) und 170 bis 210 mm (Sitz), jeder mit dem einen Querstab, der ihn zum Rahmen macht (Kopfleiste, Knieleiste, Stützstab); Fussleiste, hintere Leiste, Rasten, Schrauben und Streifen entfallen. Das Tuch besteht aus vier geraden 40 mm Bahnen von 340 mm Breite, eine je Tuchknochen, den Sehnen der Hängekurve.",
    )
    sc.decide(
        "Thermal references the body (frame, back canvas, seat canvas). Structure: the bars are the members, the four sheets the tension members, 48 mm sleeves mark the frame pivot, the prop pivot, the notch seat and the rear feet, 40 mm rods the canvas hinges, two half masses per frame the masses, four 40 mm pads the feet.",
        "Die Thermik verweist auf den Körper (Rahmen, Rückentuch, Sitztuch). Tragwerk: die Stäbe sind die Glieder, die vier Bahnen die Zugglieder, 48 mm Hülsen markieren Rahmengelenk, Stützgelenk, Rastensitz und Hinterfüsse, 40 mm Stäbe die Tuchgelenke, zwei Halbmassen je Rahmen die Massen, vier 40 mm Platten die Füsse.",
    )
    return sc


SCALES = {"50": scale_50, "100": scale_100, "200": scale_200, "500": scale_500}
