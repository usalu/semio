from body import *
from decisions import *
A = Asset("person-cooking")
names = list(A.draft)
FOLD = {k for k in names if any(t in k for t in ("fold", "crease", "nostril", "jaw-line", "lower-lip"))}
SK = FOLD | {"neck", "hoodie", "hood", "head", "hair", "ear", "hem-rib", "kangaroo-pocket", "drawstring-0", "drawstring-1", "collar-rib", "jeans-hem-l", "jeans-hem-r", "eye", "thumb-l", "thumb-r", "cuff-rib-l", "cuff-rib-r"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
tw = front_halfwidth(A, "torso", "torso")
hw = lambda u: tw(u) if u >= 0 else 0.175
A.add(loft_from(A.draft["hoodie"], rxfn=hw, amax=2.6, exp=2.5), like="hoodie")
A.add(loft_from(A.draft["hem-rib"], rxfn=hw, amax=2.6, exp=2.5), like="hem-rib")
A.add(loft_from(A.draft["hood"], lat=(-0.09, 0.09), exp=2.2), like="hood")
A.add(loft_from(A.draft["neck"], lat=(-0.05, 0.05), exp=2.0), like="neck")
hd = loft_from(A.draft["head"], lat=(-0.072, 0.072), exp=2.0)
A.add(hd, like="head")
A.add(loft_from(A.draft["hair"], lat=(-0.075, 0.075), exp=2.0), like="hair")
for sd in "lr":
    thumb(A, f"thumb-{sd}", f"hand-{sd}", glove=f"hand-{sd}")
    A.add(loft_from(A.draft[f"cuff-rib-{sd}"], exp=2.2), like="cuff-rib")
    band(A, f"jeans-hem-{sd}", f"shank-{sd}", 0.37, 0.382, 0.003, "denim", 0.1, "jeans", like="jeans-hem")
A.add(loft("pelvis", "pelvis", [sec(-0.058, 0.15, 0.105, e=2.4), sec(0.02, 0.167, 0.114, e=2.4), sec(0.095, 0.16, 0.11, e=2.4)], material="denim", role="body", tone=0.62, region="jeans"), like="pelvis")
hj = get(A, "hoodie")
patch(A, "kangaroo-pocket", "torso", -0.05, surface_v(hj, -0.05, 0.0), 0.0, [0.16, 0.012, 0.3], "kangaroo-pocket", "fleece-sweatshirt", 0.3, "hoodie")
for i, z in enumerate((-0.03, 0.03)):
    A.add(capsule(f"drawstring-{i}", "torso", [0.22, surface_v(hj, 0.22, z) + 0.002, z], [0.4, surface_v(hj, 0.4, z) + 0.002, z], 0.004, material="fleece-sweatshirt", role="detail", tone=0.1, region="hoodie"), like="drawstring", maxden=50)
band(A, "collar-rib", "hoodie", 0.4, 0.442, 0.003, "fleece-sweatshirt", 0.1, "hoodie", like="collar-rib", maxden=100)
eyes(A, u=0.083, w=0.03, rx=0.008, ry=0.006, rz=0.005, material="skin", tone=0.9, region="face-skin")
A.add(ellipsoid("nose", "head", [0.014, 0.011, 0.009], [0.065, surface_v(hd, 0.065, 0.0) + 0.003, 0.0], material="skin", role="detail", tone=0.8, region="face-skin"), like="nose", maxden=50)
for sd, z in (("l", 1), ("r", -1)):
    A.add(ellipsoid(f"ear-{sd}", "head", [0.015, 0.02, 0.008], [0.045, 0.03, z * 0.075], material="skin", role="detail", tone=0.7, region="face-skin"), like="ear")
deltoids(A, ["sleeve-l", "sleeve-r"])
for sd in "lr":
    pass
C = "counter"
CM = dict(material="laminate-chipboard", role="body", region="counter")
box_of(A, "counter-plinth")
box_of(A, "counter-body")
A.add(box("counter-panel", C, [0.012, 0.68, 0.54], [0.598, 0.48, 0.0], corner=0.003, material="laminate-chipboard", role="detail", tone=0.1, region="counter"), like="counter-panel", maxden=100)
box_of(A, "worktop", corner=0.003)
box_of(A, "hob-plate", corner=0.002)
for i, (u, r) in enumerate(((0.32, 0.075), (0.18, 0.05))):
    ring = revolve(f"burner-ring-{i + 1}", C, [[0.0, 0.0], [0.0, r], [0.004, r], [0.004, 0.0]], axis=(0, 1, 0), material="glass-ceramic", role="detail", tone=0.5, region="hob")
    ring["transform"] = {"translation": [u, 0.908, -0.14]}
    ring["repeat"] = {"count": 2, "translation": [0, 0, 0.28]}
    A.add(ring, like="hob-ring", maxden=100)
pot = revolve("pot-body", "pot", [[0.908, 0.0], [0.908, 0.102], [0.912, 0.109], [0.92, 0.11], [1.036, 0.11], [1.036, 0.118], [1.048, 0.118], [1.048, 0.108], [0.93, 0.104], [0.914, 0.098], [0.912, 0.0]], axis=(0, 1, 0), material="stainless-steel-pot", role="body", tone=0.5, region="pot")
A.add(pot, like="pot-body")
get(A, "pot-body")["transform"] = {"translation": [0.32, 0.0, -0.14]}
A.add(capsule("pot-handle", "pot", [0.12, 1.002, -0.14], [0.22, 1.002, -0.14], 0.014, material="stainless-steel-pot", role="body", tone=0.4, region="pot"), like="pot-handle")
A.add(capsule("spoon-handle", "spoon", [-0.12, 0.0, 0.046], [0.14, 0.0, 0.046], 0.009, material="beech-wood", role="body", tone=0.45, region="spoon"), like="spoon-handle")
A.add(ellipsoid("spoon-bowl", "spoon", [0.042, 0.024, 0.028], [0.18, 0.0, 0.046], material="beech-wood", role="body", tone=0.42, region="spoon"), like="spoon-bowl")
DEC = BODY + [
    d("The head, neck and hair are centred on the body axis: the 2D front and plan views draw them as revolves about an axis 0.15 m to the right of the body (axisZ -0.15), which cannot be a physical head position; the volume keeps the head 0.072 m half width on the axis and the silhouette gate shows the cost of that choice.",
      "Kopf, Hals und Haar sind auf der Körperachse zentriert: die 2D-Vorder- und Draufsicht zeichnen sie als Rotationskörper um eine Achse 0,15 m rechts des Körpers (axisZ -0,15), was keine physische Kopfposition sein kann; das Volumen hält den Kopf mit 0,072 m Halbbreite auf der Achse, und das Silhouettenmass zeigt den Preis dieser Wahl."),
    d("The kitchen counter is boxes (plinth, body, worktop, glass-ceramic hob plate) with a thin door panel on the front face, two burner discs per side (one repeated pair each) and a hollow pot revolve (wall, floor, flared rim, 0.11 m radius at 0.14 m right of the axis) with a tube handle; the spoon is a tube with an ellipsoid bowl on the hand bone.",
      "Die Küchenzeile besteht aus Quadern (Sockel, Korpus, Arbeitsplatte, Glaskeramik-Kochfeld) mit einer dünnen Türblende an der Vorderseite, zwei Kochplatten pro Seite (je ein wiederholtes Paar) und einem hohlen Rotationstopf (Wand, Boden, ausgestellter Rand, Radius 0,11 m, 0,14 m rechts der Achse) mit Rohrgriff; der Löffel ist ein Rohr mit Ellipsoid-Laffe am Handknochen."),
    d("The sweatshirt is a loft with the front width (0.205 m) over a trouser pelvis, the kangaroo pocket a flat box, the drawstrings tubes, rib bands thin rings; the drawn drawer handle lies inside the cabinet body and is not modelled.",
      "Das Sweatshirt ist ein Loft mit der Breite der Vorderansicht (0,205 m) über einem Hosen-Becken, die Känguru-Tasche ein flacher Quader, die Kordeln Röhren, die Rippbündchen dünne Ringe; der gezeichnete Schubladengriff liegt im Korpus und ist nicht modelliert."),
]
A.write(DEC)
