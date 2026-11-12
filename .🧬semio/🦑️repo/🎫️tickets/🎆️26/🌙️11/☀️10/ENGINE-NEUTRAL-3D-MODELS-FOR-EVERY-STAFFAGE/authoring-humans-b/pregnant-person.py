from lib import Builder, auto, ORGANIC
b = Builder("pregnant-person")
auto(b, over={"cuff-rib-l": None, "cuff-rib-r": None,
    "sweater": dict(front=["torso"], plan=["trunk-l", "trunk-r"], latscale=0.9, n=9, exp=2.5),
    "sweater-hem-rib": dict(lat=(-0.155, 0.155), n=3, exp=2.5, ends=1.0),
    "belly-cover": None,
    "boot-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], exp=2.6), "boot-r": dict(front=["shoe-r"], plan=["shoe-heel-r"], exp=2.6),
    "boot-toe-l": dict(front=["shoe-toe-l"], plan=["shoe-toe-l"], n=4, exp=2.6), "boot-toe-r": dict(front=["shoe-toe-r"], plan=["shoe-toe-r"], n=4, exp=2.6),
    "boot-sole-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], n=5, exp=3.0, ends=0.9), "boot-sole-r": dict(front=["shoe-r"], plan=["shoe-heel-r"], n=5, exp=3.0, ends=0.9),
    "boot-toe-sole-l": dict(front=["shoe-toe-l"], plan=["shoe-toe-l"], n=3, exp=3.0, ends=0.9), "boot-toe-sole-r": dict(front=["shoe-toe-r"], plan=["shoe-toe-r"], n=3, exp=3.0, ends=0.9),
    "head": dict(front=["head"], plan=["head"]), "hair": dict(front=["hair"], plan=["head"], n=6),
    "thumb-l": dict(lat=(0.14, 0.168)), "thumb-r": dict(lat=(-0.168, -0.14))})
b.blob("belly", "belly", "belly-cover", front=["belly"], plan=["belly"], tilt=False, maxden_ids=["belly-cover"])
b.ell("nose", "head", (0.105, 1.54, 0), (0.017, 0.017, 0.011), material="skin", role="detail", tone=b.sh("head")["tone"], region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 1.37, 0), (0.095, 0.045, 0.172), src="sweater", maxden_ids=["sweater"])
b.ell("pelvis", "pelvis", (0, 0.895, 0), (0.105, 0.085, 0.165), src="thigh-l", maxden_ids=["thigh-l"])
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"arm-{s}", (0, 1.351, z * 0.152), (0.064, 0.048, 0.048), src=f"sleeve-{s}", maxden_ids=[f"sleeve-{s}"])
    b.ell(f"elbow-{s}", f"forearm-{s}", (0, 1.041, z * 0.152), (0.047, 0.04, 0.04), src=f"forearm-{s}", maxden_ids=[f"forearm-{s}"])
    b.ell(f"fist-{s}", f"hand-{s}", (0.005, 0.796, z * 0.152), (0.03, 0.03, 0.031), src=f"hand-{s}", maxden_ids=[f"hand-{s}"])
    b.ell(f"knee-{s}", f"shank-{s}", (0, 0.462, z * 0.09), (0.05, 0.06, 0.064), src=f"shank-{s}", maxden_ids=[f"shank-{s}"])
b.dec("The belly is its own ellipsoid on the belly bone, 0.24 m ahead of and 0.38 m high on the torso and 0.30 m wide (the front and plan ellipses agree), covered by the same knit as the sweater loft; the sweater itself is narrowed to 90 % of the front width so the arms stay visible beside it.",
      "Der Bauch ist ein eigenes Ellipsoid auf dem Bauchknochen, 0,24 m vor dem Rumpf, 0,38 m hoch und 0,30 m breit (Vorder- und Grundrissellipse stimmen überein), vom selben Strick wie der Pullover-Loft bedeckt; der Pullover selbst ist auf 90 % der Breite der Vorderansicht verschmälert, damit die Arme daneben sichtbar bleiben.")
b.dec("Limbs, head, hands and boots are lofts of superelliptic sections measured on the elevation (depth) and on the front or plan twin (width); shoulder, elbow, knee and fist ellipsoids, the shoulder yoke and the pelvis close the joints the front view draws as round parts and the plan view draws as a waist ellipse. Eyes, ears and nose exist at 1:50 only; cuff ribs, finger lines and the collar are drawing texture.",
      "Gliedmassen, Kopf, Hände und Stiefel sind Lofts aus superelliptischen Schnitten, deren Tiefe der Seitenansicht und deren Breite dem Vorder- oder Grundrisszwilling entnommen ist; Schulter-, Ellbogen-, Knie- und Faust-Ellipsoide, Schulterpartie und Becken schliessen die Gelenke, die die Vorderansicht als runde Teile und der Grundriss als Taillenellipse zeichnet. Augen, Ohren und Nase gibt es nur im Massstab 1:50; Bündchen, Fingerlinien und Kragen sind Zeichnungstextur.")
b.save()
