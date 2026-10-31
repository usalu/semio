"""hedgehog (Erinaceus europaeus): rig with spine skirts that roll the animal into a ball; poses, clips, thermal"""
import math
from nw import *
from bird_common import pattern
from mammal_common import *

ID = "hedgehog"
MASS = 1.0

REFS = [
    ("reeve-1994", "Reeve, N. J. (1994). Hedgehogs. T & AD Poyser, London. Erinaceus europaeus: head and body 20-27 cm, adult mass about 0.6-1.2 kg, 5000-7000 spines of 2-3 cm, rolling up by the panniculus carnosus.", None),
    ("kleiber-1961", "Kleiber, M. (1961). The Fire of Life: An Introduction to Animal Energetics. Wiley, New York. Basal metabolic rate 70 kcal/day per kg^0.75 = 3.39 W per kg^0.75.", None),
    ("mcnab-1980", "McNab, B. K. (1980). Food habits, energetics, and the population biology of mammals. The American Naturalist 116, 106-124. Insectivores with spines have a basal rate well below the Kleiber line; the factor 0.65 used here is an authored estimate guided by this paper.", None),
    ("scholander-1950", "Scholander, P. F., Walters, V., Hock, R., Irving, L. (1950). Body insulation of some arctic and tropical mammals and birds. Biological Bulletin 99, 225-236.", None),
    ("calder-1984", "Calder, W. A. (1984). Size, Function, and Life History. Harvard University Press. Allometric breathing frequency of mammals about 53.5 M^-0.26 breaths per minute (M in kg) and tidal volume about 7.7 mL per kg; the hedgehog patterns are authored estimates guided by these.", None),
    ("meeh-1879", "Meeh, K. (1879). Oberflaechenmessungen des menschlichen Koerpers. Zeitschrift fuer Biologie 15, 425-458. Skin area about k M^(2/3) with k near 10 (cm2, g).", None),
    ("gupta-2010", "Gupta, J. K., Lin, C.-H., Chen, Q. (2010). Characterizing exhaled airflow from breathing and talking. Indoor Air 20(1), 31-39. Nostril areas and jet angles of the hedgehog are authored estimates guided by the human case.", "https://research.polyu.edu.hk/en/publications/characterizing-exhaled-airflow-from-breathing-and-talking/"),
    ("ashrae-2021", "ASHRAE Handbook Fundamentals (2021), Chapter 9 Thermal Comfort: emissivity of skin and respiratory heat loss.", None),
    ("schmidt-nielsen-1970", "Schmidt-Nielsen, K., Hainsworth, F. R., Murrish, D. E. (1970). Counter-current heat exchange in the respiratory passages. Respiration Physiology 9, 263-276.", None),
]

MATERIALS = [
    material("spines", ("Keratin spines with enclosed air", "Keratinstacheln mit eingeschlossener Luft"), 0.95, 0.75, 1200, 0.05, None, "authored-estimate", "reeve-1994"),
    material("belly-fur", ("Coarse belly fur", "Grobes Bauchfell"), 0.95, 0.5, 150, 0.04, None, "authored-estimate", "scholander-1950"),
    material("face-fur", ("Short face fur", "Kurzes Gesichtsfell"), 0.95, 0.6, 150, 0.04, None, "authored-estimate", "scholander-1950"),
    material("bare-skin", ("Bare skin of nose and paws", "Nackte Haut von Nase und Pfoten"), 0.96, 0.7, None, None, None, "literature", "ashrae-2021"),
]


