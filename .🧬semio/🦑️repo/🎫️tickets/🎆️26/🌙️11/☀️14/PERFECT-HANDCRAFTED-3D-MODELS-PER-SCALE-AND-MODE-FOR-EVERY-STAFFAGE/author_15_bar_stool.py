from author_15_lib import *
CH, LE, FO, PW = "chrome-steel", "pu-leatherette", "hr-foam", "birch-plywood"
RP = {"count": 4, "rotationAxis": Y, "rotationRad": math.pi / 2, "origin": [0, 0, 0]}
SUPPORTS = [("foot-xp", 0.1975, 0), ("foot-xn", -0.1975, 0), ("foot-zp", 0, 0.1975), ("foot-zn", 0, -0.1975)]
TILT = lambda r0, r1, y0, y1: (r0, r1, y0, y1)

def arc(n, rad, y, a):
    return [[-rad * math.cos(a * (2 * i / (n - 1) - 1)), y, rad * math.sin(a * (2 * i / (n - 1) - 1))] for i in range(n)]

def legs(c, r, top=0.70, foot=0.1975, **ex):
    c.cap("leg", "frame", CH, "body", 0.7, [0.14, top, 0], [foot, r, 0], r, repeat=RP, **ex)

def supports(c, h, r):
    c.to("structure")
    for sid, x, z in SUPPORTS:
        c.rev("support-" + sid, "frame", CH, "ground", 0.2, [[0, r], [h, r]], Y, [x, 0, z], member="frame", part="support", anchor=sid)

