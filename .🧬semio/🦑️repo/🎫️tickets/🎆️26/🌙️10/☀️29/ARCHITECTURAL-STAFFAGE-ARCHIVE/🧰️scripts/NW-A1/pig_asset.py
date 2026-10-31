# [scratch] NW-A1 pig asset document
import sys
from pig import *
from pig_poses import *
from assemble import *
from refs import REFS
import pig_shapes as PS

KEEP = ["kleiber-1932", "merck-resting-respiratory-rates", "hildebrand-1976", "taylor-1970", "brody-1945", "stahl-1967", "schmidt-nielsen-1970", "gupta-2010", "ashrae-2021", "ingram-1965", "bracke-2011", "huynh-2005", "cena-monteith-1975"]
STAND, WALK = 1.3, 2.8
SUN = {"*": 0.55, "skin-legs": 0.25, "hooves": 0.1, "ears": 0.6}
LEGNAMES = {"hn": ("Near hind hoof", "Nahe Hinterklaue"), "fn": ("Near fore hoof", "Nahe Vorderklaue"), "hf": ("Far hind hoof", "Ferne Hinterklaue"), "ff": ("Far fore hoof", "Ferne Vorderklaue")}


def regions():
    B = lambda t_en, t_de, th: Lay("bristles", T(t_en, t_de), "hair", "bristle", th, 0.08, 1.0, 40, 0.5, "authored-estimate", "cena-monteith-1975")
    R_ = []
    R_.append(Reg("skin-trunk", T("Trunk and neck skin with sparse bristles", "Rumpf- und Halshaut mit spärlichen Borsten"), "skin", "pig-skin", 1.21, 1.0, [B("Sparse bristles, 3 mm", "Spärliche Borsten, 3 mm", 0.003)], "cylinder", 0.42, 0.55, 28000, "trunk", 0.64, "authored-estimate", "brody-1945"))
    R_.append(Reg("skin-legs", T("Leg skin", "Beinhaut"), "skin", "pig-skin", 0.2, 1.0, [B("Sparse bristles, 3 mm", "Spärliche Borsten, 3 mm", 0.003)], "cylinder", 0.06, 0.25, 12000, "leg", 0.07, "authored-estimate", "brody-1945"))
    R_.append(Reg("skin-head", T("Head skin", "Kopfhaut"), "skin", "pig-skin", 0.10, 1.0, [B("Sparse bristles, 3 mm", "Spärliche Borsten, 3 mm", 0.003)], "cylinder", 0.17, 0.5, 14000, "head", 0.1, "authored-estimate", "brody-1945"))
    R_.append(Reg("snout", T("Moist snout disc", "Feuchte Rüsselscheibe"), "skin", "pig-skin", 0.012, 1.0, [], "vertical-plate", 0.05, 0.4, 6000, "head", 0.006, "authored-estimate", "ingram-1965"))
    R_.append(Reg("ears", T("Ears", "Ohren"), "skin", "pig-skin", 0.07, 1.0, [], "vertical-plate", 0.14, 0.6, 5000, "head", 0.02, "authored-estimate", "ingram-1965"))
    R_.append(Reg("tail", T("Tail", "Ringelschwanz"), "skin", "pig-skin", 0.01, 1.0, [], "cylinder", 0.03, 0.6, 6000, "tail", 0.004, "authored-estimate", "brody-1945"))
    R_.append(Reg("hooves", T("Hooves", "Klauen"), "skin", "horn", 0.018, 1.0, [], "horizontal-plate-down", 0.05, 0.0, 8000, "foot", 0.01, "authored-estimate", "brody-1945"))
    return finalize_regions(R_)


def tp(regs, act, breathing, contact_frac=0.0, over_extra=None):
    over = {"hooves": ({"exposedFraction": round(1 - 0.6 * contact_frac, 3), "contact": {"fraction": round(0.6 * contact_frac, 3), "tempC": 22.0}} if contact_frac > 0 else {"exposedFraction": 0.8})}
    over.update(over_extra or {})
    return bind(regs, act, breathing, SUN, over)


