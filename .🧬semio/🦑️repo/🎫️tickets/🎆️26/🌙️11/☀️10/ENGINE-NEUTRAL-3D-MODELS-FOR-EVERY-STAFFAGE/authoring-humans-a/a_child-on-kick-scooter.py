from body import *
from decisions import *
A = Asset("child-on-kick-scooter")
SK = {"backpack-pocket-1", "backpack-pocket-2", "lace-l0", "lace-l1", "lace-l2", "lace-r0", "lace-r1", "lace-r2", "sole-l", "sole-r", "front", "mouth", "ear", "hair", "hair-nape", "thumb-l", "thumb-r",
      "finger-l0", "finger-l1", "finger-l2", "finger-r0", "finger-r1", "finger-r2", "button-0", "button-1", "button-2"}
humanize(A, skip=SK | {"hat", "backpack"}, lats={"hair": (-0.03, 0.03)})
loft_hat = loft_from(A.draft["hat"], exp=2.0)
A.add(loft_hat, like="hat")
A.add(loft_from(A.draft["backpack"], exp=3.2), like="backpack")
A.add(loft_from(A.draft["hair"], lat=(-0.035, 0.035), exp=2.0), like="hair")
for sd in ("l", "r"):
    h = A.draft[f"hand-{sd}"]
    wc = (h["from"] + h["to"]) / 2
    A.add(capsule(f"thumb-{sd}", f"hand-{sd}", [0.02, 0.035 if False else 0.03, wc], [0.085, 0.036, wc], 0.009, 0.007, material=h["material"], role="body", tone=h["tone"], region="hands"), like="hand")
    A.add(capsule(f"fingers-{sd}", f"hand-{sd}", [0.1, -0.012, wc], [0.138, -0.012, wc], 0.01, 0.008, ls=1.6, material=h["material"], role="detail", tone=0.5, region="hands"), like="finger", maxden=50)
deltoids(A, ["sleeve-up-l", "sleeve-up-r"])
eyes(A, u=0.074, w=0.03)
A.keep("ear")
A.add(capsule("mouth", "head", [0.03, -0.068, -0.01], [0.03, -0.068, 0.01], 0.004, material="skin-child", role="detail", tone=0.5, region="face-skin"), like="mouth", maxden=50)
for i in range(3):
    s = A.draft[f"button-{i}"]
    A.add({k: v for k, v in s.items() if k != "transform"} | {"transform": {"translation": s["transform"]["translation"]}}, like="button", maxden=50)
A.add(capsule("zip", "torso", [0.06, -0.103, 0.0], [0.34, -0.052, 0.0], 0.003, material="zip-jacket", role="detail", tone=0.5, region="jacket"), like="front", maxden=50)

for sid in ("deck", "fender", "strut"):
    A.keep(sid)
A.keep("head-tube")
A.add(capsule("stem", "frame", [0.69, 0.165, 0], [0.598, 0.815, 0], 0.0135, material="alu-stem", role="body", tone=0.7, region="stem"), like="stem")
A.keep("fork", **{})
A.add(capsule("handlebar", "frame", [0.6, 0.84, -0.19], [0.6, 0.84, 0.19], 0.0175, material="alu-stem", role="body", tone=0.5, region="stem"), like="handlebar")
A.keep("grip")
A.keep("clamp")
A.keep("brake-pad")
G = lambda n: A.draft[n]
for w, bone in (("rear", "wheel-rear"), ("front", "wheel-front")):
    A.add(revolve(f"tyre-{w}", bone, [[-0.015, 0.0], [-0.015, 0.062], [-0.011, 0.0705], [0.0, 0.0725], [0.011, 0.0705], [0.015, 0.062], [0.015, 0.0]], axis=(0, 0, 1), material="pu-wheel", role="body", tone=0.14, region="wheels"), like="tyre")
    A.add(revolve(f"rim-{w}", bone, [[-0.0165, 0.0], [-0.0165, 0.046], [0.0165, 0.046], [0.0165, 0.0]], axis=(0, 0, 1), material="pu-wheel", role="body", tone=0.8, region="wheels"), like="rim")
    A.add(ellipsoid(f"hub-{w}", bone, [0.014, 0.014, 0.0185], [0, 0, 0], material="pu-wheel", role="detail", tone=0.3, region="wheels"), like="hub")
    for side, z in (("l", 0.0168), ("r", -0.0168)):
        sp = capsule(f"spoke-{w}-{side}", bone, [0, 0.014, z], [0, 0.044, z], 0.004, material="pu-wheel", role="detail", tone=0.4, region="wheels")
        sp["repeat"] = {"count": 3, "rotationAxis": [0, 0, 1], "rotationRad": 2.0943951, "origin": [0, 0, 0]}
        A.add(sp, like="spoke")
A.add(box("backpack-pocket", "torso", [0.12, 0.012, 0.15], [0.1, 0.2, 0.0], corner=0.005, material="nylon-backpack", role="detail", tone=0.3, region="backpack"), like="backpack-pocket", maxden=50)
DEC = BODY + [
    d("The scooter is a rigid bone-bound assembly: deck and fender as extruded plates, stem and handlebar as round tubes (capsules), fork legs as a repeated pair, wheels as revolves about the axle with a rim disc, a hub and three spokes per side (repeat about the axle).",
      "Der Roller ist eine starre, an Knochen gebundene Baugruppe: Trittbrett und Schutzblech als extrudierte Platten, Lenkstange und Lenker als runde Rohre (Kapseln), die Gabelholme als wiederholtes Paar, die Räder als Rotationskörper um die Achse mit Felgenscheibe, Nabe und je drei Speichen pro Seite (Wiederholung um die Achse)."),
    d("The slanted forearms reach from the shoulder width out to the grips: the lateral span of the front view is a slanted tube (round sections whose centre moves outwards along the bone), not a wide flat sleeve.",
      "Die schrägen Unterarme reichen von der Schulterbreite zu den Griffen: die seitliche Spanne der Vorderansicht ist ein schräges Rohr (runde Schnitte, deren Mitte entlang des Knochens nach aussen wandert), kein breiter flacher Ärmel."),
    d("The drawn backpack pocket outline and its seam become one thin rounded box on the back face; laces and soles drawn inside the shoe outline lie within the shoe volume and are not modelled. The face-only parts (eyes, mouth) are authored from the head, not taken from the front view.",
      "Die gezeichnete Rucksacktasche samt Naht wird zu einem flachen, abgerundeten Quader auf der Rückseite; Schnürsenkel und Sohle innerhalb der Schuhkontur liegen im Schuhvolumen und werden nicht modelliert. Die nur in der Vorderansicht gezeichneten Gesichtsteile (Augen, Mund) sind vom Kopf aus gesetzt, nicht aus der Vorderansicht übernommen."),
]
A.write(DEC)
