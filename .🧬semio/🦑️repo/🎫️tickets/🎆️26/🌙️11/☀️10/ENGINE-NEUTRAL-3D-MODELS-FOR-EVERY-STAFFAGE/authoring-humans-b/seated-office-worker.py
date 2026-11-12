import math
from lib import Builder, auto, ORGANIC, R
b = Builder("seated-office-worker")
auto(b, over={"cuff-l": None, "cuff-r": None, "eye": dict(front=["eye-l", "eye-r"]),
    "head": dict(front=["head"], plan=["face"], xmax=0.142, n=9, exp=2.2), "hair": dict(front=["hair"], plan=["head"], n=6),
    "collar": dict(lat=(-0.065, 0.065), n=4, exp=2.2),
    "shirt": dict(front=["torso"], plan=["trunk-l", "trunk-r"], latscale=0.9, n=9, exp=2.5),
    "pelvis-trousers": dict(front=["hips"], plan=["waist"], n=4, exp=2.4),
    "sleeve-l": dict(front=["upper-arm-l"], plan=["upper-arm-l"], n=6), "sleeve-r": dict(front=["upper-arm-r"], plan=["upper-arm-r"], n=6),
    "shoe-rear-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], exp=2.6), "shoe-rear-r": dict(front=["shoe-r"], plan=["shoe-heel-r"], exp=2.6),
    "thumb-l": dict(lat=(0.132, 0.158), n=3), "thumb-r": dict(lat=(-0.158, -0.132), n=3)}, bones=set(ORGANIC))
b.ell("nose", "head", (0.155, 1.27, 0), (0.017, 0.02, 0.011), material="skin", role="detail", tone=0.25, region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0.045, 1.095, 0), (0.1, 0.04, 0.17), src="shirt", maxden_ids=["shirt"])
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"arm-{s}", (0.04, 1.085, z * 0.145), (0.06, 0.045, 0.055), src=f"sleeve-{s}", maxden_ids=[f"sleeve-{s}"])
    b.ell(f"knee-{s}", f"shank-{s}", (0.474, 0.54, z * 0.085), (0.06, 0.059, 0.063), src=f"shank-{s}", maxden_ids=[f"shank-{s}"])
PA = b.meta("chair-hub")
b.add("chair-hub", "chair-base", "revolve", {"profile": [[-0.02, 0.021], [-0.017, 0.032], [-0.01, 0.045], [0.0, 0.055], [0.025, 0.055], [0.042, 0.04]], "axis": [0, 1, 0], "origin": [0, 0, 0]}, PA, ["chair-hub"])
arms = (("front", (0.34, 0.0), 0.3), ("side-a", (0.133, 0.273), 0.288), ("side-b", (0.133, -0.273), 0.288), ("back-a", (-0.203, 0.176), 0.3), ("back-b", (-0.203, -0.176), 0.3))
for name, (cx, cz), r in arms:
    ang = math.atan2(cz, cx - 0.04)
    rs = (0.03, 0.1, 0.18, 0.26, r)
    pts = [(0.04 + q * math.cos(ang), 0.043 - 0.0085 * q / 0.3, q * math.sin(ang)) for q in rs]
    b.loftw(f"chair-arm-{name}", f"chair-arm-{name}", pts, 0.024, [0.016, 0.0145, 0.0135, 0.0128, 0.0125], exp=2.6, mt=b.meta("chair-arm-front"), maxden_ids=["chair-arm-front"])
    b.ell(f"chair-castor-{name}", f"chair-castor-{name}", (cx, 0.025, cz), (0.025, 0.025, 0.025), mt=b.meta("chair-castor-front-wheel", material=b.dmat["chair-castor-front-wheel"]), maxden_ids=["chair-castor-front-wheel"])
    b.ell(f"chair-yoke-{name}", f"chair-arm-{name}", (cx, 0.025, cz), (0.022, 0.026, 0.022), mt=b.meta("chair-arm-front-yoke"), maxden_ids=["chair-arm-front-yoke"])
    b.capsule(f"chair-stem-{name}", f"chair-arm-{name}", (cx, 0.045, cz), (cx, 0.057, cz), 0.006, mt=b.meta("chair-arm-front-stem"), maxden_ids=["chair-arm-front-stem"])
