"""🧹 Hand edits of appliances/vacuum-cleaner, second round pass 1: wheel bosses, per-scale running gear, closed load path."""
import json, os
T = os.path.dirname(os.path.abspath(__file__))
P = T + "/volumes/appliances/vacuum-cleaner.json"
doc = json.load(open(P))
V = doc["volumes"]
GREY, RUBBER, STEEL = "abs-grey", "rubber-wheel", "tube-steel"
AXLE = [-0.17, 0.06, 0]

def drop(lst, ids):
    lst[:] = [o for o in lst if o["id"] not in ids]

def put(lst, solid, after=None):
    drop(lst, [solid["id"]])
    index = next((i for i, o in enumerate(lst) if o["id"] == after), len(lst) - 1) + 1 if after else len(lst)
    lst.insert(index, solid)

def get(lst, id):
    return next(o for o in lst if o["id"] == id)

def boss(side, z0, z1, r):
    sign = 1 if side == "l" else -1
    return {"id": f"wheel-boss-{side}", "kind": "revolve", "bone": "body", "material": GREY, "role": "body", "tone": 0.3,
            "profile": [[z0, 0.0], [z0, r], [z1 - 0.002, r], [z1, r - 0.002], [z1, 0.0]], "axis": [0, 0, sign], "origin": AXLE}

def drum(id, bone, half, r, chamfer, material, tone):
    return {"id": id, "kind": "revolve", "bone": bone, "material": material, "role": "body", "tone": tone,
            "profile": [[-half, 0.0], [-half, r - chamfer], [-half + chamfer, r], [half - chamfer, r], [half, r - chamfer], [half, 0.0]],
            "axis": [0, 0, 1], "origin": [0, 0, 0]}

def disc(id, side, inner, outer, r, chamfer, hub=None):
    sign = 1 if side == "l" else -1
    profile = [[inner, 0.0], [inner, r - chamfer], [inner + chamfer, r], [outer - chamfer, r], [outer, r - chamfer], [outer, 0.0]]
    if hub:
        profile = [[hub[0], 0.0], [hub[0], hub[1]], [inner, hub[1]]] + profile[1:]
    return {"id": id, "kind": "revolve", "bone": "wheel", "material": RUBBER, "role": "body", "tone": 0.2,
            "profile": profile, "axis": [0, 0, sign], "origin": [0, 0, 0]}

def socket(r_out, r_in, x0, x1):
    return {"id": "member-socket", "kind": "revolve", "bone": "body", "material": GREY, "role": "body", "tone": 0.5,
            "profile": [[x0, r_in], [x0, r_out], [x1, r_out], [x1, r_in], [x0, r_in]], "axis": [1, 0, 0], "origin": [0, 0.125, 0],
            "member": "body", "part": "member"}

for scale in ("50", "100"):
    body = V[scale]["body"]
    put(body, boss("l", 0.094, 0.1275, 0.02), after="exhaust-grille")
    put(body, boss("r", 0.094, 0.1275, 0.02), after="wheel-boss-l")

s = V["50"]["structure"]
put(s, socket(0.0245, 0.0175, 0.236, 0.276), after="chassis-wall")
get(s, "joint-nozzle").update({"r0": 0.0175, "r1": 0.0175, "p0": [0, 0, -0.022], "p1": [0, 0, 0.022]})
put(s, {"id": "member-yoke-l", "of": "nozzle-yoke-l", "member": "nozzle", "part": "member"})
put(s, {"id": "member-yoke-r", "of": "nozzle-yoke-r", "member": "nozzle", "part": "member"})
get(s, "mass-grip")["radii"] = [0.025, 0.016, 0.016]
put(s, {"id": "wheel-boss-load-l", "of": "wheel-boss-l", "member": "body", "part": "member"})
put(s, {"id": "wheel-boss-load-r", "of": "wheel-boss-r", "member": "body", "part": "member"})

b, s, t = V["100"]["body"], V["100"]["structure"], V["100"]["thermal"]
drop(b, ["caster-wheel-l", "caster-wheel-r"])
put(b, drum("caster-wheel", "caster", 0.024, 0.025, 0.004, RUBBER, 0.2), after="rear-rim-r")
put(s, socket(0.0245, 0.0095, 0.236, 0.276), after="chassis-wall")
get(s, "mass-grip")["radii"] = [0.025, 0.016, 0.016]
get(s, "support-caster-left")["of"] = "caster-wheel"
get(s, "support-caster-right")["of"] = "caster-wheel"
drop(t, ["th-caster-wheel-l", "th-caster-wheel-r"])
put(t, {"id": "th-caster-wheel", "of": "caster-wheel", "region": "wheels"}, after="th-rear-rim-r")

