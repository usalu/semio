import json
from scooter_phys import *
from docbuild import *
import scooter_draw as SD
from scooter_texts import DECS

REFS = [ref(*r) for r in (DELEVA_REF, DRILLIS_REF, DUBOIS_REF, ISO9920, ISO8996, ASHRAE, ("gagge-1971", GAGGE[1]), ("gupta-2010", GUPTA[1]), WINTER)] + [
    ref("fryar-2021", "Fryar, C. D., Carroll, M. D., Gu, Q., Afful, J. and Ogden, C. L. (2021). Anthropometric Reference Data for Children and Adults: United States, 2015-2018. Vital and Health Statistics Series 3, No. 46, NCHS (stature and mass of 8-year-old girls)."),
    ref("schofield-1985", "Schofield, W. N. (1985). Predicting basal metabolic rate, new standards and review of previous work. Human Nutrition: Clinical Nutrition 39C (Suppl 1), 5-41 (girls 3-10 years)."),
    ref("scooter-spec", "Authored kick scooter specification for children: 145 mm wheels, 0.78 m wheelbase, 0.84 m handlebar height, 0.52 m deck, 3.8 kg; typical of commercial children's kick scooters, no single product.")]
A_ = dubois(26.0, 1.28)
MATS = [
    mat("skin-child", "Child skin", "Kinderhaut", 0.98, 0.6, ref="ashrae-2021"), mat("hair-blond", "Blond hair", "Blondes Haar", 0.95, 0.6),
    mat("helmet-eps", "Polycarbonate shell over EPS foam", "Polycarbonatschale über EPS-Schaum", 0.90, 0.55),
    mat("zip-jacket", "Cotton-polyester zip jacket", "Reißverschlussjacke aus Baumwolle-Polyester", 0.90, 0.6, ref="ashrae-2021"),
    mat("tshirt-cotton", "Cotton T-shirt", "Baumwoll-T-Shirt", 0.90, 0.4, ref="ashrae-2021"), mat("leggings-knit", "Cotton-elastane leggings", "Leggings aus Baumwolle-Elastan", 0.90, 0.8),
    mat("sneaker-textile", "Textile and rubber sneaker", "Sneaker aus Textil und Gummi", 0.90, 0.5), mat("cotton-sock", "Cotton sock", "Baumwollsocke", 0.90, 0.5),
    mat("still-air", "Still air", "Ruhende Luft", 0.90), mat("nylon-backpack", "Nylon backpack", "Nylon-Rucksack", 0.90, 0.6),
    mat("alu-deck", "Aluminium deck with grip tape", "Aluminium-Trittbrett mit Griffband", 0.85, 0.7, densityKgM3=2700, conductivityWmK=160, specificHeatJKgK=900),
    mat("alu-stem", "Anodised aluminium stem and fork", "Eloxierte Aluminiumlenkstange und -gabel", 0.30, 0.5, densityKgM3=2700, conductivityWmK=160, specificHeatJKgK=900),
    mat("rubber-grip", "Rubber grips", "Gummigriffe", 0.93, 0.9, ref="ashrae-2021"), mat("pu-wheel", "Polyurethane wheels", "Polyurethanräder", 0.93, 0.8),
]
REGIONS = [
    dict(id="face-skin", en="Face and neck skin", de="Haut von Gesicht und Hals", kind="skin", model="body", material="skin-child", frac=0.06, L=0.13, geom="sphere", svf=0.6, seg="head", ms=0.12, sf=0.06, layers=[], ex=1.0, su=0.6),
    dict(id="hair", en="Ponytail and hair", de="Pferdeschwanz und Haar", kind="hair", model="body", material="hair-blond", frac=0.02, L=0.13, geom="cylinder", svf=0.7, seg="head", ms=0.02, sf=0.02,
         layers=[layer("hair-pelt", "hair", "hair-blond", 0.04, 250, "Hair and trapped air", "Haar und eingeschlossene Luft", 0.01)], ex=1.0, su=0.5),
    dict(id="helmet", en="Bicycle helmet", de="Fahrradhelm", kind="clothing", model="body", material="helmet-eps", frac=0.05, L=0.18, geom="sphere", svf=0.85, seg="head", ms=0.03, sf=0.05,
         layers=[layer("helmet-air", "air-gap", "still-air", 0.03, 0, "Air gap between head and helmet", "Luftspalt zwischen Kopf und Helm", 0.02, "authored-estimate", None),
                 layer("helmet-eps", "insulation", "helmet-eps", 0.45, 1500, "EPS foam liner (25 mm, about 0.04 W/mK)", "EPS-Schaumfutter (25 mm, etwa 0,04 W/mK)", 0.025, "authored-estimate", None)], ex=1.0, su=0.75),
    dict(id="hands", en="Hands", de="Hände", kind="skin", model="body", material="skin-child", frac=0.05, L=0.03, geom="cylinder", svf=0.5, seg="hand", ms=0.02, sf=0.05, layers=[], ex=1.0, su=0.5),
    dict(id="jacket", en="Zip jacket over T-shirt", de="Reißverschlussjacke über T-Shirt", kind="clothing", model="body", material="zip-jacket", frac=0.43, fcl=1.15, L=0.22, geom="cylinder", svf=0.5, seg="trunk", ms=0.56, sf=0.43,
         garments=[("tshirt", "clothing", "tshirt-cotton", 0.08, 300, "Cotton T-shirt", "Baumwoll-T-Shirt", 0.001), ("jacket", "clothing", "zip-jacket", 0.25, 800, "Zip jacket", "Reißverschlussjacke", 0.004)], ex=0.95, su=0.5),
    dict(id="leggings", en="Leggings", de="Leggings", kind="clothing", model="body", material="leggings-knit", frac=0.32, fcl=1.05, L=0.09, geom="cylinder", svf=0.35, seg="leg", ms=0.20, sf=0.32,
         garments=[("knit", "clothing", "leggings-knit", 0.12, 400, "Leggings", "Leggings", 0.002)], ex=0.92, su=0.4),
    dict(id="sneakers", en="Sneakers with socks", de="Sneaker mit Socken", kind="clothing", model="body", material="sneaker-textile", frac=0.07, L=0.05, geom="cylinder", svf=0.25, seg="foot", ms=0.05, sf=0.07,
         garments=[("sock", "clothing", "cotton-sock", 0.04, 250, "Cotton socks", "Baumwollsocken", 0.002), ("upper", "clothing", "sneaker-textile", 0.03, 800, "Textile upper", "Textilschaft", 0.002)], ex=1.0, su=0.2),
    dict(id="backpack", en="Backpack", de="Rucksack", kind="surface", model="passive", material="nylon-backpack", area=0.18, L=0.25, geom="vertical-plate", svf=0.3, cap=800, ex=0.7, su=0.6),
    dict(id="deck", en="Deck with grip tape and rear fender", de="Trittbrett mit Griffband und Hinterradschutzblech", kind="surface", model="passive", material="alu-deck", area=0.22, L=0.5, geom="horizontal-plate-up", svf=0.9, cap=4000, ex=1.0, su=0.7),
    dict(id="stem", en="Stem, fork and head tube", de="Lenkstange, Gabel und Steuerrohr", kind="surface", model="passive", material="alu-stem", area=0.12, L=0.027, geom="cylinder", svf=0.6, cap=3500, ex=1.0, su=0.6),
    dict(id="grips", en="Rubber handlebar grips", de="Gummi-Lenkergriffe", kind="surface", model="passive", material="rubber-grip", area=0.016, L=0.034, geom="cylinder", svf=0.5, cap=10000, ex=0.8, su=0.5),
    dict(id="wheels", en="Polyurethane wheels", de="Polyurethanräder", kind="tyre", model="passive", material="pu-wheel", area=0.07, L=0.145, geom="cylinder", svf=0.4, cap=9000, ex=1.0, su=0.3),
]
PATTERNS = [pattern("quiet", "Standing still", "Ruhiges Stehen", 20, 0.20, (0.38, 0.04, 0.45, 0.13), "nose"),
            pattern("gliding", "Gliding on the scooter", "Gleiten auf dem Roller", 28, 0.30, (0.40, 0.03, 0.47, 0.10), "nose-and-mouth"),
            pattern("kicking", "Kicking and pushing", "Anschieben", 38, 0.40, (0.42, 0.02, 0.50, 0.06), "nose-and-mouth")]
