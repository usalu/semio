# [scratch] NW-A1 red fox asset document
import sys
from fox import *
from fox_poses import *
from assemble import *
from refs import REFS
import fox_shapes as FS

KEEP = ["kleiber-1932", "muybridge-1887", "hildebrand-1976", "taylor-1970", "brody-1945", "scholander-1950", "cena-monteith-1975", "stahl-1967", "schmidt-nielsen-1970", "gupta-2010", "ashrae-2021", "crawford-1962"]
STAND, TROT = 1.4, 6.0
SUN = {"*": 0.4, "coat-legs": 0.2, "paws": 0.1, "ears": 0.5, "brush": 0.4}
LEGNAMES = {"hn": ("Near hind paw", "Nahe Hinterpfote"), "fn": ("Near fore paw", "Nahe Vorderpfote"), "hf": ("Far hind paw", "Ferne Hinterpfote"), "ff": ("Far fore paw", "Ferne Vorderpfote")}


def regions():
    Fu = lambda id, t, th, k, f, cap: Lay(id, t, "fur", "red-fur", th, k, f, cap, 0.5, "authored-estimate", "scholander-1950")
    R_ = []
    R_.append(Reg("coat-trunk", T("Trunk and neck coat", "Rumpf- und Halsfell"), "fur", "red-fur", 0.2, 1.3, [Fu("fur", T("Autumn coat, 25 mm", "Herbstfell, 25 mm"), 0.025, 0.05, 1.3, 1100)], "cylinder", 0.15, 0.55, 16000, "trunk", 0.66, "authored-estimate", "brody-1945"))
    R_.append(Reg("coat-legs", T("Leg coat (dark stockings)", "Beinfell (dunkle Strümpfe)"), "fur", "dark-fur", 0.06, 1.15, [Fu("fur", T("Short coat, 12 mm", "Kurzes Fell, 12 mm"), 0.012, 0.05, 1.15, 400)], "cylinder", 0.04, 0.25, 10000, "leg", 0.1, "authored-estimate", "brody-1945"))
    R_.append(Reg("face", T("Face and muzzle", "Gesicht und Fang"), "fur", "red-fur", 0.025, 1.0, [Fu("fur", T("Short coat, 8 mm", "Kurzes Fell, 8 mm"), 0.008, 0.05, 1.0, 300)], "cylinder", 0.08, 0.5, 9000, "head", 0.1, "authored-estimate", "brody-1945"))
    R_.append(Reg("ears", T("Ears (black backs)", "Ohren (schwarze Rückseiten)"), "fur", "dark-fur", 0.014, 1.0, [Fu("fur", T("Thin coat, 4 mm", "Dünnes Fell, 4 mm"), 0.004, 0.05, 1.0, 150)], "vertical-plate", 0.05, 0.7, 4000, "head", 0.015, "authored-estimate", "brody-1945"))
    R_.append(Reg("brush", T("Brush (tail)", "Fuchsrute"), "fur", "red-fur", 0.035, 1.8, [Lay("fur", T("Brush fur, 60 mm", "Ruten-Fell, 60 mm"), "fur", "red-fur", 0.06, 0.055, 1.8, 2200, 0.5, "authored-estimate", "scholander-1950")], "cylinder", 0.1, 0.6, 9000, "tail", 0.05, "authored-estimate", "brody-1945"))
    R_.append(Reg("paws", T("Paws with pads", "Pfoten mit Ballen"), "fur", "dark-fur", 0.012, 1.0, [Fu("fur", T("Paw fur, 5 mm", "Pfotenfell, 5 mm"), 0.005, 0.05, 1.0, 200)], "horizontal-plate-down", 0.04, 0.0, 8000, "foot", 0.01, "authored-estimate", "brody-1945"))
    return finalize_regions(R_)


def tp(regs, act, breathing, contact_frac=0.0, over_extra=None):
    over = {"paws": ({"exposedFraction": round(1 - 0.5 * contact_frac, 3), "contact": {"fraction": round(0.5 * contact_frac, 3), "tempC": 6.0}} if contact_frac > 0 else {"exposedFraction": 0.9})}
    over.update(over_extra or {})
    return bind(regs, act, breathing, SUN, over)


