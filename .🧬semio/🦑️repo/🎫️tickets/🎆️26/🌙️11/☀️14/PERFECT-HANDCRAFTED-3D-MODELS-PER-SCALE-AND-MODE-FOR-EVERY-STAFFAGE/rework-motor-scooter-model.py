"""🛵 Motor scooter, second round pass 1: hand-stated joins at 1:50 and 1:100 and the wheel and steering groups of 1:200 and 1:500 modelled for their own feature size (30 and 75 mm). Idempotent: every solid is put by id. Usage: python3 rework-motor-scooter-model.py"""
import copy
from rework_09_lib import Source, cap

src = Source("vehicles/motor-scooter")
Z = [0, 0, 1]
BAR = (-0.00529, 0.00759)
AXLE = (0.86126, 0.025)
UP = (-0.906308, 0.422618)


def rev(id, bone, profile, material, tone, origin=(0, 0, 0), axis=Z, role="body", **fields):
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": role, "tone": tone, "profile": profile, "axis": list(axis), "origin": list(origin), **fields}


def keep(scale, name, ids):
    """✂️ Keeps the stated ids of a list in the stated order."""
    items = {s["id"]: s for s in src.v[str(scale)][name]}
    src.v[str(scale)][name] = [items[i] for i in ids if i in items]


def pad(scale, thick):
    """🟩 Front contact pad one feature thick, its underside on the ground."""
    lift = (thick - 0.0075) / 2
    p = src.get(scale, "structure", "contact-pad-f")
    p["size"] = [0.09, thick, 0.11]
    p["transform"]["translation"] = [round(1.06631 + UP[0] * lift, 5), round(-0.07062 + UP[1] * lift, 5), 0]


HUB_R = [[0.008, 0], [0.008, 0.079], [0.014, 0.083], [0.046, 0.083], [0.052, 0.079], [0.052, 0.03], [0.092, 0.03], [0.092, 0]]

for scale in (50, 100, 200):
    src.get(scale, "body", "hub-r")["profile"] = HUB_R

for scale in (50, 100):
    header = src.get(scale, "body", "header")
    header["sections"][-1]["at"] = [-0.246, 0.0507, -0.15]
    top = src.get(scale, "body", "shock-f-top")
    top["origin"] = [0.63888, 0.02608, -0.046]
    top["profile"] = [[0, 0.008], [0.086, 0.008]]
    pan = src.get(scale, "structure", "seat-pan")
    if pan["kind"] == "loft":
        pan["sections"][0]["at"] = [0.004, -0.007, 0]
    for part in ("shock-f-top", "shock-f-foot"):
        src.put(scale, "structure", {"id": part, "of": part, "member": "steer", "part": "member"}, after="shock-f-body")

src.get(100, "body", "spokes-f")["to"] = 0.023
src.get(100, "body", "spokes-r")["to"] = 0.023
for side in ("l", "r"):
    for section in src.get(100, "body", "stalk-" + side)["sections"]:
        section["rx"] = section["ry"] = 0.0075
    z = 0.165 if side == "l" else -0.165
    src.put(100, "body", cap("rail-post-" + side, "frame", [-0.62, 0.69, z], [-0.62, 0.742, z * 1.018], 0.009, material="alloy", role="detail", tone=0.74), after="grab-rail")
    src.put(100, "thermal", {"id": "th-rail-post-" + side, "of": "rail-post-" + side, "region": "body-panels"}, after="th-grab-rail")
pad(100, 0.015)

CENTRE = [[0, 0], [0, 0.137], [0.03, 0.137], [0.03, 0]]
for wheel in ("f", "r"):
    src.put(200, "body", rev("centre-" + wheel, "wheel-" + wheel, CENTRE, "alloy", 0.7), after="rim-" + wheel)
    src.put(200, "thermal", {"id": "th-centre-" + wheel, "of": "centre-" + wheel, "region": "rims"}, after="th-rim-" + wheel)
    src.put(200, "structure", {"id": "wheel-member-centre-" + wheel, "of": "centre-" + wheel, "member": "wheel-" + wheel, "part": "member"}, after="wheel-member-rim-" + wheel)
