"""herring-gull: rig, poses, clips, thermal (hand-entered numbers)"""
import math
from nw import *
from bird_common import *

ID = "herring-gull"
MASS = 1.0
ROOT_X = -0.17
HIPL = (0.15, -0.05)
LENS = (0.06, 0.10, 0.065)
CIF = (0.04, -0.010)

REFS = [
    ("cramp-simmons-1983", "Cramp, S. (ed.) (1983). Handbook of the Birds of Europe, the Middle East and North Africa: The Birds of the Western Palearctic, Vol. III (Waders to Gulls). Oxford University Press. Herring gull Larus argentatus: length 56-66 cm, wingspan 123-155 cm, mass about 0.7-1.5 kg.", None),
    ("pennycuick-1989", "Pennycuick, C. J. (1989). Bird Flight Performance: A Practical Calculation Manual. Oxford University Press. Wing loading, glide polar and soaring of gulls; the glide attitude here is an authored estimate guided by it.", None),
    ("lasiewski-dawson-1967", "Lasiewski, R. C., Dawson, W. R. (1967). A re-examination of the relation between standard metabolic rate and body weight in birds. The Condor 69, 13-23. Non-passerine standard metabolic rate 78.3 M^0.723 kcal/day (M in kg).", None),
    ("lasiewski-calder-1971", "Lasiewski, R. C., Calder, W. A. (1971). A preliminary allometric analysis of respiratory variables in resting birds. Respiration Physiology 11, 152-166. Resting frequency and tidal volume scale with body mass; the gull values are authored estimates guided by these allometries.", None),
    ("walsberg-king-1978", "Walsberg, G. E., King, J. R. (1978). The relationship of the external surface area of birds to body mass. Journal of Experimental Biology 76, 185-189. Plumage-covered area about 8.1 M^0.667 cm2 (M in g).", None),
    ("scholander-1950", "Scholander, P. F., Walters, V., Hock, R., Irving, L. (1950). Body insulation of some arctic and tropical mammals and birds. Biological Bulletin 99, 225-236.", None),
    ("de-vries-van-eerden-1995", "de Vries, J., van Eerden, M. R. (1995). Thermal conductance in aquatic birds in relation to the degree of water contact, body mass, and body fat. Physiological Zoology 68, 1143-1163. Layer resistances here are authored estimates guided by it.", None),
    ("calder-king-1974", "Calder, W. A., King, J. R. (1974). Thermal and caloric relations of birds. In: Farner, D. S., King, J. R. (eds), Avian Biology IV, Academic Press, pp. 259-413. Body temperature of non-passerines about 40-42 degC.", None),
    ("kahl-1963", "Kahl, M. P. (1963). Thermoregulation in the wood stork, with special reference to the role of the legs. Physiological Zoology 36, 141-151. Bare legs and feet are the controlled heat-loss path of long-legged waterbirds.", None),
    ("schmidt-nielsen-1970", "Schmidt-Nielsen, K., Hainsworth, F. R., Murrish, D. E. (1970). Counter-current heat exchange in the respiratory passages: effect on water and heat balance. Respiration Physiology 9, 263-276.", None),
    ("gupta-2010", "Gupta, J. K., Lin, C.-H., Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39. Nares areas and jet angles of the gull are authored estimates guided by the human case.", "https://research.polyu.edu.hk/en/publications/characterizing-exhaled-airflow-from-breathing-and-talking/"),
    ("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort: emissivity of skin and respiratory heat loss.", None),
    ("kleiber-1961", "Kleiber, M. (1961). The Fire of Life: An Introduction to Animal Energetics. Wiley, New York.", None),
]

MATERIALS = [
    material("plumage", ("White body plumage over down", "Weißes Körpergefieder über Daunen"), 0.95, 0.3, 100, 0.04, None, "authored-estimate", "scholander-1950"),
    material("wing-feathers", ("Pale grey mantle and wing feathers with black tips", "Hellgraue Mantel- und Flügelfedern mit schwarzen Spitzen"), 0.95, 0.55, 120, 0.05, None, "authored-estimate", "scholander-1950"),
    material("bill-keratin", ("Yellow bill keratin with red spot", "Gelbes Schnabelhorn mit rotem Fleck"), 0.92, 0.5, None, None, None, "authored-estimate", "ashrae-2021"),
    material("scaled-skin", ("Scaled pink skin of tarsus and web", "Beschuppte rosa Haut von Lauf und Schwimmhaut"), 0.96, 0.7, None, None, None, "literature", "ashrae-2021"),
]


