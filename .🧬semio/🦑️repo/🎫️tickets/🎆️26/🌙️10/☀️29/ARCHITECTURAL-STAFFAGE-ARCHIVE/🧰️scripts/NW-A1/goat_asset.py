# [scratch] NW-A1 goat asset document
import sys
from goat import *
from goat_poses import *
from assemble import *
from refs import REFS
import goat_shapes as GS

KEEP = ["kleiber-1932", "hildebrand-1976", "taylor-1970", "brody-1945", "scholander-1950", "cena-monteith-1975", "stahl-1967", "schmidt-nielsen-1970", "gupta-2010", "ashrae-2021", "silanikove-2000", "crawford-1962"]
STAND, WALK = 1.3, 3.2
SUN = {"*": 0.5, "coat-upper-legs": 0.3, "lower-legs": 0.2, "hooves": 0.1, "ears": 0.6, "coat-neck": 0.55, "horns": 0.7}
LEGNAMES = {"hn": ("Near hind hoof", "Nahe Hinterklaue"), "fn": ("Near fore hoof", "Nahe Vorderklaue"), "hf": ("Far hind hoof", "Ferne Hinterklaue"), "ff": ("Far fore hoof", "Ferne Vorderklaue")}


def regions():
    L = lambda id, t, th, k, f, cap, kind="hair": Lay(id, t, kind, "short-coat", th, k, f, cap, 0.5, "authored-estimate", "scholander-1950")
    R_ = []
    R_.append(Reg("coat-trunk", T("Trunk coat (barrel, rump)", "Rumpffell (Rumpf, Kruppe)"), "fur", "short-coat", 0.72, 1.06, [L("coat", T("Summer coat, 12 mm", "Sommerfell, 12 mm"), 0.012, 0.05, 1.06, 700)], "cylinder", 0.33, 0.55, 18000, "trunk", 0.62, "authored-estimate", "brody-1945"))
    R_.append(Reg("coat-neck", T("Neck coat", "Halsfell"), "fur", "short-coat", 0.12, 1.04, [L("coat", T("Coat, 10 mm", "Fell, 10 mm"), 0.010, 0.05, 1.04, 600)], "cylinder", 0.12, 0.6, 16000, "trunk", 0.06, "authored-estimate", "brody-1945"))
    R_.append(Reg("coat-upper-legs", T("Thighs, shoulders and forearms", "Oberschenkel, Schultern und Unterarme"), "fur", "short-coat", 0.26, 1.02, [L("coat", T("Coat, 8 mm", "Fell, 8 mm"), 0.008, 0.05, 1.02, 450)], "cylinder", 0.10, 0.35, 14000, "leg", 0.11, "authored-estimate", "brody-1945"))
    R_.append(Reg("lower-legs", T("Lower legs", "Unterläufe"), "fur", "short-coat", 0.14, 1.0, [L("coat", T("Short hair, 4 mm", "Kurzhaar, 4 mm"), 0.004, 0.05, 1.0, 200)], "cylinder", 0.035, 0.25, 9000, "leg", 0.05, "authored-estimate", "brody-1945"))
    R_.append(Reg("face", T("Face, muzzle and beard", "Gesicht, Fang und Bart"), "fur", "short-coat", 0.06, 1.0, [L("coat", T("Short hair, 4 mm", "Kurzhaar, 4 mm"), 0.004, 0.05, 1.0, 200)], "cylinder", 0.12, 0.5, 10000, "head", 0.09, "authored-estimate", "brody-1945"))
    R_.append(Reg("ears", T("Ears", "Ohren"), "skin", "bare-skin", 0.03, 1.0, [], "vertical-plate", 0.08, 0.6, 5000, "head", 0.015, "authored-estimate", "silanikove-2000"))
    R_.append(Reg("horns", T("Horn sheaths", "Hornscheiden"), "skin", "horn", 0.012, 1.0, [], "cylinder", 0.04, 0.8, 6000, "head", 0.006, "authored-estimate", "silanikove-2000"))
    R_.append(Reg("tail", T("Tail", "Schwanz"), "fur", "short-coat", 0.015, 1.0, [L("coat", T("Coat, 8 mm", "Fell, 8 mm"), 0.008, 0.05, 1.0, 300)], "cylinder", 0.04, 0.6, 8000, "tail", 0.005, "authored-estimate", "brody-1945"))
    R_.append(Reg("hooves", T("Hooves", "Klauen"), "skin", "horn", 0.012, 1.0, [], "horizontal-plate-down", 0.04, 0.0, 8000, "foot", 0.01, "authored-estimate", "brody-1945"))
    return finalize_regions(R_)


