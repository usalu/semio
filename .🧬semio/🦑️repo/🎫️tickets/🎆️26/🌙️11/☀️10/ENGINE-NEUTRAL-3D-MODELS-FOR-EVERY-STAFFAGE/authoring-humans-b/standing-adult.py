from lib import Builder
b = Builder("standing-adult")
O, B = b.organic, b.blob
O("head", "head", "head", n=8, exp=2.2)
O("hair", "head", "hair", plan=["head"], lat=(-0.078, 0.078), n=6, exp=2.2)
for s, sg in (("l", 1), ("r", -1)):
    B(f"ear-{s}", "head", "ear", front=[f"ear-{s}"], plan=[f"ear-{s}"], maxden_ids=["ear-l"])
    B(f"eye-{s}", "head", "eye", front=[f"eye-{s}"], lat=(0.023, 0.041) if sg > 0 else (-0.041, -0.023), maxden_ids=["eye-l"])
b.ell("nose", "head", (0.104, 1.572, 0), (0.017, 0.017, 0.011), material="skin", role="detail", tone=0.25, region="face-skin", maxden_ids=["nose"])
O("neck", "neck", "neck", front=["neck"], plan=["neckwear"], n=5, exp=2.1)
O("hoodie", "torso", "hoodie", n=9, exp=2.6)
O("hood", "torso", "hood", front=["hood"], n=5, exp=2.2)
O("hem-rib", "torso", "hem-rib", front=["hem-rib"], n=3, exp=2.6, ends=1.0)
b.ell("shoulders", "torso", (0, 1.385, 0), (0.095, 0.045, 0.172), material="fleece-sweatshirt", role="body", tone=0.45, region="hoodie", maxden_ids=["hoodie"])
b.box("kangaroo-pocket", "torso", (0.092, 0.928, 0), (0.16, 0.016, 0.19), corner=0.005, maxden_ids=["kangaroo-pocket"])
for i, z in enumerate((-0.042, 0.042)):
    b.capsule(f"drawstring-{i}", "torso", (0.092, 1.37, z), (0.094, 1.20, z), 0.0035, maxden_ids=["drawstring-0"])
b.ell("pelvis", "pelvis", (0, 0.865, 0), (0.095, 0.075, 0.155), material="denim", role="body", tone=0.62, region="jeans", maxden_ids=["thigh-l"])
for s in "lr":
    O(f"sleeve-{s}", f"arm-{s}", f"sleeve-{s}", n=6, exp=2.2)
    O(f"forearm-{s}", f"forearm-{s}", f"forearm-{s}", n=6, exp=2.2)
    O(f"hand-{s}", f"hand-{s}", f"hand-{s}", n=5, exp=2.1)
    sg = 1 if s == "l" else -1
    B(f"thumb-{s}", f"hand-{s}", f"thumb-{s}", lat=(sg * 0.14, sg * 0.168) if sg > 0 else (-0.168, -0.14))
    O(f"thigh-{s}", f"thigh-{s}", f"thigh-{s}", n=7, exp=2.2)
    O(f"shank-{s}", f"shank-{s}", f"shank-{s}", n=7, exp=2.2)
    O(f"shoe-{s}", f"foot-{s}", f"shoe-{s}", plan=[f"shoe-heel-{s}"], front=[f"shoe-{s}"], n=6, exp=2.6)
    O(f"shoe-toe-{s}", f"toe-{s}", f"shoe-toe-{s}", plan=[f"shoe-toe-{s}"], n=4, exp=2.6)
    O(f"sole-{s}", f"foot-{s}", f"sole-{s}", plan=[f"shoe-heel-{s}"], n=5, exp=3.0, ends=0.9)
    O(f"toe-sole-{s}", f"toe-{s}", f"toe-sole-{s}", plan=[f"shoe-toe-{s}"], n=3, exp=3.0, ends=0.9)
b.decisions = [
    {"en": "Limbs, torso, head, hands and shoes are lofts of superelliptic sections whose depth and height are measured on the elevation outline and whose lateral width and centre are read from the front view (plan view for the near-horizontal foot), so the three silhouettes agree while every part is a rounded body instead of a view extrusion.",
     "de": "Gliedmassen, Rumpf, Kopf, Hände und Schuhe sind Lofts aus superelliptischen Schnitten; Tiefe und Höhe sind der Seitenansicht entnommen, seitliche Breite und Lage der Vorderansicht (beim fast waagrechten Fuss dem Grundriss), sodass die drei Silhouetten übereinstimmen und jedes Teil ein gerundeter Körper statt einer Ansichtsextrusion ist."},
    {"en": "The hood is a loft behind the neck that is as wide as the front view draws it (about 0.26 m) although the plan draws only the part not covered by the shoulders; the shoulder yoke and the denim pelvis are ellipsoids that the plan view shows (shoulders 0.37 m wide, waist 0.33 m) and that close the gaps between torso, arms and thighs.",
     "de": "Die Kapuze ist ein Loft hinter dem Hals, so breit wie die Vorderansicht sie zeichnet (etwa 0,26 m), obwohl der Grundriss nur den von den Schultern nicht verdeckten Teil zeigt; Schulterpartie und Jeansbecken sind Ellipsoide, die der Grundriss zeigt (Schultern 0,37 m breit, Taille 0,33 m) und die die Lücken zwischen Rumpf, Armen und Oberschenkeln schliessen."},
    {"en": "Eyes, ears, nose, the kangaroo pocket and the drawstrings are details that exist at 1:50 only; laces, finger lines, collar rib and cuff ribs are surface texture of the 2D drawing and are not modelled. The elevation draws the hood back 0.136 m behind and the toe 0.219 m ahead of the pelvis, 0.355 m in all, 8 % more than the authored widthM of 0.328 m; the 3D body follows the drawing, the rounded hood end leaves the box inside the 8 % gate.",
     "de": "Augen, Ohren, Nase, Känguru-Tasche und Kordeln sind Details, die es nur im Massstab 1:50 gibt; Schnürsenkel, Fingerlinien, Kragen- und Bündchenrippen sind Oberflächentextur der 2D-Zeichnung und werden nicht modelliert. Die Seitenansicht zeichnet die Kapuze 0,136 m hinter und die Schuhspitze 0,219 m vor dem Becken, zusammen 0,355 m, 8 % mehr als die geschriebene widthM von 0,328 m; der 3D-Körper folgt der Zeichnung, das gerundete Kapuzenende hält die Box innerhalb der 8-%-Grenze."},
]
b.save()