def rig():
    B = bone
    bones = [
        B("trunk", None, 0.0, 0.0, 0.0, 0.34, (-80, 80), ("Trunk: keel, flight muscles and gut", "Rumpf: Kiel, Flugmuskeln und Darm"), kg=0.0, com=(0.17, 0.0), dist="point"),
        B("tail", "trunk", 0.0, 0.01, 175.0, 0.14, (-60, 60), ("Tail with twelve rectrices (fan)", "Schwanz mit zwölf Steuerfedern (Fächer)"), kg=0.03),
        B("neck-1", "trunk", 0.29, 0.035, 55.0, 0.08, (-80, 80), ("Lower neck", "Unterer Hals"), kg=0.03),
        B("neck-2", "neck-1", 0.08, 0.0, 20.0, 0.07, (-80, 80), ("Upper neck", "Oberer Hals"), kg=0.025),
        B("head", "neck-2", 0.07, 0.0, -85.0, 0.125, (-90, 90), ("Head with strong bill (skull and bill)", "Kopf mit kräftigem Schnabel (Schädel und Schnabel)"), kg=0.1, com=(0.045, 0.0), dist="point"),
    ]
    for s, nm, tg in (("n", "near", "Naher"), ("f", "far", "Ferner")):
        bones += [
            B(f"arm-{s}", "trunk", 0.20, 0.04, 180.0, 0.20, (-150, 60), (f"{nm.capitalize()} arm (humerus and forearm)", f"{tg} Arm (Ober- und Unterarm)"), kg=0.045),
            B(f"hand-{s}", f"arm-{s}", 0.20, 0.0, 0.0, 0.22, (-100, 100), (f"{nm.capitalize()} hand with primaries", f"{tg} Hand mit Handschwingen"), kg=0.04),
        ]
    for s, nm, tg in (("n", "near", "Nahes"), ("f", "far", "Fernes")):
        bones += [
            B(f"thigh-{s}", "trunk", 0.15, -0.05, -60.0, 0.06, (-130, 130), (f"{nm.capitalize()} thigh (inside the plumage)", f"{tg} Oberschenkel (im Gefieder)"), kg=0.04),
            B(f"shank-{s}", f"thigh-{s}", 0.06, 0.0, -50.0, 0.10, (-170, 60), (f"{nm.capitalize()} shank (tibiotarsus)", f"{tg} Unterschenkel (Tibiotarsus)"), kg=0.028),
            B(f"tarsus-{s}", f"shank-{s}", 0.10, 0.0, 45.0, 0.065, (-140, 110), (f"{nm.capitalize()} tarsus (bare)", f"{tg} Lauf (unbefiedert)"), kg=0.008),
            B(f"foot-{s}", f"tarsus-{s}", 0.065, 0.0, 75.0, 0.075, (-100, 100), (f"{nm.capitalize()} webbed foot", f"{tg} Schwimmfuß"), kg=0.01),
        ]
    for b in bones:
        if "mass" in b: b["mass"]["reference"] = "cramp-simmons-1983"
    other = sum(b["mass"]["kg"] for b in bones if "mass" in b)
    bones[0]["mass"]["kg"] = round(MASS - other, 4)
    bones[0]["mass"]["source"] = "authored-estimate"
    bones[0]["mass"]["reference"] = "cramp-simmons-1983"
    anchors = [
        anchor("contact-n", "foot-n", 0.04, -0.010, "contact", ("Near foot, centre of the web", "Naher Fuß, Mitte der Schwimmhaut"), "contact", 90, 0.07, 0.05, 0.7),
        anchor("contact-f", "foot-f", 0.04, -0.010, "contact", ("Far foot, centre of the web", "Ferner Fuß, Mitte der Schwimmhaut"), "contact", 90, 0.07, 0.05, 0.7),
        anchor("nostrils", "head", 0.075, 0.006, "emitter", ("Nares slits in the bill", "Nasenschlitze im Schnabel"), None),
        anchor("bill-tip", "head", 0.125, -0.004, "attachment", ("Bill tip with the red spot", "Schnabelspitze mit rotem Fleck"), None),
        anchor("eye", "head", 0.035, 0.012, "attachment", ("Eye", "Auge"), None),
        anchor("back-plume", "trunk", 0.17, 0.11, "emitter", ("Back above the trunk (plume source)", "Rücken über dem Rumpf (Fahnenquelle)"), None),
    ]
    return bones, anchors