def rig():
    B = bone
    bones = [
        B("trunk", None, 0.0, 0.0, 0.0, 0.16, (-60, 60), ("Trunk under the spiny dome", "Rumpf unter der Stachelkuppel"), kg=0.0, com=(0.07, 0.0), dist="point"),
        B("coat-r", "trunk", -0.02, 0.0, 250.0, 0.07, (-130, 130), ("Rear spine skirt (panniculus flap)", "Hinterer Stachelsaum (Hautmuskel-Lappen)"), kg=0.04),
        B("coat-f", "trunk", 0.14, 0.0, -70.0, 0.07, (-130, 130), ("Front spine skirt over the forehead", "Vorderer Stachelsaum über der Stirn"), kg=0.04),
        B("neck", "trunk", 0.15, -0.005, -10.0, 0.02, (-100, 100), ("Neck", "Hals"), kg=0.02),
        B("head", "neck", 0.02, 0.0, -10.0, 0.055, (-100, 100), ("Head with pointed snout", "Kopf mit spitzer Schnauze"), kg=0.08, com=(0.03, 0.0), dist="point"),
    ]
    for s, nm, tg in (("n", "near", "nah"), ("f", "far", "fern")):
        bones += [
            B(f"fore-{s}", "trunk", 0.11, -0.03, -90.0, 0.03, (-150, 150), (f"{nm.capitalize()} fore limb", f"Vorderbein ({tg})"), kg=0.03),
            B(f"forepaw-{s}", f"fore-{s}", 0.03, 0.0, 0.0, 0.035, (-150, 150), (f"{nm.capitalize()} fore paw", f"Vorderpfote ({tg})"), kg=0.01),
            B(f"hind-{s}", "trunk", 0.02, -0.03, -90.0, 0.035, (-150, 150), (f"{nm.capitalize()} hind limb", f"Hinterbein ({tg})"), kg=0.035),
            B(f"hindpaw-{s}", f"hind-{s}", 0.035, 0.0, 0.0, 0.04, (-150, 150), (f"{nm.capitalize()} hind foot", f"Hinterfuß ({tg})"), kg=0.012),
        ]
    for b in bones:
        if "mass" in b: b["mass"]["reference"] = "reeve-1994"
    other = sum(b["mass"]["kg"] for b in bones if "mass" in b)
    bones[0]["mass"]["kg"] = round(MASS - other, 4)
    bones[0]["mass"]["source"] = "authored-estimate"; bones[0]["mass"]["reference"] = "reeve-1994"
    A = anchor
    anchors = []
    for p, bn, ln, nm, tg in (("fn", "forepaw-n", 0.035, "Near fore paw", "Vorderpfote nah"), ("ff", "forepaw-f", 0.035, "Far fore paw", "Vorderpfote fern"), ("hn", "hindpaw-n", 0.04, "Near hind foot", "Hinterfuß nah"), ("hf", "hindpaw-f", 0.04, "Far hind foot", "Hinterfuß fern")):
        anchors.append(A(f"foot-{p}", bn, ln, -0.0105, "contact", (f"{nm} on the ground", f"{tg} am Boden"), "contact", 90, 0.03, 0.02, 0.8))
    anchors += [
        A("spine-front", "coat-f", 0.07, 0.0, "contact", ("Front spine tips on the ground (rolled up)", "Vordere Stachelspitzen am Boden (eingerollt)"), "contact", 90, 0.04, 0.05, 0.6),
        A("spine-rear", "coat-r", 0.07, 0.0, "contact", ("Rear spine tips on the ground (rolled up)", "Hintere Stachelspitzen am Boden (eingerollt)"), "contact", 90, 0.04, 0.05, 0.6),
        A("nostrils", "head", 0.054, -0.004, "emitter", ("Nostrils at the snout tip", "Nasenlöcher an der Schnauzenspitze"), None),
        A("snout-tip", "head", 0.056, -0.006, "attachment", ("Snout tip", "Schnauzenspitze"), None),
        A("eye", "head", 0.028, 0.012, "attachment", ("Eye", "Auge"), None),
        A("back-plume", "trunk", 0.06, 0.07, "emitter", ("Back above the dome (plume source)", "Rücken über der Kuppel (Fahnenquelle)"), None),
    ]
    return bones, anchors


