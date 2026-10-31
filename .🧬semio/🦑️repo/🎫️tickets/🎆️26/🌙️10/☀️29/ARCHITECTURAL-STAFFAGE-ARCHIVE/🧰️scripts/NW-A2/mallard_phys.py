"""mallard-duck physical scenario (rig, anchors, masses, poses, clips, thermal) - hand-entered numbers, see sources in the references."""
import math
from nw import *

ID = "mallard-duck"
MASS = 1.15

REFS = [
    ("cramp-simmons-1977", "Cramp, S., Simmons, K. E. L. (eds) (1977). Handbook of the Birds of Europe, the Middle East and North Africa: The Birds of the Western Palearctic, Vol. I (Ostrich to Ducks). Oxford University Press. Mallard Anas platyrhynchos: length 50-65 cm, wingspan 81-98 cm, drake mass about 1.0-1.5 kg.", None),
    ("lasiewski-dawson-1967", "Lasiewski, R. C., Dawson, W. R. (1967). A re-examination of the relation between standard metabolic rate and body weight in birds. The Condor 69, 13-23. Non-passerine standard metabolic rate 78.3 M^0.723 kcal/day (M in kg).", None),
    ("prange-schmidt-nielsen-1970", "Prange, H. D., Schmidt-Nielsen, K. (1970). The metabolic cost of swimming in ducks. Journal of Experimental Biology 53, 763-777. Swimming raises the metabolic rate of ducks several-fold above resting; the factor used here is an authored estimate guided by this study.", None),
    ("lasiewski-calder-1971", "Lasiewski, R. C., Calder, W. A. (1971). A preliminary allometric analysis of respiratory variables in resting birds. Respiration Physiology 11, 152-166. Resting breathing frequency and tidal volume scale with body mass; the mallard values here are authored estimates guided by these allometries.", None),
    ("walsberg-king-1978", "Walsberg, G. E., King, J. R. (1978). The relationship of the external surface area of birds to body mass. Journal of Experimental Biology 76, 185-189. Plumage-covered external area scales as about 8.1 M^0.667 cm2 (M in g).", None),
    ("de-vries-van-eerden-1995", "de Vries, J., van Eerden, M. R. (1995). Thermal conductance in aquatic birds in relation to the degree of water contact, body mass, and body fat: energetic implications of living in a strong cooling environment. Physiological Zoology 68, 1143-1163. Plumage conductance rises strongly with water contact; layer resistances here are authored estimates guided by it.", None),
    ("scholander-1950", "Scholander, P. F., Walters, V., Hock, R., Irving, L. (1950). Body insulation of some arctic and tropical mammals and birds. Biological Bulletin 99, 225-236.", None),
    ("calder-king-1974", "Calder, W. A., King, J. R. (1974). Thermal and caloric relations of birds. In: Farner, D. S., King, J. R. (eds), Avian Biology IV, Academic Press, pp. 259-413. Body temperature of non-passerines about 40-42 degC.", None),
    ("kleiber-1961", "Kleiber, M. (1961). The Fire of Life: An Introduction to Animal Energetics. Wiley, New York. Metabolic scaling 70 kcal/day per kg^0.75.", None),
    ("gupta-2010", "Gupta, J. K., Lin, C.-H., Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39. Exhaled flow is close to a sinusoid; nostril areas and jet angles of the mallard are authored estimates guided by the human case.", "https://research.polyu.edu.hk/en/publications/characterizing-exhaled-airflow-from-breathing-and-talking/"),
    ("schmidt-nielsen-1970", "Schmidt-Nielsen, K., Hainsworth, F. R., Murrish, D. E. (1970). Counter-current heat exchange in the respiratory passages: effect on water and heat balance. Respiration Physiology 9, 263-276. Basis for exhaled air cooler than core temperature.", None),
    ("kahl-1963", "Kahl, M. P. (1963). Thermoregulation in the wood stork, with special reference to the role of the legs. Physiological Zoology 36, 141-151. Bare legs and feet are the main controlled heat-loss path of waterbirds.", None),
    ("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort: emissivity of skin and respiratory heat loss.", None),
    ("dot-pitch", "Print dot pitch and toner spread of a 600 dpi laser printer: see the print profile document of the archive.", None),
]
RID = [r[0] for r in REFS]