PIDX = {p["id"]: i for i, p in enumerate(PATTERNS)}
RESP = {"noseEmitter": "breath-nose", "mouthEmitter": "breath-mouth", "exhaledTempC": 34.0, "exhaledRelativeHumidity": 0.95, "patterns": PATTERNS, "motion": [{"bone": "torso", "angleDeg": 1.4, "dy": 0.003}], "source": "authored-estimate", "reference": "gupta-2010"}
BODY = {"species": "human", "massKg": 26.0, "skinAreaM2": round(A_, 3), "basalMetabolismW": 52.5, "metWM2": 58.15, "coreSetPointC": 36.9, "skinSetPointC": 34.2, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "gagge-1971"}
ENV = {"airTempC": 20, "meanRadiantTempC": 28, "skyTempC": 6, "groundTempC": 26, "relativeHumidity": 0.5, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 700, "diffuseFraction": 0.2, "pressurePa": 101325}


def rule(met, pat, plume, kick=False):
    r = dict(met=met, pat=pat, em={"plume": plume}, contact={"hands": {"fraction": 0.3, "tempC": 26}, "grips": {"fraction": 0.5, "tempC": 28}, "sneakers": {"fraction": 0.35, "tempC": 28}, "wheels": {"fraction": 0.01, "tempC": 26}, "deck": {"fraction": 0.12, "tempC": 28}}, su={})
    return r