def main():
    sk = make_skel()
    regs, askin = regions()
    A = round(askin, 4)
    basal = round(3.39 * MASS ** 0.75, 2)
    body = {"species": "pig", "massKg": MASS, "skinAreaM2": A, "basalMetabolismW": basal, "metWM2": round(basal / A, 2), "coreSetPointC": 39.0, "skinSetPointC": 34.0, "sweating": False, "panting": True, "maxSkinBloodFlowLM2H": 55, "minSkinBloodFlowLM2H": 3, "source": "authored-estimate", "reference": "kleiber-1932"}
    pats = [
        pattern("rest", T("Resting, nose breathing", "Ruhe, Nasenatmung"), 36, 0.42, (0.4, 0.05, 0.45, 0.1), "nose", "merck-resting-respiratory-rates"),
        pattern("walk", T("Walking, nose breathing", "Gehen, Nasenatmung"), 50, 0.5, (0.42, 0.02, 0.5, 0.06), "nose", "stahl-1967"),
        pattern("root", T("Rooting and snuffling", "Wühlen und Schnüffeln"), 60, 0.3, (0.5, 0.0, 0.5, 0.0), "nose", "stahl-1967"),
        pattern("wallow", T("Lying in the wallow, calm", "Liegen im Suhle, ruhig"), 34, 0.42, (0.4, 0.05, 0.45, 0.1), "nose", "merck-resting-respiratory-rates"),
        pattern("pant", T("Heat panting, open mouth", "Hecheln bei Hitze, offenes Maul"), 140, 0.2, (0.5, 0.0, 0.5, 0.0), "nose-and-mouth", "ingram-1965"),
    ]
    resp = respiration_block(pats, [{"bone": "thorax", "angleDeg": 0.6, "dy": 0.005}, {"bone": "lumbar", "angleDeg": 0.5, "dy": 0.006}], 34, 0.95, "schmidt-nielsen-1970")
    emitters = [
        {"id": "breath-nose", "kind": "breath", "title": T("Nostrils in the snout disc", "Nüstern in der Rüsselscheibe"), "anchor": "nose", "directionDeg": -20, "diameterM": 0.016, "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "breath-mouth", "kind": "breath", "title": T("Mouth (open-mouth panting)", "Maul (Hecheln mit offenem Maul)"), "anchor": "mouth", "directionDeg": -10, "diameterM": 0.03, "source": "authored-estimate", "reference": "ingram-1965"},
        {"id": "plume-back", "kind": "plume", "title": T("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.45, "region": "skin-trunk", "source": "authored-estimate", "reference": "cena-monteith-1975"},
    ]
    st = standing_offsets(sk)
    rt, rt_root = rooting_offsets(sk)
    ly, ly_root = lying_offsets(sk)
    four = ["contact-hn", "contact-fn", "contact-hf", "contact-ff"]
    wet = {"skin-trunk": {"exposedFraction": 0.7, "sunlitFraction": 0.45, "wettedness": 0.9, "contact": {"fraction": 0.3, "tempC": 20.0}}, "skin-legs": {"exposedFraction": 0.6, "sunlitFraction": 0.1, "wettedness": 0.85, "contact": {"fraction": 0.3, "tempC": 20.0}}, "skin-head": {"exposedFraction": 1.0, "sunlitFraction": 0.5, "wettedness": 0.4}, "ears": {"exposedFraction": 1.0, "sunlitFraction": 0.6, "wettedness": 0.3}, "snout": {"exposedFraction": 1.0, "sunlitFraction": 0.5, "wettedness": 1.0}, "tail": {"exposedFraction": 1.0, "sunlitFraction": 0.5, "wettedness": 0.7}}
    poses = [
        pose("standing-pig", T("Standing", "Stehend"), T("Standing on four short legs, head carried low, snout forward, floppy ears over the eyes, tail curled.", "Stehen auf vier kurzen Beinen, Kopf tief getragen, Rüssel nach vorn, Schlappohren über den Augen, Ringelschwanz."), st, tp(regs, STAND, "rest", 1.0), four),
        pose("rooting", T("Rooting", "Wühlend"), T("Snout pressed to the ground: pelvis and neck tilted, hind legs flexed, the far fore hoof placed forward; the snout disc a hand's width above the soil.", "Rüssel am Boden: Becken und Hals gekippt, Hinterbeine gebeugt, fernes Vorderbein vorgesetzt; die Rüsselscheibe eine Handbreit über dem Boden."), rt, tp(regs, 1.8, "root", 1.0), four, root={"y": r4(rt_root[1])}),
        pose("lying-wallowing", T("Lying in the wallow", "Liegend in der Suhle"), T("Sternal recumbency in a mud wallow: legs tucked under the barrel, belly and brisket on the wet ground, skin coated with mud and wet (wettedness 0.9), head resting up.", "Brustlage in einer Schlammsuhle: Beine unter der Tonne eingeschlagen, Bauch und Brustbein auf nassem Boden, Haut mit Schlamm überzogen und nass (Benetzung 0,9), Kopf aufgestützt."), ly, bind(regs, 1.0, "wallow", SUN, wet), ["contact-belly", "contact-brisket"], root={"y": r4(ly_root[1])}),
    ]
    clips = []
    Gh = dict(ac0=-88, ks=9, flex=38, ah0=-58, ahroll=12, ahflex=14, lift=0.05)
    Gf = dict(ac0=-90, ks=8, flex=50, ah0=-60, ahroll=12, ahflex=14, lift=0.055, scap0=-62, scapamp=6)
    xs = {"hn": 0.03, "hf": 0.03, "fn": 0.68, "ff": 0.68}
    order = {"hn": 0.0, "fn": 0.25, "hf": 0.5, "ff": 0.75}
    T_, duty, speed, N = 1.0, 0.667, 0.42, 12

    def trunk(ph):
        return {"neck": r4(3.0 * math.sin(2 * math.pi * ph)), "head": r4(-3.0 * math.sin(2 * math.pi * ph + 0.8)), "tail-2": r4(8 * math.sin(2 * math.pi * ph * 2)), "ear-n": r4(4 * math.sin(2 * math.pi * ph * 2)), "ear-f": r4(4 * math.sin(2 * math.pi * ph * 2 + 0.4))}

    def ptitle(k, N):
        return T("Walk, phase %d of %d" % (k + 1, N), "Schritt, Phase %d von %d" % (k + 1, N)), T("Lateral-sequence walk at %d/%d of the cycle." % (k, N), "Schritt in Seitenfolge bei %d/%d des Zyklus." % (k, N))

    kp, clip, S = gait_clip(sk, "walk-cycle", T("Walk, four-beat", "Schritt im Viertakt"), T("Lateral-sequence walk after Hildebrand (1976) at about 0.4 m/s with the head swinging low: near hind, near fore, far hind, far fore touch down at quarter-cycle intervals; each hoof is on the ground two thirds of the cycle.", "Schritt in Seitenfolge nach Hildebrand (1976) mit etwa 0,4 m/s und tief pendelndem Kopf: nahe Hinter-, nahe Vorder-, ferne Hinter-, ferne Vorderklaue setzen im Viertelzyklus auf; jede Klaue steht zwei Drittel des Zyklus am Boden."), ptitle, T_, N, duty, order, speed, Gh, Gf, {}, xs, lambda ph, c: tp(regs, WALK, "walk", len(c) / 4.0), FOOT, trunk_fn=trunk, names=LEGNAMES)
    clip["phases"] = [{"id": "stride-%d" % i, "fromS": r4(i * T_ / 4), "toS": r4((i + 1) * T_ / 4), "title": T("Quarter %d of the four-beat stride" % (i + 1), "Viertel %d des Viertakt-Schrittes" % (i + 1))} for i in range(4)]
    clip["tracks"] = [{"target": "breathing", "ease": "step", "keys": [{"t": 0.0, "v": 1}]}]
    clip["advanceM"] = 0.4181
    poses.extend(kp)
    clips.append(clip)
    keys = []
    for i in range(8):
        keys.append({"t": r4(i * 0.5), "pose": "rooting", "ease": "smooth", "bones": {"head": {"angleDeg": 6.0 if i % 2 == 0 else -4.0}, "neck": {"angleDeg": 4.0 if i % 2 == 0 else -2.0}, "jaw": {"angleDeg": -4.0 if i % 2 else 0.0}}})
    clips.append({"id": "root-cycle", "title": T("Rooting and snuffling", "Wühlen und Schnüffeln"), "description": T("Pushing the snout through the soil with a short upward flick every second beat while snuffling (about 60 breaths per minute); all four hooves stay planted.", "Der Rüssel pflügt durch den Boden mit kurzem Aufwärtsruck jeden zweiten Takt beim Schnüffeln (etwa 60 Atemzüge je Minute); alle vier Klauen bleiben am Boden."), "durationS": 4.0, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": [{"anchor": a, "fromS": 0.0, "toS": 4.0} for a in four], "phases": [], "events": [], "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0.0, "v": 2}]}]})
    shp = {"500": PS.shapes500(sk), "200": PS.shapes200(sk), "100": PS.shapes100(sk), "50": PS.shapes50(sk)}
    return sk, regs, A, body, resp, emitters, poses, clips, shp