for side, sign in (("l", 1), ("r", -1)):
    stalk = copy.deepcopy(src.get(50, "body", "stalk-" + side))
    for section in stalk["sections"]:
        section["rx"] = section["ry"] = 0.015
    src.put(200, "body", stalk)
    src.get(200, "body", "mirror-" + side)["radii"] = [0.015, 0.042, 0.056]
src.put(200, "body", cap("link", "steer", [0.8687, 0.112, -0.085], [AXLE[0], AXLE[1], -0.085], 0.015, material="alloy", role="body", tone=0.46))
src.put(200, "body", cap("shock-f-body", "steer", [0.847, 0.025, -0.113], [0.645, 0.026, -0.113], 0.015, material="cast-iron", role="body", tone=0.4))
src.put(200, "body", cap("shock-f-top", "steer", [0.645, 0.026, -0.113], [0.6492, 0.008, -0.06], 0.015, material="cast-iron", role="body", tone=0.4))
keep(200, "body", ["floor", "shield", "body", "engine", "muffler", "seat", "disc-f", "tyre-f", "rim-f", "centre-f", "hub-f", "tyre-r", "rim-r", "centre-r", "hub-r", "headset", "headlamp", "switch-l", "grip-l", "stalk-l", "mirror-l", "switch-r", "grip-r", "stalk-r", "mirror-r", "column", "fork-leg", "link", "stub-axle", "shock-f-body", "shock-f-spring", "shock-f-top", "caliper", "mudguard", "arm-r"])
keep(200, "thermal", ["th-floor", "th-shield", "th-body", "th-tail-lamp", "th-engine", "th-arm-r", "th-muffler", "tailpipe-outlet", "th-seat", "th-disc-f", "th-tyre-f", "th-rim-f", "th-centre-f", "th-hub-f", "th-tyre-r", "th-rim-r", "th-centre-r", "th-hub-r", "th-headset", "th-headlamp", "th-switch-l", "th-grip-l", "th-stalk-l", "th-mirror-l", "th-switch-r", "th-grip-r", "th-stalk-r", "th-mirror-r", "th-column", "th-fork-leg", "th-link", "th-stub-axle", "th-shock-f-body", "th-shock-f-spring", "th-shock-f-top", "th-caliper", "th-mudguard", "th-mudguard-wall"])
src.put(200, "structure", cap("handlebar", "steer", [BAR[0], BAR[1], -0.336], [BAR[0], BAR[1], 0.336], 0.015, material="exhaust-steel", role="body", tone=0.4, member="steer", part="member"))
src.put(200, "structure", {"id": "shock-f-top", "of": "shock-f-top", "member": "steer", "part": "member"}, after="shock-f-body")
src.get(200, "structure", "bearing-lower")["profile"] = [[0, 0.031], [0.03, 0.031]]
pivot = src.get(200, "structure", "link-pivot")
pivot["profile"] = [[0, 0.015], [0.044, 0.015]]
pivot["origin"] = [0.8687, 0.112, -0.107]
keep(200, "structure", ["axle-f", "axle-r", "engine", "arm-r", "backbone", "rail-l", "rail-r", "bulkhead-l", "bulkhead-r", "cross-rear", "shock-r", "seat-pan", "stand-frame", "swing-pivot", "seat-hinge", "stand-pin", "mass-frame", "mass-engine", "wheel-member-tyre-f", "wheel-member-rim-f", "wheel-member-centre-f", "wheel-member-hub-f", "wheel-member-tyre-r", "wheel-member-rim-r", "wheel-member-centre-r", "wheel-member-hub-r", "column", "fork-leg", "link", "stub-axle", "shock-f-body", "shock-f-top", "bearing-upper", "bearing-lower", "link-pivot", "contact-pad-f", "mass-steer", "handlebar"])
pad(200, 0.03)