def regions():
    ar = dict(body=0.050, neck=0.010, head=0.0055, bill=0.0020, wings=0.024, tail=0.0075, legs=0.0033, feet=0.0037)
    sh = dict(body=0.70, neck=0.06, head=0.07, bill=0.01, wings=0.08, tail=0.01, legs=0.05, feet=0.02)
    mats = dict(body=(0.5, 0.024), neck=(0.3, 0.014), head=(0.2, 0.008), wing=(0.35, 0.016), tail=(0.2, 0.01))
    return bird_regions(ar, sh, mats)


def land(sun=0.5, ground=14, legs_c=0.12, feet_c=0.45, wing_e=0.5):
    return {"body-plumage": rb(1.0, sun), "neck-plumage": rb(1.0, sun), "head-plumage": rb(1.0, sun), "bill": rb(1.0, sun),
            "wing-plumage": rb(wing_e, sun*0.8), "tail-plumage": rb(1.0, sun*0.6), "legs-skin": rb(1.0, sun*0.5, (legs_c, ground) if legs_c else None),
            "web-feet": rb(1.0, sun*0.3, (feet_c, ground) if feet_c else None)}


def air(sun=0.6, legs_e=0.35):
    return {"body-plumage": rb(1.0, sun), "neck-plumage": rb(1.0, sun), "head-plumage": rb(1.0, sun), "bill": rb(1.0, sun),
            "wing-plumage": rb(1.0, sun), "tail-plumage": rb(1.0, sun*0.8), "legs-skin": rb(legs_e, sun*0.3), "web-feet": rb(legs_e, sun*0.3)}