def s50():
    c = Ctx("bar-stool", 50)
    c.to("body")
    legs(c, 0.0125)
    c.rev("ferrule", "frame", CH, "body", 0.2, [[0, 0.011], [0.002, 0.0135], [0.005, 0.015], [0.0215, 0.015], [0.022, 0.0125]], Y, [0.1975, 0, 0], repeat=RP)
    c.torus("ring", "frame", CH, "body", 0.7, [0, 0.27, 0], 0.19, 0.0125, n=16)
    c.rev("plate", "seat", PW, "body", 0.6, [[-0.018, 0.174], [-0.012, 0.18], [0.022, 0.18]], Y, [0, 0.698, 0])
    padp = [[-0.01, 0.18], [0.012, 0.18], [0.0185, 0.1793], [0.0235, 0.1771], [0.0271, 0.1735], [0.0293, 0.1685], [0.03, 0.162], [0.0315, 0.1], [0.032, 0.0]]
    c.rev("pad", "pad", LE, "body", 0.3, padp, Y, [0, 0.72, 0])
    c.cap("back-post", "backrest", CH, "body", 0.7, [-0.14, 0.72, 0], [-0.14, 0.82, 0], 0.015)
    sec = lambda a, rx, ry: [(p, rx, ry, 4) for p in arc(9, 0.14, 0.825, a)]
    c.loft("back-pad", "backrest", LE, "body", 0.3, sec(0.714, 0.045, 0.02), None, None)
    c.to("thermal")
    c.ref("th-leg", "leg", region="legs", layer="leg-wall"); c.ref("th-ferrule", "ferrule", region="legs", layer="leg-wall"); c.ref("th-back-post", "back-post", region="legs", layer="leg-wall")
    c.ref("th-ring", "ring", region="footrest-ring", layer="ring-wall")
    shrink = [[a + (0.0015 if a < 0 else -0.0015 if a >= 0.03 else 0), rr - 0.0015 if rr > 0 else 0] for a, rr in [[-0.01, 0.18], [0.012, 0.18], [0.0185, 0.1793], [0.0235, 0.1771], [0.0271, 0.1735], [0.0293, 0.1685], [0.03, 0.162], [0.0315, 0.1], [0.032, 0.0]]]
    shrink = [[-0.0085, 0.1785], [0.012, 0.1785], [0.0177, 0.1778], [0.0222, 0.1757], [0.0256, 0.1724], [0.0277, 0.1678], [0.0285, 0.162], [0.0298, 0.1], [0.0305, 0.0]]
    c.rev("th-seat-pad-foam", "pad", FO, "body", 0.8, shrink, Y, [0, 0.72, 0], region="seat-pad", layer="pad-foam")
    c.ref("th-seat-pad-leatherette", "pad", region="seat-pad", layer="leatherette")
    c.rev("th-seat-plate-core", "seat", PW, "body", 0.55, [[-0.014, 0.17], [0.018, 0.17]], Y, [0, 0.698, 0], region="seat-plate", layer="plate-ply", sectionFaces=["+u"])
    c.loft("th-backrest-foam", "backrest", FO, "body", 0.8, sec(0.714, 0.0435, 0.0185), None, None, region="backrest", layer="back-foam")
    c.ref("th-backrest-leatherette", "back-pad", region="backrest", layer="leatherette")
    c.to("structure")
    for sid, m in (("leg", "frame"), ("ring", "frame"), ("ferrule", "ground"), ("plate", "seat"), ("pad", "pad"), ("back-post", "backrest"), ("back-pad", "backrest")):
        c.ref("m-" + sid, sid, member=m, part="member")
    c.rev("j-swivel", "seat", CH, "detail", 0.4, [[-0.015, 0.05], [0.015, 0.05]], Y, [0, 0.698, 0], member="seat", part="joint")
    c.rev("j-pad-guide", "pad", CH, "detail", 0.4, [[-0.02, 0.02], [0.01, 0.02]], Y, [0, 0.72, 0], member="pad", part="joint")
    c.ell("mass-frame", "frame", CH, "detail", 0.4, [0, 0.35, 0], [0.012, 0.05, 0.012], member="frame", part="mass")
    c.ell("mass-seat", "seat", PW, "detail", 0.4, [0.01, 0.72, 0], [0.05, 0.025, 0.05], member="seat", part="mass")
    c.ell("mass-backrest", "backrest", LE, "detail", 0.4, [-0.14, 0.82, 0], [0.012, 0.025, 0.03], member="backrest", part="mass")
    supports(c, 0.008, 0.0125)
    c.dec("1:50 is the full model: four splayed 25 mm chromed tubes (one solid, four copies at 90 degrees) starting inside the seat plate and ending in domed rubber caps, the footrest as a true torus welded 11 mm into the legs, the plywood plate and the rounded foam pad as revolves about the swivel axis, a 30 mm back post and a low back cushion 0.2 m long, 90 mm high and 40 mm thick that wraps the seat on its 0.14 m radius (nine-section superelliptic loft, 184 mm chord) instead of the flat block.", "1:50 ist das volle Modell: vier gespreizte 25 mm Chromrohre (ein Körper, vier Kopien im Abstand von 90 Grad), die im Sitzteller beginnen und in gewölbten Gummikappen enden, die Fußstütze als echter Torus, 11 mm in die Beine eingeschweisst, Sperrholzplatte und gerundetes Schaumpolster als Drehkörper um die Schwenkachse, ein 30 mm Lehnenpfosten und ein niedriges Rückenkissen von 0,2 m Länge, 90 mm Höhe und 40 mm Dicke, das den Sitz auf dem Radius 0,14 m umfasst (neunschnittiger Superellipsen-Loft, 184 mm Sehne) statt des flachen Blocks.")
    c.dec("Thermal: the two foam regions are nested shells (foam core shrunk by the 1.5 mm cover, leatherette is the body surface; the asset's layer order was corrected to innermost first), the hidden plate is a plywood core inside the visible plate whose upper face is a declared section face (only the underside and rim are the region's surface), tube regions reference the body tubes. Structure: tubes and ring are the frame members, the rubber caps the ground member, plate, pad and back parts their own members, a swivel bearing and a pad guide are the joints, three compact masses sit at the centres of mass, four contact pads end the legs.", "Thermik: die zwei Schaumregionen sind verschachtelte Schalen (Schaumkern um die 1,5 mm Bezug eingezogen, Kunstleder ist die Körperoberfläche; die Schichtfolge des Assets wurde auf innen zuerst korrigiert), die verdeckte Platte ist ein Sperrholzkern im sichtbaren Teller, dessen Oberseite als Schnittfläche erklärt ist (nur Unterseite und Rand sind die Fläche der Region), die Rohrregionen verweisen auf die Körperrohre. Tragwerk: Rohre und Ring sind die Gestellglieder, die Gummikappen das Bodenglied, Platte, Polster und Lehne eigene Glieder, ein Drehlager und eine Polsterführung die Gelenke, drei kompakte Massen an den Schwerpunkten, vier Kontaktflächen schliessen die Beine ab.")
    return c.out()

