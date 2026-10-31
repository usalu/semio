"""Builds S/🗂️catalog/👤️humans/parent-carrying-toddler/🔣️.json"""
import math
from lib import *
from common_build import *
from parent_rig import *
import parent_art

anchors = build_anchors()
A_SKIN = dubois(H, MASS)
T_SKIN = dubois(TH, TMASS)
T_PER = 1.0
phases = {"heel-strike-l": 0.0, "walk-1": 0.125, "midstance-l": 0.25, "walk-3": 0.375, "walk-4": 0.5, "walk-5": 0.625, "walk-6": 0.75, "walk-7": 0.875}
P = {}
P.update(poses())
for k, (name, phi) in enumerate(phases.items()):
    f, hp = make_walk(phi)
    arms_carry(f, swing=18 * math.sin(2 * math.pi * phi + math.pi))
    toddler_pose(f, bob=3 * math.sin(4 * math.pi * phi), look=0)
    f.rel("toddler-thigh-n", 10 * math.sin(2 * math.pi * phi))
    f.rel("toddler-thigh-f", -10 * math.sin(2 * math.pi * phi))
    P[name] = f
f = Fig(bones)
f.aim("torso", 98)
f.aim("head", 96)
f.leg("l", (0.07, -(L["thigh"] + L["shank"]) * 0.90), 1)
f.leg("r", (0.055, -(L["thigh"] + L["shank"]) * 0.90), 1)
f.aim("foot-l", 0)
f.aim("foot-r", 0)
hip = f.world("pelvis")[:2]
f.arm("l", (hip[0] + 0.155, hip[1] - 0.01), -1)
f.aim("hand-l", f.world("forearm-l")[2] + 10)
f.aim("upper-arm-r", -84)
f.aim("forearm-r", -70)
f.aim("hand-r", -70)
toddler_pose(f, bob=-4, look=5)
f.rel("toddler-torso", 6)
P["hoisting"] = f
for n, g in P.items():
    r = joint_report(g)
    if r:
        print("LIMIT", n, r)