def main():
    sk = make_skel()
    regs, askin = regions()
    A = round(askin, 4)
    basal = round(3.39 * MASS ** 0.75, 2)
    body = {"species": "fox", "massKg": MASS, "skinAreaM2": A, "basalMetabolismW": basal, "metWM2": round(basal / A, 2), "coreSetPointC": 38.6, "skinSetPointC": 35.0, "sweating": False, "panting": True, "maxSkinBloodFlowLM2H": 45, "minSkinBloodFlowLM2H": 3, "source": "authored-estimate", "reference": "kleiber-1932"}
    pats = [
        pattern("rest", T("Resting, nose breathing", "Ruhe, Nasenatmung"), 24, 0.055, (0.4, 0.05, 0.45, 0.1), "nose", "stahl-1967"),
        pattern("trot", T("Trotting, nose breathing", "Traben, Nasenatmung"), 80, 0.09, (0.45, 0.0, 0.5, 0.05), "nose", "stahl-1967"),
        pattern("stalk", T("Listening and stalking", "Lauschen und Anschleichen"), 30, 0.05, (0.42, 0.04, 0.46, 0.08), "nose", "stahl-1967"),
        pattern("pounce", T("Pounce, short deep breaths", "Sprung, kurze tiefe Atemzüge"), 70, 0.08, (0.5, 0.0, 0.5, 0.0), "nose", "stahl-1967"),
        pattern("sleep", T("Curled asleep, slow breaths into the brush", "Eingerollt schlafend, langsame Atemzüge in die Rute"), 16, 0.05, (0.4, 0.05, 0.45, 0.1), "nose", "stahl-1967"),
        pattern("pant", T("Heat panting, open mouth", "Hecheln bei Hitze, offenes Maul"), 300, 0.025, (0.5, 0.0, 0.5, 0.0), "mouth", "crawford-1962"),
    ]
    resp = respiration_block(pats, [{"bone": "thorax", "angleDeg": 1.0, "dy": 0.003}, {"bone": "lumbar", "angleDeg": 0.4, "dy": 0.002}], 31, 0.95, "schmidt-nielsen-1970")
    emitters = [
        {"id": "breath-nose", "kind": "breath", "title": T("Nostrils", "Nasenlöcher"), "anchor": "nose", "directionDeg": -45, "diameterM": 0.006, "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "breath-mouth", "kind": "breath", "title": T("Open mouth (panting)", "Offenes Maul (Hecheln)"), "anchor": "mouth", "directionDeg": -15, "diameterM": 0.014, "source": "authored-estimate", "reference": "crawford-1962"},
        {"id": "plume-back", "kind": "plume", "title": T("Warm-air plume above the back", "Warmluftfahne über dem Rücken"), "anchor": "back-plume", "directionDeg": 90, "diameterM": 0.2, "region": "coat-trunk", "source": "authored-estimate", "reference": "cena-monteith-1975"},
    ]
    st = standing_offsets(sk)
    cr, cr_root = crouch_offsets(sk)
    cl, cl_root = curled_offsets(sk)
    fl, fl_root = flight_offsets(sk)
    ld, ld_root = land_offsets(sk)
    four = ["contact-hn", "contact-fn", "contact-hf", "contact-ff"]
    curl_over = {"coat-trunk": {"exposedFraction": 0.55, "sunlitFraction": 0.3, "contact": {"fraction": 0.25, "tempC": 6.0}}, "coat-legs": {"exposedFraction": 0.3, "sunlitFraction": 0.1}, "face": {"exposedFraction": 0.35, "sunlitFraction": 0.1}, "brush": {"exposedFraction": 0.6, "sunlitFraction": 0.3}, "ears": {"exposedFraction": 0.6, "sunlitFraction": 0.3}, "paws": {"exposedFraction": 0.4, "sunlitFraction": 0.05}}
    poses = [
        pose("standing-alert", T("Standing alert", "Aufmerksam stehend"), T("Standing on four slim legs, head up, ears erect, brush hanging low and straight behind.", "Stehen auf vier schlanken Beinen, Kopf erhoben, Ohren aufgestellt, Rute tief und gerade hängend."), st, tp(regs, STAND, "rest", 1.0), four),
        pose("mousing-crouch", T("Mousing crouch", "Mäuseln, geduckt"), T("Listening for prey in the grass: front lowered, hindquarters high, head pointed down at the sound, ears forward, brush stretched back.", "Lauschen auf Beute im Gras: Vorderkörper abgesenkt, Hinterhand hoch, Kopf zum Geräusch gesenkt, Ohren nach vorn, Rute gestreckt."), cr, tp(regs, 2.0, "stalk", 1.0), four, root={"y": r4(cr_root[1])}),
        pose("curled-sleeping", T("Curled asleep", "Eingerollt schlafend"), T("Curled up on the ground: paws flat under the body, legs tucked, spine rounded, head turned back onto the haunch and the brush wrapped around forward over the nose.", "Auf dem Boden eingerollt: Beine eingeschlagen, Rücken gerundet, Kopf zurück auf die Hüfte gelegt und die Rute vorn über die Nase geschlungen."), cl, bind(regs, 1.0, "sleep", SUN, curl_over), four, root={"y": r4(cl_root[1])}),
        pose("pounce-flight", T("Pounce, airborne", "Sprung, in der Luft"), T("Mid-air of the mousing pounce: body pitched nose-down about 30 degrees, forelegs stretched forward and down, hind legs trailing, brush straight back.", "Mitte des Mäusesprungs: Körper etwa 30 Grad nach vorn unten geneigt, Vorderbeine nach vorn unten gestreckt, Hinterbeine nachgezogen, Rute gerade nach hinten."), fl, tp(regs, 8.0, "pounce", 0.0, {"coat-trunk": {"exposedFraction": 1.0, "sunlitFraction": 0.5}}), [], key=True, ground="authored", root={"x": 0.45, "y": fl_root[1]}),
        pose("pounce-land", T("Pounce, landing on the prey", "Sprung, Landung auf der Beute"), T("Forepaws pin the prey, head down between them, hind legs still behind and flexed.", "Die Vorderpfoten halten die Beute fest, Kopf zwischen ihnen gesenkt, Hinterbeine noch hinten und gebeugt."), ld, tp(regs, 3.0, "pounce", 1.0), four, key=True, root={"x": 0.9, "y": r4(ld_root[1])}),
    ]
    clips = []
    Gh = dict(ac0=-85, ks=10, flex=50, ah0=-48, ahroll=12, ahflex=14, lift=0.07)
    Gf = dict(ac0=-90, ks=8, flex=60, ah0=-42, ahroll=12, ahflex=14, lift=0.07, scap0=-60, scapamp=6)
    xs = {"hn": 0.04, "hf": 0.04, "fn": 0.47, "ff": 0.47}
    order = {"hn": 0.0, "ff": 0.0, "hf": 0.5, "fn": 0.5}
    T_, duty, speed, N = 0.5, 0.4, 1.2, 10
    bob = lambda ph: 0.03 * (0.5 + 0.5 * math.cos(4 * math.pi * (ph - 0.45)))

    def trunk(ph):
        return {"lumbar": r4(-4.0 * math.sin(4 * math.pi * ph)), "neck": r4(-3.0 * math.sin(4 * math.pi * ph)), "head": r4(3.0 * math.sin(4 * math.pi * ph)), "tail-1": r4(4 * math.sin(4 * math.pi * ph + 1.0)), "tail-2": r4(5 * math.sin(4 * math.pi * ph + 1.4)), "tail-3": r4(6 * math.sin(4 * math.pi * ph + 1.8))}

    def ptitle(k, N):
        return T("Trot, phase %d of %d" % (k + 1, N), "Trab, Phase %d von %d" % (k + 1, N)), T("Diagonal trot with suspension at %d/%d of the cycle." % (k, N), "Diagonaler Trab mit Schwebephase bei %d/%d des Zyklus." % (k, N))

    kp, clip, S = gait_clip(sk, "trot-cycle", T("Trot, diagonal pairs with suspension", "Trab in diagonalen Paaren mit Schwebephase"), T("Diagonal trot at about 1.2 m/s (cf. Hildebrand 1976 for symmetrical gaits): near hind with far fore, then far hind with near fore, each pair on the ground 40 % of the cycle, so two short flight phases per cycle; the brush trails and the back flexes slightly.", "Diagonaler Trab mit etwa 1,2 m/s (vgl. Hildebrand 1976 zu symmetrischen Gangarten): nahes Hinter- mit fernem Vorderbein, dann fernes Hinter- mit nahem Vorderbein, jedes Paar 40 % des Zyklus am Boden, also zwei kurze Flugphasen je Zyklus; die Rute schleppt, der Rücken beugt sich leicht."), ptitle, T_, N, duty, order, speed, Gh, Gf, {}, xs, lambda ph, c: tp(regs, TROT, "trot", len(c) / 4.0), FOOT, trunk_fn=trunk, bob=bob, ground="authored", names=LEGNAMES)
    clip["phases"] = [{"id": "stance-a", "fromS": 0.0, "toS": r4(0.4 * T_), "title": T("Near hind and far fore on the ground", "Nahes Hinter- und fernes Vorderbein am Boden")}, {"id": "flight-a", "fromS": r4(0.4 * T_), "toS": r4(0.5 * T_), "title": T("Suspension", "Schwebephase"), "ballistic": True}, {"id": "stance-b", "fromS": r4(0.5 * T_), "toS": r4(0.9 * T_), "title": T("Far hind and near fore on the ground", "Fernes Hinter- und nahes Vorderbein am Boden")}, {"id": "flight-b", "fromS": r4(0.9 * T_), "toS": T_, "title": T("Suspension", "Schwebephase"), "ballistic": True}]
    clip["tracks"] = [{"target": "breathing", "ease": "step", "keys": [{"t": 0.0, "v": 1}]}]
    poses.extend(kp)
    clips.append(clip)
    keys = [
        {"t": 0.0, "pose": "standing-alert", "ease": "smooth"},
        {"t": 0.6, "pose": "mousing-crouch", "ease": "smooth"},
        {"t": 1.0, "pose": "mousing-crouch", "ease": "smooth", "bones": {"head": {"angleDeg": -8.0}, "ear-n": {"angleDeg": 10.0}}},
        {"t": 1.4, "pose": "mousing-crouch", "ease": "smooth", "bones": {"pelvis": {"angleDeg": -4.0}}},
        {"t": 1.8, "pose": "pounce-flight", "ease": "smooth"},
        {"t": 2.2, "pose": "pounce-land", "ease": "smooth"},
        {"t": 3.0, "pose": "pounce-land", "ease": "smooth", "bones": {"head": {"angleDeg": -8.0}}},
    ]
    contacts = [{"anchor": a, "fromS": 0.0, "toS": 1.6} for a in ("contact-hn", "contact-hf")] + [{"anchor": a, "fromS": 0.0, "toS": 1.5} for a in ("contact-fn", "contact-ff")] + [{"anchor": a, "fromS": 2.2, "toS": 3.0} for a in four]
    clips.append({"id": "mousing-pounce", "title": T("Mousing pounce", "Mäusesprung"), "description": T("Standing alert, crouching to listen, a head cock, then the high leap onto the prey (suspension from 1.6 to 2.2 s, forepaws first) and holding it pinned.", "Aufmerksames Stehen, Ducken zum Lauschen, ein Kopfneigen, dann der hohe Sprung auf die Beute (Schwebe von 1,6 bis 2,2 s, Vorderpfoten zuerst) und festhalten."), "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none", "keys": keys, "contacts": contacts, "phases": [{"id": "listen", "fromS": 0.0, "toS": 1.4, "title": T("Listening", "Lauschen")}, {"id": "leap", "fromS": 1.6, "toS": 2.2, "title": T("Leap", "Sprung"), "ballistic": True}, {"id": "pin", "fromS": 2.2, "toS": 3.0, "title": T("Pinning the prey", "Beute festhalten")}], "events": [{"t": 1.6, "kind": "contact-off", "title": T("Hind paws leave the ground", "Hinterpfoten heben ab"), "target": "contact-hn"}, {"t": 2.2, "kind": "contact-on", "title": T("Forepaws land on the prey", "Vorderpfoten landen auf der Beute"), "target": "contact-fn"}], "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0.0, "v": 1.4}, {"t": 1.0, "v": 2.0}, {"t": 1.8, "v": 8.0}, {"t": 2.6, "v": 3.0}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0.0, "v": 0}, {"t": 0.6, "v": 2}, {"t": 1.6, "v": 3}]}]})
    shp = {"200": FS.shapes200(sk), "100": FS.shapes100(sk), "50": FS.shapes50(sk)}
    return sk, regs, A, body, resp, emitters, poses, clips, shp


def reps(shp):
    H = [{"id": "fur", "title": T("Coat strokes", "Fellstriche"), "pattern": "lines", "angleDeg": 65, "spacingMm": 0.9, "penMm": 0.1}]
    r = {}
    r["500"] = unavailable(500, T("A red fox is 1.15 m × 0.63 m: 2.3 × 1.3 mm on paper at 1:500, with legs 0.026-0.04 m wide (0.05-0.08 mm) and a brush 0.12 m wide. No composed silhouette separates body, legs and brush above the 0.2 mm feature size of the default print profile without drawing a blob that would hide the slender legs and the brush that identify the animal.", "Ein Rotfuchs misst 1,15 m × 0,63 m: auf dem Papier 2,3 × 1,3 mm bei 1:500, mit 0,026-0,04 m breiten Beinen (0,05-0,08 mm) und einer 0,12 m breiten Rute. Keine zusammengesetzte Silhouette trennt Körper, Beine und Rute oberhalb der Merkmalsgröße von 0,2 mm des Standarddruckprofils, ohne einen Klecks zu zeichnen, der die schlanken Beine und die Rute, die das Tier kennzeichnen, verdeckt."), T("Fore legs of the fox (real width 0.026 m = 0.05 mm)", "Vorderläufe des Fuchses (reale Breite 0,026 m = 0,05 mm)"), 0.026, [{"kind": "scale", "denominator": 200}])
    r["200"] = authored(200, shp["200"], D8(
        ("Main masses separated, 5.7 × 3.1 mm: trunk in two overlapping masses, neck, head with snout, one ear, a three-part bushy brush and per side a gaskin, cannon and paw.", "Hauptmassen getrennt, 5,7 × 3,1 mm: Rumpf in zwei überlappenden Massen, Hals, Kopf mit Fang, ein Ohr, eine dreiteilige buschige Rute und je Seite Unterschenkel, Röhre und Pfote."),
        ("The long bushy brush as its own three masses (the brush tip lighter), the slender legs with hock angle, the erect ear and the pointed muzzle.", "Die lange buschige Rute als drei eigene Massen (Rutenspitze heller), die schlanken Beine mit Sprunggelenkwinkel, das aufrechte Ohr und der spitze Fang."),
        ("Jaw into the head, thigh and shoulder into the trunk masses, both ears into the near ear, paws into one wedge each.", "Kiefer in den Kopf, Oberschenkel und Schulter in die Rumpfmassen, beide Ohren in das nahe Ohr, Pfoten zu je einem Keil."),
        ("Eye, nose, mouth, toes, fur strokes and thigh outline.", "Auge, Nase, Maul, Zehen, Fellstriche und Oberschenkelumriss."),
        ("Cannons (0.026-0.03 m = 0.13-0.15 mm) are drawn 0.042 m wide to reach the 0.2 mm feature minimum, 0.012-0.016 m wider than real; the two inner brush joints are overlapped so no sliver remains.", "Röhren (0,026-0,03 m = 0,13-0,15 mm) sind 0,042 m breit gezeichnet, um das 0,2-mm-Mindestmerkmal zu erreichen, 0,012-0,016 m breiter als real; die beiden inneren Rutengelenke überlappen, damit keine Spalte bleibt."),
        ("None; a 6 mm figure leaves no room for a hatch at the profile minimum.", "Keine; auf einer 6 mm großen Figur ist kein Platz für eine Schraffur im Profilminimum."),
        ("Mid-grey solids with a 0.18 mm outline; dark leg stockings and ear, light brush tip, far side lighter.", "Mittelgraue Flächen mit 0,18-mm-Kontur; dunkle Beinstrümpfe und Ohr, helle Rutenspitze, ferne Seite heller."),
        ("Each mass carries its region (trunk coat, leg coat, face, ear, brush, paws); the brush masses together show the largest insulation patch.", "Jede Masse trägt ihre Region (Rumpffell, Beinfell, Gesicht, Ohr, Rute, Pfoten); die Rutenmassen zeigen zusammen die größte Dämmfläche.")))
    r["100"] = authored(100, shp["100"], D8(
        ("Articulated outline, 11.5 × 6.3 mm: trunk masses with an inner near thigh, neck, head with eye, both ears, three-part brush with a pale tip, per side gaskin, cannon and paw wedge.", "Gegliederter Umriss, 11,5 × 6,3 mm: Rumpfmassen mit innerem nahem Oberschenkel, Hals, Kopf mit Auge, beide Ohren, dreiteilige Rute mit heller Spitze, je Seite Unterschenkel, Röhre und Pfotenkeil."),
        ("Thigh outline, hock and wrist angles, both ears, eye, the pale brush tip and the dark stockings.", "Oberschenkelumriss, Sprung- und Handwurzelwinkel, beide Ohren, Auge, die helle Rutenspitze und die dunklen Strümpfe."),
        ("Jaw into the head, shoulder blade and upper arm into the chest, toes into one paw wedge.", "Kiefer in den Kopf, Schulterblatt und Oberarm in die Brust, Zehen zu einem Pfotenkeil."),
        ("Nose, mouth line, toes, fur strokes.", "Nase, Maullinie, Zehen, Fellstriche."),
        ("Cannons 0.026-0.03 m = 0.26-0.3 mm at real width (widened 0.01 m on the hind cannon).", "Röhren 0,026-0,03 m = 0,26-0,3 mm in realer Breite (hinten um 0,01 m verbreitert)."),
        ("None at this scale.", "Keine in diesem Maßstab."),
        ("Mid-grey solids, 0.25 mm outline, dark stockings and ears, pale brush tip; the eye is the only black detail.", "Mittelgraue Flächen, 0,25-mm-Kontur, dunkle Strümpfe und Ohren, helle Rutenspitze; das Auge ist das einzige schwarze Detail."),
        ("One region per mass; the thigh shape carries the trunk coat as an inner patch.", "Eine Region je Masse; die Oberschenkelform trägt das Rumpffell als innere Fläche.")))
    r["50"] = authored(50, shp["50"], D8(
        ("Full articulated drawing, 23 × 12.5 mm: trunk in two masses with near thigh, neck, head with eye, nose, mouth line and jaw, both ears, three-part brush with fur strokes and a pale tip, per side gaskin, cannon and paw with toe outline.", "Voll gegliederte Zeichnung, 23 × 12,5 mm: Rumpf in zwei Massen mit nahem Oberschenkel, Hals, Kopf mit Auge, Nase, Maullinie und Unterkiefer, beide Ohren, dreiteilige Rute mit Fellstrichen und heller Spitze, je Seite Unterschenkel, Röhre und Pfote mit Zehenumriss."),
        ("Everything of the 1:100 drawing plus the jaw (so mousing and panting show), nose, mouth line and the fur strokes on trunk and brush.", "Alles der 1:100-Zeichnung plus Unterkiefer (damit Mäuseln und Hecheln sichtbar sind), Nase, Maullinie und die Fellstriche an Rumpf und Rute."),
        ("Upper arm lies inside the chest; the far thigh is hidden behind the near one.", "Der Oberarm liegt in der Brust; der ferne Oberschenkel ist hinter dem nahen verdeckt."),
        ("Whiskers, toes and claws are not drawn; the head is 3.4 mm long.", "Schnurrhaare, Zehen und Krallen sind nicht gezeichnet; der Kopf ist 3,4 mm lang."),
        ("Cannons 0.026-0.03 m = 0.5-0.6 mm at real width (hind cannon widened 0.01 m); ear 0.06 m = 1.2 mm.", "Röhren 0,026-0,03 m = 0,5-0,6 mm in realer Breite (hintere Röhre um 0,01 m verbreitert); Ohr 0,06 m = 1,2 mm."),
        ("Parallel coat strokes at 65 degrees, 0.9 mm spacing, 0.1 mm pen, on trunk masses and the first two brush parts; legs, face and brush tip stay clean.", "Parallele Fellstriche unter 65 Grad, 0,9 mm Abstand, 0,1 mm Stift, auf Rumpfmassen und den ersten beiden Rutenteilen; Beine, Gesicht und Rutenspitze bleiben sauber."),
        ("Mid-grey solids with 0.35 mm outline and 0.18 mm detail strokes, dark stockings and ears, pale brush tip; far side lighter.", "Mittelgraue Flächen mit 0,35-mm-Kontur und 0,18-mm-Detailstrichen, dunkle Strümpfe und Ohren, helle Rutenspitze; ferne Seite heller."),
        ("The hatched shapes are the furred regions with the thickest insulation, so thermal colour and coat read together; stockings, ears and paws are separate patches.", "Die schraffierten Formen sind die Felle mit der dicksten Dämmung, thermische Farbe und Fell lesen sich zusammen; Strümpfe, Ohren und Pfoten sind eigene Flächen.")), hatches=H)
    return r


def finish(args):
    sk, regs, A, body, resp, emitters, poses, clips, shp = main()
    mats = [
        {"id": "red-fur", "title": T("Red fox fur", "Rotfuchsfell"), "emissivity": 0.96, "solarAbsorptivity": 0.7, "conductivityWmK": 0.05, "source": "authored-estimate", "reference": "scholander-1950"},
        {"id": "dark-fur", "title": T("Dark (black) fur of stockings and ears", "Dunkles (schwarzes) Fell der Strümpfe und Ohren"), "emissivity": 0.96, "solarAbsorptivity": 0.9, "conductivityWmK": 0.05, "source": "authored-estimate", "reference": "cena-monteith-1975"},
    ]
    env = {"airTempC": 8, "meanRadiantTempC": 8, "skyTempC": -5, "groundTempC": 6, "relativeHumidity": 0.7, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 200, "diffuseFraction": 0.3, "pressurePa": 101325}
    assumptions = [
        T("Skin area 0.105·M^(2/3) = %.3f m² for 6 kg is an authored estimate guided by Brody (1945); the six regions cover it; the brush (0.035 m² of skin, 60 mm fur) is the thickest insulation patch." % A, "Die Hautfläche 0,105·M^(2/3) = %.3f m² für 6 kg ist ein Schätzwert nach Brody (1945); die sechs Regionen decken sie ab; die Rute (0,035 m² Haut, 60 mm Fell) ist die dickste Dämmfläche." % A),
        T("Basal metabolism is Kleiber's 3.39 W per kg^0.75 = %.1f W (verified coefficient); one met is that rate per skin area. Standing 1.4, listening crouch 2.0, curled sleep 1.0, the pounce burst 8.0 and the trot at 1.2 m/s (6.0 met) follow the cost of running after Taylor et al. (1970); the multiples are authored estimates." % body["basalMetabolismW"], "Der Grundumsatz ist Kleibers 3,39 W je kg^0,75 = %.1f W (verifizierter Koeffizient); ein Met ist diese Rate je Hautfläche. Stehen 1,4, lauschendes Ducken 2,0, eingerollter Schlaf 1,0, der Sprungstoß 8,0 und der Trab mit 1,2 m/s (6,0 Met) folgen dem Laufaufwand nach Taylor et al. (1970); die Vielfachen sind Schätzwerte." % body["basalMetabolismW"]),
        T("Fur layers (autumn coat 25 mm, legs 12 mm, face 8 mm, ears 4 mm, brush 60 mm at 0.05-0.055 W/(m K)) are authored estimates; the qualitative importance of coat thickness follows Scholander et al. (1950), the Lewis-type vapour resistance uses i_m 0.5. Curled sleeping hides legs, face and part of the trunk (exposed fractions 0.3-0.55) and wraps the brush over the nose.", "Fellschichten (Herbstfell 25 mm, Beine 12 mm, Gesicht 8 mm, Ohren 4 mm, Rute 60 mm bei 0,05-0,055 W/(m K)) sind Schätzwerte; die qualitative Bedeutung der Felldicke folgt Scholander et al. (1950), der Dampfwiderstand nach Lewis nutzt i_m 0,5. Eingerolltes Schlafen verbirgt Beine, Gesicht und Teile des Rumpfes (freie Anteile 0,3-0,55) und legt die Rute über die Nase."),
        T("The fox is not in the verified resting-rate table: rest 24, trot 80 and heat panting 300 per minute and the tidal volumes (about 9 mL/kg at rest) are authored estimates guided by allometry (Stahl 1967) and canid panting (Crawford 1962). Foxes do not sweat except at the paw pads, so sweating is off and panting on; the reference environment is a cold 8 °C day.", "Der Fuchs fehlt in der verifizierten Ruhefrequenztabelle: Ruhe 24, Trab 80 und Hecheln 300 je Minute sowie die Atemzugvolumina (etwa 9 mL/kg in Ruhe) sind Schätzwerte nach Allometrie (Stahl 1967) und Hecheln von Hundeartigen (Crawford 1962). Füchse schwitzen nur an den Pfotenballen, daher Schwitzen aus und Hecheln an; die Referenzumgebung ist ein kalter Tag mit 8 °C."),
    ]
    asset = {
        "id": "red-fox", "category": "animals", "title": T("Red Fox", "Rotfuchs"),
        "summary": T("An adult red fox (Vulpes vulpes) of 6 kg, 1.15 m nose to brush tip and 0.5 m at the shoulder: slender legs with dark stockings, pointed muzzle, erect ears and a long bushy brush with a pale tip; standing alert, a mousing crouch, curled asleep with the brush over the nose, a diagonal trot with suspension and a mousing pounce; thick brush fur as the largest insulation patch.", "Ein erwachsener Rotfuchs (Vulpes vulpes) von 6 kg, 1,15 m von der Nase bis zur Rutenspitze und 0,5 m Schulterhöhe: schlanke Beine mit dunklen Strümpfen, spitzer Fang, aufrechte Ohren und eine lange buschige Rute mit heller Spitze; aufmerksames Stehen, Ducken beim Mäuseln, eingerollt schlafend mit der Rute über der Nase, diagonaler Trab mit Schwebephase und ein Mäusesprung; dichtes Rutenfell als größte Dämmfläche."),
        "tags": ["fox", "red-fox", "wildlife", "canid", "trot", "pounce", "curled-sleeping", "quadruped", "brush-tail"],
        "provenance": {"author": "Semio staffage content worker NW-A1", "method": "handcrafted", "created": "2026-10-29", "references": [REFS[k] for k in KEEP], "notes": T("Side elevation of the near (left) side of a fox facing right; far limbs and far ear are separate chains drawn lighter and behind. Proportions, joint angles, masses and thermal parameters are authored estimates; only Kleiber's coefficient is a verified literature value. 1:500 is declared unavailable because the fox is only 2.3 mm long and its legs are far below the print profile's feature size.", "Seitenansicht der nahen (linken) Seite eines nach rechts blickenden Fuchses; ferne Gliedmaßen und fernes Ohr sind eigene Ketten, heller und dahinter gezeichnet. Proportionen, Gelenkwinkel, Massen und Wärmeparameter sind Schätzwerte; nur Kleibers Koeffizient ist ein verifizierter Literaturwert. 1:500 ist als nicht verfügbar erklärt, weil der Fuchs nur 2,3 mm lang ist und seine Beine weit unter der Merkmalsgröße des Druckprofils liegen.")},
        "dimensions": {"heightM": args[0], "widthM": args[1], "depthM": 0.16, "massKg": MASS},
        "materials": mats, "rig": {"kind": "anatomical", "root": "pelvis", "bones": finish_bones(sk)}, "anchors": sk.anchors,
        "representations": reps(shp), "symbols": [], "poses": poses, "clips": clips,
        "thermal": {"environment": env, "baseMet": STAND, "body": body, "respiration": resp, "passengers": [], "operating": [], "regions": regs, "emitters": emitters, "assumptions": assumptions},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [
            T("Segments are rigid with authored masses summing to 6 kg; contacts come from pose contacts and clip spans: paws when standing, crouching and landing, the folded paws when curled; the airborne part of the pounce and the trot suspensions are ballistic phases without support.", "Segmente sind starr mit Massen von zusammen 6 kg; Kontakte kommen aus Posenkontakten und Clip-Spannen: Pfoten im Stand, beim Ducken und Landen, die untergeschlagenen Pfoten beim Einrollen; der Flug des Sprungs und die Schwebephasen des Trabs sind ballistische Phasen ohne Auflager."),
            T("Joint moments are the moments of all distal weights and reactions about each joint; no muscle model is assumed. The pounce poses are dynamic snapshots: the centre of mass is not statically balanced.", "Gelenkmomente sind die Momente aller distalen Gewichte und Reaktionen um jedes Gelenk; ein Muskelmodell wird nicht angenommen. Die Sprungposen sind dynamische Momentaufnahmen: der Schwerpunkt ist nicht statisch ausgeglichen.")], "clinical": False},
        "presentation": {"defaultPose": "standing-alert", "defaultClip": "trot-cycle", "viewpoints": ["elevation"], "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored",
    }
    rep_shapes = {k: v for k, v in shp.items()}
    print("\n".join(verify_poses(sk, asset, rep_shapes)))
    return write(asset, "red-fox")


if __name__ == "__main__":
    print(finish((float(sys.argv[1]), float(sys.argv[2])) if len(sys.argv) > 2 else (0.63, 1.17)))
