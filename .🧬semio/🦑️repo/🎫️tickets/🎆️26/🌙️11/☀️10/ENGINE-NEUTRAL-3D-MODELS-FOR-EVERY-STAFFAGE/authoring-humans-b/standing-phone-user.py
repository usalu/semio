from lib import Builder, auto
b = Builder("standing-phone-user")
auto(b, over={"cuff-rib-l": None, "cuff-rib-r": None, "kangaroo-pocket": None, "headphone-cup": None,
    "hoodie": dict(front=["hoodie"], plan=["trunk"], latscale=1.0, n=9, exp=2.6),
    "hair": dict(plan=["head"], front=["hair", "head"]),
    "ponytail": dict(front=[], plan=["ponytail"], n=6, exp=2.2),
    "thumb-l": dict(lat=(0.14, 0.168)), "thumb-r": dict(lat=(-0.168, -0.14)),
    "hem-rib": dict(n=3, exp=2.6, ends=1.0), "hood": dict(front=["hood"], n=5, exp=2.2)})
b.ell("nose", "head", (0.104, 1.572, 0), (0.017, 0.017, 0.011), material="skin", role="detail", tone=0.25, region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 1.385, 0), (0.095, 0.045, 0.172), material="fleece-sweatshirt", role="body", tone=0.45, region=b.sh("hoodie")["region"], maxden_ids=["hoodie"])
b.ell("pelvis", "pelvis", (0, 0.865, 0), (0.095, 0.075, 0.155), material="denim", role="body", tone=0.62, region=b.sh("thigh-l")["region"], maxden_ids=["thigh-l"])
b.box("kangaroo-pocket", "torso", (0.092, 0.928, 0), (0.16, 0.016, 0.19), corner=0.005, maxden_ids=["kangaroo-pocket"])
b.ell("headphone-cup-l", "neck", (0.005, 1.445, 0.078), (0.025, 0.03, 0.026), src="headphone-cup", maxden_ids=["headphone-cup-l"])
b.ell("headphone-cup-r", "neck", (0.005, 1.445, -0.078), (0.025, 0.03, 0.026), src="headphone-cup", maxden_ids=["headphone-cup-r"])
b.boxl("phone", "phone", (0, 0, 0), (0.148, 0.010, 0.074), corner=0.004, maxden_ids=["phone"])
b.boxl("phone-screen", "phone", (0, 0.0055, 0), (0.13, 0.0012, 0.06), maxden_ids=["phone-screen"])
b.dec("The figure is the hooded-sweatshirt body of the standing adult: every limb, the torso, head, hands and shoes are lofts measured on the elevation outline (depth) and the front or plan twin (width), the pelvis and shoulder yoke are ellipsoids that close the plan-only shoulders and waist, the hair, the ponytail (width from the plan) and the hood are separate lofts.",
      "Die Figur ist der Kapuzenpullover-Körper des stehenden Erwachsenen: alle Gliedmassen, Rumpf, Kopf, Hände und Schuhe sind Lofts, deren Tiefe der Seitenansicht und deren Breite dem Vorder- oder Grundrisszwilling entnommen ist; Becken und Schulterpartie sind Ellipsoide, die die nur im Grundriss gezeichneten Schultern und die Taille schliessen; Haar, Pferdeschwanz (Breite aus dem Grundriss) und Kapuze sind eigene Lofts.")
b.dec("The headphones hang around the neck: the two cups are ellipsoids at the collar (the elevation draws them at 1.43 to 1.48 m, the front at 1.39 to 1.45 m; the mean height is used), the phone is a rounded 148 x 74 x 10 mm slab on its own bone in the right hand with a thin screen plate on the side that faces the face; ribs, laces and finger lines are drawing texture and are not modelled.",
      "Die Kopfhörer hängen um den Hals: die beiden Muscheln sind Ellipsoide am Kragen (die Seitenansicht zeichnet sie auf 1,43 bis 1,48 m, die Vorderansicht auf 1,39 bis 1,45 m; verwendet wird die mittlere Höhe), das Telefon ist eine gerundete Platte von 148 x 74 x 10 mm auf eigenem Knochen in der rechten Hand mit dünner Displayplatte auf der dem Gesicht zugewandten Seite; Bündchen, Schnürsenkel und Fingerlinien sind Zeichnungstextur und werden nicht modelliert.")
b.save()
