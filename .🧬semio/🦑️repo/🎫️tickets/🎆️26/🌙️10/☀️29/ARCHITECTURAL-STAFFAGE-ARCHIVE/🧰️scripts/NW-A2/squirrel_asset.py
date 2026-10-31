"""red-squirrel: rig, poses (limb IK), clips, thermal"""
import math
from nw import *
from bird_common import pattern
from mammal_common import *

ID = "red-squirrel"
MASS = 0.30
TRUNK_X = 0.075

REFS = [
    ("gurnell-1987", "Gurnell, J. (1987). The Natural History of Squirrels. Christopher Helm, London. Red squirrel Sciurus vulgaris: head and body 19-24 cm, tail 14-20 cm, mass about 0.25-0.40 kg.", None),
    ("kleiber-1961", "Kleiber, M. (1961). The Fire of Life: An Introduction to Animal Energetics. Wiley, New York. Basal metabolic rate about 70 kcal per day per kg^0.75 = 3.39 W per kg^0.75; Kleiber (1932), Hilgardia 6(11), 315-353.", None),
    ("scholander-1950", "Scholander, P. F., Walters, V., Hock, R., Irving, L. (1950). Body insulation of some arctic and tropical mammals and birds. Biological Bulletin 99, 225-236. Fur insulation grows with coat thickness; the numbers here are authored estimates guided by it.", None),
    ("shorten-1954", "Shorten, M. (1954). Squirrels. Collins New Naturalist, London. Gaits, bounding and climbing of the red squirrel.", None),
    ("hildebrand-1985", "Hildebrand, M. (1985). Walking and running. In: Hildebrand, M., Bramble, D. M., Liem, K. F., Wake, D. B. (eds), Functional Vertebrate Morphology, Harvard University Press, pp. 38-57. Bounding gait of small mammals.", None),
    ("meeh-1879", "Meeh, K. (1879). Oberflaechenmessungen des menschlichen Koerpers. Zeitschrift fuer Biologie 15, 425-458. Skin area about k M^(2/3) with k near 9-10 for small mammals (cm2, g).", None),
    ("gupta-2010", "Gupta, J. K., Lin, C.-H., Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39. Nostril areas and jet angles of the squirrel are authored estimates guided by the human case.", "https://research.polyu.edu.hk/en/publications/characterizing-exhaled-airflow-from-breathing-and-talking/"),
    ("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort: emissivity of skin and respiratory heat loss.", None),
    ("calder-1984", "Calder, W. A. (1984). Size, Function, and Life History. Harvard University Press. Allometric breathing frequency of mammals about 53.5 M^-0.26 breaths per minute (M in kg) and tidal volume about 7.7 mL per kg; the squirrel values are authored estimates guided by these.", None),
    ("schmidt-nielsen-1970", "Schmidt-Nielsen, K., Hainsworth, F. R., Murrish, D. E. (1970). Counter-current heat exchange in the respiratory passages. Respiration Physiology 9, 263-276.", None),
]

MATERIALS = [
    material("fur-red", ("Red-brown summer fur", "Rotbraunes Sommerfell"), 0.95, 0.7, 150, 0.04, None, "authored-estimate", "scholander-1950"),
    material("fur-white", ("White belly fur", "Weißes Bauchfell"), 0.95, 0.35, 150, 0.04, None, "authored-estimate", "scholander-1950"),
    material("tail-fur", ("Bushy tail fur", "Buschiges Schwanzfell"), 0.95, 0.7, 120, 0.04, None, "authored-estimate", "scholander-1950"),
    material("bare-skin", ("Bare skin of paws and nose", "Nackte Haut von Pfoten und Nase"), 0.96, 0.7, None, None, None, "literature", "ashrae-2021"),
]