def s100():
    c = Ctx("bar-stool", 100)
    c.to("body")
    legs(c, 0.0125)
    c.torus("ring", "frame", CH, "body", 0.7, [0, 0.27, 0], 0.19, 0.0125, n=8)
    c.rev("plate", "seat", PW, "body", 0.6, [[-0.018, 0.18], [0.022, 0.18]], Y, [0, 0.698, 0])
    c.rev("pad", "pad", LE, "body", 0.3, [[-0.01, 0.18], [0.018, 0.18], [0.03, 0.168]], Y, [0, 0.72, 0])
    c.cap("back-post", "backrest", CH, "body", 0.7, [-0.14, 0.72, 0], [-0.14, 0.82, 0], 0.015)
    c.loft("back-pad", "backrest", LE, "body", 0.3, [(p, 0.045, 0.02, 4) for p in arc(5, 0.14, 0.825, 0.714)], None, None)
    c.to("thermal")
    c.ref("th-leg", "leg", region="legs"); c.ref("th-back-post", "back-post", region="legs")
    c.ref("th-ring", "ring", region="footrest-ring")
    c.ref("th-seat-pad", "pad", region="seat-pad")
    c.rev("th-seat-plate-core", "seat", PW, "body", 0.55, [[-0.012, 0.16], [0.016, 0.16]], Y, [0, 0.698, 0], region="seat-plate", sectionFaces=["+u"])
    c.ref("th-backrest", "back-pad", region="backrest")
    c.to("structure")
    for sid, m in (("leg", "frame"), ("ring", "frame"), ("plate", "seat"), ("pad", "pad"), ("back-post", "backrest"), ("back-pad", "backrest")):
        c.ref("m-" + sid, sid, member=m, part="member")
    c.rev("m-cap", "frame", CH, "detail", 0.2, [[0, 0.02], [0.03, 0.02]], Y, [0.1975, 0, 0], member="ground", part="member", repeat=RP)
    c.rev("j-swivel", "seat", CH, "detail", 0.4, [[-0.015, 0.05], [0.015, 0.05]], Y, [0, 0.698, 0], member="seat", part="joint")
    c.rev("j-pad-guide", "pad", CH, "detail", 0.4, [[-0.02, 0.02], [0.01, 0.02]], Y, [0, 0.72, 0], member="pad", part="joint")
    c.ell("mass-frame", "frame", CH, "detail", 0.4, [0, 0.35, 0], [0.012, 0.05, 0.012], member="frame", part="mass")
    c.ell("mass-seat", "seat", PW, "detail", 0.4, [0.01, 0.72, 0], [0.05, 0.025, 0.05], member="seat", part="mass")
    c.ell("mass-backrest", "backrest", LE, "detail", 0.4, [-0.14, 0.84, 0], [0.012, 0.025, 0.03], member="backrest", part="mass")
    supports(c, 0.015, 0.0125)
    c.dec("1:100 drops the rubber caps (the tube ends at the floor), the pad rounding (a 12 mm chamfer), the plate edge radius and the 9-section cushion loft (five sections of the same wrap, 184 mm chord); the footrest torus has eight sections; tubes, seat plate, pad, ring and back keep the 1:50 positions.", "1:100 lässt die Gummikappen weg (das Rohr endet am Boden), die Polsterrundung (eine 12 mm Fase), den Plattenrand und die neunschnittige Kissenform (fünf Schnitte derselben Umfassung, 184 mm Sehne); der Fusstützen-Torus hat acht Schnitte; Rohre, Sitzplatte, Polster, Ring und Lehne behalten die Lagen von 1:50.")
    c.dec("Thermal references the body solids; the hidden plate underside is a plywood core inside the plate. Structure: tube, ring, plate, pad and back are members, the ground member is a 40 mm cap collar at each leg end, swivel and pad guide are joints, three masses, four 15 mm contact pads.", "Thermik verweist auf die Körper; die verdeckte Plattenunterseite ist ein Sperrholzkern im Teller. Tragwerk: Rohr, Ring, Platte, Polster und Lehne sind Glieder, das Bodenglied ist ein 40 mm Kappenkragen an jedem Beinende, Drehlager und Polsterführung sind Gelenke, drei Massen, vier 15 mm Kontaktflächen.")
    return c.out()