def regions():
    ar = dict(dome=0.040, skirt=0.020, belly=0.020, face=0.006, nose=0.0008, limbs=0.0095, paws=0.0035)
    sh = dict(dome=0.30, skirt=0.04, belly=0.30, face=0.12, nose=0.01, limbs=0.17, paws=0.06)
    skin = sum(ar.values())
    tot = sum(sh.values())
    sf = lambda k: round(ar[k]/skin, 5)
    s = lambda k: round(sh[k]/tot, 4)
    vr = lambda r: round(r / (0.0165 * 0.5), 2)
    SP = lambda p, R, th, cap: [layer(p + "-air", ("Air among the spine bases", "Luft zwischen den Stachelfüßen"), "air-gap", "spines", round(th*0.6, 4), round(R*0.6, 3), vr(R*0.6), round(cap*0.3), "authored-estimate", "reeve-1994"),
                                layer(p + "-spines", ("Spine layer (2-3 cm spines, still air)", "Stachelschicht (2-3 cm Stacheln, ruhende Luft)"), "hair", "spines", round(th*0.4, 4), round(R*0.4, 3), vr(R*0.4), round(cap*0.7), "authored-estimate", "reeve-1994")]
    regs = [
        region("spines-dome", ("Spiny dome (back and flanks)", "Stachelkuppel (Rücken und Flanken)"), "hair", "body", "spines", ar["dome"], 0.12, "sphere", 0.5, SP("dome", 0.30, 0.025, 600), 12000, ("trunk", s("dome"), sf("dome")), "authored-estimate", "reeve-1994"),
        region("spines-skirt", ("Spine skirts (front and rear)", "Stachelsäume (vorn und hinten)"), "hair", "body", "spines", ar["skirt"], 0.07, "cylinder", 0.5, SP("skirt", 0.25, 0.02, 500), 8000, ("trunk", s("skirt"), sf("skirt")), "authored-estimate", "reeve-1994"),
        region("belly-fur", ("Belly fur", "Bauchfell"), "fur", "body", "belly-fur", ar["belly"], 0.1, "horizontal-plate-down", 0.0, fur_layers("belly", 0.15, 0.012, 400, "belly-fur"), 10000, ("trunk", s("belly"), sf("belly")), "authored-estimate", "scholander-1950"),
        region("face-fur", ("Face and cheeks (short fur)", "Gesicht und Wangen (kurzes Fell)"), "fur", "body", "face-fur", ar["face"], 0.045, "sphere", 0.5, fur_layers("face", 0.10, 0.006, 250, "face-fur"), 8000, ("head", s("face"), sf("face")), "authored-estimate", "scholander-1950"),
        region("nose", ("Nose and snout tip (bare)", "Nase und Schnauzenspitze (nackt)"), "skin", "body", "bare-skin", ar["nose"], 0.01, "sphere", 0.5, [], 2000, ("head", s("nose"), sf("nose")), "authored-estimate", "ashrae-2021"),
        region("limbs-fur", ("Limbs (short coarse hair)", "Gliedmaßen (kurzes grobes Haar)"), "fur", "body", "belly-fur", ar["limbs"], 0.025, "cylinder", 0.3, fur_layers("limbs", 0.08, 0.005, 200, "belly-fur"), 5000, ("leg", s("limbs"), sf("limbs")), "authored-estimate", "scholander-1950"),
        region("paws-skin", ("Paws and claws (bare)", "Pfoten und Krallen (nackt)"), "skin", "body", "bare-skin", ar["paws"], 0.02, "horizontal-plate-down", 0.0, [], 2500, ("foot", s("paws"), sf("paws")), "authored-estimate", "ashrae-2021"),
    ]
    return sumshare(regs), skin


def bind(sun=0.4, rolled=False, ground=12):
    if not rolled:
        return {"spines-dome": rb(1.0, sun), "spines-skirt": rb(1.0, sun*0.8), "belly-fur": rb(1.0, 0.0, (0.1, ground)), "face-fur": rb(1.0, sun), "nose": rb(1.0, sun),
                "limbs-fur": rb(1.0, sun*0.3), "paws-skin": rb(1.0, sun*0.2, (0.45, ground))}
    return {"spines-dome": rb(1.0, sun), "spines-skirt": rb(1.0, sun*0.7, (0.25, ground)), "belly-fur": rb(0.03, 0.0), "face-fur": rb(0.03, 0.0), "nose": rb(0.0, 0.0),
            "limbs-fur": rb(0.05, 0.0), "paws-skin": rb(0.05, 0.0)}


def two(asset, off, root, names, lens, target, bend):
    W = fk(asset, off, root)
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    parent_deg = world_angle(W[bones[names[0]]["parent"]])
    origin = (W[names[0]][4], W[names[0]][5])
    t1, t2 = two_link(origin, target, lens[0], lens[1], bend)
    return offsets_from_world(asset, parent_deg, names, [t1, t2])