def rig():
    B = bone
    bones = [
        B("spine-r", None, 0.0, 0.0, 0.0, 0.10, (-90, 120), ("Pelvis and lumbar spine", "Becken und Lendenwirbelsäule"), kg=0.0, com=(0.05, 0.0), dist="point"),
        B("spine-f", "spine-r", 0.10, 0.0, 0.0, 0.10, (-90, 90), ("Thoracic spine with chest", "Brustwirbelsäule mit Brustkorb"), kg=0.075, com=(0.05, 0.0), dist="point"),
        B("neck", "spine-f", 0.10, 0.005, 15.0, 0.035, (-80, 80), ("Neck", "Hals"), kg=0.012),
        B("head", "neck", 0.035, 0.0, -25.0, 0.065, (-90, 90), ("Head with snout and ears", "Kopf mit Schnauze und Ohren"), kg=0.035, com=(0.03, 0.0), dist="point"),
        B("tail-1", "spine-r", -0.01, 0.02, 160.0, 0.09, (-140, 140), ("Tail, base", "Schwanz, Basis"), kg=0.02),
        B("tail-2", "tail-1", 0.09, 0.0, 0.0, 0.10, (-150, 150), ("Tail, tip", "Schwanz, Spitze"), kg=0.02),
    ]
    for s, nm, tg in (("n", "near", "Nahe"), ("f", "far", "Ferne")):
        bones += [
            B(f"arm-{s}", "spine-f", 0.085, -0.025, -90.0, 0.04, (-170, 170), (f"{nm.capitalize()} upper arm", f"{tg} Oberarm"), kg=0.008),
            B(f"forearm-{s}", f"arm-{s}", 0.04, 0.0, 20.0, 0.04, (-170, 170), (f"{nm.capitalize()} forearm", f"{tg} Unterarm"), kg=0.005),
            B(f"hand-{s}", f"forearm-{s}", 0.04, 0.0, 40.0, 0.03, (-170, 170), (f"{nm.capitalize()} hand with long fingers", f"{tg} Hand mit langen Fingern"), kg=0.003),
            B(f"thigh-{s}", "spine-r", 0.01, -0.03, -60.0, 0.06, (-170, 170), (f"{nm.capitalize()} thigh", f"{tg} Oberschenkel"), kg=0.022),
            B(f"shank-{s}", f"thigh-{s}", 0.06, 0.0, -60.0, 0.07, (-170, 170), (f"{nm.capitalize()} shank", f"{tg} Unterschenkel"), kg=0.012),
            B(f"foot-{s}", f"shank-{s}", 0.07, 0.0, 80.0, 0.055, (-170, 170), (f"{nm.capitalize()} hind foot with toes", f"{tg} Hinterfuß mit Zehen"), kg=0.007),
        ]
    for b in bones:
        if "mass" in b: b["mass"]["reference"] = "gurnell-1987"
    other = sum(b["mass"]["kg"] for b in bones if "mass" in b)
    bones[0]["mass"]["kg"] = round(MASS - other, 4)
    bones[0]["mass"]["source"] = "authored-estimate"
    bones[0]["mass"]["reference"] = "gurnell-1987"
    A = anchor
    anchors = []
    for s, nm, tg in (("n", "near", "nahe"), ("f", "far", "ferne")):
        anchors += [
            A(f"ground-foot-{s}", f"foot-{s}", 0.04, -0.011, "contact", (f"{nm.capitalize()} hind foot on the ground", f"Hinterfuß ({tg}) am Boden"), "contact", 90, 0.045, 0.02, 0.7),
            A(f"ground-hand-{s}", f"hand-{s}", 0.03, -0.008, "contact", (f"{nm.capitalize()} hand on the ground", f"Hand ({tg}) am Boden"), "contact", 90, 0.025, 0.015, 0.7),
            A(f"bark-foot-{s}", f"foot-{s}", 0.052, 0.0, "contact", (f"{nm.capitalize()} hind claws in the bark", f"Hinterkrallen ({tg}) in der Rinde"), "contact", 180, 0.02, 0.015, 1.5),
            A(f"bark-hand-{s}", f"hand-{s}", 0.03, 0.0, "contact", (f"{nm.capitalize()} fore claws in the bark", f"Vorderkrallen ({tg}) in der Rinde"), "contact", 180, 0.015, 0.012, 1.5),
        ]
    anchors += [
        A("nostrils", "head", 0.062, 0.004, "emitter", ("Nostrils at the snout tip", "Nasenlöcher an der Schnauzenspitze"), None),
        A("snout-tip", "head", 0.066, -0.004, "attachment", ("Snout tip (food reference)", "Schnauzenspitze (Futterbezug)"), None),
        A("eye", "head", 0.03, 0.014, "attachment", ("Eye", "Auge"), None),
        A("back-plume", "spine-r", 0.06, 0.07, "emitter", ("Back (plume source)", "Rücken (Fahnenquelle)"), None),
    ]
    return bones, anchors