def tp(regs, act, breathing, contact_frac=0.0, over_extra=None, emitters=None):
    over = {"hooves": ({"exposedFraction": round(1 - 0.6 * contact_frac, 3), "contact": {"fraction": round(0.6 * contact_frac, 3), "tempC": 18.0}} if contact_frac > 0 else {"exposedFraction": 0.8})}
    over.update(over_extra or {})
    return bind(regs, act, breathing, SUN, over, emitters)


def main():
    sk = make_skel()
    regs, askin = regions()
    A = round(askin, 4)
    basal = round(3.39 * MASS ** 0.75, 2)
    body = {"species": "goat", "massKg": MASS, "skinAreaM2": A, "basalMetabolismW": basal, "metWM2": round(basal / A, 2), "coreSetPointC": 39.1, "skinSetPointC": 35.5, "sweating": True, "panting": True, "maxSkinBloodFlowLM2H": 50, "minSkinBloodFlowLM2H": 4, "source": "authored-estimate", "reference": "kleiber-1932"}
    pats = [
        pattern("rest", T("Resting, nose breathing", "Ruhe, Nasenatmung"), 20, 0.3, (0.4, 0.05, 0.45, 0.1), "nose", "stahl-1967"),
        pattern("walk", T("Walking, nose breathing", "Gehen, Nasenatmung"), 36, 0.42, (0.42, 0.02, 0.5, 0.06), "nose", "stahl-1967"),
        pattern("browse", T("Browsing and chewing", "Äsen und Kauen"), 26, 0.34, (0.42, 0.03, 0.47, 0.08), "nose", "stahl-1967"),
        pattern("rear", T("Standing upright, deep breathing", "Aufrechtes Stehen, tiefe Atmung"), 32, 0.4, (0.42, 0.02, 0.5, 0.06), "nose", "stahl-1967"),
        pattern("pant", T("Heat panting, open mouth", "Hecheln bei Hitze, offenes Maul"), 160, 0.12, (0.5, 0.0, 0.5, 0.0), "nose-and-mouth", "silanikove-2000"),
    ]
    resp = respiration_block(pats, [{"bone": "thorax", "angleDeg": 0.8, "dy": 0.005}, {"bone": "lumbar", "angleDeg": 0.3, "dy": 0.003}], 33, 0.95, "schmidt-nielsen-1970")
    emitters = [
        {"id": "breath-nose", "kind": "breath", "title": T("Nostrils", "Nasenlöcher"), "anchor": "nose", "directionDeg": -55, "diameterM": 0.010, "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "breath-mouth", "kind": "breath", "title": T("Mouth (open-mouth panting)", "Maul (Hecheln mit offenem Maul)"), "anchor": "mouth", "directionDeg": -10, "diameterM": 0.018, "source": "authored-estimate", "reference": "silanikove-2000"},
        {"id": "plume-back", "kind": "plume", "title": T("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.35, "region": "coat-trunk", "source": "authored-estimate", "reference": "cena-monteith-1975"},
    ]
    st = standing_offsets(sk)
    up_offs, up_root, _ = upright_offsets(sk)
    lg_offs, lg_root = ledge_offsets(sk)
    four = ["contact-hn", "contact-fn", "contact-hf", "contact-ff"]
    poses = [
        pose("standing-alert", T("Standing alert", "Aufmerksam stehend"), T("Standing square on four hooves, head up, ears forward, short tail carried upright.", "Gerades Stehen auf vier Klauen, Kopf erhoben, Ohren nach vorn, kurzer Schwanz aufrecht getragen."), st, tp(regs, STAND, "rest", 1.0), four),
        pose("browsing-upright", T("Rearing up to browse", "Aufgerichtet beim Äsen"), T("Standing on the hind legs, body tilted about 60 degrees, forelegs raised and bent, neck stretched up to reach foliage at about 1.6 m; the centre of mass lies over the hind hooves.", "Auf den Hinterbeinen stehend, Körper etwa 60 Grad geneigt, Vorderbeine angehoben und gebeugt, Hals nach oben gestreckt, um Laub in etwa 1,6 m zu erreichen; der Schwerpunkt liegt über den Hinterklauen."), up_offs, tp(regs, 2.2, "rear", 1.0, {"coat-trunk": {"exposedFraction": 1.0, "sunlitFraction": 0.7}}), ["contact-hn", "contact-hf"], root={"y": r4(up_root[1])}),
        pose("climbing-ledge", T("Climbing, forefeet on a ledge", "Kletternd, Vorderklauen auf einem Absatz"), T("Hind hooves on the ground, forehooves placed on a rock ledge 0.30 m high, body tilted about 22 degrees, head forward: the posture of a goat climbing a step.", "Hinterklauen am Boden, Vorderklauen auf einem 0,30 m hohen Felsabsatz, Körper etwa 22 Grad geneigt, Kopf vorgestreckt: die Haltung einer Ziege beim Erklimmen einer Stufe."), lg_offs, tp(regs, 2.6, "walk", 1.0), four, root={"y": r4(lg_root[1])}),
    ]
    clips = []
    Gh = dict(ac0=-88, ks=10, flex=40, ah0=-52, ahroll=12, ahflex=14, lift=0.06)
    Gf = dict(ac0=-90, ks=8, flex=55, ah0=-53, ahroll=12, ahflex=14, lift=0.07, scap0=-62, scapamp=6)
    xs = {"hn": 0.03, "hf": 0.03, "fn": 0.70, "ff": 0.70}
    order = {"hn": 0.0, "fn": 0.25, "hf": 0.5, "ff": 0.75}
    T_, duty, speed, N = 1.1, 0.667, 0.5, 12

    def trunk(ph):
        return {"neck": r4(2.5 * math.sin(2 * math.pi * ph)), "head": r4(-2.0 * math.sin(2 * math.pi * ph + 0.6)), "tail-1": r4(5 * math.sin(2 * math.pi * ph * 2)), "ear-n": r4(4 * math.sin(2 * math.pi * ph * 2)), "ear-f": r4(4 * math.sin(2 * math.pi * ph * 2 + 0.5))}

    def ptitle(k, N):
        return T("Walk, phase %d of %d" % (k + 1, N), "Schritt, Phase %d von %d" % (k + 1, N)), T("Lateral-sequence walk at %d/%d of the cycle." % (k, N), "Schritt in Seitenfolge bei %d/%d des Zyklus." % (k, N))

    kp, clip, S = gait_clip(sk, "walk-cycle", T("Walk, four-beat", "Schritt im Viertakt"), T("Lateral-sequence walk after Hildebrand (1976) at about 0.5 m/s: near hind, near fore, far hind, far fore touch down at quarter-cycle intervals; each hoof is on the ground two thirds of the cycle.", "Schritt in Seitenfolge nach Hildebrand (1976) mit etwa 0,5 m/s: nahe Hinter-, nahe Vorder-, ferne Hinter-, ferne Vorderklaue setzen im Viertelzyklus auf; jede Klaue steht zwei Drittel des Zyklus am Boden."), ptitle, T_, N, duty, order, speed, Gh, Gf, {}, xs, lambda ph, c: tp(regs, WALK, "walk", len(c) / 4.0), FOOT, trunk_fn=trunk, names=LEGNAMES)
    clip["phases"] = [{"id": "stride-%d" % i, "fromS": r4(i * T_ / 4), "toS": r4((i + 1) * T_ / 4), "title": T("Quarter %d of the four-beat stride" % (i + 1), "Viertel %d des Viertakt-Schrittes" % (i + 1))} for i in range(4)]
    clip["tracks"] = [{"target": "breathing", "ease": "step", "keys": [{"t": 0.0, "v": 1}]}]
    clip["advanceM"] = 0.5489
    poses.extend(kp)
    clips.append(clip)
    ch = lambda t, a: {"jaw": {"angleDeg": a}}
    keys = [
        {"t": 0.0, "pose": "standing-alert", "ease": "smooth"},
        {"t": 1.2, "pose": "standing-alert", "ease": "smooth", "bones": {"pelvis": {"angleDeg": 30}, "neck": {"angleDeg": 14}}},
        {"t": 2.4, "pose": "browsing-upright", "ease": "smooth"},
        {"t": 3.0, "pose": "browsing-upright", "ease": "smooth", "bones": {"jaw": {"angleDeg": -9}, "head": {"angleDeg": 6}}},
        {"t": 3.6, "pose": "browsing-upright", "ease": "smooth", "bones": {"jaw": {"angleDeg": 0}, "head": {"angleDeg": -4}}},
        {"t": 4.2, "pose": "browsing-upright", "ease": "smooth", "bones": {"jaw": {"angleDeg": -9}, "head": {"angleDeg": 6}}},
        {"t": 5.0, "pose": "browsing-upright", "ease": "smooth"},
        {"t": 6.2, "pose": "standing-alert", "ease": "smooth", "bones": {"pelvis": {"angleDeg": 30}, "neck": {"angleDeg": 14}}},
        {"t": 7.0, "pose": "standing-alert", "ease": "smooth"},
    ]
    clips.append({"id": "browse-rear", "title": T("Rearing up to browse and coming down", "Aufrichten zum Äsen und Absetzen"), "description": T("The goat rises onto its hind legs, raising the forelegs (forehooves leave the ground at 1.2 s), browses and chews for about two seconds, then comes down and stands alert again.", "Die Ziege richtet sich auf die Hinterbeine auf und hebt die Vorderbeine (die Vorderklauen verlassen den Boden bei 1,2 s), äst und kaut etwa zwei Sekunden, setzt sich dann ab und steht wieder aufmerksam."), "durationS": 7.0, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": [{"anchor": "contact-hn", "fromS": 0.0, "toS": 7.0}, {"anchor": "contact-hf", "fromS": 0.0, "toS": 7.0}, {"anchor": "contact-fn", "fromS": 0.0, "toS": 1.2}, {"anchor": "contact-ff", "fromS": 0.0, "toS": 1.2}, {"anchor": "contact-fn", "fromS": 6.2, "toS": 7.0}, {"anchor": "contact-ff", "fromS": 6.2, "toS": 7.0}], "phases": [{"id": "rise", "fromS": 0.0, "toS": 2.4, "title": T("Rising", "Aufrichten")}, {"id": "browse", "fromS": 2.4, "toS": 5.0, "title": T("Browsing upright", "Aufrecht äsend")}, {"id": "descend", "fromS": 5.0, "toS": 7.0, "title": T("Coming down", "Absetzen")}], "events": [{"t": 1.2, "kind": "contact-off", "title": T("Forehooves leave the ground", "Vorderklauen heben ab"), "target": "contact-fn"}, {"t": 6.2, "kind": "contact-on", "title": T("Forehooves touch down", "Vorderklauen setzen auf"), "target": "contact-fn"}], "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0.0, "v": 1.3}, {"t": 2.4, "v": 2.2}, {"t": 5.0, "v": 2.2}, {"t": 7.0, "v": 1.3}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0.0, "v": 0}, {"t": 1.2, "v": 3}, {"t": 5.0, "v": 0}]}]})
    shp = {"500": GS.shapes500(sk), "200": GS.shapes200(sk), "100": GS.shapes100(sk), "50": GS.shapes50(sk)}
    return sk, regs, A, body, resp, emitters, poses, clips, shp