def s200():
    c = Ctx("bar-stool", 200)
    c.to("body")
    legs(c, 0.0125, 0.70, 0.1975, slender=True)
    c.torus("ring", "frame", CH, "body", 0.7, [0, 0.27, 0], 0.19, 0.015, n=8)
    c.rev("seat", "seat", LE, "body", 0.35, [[0, 0.18], [0.052, 0.18]], Y, [0, 0.698, 0])
    c.bw("back-post", "backrest", LE, "body", 0.35, (-0.16, -0.12), (0.69, 0.80), (-0.02, 0.02))
    c.bw("back-pad", "backrest", LE, "body", 0.35, (-0.16, -0.091), (0.78, 0.87), (-0.105, 0.105))
    c.to("thermal")
    c.ref("th-leg", "leg", region="legs"); c.ref("th-ring", "ring", region="footrest-ring")
    c.rev("th-seat-pad", "seat", LE, "body", 0.35, [[0.022, 0.18], [0.052, 0.18]], Y, [0, 0.698, 0], region="seat-pad")
    c.rev("th-seat-plate-core", "seat", PW, "body", 0.55, [[-0.008, 0.165], [0.022, 0.165]], Y, [0, 0.698, 0], region="seat-plate", sectionFaces=["+u"])
    c.ref("th-backrest", "back-pad", region="backrest")
    c.to("structure")
    for sid, m in (("leg", "frame"), ("ring", "frame"), ("seat", "seat"), ("seat", "pad"), ("back-post", "backrest"), ("back-pad", "backrest")):
        c.ref("m-%s-%s" % (m, sid), sid, member=m, part="member")
    c.rev("m-cap", "frame", CH, "detail", 0.2, [[0, 0.03], [0.06, 0.03]], Y, [0.1975, 0, 0], member="ground", part="member", repeat=RP)
    c.bw("mass-frame", "frame", CH, "detail", 0.4, (-0.04, 0.04), (0.29, 0.41), (-0.04, 0.04), member="frame", part="mass")
    c.bw("mass-seat", "seat", PW, "detail", 0.4, (-0.05, 0.07), (0.69, 0.75), (-0.06, 0.06), member="seat", part="mass")
    c.bw("mass-backrest", "backrest", LE, "detail", 0.4, (-0.155, -0.125), (0.81, 0.87), (-0.05, 0.05), member="backrest", part="mass")
    supports(c, 0.03, 0.015)
    c.dec("1:200 merges plate and pad into one 52 mm seat disc and the post and cushion into a 40 mm post box and a 210 mm wide, 69 mm deep cushion box; the legs stay true 25 mm tubes (slender, drawn as strokes at the scale pen, C24), the footrest ring is a 30 mm torus (a closed ring cannot be slender, so it takes the 30 mm smallest feature) with eight sections; the swivel and pad joints are hidden inside the seat disc and left out.", "1:200 fasst Platte und Polster zu einer 52 mm Sitzscheibe und Pfosten und Kissen zu einem 40 mm Pfostenquader und einem 210 mm breiten, 69 mm tiefen Kissenquader zusammen; die Beine bleiben echte 25 mm Rohre (schlank, als Striche mit dem Stiftmass des Massstabs gezeichnet, C24), der Fussring ist ein 30 mm Torus (ein geschlossener Ring kann nicht schlank sein, daher gilt das 30 mm Mindestmass) mit acht Schnitten; Drehlager und Polsterführung liegen verborgen in der Sitzscheibe und entfallen.")
    c.dec("Thermal and structure reference the merged solids; seat and pad members both stand on the seat disc.", "Thermik und Tragwerk verweisen auf die verschmolzenen Körper; Sitz- und Polsterglied stehen beide auf der Sitzscheibe.")
    return c.out()