DISH = [[-0.04, 0], [-0.04, 0.13], [-0.045, 0.15], [-0.045, 0.158], [0.045, 0.158], [0.045, 0.15], [0.04, 0.13], [0.04, 0]]
for wheel, drum in (("f", 0.1), ("r", 0.083)):
    src.put(500, "body", rev("rim-" + wheel, "wheel-" + wheel, DISH, "alloy", 0.74))
    src.put(500, "thermal", rev("th-brake-" + wheel, "wheel-" + wheel, [[-0.0375, 0], [-0.0375, drum], [0.0375, drum], [0.0375, 0]], "cast-iron", 0.3, region="brake-disc"), after="th-rim-" + wheel)
for side, sign in (("l", 1), ("r", -1)):
    src.put(500, "body", cap("grip-" + side, "steer", [BAR[0], BAR[1], sign * 0.15], [BAR[0], BAR[1], sign * 0.3], 0.0375, material="rubber", role="body", tone=0.12))
src.put(500, "body", cap("column", "steer", [0.05, 0, 0], [0.58, 0, 0], 0.0375, material="cast-iron", role="body", tone=0.4))
src.put(500, "body", cap("fork-leg", "steer", [0.58, 0, -0.01], [AXLE[0], AXLE[1], -0.085], 0.0375, material="cast-iron", role="body", tone=0.4))
keep(500, "body", ["floor", "shield", "body", "engine", "muffler", "seat", "tyre-f", "rim-f", "tyre-r", "rim-r", "headset", "headlamp", "grip-l", "mirror-l", "grip-r", "mirror-r", "column", "fork-leg", "mudguard", "arm-r"])
keep(500, "thermal", ["th-floor", "th-shield", "th-body", "th-engine", "th-muffler", "tailpipe-outlet", "th-seat", "th-tyre-f", "th-rim-f", "th-brake-f", "th-tyre-r", "th-rim-r", "th-brake-r", "th-headset", "th-headlamp", "th-grip-l", "th-mirror-l", "th-grip-r", "th-mirror-r", "th-arm-r", "th-column", "th-fork-leg", "th-mudguard", "th-mudguard-wall"])
src.put(500, "structure", cap("handlebar", "steer", [BAR[0], BAR[1], -0.3], [BAR[0], BAR[1], 0.3], 0.0375, material="exhaust-steel", role="body", tone=0.4, member="steer", part="member"))
src.put(500, "structure", rev("bearing-lower", "steer", [[0, 0.05], [0.075, 0.05]], "cast-iron", 0.3, origin=(0.3, 0, 0), axis=(1, 0, 0), role="detail", member="steer", part="joint"))
src.get(500, "structure", "mass-steer")["radii"] = [0.06, 0.0375, 0.0375]
keep(500, "structure", ["axle-f", "axle-r", "engine", "arm-r", "backbone", "bulkhead", "seat-pan", "stand-frame", "swing-pivot", "mass-frame", "mass-engine", "wheel-member-tyre-f", "wheel-member-rim-f", "wheel-member-tyre-r", "wheel-member-rim-r", "column", "fork-leg", "bearing-lower", "contact-pad-f", "mass-steer", "handlebar"])
pad(500, 0.075)

for scale, radius in ((50, 0.012), (100, 0.012), (200, 0.015)):
    for side, sign in (("l", 1), ("r", -1)):
        src.put(scale, "structure", cap("bulkhead-" + side, "frame", [-0.205, 0.44, sign * 0.125], [-0.127, 0.716, sign * 0.05], radius, material="paint", role="body", tone=0.5, member="frame", part="member"), after="rail-r")