def build():
    PA = "pennycuick-2008"
    bones = [
        bone("trunk", None, 0.0, 0.0, 0.0, 0.34, (-80, 80), ("Trunk: body of keel, flight muscles and gut", "Rumpf: Körper mit Kiel, Flugmuskeln und Darm"), kg=0.0, com=(0.17, -0.005), dist="point", ref="cramp-simmons-1977"),
        bone("tail", "trunk", 0.0, 0.012, 172.0, 0.085, (-40, 45), ("Tail with pointed upturned central feathers", "Schwanz mit zugespitzten, aufgebogenen mittleren Federn"), kg=0.02, ref="cramp-simmons-1977"),
        bone("neck-1", "trunk", 0.30, 0.035, 62.0, 0.075, (-100, 80), ("Lower neck", "Unterer Hals"), kg=0.032, ref="cramp-simmons-1977"),
        bone("neck-2", "neck-1", 0.075, 0.0, 20.0, 0.07, (-90, 90), ("Middle neck", "Mittlerer Hals"), kg=0.026, ref="cramp-simmons-1977"),
        bone("neck-3", "neck-2", 0.07, 0.0, -30.0, 0.06, (-90, 90), ("Upper neck", "Oberer Hals"), kg=0.020, ref="cramp-simmons-1977"),
        bone("head", "neck-3", 0.06, 0.0, -62.0, 0.105, (-100, 100), ("Head with flat bill (skull and bill)", "Kopf mit flachem Schnabel (Schädel und Schnabel)"), kg=0.07, com=(0.04, 0.0), dist="point", ref="cramp-simmons-1977"),
        bone("wing-n", "trunk", 0.225, 0.03, 180.0, 0.30, (-100, 30), ("Near wing, folded along the back", "Naher Flügel, am Rücken angelegt"), kg=0.075, ref="cramp-simmons-1977"),
        bone("wing-f", "trunk", 0.225, 0.03, 180.0, 0.30, (-100, 30), ("Far wing, folded along the back", "Ferner Flügel, am Rücken angelegt"), kg=0.075, ref="cramp-simmons-1977"),
    ]
    for s, nm, de in (("n", "near", "nah"), ("f", "far", "fern")):
        tag = ("Nahes" if s == "n" else "Fernes")
        bones += [
            bone(f"thigh-{s}", "trunk", 0.17, -0.06, -60.0, 0.05, (-130, 130), (f"{nm.capitalize()} thigh (femur, inside the body plumage)", f"{tag} Oberschenkel (Femur, im Körpergefieder)"), kg=0.042, ref="cramp-simmons-1977"),
            bone(f"shank-{s}", f"thigh-{s}", 0.05, 0.0, -50.0, 0.085, (-170, 60), (f"{nm.capitalize()} shank (tibiotarsus, drumstick)", f"{tag} Unterschenkel (Tibiotarsus, Keule)"), kg=0.03, ref="cramp-simmons-1977"),
            bone(f"tarsus-{s}", f"shank-{s}", 0.085, 0.0, 40.0, 0.045, (-140, 110), (f"{nm.capitalize()} tarsus (tarsometatarsus, bare)", f"{tag} Lauf (Tarsometatarsus, unbefiedert)"), kg=0.008, ref="cramp-simmons-1977"),
            bone(f"foot-{s}", f"tarsus-{s}", 0.045, 0.0, 75.0, 0.075, (-100, 100), (f"{nm.capitalize()} webbed foot with three toes", f"{tag} Schwimmfuß mit drei Zehen"), kg=0.01, ref="cramp-simmons-1977"),
        ]
    other = sum(b["mass"]["kg"] for b in bones if "mass" in b)
    bones[0]["mass"]["kg"] = round(MASS - other, 4)
    bones[0]["mass"]["source"] = "authored-estimate"
    bones[0]["mass"]["reference"] = "cramp-simmons-1977"

    anchors = [
        anchor("contact-n", "foot-n", 0.04, -0.010, "contact", ("Near foot, centre of the web", "Naher Fuß, Mitte der Schwimmhaut"), "contact", 90, 0.07, 0.05, 0.7),
        anchor("contact-f", "foot-f", 0.04, -0.010, "contact", ("Far foot, centre of the web", "Ferner Fuß, Mitte der Schwimmhaut"), "contact", 90, 0.07, 0.05, 0.7),
        anchor("waterline", "trunk", 0.17, -0.02, "contact", ("Waterline on the hull (buoyant support in water)", "Wasserlinie am Rumpf (Auftrieb im Wasser)"), "contact", 90, 0.30, 0.12, 0.02),
        anchor("nostrils", "head", 0.058, 0.011, "emitter", ("Nares on the bill base", "Nasenlöcher am Schnabelansatz"), None),
        anchor("bill-tip", "head", 0.105, -0.003, "attachment", ("Bill tip (nail)", "Schnabelspitze (Nagel)"), None),
        anchor("eye", "head", 0.032, 0.014, "attachment", ("Eye", "Auge"), None),
        anchor("back-plume", "trunk", 0.17, 0.095, "emitter", ("Back above the trunk (plume source)", "Rücken über dem Rumpf (Fahnenquelle)"), None),
    ]
    return bones, anchors


MATERIALS = [
    material("plumage", ("Contour plumage over down", "Konturgefieder über Daunen"), 0.95, 0.65, 100, 0.04, None, "authored-estimate", "scholander-1950"),
    material("wing-feathers", ("Wing coverts and flight feathers", "Flügeldecken und Schwungfedern"), 0.95, 0.7, 120, 0.05, None, "authored-estimate", "scholander-1950"),
    material("bill-keratin", ("Bill keratin (yellow-green)", "Schnabelhorn (gelbgrün)"), 0.92, 0.5, None, None, None, "authored-estimate", "ashrae-2021"),
    material("scaled-skin", ("Scaled orange skin of tarsus and web", "Beschuppte orange Haut von Lauf und Schwimmhaut"), 0.96, 0.7, None, None, None, "literature", "ashrae-2021"),
]


