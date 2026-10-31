"""NW-H1: assemble and write person-on-crutches."""
import json
from crutches_phys import *
from docbuild import *
import crutches_draw as CD

REFS = [ref(*r) for r in (DELEVA_REF, DRILLIS_REF, DUBOIS_REF, ISO9920, ISO8996, ASHRAE, ("gagge-1971", GAGGE[1]), ("gupta-2010", GUPTA[1]), WINTER, PERRY)] + [
    ref("iso-11334-1-2007", "ISO 11334-1:2007 Assistive products for walking manipulated by one arm: requirements and test methods, Part 1: Elbow crutches (forearm cuff, hand grip, adjustable length, rubber ferrule). Used as guidance for the construction; dimensions are authored."),
    ref("mifflin-1990", "Mifflin, M. D., St Jeor, S. T. et al. (1990). A new predictive equation for resting energy expenditure in healthy individuals. American Journal of Clinical Nutrition 51(2), 241-247."),
    ref("waters-mulroy-1999", "Waters, R. L. and Mulroy, S. (1999). The energy expenditure of normal and pathologic gait. Gait and Posture 9(3), 207-231 (non-weight-bearing crutch walking costs clearly more energy than normal walking). Used qualitatively; the met value is an authored estimate.")]

A_ = dubois(78.0, 1.82)
MATS = [
    mat("skin-light", "Light skin", "Helle Haut", 0.98, 0.6, ref="ashrae-2021"),
    mat("hair-dark", "Dark short hair", "Dunkles Kurzhaar", 0.95, 0.9),
    mat("fleece-hoodie", "Cotton-polyester hooded sweatshirt", "Kapuzenpullover aus Baumwolle-Polyester", 0.90, 0.5, ref="ashrae-2021"),
    mat("tshirt-cotton", "Cotton T-shirt", "Baumwoll-T-Shirt", 0.90, 0.4, ref="ashrae-2021"),
    mat("polyester-short", "Polyester sports shorts", "Sportshose aus Polyester", 0.90, 0.5),
    mat("sneaker-mesh", "Mesh and rubber sneaker", "Sneaker aus Mesh und Gummi", 0.90, 0.6),
    mat("boot-shell", "Black polymer boot shell with foam liner", "Schwarze Polymerschale mit Schaumfutter", 0.92, 0.9),
    mat("cotton-sock", "Cotton sock", "Baumwollsocke", 0.90, 0.5),
    mat("still-air", "Still air", "Ruhende Luft", 0.90),
    mat("alu-anodised", "Anodised aluminium", "Eloxiertes Aluminium", 0.30, 0.45, densityKgM3=2700, conductivityWmK=160, specificHeatJKgK=900),
    mat("rubber-black", "Black rubber", "Schwarzer Gummi", 0.93, 0.9, ref="ashrae-2021"),
    mat("cuff-polymer", "Black polymer cuff", "Schwarze Polymer-Manschette", 0.92, 0.9),
]
REGIONS = [
    dict(id="face-skin", en="Face and neck skin", de="Haut von Gesicht und Hals", kind="skin", model="body", material="skin-light", frac=0.05, L=0.16, geom="sphere", svf=0.6, seg="head", ms=0.10, sf=0.05, layers=[], ex=1.0, su=0.55),
    dict(id="hair", en="Short dark hair", de="Dunkles Kurzhaar", kind="hair", model="body", material="hair-dark", frac=0.02, L=0.16, geom="sphere", svf=0.8, seg="head", ms=0.02, sf=0.02,
         layers=[layer("hair-pelt", "hair", "hair-dark", 0.03, 200, "Short hair and trapped air", "Kurzhaar und eingeschlossene Luft", 0.008)], ex=1.0, su=0.7),
    dict(id="hands", en="Hands", de="Hände", kind="skin", model="body", material="skin-light", frac=0.05, L=0.04, geom="cylinder", svf=0.5, seg="hand", ms=0.02, sf=0.05, layers=[], ex=1.0, su=0.5),
    dict(id="hoodie", en="Hooded sweatshirt over T-shirt", de="Kapuzenpullover über T-Shirt", kind="clothing", model="body", material="fleece-hoodie", frac=0.49, fcl=1.2, L=0.30, geom="cylinder", svf=0.5, seg="trunk", ms=0.60, sf=0.49,
         garments=[("tshirt", "clothing", "tshirt-cotton", 0.08, 350, "Cotton T-shirt", "Baumwoll-T-Shirt", 0.001), ("sweat", "clothing", "fleece-hoodie", 0.34, 1100, "Hooded sweatshirt", "Kapuzenpullover", 0.005)], ex=0.95, su=0.5),
    dict(id="shorts", en="Sports shorts", de="Sportshose", kind="clothing", model="body", material="polyester-short", frac=0.09, fcl=1.1, L=0.15, geom="cylinder", svf=0.4, seg="leg", ms=0.06, sf=0.09,
         garments=[("poly", "clothing", "polyester-short", 0.06, 300, "Polyester shorts with brief", "Polyestershorts mit Slip", 0.001)], ex=0.9, su=0.45),
    dict(id="legs-skin", en="Bare legs", de="Nackte Beine", kind="skin", model="body", material="skin-light", frac=0.19, L=0.10, geom="cylinder", svf=0.35, seg="leg", ms=0.12, sf=0.19, layers=[], ex=1.0, su=0.35),
    dict(id="boot", en="Orthopaedic walking boot on the left foot", de="Orthopädischer Walking-Stiefel am linken Fuß", kind="clothing", model="body", material="boot-shell", frac=0.075, fcl=1.5, L=0.10, geom="cylinder", svf=0.2, seg="foot", ms=0.05, sf=0.075,
         garments=[("sock", "clothing", "cotton-sock", 0.04, 300, "Cotton sock", "Baumwollsocke", 0.002), ("shell", "clothing", "boot-shell", 0.45, 2500, "Boot shell with foam liner", "Stiefelschale mit Schaumfutter", 0.02)], ex=1.0, su=0.3),
    dict(id="sneaker", en="Sneaker on the right foot", de="Sneaker am rechten Fuß", kind="clothing", model="body", material="sneaker-mesh", frac=0.035, L=0.06, geom="cylinder", svf=0.25, seg="foot", ms=0.03, sf=0.035,
         garments=[("sock", "clothing", "cotton-sock", 0.04, 300, "Cotton sock", "Baumwollsocke", 0.002), ("upper", "clothing", "sneaker-mesh", 0.03, 900, "Mesh upper", "Meshschaft", 0.002)], ex=1.0, su=0.2),
    dict(id="crutch-frame", en="Anodised aluminium tubes and cuff brackets", de="Eloxierte Aluminiumrohre und Manschettenbügel", kind="surface", model="passive", material="alu-anodised", area=0.16, L=0.022, geom="cylinder", svf=0.6, cap=3600, ex=1.0, su=0.5),
    dict(id="crutch-grips", en="Rubber hand grips", de="Gummi-Handgriffe", kind="surface", model="passive", material="rubber-black", area=0.025, L=0.03, geom="cylinder", svf=0.5, cap=10000, ex=0.8, su=0.5),
    dict(id="crutch-tips", en="Rubber crutch tips", de="Gummi-Krückenspitzen", kind="surface", model="passive", material="rubber-black", area=0.006, L=0.03, geom="cylinder", svf=0.3, cap=10000, ex=1.0, su=0.1),
    dict(id="crutch-cuffs", en="Polymer forearm cuffs", de="Polymer-Unterarmmanschetten", kind="surface", model="passive", material="cuff-polymer", area=0.04, L=0.06, geom="cylinder", svf=0.5, cap=3000, ex=0.9, su=0.5),
]
PATTERNS = [
    pattern("quiet", "Quiet standing on the crutches", "Ruhiges Stehen auf den Krücken", 14, 0.55, (0.38, 0.04, 0.45, 0.13), "nose"),
    pattern("crutch-walking", "Crutch walking, non-weight-bearing", "Gehen an Krücken ohne Belastung", 26, 1.05, (0.42, 0.02, 0.50, 0.06), "nose-and-mouth"),
    pattern("recovering", "Recovering after a stretch", "Erholen nach einer Strecke", 20, 0.80, (0.40, 0.03, 0.47, 0.10), "nose-and-mouth"),
]
PIDX = {p["id"]: i for i, p in enumerate(PATTERNS)}
RESP = {"noseEmitter": "breath-nose", "mouthEmitter": "breath-mouth", "exhaledTempC": 34.0, "exhaledRelativeHumidity": 0.95, "patterns": PATTERNS,
        "motion": [{"bone": "torso", "angleDeg": 1.2, "dy": 0.006}], "source": "authored-estimate", "reference": "gupta-2010"}