def build():
    bones, anchors = rig()
    asset = {"rig": {"kind": "anatomical", "root": "trunk", "bones": bones}, "anchors": anchors}
    L = Legs(asset, ROOT_X, HIPL, LENS, CIF)

    def pb(pitch, neck, wings=None, tail=None, legs=None, tail_world=None):
        b = {"trunk": {"angleDeg": pitch}}
        b.update(offsets_from_world(asset, pitch, ["neck-1", "neck-2", "head"], list(neck)))
        if wings:
            for s, (a1, a2) in zip("nf", wings):
                b.update(offsets_from_world(asset, pitch, [f"arm-{s}", f"hand-{s}"], [a1, a2]))
        if tail is not None: b["tail"] = {"angleDeg": tail}
        if legs:
            for s in "nf":
                b.update(offsets_from_world(asset, pitch, [f"{n}-{s}" for n in ("thigh", "shank", "tarsus", "foot")], list(legs)))
        return b

    ry0 = 0.200 - (math.sin(math.radians(20))*HIPL[0] + math.cos(math.radians(20))*HIPL[1])
    stand = pb(20, (66, 90, -12), wings=[(-160 + 0, -160), (-160, -160)])
    for s, o in zip("nf", (0.03, -0.03)):
        stand.update(L.offsets(s, 20, ry0, (L.hip_x(20) + o, 0.0), -75, 0))
    def with_stand_wings(b):
        for s in "nf":
            b.update(offsets_from_world(asset, 20, [f"arm-{s}", f"hand-{s}"], [182, 178]))
        return b
    stand = with_stand_wings(stand)
    P = {}
    P["standing"] = pose("standing", ("Standing", "Stehend"), (
        "Standing upright on both pink webbed feet, body tilted 20 degrees, neck upright with the head slightly down, wings folded with the black primaries just beyond the tail.",
        "Aufrecht auf beiden rosa Schwimmfüßen stehend, Körper um 20 Grad geneigt, Hals aufrecht mit leicht gesenktem Kopf, Flügel angelegt, die schwarzen Handschwingen ragen knapp über den Schwanz."),
        stand, ["contact-n", "contact-f"], land(), {"plume-back": 1.0}, 1.5, "rest", {"x": ROOT_X})
    glide_legs = (-125, -130, -115, -100)
    glide = pb(-2, (22, 14, -6), wings=[(132, 168), (146, 176)], tail=-14, legs=glide_legs)
    P["gliding"] = pose("gliding", ("Gliding with spread wings", "Gleiten mit gespreizten Flügeln"), (
        "Soaring in lift at about 1.5 m above the ground: both wings spread, drawn in oblique projection with the wrist raised and the hand swept back (the span of 1.45 m is foreshortened in elevation), neck drawn in, tail fanned, feet trailing.",
        "Segeln in der Aufwindzone etwa 1,5 m über dem Boden: beide Flügel gespreizt, in schräger Projektion gezeichnet mit angehobenem Handgelenk und zurückgeschwungener Hand (die Spannweite von 1,45 m ist im Aufriss verkürzt), Hals eingezogen, Schwanz gefächert, Füße nach hinten."),
        glide, [], air(), {"plume-back": 0.6}, 2.0, "glide", {"x": ROOT_X, "y": 1.5}, ground="authored")
    flare = pb(30, (40, 60, -22), wings=[(100, 128), (112, 136)], tail=30, legs=(-80, -95, -90, -35))
    P["landing"] = pose("landing", ("Landing flare", "Landeanflug mit Abfangen"), (
        "The flare just before touchdown: body pitched up 30 degrees, wings raised high and cupped to brake, tail fanned down, legs and webbed feet stretched forward and down.",
        "Das Abfangen kurz vor dem Aufsetzen: Körper um 30 Grad angehoben, Flügel hoch und gewölbt zum Bremsen, Schwanz nach unten gefächert, Beine und Schwimmfüße nach vorn und unten gestreckt."),
        flare, [], air(0.6, 0.9), {"plume-back": 0.3}, 5.0, "flare", {"x": ROOT_X, "y": 0.55}, ground="authored")
    P["gliding-up"] = pose("gliding-up", ("Soaring, wings up", "Segeln, Flügel oben"), ("Wings 4 degrees higher in the gust.", "Flügel 4 Grad höher in der Böe."),
        pb(-1, (22, 14, -6), wings=[(136, 170), (150, 178)], tail=-12, legs=glide_legs), [], air(), {"plume-back": 0.6}, 2.0, "glide", {"x": ROOT_X, "y": 1.5}, ground="authored", key=True)
    P["gliding-down"] = pose("gliding-down", ("Soaring, wings down", "Segeln, Flügel unten"), ("Wings 4 degrees lower.", "Flügel 4 Grad tiefer."),
        pb(-3, (22, 14, -6), wings=[(128, 166), (142, 174)], tail=-16, legs=glide_legs), [], air(), {"plume-back": 0.6}, 2.0, "glide", {"x": ROOT_X, "y": 1.5}, ground="authored", key=True)
    # landing keys
    def ground_y(bones, contacts):
        tmp = {"id": "t", "bones": bones, "root": {"x": ROOT_X}, "ground": "auto", "contacts": contacts}
        W = pose_world(asset | {"poses": []}, tmp)
        return round(W["trunk"][5], 4)
    td_b = pb(18, (62, 86, -10), wings=[(112, 140), (120, 146)], tail=20)
    for s, o in zip("nf", (0.10, 0.07)):
        td_b.update(L.offsets(s, 18, ry0 + 0.0, (L.hip_x(18) + o, 0.0), -70, 0))
    ty = ground_y(td_b, ["contact-n", "contact-f"])
    P["touchdown"] = pose("touchdown", ("Touchdown", "Aufsetzen"), ("Both feet touch down with the wings still raised.", "Beide Füße setzen auf, die Flügel sind noch angehoben."),
        td_b, ["contact-n", "contact-f"], land(wing_e=1.0), {"plume-back": 0.6}, 4.0, "flare", {"x": ROOT_X, "y": ty}, ground="authored", key=True)
    sy = ground_y(stand, ["contact-n", "contact-f"])
    P["settled"] = pose("settled", ("Settled after landing", "Nach der Landung ruhend"), ("Wings folded, standing.", "Flügel angelegt, stehend."),
        stand, ["contact-n", "contact-f"], land(), {"plume-back": 1.0}, 2.0, "rest", {"x": ROOT_X, "y": sy}, ground="authored", key=True)
    def apr(k, x, y, name):
        q = dict(P["gliding"]); q = json.loads(json.dumps(P["gliding"])); q["id"] = name; q["key"] = True
        q["title"] = tx(f"Approach {k}", f"Anflug {k}"); q["description"] = tx("Key pose of the landing approach.", "Schlüsselpose des Landeanflugs.")
        q["root"] = {"x": x, "y": y}
        return q
    P["approach-a"] = apr("a", ROOT_X - 1.4, 1.5, "approach-a")
    fl = json.loads(json.dumps(P["landing"])); fl["id"] = "flare-key"; fl["key"] = True; fl["root"] = {"x": ROOT_X - 0.55, "y": 0.55}
    fl["title"] = tx("Flare (key)", "Abfangen (Schlüssel)"); fl["description"] = tx("Key pose of the flare.", "Schlüsselpose des Abfangens.")
    P["flare-key"] = fl
    asset["poses"] = list(P.values())
    asset["_ty"] = ty
    asset["_sy"] = sy
    return asset