def regions():
    ar = dict(back=0.022, belly=0.010, head=0.0058, nose=0.0006, ears=0.0016, tail=0.017, arms=0.0042, hands=0.0008, legs=0.0052, feet=0.0016)
    sh = dict(back=0.36, belly=0.14, head=0.09, nose=0.01, ears=0.01, tail=0.04, arms=0.08, hands=0.01, legs=0.2, feet=0.01)
    skin = sum(ar.values())
    tot = sum(sh.values())
    sf = lambda k: round(ar[k]/skin, 5)
    s = lambda k: round(sh[k]/tot, 4)
    F = lambda p, R, th, cap, m: fur_layers(p, R, th, cap, m)
    regs = [
        region("fur-back", ("Dorsal fur (back and flanks)", "Rückenfell (Rücken und Flanken)"), "fur", "body", "fur-red", ar["back"], 0.12, "cylinder", 0.5, F("back", 0.40, 0.014, 500, "fur-red"), 12000, ("trunk", s("back"), sf("back")), "authored-estimate", "scholander-1950"),
        region("fur-belly", ("Ventral fur (chest and belly)", "Bauchfell (Brust und Bauch)"), "fur", "body", "fur-white", ar["belly"], 0.1, "horizontal-plate-down", 0.0, F("belly", 0.28, 0.010, 400, "fur-white"), 10000, ("trunk", s("belly"), sf("belly")), "authored-estimate", "scholander-1950"),
        region("head-fur", ("Head fur and cheeks", "Kopffell und Wangen"), "fur", "body", "fur-red", ar["head"], 0.05, "sphere", 0.5, F("head", 0.18, 0.007, 300, "fur-red"), 8000, ("head", s("head"), sf("head")), "authored-estimate", "scholander-1950"),
        region("nose", ("Nose and lips (bare skin)", "Nase und Lippen (nackte Haut)"), "skin", "body", "bare-skin", ar["nose"], 0.01, "sphere", 0.5, [], 2000, ("head", s("nose"), sf("nose")), "authored-estimate", "ashrae-2021"),
        region("ears", ("Ears with tufts", "Ohren mit Pinseln"), "fur", "body", "fur-red", ar["ears"], 0.02, "vertical-plate", 0.6, F("ears", 0.10, 0.004, 100, "fur-red"), 1500, ("head", s("ears"), sf("ears")), "authored-estimate", "scholander-1950"),
        region("tail-fur", ("Bushy tail", "Buschiger Schwanz"), "fur", "body", "tail-fur", ar["tail"], 0.05, "cylinder", 0.5, F("tail", 0.30, 0.025, 250, "tail-fur"), 4000, ("tail", s("tail"), sf("tail")), "authored-estimate", "scholander-1950"),
        region("arms-fur", ("Fore limb fur", "Fell der Vordergliedmaßen"), "fur", "body", "fur-red", ar["arms"], 0.03, "cylinder", 0.4, F("arms", 0.12, 0.005, 200, "fur-red"), 5000, ("arm", s("arms"), sf("arms")), "authored-estimate", "scholander-1950"),
        region("hands-skin", ("Hands and fingers (bare)", "Hände und Finger (nackt)"), "skin", "body", "bare-skin", ar["hands"], 0.01, "cylinder", 0.3, [], 2000, ("hand", s("hands"), sf("hands")), "authored-estimate", "ashrae-2021"),
        region("legs-fur", ("Hind limb fur", "Fell der Hintergliedmaßen"), "fur", "body", "fur-red", ar["legs"], 0.04, "cylinder", 0.4, F("legs", 0.16, 0.006, 250, "fur-red"), 6000, ("leg", s("legs"), sf("legs")), "authored-estimate", "scholander-1950"),
        region("feet-skin", ("Hind feet and toes (bare)", "Hinterfüße und Zehen (nackt)"), "skin", "body", "bare-skin", ar["feet"], 0.02, "horizontal-plate-down", 0.0, [], 2500, ("foot", s("feet"), sf("feet")), "authored-estimate", "ashrae-2021"),
    ]
    return sumshare(regs), skin


def bind(sun=0.5, ground=None, bark=None, belly_e=1.0, hands_c=None, feet_c=None, tail_e=1.0, back_e=1.0):
    g = lambda c, t: rb(1.0, sun*0.4, (c, t)) if c else rb(1.0, sun*0.4)
    return {"fur-back": rb(back_e, sun), "fur-belly": rb(belly_e, sun*0.1), "head-fur": rb(1.0, sun), "nose": rb(1.0, sun), "ears": rb(1.0, sun),
            "tail-fur": rb(tail_e, sun*0.8), "arms-fur": rb(0.9, sun*0.6), "hands-skin": g(hands_c[0], hands_c[1]) if hands_c else rb(1.0, sun*0.3),
            "legs-fur": rb(0.9, sun*0.6), "feet-skin": g(feet_c[0], feet_c[1]) if feet_c else rb(1.0, sun*0.3)}