b, s, t = V["200"]["body"], V["200"]["structure"], V["200"]["thermal"]
drop(b, ["rear-tyre-l", "rear-tyre-r", "rear-rim-l", "rear-rim-r", "caster-wheel-l", "caster-wheel-r", "caster-fork"])
put(b, disc("rear-tyre-l", "l", 0.12, 0.15, 0.06, 0.006, hub=(0.098, 0.02)))
put(b, disc("rear-tyre-r", "r", 0.12, 0.15, 0.06, 0.006, hub=(0.098, 0.02)))
put(b, drum("caster-wheel", "caster", 0.024, 0.025, 0.004, RUBBER, 0.2))
base = get(s, "chassis-base")
if len(base["sections"]) == 3:
    base["sections"].append({"at": [0.0, 0.082, 0.0], "rx": 0.1225, "ry": 0.2475, "exponent": 3.0, "rotation": [-0.5, -0.5, -0.5, 0.5]})
drop(s, ["castor-yoke", "load-rim-l", "load-rim-r"])
get(s, "support-caster-right")["of"] = "caster-wheel"
get(s, "support-caster-left")["of"] = "caster-wheel"
drop(t, ["th-rear-rim-l", "th-rear-rim-r", "th-caster-wheel-l", "th-caster-wheel-r", "th-caster-fork"])
put(t, {"id": "th-caster-wheel", "of": "caster-wheel", "region": "wheels"}, after="th-rear-tyre-r")

b, s, t = V["500"]["body"], V["500"]["structure"], V["500"]["thermal"]
drop(b, ["rear-tyre-l", "rear-tyre-r", "rear-rim-l", "rear-rim-r", "caster-wheel-l", "caster-wheel-r", "caster-fork"])
put(b, drum("rear-wheels", "wheel", 0.15, 0.06, 0.008, RUBBER, 0.2))
put(b, drum("caster-wheel", "caster", 0.0375, 0.025, 0.004, RUBBER, 0.2))
canister = get(b, "canister")
quarter = [-0.5, -0.5, -0.5, 0.5]
canister["sections"] = canister["sections"][:4] + [
    {"at": [-0.004, 0.212, 0.0], "rx": 0.119, "ry": 0.236, "exponent": 3.0, "rotation": quarter},
    {"at": [-0.012, 0.232, 0.0], "rx": 0.105, "ry": 0.205, "exponent": 2.8, "rotation": quarter},
    {"at": [-0.022, 0.244, 0.0], "rx": 0.084, "ry": 0.165, "exponent": 2.5, "rotation": quarter},
    {"at": [-0.03, 0.25, 0.0], "rx": 0.055, "ry": 0.115, "exponent": 2.2, "rotation": quarter}]
drop(s, ["castor-yoke", "load-rim-l", "load-rim-r"])
get(s, "support-wheel-right")["of"] = "rear-wheels"
get(s, "support-caster-right")["of"] = "caster-wheel"
get(s, "support-wheel-left")["of"] = "rear-wheels"
get(s, "support-caster-left")["of"] = "caster-wheel"
drop(t, ["th-rear-tyre-l", "th-rear-tyre-r", "th-rear-rim-l", "th-rear-rim-r", "th-caster-wheel-l", "th-caster-wheel-r", "th-caster-fork"])
put(t, {"id": "th-rear-wheels", "of": "rear-wheels", "region": "wheels"})
put(t, {"id": "th-caster-wheel", "of": "caster-wheel", "region": "wheels"})
quarter = [-0.5, -0.5, -0.5, 0.5]
put(s, {"id": "chassis-base", "kind": "loft", "bone": "body", "material": "abs-red", "role": "body", "tone": 0.5, "sections": [
    {"at": [-0.0024, 0.037, 0.0], "rx": 0.093, "ry": 0.2028, "exponent": 3.0, "rotation": quarter},
    {"at": [-0.0006, 0.06, 0.0], "rx": 0.1178, "ry": 0.2402, "exponent": 3.0, "rotation": quarter},
    {"at": [0.0, 0.112, 0.0], "rx": 0.1235, "ry": 0.2485, "exponent": 3.0, "rotation": quarter}],
    "capStart": True, "capEnd": True, "member": "body", "part": "member"})
lst = V["500"]["structure"]
lst.insert(0, lst.pop())
plug = socket(0.0245, 0.0, 0.232, 0.276)
plug["profile"] = [[0.232, 0.0], [0.232, 0.0245], [0.276, 0.0245], [0.276, 0.0]]
put(V["200"]["structure"], plug, after="chassis-wall")

for scale in ("50", "100"):
    template = get(V[scale]["thermal"], "th-socket")
    for side in ("l", "r"):
        put(V[scale]["thermal"], {**template, "id": f"th-wheel-boss-{side}", "of": f"wheel-boss-{side}"}, after="th-socket")