def reps(shp):
    r = {}
    r["500"] = authored(500, shp["500"], D8(
        ("One composed side silhouette of 2.3 × 2.3 mm: hindquarters and chest as two overlapping masses, neck wedge, one head mass, ear and horn strokes; four legs as strokes along the limb bones.", "Eine zusammengesetzte Seitensilhouette von 2,3 × 2,3 mm: Hinterhand und Brust als zwei überlappende Massen, Halskeil, eine Kopfmasse, Ohr- und Hornstriche; vier Beine als Striche entlang der Gliedmaßenknochen."),
        ("Long legs under a lean barrel, the long neck, the swept-back horn stroke and the upright ear as the identity marks of the goat.", "Lange Beine unter schlanker Tonne, der lange Hals, der zurückgeschwungene Hornstrich und das aufrechte Ohr als Erkennungsmerkmale der Ziege."),
        ("Thigh, shoulder and upper arm into the barrel masses, jaw and beard into the head; tail into the hindquarter mass; hair legs and hooves into one stroke per lower leg. All regions are listed on the merged masses.", "Oberschenkel, Schulter und Oberarm in die Tonnenmassen, Kiefer und Bart in den Kopf; Schwanz in die Hinterhandmasse; Haarbeine und Klauen in einen Strich je Unterbein. Alle Regionen stehen an den verschmolzenen Massen."),
        ("Eye, mouth, beard, hooves as shapes, the second ear and horn: the head is 0.23 m = 0.46 mm long.", "Auge, Maul, Bart, Klauen als Formen, zweites Ohr und Horn: der Kopf ist 0,23 m = 0,46 mm lang."),
        ("Legs (real 0.03 m = 0.06 mm), horn (0.03 m) and ear are 0.13-0.2 mm strokes; the 0.13 mm outline adds the same width, so they stay inside the printed line weight.", "Beine (real 0,03 m = 0,06 mm), Horn (0,03 m) und Ohr sind 0,13-0,2-mm-Striche; die 0,13-mm-Kontur trägt dieselbe Breite auf, sie bleiben innerhalb der gedruckten Strichstärke."),
        ("None.", "Keine."),
        ("Mid-grey masses with a 0.13 mm outline, far legs lighter, horn and ear as dark strokes.", "Mittelgraue Massen mit 0,13-mm-Kontur, ferne Beine heller, Horn und Ohr als dunkle Striche."),
        ("Merged masses are painted with the area-weighted value of the listed regions (labelled merged); horns and ears appear only as strokes in their own colour.", "Verschmolzene Massen werden mit dem flächengewichteten Wert der genannten Regionen gefärbt (als verschmolzen gekennzeichnet); Hörner und Ohren erscheinen nur als Striche in eigener Farbe.")))
    r["200"] = authored(200, shp["200"], D8(
        ("Main masses separated, 5.8 × 5.7 mm: hindquarters, chest, neck, head, ear, horn, tail, thigh, and per side a gaskin or forearm, cannon and hoof.", "Hauptmassen getrennt, 5,8 × 5,7 mm: Hinterhand, Brust, Hals, Kopf, Ohr, Horn, Schwanz, Oberschenkel und je Seite Unterschenkel oder Unterarm, Röhre und Klaue."),
        ("Long legs with hock angle, the horn blade swept back, the upright ear, the upturned tail and the near thigh as an inner mass.", "Lange Beine mit Sprunggelenkwinkel, die zurückgeschwungene Hornklinge, das aufrechte Ohr, der aufgestellte Schwanz und der nahe Oberschenkel als innere Masse."),
        ("Jaw and beard into the head; shoulder and upper arm into the chest; far thigh into the hindquarters; both ears into the near ear; second tail bone into the tail.", "Kiefer und Bart in den Kopf; Schulter und Oberarm in die Brust; ferner Oberschenkel in die Hinterhand; beide Ohren in das nahe Ohr; zweiter Schwanzknochen in den Schwanz."),
        ("Eye, mouth, fold lines, hoof cleft, far horn.", "Auge, Maul, Faltenlinien, Klauenspalte, fernes Horn."),
        ("Cannons (0.03 m = 0.15 mm) are drawn 0.048 m wide to reach 0.2 mm; 0.018 m wider than real.", "Röhren (0,03 m = 0,15 mm) sind 0,048 m breit gezeichnet, um 0,2 mm zu erreichen; 0,018 m breiter als real."),
        ("None; no hatch fits a 6 mm figure at the profile minimum.", "Keine; auf einer 6 mm großen Figur passt kein Schraffurabstand ab dem Profilminimum."),
        ("Mid-grey solids with a 0.18 mm outline; far side lighter, horn darkest.", "Mittelgraue Flächen mit 0,18-mm-Kontur; ferne Seite heller, Horn am dunkelsten."),
        ("Each mass carries its region (coat, upper leg, lower leg, hooves, ears, horn, tail); the head carries the face.", "Jede Masse trägt ihre Region (Fell, Oberbein, Unterbein, Klauen, Ohren, Horn, Schwanz); der Kopf trägt das Gesicht.")))
    r["100"] = authored(100, shp["100"], D8(
        ("Articulated outline, 11.6 × 11.4 mm: coat masses with a near thigh and shoulder shape, neck, head with eye, both ears and horns, tail, per side gaskin or forearm, cannon and hoof wedge.", "Gegliederter Umriss, 11,6 × 11,4 mm: Fellmassen mit nahem Oberschenkel und Schulter, Hals, Kopf mit Auge, beide Ohren und Hörner, Schwanz, je Seite Unterschenkel oder Unterarm, Röhre und Klauenkeil."),
        ("Thigh and shoulder muscle masses, hock and carpus angles, the horn blades, both ears, the eye and the hoof wedge.", "Oberschenkel- und Schultermuskelmassen, Sprung- und Vorderfusswurzelwinkel, die Hornklingen, beide Ohren, das Auge und der Klauenkeil."),
        ("Jaw and beard into the head; both tail bones into one tail; upper arm into the chest.", "Kiefer und Bart in den Kopf; beide Schwanzknochen zu einem Schwanz; Oberarm in die Brust."),
        ("Mouth line, nostril, beard, hoof cleft.", "Maullinie, Nüstern, Bart, Klauenspalte."),
        ("Cannons 0.03-0.035 m = 0.3 mm at real width; horns 0.03 m = 0.3 mm.", "Röhren 0,03-0,035 m = 0,3 mm in realer Breite; Hörner 0,03 m = 0,3 mm."),
        ("None; short hair needs no material indication.", "Keine; Kurzhaar braucht keine Materialangabe."),
        ("Mid-grey solids, 0.25 mm outline; far side and far ear lighter, horns darkest; the eye is the only black detail.", "Mittelgraue Flächen, 0,25-mm-Kontur; ferne Seite und fernes Ohr heller, Hörner am dunkelsten; das Auge ist das einzige schwarze Detail."),
        ("One region per mass; the thigh and shoulder shapes carry the upper-leg coat so the leg regions read apart from the trunk.", "Eine Region je Masse; Oberschenkel- und Schulterform tragen das Oberbeinfell, die Beinregionen lesen sich getrennt vom Rumpf.")))
    r["50"] = authored(50, shp["50"], D8(
        ("Full articulated drawing, 23 × 23 mm: trunk coat in two masses with near thigh and shoulder muscle shapes, neck, head with eye, mouth line, jaw and beard, both ears, both horns, tail in two parts, per side gaskin and forearm, cannon and cloven hoof wedge.", "Voll gegliederte Zeichnung, 23 × 23 mm: Rumpffell in zwei Massen mit nahem Oberschenkel und Schultermuskelformen, Hals, Kopf mit Auge, Maullinie, Unterkiefer und Bart, beide Ohren, beide Hörner, Schwanz in zwei Teilen, je Seite Unterschenkel und Unterarm, Röhre und Klauenkeil."),
        ("Everything of the 1:100 drawing plus the jaw with beard (so browsing and chewing show), two tail parts, mouth line and a hock mark.", "Alles der 1:100-Zeichnung plus Unterkiefer mit Bart (damit Äsen und Kauen sichtbar sind), zwei Schwanzteile, Maullinie und eine Sprunggelenkmarke."),
        ("Upper arm lies inside the chest; the second thigh is hidden behind the first (its bone still moves the far gaskin).", "Der Oberarm liegt in der Brust; der zweite Oberschenkel ist hinter dem ersten verdeckt (sein Knochen bewegt weiter den fernen Unterschenkel)."),
        ("Nostril, toes, hoof cleft and hair strokes; the head is 5 mm long.", "Nüstern, Zehen, Klauenspalte und Haarstriche; der Kopf ist 5 mm lang."),
        ("Cannons 0.03 m = 0.6 mm and horns 0.03 m = 0.6 mm at real size; ear 0.05 m = 1 mm.", "Röhren 0,03 m = 0,6 mm und Hörner 0,03 m = 0,6 mm in realer Größe; Ohr 0,05 m = 1 mm."),
        ("None: the short summer coat is left white; texture would mimic fleece.", "Keine: das kurze Sommerfell bleibt weiß; Textur würde Vlies nachahmen."),
        ("Mid-grey solids with 0.35 mm outline, 0.18 mm detail strokes; far side lighter, horns darkest.", "Mittelgraue Flächen mit 0,35-mm-Kontur, 0,18-mm-Detailstriche; ferne Seite heller, Hörner am dunkelsten."),
        ("Each region keeps its own shape; horns and ears are separate thermal windows; the hock mark carries the upper-leg region.", "Jede Region behält ihre eigene Form; Hörner und Ohren sind eigene thermische Fenster; die Sprunggelenkmarke trägt die Oberbeinregion.")))
    return r