def clips():
    soar = {"id": "soar", "title": tx("Soaring in lift", "Segeln im Aufwind"), "description": tx(
        "Hovering soaring of a gull in ridge lift: the wings flex 4 degrees up and down with the gusts over 4 s while the body stays level; the clip is a ballistic phase, nothing touches the ground.",
        "Rüttelndes Segeln einer Möwe im Hangaufwind: Die Flügel federn über 4 s mit den Böen um 4 Grad auf und ab, während der Körper waagerecht bleibt; der Clip ist eine ballistische Phase, nichts berührt den Boden."),
        "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0.0, "pose": "gliding", "ease": "smooth"}, {"t": 1.0, "pose": "gliding-up", "ease": "smooth"}, {"t": 2.0, "pose": "gliding", "ease": "smooth"}, {"t": 3.0, "pose": "gliding-down", "ease": "smooth"}, {"t": 4.0, "pose": "gliding", "ease": "smooth"}],
        "contacts": [], "phases": [{"id": "soar", "fromS": 0, "toS": 4.0, "title": tx("Soaring in lift", "Segeln im Aufwind"), "ballistic": True}],
        "events": [{"t": 1.0, "kind": "motion", "title": tx("Gust lifts the wings", "Böe hebt die Flügel")}, {"t": 3.0, "kind": "motion", "title": tx("Wings sink", "Flügel sinken ab")}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.0}, {"t": 1.0, "v": 2.3}, {"t": 3.0, "v": 1.8}, {"t": 4.0, "v": 2.0}]}]}
    landing = {"id": "landing", "title": tx("Landing", "Landung"), "description": tx(
        "A landing in 3.2 s: gliding approach, flare with raised wings and forward feet, touchdown with both feet at 2.0 s and folding of the wings to the standing posture; breathing steps from gliding to the flare pattern and back to rest.",
        "Eine Landung in 3,2 s: Gleitanflug, Abfangen mit angehobenen Flügeln und Füßen nach vorn, Aufsetzen mit beiden Füßen bei 2,0 s und Anlegen der Flügel in die Standhaltung; die Atmung wechselt vom Gleiten zum Abfangmuster und zurück zur Ruhe."),
        "durationS": 3.2, "fps": 30, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0.0, "pose": "approach-a", "ease": "smooth"}, {"t": 1.3, "pose": "flare-key", "ease": "smooth"}, {"t": 2.0, "pose": "touchdown", "ease": "smooth"}, {"t": 3.2, "pose": "settled", "ease": "smooth"}],
        "contacts": [{"anchor": "contact-n", "fromS": 2.0, "toS": 3.2}, {"anchor": "contact-f", "fromS": 2.0, "toS": 3.2}],
        "phases": [{"id": "approach", "fromS": 0, "toS": 2.0, "title": tx("Approach and flare", "Anflug und Abfangen"), "ballistic": True}, {"id": "ground", "fromS": 2.0, "toS": 3.2, "title": tx("On the ground, folding the wings", "Am Boden, Flügel anlegen")}],
        "events": [{"t": 2.0, "kind": "contact-on", "title": tx("Feet touch down", "Füße setzen auf"), "target": "contact-n"}, {"t": 2.0, "kind": "contact-on", "title": tx("Far foot touches down", "Ferner Fuß setzt auf"), "target": "contact-f"}],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1.0, "v": 2}, {"t": 2.4, "v": 0}]},
                   {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.0}, {"t": 1.3, "v": 5.0}, {"t": 2.0, "v": 4.0}, {"t": 3.2, "v": 2.0}]}]}
    return [soar, landing]