b.add("chair-column-sleeve", "chair-column-sleeve", "revolve", {"profile": [[0.02, 0.034], [0.3, 0.026]], "axis": [1, 0, 0], "origin": [0, 0, 0]}, b.meta("chair-column-sleeve"), ["chair-column-sleeve"])
b.add("chair-column-collar", "chair-column-sleeve", "revolve", {"profile": [[0.285, 0.032], [0.305, 0.032]], "axis": [1, 0, 0], "origin": [0, 0, 0]}, b.meta("chair-column-collar"), ["chair-column-collar"])
b.capsule("chair-column", "chair-column", (0.04, 0.224, 0), (0.04, 0.456, 0), 0.012, maxden_ids=["chair-column"])
b.add("chair-column-cone", "chair-column", "revolve", {"profile": [[0.37, 0.014], [0.435, 0.032]], "axis": [1, 0, 0], "origin": [0, 0, 0]}, b.meta("chair-column-cone"), ["chair-column-cone"])
b.organic("chair-seat-cushion", "chair-seat", "chair-seat-cushion", front=["chair-seat"], plan=["chair-seat-plan"], n=9, exp=4.0, ends=0.9)
b.box("chair-seat-mechanism", "chair-seat", (0.04, 0.404, 0), (0.2, 0.042, 0.14), corner=0.004, maxden_ids=["chair-seat-mechanism"])
b.capsule("chair-seat-lever", "chair-seat", (0.14, 0.4, -0.12), (0.21, 0.398, -0.12), 0.005, src="chair-seat-lever", maxden_ids=["chair-seat-lever"], role="detail")
b.loft("chair-armrest-post", "chair-seat", [{"at": [R(c) for c in b.loc("chair-seat", p)], "rx": 0.012, "ry": ry, "exponent": 2.2} for p, ry in (((0.005, 0.485, 0.268), 0.015), ((0.005, 0.645, 0.268), 0.01))], maxden_ids=["chair-armrest-post"])
b.solids[-1]["repeat"] = {"count": 2, "translation": [0, 0, -0.536]}
b.loft("chair-armrest-pad", "chair-seat", [{"at": [R(c) for c in b.loc("chair-seat", p)], "rx": 0.021, "ry": 0.018, "exponent": 3.0} for p in ((-0.163, 0.66, 0.268), (-0.14, 0.66, 0.268), (0.15, 0.66, 0.268), (0.172, 0.66, 0.268))], maxden_ids=["chair-armrest-pad"])
b.solids[-1]["repeat"] = {"count": 2, "translation": [0, 0, -0.536]}
b.organic("chair-backrest-shell", "chair-backrest", "chair-backrest-shell", front=["chair-backrest"], plan=["chair-backrest-plan"], n=9, exp=4.0, ends=0.85)
b.capsule("chair-backrest-strut", "chair-backrest", (-0.2, 0.442, 0), (-0.2, 0.686, 0), 0.017, maxden_ids=["chair-backrest-strut"])
b.boxl("desk-top", "desk-top", (0.39, 0.0125, 0), (0.78, 0.025, 0.6), corner=0.004, maxden_ids=["desk-top"], src="desk-top")
b.box("desk-apron", "desk-top", (0.99, 0.645, 0), (0.02, 0.1, 0.51), src="desk-top", maxden_ids=["desk-top"])
b.box("desk-post", "desk-leg", (1.02, 0.3475, 0.275), (0.695, 0.04, 0.04), src="desk-end-panel", maxden_ids=["desk-end-panel"], repeat={"count": 2, "translation": [0, 0, -0.55]})
b.box("desk-foot-plate", "desk-leg", (1.015, 0.006, 0.275), (0.012, 0.09, 0.06), src="desk-foot-plate", maxden_ids=["desk-foot-plate"], repeat={"count": 2, "translation": [0, 0, -0.55]})
b.box("laptop-base", "laptop", (0.58, 0.7305, 0), (0.32, 0.021, 0.34), corner=0.004, src="laptop-base", maxden_ids=["laptop-base"])
b.capsule("laptop-hinge", "laptop", (0.741, 0.741, -0.12), (0.741, 0.741, 0.12), 0.005, src="laptop-hinge", maxden_ids=["laptop-hinge"], role="detail", material=b.dmat["laptop-lid"])
pts = [b.loc("screen", [p[0], p[1], 0]) for p in b.rings("elevation", "laptop-lid")[0]]
us = [p[0] for p in pts]; vs = [p[1] for p in pts]
b.boxl("laptop-lid", "screen", ((max(us) + min(us)) / 2, (max(vs) + min(vs)) / 2, 0), (max(us) - min(us), max(vs) - min(vs), 0.34), corner=0.004, src="laptop-lid", maxden_ids=["laptop-lid"])
b.dec("The five-star chair base is five round-section arms from the hub (a revolve) to ball castors with a dome yoke and a stem each, set at 0, 71, 144 degrees and mirrored as the plan draws them (castor circle 0.29 to 0.30 m), column sleeve, collar, gas cylinder and cone are revolves and a capsule on the vertical axis; seat cushion and backrest shell are rounded-rectangle lofts, the armrests two posts and two pads (one solid each with a repeat), the strut a capsule.",
      "Der fünfarmige Stuhlfuss besteht aus fünf Armen mit rundlichem Querschnitt von der Nabe (ein Rotationskörper) zu Kugelrollen mit Kuppelgabel und Stift, im Winkel von 0, 71 und 144 Grad und gespiegelt wie der Grundriss sie zeichnet (Rollenkreis 0,29 bis 0,30 m); Säulenhülse, Manschette, Gaszylinder und Konus sind Rotationskörper und eine Kapsel auf der senkrechten Achse; Sitzpolster und Rückenschale sind Lofts mit abgerundetem Rechteck, die Armlehnen zwei Pfosten und zwei Polster (je ein Körper mit Wiederholung), die Strebe eine Kapsel.")
