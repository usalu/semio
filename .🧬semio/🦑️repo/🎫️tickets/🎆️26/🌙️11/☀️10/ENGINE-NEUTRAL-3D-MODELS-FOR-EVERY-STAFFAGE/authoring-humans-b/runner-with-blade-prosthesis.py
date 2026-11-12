from lib import Builder, auto
b = Builder("runner-with-blade-prosthesis")
auto(b, bones=set(__import__("lib").ORGANIC) | {"shank-r"}, over={
    "arm-l": dict(front=["upper-arm-l"], plan=["upper-arm-l"]), "arm-r": dict(front=["upper-arm-r"], plan=["upper-arm-r"]),
    "singlet": dict(front=["torso"], plan=["trunk-l", "trunk-r"], latscale=0.95, n=9, exp=2.5),
    "singlet-hem": dict(lat=(-0.125, 0.125), n=3, exp=2.5, ends=1.0), "bib": None,
    "pelvis-shorts": dict(front=["hips"], plan=["waist"], n=4, exp=2.4),
    "shoe-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], exp=2.6), "shoe-toe-l": dict(front=["shoe-toe-l"], plan=["shoe-toe-l"], n=4, exp=2.6),
    "shoe-sole": dict(front=["shoe-l"], plan=["shoe-heel-l"], n=5, exp=3.0, ends=0.9), "shoe-toe-sole": dict(front=["shoe-toe-l"], plan=["shoe-toe-l"], n=3, exp=3.0, ends=0.9),
    "head": dict(front=["head"], plan=["head"]), "hair": dict(front=["hair"], plan=["head"], n=6),
    "socket": dict(front=["socket"], plan=["socket"], n=7, exp=2.3), "socket-trim": dict(front=["socket"], plan=["socket"], n=3, exp=2.3, ends=1.0, grow=1.03),
    "socket-pyramid": None,
    "thumb-l": dict(lat=(0.10, 0.12)), "thumb-r": dict(lat=(-0.12, -0.10))})
Z = -0.075
b.box("socket-pyramid", "shank-r", (0, 0.3125, Z), (0.033, 0.04, 0.04), corner=0.004, src="socket", maxden_ids=["socket-pyramid"])
b.capsule("blade-bolt", "blade-r", (-0.002, 0.309, -0.1), (-0.002, 0.309, -0.05), 0.0055, src="blade-upper", maxden_ids=["blade-bolt"], role="detail")
b.loftw("blade-upper", "blade-r", [(0.002, 0.318, Z), (0.006, 0.26, Z), (0.0, 0.205, Z), (-0.022, 0.152, Z), (-0.05, 0.118, Z)], 0.031, [0.012, 0.0115, 0.0115, 0.011, 0.0105], exp=4.0)
b.loftw("blade-curl", "blade-tip-r", [(-0.05, 0.108, Z), (-0.03, 0.083, Z), (0.0, 0.066, Z), (0.045, 0.045, Z), (0.1, 0.029, Z), (0.17, 0.009, Z), (0.202, -0.004, Z)], 0.031, [0.0105, 0.0102, 0.0098, 0.0096, 0.0094, 0.0092, 0.008], exp=4.0)
b.loftw("blade-toe", "blade-tip-r", [(0.127, 0.03, Z), (0.168, 0.011, Z), (0.215, -0.009, Z)], 0.037, 0.011, exp=3.2)
b.ell("nose", "head", (0.12, 1.675, 0), (0.022, 0.017, 0.011), material="skin", role="detail", tone=b.sh("head")["tone"], region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 1.485, 0), (0.085, 0.045, 0.135), src="singlet", maxden_ids=["singlet"])
b.ell("pelvis", "pelvis", (0, 0.955, 0), (0.11, 0.085, 0.145), src="shorts-l", maxden_ids=["shorts-l"])
b.box("bib", "torso", (0.1, 1.247, 0), (0.18, 0.006, 0.2), corner=0.002, src="singlet", maxden_ids=["bib"])
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"knee-{s}", f"shank-{s}", (0, 0.498, z * 0.075), (0.052, 0.055, 0.058), src="shank-l", maxden_ids=["shank-l"])
b.dec("The right leg is a transtibial prosthesis: the shank bone carries the carbon socket (a loft with a trim ring and a pyramid adapter box) and the two blade bones carry the J-shaped leaf spring as lofts through hand-set centre points (60 mm wide, 19 to 24 mm thick, rounded-rectangle sections) with a rubber tread plate under the toe; the bolt is a bar across the blade. It ends 8 mm below the sole line, as the drawing does.",
      "Das rechte Bein ist eine Unterschenkelprothese: der Unterschenkelknochen trägt den Karbonschaft (ein Loft mit Randring und einem Pyramidenadapter als Quader), die beiden Klingenknochen tragen die J-förmige Blattfeder als Lofts durch von Hand gesetzte Mittelpunkte (60 mm breit, 19 bis 24 mm dick, Rechteckschnitte mit Rundung) mit einer Gummilaufplatte unter der Spitze; der Bolzen ist ein Stab quer durch die Klinge. Sie endet 8 mm unter der Sohlenlinie wie in der Zeichnung.")
b.dec("The left leg and the upper body are lofts measured on the elevation (depth) and on the front or plan twin (width); the singlet is narrowed to 95 % of the front width, the bib is a 6 mm plate on the chest, shoulder yoke and pelvis ellipsoids close the plan-only shoulders and waist. Eyes, ears and nose exist at 1:50 only; fist lines, laces and lamination are drawing texture.",
      "Das linke Bein und der Oberkörper sind Lofts, deren Tiefe der Seitenansicht und deren Breite dem Vorder- oder Grundrisszwilling entnommen ist; das Lauftop ist auf 95 % der Breite der Vorderansicht verschmälert, die Startnummer ist eine 6 mm dicke Platte auf der Brust, Schulterpartie und Becken als Ellipsoide schliessen die nur im Grundriss gezeichneten Schultern und die Taille. Augen, Ohren und Nase gibt es nur im Massstab 1:50; Faustlinien, Schnürsenkel und Laminierung sind Zeichnungstextur.")
b.save()