def pe(id_, en, de, den, dede, st, pel, r, key=False):
    offs, wa, bad = st
    assert not bad, (id_, bad)
    lim_check(rig, offs, id_)
    return dict(id=id_, en=en, de=de, den=den, dede=dede, offs=offs, root=(0.0, pel[1] - h.hh), contacts=[], rule=r, loads=[], key=key, ground="authored")


def build():
    g, gp = glide_state()
    p_, pp = push_pose()
    b_, bp = brake_pose()
    poses = [
        pe("gliding", "Gliding", "Gleiten", "Both feet on the deck one behind the other, knees bent, hands on the handlebar, gliding on two wheels.", "Beide Füße hintereinander auf dem Trittbrett, Knie gebeugt, Hände am Lenker, Gleiten auf zwei Rädern.", g, gp, rule(3.0, "gliding", 0.55)),
        pe("pushing-off", "Pushing off", "Anschieben", "Standing on the front foot with the knee deeply bent, the right foot on the ground pushing the scooter forward.", "Auf dem vorderen Fuß mit tief gebeugtem Knie, der rechte Fuß am Boden schiebt den Roller vorwärts.", p_, pp, rule(5.0, "kicking", 0.8)),
        pe("braking", "Braking on the fender", "Bremsen am Schutzblech", "The weight on the front foot, the right foot pressed on the rear fender against the rear wheel, the torso upright.", "Das Gewicht auf dem vorderen Fuß, der rechte Fuß drückt das Hinterradschutzblech gegen das Hinterrad, der Oberkörper aufrecht.", b_, bp, rule(2.2, "gliding", 0.45)),
    ]
    N = 44
    dt = TC / N
    keyposes = []
    clipkeys = []
    for i in range(N + 1):
        t = i * dt
        (o, wa, bad), pel = cycle(t if i < N else TC)
        assert not bad, (i, bad)
        lim_check(rig, o, "k%d" % i)
        kid = "kick-%02d" % (i if i < N else 0)
        if i < N:
            kick = t < 0.80
            kp = pe(kid, "Kick cycle key %d" % i, "Schwungzyklus Schlüssel %d" % i, "Key pose of the kick-and-glide cycle.", "Schlüsselpose des Anschieb-und-Gleit-Zyklus.", (o, wa, bad), pel, rule(4.4 if kick else 3.0, "kicking" if kick else "gliding", 0.8 if kick else 0.55), key=True)
            keyposes.append(kp)
        clipkeys.append({"t": r4(t), "pose": kid, "ease": "smooth", "bones": {k: {kk: r4(vv) for kk, vv in v.items()} for k, v in o.items()}, "_root": (0.0, pel[1] - h.hh)})
    for k in clipkeys: k.pop("_root")
    cont = [{"anchor": "heel-r", "fromS": 0, "toS": 0.10}, {"anchor": "ball-r", "fromS": 0, "toS": 0.34}]
    phases = [{"id": "push", "fromS": 0, "toS": 0.34, "title": tx("Right foot pushes against the ground", "Rechter Fuß stößt sich vom Boden ab")},
              {"id": "return-swing", "fromS": 0.34, "toS": 0.80, "title": tx("The pushing leg swings back to the deck", "Das Schubbein schwingt zurück auf das Trittbrett")},
              {"id": "glide", "fromS": 0.80, "toS": 1.85, "title": tx("Gliding on both feet", "Gleiten auf beiden Füßen")},
              {"id": "step-down", "fromS": 1.85, "toS": TC, "title": tx("The right foot reaches down to the ground", "Der rechte Fuß greift zum Boden")}]
    events = [ev(0, "contact-on", "Right foot lands on the ground", "Rechter Fuß setzt am Boden auf", "heel-r"), ev(0.34, "contact-off", "Right toe-off, end of the push", "Rechter Abdruck, Ende des Schubs", "ball-r")]
    tracks = [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": PIDX["kicking"]}, {"t": 0.8, "v": PIDX["gliding"]}]},
              {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 5.0}, {"t": 0.8, "v": 3.0}, {"t": 1.85, "v": 3.0}, {"t": TC, "v": 5.0}]},
              {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.8}, {"t": 0.8, "v": 0.55}, {"t": 1.85, "v": 0.55}, {"t": TC, "v": 0.8}]}]
    kick = make_clip("kick-and-glide", "Kick and glide", "Anschieben und Gleiten",
                     "One cycle of 2.2 s: a short push with the right foot (0.34 s), the leg swings back onto the deck, the child glides on both feet and reaches down for the next push; the wheels turn exactly five times per cycle (2.28 m, 1.04 m/s).",
                     "Ein Zyklus von 2,2 s: ein kurzer Schub mit dem rechten Fuß (0,34 s), das Bein schwingt zurück aufs Trittbrett, das Kind gleitet auf beiden Füßen und greift zum nächsten Schub nach unten; die Räder drehen sich pro Zyklus genau fünfmal (2,28 m, 1,04 m/s).",
                     TC, 30, True, "plant", clipkeys, cont, phases, events, tracks, advance=S_ADV)
    bkeys = [{"t": 0.0, "pose": "gliding", "ease": "smooth"}, {"t": 1.0, "pose": "gliding", "ease": "smooth"}, {"t": 2.2, "pose": "braking", "ease": "smooth"}, {"t": 4.0, "pose": "braking", "ease": "smooth"}]
    brake = make_clip("glide-and-brake", "Gliding to a stop", "Gleiten bis zum Stillstand",
                      "The child glides, shifts the weight back and presses the right foot on the rear fender to brake; the treadmill frame keeps the scooter in place while the wheels keep turning.",
                      "Das Kind gleitet, verlagert das Gewicht nach hinten und drückt zum Bremsen den rechten Fuß auf das Schutzblech; der Laufbandrahmen hält den Roller an Ort und Stelle, während die Räder weiterlaufen.",
                      4.0, 30, False, "none", bkeys, [], [{"id": "glide", "fromS": 0, "toS": 1.0, "title": tx("Gliding", "Gleiten")}, {"id": "braking", "fromS": 1.0, "toS": 4.0, "title": tx("Braking", "Bremsen")}],
                      [ev(1.0, "motion", "Foot moves to the fender", "Fuß wandert zum Schutzblech"), ev(2.2, "motion", "Brake applied", "Bremse angezogen")],
                      [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": PIDX["gliding"]}, {"t": 2.2, "v": PIDX["quiet"]}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3.0}, {"t": 2.2, "v": 2.2}, {"t": 4.0, "v": 1.8}]}])
    structure = {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": ["wheel-rear-contact", "wheel-front-contact"],
                 "assumptions": [tx("The scooter (3.8 kg) rolls on two permanent rolling supports; only the pushing foot is a ground contact, planted while it pushes. The share of the weight on the wheels follows the elastic-support model of the solver.", "Der Roller (3,8 kg) rollt auf zwei dauerhaften Rollenlagern; nur der Schubfuß ist ein Bodenkontakt und steht während des Schubs fest. Der Gewichtsanteil auf den Rädern folgt dem Elastizitätsmodell des Lösers."),
                                 tx("Because the rider's feet on the deck are no ground contacts, the pose heights are authored (ground 'authored').", "Weil die Füße des Kindes auf dem Trittbrett keine Bodenkontakte sind, sind die Posenhöhen vorgegeben (Boden 'authored')."),
                                 tx("Not a clinical or dynamics model of scooting.", "Kein klinisches oder dynamisches Modell des Rollerfahrens.")], "clinical": False}
    mass = mass_total(rig)
    reps = SD.reps(DECS)
    spec = dict(
        id=ID, human=h, rig=rig, anchors=ANCH, title=tx("Child on Kick Scooter", "Kind auf Tretroller"),
        summary=tx("An 8-year-old girl, 1.28 m, in a helmet, zip jacket and backpack rides a two-wheel kick scooter: she pushes with the right foot, swings it back onto the deck and glides crouched with both hands on the T-bar; the right foot also brakes on the rear fender.",
                   "Ein 8-jähriges Mädchen, 1,28 m, mit Helm, Reißverschlussjacke und Rucksack fährt einen zweirädrigen Tretroller: Sie schiebt mit dem rechten Fuß an, schwingt ihn zurück aufs Trittbrett und gleitet geduckt mit beiden Händen am T-Lenker; mit dem rechten Fuß bremst sie am Hinterradschutzblech."),
        tags=["human", "child", "kick-scooter", "helmet", "gliding", "wheels"], refs=REFS,
        notes=tx("Stature and mass after NCHS 2015-2018 (8-year-old girl). Segment mass fractions are an authored child variant of de Leva with a larger head; proportions after Drillis and Contini, surface area after DuBois, basal metabolism after Schofield. The scooter is an authored typical children's specification (145 mm wheels, 0.78 m wheelbase, 0.84 m handlebar).", "Körpergröße und Masse nach NCHS 2015-2018 (8-jähriges Mädchen). Segmentmassenanteile sind eine eigene Kindervariante nach de Leva mit größerem Kopf; Proportionen nach Drillis und Contini, Oberfläche nach DuBois, Grundumsatz nach Schofield. Der Roller ist eine eigene typische Kinderspezifikation (145-mm-Räder, 0,78 m Radstand, 0,84 m Lenkerhöhe)."),
        dims=dict(heightM=1.35, widthM=1.1, depthM=0.45, massKg=round(mass, 2)), materials=MATS, regions=REGIONS, env=ENV, body=BODY, respiration=RESP,
        emitters=std_emitters("jacket", 0.18, child=True), emit_default={"plume": 0.55}, poses=poses + keyposes, clips=[kick, brake], reps=reps,
        assumptions=[tx("Reference case: a sunny day, 20 degrees C air, 28 degrees C radiant temperature, light breeze.", "Referenzfall: sonniger Tag, 20 Grad C Luft, 28 Grad C Strahlungstemperatur, leichte Brise."),
                     tx("The helmet is a body region with an air gap and EPS foam; the scooter and the backpack are passive regions.", "Der Helm ist eine Körperregion mit Luftspalt und EPS-Schaum; Roller und Rucksack sind passive Regionen."),
                     tx("Child respiration (higher rate, lower tidal volume) and all angles are authored estimates guided by Gupta et al. 2010.", "Kinderatmung (höhere Rate, kleineres Atemzugvolumen) und alle Winkel sind Schätzungen, angelehnt an Gupta et al. 2010.")],
        structure=structure, presentation={"defaultPose": "gliding", "defaultClip": "kick-and-glide", "defaultScale": 100, "viewpoints": ["elevation"]})
    return assemble(spec)


if __name__ == "__main__":
    print(write_doc(build(), ID))
