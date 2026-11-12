from lib import Builder, auto
b = Builder("walking-commuter")
auto(b, over={"sleeve-cuff-l": None, "sleeve-cuff-r": None, "coat-lapel": None, "coat-pocket": None,
    "coat": dict(front=["coat"], plan=["trunk"], latscale=0.76, n=10, exp=2.5),
    "hair": dict(plan=["head"], front=["hair", "head"]),
    "thumb-l": dict(lat=(0.14, 0.166)), "thumb-r": dict(lat=(-0.166, -0.14)),
    "pelvis-trousers": dict(front=[], plan=["waist"], n=4),
    "bag-strap-front": None, "bag-strap-back": None, "bag-flap": None, "bag-buckle": None,
    "coat-button-3": dict(front=["coat-button-3"])})
b.ell("nose", "head", (0.112, 1.665, 0), (0.017, 0.017, 0.011), material="skin", role="detail", tone=0.25, region="face-skin" if "face-skin" in b.ctx["regions"] else b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 1.44, 0), (0.115, 0.05, 0.175), material="wool-coat", role="body", tone=b.sh("coat")["tone"], region=b.sh("coat")["region"], maxden_ids=["coat"])
b.organic("bag-body", "bag", "bag-body", front=["bag-body"], plan=["bag"], n=5, exp=5.0, ends=0.9)
b.organic("bag-flap", "bag", "bag-flap", lat=(0.272, 0.292), n=4, exp=6.0, ends=0.95)
b.box("bag-buckle", "bag", (0, 0.94, 0.294), (0.03, 0.024, 0.008), corner=0.002, maxden_ids=["bag-buckle"])
b.polyline("bag-strap-front", "bag", [(0.0, 1.47, -0.09), (0.135, 1.40, -0.07), (0.145, 1.22, 0.06), (0.14, 1.03, 0.17)], 0.013, maxden_ids=["bag-strap-front"])
b.polyline("bag-strap-back", "bag", [(0.0, 1.47, -0.09), (-0.135, 1.40, -0.07), (-0.145, 1.22, 0.06), (-0.14, 1.03, 0.17)], 0.013, maxden_ids=["bag-strap-back"])
b.dec("The torso is the coat loft narrowed to 76 % of the front-view width (about 0.30 m): the front draws the coat as wide as the whole figure (0.40 m, arms included) while the plan draws the trunk only 0.25 m wide, so the arms stay visible beside it; the shoulder ellipsoid carries the plan shoulder width.",
      "Der Rumpf ist der Mantel-Loft, auf 76 % der Breite der Vorderansicht verschmälert (rund 0,30 m): die Vorderansicht zeichnet den Mantel so breit wie die ganze Figur (0,40 m samt Armen), der Grundriss den Rumpf nur 0,25 m breit, sodass die Arme daneben sichtbar bleiben; das Schulter-Ellipsoid trägt die Schulterbreite des Grundrisses.")
b.dec("Limbs, head, hands, shoes and scarf are lofts measured on the elevation outline (depth) and the front or plan twin (width); eyes, ears and coat buttons are ellipsoids that exist at 1:50 only. Lapels, pocket flaps, sleeve cuffs, fingers and laces are surface texture of the drawing and are not modelled.",
      "Gliedmassen, Kopf, Hände, Schuhe und Schal sind Lofts, deren Tiefe der Seitenansicht und deren Breite der Vorder- oder Grundrisszwilling entnommen ist; Augen, Ohren und Mantelknöpfe sind Ellipsoide, die es nur im Massstab 1:50 gibt. Revers, Taschenklappen, Ärmelbündchen, Finger und Schnürsenkel sind Oberflächentextur der Zeichnung und werden nicht modelliert.")
b.dec("The messenger bag is a rounded loft hanging from the torso bone at the left hip (width from the front view, depth from the plan) with a flap plate, a buckle and a strap that crosses the chest and the back as two round bars lying on the coat surface. The front view draws the bag 0.149 to 0.289 m left of the centre, where the forearm is drawn too (0.111 to 0.191 m): the bag overlaps the forearm by 4 cm, which is kept as drawn.",
      "Die Umhängetasche ist ein gerundeter Loft, am Rumpfknochen auf der linken Hüfte aufgehängt (Breite aus der Vorderansicht, Tiefe aus dem Grundriss), mit Klappenplatte, Schnalle und einem Riemen, der als zwei runde Stäbe auf der Manteloberfläche über Brust und Rücken läuft. Die Vorderansicht zeichnet die Tasche 0,149 bis 0,289 m links der Mitte, wo auch der Unterarm gezeichnet ist (0,111 bis 0,191 m): die Tasche überlappt den Unterarm um 4 cm, was wie gezeichnet belassen wird.")
b.save()
