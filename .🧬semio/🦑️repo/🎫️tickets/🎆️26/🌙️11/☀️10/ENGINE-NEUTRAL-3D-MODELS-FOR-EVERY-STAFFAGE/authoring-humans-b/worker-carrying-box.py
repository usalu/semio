from lib import Builder, auto
b = Builder("worker-carrying-box")
auto(b, over={"helmet": None, "box": None, "box-tape": None, "box-handle": None, "knee-pad-r": None, "vest-band-low": None, "vest-band-high": None,
    "eye": dict(front=["eye-l", "eye-r"]),
    "head": dict(front=["neck", "head"], plan=["head"], xmax=0.108, n=9, exp=2.2), "hair": dict(front=["hair"], plan=["head"], n=6),
    "torso": dict(front=["torso"], plan=["trunk"], latscale=0.82, n=9, exp=2.5),
    "vest": dict(front=["vest"], plan=["trunk"], latscale=0.87, n=9, exp=2.5),
    "pelvis": dict(front=["pelvis"], plan=["waist"], n=5, exp=2.4),
    "shoe-toe-l": dict(front=["shoe-l"], plan=["shoe-l"], n=4, exp=2.6), "shoe-toe-r": dict(front=["shoe-r"], plan=["shoe-r"], n=4, exp=2.6),
    "shoe-l": dict(exp=2.6), "shoe-r": dict(exp=2.6),
    "thumb-l": dict(lat=(0.19, 0.212)), "thumb-r": dict(lat=(-0.212, -0.19))})
b.ell("nose", "head", (0.112, 0.708, 0), (0.016, 0.02, 0.011), material="skin", role="detail", tone=b.sh("head")["tone"], region=b.sh("head")["region"], maxden_ids=["nostril"])
b.ell("shoulders", "torso", (0, 0.54, 0), (0.1, 0.045, 0.19), src="torso", maxden_ids=["torso"])
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"upper-arm-{s}", (0, 0.536, z * 0.201), (0.056, 0.05, 0.052), src=f"upper-arm-{s}", maxden_ids=[f"upper-arm-{s}"])
for sid in ("vest-band-low", "vest-band-high"):
    b.organic(sid, "torso", sid, lat=(-0.172, 0.172), n=3, exp=2.5, ends=1.0, grow=1.0)
b.add("helmet", "head", "revolve", {"profile": [[0.168, 0.119], [0.2, 0.119], [0.23, 0.113], [0.255, 0.101], [0.277, 0.085], [0.295, 0.065], [0.308, 0.04], [0.316, 0.013], [0.32, 0.0]], "axis": [1, 0, 0], "origin": [0, -0.003, 0], "lateralScale": 0.85}, b.meta("helmet"), ["helmet"])
b.loftw("helmet-peak", "head", [(0.118, 0.795, 0), (0.15, 0.791, 0), (0.175, 0.786, 0), (0.187, 0.783, 0)], [0.068, 0.06, 0.042, 0.02], 0.0055, exp=3.0, src="helmet", maxden_ids=["helmet"])
b.box("knee-pad-r", "shank-r", (0.056, -0.47, -0.093), (0.096, 0.026, 0.07), corner=0.01, src="knee-pad-r", maxden_ids=["knee-pad-r"])
b.box("box", "box-r", (0.0, -0.127, 0.0), (0.4, 0.3, 0.4), corner=0.004, src="box", maxden_ids=["box"])
b.box("box-tape", "box-r", (0.095, -0.127, 0.0), (0.404, 0.04, 0.404), src="box-tape", maxden_ids=["box-tape"])
b.box("box-handle", "box-r", (0.0075, -0.1265, -0.2), (0.151, 0.045, 0.004), corner=0.002, src="box-handle", maxden_ids=["box-handle"], repeat={"count": 2, "translation": [0, 0, 0.4]})
b.dec("The torso and the high-visibility vest are two lofts, the vest 5 % wider, each narrowed to 82 and 87 % of the front-view width (the front draws the jacket as wide as the sleeves reach, 0.43 m; the plan draws the trunk 0.25 m); reflective bands are short bands around the vest, a shoulder yoke ellipsoid carries the plan shoulder width and a shoulder ball on each upper-arm bone keeps the raised arms joined to it. Head and neck are one loft (the figure has no neck bone) whose face outline is clipped at 0.108 m ahead so that the nose is its own small ellipsoid.",
      "Rumpf und Warnweste sind zwei Lofts, die Weste 5 % breiter, auf 82 und 87 % der Breite der Vorderansicht verschmälert (die Vorderansicht zeichnet die Jacke so breit, wie die Ärmel reichen, 0,43 m; der Grundriss den Rumpf 0,25 m); die Reflexstreifen sind kurze Bänder um die Weste, ein Schulter-Ellipsoid trägt die Schulterbreite des Grundrisses und eine Schulterkugel an jedem Oberarmknochen hält die angehobenen Arme daran fest. Kopf und Hals sind ein Loft (die Figur hat keinen Halsknochen), dessen Gesichtskontur bei 0,108 m nach vorn abgeschnitten ist, damit die Nase ein eigenes kleines Ellipsoid wird.")
b.dec("The hard hat is a revolve about the head axis (crown 0.24 m wide in the side view, 0.20 m in the front, hence lateralScale 0.85) with a peak loft; the knee pad is a rounded plate on the front of the right shin. The carton hangs on the right-hand bone (the left box bone stays empty) as a 0.40 x 0.30 x 0.40 m rounded box, its tape a 40 mm band around it (side view and front view draw a band, the plan draws only a 4 cm square: the plan is the outlier) and its handle slots as two thin plates on the side faces.",
      "Der Schutzhelm ist ein Rotationskörper um die Kopfachse (Kalotte 0,24 m breit in der Seitenansicht, 0,20 m in der Vorderansicht, daher lateralScale 0,85) mit einem Schirm-Loft; das Knieschützer ist eine gerundete Platte auf der Vorderseite des rechten Schienbeins. Der Karton hängt am Knochen der rechten Hand (der linke Kartonknochen bleibt leer) als gerundeter Quader von 0,40 x 0,30 x 0,40 m, sein Klebeband ist ein 40 mm breites Band darum (Seiten- und Vorderansicht zeichnen ein Band, der Grundriss nur ein 4-cm-Quadrat: der Grundriss ist der Ausreisser) und die Griffschlitze sind zwei dünne Platten auf den Seitenflächen.")
b.save()