src.put(500, "structure", cap("bulkhead", "frame", [-0.2776, 0.4694, 0], [-0.165, 0.715, 0], 0.0375, material="paint", role="body", tone=0.5, member="frame", part="member"), after="backbone")
pan100 = src.get(100, "structure", "seat-pan")
pan100["profile"][0] = [0.085, 0.0]
pan100["profile"][-1] = [-0.085, 0.0]
pan200 = src.get(200, "structure", "seat-pan")
pan200["size"] = [0.66, 0.03, 0.26]
pan200["transform"]["translation"] = [0.335, -0.003, 0]
column200 = src.get(200, "body", "column")
column200["origin"] = [-0.012, 0, 0]
column200["profile"] = [[0, 0.02], [0.612, 0.02]]
guard = src.get(100, "body", "silencer-guard")
guard["profile"] = [[0.045, 0.0565], [0.3254, 0.0565], [0.3254, 0.065], [0.045, 0.065], [0.045, 0.0565]]
src.put(200, "body", cap("arm-r", "swing", [-0.16, 0.045, -0.078], [-0.4, 0.025, -0.078], 0.02, material="engine-metal", role="body", tone=0.36), after="engine")
src.put(500, "body", cap("arm-r", "swing", [-0.12, 0.05, -0.06], [-0.4, 0.025, -0.07], 0.0375, material="engine-metal", role="body", tone=0.36), after="engine")
for scale in (200, 500):
    src.put(scale, "thermal", {"id": "th-arm-r", "of": "arm-r", "region": "engine"}, after="th-engine")
    src.put(scale, "structure", {"id": "arm-r", "of": "arm-r", "member": "swing", "part": "member"}, after="engine")
src.drop(200, "structure", "wheel-member-hub-f", "wheel-member-hub-r", "stub-axle", "bearing-upper")
for side, sign in (("l", 1), ("r", -1)):
    src.put(500, "body", cap("mirror-" + side, "steer", [-0.03, 0.014, sign * 0.2], [-0.10353, 0.03354, sign * 0.314], 0.0375, 0.045, material="alloy", role="body", tone=0.74), after="grip-" + side)
    src.put(500, "thermal", {"id": "th-mirror-" + side, "of": "mirror-" + side, "region": "steer-assembly"}, after="th-grip-" + side)

for wheel in ("f", "r"):
    src.drop(200, "structure", "wheel-member-rim-" + wheel, "wheel-member-centre-" + wheel)
    src.put(200, "structure", rev("wheel-disc-" + wheel, "wheel-" + wheel, [[-0.04, 0], [-0.04, 0.155], [0.04, 0.155], [0.04, 0]], "alloy", 0.7, member="wheel-" + wheel, part="member"), after="wheel-member-tyre-" + wheel)

