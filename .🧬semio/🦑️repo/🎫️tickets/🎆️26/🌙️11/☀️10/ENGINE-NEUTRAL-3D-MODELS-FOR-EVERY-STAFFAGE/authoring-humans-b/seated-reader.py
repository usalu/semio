from lib import Builder, auto, ORGANIC
b = Builder("seated-reader")
auto(b, over={"sweater-hem": dict(lat=(-0.145, 0.145), n=3, exp=2.5, ends=1.0), "cuff-l": None, "cuff-r": None, "eye": dict(front=["eye-l", "eye-r"]),
    "head": dict(front=["head"], plan=["face"], xmax=0.1, n=9, exp=2.2), "hair": dict(front=["hair"], plan=["head"], n=6),
    "cowl": dict(lat=(-0.065, 0.065), n=4, exp=2.2),
    "sweater": dict(front=["torso"], plan=["trunk-l", "trunk-r"], latscale=0.9, n=9, exp=2.5),
    "pelvis-jeans": dict(front=["hips"], plan=["waist"], n=4, exp=2.4),
    "sleeve-l": dict(front=["upper-arm-l"], plan=["upper-arm-l"], n=6), "sleeve-r": dict(front=["upper-arm-r"], plan=["upper-arm-r"], n=6),
    "shoe-rear-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], exp=2.6), "shoe-rear-r": dict(front=["shoe-r"], plan=["shoe-heel-r"], exp=2.6),
    "thumb-l": dict(lat=(0.128, 0.152), n=3), "thumb-r": dict(lat=(-0.152, -0.128), n=3)},
    bones=set(ORGANIC))
b.ell("nose", "head", (0.108, 1.165, 0), (0.017, 0.017, 0.011), material="skin", role="detail", tone=0.25, region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 0.99, 0), (0.095, 0.04, 0.16), src="sweater", maxden_ids=["sweater"])
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"arm-{s}", (0, 0.985, z * 0.14), (0.06, 0.045, 0.055), src=f"sleeve-{s}", maxden_ids=[f"sleeve-{s}"])
    b.ell(f"knee-{s}", f"shank-{s}", (0.41, 0.47, z * 0.085), (0.06, 0.057, 0.061), src=f"shank-{s}", maxden_ids=[f"shank-{s}"])
b.extrude("bench-seat", "seat", "bench-seat", (-0.25, 0.25), tol=0.002)
b.extrude("bench-back", "seat", "bench-back", (-0.25, 0.25), tol=0.002)
M = b.meta("bench-leg-front")
b.box("bench-leg-front", "seat", (0.145, 0.19, 0.215), (0.05, 0.38, 0.05), mt=M, repeat={"count": 2, "translation": [0, 0, -0.43]}, maxden_ids=["bench-leg-front"])
b.box("bench-leg-rear", "seat", (-0.225, 0.19, 0.215), (0.05, 0.38, 0.05), mt=b.meta("bench-leg-rear"), repeat={"count": 2, "translation": [0, 0, -0.43]}, maxden_ids=["bench-leg-rear"])
b.add("cup-body", "cup", "revolve", {"profile": [[0.0, 0.04], [0.105, 0.0335]], "axis": [1, 0, 0], "origin": [0, 0, 0]}, b.meta("cup-body"), ["cup-body"])
b.add("cup-lid", "cup", "revolve", {"profile": [[0.105, 0.0345], [0.108, 0.037], [0.118, 0.037], [0.12, 0.033]], "axis": [1, 0, 0], "origin": [0, 0, 0]}, b.meta("cup-lid"), ["cup-lid"])
b.add("cup-sleeve", "cup", "revolve", {"profile": [[0.03, 0.0425], [0.075, 0.0415]], "axis": [1, 0, 0], "origin": [0, 0, 0]}, b.meta("cup-sleeve", src="cup-body", role="detail", material="paper-cup"), ["cup-sleeve"])
b.box("pad-cover", "pad", (0.008, 0.155, 0.14), (0.2, 0.016, 0.15), corner=0.003, maxden_ids=["pad-cover"])
b.box("pad-pages", "pad", (-0.006, 0.155, 0.14), (0.2, 0.012, 0.145), maxden_ids=["pad-pages"])
b.box("page-sheet", "page", (-0.014, 0.15, 0.14), (0.19, 0.004, 0.14), maxden_ids=["page-sheet"])
b.dec("The figure sits already in the rig's rest pose (thighs horizontal on the bench, shanks down), so the volume follows the 1:50 drawing directly: the thighs are lofts along the horizontal bones with the width from the plan, the sweater is 90 % of the front width, the cowl a loft around the neck, the hair bun an ellipsoid; shoulder balls, knee balls, the shoulder yoke and the pelvis loft close the joints.",
      "Die Figur sitzt schon in der Ruhehaltung des Rigs (Oberschenkel waagrecht auf der Bank, Unterschenkel nach unten), sodass das Volumen der Zeichnung 1:50 direkt folgt: die Oberschenkel sind Lofts entlang der waagrechten Knochen mit der Breite aus dem Grundriss, der Pullover ist 90 % der Breite der Vorderansicht, der Rollkragen ein Loft um den Hals, der Haarknoten ein Ellipsoid; Schulterkugeln, Knieskugeln, Schulterjoch und das Beckenloft schliessen die Gelenke.")
b.dec("The bench is a 0.50 x 0.48 x 0.04 m seat slab and a slanted back plate (extrusions of the side outline), four 50 mm square legs (two solids with a repeat), the slat lines are surface texture and not modelled. The cup is a revolved tapered body with a lid and a sleeve band; its radius is 0.04 m because front and plan draw it 0.08 m wide while the side view draws it only 0.06 m (the side view is the outlier). The pad and its page are thin boxes held upright in the left hand.",
      "Die Bank ist eine Sitzplatte von 0,50 x 0,48 x 0,04 m und eine geneigte Rückenplatte (Extrusionen der Seitenkontur) auf vier quadratischen Beinen von 50 mm (zwei Körper mit Wiederholung), die Latten sind Oberflächentextur und werden nicht modelliert. Der Becher ist ein rotierter, sich verjüngender Körper mit Deckel und Manschette; sein Radius beträgt 0,04 m, weil Vorderansicht und Grundriss ihn 0,08 m breit zeichnen, die Seitenansicht nur 0,06 m (die Seitenansicht ist der Ausreisser). Block und Blatt sind dünne Quader, die in der linken Hand aufrecht gehalten werden.")
b.save()