def thermal():
    regs, skin = regions()
    pats = [pattern("rest", ("Resting, nostril breathing", "Ruhe, Nasenatmung"), 18, 0.013, 0.45, 0.03, 0.45, 0.07, "lasiewski-calder-1971"),
            pattern("glide", ("Soaring, steady breathing", "Segeln, gleichmäßige Atmung"), 40, 0.016, 0.47, 0.01, 0.47, 0.05, "lasiewski-calder-1971"),
            pattern("flare", ("Flapping and landing effort", "Flügelschlag und Landeanstrengung"), 90, 0.022, 0.5, 0.0, 0.5, 0.0, "lasiewski-calder-1971")]
    return {
        "environment": {"airTempC": 12, "meanRadiantTempC": 12, "skyTempC": 2, "groundTempC": 14, "relativeHumidity": 0.75, "windMS": 4.0, "windDirectionDeg": 180, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
        "baseMet": 1.5,
        "body": {"species": "gull", "massKg": MASS, "skinAreaM2": round(skin, 4), "basalMetabolismW": 3.8, "metWM2": round(3.8/skin, 1), "coreSetPointC": 41.0, "skinSetPointC": 38.5, "sweating": False, "panting": True,
                 "maxSkinBloodFlowLM2H": 70, "minSkinBloodFlowLM2H": 5, "source": "authored-estimate", "reference": "lasiewski-dawson-1967"},
        "respiration": {"noseEmitter": "breath-nares", "exhaledTempC": 30, "exhaledRelativeHumidity": 0.95, "patterns": pats, "motion": [{"bone": "trunk", "angleDeg": 1.2, "dy": 0.0015}], "source": "authored-estimate", "reference": "schmidt-nielsen-1970"},
        "passengers": [], "operating": [], "regions": regs,
        "emitters": [
            {"id": "breath-nares", "kind": "breath", "title": tx("Nares slits in the bill (both, combined)", "Nasenschlitze im Schnabel (beide zusammen)"), "anchor": "nostrils", "directionDeg": -15, "diameterM": 0.006, "source": "authored-estimate", "reference": "gupta-2010"},
            {"id": "plume-back", "kind": "plume", "title": tx("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.16, "region": "body-plumage", "source": "authored-estimate", "reference": "scholander-1950"}],
        "assumptions": [
            tx("Surface temperatures are solved, not authored: heat flows from the 41 degC core through tissue and the plumage layers (resistance about 0.5 m2K/W on the body, 0.2-0.35 elsewhere, authored estimates guided by plumage conductance data of waterbirds); the 4 m/s coastal wind acts through the convection model.",
               "Oberflächentemperaturen werden gelöst, nicht vorgegeben: Wärme fließt vom 41-Grad-Kern durch Gewebe und die Gefiederschichten (Widerstand rund 0,5 m2K/W am Rumpf, 0,2-0,35 sonst, Schätzwerte nach Leitwertdaten von Wasservögeln); der Küstenwind von 4 m/s wirkt über das Konvektionsmodell."),
            tx("Bare legs and feet carry 7 % of the heat production as a controlled heat-loss path; in flight they are held against the tail and bind a reduced exposed fraction.",
               "Unbefiederte Beine und Füße tragen 7 % der Wärmeproduktion als geregelten Wärmeabgabeweg; im Flug werden sie an den Schwanz gelegt und binden einen verringerten freien Anteil."),
            tx("Birds breathe through the nares; the exhaled air leaves at 30 degC and 95 % relative humidity.", "Vögel atmen durch die Nasenlöcher; die Ausatemluft verlässt sie mit 30 Grad und 95 % relativer Feuchte."),
            tx("Spread wings in elevation are an oblique projection: the 1.45 m wingspan is not visible in the side view, but the thermal wing region keeps its full area.",
               "Gespreizte Flügel sind im Aufriss eine schräge Projektion: die Spannweite von 1,45 m ist in der Seitenansicht nicht sichtbar, die thermische Flügelregion behält aber ihre volle Fläche.")]}


def base():
    a = build()
    ty, sy = a.pop("_ty"), a.pop("_sy")
    a["clips"] = clips()
    a["thermal"] = thermal()
    a["materials"] = MATERIALS
    return a


if __name__ == "__main__":
    import sys
    a = base()
    print("limits", check_limits(a))
    for pid in sys.argv[1:] or ["standing"]:
        skel(a, pid, f"{GEN}/skel-gull-{pid}.png", view=(-0.9, -0.2, 0.9, 1.9) if pid in ("gliding", "landing") else (-0.45, -0.1, 0.45, 0.65), den=5)