MOTOR = {"en": "Thermal: the heat source is a real part inside the canister, the bypass motor-fan unit (88 mm motor can, 136 mm fan stage) in the rear chamber on the canister axis in front of the exhaust grille; the 148 mm dust bag fills the front chamber behind the hose socket. Both show only in the cut.",
         "de": "Thermik: Die Wärmequelle ist ein echtes Bauteil im Kessel, die Motor-Gebläse-Einheit (Motorbecher 88 mm, Gebläsestufe 136 mm) in der hinteren Kammer auf der Kesselachse vor dem Abluftgitter; der Staubbeutel von 148 mm füllt die vordere Kammer hinter dem Schlauchstutzen. Beide sind nur im Schnitt sichtbar."}
d = V["50"]["decisions"]
d[0]["en"] = d[0]["en"].replace("rear wheels are tyre rings on dished rims with hub caps,", "the 120 mm rear wheels are tyre rings on dished rims with hub caps and run on two 40 mm bearing bosses moulded onto the flanks of the shell, so each wheel meets the canister at its axle,")
d[0]["de"] = d[0]["de"].replace("die Hinterräder sind Reifenringe auf Schüsselfelgen mit Nabenkappen,", "die 120-mm-Hinterräder sind Reifenringe auf Schüsselfelgen mit Nabenkappen und laufen auf zwei an die Schalenflanken angeformten 40-mm-Lagernaben, sodass jedes Rad an seiner Achse am Kessel anliegt,")
d[1]["en"] = d[1]["en"].split(" The motor and dust bag are not regions")[0]
d[1]["de"] = d[1]["de"].split(" Motor und Staubbeutel sind keine Regionen")[0]
d[2]["en"] = "Structure: the load path is the canister's base tub and its 3 mm side wall with both wheel bosses and the hose socket, which takes the pull of the hose; the 10 mm steel axle with both hub bearings, the caster stem and axle pin, the ten 0.6 mm hose walls with their joints, the handle tube on its swivel, the outer and inner wand tubes and the nozzle elbow, whose 35 mm pivot barrel sits between the two yoke cheeks of the hollow 2.5 mm nozzle housing; supports are a contact strip under both tyres, the caster tread and the two bristle strips; masses sit at the centres of mass of canister (motor and cord reel at the rear), wand, nozzle and handle."
d[2]["de"] = "Tragwerk: Der Lastpfad ist die Bodenwanne des Kessels mit ihrer 3-mm-Seitenwand, beiden Radnaben und dem Schlauchstutzen, der den Zug des Schlauchs aufnimmt; die 10-mm-Stahlachse mit beiden Nabenlagern, Lenkrollenschaft und Achsstift, die zehn 0,6-mm-Schlauchwände mit ihren Gelenken, das Griffrohr auf seinem Drehgelenk, Außen- und Innenrohr und der Düsenkrümmer, dessen 35-mm-Gelenkwalze zwischen den beiden Gabelwangen des hohlen 2,5-mm-Düsengehäuses sitzt; Auflager sind ein Kontaktstreifen unter beiden Reifen, die Lauffläche der Lenkrolle und die zwei Borstenleisten; Massen liegen in den Schwerpunkten von Kessel (Motor und Kabeltrommel hinten), Rohr, Düse und Handgriff."
V["100"]["decisions"] = [
    {"en": "Body for a 15 mm feature: separate shell and lid lofts carry the handle, the intake collar and the grille plate. The two 25 mm rear tyres on their rims run on the 40 mm bearing bosses of the shell; the twin swivel wheels (12 mm each) are drawn as one 48 mm roller in its fork, as the 2D drawings of this scale do. The hose, wand and floor head retain their working dimensions.",
     "de": "Körper für 15 mm Detailgröße: Getrennte Wannen- und Deckelkörper tragen Griff, Ansaugstutzen und Gitterplatte. Die zwei 25 mm breiten Hinterreifen auf ihren Felgen laufen auf den 40-mm-Lagernaben der Schale; die Doppellenkrolle (je 12 mm) ist als eine 48 mm breite Rolle in ihrer Gabel dargestellt, wie in den Zeichnungen dieses Maßstabs. Schlauch, Saugrohr und Bodenkopf behalten ihre Arbeitsmaße."},
    {"en": "Thermal and structure: the same regions on the simpler body (lid as open dome, grille plate with its built-in back, outlet patch, the caster roller as one tyre surface); the structure keeps the base tub with its wall, the hose socket as a 15 mm walled sleeve that ties the hose to the tub, the axle, the ten hose walls, handle and wand tubes, the hollow nozzle housing, joints only at the wheels, the handle swivel and the nozzle pivot, the supports under tyres, roller and bristles and four masses.",
     "de": "Thermik und Tragwerk: dieselben Regionen am einfacheren Körper (Deckel als offene Kuppel, Gitterplatte mit eingebauter Rückseite, Austrittsfläche, die Lenkrolle als eine Reifenfläche); das Tragwerk behält Bodenwanne mit Wand, den Schlauchstutzen als Hülse mit 15 mm Wand, die den Schlauch an die Wanne bindet, die Achse, die zehn Schlauchwände, Griff- und Saugrohr, das hohle Düsengehäuse, Gelenke nur an den Rädern, am Griffgelenk und am Düsengelenk, die Auflager unter Reifen, Rolle und Borsten und vier Massen."},
    MOTOR]