def s500():
    c = Ctx("bar-stool", 500)
    c.to("body")
    legs(c, 0.0125, 0.70, 0.1975, slender=True)
    c.torus("ring", "frame", CH, "body", 0.7, [0, 0.27, 0], 0.19, 0.0375, n=6)
    c.rev("seat", "seat", LE, "body", 0.35, [[-0.048, 0.18], [0.052, 0.18]], Y, [0, 0.698, 0])
    c.bw("back", "backrest", LE, "body", 0.35, (-0.175, -0.10), (0.75, 0.87), (-0.105, 0.105))
    c.to("thermal")
    c.ref("th-leg", "leg", region="legs"); c.ref("th-ring", "ring", region="footrest-ring")
    c.rev("th-seat-pad", "seat", LE, "body", 0.35, [[-0.023, 0.18], [0.052, 0.18]], Y, [0, 0.698, 0], region="seat-pad")
    c.rev("th-seat-plate-core", "seat", PW, "body", 0.55, [[-0.048, 0.165], [0.027, 0.165]], Y, [0, 0.698, 0], region="seat-plate", sectionFaces=["+u"])
    c.ref("th-backrest", "back", region="backrest")
    c.to("structure")
    for sid, m in (("leg", "frame"), ("ring", "frame"), ("seat", "seat"), ("seat", "pad"), ("back", "backrest")):
        c.ref("m-%s-%s" % (m, sid), sid, member=m, part="member")
    c.rev("m-cap", "frame", CH, "detail", 0.2, [[0, 0.0386], [0.075, 0.0386]], Y, [0.171, 0, 0], member="ground", part="member", repeat=RP)
    c.bw("mass-frame", "frame", CH, "detail", 0.4, (-0.075, 0.075), (0.275, 0.425), (-0.075, 0.075), member="frame", part="mass")
    c.bw("mass-seat", "seat", PW, "detail", 0.4, (-0.075, 0.075), (0.68, 0.76), (-0.075, 0.075), member="seat", part="mass")
    supports(c, 0.075, 0.0375)
    c.dec("Minimal: 1:500 is four true 25 mm slender legs (drawn as 90 mm pen strokes, C24), a six-section 75 mm ring, one 100 mm seat disc and one 75 x 120 x 210 mm back block; the closed ring cannot be slender, so the 75 mm smallest feature makes it three times thicker than the real 25 mm tube.", "Minimal: 1:500 besteht aus vier echten schlanken 25 mm Beinen (als Striche mit 90 mm Stiftmass gezeichnet, C24), einem sechsschnittigen 75 mm Ring, einer 100 mm Sitzscheibe und einem 75 x 120 x 210 mm Lehnenblock; der geschlossene Ring kann nicht schlank sein, das 75 mm Mindestmass macht ihn dreimal dicker als das echte 25 mm Rohr.")
    c.dec("Thermal and structure reference the body solids; the backrest mass is merged into the back block.", "Thermik und Tragwerk verweisen auf die Körper; die Lehnenmasse ist im Lehnenblock aufgegangen.")
    return c.out()

if __name__ == "__main__":
    save("bar-stool", {50: s50(), 100: s100(), 200: s200(), 500: s500()})
