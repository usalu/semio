import json
from cane_phys import *
from docbuild import *
import cane_draw as CD
from cane_texts import DECS

REFS = [ref(*r) for r in (DELEVA_REF, DRILLIS_REF, DUBOIS_REF, ISO9920, ISO8996, ASHRAE, ("gagge-1971", GAGGE[1]), ("gupta-2010", GUPTA[1]), WINTER, PERRY)] + [
    ref("jacobson-2013", "Jacobson, W. H. (2013). The Art and Science of Teaching Orientation and Mobility to Persons with Visual Impairments, 2nd ed., AFB Press (long cane techniques: constant contact and two-point touch, diagonal technique, cane length to the sternum). Used as qualitative guidance; lengths and angles are authored."),
    ref("roza-shizgal-1984", "Roza, A. M. and Shizgal, H. M. (1984). The Harris Benedict equation reevaluated: resting energy requirements and the body cell mass. American Journal of Clinical Nutrition 40(1), 168-182.")]
A_ = dubois(62.0, 1.68)
MATS = [
    mat("skin-medium", "Skin", "Haut", 0.98, 0.65, ref="ashrae-2021"), mat("hair-brown", "Brown hair", "Braunes Haar", 0.95, 0.85),
    mat("denim-jacket", "Denim", "Jeansstoff", 0.92, 0.7, ref="ashrae-2021"), mat("denim-jeans", "Dark denim jeans", "Dunkle Jeans", 0.92, 0.8, ref="ashrae-2021"),
    mat("leather-boot", "Black leather ankle boot", "Schwarze Lederstiefelette", 0.90, 0.9), mat("cotton-shirt", "Cotton shirt", "Baumwollhemd", 0.90, 0.4, ref="ashrae-2021"), mat("still-air", "Still air", "Ruhende Luft", 0.90),
    mat("sunglass-polymer", "Dark polycarbonate lenses", "Dunkle Polycarbonatgläser", 0.90, 0.9), mat("bag-leather", "Leather bag", "Ledertasche", 0.90, 0.8),
    mat("cane-alu-white", "White painted aluminium with reflective band", "Weiß lackiertes Aluminium mit Reflexband", 0.88, 0.25, densityKgM3=2700, conductivityWmK=160, specificHeatJKgK=900),
    mat("grip-eva", "EVA foam grip", "EVA-Schaumgriff", 0.92, 0.5), mat("nylon-roller", "Nylon roller tip", "Nylon-Rollspitze", 0.90, 0.6),
]
REGIONS = [
    dict(id="face-skin", en="Face and neck skin", de="Haut von Gesicht und Hals", kind="skin", model="body", material="skin-medium", frac=0.045, L=0.15, geom="sphere", svf=0.6, seg="head", ms=0.10, sf=0.045, layers=[], ex=1.0, su=0.5),
    dict(id="hair", en="Shoulder-length hair", de="Schulterlanges Haar", kind="hair", model="body", material="hair-brown", frac=0.02, L=0.16, geom="sphere", svf=0.8, seg="head", ms=0.02, sf=0.02,
         layers=[layer("hair-pelt", "hair", "hair-brown", 0.05, 250, "Hair and trapped air", "Haar und eingeschlossene Luft", 0.012)], ex=1.0, su=0.6),
    dict(id="hands", en="Hands", de="Hände", kind="skin", model="body", material="skin-medium", frac=0.05, L=0.04, geom="cylinder", svf=0.5, seg="hand", ms=0.02, sf=0.05, layers=[], ex=1.0, su=0.45),
    dict(id="jacket", en="Denim jacket over cotton shirt", de="Jeansjacke über Baumwollhemd", kind="clothing", model="body", material="denim-jacket", frac=0.47, fcl=1.2, L=0.30, geom="cylinder", svf=0.5, seg="trunk", ms=0.58, sf=0.47,
         garments=[("shirt", "clothing", "cotton-shirt", 0.20, 400, "Cotton shirt", "Baumwollhemd", 0.001), ("denim", "clothing", "denim-jacket", 0.30, 1000, "Denim jacket", "Jeansjacke", 0.002)], ex=0.95, su=0.45),
    dict(id="jeans", en="Dark denim jeans", de="Dunkle Jeans", kind="clothing", model="body", material="denim-jeans", frac=0.32, fcl=1.1, L=0.12, geom="cylinder", svf=0.35, seg="leg", ms=0.22, sf=0.32,
         garments=[("denim", "clothing", "denim-jeans", 0.24, 1000, "Denim jeans", "Jeans", 0.002)], ex=0.9, su=0.4),
    dict(id="boots", en="Leather ankle boots with socks", de="Lederstiefeletten mit Socken", kind="clothing", model="body", material="leather-boot", frac=0.095, L=0.08, geom="cylinder", svf=0.25, seg="foot", ms=0.06, sf=0.095,
         garments=[("sock", "clothing", "cotton-shirt", 0.04, 300, "Cotton socks", "Baumwollsocken", 0.002), ("leather", "clothing", "leather-boot", 0.06, 1500, "Leather upper", "Lederschaft", 0.003)], ex=1.0, su=0.2),
    dict(id="sunglasses", en="Dark sunglasses", de="Dunkle Sonnenbrille", kind="glazing", model="passive", material="sunglass-polymer", area=0.006, L=0.05, geom="vertical-plate", svf=0.6, cap=2500, ex=1.0, su=0.7),
    dict(id="bag", en="Leather shoulder bag", de="Leder-Umhängetasche", kind="surface", model="passive", material="bag-leather", area=0.12, L=0.2, geom="vertical-plate", svf=0.3, cap=1500, ex=0.8, su=0.4),
    dict(id="cane-shaft", en="White aluminium shaft with red reflective band", de="Weißer Aluminiumschaft mit rotem Reflexband", kind="surface", model="passive", material="cane-alu-white", area=0.05, L=0.012, geom="cylinder", svf=0.5, cap=2500, ex=1.0, su=0.6),
    dict(id="cane-grip", en="Foam grip with wrist strap", de="Schaumgriff mit Handschlaufe", kind="surface", model="passive", material="grip-eva", area=0.008, L=0.03, geom="cylinder", svf=0.5, cap=8000, ex=0.8, su=0.5),
    dict(id="cane-tip", en="Roller tip", de="Rollspitze", kind="surface", model="passive", material="nylon-roller", area=0.003, L=0.03, geom="sphere", svf=0.2, cap=5000, ex=1.0, su=0.1),
]
PATTERNS = [pattern("quiet", "Quiet standing", "Ruhiges Stehen", 15, 0.50, (0.38, 0.04, 0.45, 0.13), "nose"),
            pattern("brisk-walking", "Brisk walking with the cane", "Zügiges Gehen mit dem Stock", 20, 0.95, (0.42, 0.02, 0.52, 0.04), "nose")]