b.dec("The desk is a 0.78 x 0.025 x 0.60 m top with a 0.51 m modesty panel under the far end and two 40 mm posts with foot plates (one solid each with a repeat); the laptop a 0.32 x 0.34 m base with a hinge bar and a lid plate on the screen bone. The figure is seated in the rig's rest pose: thigh lofts along the horizontal bones with the plan width, the shirt at 90 % of the front width, shoulder and knee balls, a shoulder yoke and the pelvis loft close the joints; the head outline is clipped at 0.142 m ahead and the nose is its own small ellipsoid.",
      "Der Schreibtisch ist eine Platte von 0,78 x 0,025 x 0,60 m mit einer 0,51 m breiten Blende unter dem fernen Ende und zwei Pfosten von 40 mm mit Fussplatten (je ein Körper mit Wiederholung); der Laptop ein Unterteil von 0,32 x 0,34 m mit Scharnierstab und einer Deckelplatte auf dem Bildschirmknochen. Die Figur sitzt in der Ruhehaltung des Rigs: Oberschenkel-Lofts entlang der waagrechten Knochen mit der Breite aus dem Grundriss, das Hemd mit 90 % der Breite der Vorderansicht, Schulter- und Knieskugeln, ein Schulterjoch und das Beckenloft schliessen die Gelenke; die Kopfkontur ist bei 0,142 m nach vorn abgeschnitten und die Nase ein eigenes kleines Ellipsoid.")
b.save()