def build():
    bones, anchors = rig()
    asset = {"rig": {"kind": "anatomical", "root": "trunk", "bones": bones}, "anchors": anchors}
    FL, HL = (0.03, 0.035), (0.035, 0.04)

    def core(w):
        off = {"trunk": {"angleDeg": w["pitch"]}}
        off.update(offsets_from_world(asset, w["pitch"], ["neck", "head"], [w["neck"], w["head"]]))
        off["coat-f"] = {"angleDeg": round(norm_deg(w["cf"] - w["pitch"] + 70), 2)}
        off["coat-r"] = {"angleDeg": round(norm_deg(w["cr"] - w["pitch"] - 250), 2)}
        return off

    def stance(w, ry, feet, rx=0.0):
        """feet: dict name -> target x (world), ground y = 0.004; names fn ff hn hf"""
        off = core(w)
        rt = {"x": rx, "y": ry, "angleDeg": 0}
        for p, x in feet.items():
            side = "n" if p[1] == "n" else "f"
            fore = p[0] == "f"
            names = [f"fore-{side}", f"forepaw-{side}"] if fore else [f"hind-{side}", f"hindpaw-{side}"]
            off.update(two(asset, off, rt, names, FL if fore else HL, (x, 0.0105), -1 if fore else 1))
        return off, rt

    def mk(id_, title, desc, w, ry, feet, contacts, rolled=False, key=False, met=2.0, br="forage", emit=1.0, ground="auto", rx=0.0, tuck=None):
        off, rt = stance(w, ry, feet, rx) if feet is not None else (core(w), {"x": rx, "y": ry, "angleDeg": 0})
        if tuck:
            for k, v in tuck.items(): off[k] = {"angleDeg": v}
        return pose(id_, title, desc, off, contacts, bind(rolled=rolled), {"plume-back": emit}, met, br, {"x": rx} if ground == "auto" else {"x": rx, "y": ry}, ground=ground, key=key)

    P = {}
    wf = dict(pitch=0, neck=-28, head=-42, cf=-70, cr=250)
    hx = 0.0
    feet0 = {"fn": 0.13, "ff": 0.09, "hn": 0.05, "hf": 0.01}
    P["foraging"] = mk("foraging", ("Foraging, snout down", "Auf Nahrungssuche, Schnauze unten"),
        ("Standing on all four feet with the head lowered and the snout near the ground, the front spine skirt hanging over the forehead, the back dome relaxed.", "Auf allen vier Füßen, Kopf gesenkt und Schnauze nah am Boden, der vordere Stachelsaum hängt über die Stirn, die Rückenkuppel ist entspannt."),
        wf, 0.07, feet0, ["foot-fn", "foot-ff", "foot-hn", "foot-hf"], met=2.0, br="forage")
    wa = dict(pitch=4, neck=8, head=-2, cf=-60, cr=245)
    P["alert"] = mk("alert", ("Alert, sniffing", "Wachsam, witternd"),
        ("Standing still with the head raised and the snout level, sniffing the air; the spine skirts a little raised.", "Ruhig stehend mit erhobenem Kopf und waagerechter Schnauze, die Luft witternd; die Stachelsäume leicht angehoben."),
        wa, 0.072, {"fn": 0.13, "ff": 0.09, "hn": 0.05, "hf": 0.01}, ["foot-fn", "foot-ff", "foot-hn", "foot-hf"], met=1.4, br="rest", emit=1.0)
    wr = dict(pitch=0, neck=-100, head=-170, cf=-150, cr=-30)
    off_r = core(wr)
    off_r.update({"fore-n": {"angleDeg": -100}, "forepaw-n": {"angleDeg": -20}, "fore-f": {"angleDeg": -95}, "forepaw-f": {"angleDeg": -25}, "hind-n": {"angleDeg": 150}, "hindpaw-n": {"angleDeg": 0}, "hind-f": {"angleDeg": 145}, "hindpaw-f": {"angleDeg": 0}})
    ry_r = 0.062
    P["rolled"] = pose("rolled", ("Rolled up in defence", "Eingerollt zur Verteidigung"),
        ("The defensive ball: both spine skirts swung down around the body until their tips meet under it, the head tucked inside, the limbs folded away; only spines face the world and the animal rests on the skirt tips; breathing slows.",
         "Die Abwehrkugel: beide Stachelsäume sind um den Körper nach unten geschwungen, bis sich ihre Spitzen darunter treffen, der Kopf ist eingezogen, die Gliedmaßen sind weggeklappt; nur Stacheln zeigen nach außen, das Tier ruht auf den Saumspitzen; die Atmung verlangsamt sich."),
        off_r, ["spine-front", "spine-rear"], bind(rolled=True), {"plume-back": 0.6}, 1.1, "rolled", {"x": 0.0})
    # walking keys (lateral sequence), stance 0.66
    N = 8
    S0 = 0.035
    def foot_x(base, ph, lift=0.010):
        ph %= 1.0
        if ph < 0.66:
            u = ph/0.66
            return base + S0 - 2*S0*u, 0.0
        u = (ph - 0.66)/0.34
        e = u*u*(3-2*u)
        return base - S0 + 2*S0*e, lift*math.sin(math.pi*u)
    offsets_ph = {"hn": 0.0, "fn": 0.25, "hf": 0.5, "ff": 0.75}
    walk_keys = []
    for i in range(N):
        ph = i/N
        w = dict(wf); w["pitch"] = 1.5*math.sin(4*math.pi*ph)
        feet = {}; lifts = {}
        for p, o in offsets_ph.items():
            x, l = foot_x(feet0[p], ph - o)
            feet[p] = x; lifts[p] = l
        off = core(w)
        rt = {"x": 0.0, "y": 0.07, "angleDeg": 0}
        for p, x in feet.items():
            side = p[1]; fore = p[0] == "f"
            names = [f"fore-{side}", f"forepaw-{side}"] if fore else [f"hind-{side}", f"hindpaw-{side}"]
            off.update(two(asset, off, rt, names, FL if fore else HL, (x, 0.0105 + lifts[p]), -1 if fore else 1))
        contacts = [f"foot-{p}" for p in feet if lifts[p] == 0.0]
        pid = f"walk-{i}"
        P[pid] = pose(pid, (f"Walk phase {i}", f"Gehphase {i}"), (f"Key pose {i} of 8 of the foraging walk.", f"Schlüsselpose {i} von 8 des Suchgangs."), off, contacts, bind(), {"plume-back": 1.0}, 2.4, "forage", {"x": 0.0}, key=True)
        walk_keys.append(pid)
    # roll-up keys
    wh = dict(pitch=0, neck=-50, head=-95, cf=-110, cr=-70)
    half = mk("rolling-half", ("Rolling up, halfway", "Beim Einrollen, halb"), ("Key pose: the skirts are half closed and the head is drawn in.", "Schlüsselpose: die Säume sind halb geschlossen und der Kopf ist eingezogen."),
              wh, 0.07, {"fn": 0.12, "ff": 0.08, "hn": 0.05, "hf": 0.01}, ["foot-fn", "foot-ff", "foot-hn", "foot-hf"], key=True, met=3.0, br="rolled")
    P["rolling-half"] = half
    asset["poses"] = list(P.values())
    asset["_walk"] = walk_keys
    return asset


