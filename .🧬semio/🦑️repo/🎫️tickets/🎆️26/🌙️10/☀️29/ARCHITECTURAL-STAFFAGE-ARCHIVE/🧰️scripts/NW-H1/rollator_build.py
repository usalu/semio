"""NW-H1: assemble and write person-with-rollator."""
import sys, json
from rollator_phys import *
from docbuild import *
import rollator_draw as RD

GREF = [DELEVA_REF, DRILLIS_REF, DUBOIS_REF, ISO9920, ISO8996, ASHRAE, ("gagge-1971", GAGGE[1]), ("gupta-2010", GUPTA[1]), WINTER, PERRY]
REFS = [ref(*r) if isinstance(r, tuple) else r for r in GREF] + [
    ref("iso-11199-2-2021", "ISO 11199-2:2021 Assistive products for walking manipulated by both arms: requirements and test methods, Part 2: Rollators (overall dimensions, handle height range, seat height, wheel diameters, brakes). Used as guidance for the construction; dimensions are authored."),
    ref("roza-shizgal-1984", "Roza, A. M. and Shizgal, H. M. (1984). The Harris Benedict equation reevaluated: resting energy requirements and the body cell mass. American Journal of Clinical Nutrition 40(1), 168-182.")]

A_ = dubois(62.0, 1.58)

MATS = [
    mat("skin-aged", "Aged skin", "Gealterte Haut", 0.98, 0.6, src="authored-estimate", ref="ashrae-2021"),
    mat("hair-white", "White hair", "Weißes Haar", 0.95, 0.45),
    mat("loden-wool", "Loden wool", "Lodenwolle", 0.92, 0.65, ref="ashrae-2021"),
    mat("wool-beret", "Wool felt beret", "Baskenmütze aus Wollfilz", 0.92, 0.85),
    mat("wool-trouser", "Wool trousers", "Wollhose", 0.92, 0.7, ref="ashrae-2021"),
    mat("leather-shoe", "Leather shoe", "Lederschuh", 0.90, 0.8),
    mat("cotton-blouse", "Cotton blouse", "Baumwollbluse", 0.90, 0.4, ref="ashrae-2021"),
    mat("wool-knit", "Wool cardigan", "Wollstrickjacke", 0.90, 0.6),
    mat("still-air", "Still air", "Ruhende Luft", 0.90),
    mat("alu-red-paint", "Red powder-coated aluminium", "Rot pulverbeschichtetes Aluminium", 0.90, 0.65, densityKgM3=2700, conductivityWmK=160, specificHeatJKgK=900),
    mat("nylon-seat", "Nylon seat and mesh basket", "Nylon-Sitz und Netzkorb", 0.90, 0.6),
    mat("rubber-grip", "Rubber grip", "Gummigriff", 0.93, 0.9, ref="ashrae-2021"),
    mat("pu-tyre", "Polyurethane tyre", "Polyurethanreifen", 0.93, 0.9),
    mat("cotton-tote", "Cotton tote bag", "Baumwolltasche", 0.90, 0.5),
]