def reps(shp):
    H = [{"id": "bristles", "title": T("Sparse bristle dots", "Punkte für spärliche Borsten"), "pattern": "dots", "angleDeg": 0, "spacingMm": 1.6, "penMm": 0.1}]
    r = {}
    r["500"] = authored(500, shp["500"], D8(
        ("One composed side silhouette of 2.1 × 1.5 mm: ham-and-barrel as two overlapping masses, a short neck wedge, one wedge-shaped head with ear and snout; four short legs as strokes along the limb bones.", "Eine zusammengesetzte Seitensilhouette von 2,1 × 1,5 mm: Schinken-und-Tonne als zwei überlappende Massen, ein kurzer Halskeil, ein keilförmiger Kopf mit Ohr und Rüssel; vier kurze Beine als Striche entlang der Gliedmaßenknochen."),
        ("The deep barrel on very short legs, the nearly absent neck, the low wedge head and the high belly line that make the pig read as a pig.", "Die tiefe Tonne auf sehr kurzen Beinen, der kaum vorhandene Hals, der tiefe Keilkopf und die hohe Bauchlinie, die das Schwein als Schwein lesbar machen."),
        ("Ham, shoulder, neck, jaw, ears and snout into the barrel and head masses; the curly tail into the hindquarter mass; leg skin and hooves into one stroke per lower leg. All regions are listed on the merged masses.", "Schinken, Schulter, Hals, Kiefer, Ohren und Rüssel in die Tonnen- und Kopfmassen; der Ringelschwanz in die Hinterhandmasse; Beinhaut und Klauen in einen Strich je Unterbein. Alle Regionen stehen an den verschmolzenen Massen."),
        ("Eye, mouth, snout disc as a shape, hooves as shapes, bristles: the head is 0.3 m = 0.6 mm long.", "Auge, Maul, Rüsselscheibe als Form, Klauen als Formen, Borsten: der Kopf ist 0,3 m = 0,6 mm lang."),
        ("Legs (real 0.06 m = 0.12 mm) are 0.2 mm strokes, i.e. 0.1 m wide, 0.04 m wider than real; the 0.13 mm outline already adds 0.13 m of ink.", "Beine (real 0,06 m = 0,12 mm) sind 0,2-mm-Striche, also 0,1 m breit, 0,04 m breiter als real; die 0,13-mm-Kontur trägt ohnehin 0,13 m Tinte auf."),
        ("None.", "Keine."),
        ("Light-grey masses (pale skin) with a 0.13 mm outline; far legs lighter.", "Hellgraue Massen (helle Haut) mit 0,13-mm-Kontur; ferne Beine heller."),
        ("Merged masses are painted with the area-weighted value of the listed regions (labelled merged); ears, snout and tail cannot be told apart from the head and hindquarters.", "Verschmolzene Massen werden mit dem flächengewichteten Wert der genannten Regionen gefärbt (als verschmolzen gekennzeichnet); Ohren, Rüssel und Schwanz sind von Kopf und Hinterhand nicht zu unterscheiden.")))
    r["200"] = authored(200, shp["200"], D8(
        ("Main masses separated, 5.3 × 3.8 mm: hindquarters (with tail), chest, neck, head with snout, ear and per side gaskin or forearm, cannon and hoof.", "Hauptmassen getrennt, 5,3 × 3,8 mm: Hinterhand (mit Schwanz), Brust, Hals, Kopf mit Rüssel, Ohr und je Seite Unterschenkel oder Unterarm, Röhre und Klaue."),
        ("The round barrel, short thick legs with hock angle, the drooping ear and the hoof marks so the gait reads.", "Die runde Tonne, kurze dicke Beine mit Sprunggelenkwinkel, das hängende Ohr und die Klauenmarken, damit der Gang lesbar bleibt."),
        ("Jaw into the head, snout disc into the head (listed), tail into the hindquarters (listed), shoulder and ham into the barrel, both ears into the near ear.", "Kiefer in den Kopf, Rüsselscheibe in den Kopf (aufgeführt), Schwanz in die Hinterhand (aufgeführt), Schulter und Schinken in die Tonne, beide Ohren in das nahe Ohr."),
        ("Eye, mouth, ham outline, curly tail.", "Auge, Maul, Schinkenumriss, Ringelschwanz."),
        ("Cannons (0.04-0.05 m = 0.2-0.25 mm) reach the 0.2 mm feature minimum at real width; the tail (0.025 m = 0.13 mm) is below it and is merged.", "Röhren (0,04-0,05 m = 0,2-0,25 mm) erreichen das 0,2-mm-Mindestmerkmal in realer Breite; der Schwanz (0,025 m = 0,13 mm) liegt darunter und ist verschmolzen."),
        ("None; no hatch fits at the profile minimum.", "Keine; kein Schraffurabstand ab dem Profilminimum passt."),
        ("Light-grey solids with a 0.18 mm outline; far side lighter, ear lightest.", "Hellgraue Flächen mit 0,18-mm-Kontur; ferne Seite heller, Ohr am hellsten."),
        ("Each mass carries its region; the head carries head skin and snout together.", "Jede Masse trägt ihre Region; der Kopf trägt Kopfhaut und Rüssel gemeinsam.")))
    r["100"] = authored(100, shp["100"], D8(
        ("Articulated outline, 10.6 × 7.6 mm: ham shape, shoulder-less barrel in two masses, short neck, head with eye and snout disc, near and far ears, curly tail, per side gaskin or forearm, cannon and hoof wedge.", "Gegliederter Umriss, 10,6 × 7,6 mm: Schinkenform, schulterlose Tonne in zwei Massen, kurzer Hals, Kopf mit Auge und Rüsselscheibe, nahes und fernes Ohr, Ringelschwanz, je Seite Unterschenkel oder Unterarm, Röhre und Klauenkeil."),
        ("The ham as an inner mass (hind leg muscling), the snout disc, the drooping ears, hock and fetlock angles, hoof wedges, the curl of the tail.", "Der Schinken als innere Masse (Hinterbeinmuskulatur), die Rüsselscheibe, die hängenden Ohren, Sprung- und Fesselwinkel, Klauenkeile, die Ringelung des Schwanzes."),
        ("Shoulder into the barrel, jaw into the head, both tail bones into one stroke-like tail.", "Schulter in die Tonne, Kiefer in den Kopf, beide Schwanzknochen zu einem strichartigen Schwanz."),
        ("Mouth line, bristles, hoof cleft, dewclaws.", "Maullinie, Borsten, Klauenspalte, Afterklauen."),
        ("Cannons 0.04-0.05 m = 0.4-0.5 mm at real width; tail 0.025 m = 0.25 mm.", "Röhren 0,04-0,05 m = 0,4-0,5 mm in realer Breite; Schwanz 0,025 m = 0,25 mm."),
        ("None at this scale: the sparse bristles are carried by the 1:50 stipple only.", "Keine in diesem Maßstab: die spärlichen Borsten trägt nur das Punktraster bei 1:50."),
        ("Light-grey solids with a 0.25 mm outline, far side lighter; the eye is the only black detail and the snout disc is darker.", "Hellgraue Flächen mit 0,25-mm-Kontur, ferne Seite heller; das Auge ist das einzige schwarze Detail, die Rüsselscheibe dunkler."),
        ("One region per mass; the ham carries the trunk skin region as a visible inner shape.", "Eine Region je Masse; der Schinken trägt die Rumpfhautregion als sichtbare Innenform.")))
    r["50"] = authored(50, shp["50"], D8(
        ("Full articulated drawing, 21 × 15 mm: barrel in two masses with ham and shoulder shapes, bristle dots, short neck, head with eye, mouth line, jaw and snout disc, both ears, two-part curly tail, per side gaskin and forearm, cannon and cloven hoof.", "Voll gegliederte Zeichnung, 21 × 15 mm: Tonne in zwei Massen mit Schinken- und Schulterformen, Borstenpunkte, kurzer Hals, Kopf mit Auge, Maullinie, Unterkiefer und Rüsselscheibe, beide Ohren, zweiteiliger Ringelschwanz, je Seite Unterschenkel und Unterarm, Röhre und Klaue."),
        ("Everything of the 1:100 drawing plus the jaw (so snuffling and panting show), two tail parts, the mouth line and the bristle stipple.", "Alles der 1:100-Zeichnung plus Unterkiefer (damit Schnüffeln und Hecheln sichtbar sind), zwei Schwanzteile, die Maullinie und das Borstenraster."),
        ("The upper arm lies inside the barrel; the far ham is hidden behind the near one.", "Der Oberarm liegt in der Tonne; der ferne Schinken ist hinter dem nahen verdeckt."),
        ("Nostril holes, eyelashes, individual toes and dewclaws are not drawn; the head is 6 mm long.", "Nüstern, Wimpern, einzelne Zehen und Afterklauen sind nicht gezeichnet; der Kopf ist 6 mm lang."),
        ("Cannons 0.04-0.05 m = 0.8-1 mm at real size; tail 0.025 m = 0.5 mm.", "Röhren 0,04-0,05 m = 0,8-1 mm in realer Größe; Schwanz 0,025 m = 0,5 mm."),
        ("Sparse dot stipple (1.6 mm spacing, 0.1 mm pen) on barrel, ham and shoulder only: the bristles are sparse, so the skin stays mostly white; legs, head, ears and tail are clean.", "Spärliches Punktraster (1,6 mm Abstand, 0,1 mm Stift) nur auf Tonne, Schinken und Schulter: die Borsten sind spärlich, die Haut bleibt überwiegend weiß; Beine, Kopf, Ohren und Schwanz sind sauber."),
        ("Light-grey solids with a 0.35 mm outline, 0.18 mm detail strokes, 0.1 mm stipple; far side lighter, snout disc darkest.", "Hellgraue Flächen mit 0,35-mm-Kontur, 0,18-mm-Detailstriche, 0,1-mm-Punktraster; ferne Seite heller, Rüsselscheibe am dunkelsten."),
        ("The ham and shoulder shapes carry the trunk skin so that wet and dry skin can be read as separate patches; snout and ears are their own thermal windows.", "Schinken- und Schulterform tragen die Rumpfhaut, nasse und trockene Haut lässt sich als getrennte Flächen lesen; Rüssel und Ohren sind eigene thermische Fenster.")), hatches=H)
    return r