def build():
    bones, anchors = rig()
    asset = {"rig": {"kind": "anatomical", "root": "spine-r", "bones": bones}, "anchors": anchors}
    LEN = dict(arm=(0.04, 0.04, 0.03), leg=(0.06, 0.07, 0.055))

    def pose_core(root, w):
        """w: world angles: sr, sf, neck, head, t1, t2"""
        off = {"spine-r": {"angleDeg": w["sr"]}}
        off.update(offsets_from_world(asset, w["sr"], ["spine-f", "neck", "head"], [w["sf"], w["neck"], w["head"]]))
        off.update(offsets_from_world(asset, w["sr"], ["tail-1", "tail-2"], [w["t1"], w["t2"]]))
        return off

    def full_pose(root, w, fore, hind):
        """fore/hind = {side: (tip(x,y), end_deg, bend)}"""
        off = pose_core(root, w)
        rt = {"x": root[0], "y": root[1], "angleDeg": 0}
        for s, (tip, end, bend) in fore.items():
            off.update(limb(asset, off, rt, [f"arm-{s}", f"forearm-{s}", f"hand-{s}"], LEN["arm"], tip, end, bend))
        for s, (tip, end, bend) in hind.items():
            off.update(limb(asset, off, rt, [f"thigh-{s}", f"shank-{s}", f"foot-{s}"], LEN["leg"], tip, end, bend))
        return off, rt

    def anch(off, rt, aid):
        return anchor_world(asset, fk(asset, off, rt), aid)

    P = {}
    # ---------------- sitting and eating (default)
    w = dict(sr=58, sf=86, neck=78, head=-22, t1=118, t2=62)
    root = (0.0, 0.052)
    wr = fk(asset, pose_core(root, w), {"x": root[0], "y": root[1], "angleDeg": 0})
    nose_w = apply(wr["head"], (0.066, -0.004))
    hind = {"n": ((0.075, 0.0), 0, 1), "f": ((0.052, 0.0), 0, 1)}
    # foot flat: contact point of the foot at the target -> tip of the foot bone is contact + (0.015, 0.006)
    hind = {s: ((hx + 0.015, 0.011), 0, 1) for s, hx in (("n", 0.085), ("f", 0.06))}
    fore = {"n": ((nose_w[0] - 0.012, nose_w[1] - 0.018), 70, -1), "f": ((nose_w[0] - 0.024, nose_w[1] - 0.026), 78, -1)}
    off, rt = full_pose(root, w, fore, hind)
    sit_off = off
    gl = lambda aid: anch(off, rt, aid)
    P["sitting-eating"] = pose("sitting-eating", ("Sitting upright, eating", "Aufrecht sitzend beim Fressen"),
        ("Sitting on the haunches with both hind feet flat, the body upright, both hands holding a nut at the mouth, the head bent forward, the tail arched up over the back.",
         "Auf den Hinterbeinen sitzend, beide Hinterfüße flach, Körper aufrecht, beide Hände halten eine Nuss vor dem Maul, Kopf nach vorn geneigt, Schwanz in hohem Bogen über dem Rücken."),
        off, ["ground-foot-n", "ground-foot-f"], bind(0.5, feet_c=(0.4, 18)), {"plume-back": 1.0}, 1.5, "rest", {"x": 0.0})
    # ---------------- running (bound gather)
    def run_pose(rootx, w, fore, hind, contacts, ground="auto", ry=0.07, key=False, id_="running", title=None, met=6.0, breathing="run", emit=0.5):
        off, rt = full_pose((rootx, ry), w, fore, hind)
        pz = pose(id_, tuple(title[0]), tuple(title[1]), off, contacts, bind(0.5, feet_c=(0.3, 18) if any("foot" in c for c in contacts) else None, hands_c=(0.3, 18) if any("hand" in c for c in contacts) else None),
                  {"plume-back": emit}, met, breathing, {"x": rootx} if ground == "auto" else {"x": rootx, "y": ry}, ground=ground, key=key)
        return pz
    run_specs = {
        # id: (w, fore tips, hind tips, contacts)
    }
    # gather: hind feet planted under the body, fore paws reaching
    wg = dict(sr=12, sf=-6, neck=10, head=-12, t1=172, t2=190)
    P["running"] = run_pose(0.0, wg, {"n": ((0.30, 0.10), -25, 1), "f": ((0.27, 0.08), -30, 1)}, {"n": ((0.085, 0.0), 0, 1), "f": ((0.06, 0.0), 0, 1)}, ["ground-foot-n", "ground-foot-f"],
                            id_="running", title=(("Running, hind feet planted", "Laufen, Hinterfüße aufgesetzt"), ("Bounding gait at the gather: both hind feet planted under the pelvis, the back arched, the hands reaching forward, the tail streaming out behind.", "Sprunglauf beim Sammeln: beide Hinterfüße unter dem Becken aufgesetzt, Rücken gewölbt, Hände greifen nach vorn, Schwanz weht nach hinten.")))
    # ---------------- climbing
    wc = dict(sr=82, sf=92, neck=88, head=38, t1=250, t2=268)
    cy = 0.25
    clim_f = {"n": ((TRUNK_X + 0.0, 0.40), 100, -1), "f": ((TRUNK_X + 0.0, 0.34), 100, -1)}
    clim_h = {"n": ((TRUNK_X, 0.17), 85, 1), "f": ((TRUNK_X, 0.12), 85, 1)}
    P["climbing"] = run_pose(0.0, wc, clim_f, clim_h, ["bark-hand-n", "bark-hand-f", "bark-foot-n", "bark-foot-f"], ground="authored", ry=cy, id_="climbing",
                             title=(("Climbing a trunk", "Klettern am Stamm"), ("Head up on a vertical trunk at x = 0.075 m (not part of the asset): claws of both hands and both hind feet hooked in the bark as supports, the body hugging the trunk, the tail hanging down.", "Kopf oben an einem senkrechten Stamm bei x = 0,075 m (nicht Teil des Assets): Krallen beider Hände und beider Hinterfüße in der Rinde als Auflager, der Körper schmiegt sich an den Stamm, der Schwanz hängt herab.")), met=7.0, breathing="run", emit=0.5)
    P["climbing"]["thermal"]["regions"] = bind(0.4, belly_e=0.4, hands_c=(0.9, 14), feet_c=(0.9, 14))
    P["climbing"]["thermal"]["regions"]["fur-belly"] = rb(0.3, 0.0, (0.7, 14))

    # ---------------- key poses of the clips
    nb = json.loads(json.dumps(P["sitting-eating"])); nb["id"] = "nibble-b"; nb["key"] = True
    nb["title"] = tx("Nibbling, head down", "Knabbern, Kopf gesenkt"); nb["description"] = tx("Key pose: the head dips 8 degrees while the hands turn the nut.", "Schlüsselpose: der Kopf senkt sich um 8 Grad, während die Hände die Nuss drehen.")
    nb["bones"]["head"] = {"angleDeg": nb["bones"]["head"]["angleDeg"] - 8}
    nb["bones"]["hand-n"] = {"angleDeg": nb["bones"]["hand-n"]["angleDeg"] + 14}; nb["bones"]["hand-f"] = {"angleDeg": nb["bones"]["hand-f"]["angleDeg"] - 12}
    P["nibble-b"] = nb
    H = 0.07
    ext = dict(sr=6, sf=-12, neck=4, head=-14, t1=176, t2=196)
    P["bound-extension"] = run_pose(0.0, ext, {"n": ((0.36, 0.0), -10, 1), "f": ((0.33, 0.0), -10, 1)}, {"n": ((-0.12, 0.10), -30, 1), "f": ((-0.15, 0.08), -30, 1)}, ["ground-hand-n", "ground-hand-f"], ry=0.14, id_="bound-extension", key=True,
        title=(("Bound: extension, hands land", "Sprung: Streckung, Hände landen"), ("Key pose: body stretched, both hands touch down ahead, hind feet trail off the ground.", "Schlüsselpose: Körper gestreckt, beide Hände landen vorn, die Hinterfüße schleppen über dem Boden.")))
    sw = dict(sr=24, sf=14, neck=18, head=-8, t1=170, t2=200)
    P["bound-swing"] = run_pose(0.12, sw, {"n": ((0.40, 0.10), -20, 1), "f": ((0.37, 0.08), -20, 1)}, {"n": ((0.20, 0.12), 20, 1), "f": ((0.17, 0.10), 20, 1)}, [], ry=0.17, ground="authored", id_="bound-swing", key=True,
        title=(("Bound: hind legs swing through", "Sprung: Hinterbeine schwingen durch"), ("Key pose in the air: the back arches, the hind legs swing forward past the hands.", "Schlüsselpose in der Luft: der Rücken wölbt sich, die Hinterbeine schwingen nach vorn an den Händen vorbei.")))
    P["bound-gather"] = run_pose(0.0, wg, {"n": ((0.30, 0.10), -25, 1), "f": ((0.27, 0.08), -30, 1)}, {"n": ((0.085, 0.0), 0, 1), "f": ((0.06, 0.0), 0, 1)}, ["ground-foot-n", "ground-foot-f"], id_="bound-gather", key=True,
        title=(("Bound: gather", "Sprung: Sammeln"), ("Key pose: hind feet planted under the pelvis.", "Schlüsselpose: Hinterfüße unter dem Becken aufgesetzt.")))
    ps = dict(sr=2, sf=-8, neck=2, head=-10, t1=178, t2=198)
    P["bound-push"] = run_pose(0.0, ps, {"n": ((0.40, 0.12), -20, 1), "f": ((0.37, 0.10), -20, 1)}, {"n": ((-0.02, 0.0), 0, 1), "f": ((-0.05, 0.0), 0, 1)}, ["ground-foot-n", "ground-foot-f"], ry=0.10, id_="bound-push", key=True,
        title=(("Bound: push-off", "Sprung: Abdruck"), ("Key pose: the hind legs extend and push the body forward, the hands reach ahead.", "Schlüsselpose: die Hinterbeine strecken sich und schieben den Körper nach vorn, die Hände greifen voraus.")))
    P["bound-flight"] = run_pose(0.3, ext, {"n": ((0.40, 0.14), -20, 1), "f": ((0.37, 0.12), -20, 1)}, {"n": ((-0.14, 0.14), -30, 1), "f": ((-0.17, 0.12), -30, 1)}, [], ry=0.16, ground="authored", id_="bound-flight", key=True,
        title=(("Bound: flight", "Sprung: Flugphase"), ("Key pose in the air: the body fully stretched, nothing touches the ground.", "Schlüsselpose in der Luft: der Körper ist ganz gestreckt, nichts berührt den Boden.")))
    for i, (dy, ry_) in enumerate(((0.0, 0.25), (0.06, 0.30), (0.12, 0.36))):
        cf = {"n": ((TRUNK_X, 0.40 + dy), 100, -1), "f": ((TRUNK_X, 0.34 + dy), 100, -1)}
        ch = {"n": ((TRUNK_X, 0.17 + dy + (0.05 if i == 1 else 0)), 85, 1), "f": ((TRUNK_X, 0.12 + dy + (0.05 if i == 1 else 0)), 85, 1)}
        q = run_pose(0.0, wc, cf, ch, ["bark-hand-n", "bark-hand-f", "bark-foot-n", "bark-foot-f"], ground="authored", ry=ry_, id_=f"climb-{'abc'[i]}", key=True,
            title=((f"Climb phase {'abc'[i]}", f"Kletterphase {'abc'[i]}"), ("Key pose of the climb, one hand-over-hand step further up the trunk.", "Schlüsselpose des Kletterns, einen Griffwechsel weiter den Stamm hinauf.")), met=7.0)
        q["thermal"]["regions"] = P["climbing"]["thermal"]["regions"]
        P[f"climb-{'abc'[i]}"] = q
    asset["_P"] = P
    asset["_helpers"] = (full_pose, run_pose, pose_core, anch, w, sit_off)
    asset["poses"] = list(P.values())
    return asset