REGIONS = [
    dict(id="face-skin", en="Face and neck skin", de="Haut von Gesicht und Hals", kind="skin", model="body", material="skin-aged", frac=0.045, L=0.15, geom="sphere", svf=0.6, seg="head", ms=0.10, sf=0.045, layers=[], ex=1.0, su=0.2),
    dict(id="hair", en="White hair at nape and temples", de="Weißes Haar an Nacken und Schläfen", kind="hair", model="body", material="hair-white", frac=0.010, L=0.15, geom="sphere", svf=0.7, seg="head", ms=0.01, sf=0.01,
         layers=[layer("hair-pelt", "hair", "hair-white", 0.04, 300, "Hair and trapped air", "Haar und eingeschlossene Luft", 0.01)], ex=1.0, su=0.25),
    dict(id="beret", en="Wool beret", de="Baskenmütze", kind="clothing", model="body", material="wool-beret", frac=0.020, L=0.20, geom="sphere", svf=0.8, seg="head", ms=0.02, sf=0.02,
         garments=[("felt", "clothing", "wool-beret", 0.18, 800, "Wool felt", "Wollfilz", 0.004)], ex=1.0, su=0.3),
    dict(id="hands", en="Hands", de="Hände", kind="skin", model="body", material="skin-aged", frac=0.05, L=0.04, geom="cylinder", svf=0.5, seg="hand", ms=0.02, sf=0.05, layers=[], ex=1.0, su=0.2),
    dict(id="coat", en="Loden coat with cardigan and blouse", de="Lodenmantel mit Strickjacke und Bluse", kind="clothing", model="body", material="loden-wool", frac=0.621, fcl=1.25, L=0.30, geom="cylinder", svf=0.5, seg="trunk", ms=0.60, sf=0.621,
         garments=[("blouse", "clothing", "cotton-blouse", 0.25, 400, "Cotton blouse", "Baumwollbluse", 0.0008), ("cardigan", "clothing", "wool-knit", 0.28, 600, "Wool cardigan", "Wollstrickjacke", 0.004), ("loden", "clothing", "loden-wool", 0.58, 1500, "Loden coat", "Lodenmantel", 0.006)], ex=0.95, su=0.22),
    dict(id="trousers", en="Wool trousers with tights", de="Wollhose mit Strumpfhose", kind="clothing", model="body", material="wool-trouser", frac=0.184, fcl=1.1, L=0.12, geom="cylinder", svf=0.35, seg="leg", ms=0.20, sf=0.184,
         garments=[("tights", "clothing", "cotton-blouse", 0.05, 150, "Tights", "Strumpfhose", 0.001), ("wool", "clothing", "wool-trouser", 0.32, 900, "Wool trousers", "Wollhose", 0.003)], ex=0.9, su=0.18),
    dict(id="shoes", en="Laced leather shoes with wool socks", de="Lederschnürschuhe mit Wollsocken", kind="clothing", model="body", material="leather-shoe", frac=0.07, L=0.06, geom="cylinder", svf=0.25, seg="foot", ms=0.05, sf=0.07,
         garments=[("socks", "clothing", "wool-knit", 0.08, 500, "Wool socks", "Wollsocken", 0.003), ("leather", "clothing", "leather-shoe", 0.05, 1200, "Leather upper", "Lederschaft", 0.003)], ex=1.0, su=0.12),
    dict(id="frame", en="Red aluminium frame, rims and hubs", de="Roter Aluminiumrahmen, Felgen und Naben", kind="surface", model="passive", material="alu-red-paint", area=0.57, L=0.025, geom="cylinder", svf=0.6, cap=4900, ex=1.0, su=0.3),
    dict(id="seat-basket", en="Seat pad, backrest strap and mesh basket", de="Sitzpolster, Rückenband und Netzkorb", kind="surface", model="passive", material="nylon-seat", area=0.70, L=0.40, geom="vertical-plate", svf=0.5, cap=1500, ex=0.85, su=0.25),
    dict(id="grips", en="Rubber hand grips", de="Gummi-Handgriffe", kind="surface", model="passive", material="rubber-grip", area=0.034, L=0.03, geom="cylinder", svf=0.5, cap=13000, ex=0.8, su=0.2),
    dict(id="tyres", en="Polyurethane tyres", de="Polyurethanreifen", kind="tyre", model="passive", material="pu-tyre", area=0.10, L=0.20, geom="cylinder", svf=0.4, cap=8000, ex=1.0, su=0.1),
    dict(id="bag", en="Shopping bag in the basket", de="Einkaufstasche im Korb", kind="surface", model="passive", material="cotton-tote", area=0.35, L=0.30, geom="vertical-plate", svf=0.3, cap=800, ex=0.85, su=0.2),
]

PATTERNS = [
    pattern("quiet", "Quiet standing at the grips", "Ruhiges Stehen an den Griffen", 16, 0.45, (0.38, 0.04, 0.45, 0.13), "nose"),
    pattern("slow-walking", "Slow walking with the rollator", "Langsames Gehen mit dem Rollator", 20, 0.70, (0.42, 0.02, 0.52, 0.04), "nose"),
    pattern("seated-rest", "Resting on the seat", "Ausruhen auf dem Sitz", 15, 0.45, (0.38, 0.04, 0.45, 0.13), "nose"),
    pattern("stooping", "Stooping to the basket", "Bücken zum Korb", 19, 0.6, (0.42, 0.02, 0.50, 0.06), "nose-and-mouth"),
]
PIDX = {p["id"]: i for i, p in enumerate(PATTERNS)}

RESP = {"noseEmitter": "breath-nose", "mouthEmitter": "breath-mouth", "exhaledTempC": 33.0, "exhaledRelativeHumidity": 0.95, "patterns": PATTERNS,
        "motion": [{"bone": "torso", "angleDeg": 0.8, "dy": 0.004}], "source": "authored-estimate", "reference": "gupta-2010"}

BODY = {"species": "human", "massKg": 62.0, "skinAreaM2": round(A_, 3), "basalMetabolismW": 56.6, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False,
        "maxSkinBloodFlowLM2H": 60, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "gagge-1971"}