def finish(args):
    sk, regs, A, body, resp, emitters, poses, clips, shp = main()
    mats = [
        {"id": "pig-skin", "title": T("Pale pig skin", "Helle Schweinehaut"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "source": "authored-estimate", "reference": "ingram-1965"},
        {"id": "bristle", "title": T("Sparse bristles", "Spärliche Borsten"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "conductivityWmK": 0.08, "source": "authored-estimate", "reference": "cena-monteith-1975"},
        {"id": "horn", "title": T("Hoof horn", "Klauenhorn"), "emissivity": 0.9, "solarAbsorptivity": 0.75, "source": "authored-estimate"},
    ]
    env = {"airTempC": 28, "meanRadiantTempC": 32, "skyTempC": 12, "groundTempC": 24, "relativeHumidity": 0.5, "windMS": 0.5, "windDirectionDeg": 180, "solarWM2": 700, "diffuseFraction": 0.2, "pressurePa": 101325}
    assumptions = [
        T("Skin area 0.075·M^(2/3) = %.2f m² for 100 kg is an authored estimate guided by Brody (1945); the seven regions cover it. The thick back fat is not modelled as a layer: it enters through the low minimum skin blood flow (3 L/(m² h))." % A, "Die Hautfläche 0,075·M^(2/3) = %.2f m² für 100 kg ist ein Schätzwert nach Brody (1945); die sieben Regionen decken sie ab. Das dicke Rückenfett ist nicht als Schicht modelliert: es geht über den niedrigen Mindest-Hautdurchfluss (3 L/(m² h)) ein." % A),
        T("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = %.1f W (verified coefficient); one met is that rate per skin area. Standing 1.3, rooting 1.8, lying 1.0 and the slow walk at 0.4 m/s (2.8 met) follow the cost of locomotion after Taylor et al. (1970); the multiples are authored estimates." % body["basalMetabolismW"], "Der Grundumsatz ist Kleibers 3,39 W je kg^0,75 = %.1f W (verifizierter Koeffizient); ein Met ist diese Rate je Hautfläche. Stehen 1,3, Wühlen 1,8, Liegen 1,0 und der langsame Schritt mit 0,4 m/s (2,8 Met) folgen dem Fortbewegungsaufwand nach Taylor et al. (1970); die Vielfachen sind Schätzwerte." % body["basalMetabolismW"]),
        T("Pigs have no functional sweat glands and bristles insulate hardly at all (3 mm, 0.08 W/(m K), authored estimates): heat is lost from the bare skin, by panting and by wetting the skin, so sweating is off and panting on (guidance Ingram 1965, Huynh et al. 2005); wallowing coats the skin with wet mud, bound as wettedness 0.9 and contact with 20 °C mud in the lying pose (Bracke 2011).", "Schweine haben keine funktionierenden Schweißdrüsen und die Borsten dämmen kaum (3 mm, 0,08 W/(m K), Schätzwerte): Wärme wird über die nackte Haut, durch Hecheln und durch Benetzen der Haut abgegeben, daher Schwitzen aus und Hecheln an (nach Ingram 1965, Huynh et al. 2005); Suhlen überzieht die Haut mit nassem Schlamm, als Benetzung 0,9 und Kontakt mit 20 °C kaltem Schlamm in der Liegepose gebunden (Bracke 2011)."),
        T("Resting rate 36 per minute lies inside the verified resting range of pigs (32-58, Merck Veterinary Manual after Reece 2004); tidal volumes (0.42 L at rest) and the panting pattern (140 per minute, 0.2 L) are authored estimates.", "Die Ruhefrequenz von 36 je Minute liegt im verifizierten Ruhebereich von Schweinen (32-58, Merck Veterinary Manual nach Reece 2004); Atemzugvolumina (0,42 L in Ruhe) und das Hechelmuster (140 je Minute, 0,2 L) sind Schätzwerte."),
    ]
    asset = {
        "id": "pig", "category": "animals", "title": T("Domestic Pig (Finishing Pig)", "Hausschwein (Mastschwein)"),
        "summary": T("A finishing pig of 100 kg with 0.64 m withers height: deep barrel on very short legs, nearly no neck, wedge head with snout disc, floppy ears, curly tail and sparse bristles on pale skin; standing, rooting with the snout down, lying wet in a mud wallow, and a slow four-beat walk; no sweat glands, panting and wallowing for cooling.", "Ein Mastschwein von 100 kg mit 0,64 m Widerristhöhe: tiefe Tonne auf sehr kurzen Beinen, kaum Hals, Keilkopf mit Rüsselscheibe, Schlappohren, Ringelschwanz und spärliche Borsten auf heller Haut; Stehen, Wühlen mit gesenktem Rüssel, nasses Liegen in der Schlammsuhle und ein langsamer Viertakt-Schritt; keine Schweißdrüsen, Kühlung durch Hecheln und Suhlen."),
        "tags": ["pig", "swine", "livestock", "wallowing", "rooting", "quadruped", "sparse-hair", "wet-skin"],
        "provenance": {"author": "Semio staffage content worker NW-A1", "method": "handcrafted", "created": "2026-10-29", "references": [REFS[k] for k in KEEP], "notes": T("Side elevation of the near (left) side of a pig facing right; far limbs and far ear are separate chains drawn lighter and behind. Proportions, joint angles, masses and thermal parameters are authored estimates; only Kleiber's coefficient and the Merck resting respiratory range are verified literature values. The reference environment is a hot day (28 °C, 700 W/m²) because wallowing and wet skin matter there.", "Seitenansicht der nahen (linken) Seite eines nach rechts blickenden Schweins; ferne Gliedmaßen und fernes Ohr sind eigene Ketten, heller und dahinter gezeichnet. Proportionen, Gelenkwinkel, Massen und Wärmeparameter sind Schätzwerte; nur Kleibers Koeffizient und der Merck-Ruhebereich der Atemfrequenz sind verifizierte Literaturwerte. Die Referenzumgebung ist ein heißer Tag (28 °C, 700 W/m²), weil Suhlen und nasse Haut dort wichtig sind.")},
        "dimensions": {"heightM": args[0], "widthM": args[1], "depthM": 0.42, "massKg": MASS},
        "materials": mats, "rig": {"kind": "anatomical", "root": "pelvis", "bones": finish_bones(sk)}, "anchors": sk.anchors,
        "representations": reps(shp), "symbols": [], "poses": poses, "clips": clips,
        "thermal": {"environment": env, "baseMet": STAND, "body": body, "respiration": resp, "passengers": [], "operating": [], "regions": regs, "emitters": emitters, "assumptions": assumptions},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [
            T("Segments are rigid with authored masses summing to 100 kg; contacts come from pose contacts and clip spans: four hooves standing and rooting, belly and brisket when lying in the wallow.", "Segmente sind starr mit Massen von zusammen 100 kg; Kontakte kommen aus Posenkontakten und Clip-Spannen: vier Klauen im Stand und beim Wühlen, Bauch und Brustbein beim Liegen in der Suhle."),
            T("Joint moments are the moments of all distal weights and reactions about each joint; no muscle model is assumed.", "Gelenkmomente sind die Momente aller distalen Gewichte und Reaktionen um jedes Gelenk; ein Muskelmodell wird nicht angenommen.")], "clinical": False},
        "presentation": {"defaultPose": "standing-pig", "defaultClip": "walk-cycle", "viewpoints": ["elevation"], "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored",
    }
    print("\n".join(verify_poses(sk, asset, shp)))
    return write(asset, "pig")


if __name__ == "__main__":
    print(finish((float(sys.argv[1]), float(sys.argv[2])) if len(sys.argv) > 2 else (0.76, 1.2)))