PIDX = {p["id"]: i for i, p in enumerate(PATTERNS)}
RESP = {"noseEmitter": "breath-nose", "mouthEmitter": "breath-mouth", "exhaledTempC": 33.5, "exhaledRelativeHumidity": 0.95, "patterns": PATTERNS, "motion": [{"bone": "torso", "angleDeg": 1.0, "dy": 0.005}], "source": "authored-estimate", "reference": "gupta-2010"}
BODY = {"species": "human", "massKg": 62.0, "skinAreaM2": round(A_, 3), "basalMetabolismW": 64.4, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "gagge-1971"}
ENV = {"airTempC": 12, "meanRadiantTempC": 14, "skyTempC": 4, "groundTempC": 12, "relativeHumidity": 0.65, "windMS": 2.0, "windDirectionDeg": 180, "solarWM2": 400, "diffuseFraction": 0.4, "pressurePa": 101325}


def rule(met, pat, plume, walk=False):
    return dict(met=met, pat=pat, em={"plume": plume}, contact={"hands": {"fraction": 0.2, "tempC": 12}, "cane-grip": {"fraction": 0.4, "tempC": 28}, "boots": {"fraction": 0.3, "tempC": 12}, "cane-tip": {"fraction": 0.05, "tempC": 12}}, su={})


def pe(id_, en, de, den, dede, st, pel, contacts, r):
    offs, wa, bad = st[0], st[1], st[2]
    assert not bad, (id_, bad)
    lim_check(rig, offs, id_)
    return dict(id=id_, en=en, de=de, den=den, dede=dede, offs=offs, root=(0.0, pel[1] - h.hh), contacts=contacts, rule=r, loads=[])