ENV = {"airTempC": 8, "meanRadiantTempC": 8, "skyTempC": 0, "groundTempC": 9, "relativeHumidity": 0.8, "windMS": 1.2, "windDirectionDeg": 180, "solarWM2": 150, "diffuseFraction": 0.9, "pressurePa": 101325}

FEET = ["heel-l", "ball-l", "heel-r", "ball-r"]
HAND_C = {"fraction": 0.35, "tempC": 8}
FOOT_C = {"fraction": 0.30, "tempC": 8}
LOAD = [{"id": "shopping", "kind": "carried", "title": tx("Shopping in the basket (2 kg)", "Einkauf im Korb (2 kg)"), "anchor": "basket", "fxN": 0, "fyN": -19.61, "source": "authored-estimate"}]


def pose_entry(id_, en, de, den, dede, state, pel, contacts, rule, key=False):
    offs, wa, bad = state
    assert not bad, (id_, bad)
    lim_check(rig, offs, id_)
    return dict(id=id_, en=en, de=de, den=den, dede=dede, offs=offs, root=(0.0, pel[1] - h.hh), contacts=contacts, rule=rule, loads=LOAD, key=key)


RULE_STAND = dict(met=1.2, pat="quiet", em={"plume": 0.4}, contact={"hands": HAND_C, "shoes": FOOT_C, "grips": {"fraction": 0.5, "tempC": 30}, "tyres": {"fraction": 0.01, "tempC": 8}})
RULE_WALK = dict(met=1.9, pat="slow-walking", em={"plume": 0.65}, contact={"hands": HAND_C, "shoes": FOOT_C, "grips": {"fraction": 0.5, "tempC": 30}, "tyres": {"fraction": 0.01, "tempC": 8}}, su={"coat": 0.3})
RULE_SEAT = dict(met=1.0, pat="seated-rest", em={"plume": 0.3}, contact={"hands": {"fraction": 0.2, "tempC": 12}, "coat": {"fraction": 0.1, "tempC": 10}, "trousers": {"fraction": 0.06, "tempC": 10}, "shoes": FOOT_C, "tyres": {"fraction": 0.01, "tempC": 8}},
                 ex={"coat": 0.85, "trousers": 0.8, "grips": 0.9, "seat-basket": 0.7})
RULE_STOOP = dict(met=1.8, pat="stooping", em={"plume": 0.5}, contact={"hands": {"fraction": 0.2, "tempC": 8}, "shoes": FOOT_C, "tyres": {"fraction": 0.01, "tempC": 8}}, su={"coat": 0.15})