def regions():
    VR = lambda R: round(R / (0.0165 * 0.6), 2)
    def fl(id, R, th, cap, ti=("Plumage", "Gefieder")):
        return [layer(id + "-down", ("Down layer with enclosed air", "Daunenschicht mit eingeschlossener Luft"), "feather", "plumage", round(th*0.55, 4), round(R*0.6, 3), VR(R*0.6), round(cap*0.4), "authored-estimate", "de-vries-van-eerden-1995"),
                layer(id + "-contour", ("Contour feather layer", "Konturfederschicht"), "feather", "plumage", round(th*0.45, 4), round(R*0.4, 3), VR(R*0.4), round(cap*0.6), "authored-estimate", "scholander-1950")]
    ar = dict(body=0.054, neck=0.012, head=0.006, bill=0.0022, wings=0.021, tail=0.0062, legs=0.0045, feet=0.005)
    skin = sum(ar.values())
    sh = dict(body=0.71, neck=0.06, head=0.07, bill=0.01, wings=0.07, tail=0.01, legs=0.05, feet=0.02)
    tot = sum(sh.values())
    sh = {k: v/tot for k, v in sh.items()}
    sf = lambda k: round(ar[k]/skin, 5)
    regs = [
        region("body-plumage", ("Body plumage (back, breast, belly)", "Körpergefieder (Rücken, Brust, Bauch)"), "feather", "body", "plumage", ar["body"], 0.22, "cylinder", 0.5, fl("body", 0.5, 0.024, 900), 14000, ("trunk", round(sh["body"], 4), sf("body")), "authored-estimate", "walsberg-king-1978"),
        region("neck-plumage", ("Neck plumage", "Halsgefieder"), "feather", "body", "plumage", ar["neck"], 0.05, "cylinder", 0.5, fl("neck", 0.3, 0.014, 600), 10000, ("head", round(sh["neck"], 4), sf("neck")), "authored-estimate", "scholander-1950"),
        region("head-plumage", ("Head plumage (crown, cheek, eye region)", "Kopfgefieder (Scheitel, Wange, Augenpartie)"), "feather", "body", "plumage", ar["head"], 0.045, "sphere", 0.5, fl("head", 0.2, 0.008, 400), 9000, ("head", round(sh["head"], 4), sf("head")), "authored-estimate", "scholander-1950"),
        region("bill", ("Bill (keratin sheath)", "Schnabel (Hornscheide)"), "skin", "body", "bill-keratin", ar["bill"], 0.03, "cylinder", 0.5, [], 2500, ("head", round(sh["bill"], 4), sf("bill")), "authored-estimate", "kahl-1963"),
        region("wing-plumage", ("Folded wings (both)", "Angelegte Flügel (beide)"), "feather", "body", "wing-feathers", ar["wings"], 0.2, "vertical-plate", 0.5, fl("wing", 0.35, 0.016, 700), 6000, ("wing", round(sh["wings"], 4), sf("wings")), "authored-estimate", "scholander-1950"),
        region("tail-plumage", ("Tail feathers", "Schwanzfedern"), "feather", "body", "plumage", ar["tail"], 0.08, "vertical-plate", 0.5, fl("tail", 0.2, 0.01, 400), 3500, ("tail", round(sh["tail"], 4), sf("tail")), "authored-estimate", "scholander-1950"),
        region("legs-skin", ("Bare shank and tarsus (both legs)", "Unbefiederter Unterschenkel und Lauf (beide Beine)"), "skin", "body", "scaled-skin", ar["legs"], 0.015, "cylinder", 0.3, [], 3000, ("leg", round(sh["legs"], 4), sf("legs")), "authored-estimate", "kahl-1963"),
        region("web-feet", ("Webbed feet (both)", "Schwimmfüße (beide)"), "skin", "body", "scaled-skin", ar["feet"], 0.07, "horizontal-plate-down", 0.0, [], 2500, ("foot", round(sh["feet"], 4), sf("feet")), "authored-estimate", "kahl-1963"),
    ]
    # exact sums
    s1 = sum(r["body"]["metabolicShare"] for r in regs)
    regs[0]["body"]["metabolicShare"] = round(regs[0]["body"]["metabolicShare"] + (1 - s1), 4)
    s2 = sum(r["body"]["skinFraction"] for r in regs)
    regs[0]["body"]["skinFraction"] = round(regs[0]["body"]["skinFraction"] + (1 - s2), 5)
    return regs, skin


def pattern(id, title, rate, vt, i, p1, e, p2, ref):
    return {"id": id, "title": tx(*title), "rateBpm": rate, "tidalVolumeL": vt, "inhaleFraction": i, "pauseAfterInhaleFraction": p1,
            "exhaleFraction": e, "pauseAfterExhaleFraction": p2, "route": "nose", "source": "authored-estimate", "reference": ref}