def finish(args):
    sk, regs, A, body, resp, emitters, poses, clips, shp = main()
    mats = [
        {"id": "short-coat", "title": T("Short summer coat", "Kurzes Sommerfell"), "emissivity": 0.96, "solarAbsorptivity": 0.6, "conductivityWmK": 0.05, "source": "authored-estimate", "reference": "cena-monteith-1975"},
        {"id": "bare-skin", "title": T("Bare skin", "Nackte Haut"), "emissivity": 0.98, "solarAbsorptivity": 0.65, "source": "literature", "reference": "ashrae-2021"},
        {"id": "horn", "title": T("Keratin horn and hoof", "Keratinhorn und Klaue"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "source": "authored-estimate"},
    ]
    env = {"airTempC": 20, "meanRadiantTempC": 22, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325}
    assumptions = [
        T("Skin area 0.095·M^(2/3) = %.3f m² for 55 kg is an authored estimate guided by Brody (1945); the nine regions cover it." % A, "Die Hautfläche 0,095·M^(2/3) = %.3f m² für 55 kg ist ein Schätzwert nach Brody (1945); die neun Regionen decken sie ab." % A),
        T("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = %.1f W (verified coefficient); one met is that rate per skin area. Standing 1.3, browsing upright 2.2, climbing 2.6 and walking at 0.5 m/s (3.2 met) follow the cost of locomotion after Taylor et al. (1970); the multiples are authored estimates." % body["basalMetabolismW"], "Der Grundumsatz ist Kleibers 3,39 W je kg^0,75 = %.1f W (verifizierter Koeffizient); ein Met ist diese Rate je Hautfläche. Stehen 1,3, Äsen aufrecht 2,2, Klettern 2,6 und Gehen mit 0,5 m/s (3,2 Met) folgen dem Fortbewegungsaufwand nach Taylor et al. (1970); die Vielfachen sind Schätzwerte." % body["basalMetabolismW"]),
        T("A short summer coat of 4-12 mm with conductivity 0.05 W/(m K) is an authored estimate; the thin coat makes goats lose heat easily, horns and ears are bare thermal windows (guided by Silanikove 2000, no numbers taken over).", "Ein kurzes Sommerfell von 4-12 mm mit der Leitfähigkeit 0,05 W/(m K) ist ein Schätzwert; das dünne Fell lässt Ziegen leicht Wärme abgeben, Hörner und Ohren sind nackte thermische Fenster (nach Silanikove 2000, keine Zahlen übernommen)."),
        T("Respiratory rates (rest 20, walking 36, heat panting 160 per minute) and tidal volumes are authored estimates (the goat is not in the verified resting-rate table; allometric guidance Stahl 1967, panting guidance Silanikove 2000). Goats sweat moderately and pant; both are enabled.", "Atemfrequenzen (Ruhe 20, Gehen 36, Hecheln 160 je Minute) und Atemzugvolumina sind Schätzwerte (die Ziege fehlt in der verifizierten Ruhefrequenztabelle; allometrische Orientierung Stahl 1967, Hecheln Silanikove 2000). Ziegen schwitzen mäßig und hecheln; beides ist aktiviert."),
    ]
    asset = {
        "id": "goat", "category": "animals", "title": T("Goat (Horned Doe)", "Ziege (gehörnte Geiß)"),
        "summary": T("An adult horned goat of 55 kg with 0.76 m withers height: lean barrel on long legs, swept-back horns, beard and upright ears; standing alert, rearing on the hind legs to browse foliage, climbing a ledge, and a four-beat walk; short summer coat, sweating and panting, bare horns and ears as thermal windows.", "Eine erwachsene gehörnte Ziege von 55 kg mit 0,76 m Widerristhöhe: schlanke Tonne auf langen Beinen, zurückgeschwungene Hörner, Bart und aufrechte Ohren; aufmerksames Stehen, Aufrichten auf die Hinterbeine zum Äsen von Laub, Klettern über einen Absatz und ein Viertakt-Schritt; kurzes Sommerfell, Schwitzen und Hecheln, nackte Hörner und Ohren als thermische Fenster."),
        "tags": ["goat", "horned", "livestock", "browsing", "climbing", "quadruped", "ruminant", "hind-legs"],
        "provenance": {"author": "Semio staffage content worker NW-A1", "method": "handcrafted", "created": "2026-10-29", "references": [REFS[k] for k in KEEP], "notes": T("Side elevation of the near (left) side of a goat facing right; far limbs, far ear and far horn are separate chains drawn lighter and behind. Proportions, joint angles, masses and thermal parameters are authored estimates; only Kleiber's coefficient is a verified literature value. The rearing pose is balanced statically (centre of mass over the hind hooves); four preset scales are individually drawn.", "Seitenansicht der nahen (linken) Seite einer nach rechts blickenden Ziege; ferne Gliedmaßen, fernes Ohr und fernes Horn sind eigene Ketten, heller und dahinter gezeichnet. Proportionen, Gelenkwinkel, Massen und Wärmeparameter sind Schätzwerte; nur Kleibers Koeffizient ist ein verifizierter Literaturwert. Die aufgerichtete Pose ist statisch ausbalanciert (Schwerpunkt über den Hinterklauen); vier Maßstäbe sind einzeln gezeichnet.")},
        "dimensions": {"heightM": args[0], "widthM": args[1], "depthM": 0.34, "massKg": MASS},
        "materials": mats, "rig": {"kind": "anatomical", "root": "pelvis", "bones": finish_bones(sk)}, "anchors": sk.anchors,
        "representations": reps(shp), "symbols": [], "poses": poses, "clips": clips,
        "thermal": {"environment": env, "baseMet": STAND, "body": body, "respiration": resp, "passengers": [], "operating": [], "regions": regs, "emitters": emitters, "assumptions": assumptions},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [
            T("Segments are rigid with authored masses summing to 55 kg; contacts come from pose contacts and clip spans: four hooves standing, the two hind hooves when rearing, hind hooves plus ledge-resting forehooves when climbing.", "Segmente sind starr mit Massen von zusammen 55 kg; Kontakte kommen aus Posenkontakten und Clip-Spannen: vier Klauen im Stand, die zwei Hinterklauen beim Aufrichten, Hinterklauen plus aufliegende Vorderklauen beim Klettern."),
            T("Joint moments are the moments of all distal weights and reactions about each joint; no muscle model is assumed.", "Gelenkmomente sind die Momente aller distalen Gewichte und Reaktionen um jedes Gelenk; ein Muskelmodell wird nicht angenommen.")], "clinical": False},
        "presentation": {"defaultPose": "standing-alert", "defaultClip": "walk-cycle", "viewpoints": ["elevation"], "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored",
    }
    msgs = verify_poses(sk, asset, shp)
    print("\n".join(msgs))
    return write(asset, "goat")


if __name__ == "__main__":
    print(finish((float(sys.argv[1]), float(sys.argv[2])) if len(sys.argv) > 2 else (1.14, 1.32)))
