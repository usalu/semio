from lib import Builder, auto, ORGANIC
b = Builder("tour-guide")
auto(b, over={"cap": None, "badge": None, "eye": dict(front=["eye-l", "eye-r"]),
    "head": dict(front=["neck", "head"], plan=["head"], xmax=0.103, n=9, exp=2.2), "hair": dict(front=["hair"], plan=["head"], n=6),
    "torso": dict(front=["torso"], plan=["trunk"], latscale=0.85, n=9, exp=2.5),
    "pelvis": dict(front=["pelvis"], plan=["waist"], n=5, exp=2.4),
    "shoe-toe-l": dict(front=["shoe-l"], plan=["shoe-l"], n=4, exp=2.6), "shoe-toe-r": dict(front=["shoe-r"], plan=["shoe-r"], n=4, exp=2.6),
    "shoe-l": dict(exp=2.6), "shoe-r": dict(exp=2.6),
    "thumb-l": dict(lat=(0.19, 0.212)), "thumb-r": dict(lat=(-0.212, -0.19))})
b.ell("nose", "head", (0.108, 0.705, 0), (0.016, 0.02, 0.011), material="skin", role="detail", tone=b.sh("head")["tone"], region=b.sh("head")["region"], maxden_ids=["nostril"])
b.ell("shoulders", "torso", (0, 0.53, 0), (0.1, 0.045, 0.19), src="torso", maxden_ids=["torso"])
b.add("cap", "head", "revolve", {"profile": [[0.179, 0.1], [0.203, 0.1], [0.233, 0.095], [0.263, 0.08], [0.288, 0.05], [0.301, 0.0]], "axis": [1, 0, 0], "origin": [0, 0.007, 0], "lateralScale": 0.93}, b.meta("cap"), ["cap"])
b.loftw("cap-visor", "head", [(0.085, 0.768, 0), (0.12, 0.762, 0), (0.15, 0.757, 0), (0.173, 0.752, 0)], [0.085, 0.078, 0.063, 0.035], 0.0055, exp=3.0, src="cap", maxden_ids=["cap"])
b.box("badge", "torso", (0.125, 0.39, 0.075), (0.075, 0.006, 0.088), corner=0.002, src="badge", maxden_ids=["badge"])
b.capsule("pointer", "pointer", (0, -0.149, -0.201), (0, -0.643, -0.201), 0.0055, maxden_ids=["pointer"])
b.capsule("pointer-tip", "pointer", (0, -0.643, -0.201), (0, -0.7, -0.201), 0.006, maxden_ids=["pointer-tip"], role="detail")
b.capsule("pole", "flag-pole", (0.0015, 0.185, 0.201), (0.0015, -1.178, 0.201), 0.0125, maxden_ids=["pole"])
b.capsule("pole-grip", "hand-l", (0.0015, -0.027, 0.201), (0.0015, -0.322, 0.201), 0.0145, src="pole", maxden_ids=["pole"])
b.ell("pole-finial", "flag-pole", (0.0, -1.184, 0.201), (0.016, 0.016, 0.016), src="pole-finial", maxden_ids=["pole-finial"])
b.box("flag", "flag", (0.226, -0.9525, 0.201), (0.468, 0.301, 0.016), src="flag", maxden_ids=["flag"])
b.ell("flag-emblem", "flag", (0.22, -0.954, 0.2095), (0.07, 0.07, 0.0025), src="flag-emblem", maxden_ids=["flag-emblem"], repeat={"count": 2, "translation": [0, 0, -0.017]})
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"upper-arm-{s}", (0, 0.527, z * 0.201), (0.056, 0.05, 0.052), src=f"upper-arm-{s}", maxden_ids=[f"upper-arm-{s}"])
b.dec("The torso loft is 85 % of the front-view width (about 0.36 m): the front draws the jacket as wide as the sleeves reach (0.42 m), the plan only 0.25 m, so a shoulder yoke ellipsoid carries the plan shoulder width and the sleeves stay visible as separate arms (a shoulder ball on each upper-arm bone keeps the arm joined to the yoke when it is raised). The figure has no neck bone: head and neck are one loft whose neck part is 0.108 m wide as the front neck, and the face outline is clipped at 0.103 m ahead so the nose becomes its own small ellipsoid.",
      "Der Rumpf-Loft ist 85 % der Breite der Vorderansicht (rund 0,36 m): die Vorderansicht zeichnet die Jacke so breit, wie die Ärmel reichen (0,42 m), der Grundriss nur 0,25 m, daher trägt ein Schulter-Ellipsoid die Schulterbreite des Grundrisses und die Ärmel bleiben als eigene Arme sichtbar (eine Schulterkugel an jedem Oberarmknochen hält den Arm beim Anheben am Joch). Die Figur hat keinen Halsknochen: Kopf und Hals sind ein Loft, dessen Halsteil 0,108 m breit ist wie der Hals der Vorderansicht, und die Gesichtskontur ist bei 0,103 m nach vorn abgeschnitten, damit die Nase ein eigenes kleines Ellipsoid wird.")
b.dec("The cap is a revolve about the head axis (crown, 0.19 m wide) plus a visor loft that narrows from 0.17 to 0.07 m wide over 8.8 cm although the plan view draws the cap only 0.114 m ahead; the elevation visor is followed. The pointer is a 5.5 mm aluminium rod with a tip, the flag pole a 25 mm rod with a grip over the left hand and a finial, the flag a 468 x 301 x 16 mm cloth box on its own bone with a thin emblem disc on both faces.",
      "Die Kappe ist ein Rotationskörper um die Kopfachse (Krone, 0,19 m breit) plus ein Schirm-Loft, der sich über 8,8 cm von 0,17 auf 0,07 m verengt, obwohl der Grundriss die Kappe nur 0,114 m nach vorn zeichnet; verfolgt wird der Schirm der Seitenansicht. Der Zeigestab ist ein 5,5-mm-Aluminiumstab mit Spitze, die Fahnenstange ein 25-mm-Stab mit Griff über der linken Hand und einem Knauf, die Fahne ein Stoffquader von 468 x 301 x 16 mm auf eigenem Knochen mit dünner Emblemscheibe auf beiden Seiten.")
b.save()