def build():
    py0 = h.hh - 0.012
    tw = 0.3
    ow, pw, bw = walk_state(tw)
    poses = [
        pe("constant-contact-stance", "Constant-contact stance", "Stand mit Dauerkontakt", "Standing on both feet with the cane held diagonally ahead, the roller tip in contact with the ground about a step in front; the head turned up and listening.", "Stand auf beiden Füßen, der Stock diagonal voraus gehalten, die Rollspitze etwa einen Schritt vor dem Körper am Boden; der Kopf nach oben gewandt, lauschend.", stand_state(), (0, py0), ["heel-l", "ball-l", "heel-r", "ball-r"], rule(1.2, "quiet", 0.4)),
        pe("diagonal-technique", "Diagonal technique", "Diagonaltechnik", "The cane held across the body with the tip close ahead of the feet, as used in crowded rooms and on stairs.", "Der Stock quer vor dem Körper gehalten, die Spitze dicht vor den Füßen, wie in engen Räumen und auf Treppen.", diag_state(), (0, py0), ["heel-l", "ball-l", "heel-r", "ball-r"], rule(1.2, "quiet", 0.4)),
        pe("walking-midstance", "Walking, mid-stance", "Gehen, Standbeinmitte", "Left foot flat under the body, right leg swinging, the left arm swinging opposite and the cane sliding along on its roller tip.", "Linker Fuß flach unter dem Körper, rechtes Bein schwingt, der linke Arm schwingt gegengleich und der Stock gleitet auf seiner Rollspitze mit.", (ow, None, bw), pw, ["heel-l", "ball-l"], rule(2.6, "brisk-walking", 0.7)),
        pe("cane-parked", "Cane held upright", "Stock aufrecht gehalten", "Standing and waiting with the cane held almost upright close to the body, the tip beside the foot.", "Stehen und Warten, der Stock fast senkrecht nahe am Körper gehalten, die Spitze neben dem Fuß.", vertical_state(), (0, py0), ["heel-l", "ball-l", "heel-r", "ball-r"], rule(1.1, "quiet", 0.35)),
    ]
    N = 22
    dt = T_WALK / N
    states = []
    for i in range(N + 1):
        o, pel, bad = walk_state(i * dt)
        assert not bad
        lim_check(rig, o, "w%d" % i)
        states.append(o)
    keys = keys_from_states(states, dt, "walking-midstance")
    T = T_WALK
    cont = GAIT.spans()
    phases = [{"id": "double-support-left-lead", "fromS": 0, "toS": r4(0.1 * T), "title": tx("Double support, left foot leading", "Doppelstütz, linker Fuß vorn")},
              {"id": "left-single-support", "fromS": r4(0.1 * T), "toS": r4(0.5 * T), "title": tx("Left single support", "Linke Einzelstütz")},
              {"id": "double-support-right-lead", "fromS": r4(0.5 * T), "toS": r4(0.6 * T), "title": tx("Double support, right foot leading", "Doppelstütz, rechter Fuß vorn")},
              {"id": "right-single-support", "fromS": r4(0.6 * T), "toS": T, "title": tx("Right single support", "Rechte Einzelstütz")}]
    events = [ev(0, "contact-on", "Left heel strike", "Linker Fersenkontakt", "heel-l"), ev(0.5 * T, "contact-on", "Right heel strike", "Rechter Fersenkontakt", "heel-r"), ev(0.6 * T, "contact-off", "Left toe-off", "Linker Abdruck", "ball-l")]
    tracks = [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": PIDX["brisk-walking"]}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.7}, {"t": T, "v": 0.7}]}]
    walk = make_clip("constant-contact-walk", "Walking with constant contact", "Gehen mit Dauerkontakt",
                     "Brisk walking at 1.0 m/s with a 1.1 m stride; the cane tip rolls along the ground a step ahead and its sagittal angle follows the steps, the free arm swings opposite the leg.",
                     "Zügiges Gehen mit 1,0 m/s und 1,1 m Doppelschritt; die Stockspitze rollt einen Schritt voraus über den Boden und der Stockwinkel folgt den Schritten, der freie Arm schwingt gegengleich zum Bein.",
                     T, 30, True, "plant", keys, cont, phases, events, tracks, advance=S_WALK)
    rk = [0.0, 1.5, 3.0, 4.5, 6.0]
    var = [dict(), dict(torso=87.0, head=96.0), dict(torso=90.0, head=84.0), dict(torso=89.0, head=92.0, grip=(0.10, -0.30)), dict()]
    rkeys = []
    for t, v in zip(rk, var):
        st = stand_state(torso=v.get("torso", 89.0), grip=v.get("grip", (0.14, -0.28)), head=v.get("head", 92.0))
        assert not st[2]
        rkeys.append({"t": t, "pose": "constant-contact-stance", "ease": "smooth", "bones": {k: {kk: r4(vv) for kk, vv in o.items()} for k, o in st[0].items()}})
    rcont = [{"anchor": a, "fromS": 0, "toS": 6.0} for a in ("heel-l", "ball-l", "heel-r", "ball-r")]
    rest = make_clip("listen-and-scan", "Listening and scanning", "Lauschen und Abtasten",
                     "Standing at a crossing: the head turns up and down while the cane is pressed ahead and drawn back, breathing quietly.",
                     "Stehen an einer Querung: Der Kopf wendet sich auf und ab, während der Stock vorgeschoben und zurückgezogen wird, ruhiges Atmen.",
                     6.0, 30, True, "none", rkeys, rcont, [{"id": "listening", "fromS": 0, "toS": 6.0, "title": tx("Listening", "Lauschen")}], [],
                     [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": PIDX["quiet"]}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.4}, {"t": 6, "v": 0.4}]}])
    structure = {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": ["cane-tip"],
                 "assumptions": [tx("The cane tip is a permanent rolling support on the ground (a roller carries only vertical force, no braking); the share it carries follows the elastic-support model of the solver and is not a clinical load measurement.", "Die Stockspitze ist ein dauerhaftes Rollenlager am Boden (eine Rolle trägt nur Vertikalkraft, keine Bremskraft); ihr Lastanteil folgt dem Elastizitätsmodell des Lösers und ist keine klinische Lastmessung."),
                                 tx("The two-point sweep happens across the walking direction and is not visible in the elevation; the sagittal tip movement stands for it.", "Das Zweipunkt-Pendeln geschieht quer zur Gehrichtung und ist in der Ansicht nicht sichtbar; die Spitzenbewegung in Gehrichtung steht dafür.")], "clinical": False}
    mass = mass_total(rig)
    reps = CD.reps(DECS)
    spec = dict(
        id=ID, human=h, rig=rig, anchors=ANCH, title=tx("Person with White Cane", "Person mit Blindenlangstock"),
        summary=tx("A blind woman, 1.68 m, in a denim jacket and sunglasses walks briskly with a 1.3 m long white cane held diagonally ahead of her; the roller tip stays in contact with the ground and sweeps ahead of every step.",
                   "Eine blinde Frau, 1,68 m, in Jeansjacke und Sonnenbrille geht zügig mit einem 1,3 m langen weißen Langstock, der diagonal vor ihr gehalten wird; die Rollspitze bleibt am Boden und tastet jedem Schritt voraus."),
        tags=["human", "visually-impaired", "white-cane", "mobility-aid", "walking", "long-cane"], refs=REFS,
        notes=tx("Proportions after Drillis and Contini, masses after de Leva (female), surface area after DuBois, clothing after ISO 9920. Cane length and technique after Jacobson 2013 (qualitative). The 12 mm shaft, the 34 mm roller tip and the red reflective band are authored.", "Proportionen nach Drillis und Contini, Massen nach de Leva (weiblich), Oberfläche nach DuBois, Kleidung nach ISO 9920. Stocklänge und Technik nach Jacobson 2013 (qualitativ). Der 12-mm-Schaft, die 34-mm-Rollspitze und das rote Reflexband sind eigene Annahmen."),
        dims=dict(heightM=1.69, widthM=1.3, depthM=0.4, massKg=round(mass, 2)), materials=MATS, regions=REGIONS, env=ENV, body=BODY, respiration=RESP,
        emitters=std_emitters("jacket", 0.22), emit_default={"plume": 0.4}, poses=poses, clips=[walk, rest], reps=reps,
        assumptions=[tx("Reference case: a cool partly cloudy day, 12 degrees C, moderate wind.", "Referenzfall: kühler, teils bewölkter Tag, 12 Grad C, mäßiger Wind."),
                     tx("Clothing: cotton shirt, denim jacket and jeans; sunglasses, bag and the cane are passive regions.", "Kleidung: Baumwollhemd, Jeansjacke und Jeans; Sonnenbrille, Tasche und Stock sind passive Regionen."),
                     tx("Respiration angles, areas and rates are authored estimates guided by Gupta et al. 2010.", "Atemwinkel, Flächen und Raten sind Schätzungen, angelehnt an Gupta et al. 2010.")],
        structure=structure, presentation={"defaultPose": "constant-contact-stance", "defaultClip": "constant-contact-walk", "defaultScale": 100, "viewpoints": ["elevation"]})
    return assemble(spec)


if __name__ == "__main__":
    print(write_doc(build(), ID))
