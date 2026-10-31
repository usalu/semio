# [scratch] NW-A1 sheep asset document
import sys
from sheep import *
from assemble import *
from refs import REFS
import sheep_shapes as SS

KEEP = ["kleiber-1932", "merck-resting-respiratory-rates", "hildebrand-1976", "taylor-1970", "brody-1945", "scholander-1950", "cena-monteith-1975", "stahl-1967", "schmidt-nielsen-1970", "gupta-2010", "ashrae-2021", "hales-webster-1967"]
STAND, WALK = 1.2, 3.0


def regions():
    f_tr, f_nk, f_ul, f_ll, f_tl = 1.28, 1.16, 1.12, 1.0, 1.12
    R_ = []
    R_.append(Reg("fleece-trunk", T("Trunk fleece (barrel, rump, shoulders)", "Rumpfvlies (Rumpf, Kruppe, Schultern)"), "fur", "fleece", 0.88, f_tr, [Lay("fleece", T("Fleece, 60 mm staple", "Vlies, 60 mm Stapel"), "fur", "fleece", 0.06, 0.085, f_tr, 2100, 0.5, "authored-estimate", "scholander-1950")], "cylinder", 0.38, 0.55, 20000, "trunk", 0.62, "authored-estimate", "brody-1945"))
    R_.append(Reg("fleece-neck", T("Neck fleece", "Halsvlies"), "fur", "fleece", 0.115, f_nk, [Lay("fleece", T("Fleece, 45 mm", "Vlies, 45 mm"), "fur", "fleece", 0.045, 0.085, f_nk, 1500, 0.5, "authored-estimate", "scholander-1950")], "cylinder", 0.15, 0.6, 18000, "trunk", 0.06, "authored-estimate", "brody-1945"))
    R_.append(Reg("wool-upper-legs", T("Wool on the thighs and forearms", "Wolle an Oberschenkeln und Unterarmen"), "fur", "fleece", 0.17, f_ul, [Lay("wool", T("Short wool, 25 mm", "Kurzwolle, 25 mm"), "fur", "fleece", 0.025, 0.07, f_ul, 900, 0.5, "authored-estimate", "scholander-1950")], "cylinder", 0.08, 0.3, 14000, "leg", 0.09, "authored-estimate", "brody-1945"))
    R_.append(Reg("lower-legs", T("Hair-covered lower legs", "Behaarte Unterläufe"), "fur", "short-hair", 0.15, f_ll, [Lay("hair", T("Short hair, 4 mm", "Kurzhaar, 4 mm"), "hair", "short-hair", 0.004, 0.05, f_ll, 80, 0.5, "authored-estimate", "cena-monteith-1975")], "cylinder", 0.035, 0.25, 9000, "leg", 0.05, "authored-estimate", "brody-1945"))
    R_.append(Reg("face", T("Face and muzzle", "Gesicht und Fang"), "fur", "short-hair", 0.055, 1.0, [Lay("hair", T("Short hair, 3 mm", "Kurzhaar, 3 mm"), "hair", "short-hair", 0.003, 0.05, 1.0, 60, 0.5, "authored-estimate", "cena-monteith-1975")], "cylinder", 0.12, 0.5, 10000, "head", 0.1, "authored-estimate", "brody-1945"))
    R_.append(Reg("ears", T("Ears", "Ohren"), "skin", "bare-skin", 0.022, 1.0, [], "vertical-plate", 0.07, 0.6, 5000, "head", 0.01, "authored-estimate", "brody-1945"))
    R_.append(Reg("tail", T("Wool tail", "Wollschwanz"), "fur", "fleece", 0.035, f_tl, [Lay("wool", T("Wool, 30 mm", "Wolle, 30 mm"), "fur", "fleece", 0.03, 0.08, f_tl, 700, 0.5, "authored-estimate", "scholander-1950")], "cylinder", 0.05, 0.3, 8000, "tail", 0.01, "authored-estimate", "brody-1945"))
    R_.append(Reg("hooves", T("Hooves", "Klauen"), "skin", "hoof-horn", 0.016, 1.0, [], "horizontal-plate-down", 0.04, 0.0, 8000, "foot", 0.01, "authored-estimate", "brody-1945"))
    return finalize_regions(R_)