def clips(walk_keys, advance):
    wd = 0.8
    walk = {"id": "forage-walk", "title": tx("Foraging walk", "Suchgang"), "description": tx(
        "A slow lateral-sequence walk with the snout low: hind near, front near, hind far, front far, each foot on the ground 66 % of the 0.8 s cycle, the spiny dome bobbing 1.5 degrees; about 0.15 m/s.",
        "Ein langsamer Passgang mit tiefer Schnauze in Folge: hinten nah, vorn nah, hinten fern, vorn fern, jeder Fuß steht 66 % des 0,8-s-Zyklus am Boden, die Stachelkuppel wippt um 1,5 Grad; etwa 0,15 m/s."),
        "durationS": wd, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": advance,
        "keys": [{"t": round(wd*i/8, 5), "pose": p, "ease": "smooth"} for i, p in enumerate(walk_keys)],
        "contacts": [], "phases": [], "events": [], "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.4}, {"t": wd/2, "v": 2.5}, {"t": wd, "v": 2.4}]}]}
    for anchor_, ph in (("foot-hn", 0.0), ("foot-fn", 0.25), ("foot-hf", 0.5), ("foot-ff", 0.75)):
        a, b = ph*wd, (ph + 0.66)*wd
        if b <= wd: walk["contacts"].append({"anchor": anchor_, "fromS": round(a, 4), "toS": round(b, 4)})
        else:
            walk["contacts"].append({"anchor": anchor_, "fromS": round(a, 4), "toS": wd}); walk["contacts"].append({"anchor": anchor_, "fromS": 0, "toS": round(b - wd, 4)})
        walk["events"].append({"t": round(a, 4), "kind": "contact-on", "title": tx("Foot touches down", "Fuß setzt auf"), "target": anchor_})
    walk["phases"] = [{"id": "cycle", "fromS": 0, "toS": wd, "title": tx("Lateral-sequence walk", "Passgang in Folge")}]
    roll = {"id": "roll-up", "title": tx("Rolling up", "Einrollen"), "description": tx(
        "The defensive reaction in 2.4 s: the hedgehog freezes alert, the head is drawn in and the spine skirts swing down around the body until the ball is closed on the skirt tips; breathing drops to the slow rolled pattern and stays there.",
        "Die Abwehrreaktion in 2,4 s: Der Igel erstarrt wachsam, der Kopf wird eingezogen und die Stachelsäume schwingen um den Körper nach unten, bis sich die Kugel auf den Saumspitzen schließt; die Atmung fällt auf das langsame Einrollmuster und bleibt dort."),
        "durationS": 2.4, "fps": 30, "loop": False, "rootMotion": "none",
        "keys": [{"t": 0.0, "pose": "foraging", "ease": "smooth"}, {"t": 0.5, "pose": "alert", "ease": "smooth"}, {"t": 1.1, "pose": "rolling-half", "ease": "smooth"}, {"t": 1.8, "pose": "rolled", "ease": "smooth"}, {"t": 2.4, "pose": "rolled", "ease": "smooth"}],
        "contacts": [{"anchor": "foot-fn", "fromS": 0, "toS": 1.3}, {"anchor": "foot-ff", "fromS": 0, "toS": 1.3}, {"anchor": "foot-hn", "fromS": 0, "toS": 1.3}, {"anchor": "foot-hf", "fromS": 0, "toS": 1.3}, {"anchor": "spine-front", "fromS": 1.1, "toS": 2.4}, {"anchor": "spine-rear", "fromS": 1.1, "toS": 2.4}],
        "phases": [{"id": "freeze", "fromS": 0, "toS": 0.5, "title": tx("Freezing alert", "Wachsames Erstarren")}, {"id": "closing", "fromS": 0.5, "toS": 1.8, "title": tx("Skirts close around the body", "Säume schließen sich um den Körper")}, {"id": "ball", "fromS": 1.8, "toS": 2.4, "title": tx("Closed ball", "Geschlossene Kugel")}],
        "events": [{"t": 0.5, "kind": "motion", "title": tx("Head lifts, startle", "Kopf hebt sich, Erschrecken")}, {"t": 1.1, "kind": "contact-on", "title": tx("Rear skirt tips reach the ground", "Hintere Saumspitzen erreichen den Boden"), "target": "spine-rear"}, {"t": 1.8, "kind": "motion", "title": tx("Ball closed", "Kugel geschlossen")}],
        "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 0.5, "v": 0}, {"t": 1.5, "v": 2}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.0}, {"t": 1.1, "v": 3.0}, {"t": 2.4, "v": 1.1}]}]}
    return [walk, roll]


def thermal():
    regs, skin = regions()
    pats = [pattern("rest", ("Resting alert, nose breathing", "Wachsam ruhend, Nasenatmung"), 40, 0.0070, 0.4, 0.04, 0.45, 0.11, "calder-1984"),
            pattern("forage", ("Foraging walk, sniffing", "Suchgang, Schnuppern"), 80, 0.0075, 0.45, 0.02, 0.45, 0.08, "calder-1984"),
            pattern("rolled", ("Rolled up, slow breathing with long pauses", "Eingerollt, langsame Atmung mit langen Pausen"), 18, 0.0050, 0.3, 0.1, 0.3, 0.3, "calder-1984")]
    return {
        "environment": {"airTempC": 10, "meanRadiantTempC": 10, "skyTempC": 2, "groundTempC": 9, "relativeHumidity": 0.85, "windMS": 0.3, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 0.0, "pressurePa": 101325},
        "baseMet": 1.5,
        "body": {"species": "hedgehog", "massKg": MASS, "skinAreaM2": round(skin, 4), "basalMetabolismW": 2.2, "metWM2": round(2.2/skin, 1), "coreSetPointC": 35.5, "skinSetPointC": 32.0, "sweating": False, "panting": False,
                 "maxSkinBloodFlowLM2H": 30, "minSkinBloodFlowLM2H": 3, "source": "authored-estimate", "reference": "mcnab-1980"},
        "respiration": {"noseEmitter": "breath-nose", "exhaledTempC": 31, "exhaledRelativeHumidity": 0.95, "patterns": pats, "motion": [{"bone": "trunk", "angleDeg": 1.0, "dy": 0.001}], "source": "authored-estimate", "reference": "schmidt-nielsen-1970"},
        "passengers": [], "operating": [], "regions": regs,
        "emitters": [
            {"id": "breath-nose", "kind": "breath", "title": tx("Both nostrils at the snout tip (combined)", "Beide Nasenlöcher an der Schnauzenspitze (zusammen)"), "anchor": "nostrils", "directionDeg": -10, "diameterM": 0.004, "source": "authored-estimate", "reference": "gupta-2010"},
            {"id": "plume-back", "kind": "plume", "title": tx("Warm-air plume above the spiny dome", "Warmluftfahne über der Stachelkuppel"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.1, "region": "spines-dome", "source": "authored-estimate", "reference": "reeve-1994"}],
        "assumptions": [
            tx("Basal metabolism 2.2 W is Kleiber's 3.39 W per kg^0.75 times an authored factor of 0.65 (hedgehogs lie below the Kleiber line, McNab 1980); the 35.5 degC core is a typical value of an active hedgehog (authored estimate); the dome is a 25 mm layer of spines and still air with a resistance of 0.30 m2K/W (authored estimate).",
               "Der Grundumsatz von 2,2 W ist Kleibers 3,39 W pro kg^0,75 mal ein Schätzfaktor von 0,65 (Igel liegen unter der Kleiber-Linie, McNab 1980); die Kerntemperatur von 35,5 Grad ist ein üblicher Wert aktiver Igel (Schätzwert); die Kuppel ist eine 25 mm dicke Schicht aus Stacheln und ruhender Luft mit 0,30 m2K/W Widerstand (Schätzwert)."),
            tx("Rolling up closes the air layer around the body: the belly, face, limbs and paws bind an exposed fraction of 0.03-0.05, so only the spines face the environment; the enclosed air is represented by that exposure, not by an extra layer.",
               "Das Einrollen schließt die Luftschicht um den Körper: Bauch, Gesicht, Gliedmaßen und Pfoten binden einen freien Anteil von 0,03-0,05, sodass nur die Stacheln der Umgebung zugewandt sind; die eingeschlossene Luft wird durch diese Bindung, nicht durch eine zusätzliche Schicht abgebildet."),
            tx("Breathing is nasal; the exhaled air leaves at 31 degC and 95 % relative humidity; there is no sweating or panting.", "Die Atmung erfolgt über die Nase; die Ausatemluft verlässt sie mit 31 Grad und 95 % relativer Feuchte; es gibt weder Schwitzen noch Hecheln.")]}


def base(advance=0.12):
    a = build()
    wk = a.pop("_walk")
    a["clips"] = clips(wk, advance)
    a["thermal"] = thermal()
    a["materials"] = MATERIALS
    return a


if __name__ == "__main__":
    import sys
    a = base()
    print("limits", check_limits(a))
    for pid in sys.argv[1:]:
        skel(a, pid, f"{GEN}/skel-hedgehog-{pid}.png", view=(-0.15, -0.03, 0.3, 0.2), den=1, px_per_mm=2.4*1.0)