def build():
    st_stand = stand_state()
    st_seat = seated_state()
    st_stoop = stoop_state()
    tw = 0.26 * T_WALK
    offs_w, pel_w, bad = walk_state(tw, GAIT)
    poses = [
        pose_entry("stand-at-grips", "Standing at the grips", "Stehen an den Griffen", "Both hands on the grips, the back rounded, weight evenly on both feet, brakes locked; the shopping rides in the basket.", "Beide Hände an den Griffen, runder Rücken, Gewicht gleichmäßig auf beiden Füßen, Bremsen angezogen; der Einkauf liegt im Korb.", st_stand, (0, 0.835), FEET, RULE_STAND),
        pose_entry("walking-midstance", "Walking, mid-stance", "Gehen, Standbeinmitte", "Left foot flat under the body, right leg swinging through, the rollator rolling ahead on its four wheels.", "Linker Fuß flach unter dem Körper, rechtes Bein schwingt durch, der Rollator rollt auf vier Rädern voraus.", (offs_w, None, []), pel_w, ["heel-l", "ball-l"], RULE_WALK),
        pose_entry("seated-rest", "Resting on the seat", "Ausruhen auf dem Sitz", "Sitting upright on the seat against the strap, hands on the lap, feet flat on the ground; the grips stand behind her back.", "Aufrecht auf dem Sitz am Rückenband, Hände im Schoß, Füße flach am Boden; die Griffe stehen hinter dem Rücken.", st_seat, (0, SEAT_Y + 0.095), FEET, RULE_SEAT),
        pose_entry("stoop-to-basket", "Stooping to the basket", "Bücken zum Korb", "Knees and back flexed, both hands reaching into the basket to put the shopping down.", "Knie und Rücken gebeugt, beide Hände greifen in den Korb, um den Einkauf abzulegen.", st_stoop, (0, 0.73), FEET, RULE_STOOP),
    ]
    N = 26
    dt = T_WALK / N
    states = []
    for i in range(N + 1):
        o, pel, bad = walk_state(i * dt, GAIT)
        assert not bad, (i, bad)
        lim_check(rig, o, "walk%d" % i)
        states.append(o)
    wkeys = keys_from_states(states, dt, "walking-midstance")
    wcont = GAIT.spans()
    wphases = [{"id": "double-support-left-lead", "fromS": 0, "toS": r4(0.14 * T_WALK), "title": tx("Double support, left foot leading", "Doppelstütz, linker Fuß vorn")},
               {"id": "left-single-support", "fromS": r4(0.14 * T_WALK), "toS": r4(0.5 * T_WALK), "title": tx("Left single support, right leg swinging", "Linke Einzelstütz, rechtes Bein schwingt")},
               {"id": "double-support-right-lead", "fromS": r4(0.5 * T_WALK), "toS": r4(0.64 * T_WALK), "title": tx("Double support, right foot leading", "Doppelstütz, rechter Fuß vorn")},
               {"id": "right-single-support", "fromS": r4(0.64 * T_WALK), "toS": T_WALK, "title": tx("Right single support, left leg swinging", "Rechte Einzelstütz, linkes Bein schwingt")}]
    wev = [ev(0, "contact-on", "Left heel strike", "Linker Fersenkontakt", "heel-l"), ev(0.14 * T_WALK, "contact-off", "Right toe-off", "Rechter Abdruck", "ball-r"),
           ev(0.5 * T_WALK, "contact-on", "Right heel strike", "Rechter Fersenkontakt", "heel-r"), ev(0.64 * T_WALK, "contact-off", "Left toe-off", "Linker Abdruck", "ball-l")]
    wtracks = [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": PIDX["slow-walking"]}]},
               {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.65}, {"t": T_WALK, "v": 0.65}]}]
    walk = make_clip("rollator-walk", "Walking with the rollator", "Gehen mit dem Rollator",
                     "Slow stride of 0.63 m in 1.3 s (0.48 m/s) behind a rollator whose 0.2 m wheels turn exactly once per stride; the trunk stays stooped, the forearms rest on the grips.",
                     "Langsamer Schritt von 0,63 m in 1,3 s (0,48 m/s) hinter einem Rollator, dessen 0,2-m-Räder sich genau einmal pro Doppelschritt drehen; der Rumpf bleibt gebeugt, die Unterarme ruhen auf den Griffen.",
                     T_WALK, 30, True, "plant", wkeys, wcont, wphases, wev, wtracks, advance=S_WALK)
    rest_states = []
    kt = [0.0, 1.5, 3.0, 4.5, 6.0]
    specs = [dict(), dict(sway=0.025, lean=71.0, neck=60.0, head=76.0), dict(sway=0.0, lean=77.0, neck=67.0, head=84.0), dict(sway=-0.02, lean=75.0, neck=65.0, head=77.0, y=0.84), dict()]
    for sp in specs:
        o, wa, bad = stand_state(**sp)
        assert not bad
        lim_check(rig, o, "rest")
        rest_states.append(o)
    rkeys = []
    for t, o in zip(kt, rest_states):
        rkeys.append({"t": t, "pose": "stand-at-grips", "ease": "smooth", "bones": {k: {kk: r4(vv) for kk, vv in v.items()} for k, v in o.items()}})
    rcont = [{"anchor": a, "fromS": 0, "toS": 6.0} for a in FEET]
    rest = make_clip("rest-at-the-grips", "Resting at the grips", "Ausruhen an den Griffen",
                     "Standing still behind the locked rollator: she leans onto the grips, straightens, looks around and shifts her weight, breathing quietly.",
                     "Ruhiges Stehen hinter dem festgestellten Rollator: Sie lehnt sich auf die Griffe, richtet sich auf, schaut umher und verlagert das Gewicht, ruhig atmend.",
                     6.0, 30, True, "none", rkeys, rcont, [{"id": "rest", "fromS": 0, "toS": 6.0, "title": tx("Quiet standing", "Ruhiges Stehen")}],
                     [ev(0, "load-on", "Forearm weight on the grips", "Unterarmlast auf den Griffen", None)] if False else [],
                     [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": PIDX["quiet"]}]}, {"target": "emitter:plume", "ease": "smooth", "keys": [{"t": 0, "v": 0.4}, {"t": 3, "v": 0.5}, {"t": 6, "v": 0.4}]}])
    structure = {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": ["wheel-rear-contact", "wheel-front-contact"],
                 "assumptions": [tx("The rollator (7.2 kg) is carried by its four wheels, modelled as two rolling supports (rear pair, front pair) that are always active; the hands push on it through the grips and the feet carry the rest according to the elastic-support model of the solver.", "Der Rollator (7,2 kg) steht auf seinen vier Rädern, modelliert als zwei immer aktive Rollenlager (hinteres und vorderes Paar); die Hände drücken über die Griffe darauf, die Füße tragen den Rest nach dem Elastizitätsmodell des Lösers."),
                                 tx("Segments are point masses at the de Leva centres; the 2 kg of shopping is an applied load in the basket and not part of the listed mass.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; die 2 kg Einkauf sind eine aufgebrachte Last im Korb und nicht in der Gesamtmasse enthalten."),
                                 tx("Not a clinical gait or load model.", "Kein klinisches Gang- oder Lastmodell.")], "clinical": False}
    mass = mass_total(rig)
    reps = RD.reps()
    for k in ("500", "200", "100"):
        if k not in reps:
            reps[k] = {"unavailable": {"status": "unavailable", "denominator": int(k), "reason": tx("Not yet drawn.", "Noch nicht gezeichnet."), "limitingFeature": tx("Draft", "Entwurf"), "limitingSizeM": 0.025, "alternatives": [{"kind": "scale", "denominator": 50}]}}
    spec = dict(
        id=ID, human=h, rig=rig, anchors=ANCH, title=tx("Person with Rollator", "Person mit Rollator"),
        summary=tx("A 79-year-old woman, 1.58 m, in a loden coat and beret walks stooped behind a four-wheel rollator with seat, basket, brake levers and a shopping bag; the aid carries part of her weight, rolls on 0.2 m wheels and doubles as a seat.",
                   "Eine 79-jährige Frau, 1,58 m, in Lodenmantel und Baskenmütze geht gebeugt hinter einem vierrädrigen Rollator mit Sitz, Korb, Bremshebeln und Einkaufstasche; das Hilfsmittel trägt einen Teil ihres Gewichts, rollt auf 0,2-m-Rädern und dient als Sitz."),
        tags=["human", "elderly", "rollator", "mobility-aid", "walking", "seat"], refs=REFS,
        notes=tx("Proportions after Drillis and Contini, masses after de Leva (female), surface area after DuBois, clothing insulation after ISO 9920. The rollator follows the dimensional ranges of ISO 11199-2 (wheelbase 0.52 m, grip height 0.85 m, seat height 0.52 m, 0.2 m wheels, 7.2 kg). All other numbers are authored estimates.", "Proportionen nach Drillis und Contini, Massen nach de Leva (weiblich), Oberfläche nach DuBois, Kleidungsdämmung nach ISO 9920. Der Rollator folgt den Maßbereichen von ISO 11199-2 (Radstand 0,52 m, Griffhöhe 0,85 m, Sitzhöhe 0,52 m, 0,2-m-Räder, 7,2 kg). Alle übrigen Zahlen sind Schätzungen des Autors."),
        dims=dict(heightM=1.61, widthM=1.16, depthM=0.62, massKg=round(mass, 2)), materials=MATS, regions=REGIONS, env=ENV, body=BODY, respiration=RESP,
        emitters=std_emitters("coat", 0.22), emit_default={"plume": 0.4}, poses=poses, clips=[walk, rest], reps=reps,
        assumptions=[tx("Reference case: an overcast autumn morning, 8 degrees C, light headwind, diffuse light.", "Referenzfall: bedeckter Herbstmorgen, 8 Grad C, leichter Gegenwind, diffuses Licht."),
                     tx("Clothing: blouse, cardigan and loden coat (ISO 9920 garment values, I_cl = 0.161 + 0.835 sum I_clu); the thermal regions of the aid are passive.", "Kleidung: Bluse, Strickjacke und Lodenmantel (ISO-9920-Kleidungswerte, I_cl = 0,161 + 0,835 Summe I_clu); die thermischen Regionen des Hilfsmittels sind passiv."),
                     tx("Respiration angles, areas and rates are authored estimates guided by Gupta et al. 2010.", "Atemwinkel, Flächen und Raten sind Schätzungen, angelehnt an Gupta et al. 2010.")],
        structure=structure, presentation={"defaultPose": "stand-at-grips", "defaultClip": "rollator-walk", "defaultScale": 100, "viewpoints": ["elevation"]})
    return assemble(spec)


if __name__ == "__main__":
    doc = build()
    p = write_doc(doc, ID)
    print(p, len(json.dumps(doc)))