MAT = [
    {"id": "skin", "title": tx("Skin", "Haut"), "emissivity": 0.98, "solarAbsorptivity": 0.6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "knit-hat", "title": tx("Wool knit (hat, scarf, mittens)", "Wollstrick (Mütze, Schal, Fäustlinge)"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "wool", "title": tx("Wool coat cloth", "Wollmantelstoff"), "emissivity": 0.92, "solarAbsorptivity": 0.75, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "denim", "title": tx("Denim jeans", "Jeansstoff"), "emissivity": 0.92, "solarAbsorptivity": 0.7, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "leather", "title": tx("Leather ankle boot", "Lederstiefelette"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "source": "authored-estimate"},
    {"id": "leather-glove", "title": tx("Leather glove", "Lederhandschuh"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "source": "authored-estimate"},
    {"id": "padded-suit", "title": tx("Padded toddler overall (polyester fill)", "Gefütterter Kleinkind-Overall (Polyesterfüllung)"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "rubber", "title": tx("Rubber toddler boot", "Gummistiefel für Kleinkinder"), "emissivity": 0.92, "solarAbsorptivity": 0.75, "source": "authored-estimate"},
]
fc = dict(coat=1.3, jeans=1.12, suit=1.45)


def regions():
    R = []
    R.append(region("face", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.04 * A_SKIN, 0.15, "sphere", 0.45, [], 15000, "head", 0.06, 0.04, ref="dubois-1916"))
    R.append(region("beanie", "Knit beanie with hair", "Strickmütze mit Haar", "hair", "knit-hat", 0.04 * A_SKIN * 1.1, 0.16, "sphere", 0.55, [layer("beanie-air", "Hair and air under the beanie", "Haar und Luft unter der Mütze", "air-gap", "knit-hat", 0.006, 0.03), layer("beanie", "Knit beanie (0.15 clo)", "Strickmütze (0,15 clo)", "clothing", "knit-hat", 0.004, clo_r(0.15, 1.1), 600)], 4000, "head", 0.04, 0.04))
    R.append(region("scarf", "Knitted scarf", "Strickschal", "clothing", "knit-hat", 0.03 * A_SKIN * 1.5, 0.1, "cylinder", 0.5, [layer("scarf", "Wool scarf (0.3 clo)", "Wollschal (0,3 clo)", "clothing", "knit-hat", 0.01, clo_r(0.30, 1.5), 900)], 6000, "trunk", 0.01, 0.03))
    R.append(region("coat-torso", "Wool coat, trunk and skirt", "Wollmantel, Rumpf und Schoß", "clothing", "wool", 0.33 * A_SKIN * fc["coat"], 0.35, "cylinder", 0.5, [
        layer("torso-air", "Air between skin and sweater", "Luft zwischen Haut und Pullover", "air-gap", "wool", 0.008, 0.03),
        layer("sweater", "Knit sweater (0.30 clo)", "Strickpullover (0,30 clo)", "clothing", "wool", 0.003, clo_r(0.30, fc["coat"]), 700),
        layer("coat", "Wool coat (0.40 clo)", "Wollmantel (0,40 clo)", "clothing", "wool", 0.006, clo_r(0.40, fc["coat"]), 1500)], 20000, "trunk", 0.38, 0.33))
    R.append(region("coat-sleeves", "Coat and sweater sleeves", "Mantel- und Pulloverärmel", "clothing", "wool", 0.14 * A_SKIN * fc["coat"], 0.09, "cylinder", 0.5, [
        layer("sleeve-air", "Air in the sleeve", "Luft im Ärmel", "air-gap", "wool", 0.006, 0.03),
        layer("sleeve", "Sweater and coat sleeve (0.55 clo)", "Pullover- und Mantelärmel (0,55 clo)", "clothing", "wool", 0.008, clo_r(0.55, fc["coat"]), 1500)], 16000, "arm", 0.08, 0.14))
    R.append(region("gloves", "Leather gloves", "Lederhandschuhe", "clothing", "leather-glove", 0.05 * A_SKIN * 1.1, 0.06, "cylinder", 0.5, [layer("glove", "Lined leather glove (0.2 clo)", "Gefütterter Lederhandschuh (0,2 clo)", "clothing", "leather-glove", 0.003, clo_r(0.20, 1.1), 700)], 6000, "hand", 0.02, 0.05))
    R.append(region("jeans", "Denim jeans", "Jeans", "clothing", "denim", 0.31 * A_SKIN * fc["jeans"], 0.12, "cylinder", 0.35, [layer("leg-air", "Air between skin and jeans", "Luft zwischen Haut und Jeans", "air-gap", "denim", 0.005, 0.03), layer("denim", "Denim (0.26 clo)", "Jeansstoff (0,26 clo)", "clothing", "denim", 0.001, clo_r(0.26, fc["jeans"]), 550)], 18000, "leg", 0.34, 0.31))
    R.append(region("boots", "Leather ankle boots with socks", "Lederstiefeletten mit Socken", "clothing", "leather", 0.06 * A_SKIN, 0.12, "horizontal-plate-up", 0.25, [layer("sock", "Wool sock (0.1 clo)", "Wollsocke (0,1 clo)", "clothing", "leather", 0.003, 0.0155, 400), layer("boot-air", "Air in the boot", "Luft im Stiefel", "air-gap", "leather", 0.008, 0.035), layer("boot", "Leather upper (0.1 clo)", "Lederschaft (0,1 clo)", "shell", "leather", 0.004, 0.03, 1100)], 8000, "foot", 0.07, 0.06))
    # toddler (passenger "toddler")
    ts = "toddler"
    R.append(region("toddler-face", "Toddler face and neck", "Gesicht und Hals des Kleinkinds", "skin", "skin", 0.06 * T_SKIN, 0.08, "sphere", 0.5, [], 12000, "head", 0.10, 0.06, subject=ts, ref="dubois-1916"))
    R.append(region("toddler-hat", "Toddler knit hat with hair", "Strickmütze des Kleinkinds mit Haar", "hair", "knit-hat", 0.06 * T_SKIN * 1.3, 0.09, "sphere", 0.55, [layer("hat-air", "Hair and air under the hat", "Haar und Luft unter der Mütze", "air-gap", "knit-hat", 0.005, 0.03), layer("hat", "Knit hat (0.3 clo)", "Strickmütze (0,3 clo)", "clothing", "knit-hat", 0.006, clo_r(0.30, 1.3), 700)], 3500, "head", 0.05, 0.06, subject=ts))
    R.append(region("toddler-suit", "Padded overall, trunk, arms and legs", "Gefütterter Overall, Rumpf, Arme und Beine", "clothing", "padded-suit", 0.76 * T_SKIN * fc["suit"], 0.12, "cylinder", 0.45, [layer("suit-air", "Air under the overall", "Luft unter dem Overall", "air-gap", "padded-suit", 0.008, 0.035), layer("suit", "Padded overall over a body suit (0.9 clo)", "Gefütterter Overall über Body (0,9 clo)", "clothing", "padded-suit", 0.012, clo_r(0.90, fc["suit"]), 1800)], 12000, "trunk", 0.75, 0.76, subject=ts))
    R.append(region("toddler-mittens", "Toddler mittens", "Fäustlinge des Kleinkinds", "clothing", "knit-hat", 0.05 * T_SKIN * 1.3, 0.04, "cylinder", 0.5, [layer("mitten", "Knit mitten (0.3 clo)", "Strickfäustling (0,3 clo)", "clothing", "knit-hat", 0.006, clo_r(0.30, 1.3), 500)], 4000, "hand", 0.04, 0.05, subject=ts))
    R.append(region("toddler-boots", "Toddler rubber boots with socks", "Gummistiefel des Kleinkinds mit Socken", "clothing", "rubber", 0.07 * T_SKIN, 0.08, "horizontal-plate-up", 0.25, [layer("sock", "Wool sock (0.15 clo)", "Wollsocke (0,15 clo)", "clothing", "rubber", 0.003, 0.0233, 400), layer("boot-air", "Air in the boot", "Luft im Stiefel", "air-gap", "rubber", 0.006, 0.03), layer("boot", "Rubber boot shell", "Gummischaft", "shell", "rubber", 0.004, 0.012, 1500)], 6000, "foot", 0.06, 0.07, subject=ts))
    return R


REG = regions()
RIDS = [r["id"] for r in REG]


def bind(act, br, tact, tbr, sun=0.15, plume=0.5):
    reg = {}
    for rid in RIDS:
        if rid.startswith("toddler"):
            reg[rid] = rbind(1.0 if rid != "toddler-suit" else 0.85, sun, contact=(0.15, 30) if rid == "toddler-suit" else None)
        elif rid == "boots":
            reg[rid] = rbind(0.85, sun * 0.6, contact=(0.15, 8))
        elif rid == "coat-torso":
            reg[rid] = rbind(0.85, sun, contact=(0.15, 30))
        else:
            reg[rid] = rbind(1.0, sun)
    return {"activityMet": act, "breathing": br, "passengers": {"toddler": {"activityMet": tact, "breathing": tbr}}, "regions": reg, "emitters": {"plume": plume, "toddler-plume": 0.4}}


def contacts_for(phi):
    c = []
    for foot, ph in (("l", phi), ("r", (phi + 0.5) % 1.0)):
        if ph < 0.62:
            if ph < 0.36:
                c.append(f"heel-{foot}")
            if ph > 0.06 or ph == 0.0 and False:
                c.append(f"contact-{foot}")
    if not c:
        c = ["contact-l"]
    return c


STAND = ["heel-l", "contact-l", "heel-r", "contact-r"]


def pose_list():
    out = []
    out.append(pose_record("standing-carry", "Standing with the toddler on the hip", "Stehen mit dem Kleinkind auf der Hüfte", "Upright stance, the near arm cradles the toddler's seat, the far arm hangs; the toddler looks ahead and holds the parent's shoulder.", "Aufrechter Stand, der nahe Arm trägt das Gesäß des Kleinkinds, der ferne Arm hängt; das Kleinkind schaut nach vorn und hält sich an der Schulter fest.", P["standing-carry"], STAND, bind(1.4, "rest", 1.0, "rest")))
    for name, phi in phases.items():
        key = name not in ("heel-strike-l",)
        en = {"heel-strike-l": ("Heel strike, left", "Fersenaufsatz links"), "midstance-l": ("Midstance, left", "Mittlere Standphase links")}.get(name, (f"Walking, phase {phi:.3f}", f"Gehen, Phase {phi:.3f}"))
        out.append(pose_record(name, en[0], en[1], f"Walking while carrying: gait phase {phi:.3f} of the left foot with the right foot half a cycle later; the toddler bobs on the hip.", f"Gehen mit Last: Gangphase {phi:.3f} des linken Fußes, der rechte Fuß folgt einen halben Zyklus später; das Kleinkind wippt auf der Hüfte.", P[name], contacts_for(phi), bind(2.8, "walk", 1.2, "rest"), key=key))
    out.append(pose_record("hoisting", "Hoisting the toddler up", "Das Kleinkind höher nehmen", "Knees bent and trunk leaning back while the near arm lifts the toddler higher on the hip.", "Knie gebeugt und Rumpf zurückgelehnt, während der nahe Arm das Kleinkind höher auf die Hüfte hebt.", P["hoisting"], STAND, bind(3.0, "walk", 1.6, "fuss")))
    return out


def clips():
    ph = lambda i, fr, t, en, de: (i, fr, t, (en, de), False)
    T = T_PER
    c = []
    keys = [(round(phi * T, 4), n, "smooth") for n, phi in phases.items()]
    contacts = [("heel-l", 0, round(0.36 * T, 4)), ("contact-l", round(0.06 * T, 4), round(0.62 * T, 4)), ("heel-r", round(0.5 * T, 4), round(0.86 * T, 4)), ("contact-r", round(0.56 * T, 4), T), ("contact-r", 0, round(0.12 * T, 4))]
    c.append(clip_record("walk-carrying", "Walking with the toddler", "Gehen mit dem Kleinkind",
        "Walking cycle of 1.0 s (about 0.95 m/s, step 0.47 m) with the toddler on the hip: the feet are planted while in contact, the far arm swings less than without a load and the toddler bobs and swings its legs.",
        "Gehzyklus von 1,0 s (etwa 0,95 m/s, Schritt 0,47 m) mit dem Kleinkind auf der Hüfte: Die Füße gleiten im Kontakt nicht, der ferne Arm schwingt weniger als ohne Last und das Kleinkind wippt und schwingt die Beine.",
        T, True, keys, contacts,
        [ph("stance-l", 0, round(0.5 * T, 4), "Left stance", "Linke Standphase"), ph("stance-r", round(0.5 * T, 4), T, "Right stance", "Rechte Standphase")],
        [(0, "contact-on", ("Left heel strike", "Linker Fersenaufsatz"), "heel-l"), (round(0.5 * T, 4), "contact-on", ("Right heel strike", "Rechter Fersenaufsatz"), "heel-r")],
        [("breathing", "step", [(0, 1), (T, 1)])], root_motion="plant", advance=0.9496))
    c.append(clip_record("shift-toddler", "Hoisting and settling the toddler", "Das Kleinkind hochnehmen und zurechtsetzen",
        "Non-looping 3 s: the parent bends the knees, leans back and hoists the toddler higher on the hip, then stands again; the toddler fusses briefly and settles.",
        "Nicht wiederholte Folge von 3 s: Die Eltern beugen die Knie, lehnen sich zurück und nehmen das Kleinkind höher auf die Hüfte, dann stehen sie wieder; das Kleinkind quengelt kurz und beruhigt sich.",
        3.0, False, [(0, "standing-carry", "smooth"), (0.9, "hoisting", "smooth"), (1.5, "hoisting", "smooth"), (2.6, "standing-carry", "smooth")],
        [(a, 0, 3.0) for a in STAND], [ph("bend", 0, 0.9, "Bending and leaning back", "Beugen und Zurücklehnen"), ph("hoist", 0.9, 1.5, "Hoisting", "Hochnehmen"), ph("settle", 1.5, 3.0, "Settling", "Zurechtsetzen")],
        [(1.2, "load-on", ("The toddler's seat is lifted", "Das Gesäß des Kleinkinds wird gehoben"), None)], [("breathing", "step", [(0, 0), (0.9, 1), (2.4, 0)])]))
    return c


def thermal():
    pats = [
        breath_pattern("rest", "Resting breathing", "Ruheatmung", 14, 0.5, 0.38, 0.04, 0.45, 0.13, "nose", "gupta-2010"),
        breath_pattern("walk", "Walking with a load", "Gehen mit Last", 20, 1.0, 0.42, 0.02, 0.50, 0.06, "nose-and-mouth", "ainsworth-2011"),
    ]
    tpats = [
        breath_pattern("rest", "Toddler at rest", "Kleinkind in Ruhe", 26, 0.09, 0.40, 0.05, 0.45, 0.10, "nose", "who-2006"),
        breath_pattern("fuss", "Toddler fussing", "Kleinkind quengelt", 38, 0.11, 0.38, 0.02, 0.58, 0.02, "mouth", "who-2006"),
    ]
    env = {"airTempC": 9, "meanRadiantTempC": 10, "skyTempC": 0, "groundTempC": 8, "relativeHumidity": 0.7, "windMS": 2.0, "windDirectionDeg": 180, "solarWM2": 200, "diffuseFraction": 0.6, "pressurePa": 101325}
    tb = {"species": "human", "massKg": TMASS, "skinAreaM2": round(T_SKIN, 3), "basalMetabolismW": 33.0, "metWM2": 58.15, "coreSetPointC": 37.0, "skinSetPointC": 34.0, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 70, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "schofield-1985"}
    toddler = {"id": "toddler", "title": tx("Toddler (24 months) carried on the hip", "Kleinkind (24 Monate) auf der Hüfte getragen"), "physiology": tb,
        "respiration": {"noseEmitter": "toddler-nose-breath", "mouthEmitter": "toddler-mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95, "patterns": tpats, "motion": [{"bone": "toddler-torso", "angleDeg": 2.5, "dy": 0.002}], "source": "authored-estimate", "reference": "who-2006"}}
    emit = nose_mouth_emitters(-150, -100) + [
        {"id": "toddler-nose-breath", "kind": "breath", "title": tx("Toddler nostril jet", "Nasenstrahl des Kleinkinds"), "anchor": "toddler-nostrils", "directionDeg": -150, "diameterM": 0.0055, "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "toddler-mouth-breath", "kind": "breath", "title": tx("Toddler mouth jet", "Mundstrahl des Kleinkinds"), "anchor": "toddler-mouth", "directionDeg": -100, "diameterM": 0.0075, "source": "authored-estimate", "reference": "gupta-2010"},
        plume_emitter("coat-torso", 0.35),
        {"id": "toddler-plume", "kind": "plume", "title": tx("Warm air above the toddler", "Warmluft über dem Kleinkind"), "anchor": "toddler-crown", "directionDeg": 0, "diameterM": 0.2, "region": "toddler-suit", "source": "authored-estimate", "reference": "incropera-2011"}]
    return {"environment": env, "baseMet": 1.4, "body": physiology(MASS, H, False, 34), "respiration": {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95, "patterns": pats, "motion": [{"bone": "torso", "angleDeg": 1.2, "dy": 0.004}, {"bone": "pelvis", "angleDeg": 0.5, "dy": 0.003}], "source": "authored-estimate", "reference": "hoeppe-1981"},
        "passengers": [toddler], "operating": [], "regions": REG, "emitters": emit,
        "assumptions": [
            tx("DuBois skin area 1.74 m2 for the parent (1.70 m, 66 kg) and 0.52 m2 for the toddler (0.865 m, 12.2 kg; DuBois is less accurate for small children). Parent: sweater under a wool coat (0.70 clo trunk, 0.55 clo sleeves), jeans, boots, scarf, beanie and gloves; toddler: padded overall (0.9 clo), knit hat, mittens and rubber boots. Garment values are authored estimates guided by ISO 9920 (1 clo = 0.155 m2K/W).", "DuBois-Hautfläche 1,74 m2 für das Elternteil (1,70 m, 66 kg) und 0,52 m2 für das Kleinkind (0,865 m, 12,2 kg; DuBois ist bei kleinen Kindern ungenauer). Elternteil: Pullover unter Wollmantel (0,70 clo Rumpf, 0,55 clo Ärmel), Jeans, Stiefel, Schal, Mütze und Handschuhe; Kleinkind: gefütterter Overall (0,9 clo), Strickmütze, Fäustlinge und Gummistiefel. Die Kleidungswerte sind Autorenschätzungen nach ISO 9920 (1 clo = 0,155 m2K/W)."),
            tx("Toddler: 24 months, median 12.2 kg and 0.865 m after the WHO growth standards 2006; basal metabolism 33 W after the Schofield equations (authored estimate); respiration 26 per minute at rest with 0.09 L tidal volume (about 7 mL/kg) and 38 per minute with 0.11 L when fussing. The toddler is a passenger with its own physiology, regions and breath emitters; its heat is not part of the parent's balance.", "Kleinkind: 24 Monate, Median 12,2 kg und 0,865 m nach den WHO-Wachstumsstandards 2006; Grundumsatz 33 W nach den Schofield-Gleichungen (Autorenschätzung); Atmung 26 pro Minute in Ruhe mit 0,09 L Atemzugvolumen (etwa 7 mL/kg) und 38 pro Minute mit 0,11 L beim Quengeln. Das Kleinkind ist ein Passagier mit eigener Physiologie, eigenen Regionen und Atem-Emittern; seine Wärme gehört nicht zur Bilanz des Elternteils."),
            tx("Parent: 1.4 met standing, 2.8 met walking with the load (about 4 km/h; Compendium of Physical Activities, walking with a child carried about 3 to 4 METs); first respiration pattern (rest) is the reference of the chest motion. Contact of 15 % of the coat and overall with the other body at 30 degrees Celsius.", "Elternteil: 1,4 met im Stehen, 2,8 met beim Gehen mit Last (etwa 4 km/h; Compendium of Physical Activities, Gehen mit getragenem Kind etwa 3 bis 4 MET); das erste Atemmuster (Ruhe) ist die Referenz der Brustbewegung. Kontakt von 15 % des Mantels und des Overalls mit dem anderen Körper bei 30 Grad Celsius.")]}


PENS = {100: dict(body=0.2, detail=0.13, texture=0.13, glazing=0.13, ground=0.3), 50: dict(body=0.25, detail=0.18, texture=0.13, glazing=0.13, ground=0.35)}
VR = {500: (350, 1000), 200: (150, 350), 100: (75, 150), 50: (20, 75)}


def build():
    import parent_decisions
    shapes = parent_art.all_shapes()
    d = parent_decisions.decisions()
    reps = {"500": parent_decisions.unavailable500(), "200": parent_decisions.unavailable200()}
    for sc in (100, 50):
        reps[str(sc)] = rep_authored(sc, VR[sc], PENS[sc], parent_decisions.HATCHES.get(sc, []), shapes[sc], d[sc])
    x0, y0, x1, y1 = bounds(P["standing-carry"], shapes[50])
    dims = {"heightM": round(y1 - y0, 3), "widthM": round(x1 - x0, 3), "depthM": 0.45, "massKg": round(MASS + TMASS, 2)}
    print("dims", dims)
    for sc in (100,):
        b = bounds(P["standing-carry"], shapes[sc])
        print(sc, "h", round(b[3] - b[1], 3), "w", round(b[2] - b[0], 3), "y0", round(b[1], 3))
    return {
        "id": "parent-carrying-toddler", "category": "humans", "title": tx("Parent carrying a toddler", "Elternteil trägt ein Kleinkind"),
        "summary": tx("A 1.70 m parent in wool coat, scarf, beanie and boots carrying a 24-month toddler (12.2 kg) in a padded overall on the hip: the toddler is a second living body with its own breathing; standing, a walking cycle with planted feet, hoisting and settling; two individually drawn scales (1:100 and 1:50); 1:500 and 1:200 are declared unavailable.", "Ein 1,70 m großes Elternteil in Wollmantel, Schal, Mütze und Stiefeln trägt ein 24 Monate altes Kleinkind (12,2 kg) im gefütterten Overall auf der Hüfte: Das Kleinkind ist ein zweiter lebender Körper mit eigener Atmung; Stehen, ein Gehzyklus mit festen Füßen, Hochnehmen und Zurechtsetzen; zwei einzeln gezeichnete Maßstäbe (1:100 und 1:50); 1:500 und 1:200 sind als nicht verfügbar erklärt."),
        "tags": ["human", "parent", "child", "toddler", "carrying", "passenger", "walking"],
        "provenance": {"author": "Semio staffage content worker NW-H2", "method": "handcrafted", "created": "2026-10-29",
            "references": refs("de-leva-1996", "dubois-1916", "iso-9920-2007", "ainsworth-2011", "ashrae-2021", "gagge-1971", "gupta-2010", "hoeppe-1981", "mifflin-1990", "incropera-2011", "winter-2009", extra=[
                {"id": "who-2006", "citation": "WHO Multicentre Growth Reference Study Group (2006). WHO Child Growth Standards: length/height-for-age, weight-for-age (24 months: median about 87 cm length and 12 kg). Breathing rates and tidal volumes of the toddler are authored estimates."},
                {"id": "schofield-1985", "citation": "Schofield, W. N. (1985). Predicting basal metabolic rate, new standards and review of previous work. Human Nutrition: Clinical Nutrition 39C (Suppl. 1), 5-41 (basal metabolism of children by weight and height; the 33 W of the toddler is an authored estimate guided by it)."}]),
            "notes": tx("Newly authored for semio.staffage/v2: the toddler is a passenger (second body, own regions and respiration) carried on the near hip; the walking gait is generated by inverse kinematics for the left foot with the right foot half a cycle later and planted by the engine.", "Neu erstellt für semio.staffage/v2: Das Kleinkind ist ein Passagier (zweiter Körper, eigene Regionen und Atmung) auf der nahen Hüfte; der Gang ist per inverser Kinematik für den linken Fuß erzeugt, der rechte folgt einen halben Zyklus später und wird von der Engine geplant.")},
        "dimensions": dims, "materials": MAT, "rig": {"kind": "anatomical", "root": "pelvis", "bones": bones}, "anchors": anchors,
        "representations": reps, "symbols": [], "poses": pose_list(), "clips": clips(), "thermal": thermal(),
        "structure": structure([("Inertial model: segment masses as point masses at de Leva centres, the toddler's bones carry 12.2 kg on the parent's pelvis; heel and ball contacts are unilateral supports.", "Trägheitsmodell: Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva, die Knochen des Kleinkinds tragen 12,2 kg am Becken des Elternteils; Fersen- und Ballenkontakte sind einseitige Auflager.")]),
        "presentation": {"defaultPose": "standing-carry", "defaultClip": "walk-carrying", "defaultScale": 100, "viewpoints": ["elevation"]},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}


if __name__ == "__main__":
    dump(asset_doc(build()), out_path("parent-carrying-toddler"), depth=5)
    print("written")