BODY = {"species": "human", "massKg": 78.0, "skinAreaM2": round(A_, 3), "basalMetabolismW": 87.3, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False,
        "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "gagge-1971"}
ENV = {"airTempC": 18, "meanRadiantTempC": 24, "skyTempC": 5, "groundTempC": 22, "relativeHumidity": 0.55, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 650, "diffuseFraction": 0.2, "pressurePa": 101325}
GC = {"fraction": 0.4, "tempC": 24}
CG = {"fraction": 0.5, "tempC": 28}


def rule(met, pat, plume, extra=None):
    r = dict(met=met, pat=pat, em={"plume": plume}, contact={"hands": {"fraction": 0.3, "tempC": 26}, "crutch-grips": CG, "crutch-tips": {"fraction": 0.6, "tempC": 22}, "sneaker": {"fraction": 0.3, "tempC": 22}}, su={})
    if extra: r.update(extra)
    return r


def pe(id_, en, de, den, dede, st, pel, contacts, r):
    offs, wa, bad = st
    assert not bad, (id_, bad)
    lim_check(rig, offs, id_)
    return dict(id=id_, en=en, de=de, den=den, dede=dede, offs=offs, root=(0.0, pel[1] - h.hh), contacts=contacts, rule=r, loads=[])


def build():
    tri = stand_tripod()
    cf = swing_crutch_pose()
    bs = swing_body_pose()
    poses = [
        pe("tripod-stand", "Tripod stance", "Dreipunktstand", "Standing on the right foot with both crutches planted a hand ahead; the left foot in its boot hangs clear of the ground.", "Stand auf dem rechten Fuß, beide Krücken eine Handbreit voraus aufgesetzt; der linke Fuß im Stiefel hängt frei über dem Boden.", tri, (0, h.hh - 0.02), ["heel-r", "ball-r", "tip-l", "tip-r"], rule(1.5, "quiet", 0.45)),
        pe("crutches-forward", "Crutches swinging forward", "Krücken schwingen vor", "Weight on the right leg alone while both crutches lift and reach forward with nearly straight arms.", "Gewicht allein auf dem rechten Bein, während beide Krücken abheben und mit fast gestreckten Armen nach vorn greifen.", cf, (0, h.hh - 0.02), ["heel-r", "ball-r"], rule(2.6, "crutch-walking", 0.6)),
        pe("body-swing", "Swinging through the crutches", "Durchschwingen zwischen den Krücken", "Weight on the straight arms and both crutch tips, the body swings forward with the right leg reaching for the next landing.", "Gewicht auf den gestreckten Armen und beiden Krückenspitzen, der Körper schwingt nach vorn, das rechte Bein greift zur nächsten Landung.", bs, (0.2, h.hh + 0.045), ["tip-l", "tip-r"], rule(3.4, "crutch-walking", 0.75)),
    ]
    N = 36
    dt = T_GAIT / N
    states = []
    for i in range(N + 1):
        (o, wa, bad), pel = gait_state(i * dt)
        assert not bad, (i, bad)
        lim_check(rig, o, "g%d" % i)
        states.append(o)
    gkeys = keys_from_states(states, dt, "crutches-forward")
    T = T_GAIT
    cont = [{"anchor": "heel-r", "fromS": 0, "toS": 0.7}, {"anchor": "ball-r", "fromS": 0, "toS": 0.83},
            {"anchor": "tip-r", "fromS": 0, "toS": 0.02}, {"anchor": "tip-r", "fromS": 0.68, "toS": T}, {"anchor": "tip-l", "fromS": 0, "toS": 0.02}, {"anchor": "tip-l", "fromS": 0.68, "toS": T}]
    phases = [{"id": "crutches-advance", "fromS": 0, "toS": 0.7, "title": tx("Crutches advance on the right leg", "Krücken rücken auf dem rechten Bein vor")},
              {"id": "body-swing-through", "fromS": 0.7, "toS": 1.35, "title": tx("Body swings forward on the crutches", "Körper schwingt an den Krücken nach vorn")},
              {"id": "landing-and-loading", "fromS": 1.35, "toS": T, "title": tx("Right foot lands and takes the weight", "Rechter Fuß landet und übernimmt das Gewicht")}]
    events = [ev(0.7, "contact-on", "Both crutch tips land", "Beide Krückenspitzen landen", "tip-r"), ev(0.7, "contact-off", "Right heel lifts", "Rechte Ferse hebt ab", "heel-r"), ev(0.83, "contact-off", "Right toe-off", "Rechter Abdruck", "ball-r"),
              ev(1.35, "contact-on", "Right heel strike", "Rechter Fersenkontakt", "heel-r"), ev(1.8, "contact-off", "Crutch tips lift", "Krückenspitzen heben ab", "tip-r")]
    ev_ = [e for e in events if e["t"] < T]
    tracks = [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": PIDX["crutch-walking"]}]},
              {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.4}, {"t": 1.0, "v": 3.4}, {"t": T, "v": 2.4}]},
              {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 1.0, "v": 0.8}, {"t": T, "v": 0.6}]}]
    gait = make_clip("three-point-gait", "Three-point crutch gait", "Dreipunktgang an Krücken",
                     "Swing-to gait with the left foot off-loaded: crutches reach forward, the body swings to the crutch line on the right leg and the right foot lands beside the tips; 0.4 m per 1.8 s.",
                     "Durchschwinggang bei entlastetem linkem Fuß: Die Krücken greifen vor, der Körper schwingt auf dem rechten Bein zur Krückenlinie und der rechte Fuß landet neben den Spitzen; 0,4 m pro 1,8 s.",
                     T, 30, True, "plant", gkeys, cont, phases, ev_, tracks, advance=D_STEP)
    kt = [0.0, 1.5, 3.0, 4.5, 6.0]
    vars_ = [dict(), dict(torso=80.0, head=80.0), dict(torso=86.0, head=92.0), dict(torso=83.0, head=84.0), dict()]
    rstates = []
    for v in vars_:
        py = h.hh - 0.02
        st = state(0.0, py, v.get("torso", 84.0), 78.0, v.get("head", 88.0), flat_foot(0.0), left_leg(v.get("sw", 0.0)), {"l": (0.20, 0.0), "r": (0.15, 0.0)})
        assert not st[2]
        rstates.append(st[0])
    rkeys = [{"t": t, "pose": "tripod-stand", "ease": "smooth", "bones": {k: {kk: r4(vv) for kk, vv in o.items()} for k, o in s.items()}} for t, s in zip(kt, rstates)]
    rcont = [{"anchor": a, "fromS": 0, "toS": 6.0} for a in ("heel-r", "ball-r", "tip-l", "tip-r")]
    rest = make_clip("rest-on-crutches", "Resting on the crutches", "Ausruhen auf den Krücken",
                     "Standing in the tripod stance: leaning onto the crutch grips, straightening up and looking around while breathing slowly.",
                     "Stehen im Dreipunktstand: Er lehnt sich auf die Krückengriffe, richtet sich auf und schaut umher, während er ruhig atmet.",
                     6.0, 30, True, "none", rkeys, rcont, [{"id": "rest", "fromS": 0, "toS": 6.0, "title": tx("Quiet standing", "Ruhiges Stehen")}], [],
                     [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": PIDX["recovering"]}, {"t": 3.0, "v": PIDX["quiet"]}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.55}, {"t": 3, "v": 0.45}, {"t": 6, "v": 0.55}]}])
    structure = {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": [],
                 "assumptions": [tx("The crutch tips (0.6 kg for two crutches with cuffs) are unilateral contacts with rubber friction; the share they carry follows the elastic-support model of the solver and is not a clinical load measurement.", "Die Krückenspitzen (1,1 kg für zwei Krücken mit Manschetten) sind einseitige Kontakte mit Gummireibung; ihr Lastanteil folgt dem Elastizitätsmodell des Lösers und ist keine klinische Lastmessung."),
                                 tx("The left foot is off-loaded and never a contact; segments are point masses at the de Leva centres.", "Der linke Fuß ist entlastet und nie ein Kontakt; Segmente sind Punktmassen in den Schwerpunkten nach de Leva."),
                                 tx("Not a clinical gait model.", "Kein klinisches Gangmodell.")], "clinical": False}
    mass = mass_total(rig)
    reps = CD.reps(DECS)
    spec = dict(
        id=ID, human=h, rig=rig, anchors=ANCH, title=tx("Person on Forearm Crutches", "Person an Unterarmgehstützen"),
        summary=tx("A 24-year-old man, 1.82 m, in a hooded sweatshirt and sports shorts moves on two forearm crutches in a three-point swing gait: the left foot sits in an orthopaedic walking boot and stays off the ground while the crutch tips and the right foot carry him.",
                   "Ein 24-jähriger Mann, 1,82 m, in Kapuzenpullover und Sportshorts bewegt sich mit zwei Unterarmgehstützen im Dreipunkt-Durchschwunggang: Der linke Fuß steckt in einem orthopädischen Walking-Stiefel und bleibt vom Boden frei, während Krückenspitzen und rechter Fuß ihn tragen."),
        tags=["human", "crutches", "mobility-aid", "three-point-gait", "non-weight-bearing", "walking-boot"], refs=REFS,
        notes=tx("Proportions after Drillis and Contini, masses after de Leva (male), surface area after DuBois, clothing after ISO 9920, basal metabolism after Mifflin-St Jeor. The forearm crutches follow ISO 11334-1 in construction (cuff, grip, ferrule); length 0.9 m grip to tip. Crutch-walking effort is an authored estimate guided by Waters and Mulroy 1999.", "Proportionen nach Drillis und Contini, Massen nach de Leva (männlich), Oberfläche nach DuBois, Kleidung nach ISO 9920, Grundumsatz nach Mifflin-St Jeor. Die Unterarmgehstützen folgen ISO 11334-1 im Aufbau (Manschette, Griff, Zwinge); Länge 0,9 m von Griff bis Spitze. Der Aufwand beim Gehen an Krücken ist eine Schätzung, angelehnt an Waters und Mulroy 1999."),
        dims=dict(heightM=1.83, widthM=1.0, depthM=0.65, massKg=round(mass, 2)), materials=MATS, regions=REGIONS, env=ENV, body=BODY, respiration=RESP,
        emitters=std_emitters("hoodie", 0.25), emit_default={"plume": 0.45}, poses=poses, clips=[gait, rest], reps=reps,
        assumptions=[tx("Reference case: a sunny spring day, 18 degrees C air, 24 degrees C radiant temperature, light tailwind.", "Referenzfall: sonniger Frühlingstag, 18 Grad C Luft, 24 Grad C Strahlungstemperatur, leichter Rückenwind."),
                     tx("Clothing: T-shirt and hooded sweatshirt, shorts, bare legs; the boot has a foam liner; the crutch regions are passive.", "Kleidung: T-Shirt und Kapuzenpullover, Shorts, nackte Beine; der Stiefel hat ein Schaumfutter; die Krückenregionen sind passiv."),
                     tx("Respiration angles, areas and rates are authored estimates guided by Gupta et al. 2010; exertion raises rate and volume and switches to nose and mouth.", "Atemwinkel, Flächen und Raten sind Schätzungen, angelehnt an Gupta et al. 2010; Anstrengung erhöht Rate und Volumen und schaltet auf Nase und Mund.")],
        structure=structure, presentation={"defaultPose": "tripod-stand", "defaultClip": "three-point-gait", "defaultScale": 100, "viewpoints": ["elevation"]})
    return assemble(spec)


from crutches_texts import DECS

if __name__ == "__main__":
    doc = build()
    print(write_doc(doc, ID), len(json.dumps(doc)))