D200 = [
    ("Body at 1:200, authored for 30 mm features: 35 solids. Floor, leg shield and rear body are one mass each; the engine unit is one body left of the rear wheel with the right arm as a 40 mm bar that carries the silencer capsule. Each wheel is the tyre, the rim ring, one closed 30 mm centre dish in place of the five spokes and the hub (the rear drum hub with its output boss reaches the belt case); the front brake disc stays as a declared thin disc. Steering: headset, headlamp, switch housings and grips, the mirrors as 30 mm discs on typified 30 mm stalks, column, the bent fork leg, the trailing link and the damper with its top lug as 30 mm bars, the coil spring, the caliper and the mudguard. Levers, brake hose, instrument dome, clamp hardware, lamps on the body, grab rail and stand are left out.",
     "Körper bei 1:200, für 30-mm-Einzelheiten entworfen: 35 Körper. Boden, Beinschild und Heckkarosserie sind je eine Masse; der Triebsatz ist ein Körper links vom Hinterrad, der rechte Arm ein 40-mm-Stab, der die Schalldämpferkapsel trägt. Jedes Rad besteht aus Reifen, Felgenring, einer geschlossenen 30-mm-Radschüssel anstelle der fünf Speichen und der Nabe (die hintere Trommelnabe reicht mit ihrem Abtriebsbund bis zum Riemengehäuse); die vordere Bremsscheibe bleibt als erklärte dünne Scheibe. Lenkung: Lenkkopf, Scheinwerfer, Schaltergehäuse und Griffe, die Spiegel als 30-mm-Scheiben auf typisierten 30-mm-Stielen, Lenksäule, das gebogene Gabelrohr, die gezogene Schwinge und der Dämpfer mit seiner oberen Lasche als 30-mm-Stäbe, die Schraubenfeder, der Bremssattel und das Schutzblech. Hebel, Bremsschlauch, Instrumentenkuppel, Klemmteile, Leuchten an der Karosserie, Haltebügel und Ständer entfallen."),
    ("Thermal: every body solid by reference in its region (panels, steering, engine with the right arm, exhaust, seat, tyres, rims with the centre dish, brake disc with the rear drum hub and the caliper, grips, headlamp lens), the tail lamp as an own solid, the steel wall inside the mudguard film and the exhaust outlet patch on the rear end of the silencer.",
     "Thermik: jeder Körper per Verweis in seinem Bereich (Bleche, Lenkung, Motor mit rechtem Arm, Abgasanlage, Sitz, Reifen, Felgen mit Radschüssel, Bremsscheibe mit hinterer Trommelnabe und Bremssattel, Griffe, Scheinwerferglas), das Rücklicht als eigener Körper, die Stahlwand in der Lackschicht des Schutzblechs und der Auslassfleck am hinteren Ende des Schalldämpfers."),
    ("Structure: one backbone from the head tube down the leg shield and along the floor, two shell rails to the tail with a rear cross member, two bulkhead struts from the rails up to the seat hinge (the raked front of the rear shell), the rear shock; handlebar tube, column, fork leg, trailing link and front damper with its lug; the engine body and the right arm are the swinging unit; per wheel the tyre and one rim disc; seat pan reaching its hinge; the stand as one U-frame plate. Joints: lower head bearing, link pivot, swing pivot, seat hinge, stand pin, two axles. Contact pad one feature thick under the front tyre; three masses.",
     "Tragwerk: ein Rückgrat vom Steuerrohr durch das Beinschild und den Boden entlang, zwei Schalenholme zum Heck mit einem Heckquerträger, zwei Schottstreben von den Holmen hinauf zum Sitzscharnier (die geneigte Front der Heckschale), das hintere Federbein; Lenkerrohr, Lenksäule, Gabelrohr, gezogene Schwinge und vorderer Dämpfer mit Lasche; Motorkörper und rechter Arm bilden die Triebsatzschwinge; je Rad der Reifen und eine Felgenscheibe; die Sitzwanne reicht bis an ihr Scharnier; der Ständer als eine U-Rahmen-Platte. Gelenke: unteres Steuerlager, Schwingenlager vorn, Triebsatzlager, Sitzscharnier, Ständerbolzen, zwei Achsen. Ein Kontaktpolster von einer Einzelheit Dicke unter dem Vorderreifen; drei Massen."),
]
D500 = [
    ("Body at 1:500, authored for 75 mm features: 20 solids. The floor slab; the leg shield as one lens-shaped plate; the rear body with cowls and lamp; the seat; the engine unit left of the rear wheel, the right arm as a 75 mm bar and the silencer capsule on it. Each wheel is the tyre and one closed rim dish. Steering: the headset with the headlamp, two typified 75 mm grips, each mirror with its stalk as one tapered 75 to 90 mm horn (they carry the 1.15 m height and the 0.74 m width), the column and one straight 75 mm fork leg from the column to beside the front axle on the right, and the mudguard. Spokes, hubs, brake parts, shock, link, levers, lamps and the stand are left out.",
     "Körper bei 1:500, für 75-mm-Einzelheiten entworfen: 20 Körper. Die Bodenplatte; das Beinschild als eine linsenförmige Platte; die Heckkarosserie mit Backen und Leuchte; der Sitz; der Triebsatz links vom Hinterrad, der rechte Arm als 75-mm-Stab und darauf die Schalldämpferkapsel. Jedes Rad besteht aus dem Reifen und einer geschlossenen Felgenschüssel. Lenkung: der Lenkkopf mit Scheinwerfer, zwei typisierte 75-mm-Griffe, jeder Spiegel samt Stiel als ein konisches Horn von 75 bis 90 mm (sie tragen die Höhe von 1,15 m und die Breite von 0,74 m), die Lenksäule und ein gerades 75-mm-Gabelrohr von der Säule bis rechts neben die Vorderachse sowie das Schutzblech. Speichen, Naben, Bremsteile, Federbein, Schwinge, Hebel, Leuchten und Ständer entfallen."),
    ("Thermal: the body solids by reference in their regions, the steel wall inside the mudguard film, one hidden 75 mm brake cylinder inside each rim dish (front disc, rear drum) and the exhaust outlet patch on the rear end of the silencer.",
     "Thermik: die Körper per Verweis in ihren Bereichen, die Stahlwand in der Lackschicht des Schutzblechs, je ein verdeckter 75-mm-Bremszylinder in jeder Felgenschüssel (vorn Scheibe, hinten Trommel) und der Auslassfleck am hinteren Ende des Schalldämpfers."),
    ("Structure: one backbone from the head tube through the leg shield, along the floor and up to the tail, with one bulkhead strut up to the seat pan; handlebar, column and fork leg; the engine body and the right arm as the swinging unit; tyre and rim dish per wheel; the seat pan and the stand as plates. Joints: one head bearing, swing pivot, two axles. Contact pad one feature thick under the front tyre; three masses.",
     "Tragwerk: ein Rückgrat vom Steuerrohr durch das Beinschild, den Boden entlang und hinauf zum Heck, mit einer Schottstrebe hinauf zur Sitzwanne; Lenker, Lenksäule und Gabelrohr; Motorkörper und rechter Arm als Triebsatzschwinge; Reifen und Felgenschüssel je Rad; Sitzwanne und Ständer als Platten. Gelenke: ein Steuerlager, Triebsatzlager, zwei Achsen. Ein Kontaktpolster von einer Einzelheit Dicke unter dem Vorderreifen; drei Massen."),
]
src.v["200"]["decisions"] = [{"en": en, "de": de} for en, de in D200]
src.v["500"]["decisions"] = [{"en": en, "de": de} for en, de in D500]
JOIN = ("Joins: the rear drum hub carries an output boss into the belt case, the header pipe enters the silencer, the front shock top bolt reaches its lug on the fork leg, the seat pan reaches the hinge, and two bulkhead struts carry the seat hinge from the shell rails.", "Verbindungen: die hintere Trommelnabe trägt einen Abtriebsbund bis ins Riemengehäuse, der Krümmer mündet im Schalldämpfer, der obere Bolzen des vorderen Federbeins reicht bis zu seiner Lasche am Gabelrohr, die Sitzwanne reicht an das Scharnier, und zwei Schottstreben tragen das Sitzscharnier von den Schalenholmen.")
for scale in (50, 100):
    src.decide(scale, *JOIN)

BARE = ("th-steering-tube", "th-bar-saddle-cap", "th-bar-saddle-base", "th-upper-column-socket", "th-saddle-pinch-lower", "th-saddle-pinch-upper")
for scale in (100, 200, 500):
    for solid in src.v[str(scale)]["thermal"]:
        if solid.get("region") in ("body-panels", "steer-assembly") and "layer" not in solid and solid["id"] not in BARE:
            solid["layer"] = "paint"

src.save()
print("[DEBUG] saved", {k: {n: len(v[n]) for n in ("body", "thermal", "structure")} for k, v in src.v.items()})