def thermal_pose(regs, act, breathing, contact_frac=0.0, sun=None, lying=False, ground_t=18.0):
    sun = sun or {"*": 0.5, "wool-upper-legs": 0.25, "lower-legs": 0.2, "hooves": 0.1, "ears": 0.6, "fleece-neck": 0.55}
    over = {"hooves": {"exposedFraction": round(1 - 0.6 * contact_frac, 3), "contact": {"fraction": round(0.6 * contact_frac, 3), "tempC": ground_t}} if contact_frac > 0 else {"exposedFraction": 0.8}}
    if lying:
        over["fleece-trunk"] = {"exposedFraction": 0.78, "contact": {"fraction": 0.22, "tempC": ground_t}}
        over["wool-upper-legs"] = {"exposedFraction": 0.5, "contact": {"fraction": 0.5, "tempC": ground_t}}
        over["lower-legs"] = {"exposedFraction": 0.5, "contact": {"fraction": 0.3, "tempC": ground_t}}
    return bind(regs, act, breathing, sun, over)


def build():
    sk = make_skel()
    regs, askin = regions()
    A_SKIN = round(askin, 4)
    basal = round(3.39 * MASS ** 0.75, 2)
    body = {"species": "sheep", "massKg": MASS, "skinAreaM2": A_SKIN, "basalMetabolismW": basal, "metWM2": round(basal / A_SKIN, 2), "coreSetPointC": 39.0, "skinSetPointC": 36.0, "sweating": False, "panting": True, "maxSkinBloodFlowLM2H": 40, "minSkinBloodFlowLM2H": 4, "source": "authored-estimate", "reference": "kleiber-1932"}
    pats = [
        pattern("rest", T("Resting, nose breathing", "Ruhe, Nasenatmung"), 22, 0.45, (0.4, 0.05, 0.45, 0.1), "nose", "merck-resting-respiratory-rates"),
        pattern("walk", T("Walking, nose breathing", "Gehen, Nasenatmung"), 38, 0.6, (0.42, 0.02, 0.5, 0.06), "nose", "stahl-1967"),
        pattern("graze", T("Grazing and chewing", "Grasen und Kauen"), 28, 0.5, (0.42, 0.03, 0.47, 0.08), "nose", "stahl-1967"),
        pattern("pant", T("Heat panting, open nostrils and mouth", "Hecheln bei Hitze, Nase und Maul"), 200, 0.16, (0.5, 0.0, 0.5, 0.0), "nose-and-mouth", "hales-webster-1967"),
        pattern("ruminate", T("Lying, ruminating", "Liegen, Wiederkäuen"), 20, 0.42, (0.4, 0.05, 0.45, 0.1), "nose", "merck-resting-respiratory-rates"),
    ]
    resp = respiration_block(pats, [{"bone": "thorax", "angleDeg": 0.8, "dy": 0.006}, {"bone": "lumbar", "angleDeg": 0.3, "dy": 0.003}], 33, 0.95, "schmidt-nielsen-1970")
    emitters = [
        {"id": "breath-nose", "kind": "breath", "title": T("Nostrils", "Nasenlöcher"), "anchor": "nose", "directionDeg": -55, "diameterM": 0.011, "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "breath-mouth", "kind": "breath", "title": T("Mouth (open-mouth panting)", "Maul (Hecheln mit offenem Maul)"), "anchor": "mouth", "directionDeg": -10, "diameterM": 0.02, "source": "authored-estimate", "reference": "hales-webster-1967"},
        {"id": "plume-back", "kind": "plume", "title": T("Warm-air plume above the fleece", "Warmluftfahne über dem Vlies"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.5, "region": "fleece-trunk", "source": "authored-estimate", "reference": "cena-monteith-1975"},
    ]
    st_offs = standing_offsets(sk)
    st_offs.update({"ear-n": 0, "ear-f": 0})
    gr_offs, gr_root = grazing_offsets(sk)
    ly_offs, ly_root = lying_offsets(sk)
    poses = []
    four = ["contact-hn", "contact-fn", "contact-hf", "contact-ff"]
    poses.append(pose("standing-resting", T("Standing at rest", "Ruhig stehend"), T("Standing square on four hooves, head carried level, ears out to the side, short tail hanging.", "Gerades Stehen auf vier Klauen, Kopf waagerecht getragen, Ohren seitlich, kurzer Schwanz hängend."), st_offs, thermal_pose(regs, STAND, "rest", 1.0), four))
    poses.append(pose("grazing", T("Grazing", "Grasend"), T("Head lowered to the pasture: pelvis and neck tilted, hind legs flexed, the near fore hoof placed forward, nose a hand above the ground.", "Kopf zur Weide gesenkt: Becken und Hals gekippt, Hinterbeine gebeugt, nahes Vorderbein vorgesetzt, Nase eine Handbreit über dem Boden."), gr_offs, thermal_pose(regs, 1.5, "graze", 1.0), four, root={"y": r4(gr_root[1])}))
    poses.append(pose("lying-ruminating", T("Lying, ruminating", "Liegend, wiederkäuend"), T("Sternal recumbency: legs folded under the fleece, belly and brisket on the ground, head up for chewing the cud.", "Brustlage: Beine unter dem Vlies eingeschlagen, Bauch und Brustbein am Boden, Kopf erhoben zum Wiederkäuen."), ly_offs, thermal_pose(regs, 1.0, "ruminate", 0.0, lying=True), ["contact-belly", "contact-brisket"], root={"y": r4(ly_root[1])}))
    return sk, regs, A_SKIN, body, resp, emitters, poses


LEGNAMES = {"hn": ("Near hind hoof", "Nahe Hinterklaue"), "fn": ("Near fore hoof", "Nahe Vorderklaue"), "hf": ("Far hind hoof", "Ferne Hinterklaue"), "ff": ("Far fore hoof", "Ferne Vorderklaue")}


def add_walk(sk, regs, poses, clips):
    Gh = dict(ac0=-88, ks=10, flex=40, ah0=-52, ahroll=12, ahflex=14, lift=0.06)
    Gf = dict(ac0=-90, ks=8, flex=55, ah0=-53, ahroll=12, ahflex=14, lift=0.07, scap0=-62, scapamp=6)
    xs = {"hn": 0.05, "hf": 0.05, "fn": 0.65, "ff": 0.65}
    order = {"hn": 0.0, "fn": 0.25, "hf": 0.5, "ff": 0.75}
    T_, duty, speed, N = 1.15, 0.667, 0.4955, 12

    def trunk(ph):
        return {"neck": r4(3.0 * math.sin(2 * math.pi * ph)), "head": r4(-2.0 * math.sin(2 * math.pi * ph + 0.6)), "tail-1": r4(4 * math.sin(2 * math.pi * ph + 1.0)), "ear-n": r4(3 * math.sin(2 * math.pi * ph * 2)), "ear-f": r4(3 * math.sin(2 * math.pi * ph * 2 + 0.5))}

    def thermal(ph, contacts):
        return thermal_pose(regs, WALK, "walk", len(contacts) / 4.0)

    def ptitle(k, N):
        return T("Walk, phase %d of %d" % (k + 1, N), "Schritt, Phase %d von %d" % (k + 1, N)), T("Lateral-sequence walk at %d/%d of the cycle." % (k, N), "Schritt in Seitenfolge bei %d/%d des Zyklus." % (k, N))

    kp, clip, S = gait_clip(sk, "walk-cycle", T("Slow walk, four-beat", "Langsamer Schritt im Viertakt"), T("Lateral-sequence walk after Hildebrand (1976): near hind, near fore, far hind, far fore touch down at quarter-cycle intervals; each hoof is on the ground two thirds of the cycle, so two to four hooves always carry.", "Schritt in Seitenfolge nach Hildebrand (1976): nahe Hinter-, nahe Vorder-, ferne Hinter-, ferne Vorderklaue setzen im Viertelzyklus auf; jede Klaue steht zwei Drittel des Zyklus am Boden, sodass stets zwei bis vier Klauen tragen."), ptitle, T_, N, duty, order, speed, Gh, Gf, {}, xs, thermal, FOOT, trunk_fn=trunk, names=LEGNAMES)
    clip["phases"] = [{"id": "stride-%d" % i, "fromS": r4(i * T_ / 4), "toS": r4((i + 1) * T_ / 4), "title": T("Quarter %d of the four-beat stride" % (i + 1), "Viertel %d des Viertakt-Schrittes" % (i + 1))} for i in range(4)]
    clip["tracks"] = [{"target": "breathing", "ease": "step", "keys": [{"t": 0.0, "v": 1}]}]
    clip["advanceM"] = 0.5529
    poses.extend(kp)
    clips.append(clip)


def add_graze(sk, regs, poses, clips):
    def jaw_cycle(t0, n, period, amp):
        return [{"t": r4(t0 + i * period / 2), "pose": "grazing", "ease": "smooth", "bones": {"jaw": {"angleDeg": -amp if i % 2 == 0 else 0.0}}} for i in range(n)]

    keys = [{"t": 0.0, "pose": "grazing", "ease": "smooth"}] + jaw_cycle(0.4, 6, 0.8, 9) + [{"t": 3.0, "pose": "standing-resting", "ease": "smooth", "bones": {"jaw": {"angleDeg": -6.0}}}, {"t": 3.5, "pose": "standing-resting", "ease": "smooth", "bones": {"jaw": {"angleDeg": 0.0}}}]
    keys.sort(key=lambda k: k["t"])
    keys.insert(1, {"t": 0.2, "pose": "grazing", "ease": "smooth", "bones": {"jaw": {"angleDeg": 0.0}}})
    clips.append({"id": "graze-cycle", "title": T("Grazing, lifting the head to look", "Grasen mit aufblickendem Kopf"), "description": T("Head down tearing and chewing grass (jaw about 75 chews per minute), then lifting the head to look around and back down; all four hooves stay planted.", "Kopf unten beim Abreißen und Kauen von Gras (Kiefer etwa 75 Kauschläge je Minute), dann Kopf zum Umschauen heben und wieder senken; alle vier Klauen bleiben am Boden."), "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [{"anchor": a, "fromS": 0.0, "toS": 4.0} for a in ("contact-hn", "contact-hf", "contact-fn", "contact-ff")], "phases": [{"id": "head-down", "fromS": 0.0, "toS": 3.0, "title": T("Head down, chewing", "Kopf unten, kauend")}, {"id": "head-up", "fromS": 3.0, "toS": 4.0, "title": T("Head up, looking", "Kopf oben, umschauend")}], "events": [{"t": 3.0, "kind": "motion", "title": T("Head lifts", "Kopf hebt sich")}], "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0.0, "v": 1.5}, {"t": 3.0, "v": 1.5}, {"t": 3.5, "v": 1.2}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0.0, "v": 2}, {"t": 3.0, "v": 0}]}]})
    keys2 = []
    for i in range(8):
        keys2.append({"t": r4(i * 0.5), "pose": "lying-ruminating", "ease": "smooth", "bones": {"jaw": {"angleDeg": -8.0 if i % 2 == 0 else 0.0}, "head": {"angleDeg": (20.0 + (2.0 if i % 4 == 0 else 0.0))}}})
    clips.append({"id": "ruminate-lying", "title": T("Lying, chewing the cud", "Liegend wiederkäuen"), "description": T("Chewing the cud while lying: the jaw works about 60 times a minute, the head nods slightly with each bolus; belly and brisket stay on the ground.", "Wiederkäuen im Liegen: der Kiefer arbeitet etwa 60-mal je Minute, der Kopf nickt mit jedem Bissen leicht; Bauch und Brustbein bleiben am Boden."), "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys2, "contacts": [{"anchor": a, "fromS": 0.0, "toS": 4.0} for a in ("contact-belly", "contact-brisket")], "phases": [], "events": [], "tracks": []})


def reps(sk, shp):
    H = lambda sp, pen: [{"id": "fleece", "title": T("Wool crimp stipple", "Wollkräuselung, Punktraster"), "pattern": "dots", "angleDeg": 0, "spacingMm": sp, "penMm": pen}]
    r = {}
    r["500"] = authored(500, shp["500"], D8(
        ("One composed side silhouette of 2.5 × 1.7 mm: hindquarters and chest as two overlapping masses, a neck wedge, one head mass with ear tip; the four legs are strokes along the limb bones.", "Eine zusammengesetzte Seitensilhouette von 2,5 × 1,7 mm: Hinterhand und Brust als zwei überlappende Massen, ein Halskeil, eine Kopfmasse mit Ohrspitze; die vier Beine sind Striche entlang der Gliedmaßenknochen."),
        ("The heavy fleece barrel, the short neck, the level head carriage and the thin legs under the barrel; near and far legs stay separate bone chains so the gait stays legible.", "Die schwere Vliestonne, der kurze Hals, die waagerechte Kopfhaltung und die dünnen Beine unter der Tonne; nahe und ferne Beine bleiben getrennte Knochenketten, damit der Gang lesbar bleibt."),
        ("Shoulder, upper arm and thigh vanish in the barrel; ears into the head mass; tail and thigh wool into the hindquarter mass; hair legs and hooves into one stroke per lower leg. All regions are listed on the merged masses.", "Schulter, Oberarm und Oberschenkel gehen in der Tonne auf; Ohren in der Kopfmasse; Schwanz und Schenkelwolle in der Hinterhandmasse; Haarbeine und Klauen in einen Strich je Unterbein. Alle Regionen stehen an den verschmolzenen Massen."),
        ("Eye, mouth, jaw, hooves as shapes, fleece structure and fold lines: the head is 0.23 m = 0.46 mm long.", "Auge, Maul, Unterkiefer, Klauen als Formen, Vliesstruktur und Faltenlinien: der Kopf ist 0,23 m = 0,46 mm lang."),
        ("Lower legs (real width 0.035 m = 0.07 mm) are 0.2 mm strokes, i.e. 0.1 m wide; the 0.13 mm outline already adds 0.13 m of ink, so the legs read as legs without a wider body.", "Unterbeine (reale Breite 0,035 m = 0,07 mm) sind 0,2-mm-Striche, also 0,1 m breit; die 0,13-mm-Kontur trägt ohnehin 0,13 m Tinte auf, die Beine lesen sich als Beine ohne breiteren Körper."),
        ("None.", "Keine."),
        ("Mid-grey masses with a 0.13 mm outline, far legs lighter; the fleece barrel reads as the darkest, heaviest mass.", "Mittelgraue Massen mit 0,13-mm-Kontur, ferne Beine heller; die Vliestonne wirkt als dunkelste, schwerste Masse."),
        ("Thermal colours paint the merged masses with the area-weighted value of the listed regions (labelled merged); the fleece barrel dominates, face, ears and legs cannot be told apart from the neighbouring mass.", "Thermische Farben färben die verschmolzenen Massen mit dem flächengewichteten Wert der genannten Regionen (als verschmolzen gekennzeichnet); die Vliestonne dominiert, Gesicht, Ohren und Beine sind von der Nachbarmasse nicht zu trennen.")))
    r["200"] = authored(200, shp["200"], D8(
        ("Main masses separated, 5.0 × 4.3 mm: fleece hindquarters, fleece chest, neck, head, ear, tail and per side a wool gaskin or forearm, a hair cannon and a hoof.", "Hauptmassen getrennt, 5,0 × 4,3 mm: Vlieshinterhand, Vliesbrust, Hals, Kopf, Ohr, Schwanz und je Seite Wollunterschenkel oder Unterarm, Haarröhre und Klaue."),
        ("Fleece volume with the level, long head, the hock angle of the hind leg and the hoof as its own mark so the gait reads.", "Vliesvolumen mit dem langen waagerechten Kopf, der Sprunggelenkwinkelung des Hinterbeins und der Klaue als eigene Marke, damit der Gang lesbar bleibt."),
        ("Jaw into the head, shoulder, upper arm and thigh into the fleece masses, the second tail bone into the tail, the pair of ears into the near ear.", "Unterkiefer in den Kopf, Schulter, Oberarm und Oberschenkel in die Vliesmassen, zweiter Schwanzknochen in den Schwanz, das Ohrenpaar in das nahe Ohr."),
        ("Eye, mouth, hoof cleft, fleece structure and fold lines.", "Auge, Maul, Klauenspalte, Vliesstruktur und Faltenlinien."),
        ("Cannons (0.035 m = 0.18 mm) are drawn 0.048 m wide so they reach the 0.2 mm feature minimum; this is 0.013 m wider than real.", "Röhrbeine (0,035 m = 0,18 mm) sind 0,048 m breit gezeichnet, damit sie das Mindestmerkmal von 0,2 mm erreichen; das ist 0,013 m breiter als real."),
        ("None; no hatch spacing at or above the profile minimum fits on a 5 mm figure.", "Keine; auf einer 5 mm großen Figur passt kein Schraffurabstand ab dem Profilminimum."),
        ("Mid-grey solids with a 0.18 mm outline; far side lighter, ear lightest.", "Mittelgraue Flächen mit 0,18-mm-Kontur; ferne Seite heller, Ohr am hellsten."),
        ("Each mass carries its own region (trunk fleece, neck fleece, wool leg parts, lower legs, hooves, ears, tail); head carries the face region.", "Jede Masse trägt ihre eigene Region (Rumpfvlies, Halsvlies, Wollbeinteile, Unterläufe, Klauen, Ohren, Schwanz); der Kopf trägt die Gesichtsregion.")))
    r["100"] = authored(100, shp["100"], D8(
        ("Articulated outline, 10 × 8.6 mm: stippled fleece barrel in two overlapping masses, neck, head with eye, near and far ears, tail, and per side wool gaskin or forearm, cannon and hoof wedge.", "Gegliederter Umriss, 10 × 8,6 mm: gepunktete Vliestonne in zwei überlappenden Massen, Hals, Kopf mit Auge, nahes und fernes Ohr, Schwanz, je Seite Wollunterschenkel oder Unterarm, Röhre und Klauenkeil."),
        ("Hock and fetlock angles, the hoof wedge, ear leaves, the long ewe head with eye, the fleece barrel with a lighter stipple.", "Sprunggelenk- und Fesselwinkel, der Klauenkeil, die Ohrblätter, der lange Schafskopf mit Auge, die Vliestonne mit hellem Punktraster."),
        ("Shoulder, upper arm and thigh into the fleece masses; both tail bones into one tail; the lower jaw into the head (chewing is drawn from 1:50).", "Schulter, Oberarm und Oberschenkel in die Vliesmassen; beide Schwanzknochen zu einem Schwanz; der Unterkiefer in den Kopf (Kauen wird ab 1:50 gezeichnet)."),
        ("Mouth line, nostril, fold lines, hoof cleft and ear lines.", "Maullinie, Nüstern, Faltenlinien, Klauenspalte und Ohrlinien."),
        ("Cannons (0.035 m = 0.35 mm) and fetlocks are drawn at real width; the ears are 0.04 m wide leaves.", "Röhren (0,035 m = 0,35 mm) und Fesseln sind in realer Breite gezeichnet; die Ohren sind 0,04 m breite Blätter."),
        ("A light dot stipple (0.8 mm spacing, 0.13 mm pen) marks the fleece on barrel, gaskin and forearm only; hair legs, face and hooves stay clean.", "Ein leichtes Punktraster (0,8 mm Abstand, 0,13 mm Stift) kennzeichnet das Vlies nur an Tonne, Unterschenkel und Unterarm; Haarbeine, Gesicht und Klauen bleiben sauber."),
        ("Mid-grey solids, 0.25 mm outline, far side lighter; the eye is the only dark detail.", "Mittelgraue Flächen, 0,25-mm-Kontur, ferne Seite heller; das Auge ist das einzige dunkle Detail."),
        ("One region per mass; the stippled shapes are the wool regions, so thermal colour and material read together.", "Eine Region je Masse; die gepunkteten Formen sind die Wollregionen, thermische Farbe und Material lesen sich zusammen.")), hatches=H(0.8, 0.13))
    r["50"] = authored(50, shp["50"], D8(
        ("Full articulated drawing, 20 × 17 mm: stippled fleece barrel in two masses with fold lines at stifle and elbow, neck, head with eye, mouth line and jaw, both ears, wool tail in two parts, per side wool gaskin and forearm, cannon and cloven hoof.", "Voll gegliederte Zeichnung, 20 × 17 mm: gepunktete Vliestonne in zwei Massen mit Faltenlinien an Knie und Ellbogen, Hals, Kopf mit Auge, Maullinie und Unterkiefer, beide Ohren, Wollschwanz in zwei Teilen, je Seite Wollunterschenkel und Unterarm, Röhre und Klaue."),
        ("Everything of the 1:100 drawing plus the jaw (so chewing and panting show), the two tail parts, the stifle and elbow fold lines and a 0.18 mm eye and mouth line.", "Alles der 1:100-Zeichnung plus Unterkiefer (damit Kauen und Hecheln sichtbar sind), beide Schwanzteile, Knie- und Ellbogenfaltenlinien sowie ein 0,18-mm-Auge und eine Maullinie."),
        ("Shoulder blade and upper arm and the thigh are not drawn as masses: they lie inside the fleece (their bones move the fold lines).", "Schulterblatt, Oberarm und Oberschenkel sind nicht als Massen gezeichnet: sie liegen im Vlies (ihre Knochen bewegen die Faltenlinien)."),
        ("Nostril, individual toes and the cleft of the hoof are not drawn; the head is 4.6 mm long.", "Nüstern, einzelne Zehen und die Klauenspalte sind nicht gezeichnet; der Kopf ist 4,6 mm lang."),
        ("Cannons 0.034 m = 0.7 mm and the fetlock bulge are drawn at real size; ear 0.04 m = 0.8 mm.", "Röhren 0,034 m = 0,7 mm und die Fesselwulst sind in realer Größe gezeichnet; Ohr 0,04 m = 0,8 mm."),
        ("Dot stipple, 0.8 mm spacing, 0.1 mm pen, on every wool region (barrel, neck, gaskin, forearm, tail root); hair and horn are left white.", "Punktraster, 0,8 mm Abstand, 0,1 mm Stift, auf allen Wollregionen (Tonne, Hals, Unterschenkel, Unterarm, Schwanzwurzel); Haar und Horn bleiben weiß."),
        ("Mid-grey solids with a 0.35 mm outline, 0.18 mm detail strokes, 0.1 mm stipple; far side lighter, ear lightest.", "Mittelgraue Flächen mit 0,35-mm-Kontur, 0,18-mm-Detailstriche, 0,1-mm-Punktraster; ferne Seite heller, Ohr am hellsten."),
        ("Each wool region is stippled and keeps its own shape; the fold lines carry the trunk region; the face carries eye, mouth and jaw.", "Jede Wollregion ist gepunktet und behält ihre eigene Form; die Faltenlinien tragen die Rumpfregion; das Gesicht trägt Auge, Maul und Kiefer.")), hatches=H(0.8, 0.1))
    return r


def main():
    sk, regs, A_SKIN, body, resp, emitters, poses = build()
    clips = []
    add_walk(sk, regs, poses, clips)
    add_graze(sk, regs, poses, clips)
    shp = {"500": SS.shapes500(sk), "200": SS.shapes200(sk), "100": SS.shapes100(sk), "50": SS.shapes50(sk)}
    mats = [
        {"id": "fleece", "title": T("Wool fleece", "Wollvlies"), "emissivity": 0.95, "solarAbsorptivity": 0.45, "densityKgM3": 1300, "conductivityWmK": 0.085, "source": "authored-estimate", "reference": "scholander-1950"},
        {"id": "short-hair", "title": T("Short hair", "Kurzhaar"), "emissivity": 0.96, "solarAbsorptivity": 0.6, "conductivityWmK": 0.05, "source": "authored-estimate", "reference": "cena-monteith-1975"},
        {"id": "bare-skin", "title": T("Bare skin", "Nackte Haut"), "emissivity": 0.98, "solarAbsorptivity": 0.65, "source": "literature", "reference": "ashrae-2021"},
        {"id": "hoof-horn", "title": T("Hoof horn", "Klauenhorn"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "source": "authored-estimate"},
    ]
    env = {"airTempC": 20, "meanRadiantTempC": 22, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325}
    assumptions = [
        T("Skin area 0.085·M^(2/3) = %.3f m² for 70 kg is an authored estimate guided by Brody (1945); the eight regions cover it; outer areas include a fleece area factor (1.0-1.28) and layer resistances are intrinsic fleece resistance times that factor." % A_SKIN, "Die Hautfläche 0,085·M^(2/3) = %.3f m² für 70 kg ist ein Schätzwert nach Brody (1945); die acht Regionen decken sie ab; Außenflächen enthalten einen Vliesflächenfaktor (1,0-1,28), Schichtwiderstände sind der intrinsische Vlieswiderstand mal dieser Faktor." % A_SKIN),
        T("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = %.1f W (verified coefficient); one met is that rate per skin area. Standing 1.2, grazing 1.5, lying 1.0 and the slow walk at 0.5 m/s (3.0 met) follow the cost of locomotion after Taylor et al. (1970); the multiples are authored estimates." % body["basalMetabolismW"], "Der Grundumsatz ist Kleibers 3,39 W je kg^0,75 = %.1f W (verifizierter Koeffizient); ein Met ist diese Rate je Hautfläche. Stehen 1,2, Grasen 1,5, Liegen 1,0 und der langsame Schritt mit 0,5 m/s (3,0 Met) folgen dem Fortbewegungsaufwand nach Taylor et al. (1970); die Vielfachen sind Schätzwerte." % body["basalMetabolismW"]),
        T("Fleece conductivity 0.085 W/(m K) (open fleece incl. radiation and convection inside the coat) and 60 mm / 45 mm / 25 mm thicknesses are authored estimates; the qualitative role of coat thickness follows Scholander et al. (1950). Legs below the carpus and hock and the face are short-haired.", "Die Vlies-Leitfähigkeit von 0,085 W/(m K) (offenes Vlies einschließlich Strahlung und Konvektion im Fell) und die Dicken 60 / 45 / 25 mm sind Schätzwerte; die qualitative Rolle der Felldicke folgt Scholander et al. (1950). Beine unter Vorderfusswurzel und Sprunggelenk sowie das Gesicht sind kurzhaarig."),
        T("Resting rate 22 per minute lies inside the verified resting range of sheep (16-34, Merck Veterinary Manual after Reece 2004); tidal volumes (0.45 L at rest, about 6.4 mL/kg) and the heat-panting pattern (200 per minute, 0.16 L, guided by Hales and Webster 1967) are authored estimates. Nose breath leaves at 33 °C (Schmidt-Nielsen et al. 1970).", "Die Ruhefrequenz von 22 je Minute liegt im verifizierten Ruhebereich von Schafen (16-34, Merck Veterinary Manual nach Reece 2004); Atemzugvolumina (0,45 L in Ruhe, etwa 6,4 mL/kg) und das Hechelmuster (200 je Minute, 0,16 L, nach Hales und Webster 1967) sind Schätzwerte. Nasenluft verlässt den Körper mit 33 °C (Schmidt-Nielsen et al. 1970)."),
        T("Sheep have little functional sweating under the fleece: heat is lost by panting, the ears, the face and the shorn legs, so those regions are bare or short-haired.", "Schafe schwitzen unter dem Vlies kaum nennenswert: Wärme wird durch Hecheln, Ohren, Gesicht und die kurzbehaarten Beine abgegeben, daher sind diese Regionen nackt oder kurzhaarig."),
    ]
    refs = [REFS[k] for k in KEEP]
    asset = {
        "id": "sheep", "category": "animals",
        "title": T("Sheep (Ewe in Full Fleece)", "Schaf (Mutterschaf im vollen Vlies)"),
        "summary": T("An adult wool-breed ewe of 70 kg with 0.74 m withers height in full fleece: a heavy fleece barrel on thin hair-covered legs, short wool tail, level head carriage; standing, grazing with the head down, lying and ruminating, a slow four-beat walk, with fleece as a thick insulating layer, nose and open-mouth heat panting.", "Ein erwachsenes Mutterschaf einer Wollrasse von 70 kg mit 0,74 m Widerristhöhe im vollen Vlies: eine schwere Vliestonne auf dünnen behaarten Beinen, kurzer Wollschwanz, waagerechte Kopfhaltung; Stehen, Grasen mit gesenktem Kopf, Liegen und Wiederkäuen, ein langsamer Viertakt-Schritt, mit dem Vlies als dicker Dämmschicht, Nasenatmung und Hecheln."),
        "tags": ["sheep", "ewe", "livestock", "fleece", "grazing", "quadruped", "pasture", "ruminant"],
        "provenance": {"author": "Semio staffage content worker NW-A1", "method": "handcrafted", "created": "2026-10-29", "references": refs, "notes": T("Side elevation of the near (left) side of a ewe facing right; far-side legs and the far ear are separate chains drawn lighter and behind. All proportions, joint angles, masses and thermal parameters are authored estimates; only Kleiber's coefficient and the Merck resting respiratory range are verified literature values. Four preset scales are individually drawn.", "Seitenansicht der nahen (linken) Seite eines nach rechts blickenden Mutterschafs; ferne Beine und fernes Ohr sind eigene Ketten, heller und dahinter gezeichnet. Alle Proportionen, Gelenkwinkel, Massen und Wärmeparameter sind Schätzwerte; nur Kleibers Koeffizient und der Merck-Ruhebereich der Atemfrequenz sind verifizierte Literaturwerte. Vier Maßstäbe sind einzeln gezeichnet.")},
        "dimensions": {"heightM": DIM[0], "widthM": DIM[1], "depthM": 0.44, "massKg": MASS},
        "materials": mats,
        "rig": {"kind": "anatomical", "root": "pelvis", "bones": finish_bones(sk)},
        "anchors": sk.anchors,
        "representations": {}, "symbols": [],
        "poses": poses, "clips": clips,
        "thermal": {"environment": env, "baseMet": STAND, "body": body, "respiration": resp, "passengers": [], "operating": [], "regions": regs, "emitters": emitters, "assumptions": assumptions},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [
            T("Segments are rigid with authored masses summing to 70 kg including about 5 kg of fleece on the trunk; contacts come from pose contacts and clip spans: hooves carry when standing, belly and brisket when lying.", "Segmente sind starr mit Massen von zusammen 70 kg einschließlich etwa 5 kg Vlies am Rumpf; Kontakte kommen aus Posenkontakten und Clip-Spannen: Klauen tragen im Stand, Bauch und Brustbein im Liegen."),
            T("Joint moments are the moments of all distal weights and reactions about each joint; no muscle model is assumed.", "Gelenkmomente sind die Momente aller distalen Gewichte und Reaktionen um jedes Gelenk; ein Muskelmodell wird nicht angenommen.")], "clinical": False},
        "presentation": {"defaultPose": "standing-resting", "defaultClip": "walk-cycle", "viewpoints": ["elevation"], "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"],
        "completion": "authored",
    }
    asset["representations"] = reps(sk, shp)
    msgs = verify_poses(sk, asset, shp)
    print("\n".join(msgs))
    return write(asset, "sheep")


DIM = (0.0, 0.0)
if __name__ == "__main__":
    DIM = (float(sys.argv[1]), float(sys.argv[2])) if len(sys.argv) > 2 else (0.85, 1.22)
    print(main())
