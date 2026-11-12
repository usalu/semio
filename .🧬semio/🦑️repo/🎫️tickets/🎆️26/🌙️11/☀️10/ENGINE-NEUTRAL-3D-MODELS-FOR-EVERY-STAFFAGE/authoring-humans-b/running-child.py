from lib import Builder, auto
b = Builder("running-child")
auto(b, over={"tshirt-print": None, "pelvis-shorts": dict(front=["hips"], plan=["waist"], n=4, exp=2.4),
    "tshirt": dict(front=["torso"], plan=["trunk-l", "trunk-r"], n=8, exp=2.4),
    "sleeve-l": dict(front=["upper-arm-l"], plan=["upper-arm-l"], grow=1.09, n=4), "sleeve-r": dict(front=["upper-arm-r"], plan=["upper-arm-r"], grow=1.09, n=4),
    "arm-skin-l": dict(front=["upper-arm-l"], plan=["upper-arm-l"]), "arm-skin-r": dict(front=["upper-arm-r"], plan=["upper-arm-r"]),
    "shorts-l": dict(grow=1.06, n=5), "shorts-r": dict(grow=1.06, n=5), "sock-l": dict(front=["shank-l"], plan=["shank-l"], grow=1.08, n=4), "sock-r": dict(front=["shank-r"], plan=["shank-r"], grow=1.08, n=4),
    "thigh-skin-l": dict(front=["thigh-l"], plan=["thigh-l"]), "thigh-skin-r": dict(front=["thigh-r"], plan=["thigh-r"]),
    "shoe-rear-l": dict(front=["shoe-l"], plan=["shoe-heel-l"], exp=2.6), "shoe-rear-r": dict(front=["shoe-r"], plan=["shoe-heel-r"], exp=2.6),
    "shoe-toe-l": dict(front=["shoe-toe-l"], plan=["shoe-toe-l"], n=4, exp=2.6), "shoe-toe-r": dict(front=["shoe-toe-r"], plan=["shoe-toe-r"], n=4, exp=2.6),
    "head": dict(front=["head"], plan=["head"]), "hair": dict(front=["hair"], plan=["head"], n=6),
    "collar": dict(lat=(-0.045, 0.045), n=4),
    "thumb-l": dict(lat=(0.092, 0.112)), "thumb-r": dict(lat=(-0.112, -0.092)),
    "pack-body": dict(front=["pack-body"], plan=["pack-body"], n=6, exp=5.0, ends=0.92)} , bones=set(__import__("lib").ORGANIC) | {"pack"})
for s, z in (("l", 1), ("r", -1)):
    b.ell(f"shoulder-{s}", f"arm-{s}", (0, 0.874, z * 0.105), (0.035, 0.033, 0.033), src=f"sleeve-{s}", maxden_ids=[f"sleeve-{s}"])
    b.ell(f"elbow-{s}", f"forearm-{s}", (0, 0.689, z * 0.105), (0.026, 0.028, 0.028), src=f"forearm-{s}", maxden_ids=[f"forearm-{s}"])
    b.ell(f"fist-{s}", f"hand-{s}", (0.004, 0.543, z * 0.105), (0.02, 0.02, 0.02), src=f"hand-{s}", maxden_ids=[f"hand-{s}"])
    b.ell(f"knee-{s}", f"shank-{s}", (0, 0.273, z * 0.055), (0.036, 0.036, 0.039), src=f"shank-{s}", maxden_ids=[f"shank-{s}"])
    b.polyline(f"pack-strap-{s}", "pack", [(-0.075, 0.82, z * 0.065), (0.0, 0.874, z * 0.065), (0.052, 0.807, z * 0.065), (0.025, 0.66, z * 0.065)], 0.011, src="pack-body", maxden_ids=["pack-strap"])
b.ell("nose", "head", (0.1, 1.025, 0), (0.014, 0.012, 0.011), material="skin", role="detail", tone=b.sh("head")["tone"], region=b.sh("head")["region"], maxden_ids=["eye"])
b.ell("shoulders", "torso", (0, 0.865, 0), (0.06, 0.03, 0.11), src="tshirt", maxden_ids=["tshirt"])
b.box("pack-pocket", "pack", (-0.1435, 0.66, 0), (0.07, 0.014, 0.10), corner=0.004, src="pack-body", maxden_ids=["pack-pocket"])
b.box("tshirt-print", "torso", (0.099, 0.745, 0), (0.065, 0.004, 0.08), src="tshirt-print", maxden_ids=["tshirt-print"])
b.dec("Every body part is a loft of superelliptic sections: depth and height are measured on the side outline, width and lateral position on the front twin (or the plan twin for the near-horizontal foot). Clothing that the drawing lays over skin (sleeves, shorts, socks, collar) is a loft 6 to 9 % larger than the skin loft beneath it, so skin stays visible where the drawing shows it.",
      "Jeder Körperteil ist ein Loft aus superelliptischen Schnitten: Tiefe und Höhe sind der Seitenansicht entnommen, Breite und seitliche Lage dem Vorderansichtszwilling (beim fast waagrechten Fuss dem Grundriss). Kleidung, die die Zeichnung über die Haut legt (Ärmel, Shorts, Socken, Kragen), ist ein um 6 bis 9 % grösserer Loft über dem Hautloft, sodass die Haut dort sichtbar bleibt, wo die Zeichnung sie zeigt.")
b.dec("Shoulder, elbow, knee and fist ellipsoids exist because the front view draws them as separate round parts; the backpack is a rounded loft with a pocket box on its back and two round straps (0.13 m apart) that run over the shoulder on the chest surface, as the elevation polyline shows. Fingers, stripes and the shirt print line are texture; the print is a 4 mm plate on the chest.",
      "Schulter-, Ellbogen-, Knie- und Faust-Ellipsoide gibt es, weil die Vorderansicht sie als eigene runde Teile zeichnet; der Rucksack ist ein gerundeter Loft mit einer Tasche als Quader auf der Rückseite und zwei runden Riemen (0,13 m auseinander), die wie in der Seitenansichtspolylinie über die Schulter auf der Brustoberfläche laufen. Finger, Streifen und die Druckzeile des Shirts sind Textur; der Aufdruck ist eine 4 mm dicke Platte auf der Brust.")
b.save()