def thermal():
    regs, skin = regions()
    pats = [pattern("rest", ("Resting, nose breathing", "Ruhe, Nasenatmung"), 60, 0.0024, 0.4, 0.04, 0.45, 0.11, "calder-1984"),
            pattern("eat", ("Feeding, gnawing with short breath", "Fressen, Nagen mit kurzen Atemzügen"), 90, 0.0020, 0.42, 0.03, 0.45, 0.10, "calder-1984"),
            pattern("run", ("Bounding and climbing", "Sprunglauf und Klettern"), 200, 0.0035, 0.5, 0.0, 0.5, 0.0, "calder-1984")]
    for p in pats: pass
    for p in pats: p["route"] = "nose"
    return {
        "environment": {"airTempC": 12, "meanRadiantTempC": 12, "skyTempC": 4, "groundTempC": 11, "relativeHumidity": 0.7, "windMS": 0.8, "windDirectionDeg": 180, "solarWM2": 250, "diffuseFraction": 0.35, "pressurePa": 101325},
        "baseMet": 1.5,
        "body": {"species": "squirrel", "massKg": MASS, "skinAreaM2": round(skin, 4), "basalMetabolismW": 1.4, "metWM2": round(1.4/skin, 1), "coreSetPointC": 38.5, "skinSetPointC": 35.0, "sweating": False, "panting": False,
                 "maxSkinBloodFlowLM2H": 40, "minSkinBloodFlowLM2H": 3, "source": "authored-estimate", "reference": "kleiber-1961"},
        "respiration": {"noseEmitter": "breath-nose", "exhaledTempC": 33, "exhaledRelativeHumidity": 0.95, "patterns": pats, "motion": [{"bone": "spine-f", "angleDeg": 1.5, "dy": 0.0008}], "source": "authored-estimate", "reference": "schmidt-nielsen-1970"},
        "passengers": [], "operating": [], "regions": regs,
        "emitters": [
            {"id": "breath-nose", "kind": "breath", "title": tx("Both nostrils at the snout tip (combined)", "Beide Nasenlöcher an der Schnauzenspitze (zusammen)"), "anchor": "nostrils", "directionDeg": -20, "diameterM": 0.003, "source": "authored-estimate", "reference": "gupta-2010"},
            {"id": "plume-back", "kind": "plume", "title": tx("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.07, "region": "fur-back", "source": "authored-estimate", "reference": "scholander-1950"}],
        "assumptions": [
            tx("Basal metabolism 1.4 W is Kleiber's 3.39 W per kg^0.75 (0.30 kg gives 1.37 W); the 38.5 degC core temperature and the fur resistances (0.40 m2K/W dorsal, 0.28 ventral, tail 0.30) are authored estimates guided by Scholander et al. (1950).",
               "Der Grundumsatz von 1,4 W ist Kleibers 3,39 W pro kg^0,75 (0,30 kg ergeben 1,37 W); die Kerntemperatur von 38,5 Grad und die Fellwiderstände (0,40 m2K/W Rücken, 0,28 Bauch, Schwanz 0,30) sind Schätzwerte nach Scholander et al. (1950)."),
            tx("The squirrel's fur is the summer coat; the winter coat with ear tufts has about 1.5 times the insulation and is not modelled.", "Das Fell ist das Sommerfell; das Winterfell mit Ohrpinseln isoliert etwa 1,5-mal so gut und ist nicht modelliert."),
            tx("Bark contacts are claw supports with a friction coefficient of 1.5 (authored estimate: claws interlock with the rough bark) and the normal pointing away from the trunk; the trunk itself is not part of the asset.",
               "Rindenkontakte sind Krallenauflager mit Reibungszahl 1,5 (Schätzwert: Krallen verzahnen sich mit der rauen Rinde) und der Normalen vom Stamm weg; der Stamm selbst ist nicht Teil des Assets."),
            tx("Breathing is nasal; the exhaled air leaves at 33 degC and 95 % relative humidity.", "Die Atmung erfolgt über die Nase; die Ausatemluft verlässt sie mit 33 Grad und 95 % relativer Feuchte.")]}


if __name__ == "__main__":
    import sys
    a = build()
    a["clips"] = []; a["thermal"] = {}; a["materials"] = MATERIALS
    print("limits", check_limits(a))
    for pid in sys.argv[1:]:
        view = (-0.15, -0.05, 0.4, 0.5) if pid != "climbing" else (-0.2, 0.0, 0.2, 0.55)
        skel(a, pid, f"{GEN}/skel-squirrel-{pid}.png", view=view, den=2, px_per_mm=12)


def clips():
    nib = {"id": "nibble", "title": tx("Nibbling a nut", "Nuss knabbern"), "description": tx(
        "Sitting upright and gnawing a nut for 2 s: the head dips 8 degrees at 1 Hz while the hands turn the nut; the hind feet stay planted; breathing is the short feeding pattern.",
        "Aufrecht sitzend eine Nuss knabbern, 2 s: Der Kopf senkt sich mit 1 Hz um 8 Grad, während die Hände die Nuss drehen; die Hinterfüße bleiben stehen; die Atmung ist das kurze Fressmuster."),
        "durationS": 2.0, "fps": 30, "loop": True, "rootMotion": "none",
        "keys": [{"t": 0.0, "pose": "sitting-eating", "ease": "smooth"}, {"t": 0.5, "pose": "nibble-b", "ease": "smooth"}, {"t": 1.0, "pose": "sitting-eating", "ease": "smooth"}, {"t": 1.5, "pose": "nibble-b", "ease": "smooth"}, {"t": 2.0, "pose": "sitting-eating", "ease": "smooth"}],
        "contacts": [{"anchor": "ground-foot-n", "fromS": 0, "toS": 2.0}, {"anchor": "ground-foot-f", "fromS": 0, "toS": 2.0}], "phases": [{"id": "gnaw", "fromS": 0, "toS": 2.0, "title": tx("Gnawing", "Nagen")}],
        "events": [{"t": 0.5, "kind": "motion", "title": tx("Bite", "Biss")}, {"t": 1.5, "kind": "motion", "title": tx("Bite", "Biss")}],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.8}, {"t": 1.0, "v": 2.2}, {"t": 2.0, "v": 1.8}]}]}
    bd = 0.36
    bound = {"id": "bound", "title": tx("Bounding run", "Sprunglauf"), "description": tx(
        "A bounding run of 0.36 s per cycle (about 2.8 bounds per second): the hands land, the hind legs swing through, the hind feet plant under the pelvis and push off; two airborne phases per cycle, the tail streams behind.",
        "Ein Sprunglauf mit 0,36 s je Zyklus (etwa 2,8 Sprünge pro Sekunde): Die Hände landen, die Hinterbeine schwingen durch, die Hinterfüße setzen unter dem Becken auf und drücken ab; zwei Flugphasen je Zyklus, der Schwanz weht hinterher."),
        "durationS": bd, "fps": 60, "loop": True, "rootMotion": "plant", "advanceM": 0.5,
        "keys": [{"t": 0.0, "pose": "bound-extension", "ease": "smooth"}, {"t": 0.09, "pose": "bound-swing", "ease": "smooth"}, {"t": 0.14, "pose": "bound-gather", "ease": "smooth"}, {"t": 0.24, "pose": "bound-push", "ease": "smooth"}, {"t": 0.30, "pose": "bound-flight", "ease": "smooth"}],
        "contacts": [{"anchor": "ground-hand-n", "fromS": 0, "toS": 0.09}, {"anchor": "ground-hand-f", "fromS": 0, "toS": 0.09}, {"anchor": "ground-foot-n", "fromS": 0.14, "toS": 0.26}, {"anchor": "ground-foot-f", "fromS": 0.14, "toS": 0.26}],
        "phases": [{"id": "fore-stance", "fromS": 0, "toS": 0.09, "title": tx("Hands on the ground", "Hände am Boden")}, {"id": "air-a", "fromS": 0.09, "toS": 0.14, "title": tx("Airborne, hind legs swing", "Flugphase, Hinterbeine schwingen"), "ballistic": True},
                   {"id": "hind-stance", "fromS": 0.14, "toS": 0.26, "title": tx("Hind feet on the ground", "Hinterfüße am Boden")}, {"id": "air-b", "fromS": 0.26, "toS": bd, "title": tx("Airborne, stretched", "Flugphase, gestreckt"), "ballistic": True}],
        "events": [{"t": 0.0, "kind": "contact-on", "title": tx("Hands touch down", "Hände setzen auf"), "target": "ground-hand-n"}, {"t": 0.09, "kind": "contact-off", "title": tx("Hands lift", "Hände heben ab"), "target": "ground-hand-n"},
                   {"t": 0.14, "kind": "contact-on", "title": tx("Hind feet touch down", "Hinterfüße setzen auf"), "target": "ground-foot-n"}, {"t": 0.26, "kind": "contact-off", "title": tx("Push-off", "Abdruck"), "target": "ground-foot-n"}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 6.0}, {"t": 0.24, "v": 8.0}, {"t": bd, "v": 6.0}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}]}]}
    cd = 2.4
    climb = {"id": "climb", "title": tx("Climbing a trunk", "Stamm hinaufklettern"), "description": tx(
        "Three grip changes up a vertical trunk in 2.4 s (about 0.12 m): the claws of the hands and the hind feet are hooked in the bark as supports, never fewer than two grips at a time, the body rises 0.11 m.",
        "Drei Griffwechsel an einem senkrechten Stamm in 2,4 s (etwa 0,12 m): Die Krallen der Hände und Hinterfüße haken sich als Auflager in die Rinde, nie weniger als zwei Griffe zugleich, der Körper steigt um 0,11 m."),
        "durationS": cd, "fps": 30, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0.0, "pose": "climb-a", "ease": "smooth"}, {"t": 1.2, "pose": "climb-b", "ease": "smooth"}, {"t": 2.4, "pose": "climb-c", "ease": "smooth"}],
        "contacts": [{"anchor": "bark-foot-n", "fromS": 0, "toS": cd}, {"anchor": "bark-foot-f", "fromS": 0, "toS": cd}, {"anchor": "bark-hand-n", "fromS": 0, "toS": 1.4}, {"anchor": "bark-hand-f", "fromS": 0, "toS": 1.0}, {"anchor": "bark-hand-n", "fromS": 1.6, "toS": cd}, {"anchor": "bark-hand-f", "fromS": 1.3, "toS": cd}],
        "phases": [{"id": "climb", "fromS": 0, "toS": cd, "title": tx("Climbing", "Klettern")}],
        "events": [{"t": 1.0, "kind": "contact-off", "title": tx("Far hand releases", "Ferne Hand löst sich"), "target": "bark-hand-f"}, {"t": 1.4, "kind": "contact-off", "title": tx("Near hand releases", "Nahe Hand löst sich"), "target": "bark-hand-n"}],
        "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 7.0}, {"t": cd, "v": 7.0}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}]}]}
    return [nib, bound, climb]


def base():
    a = build()
    for k in ("_P", "_helpers"): a.pop(k)
    a["clips"] = clips()
    a["thermal"] = thermal()
    a["materials"] = MATERIALS
    return a