V["200"]["decisions"] = [
    {"en": "Body for a 30 mm feature: one canister loft carries the handle and the intake collar. Each rear wheel is one 120 mm disc, 30 mm wide, with its bearing boss turned in one piece so it meets the canister; the swivel caster is one 50 mm roller that reaches into the underside of the shell, without a fork. The hose, wand and floor head retain their working dimensions.",
     "de": "Körper für 30 mm Detailgröße: Ein Kesselkörper trägt Griff und Ansaugstutzen. Jedes Hinterrad ist eine 120-mm-Scheibe von 30 mm Breite, mit ihrer Lagernabe aus einem Stück gedreht, sodass sie am Kessel anliegt; die Lenkrolle ist eine 50-mm-Rolle, die in die Unterseite der Schale greift, ohne Gabel. Schlauch, Saugrohr und Bodenkopf behalten ihre Arbeitsmaße."},
    {"en": "Thermal: shell and lid as their own skins under the merged body, the grille plate, both bristle strips and the outlet patch as thermal-only solids at their layer thicknesses, the wheel discs and the roller as the tyre surfaces. Structure: the base tub drawn up into the hollow tub wall, the hose socket as a solid plug between wall and hose, the axle, hose and tube walls (their bores exceed 30 mm), the nozzle as a solid block with its pivot, supports and masses.",
     "de": "Thermik: Schale und Deckel als eigene Häute unter dem zusammengelegten Körper, Gitterplatte, beide Borstenleisten und Austrittsfläche als reine Thermikkörper in ihren Schichtdicken, Radscheiben und Rolle als Reifenflächen. Tragwerk: die bis in die hohle Wannenwand hochgezogene Bodenwanne, der Schlauchstutzen als voller Zapfen zwischen Wand und Schlauch, die Achse, Schlauch- und Rohrwände (ihre Bohrungen übersteigen 30 mm), die Düse als voller Block mit Gelenk, Auflager und Massen."},
    V["200"]["decisions"][2], MOTOR]
V["500"]["decisions"] = [
    {"en": "Body for a 75 mm feature: the canister is a single shell with a domed top lofted through eight plan sections. The rear wheel pair is one 120 mm roller across the full 0.30 m track, as the wheel pair of the plan and front drawings of this scale; the caster is one 50 mm roller 75 mm wide under the nose. Hose, wand and floor head keep their lengths; the handle is a 60 mm sleeve.",
     "de": "Körper für 75 mm Detailgröße: Der Kessel ist eine einzige Hülle mit gewölbter Oberseite, durch acht Grundrissschnitte geloftet. Das Hinterradpaar ist eine 120-mm-Walze über die volle Spur von 0,30 m, wie das Radpaar in Grundriss und Vorderansicht dieses Maßstabs; die Lenkrolle ist eine 50-mm-Rolle von 75 mm Breite unter dem Bug. Schlauch, Saugrohr und Bodenkopf behalten ihre Längen; der Griff ist eine 60-mm-Hülse."},
    {"en": "Thermal: the regions keep their true sizes as thermal-only skins (hose, wand and handle at 36, 35 and 42 mm, nozzle head, brush, grille, outlet) under the generalised body; the wheels region follows the wheel roller and the caster roller of the body. Structure: a 75 mm base tub under the hollow tub wall carries the axle roller and the caster, solid 60 mm hose, handle and wand members (their bores are below 75 mm), the nozzle block, supports and masses.",
     "de": "Thermik: Die Regionen behalten als reine Thermikhäute ihre wahre Größe (Schlauch, Rohr und Griff in 36, 35 und 42 mm, Düsenkopf, Bürste, Gitter, Austritt) unter dem verallgemeinerten Körper; die Region Räder folgt der Radwalze und der Lenkrolle des Körpers. Tragwerk: Eine 75 mm hohe Bodenwanne unter der hohlen Wannenwand trägt Achswalze und Lenkrolle, volle 60-mm-Glieder für Schlauch, Griff und Rohr (ihre Bohrungen liegen unter 75 mm), der Düsenblock, Auflager und Massen."},
    V["500"]["decisions"][2], MOTOR]
V["50"]["decisions"] = d[:3] + [MOTOR]

json.dump(doc, open(P, "w"), ensure_ascii=False, indent=2)
print("written", P)
