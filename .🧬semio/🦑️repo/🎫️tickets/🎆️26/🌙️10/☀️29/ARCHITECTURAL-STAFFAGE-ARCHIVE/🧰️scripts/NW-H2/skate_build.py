"""Builds S/🗂️catalog/👤️humans/skateboarder/🔣️.json"""
from lib import *
from common_build import *
from skate_rig import *
from skate_poses import poses, GROUND
from skate_art import all_shapes
import skate_art

bones, L = build_rig()
anchors = build_anchors(L)
anchors = [a for a in anchors]
# emitter anchors follow the head profile
for a in anchors:
    if a["id"] == "nostrils":
        a["x"], a["y"] = round(0.145 * skate_art.F, 4), round(-0.102 * skate_art.F, 4)
    if a["id"] == "mouth":
        a["x"], a["y"] = round(0.117 * skate_art.F, 4), round(-0.092 * skate_art.F, 4)
P = poses()
A_SKIN = dubois(H, MASS)

MAT = [
    {"id": "skin", "title": tx("Skin", "Haut"), "emissivity": 0.98, "solarAbsorptivity": 0.6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "knit", "title": tx("Wool-blend knit beanie", "Strickmütze aus Wollmischung"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "fleece-cotton", "title": tx("Cotton fleece (hooded sweatshirt)", "Baumwollfleece (Kapuzenpullover)"), "emissivity": 0.93, "solarAbsorptivity": 0.6, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "twill", "title": tx("Cotton twill trousers", "Baumwoll-Twill-Hose"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "skate-shoe", "title": tx("Vulcanised skate shoe (suede and rubber)", "Vulkanisierter Skateschuh (Veloursleder und Gummi)"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "source": "authored-estimate"},
    {"id": "maple", "title": tx("Seven-ply maple deck with black grip tape", "Siebenlagiges Ahorndeck mit schwarzem Griptape"), "emissivity": 0.93, "solarAbsorptivity": 0.8, "densityKgM3": 700, "specificHeatJKgK": 1600, "source": "authored-estimate"},
    {"id": "grip-tape", "title": tx("Grip tape (silicon carbide on paper)", "Griptape (Siliziumkarbid auf Papier)"), "emissivity": 0.95, "solarAbsorptivity": 0.9, "source": "authored-estimate"},
    {"id": "aluminium", "title": tx("Cast aluminium truck", "Achse aus Aluminiumguss"), "emissivity": 0.15, "solarAbsorptivity": 0.35, "densityKgM3": 2700, "conductivityWmK": 160, "specificHeatJKgK": 900, "source": "literature", "reference": "incropera-2011"},
    {"id": "polyurethane", "title": tx("Polyurethane wheel", "Polyurethanrad"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1200, "specificHeatJKgK": 1800, "source": "authored-estimate", "reference": "incropera-2011"},
]

fc = {"hood": 1.25, "slv": 1.2, "trs": 1.15, "bean": 1.1}


def reg_list():
    R = []
    R.append(region("face", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.04 * A_SKIN, 0.15, "sphere", 0.55, [], 15000, "head", 0.06, 0.04, ref="dubois-1916"))
    R.append(region("beanie", "Beanie and hair under it", "Mütze und Haar darunter", "hair", "knit", 0.04 * A_SKIN * fc["bean"], 0.16, "sphere", 0.6, [
        layer("beanie-air", "Air under the beanie", "Luft unter der Mütze", "air-gap", "knit", 0.004, 0.02),
        layer("beanie-knit", "Knit beanie (0.15 clo)", "Strickmütze (0,15 clo)", "clothing", "knit", 0.004, clo_r(0.15, fc["bean"]), 600)], 4000, "head", 0.04, 0.04))
    R.append(region("hoodie-torso", "Hooded sweatshirt over T-shirt, trunk", "Kapuzenpullover über T-Shirt, Rumpf", "clothing", "fleece-cotton", 0.35 * A_SKIN * fc["hood"], 0.32, "cylinder", 0.5, [
        layer("torso-air", "Air between skin and T-shirt", "Luft zwischen Haut und T-Shirt", "air-gap", "fleece-cotton", 0.004, 0.025),
        layer("tshirt", "Cotton T-shirt (0.09 clo)", "Baumwoll-T-Shirt (0,09 clo)", "clothing", "fleece-cotton", 0.0008, clo_r(0.09, fc["hood"]), 250),
        layer("hoodie", "Cotton fleece hoodie (0.34 clo)", "Baumwollfleece-Hoodie (0,34 clo)", "clothing", "fleece-cotton", 0.004, clo_r(0.34, fc["hood"]), 900)], 20000, "trunk", 0.40, 0.35))
    R.append(region("hoodie-sleeves", "Hoodie sleeves", "Hoodie-Ärmel", "clothing", "fleece-cotton", 0.14 * A_SKIN * fc["slv"], 0.09, "cylinder", 0.5, [
        layer("sleeve-air", "Air in the sleeve", "Luft im Ärmel", "air-gap", "fleece-cotton", 0.005, 0.03),
        layer("sleeve", "Cotton fleece sleeve (0.30 clo)", "Baumwollfleece-Ärmel (0,30 clo)", "clothing", "fleece-cotton", 0.004, clo_r(0.30, fc["slv"]), 800)], 16000, "arm", 0.08, 0.14))
    R.append(region("hands", "Bare hands", "Nackte Hände", "skin", "skin", 0.05 * A_SKIN, 0.06, "cylinder", 0.5, [], 12000, "hand", 0.02, 0.05, ref="dubois-1916"))
    R.append(region("trousers", "Cotton twill trousers", "Baumwoll-Twill-Hose", "clothing", "twill", 0.31 * A_SKIN * fc["trs"], 0.11, "cylinder", 0.4, [
        layer("leg-air", "Air between skin and trousers", "Luft zwischen Haut und Hose", "air-gap", "twill", 0.006, 0.03),
        layer("twill", "Cotton twill (0.24 clo)", "Baumwoll-Twill (0,24 clo)", "clothing", "twill", 0.0007, clo_r(0.24, fc["trs"]), 500)], 18000, "leg", 0.35, 0.31))
    R.append(region("shoes", "Skate shoes with socks", "Skateschuhe mit Socken", "clothing", "skate-shoe", 0.07 * A_SKIN, 0.12, "horizontal-plate-up", 0.25, [
        layer("sock", "Cotton ankle sock (0.04 clo)", "Baumwoll-Sneakersocke (0,04 clo)", "clothing", "skate-shoe", 0.002, 0.0062, 300),
        layer("shoe-air", "Air in the shoe", "Luft im Schuh", "air-gap", "skate-shoe", 0.008, 0.035),
        layer("upper", "Suede upper and foam collar", "Veloursoberleder und Schaumkragen", "shell", "skate-shoe", 0.003, 0.03, 900)], 7000, "foot", 0.05, 0.07))
    R.append(region("deck", "Maple deck with grip tape", "Ahorndeck mit Griptape", "surface", "maple", 0.30, 0.8, "horizontal-plate-up", 0.7, [], 12000, None, None, None, model="passive", ref="incropera-2011"))
    R.append(region("trucks", "Aluminium trucks", "Aluminiumachsen", "surface", "aluminium", 0.05, 0.14, "cylinder", 0.2, [], 4500, None, None, None, model="passive", ref="incropera-2011"))
    R.append(region("wheels", "Polyurethane wheels", "Polyurethanräder", "tyre", "polyurethane", 0.046, 0.054, "cylinder", 0.15, [], 20000, None, None, None, model="passive", ref="incropera-2011"))
    return R


REGIONS = reg_list()
RIDS = [r["id"] for r in REGIONS]


def bind(act, breath, sun=0.55, shoe_contact=(0.3, 26), wheel_contact=(0.02, 28), plume=0.6, wet=0.0, deck_sun=0.8):
    reg = {}
    for rid in RIDS:
        if rid in ("face", "beanie"):
            reg[rid] = rbind(1.0, min(1.0, sun + 0.05))
        elif rid == "hands":
            reg[rid] = rbind(1.0, sun, wet=wet)
        elif rid == "shoes":
            reg[rid] = rbind(1.0 - shoe_contact[0], sun * 0.7, contact=shoe_contact)
        elif rid == "deck":
            reg[rid] = rbind(0.9, deck_sun)
        elif rid == "trucks":
            reg[rid] = rbind(0.8, 0.1)
        elif rid == "wheels":
            reg[rid] = rbind(0.9, 0.1, contact=wheel_contact)
        else:
            reg[rid] = rbind(1.0, sun, wet=wet if wet else None)
    return {"activityMet": act, "breathing": breath, "regions": reg, "emitters": {"plume": plume}}


T_ = {
    "cruising": bind(2.5, "ride"),
    "pushing": bind(5.0, "push", shoe_contact=(0.45, 28), plume=1.0, wet=0.1),
    "crouch": bind(3.0, "ride", plume=0.8),
    "ollie-air": bind(6.0, "effort", shoe_contact=(0.2, 26), wheel_contact=(0.0, 28), plume=1.0, wet=0.1),
    "push-plant": bind(4.5, "push", shoe_contact=(0.4, 28), plume=1.0, wet=0.1),
    "push-lift": bind(4.5, "push", shoe_contact=(0.3, 28), plume=1.0, wet=0.1),
    "push-swing": bind(4.0, "push", plume=1.0, wet=0.1),
    "push-reach": bind(3.5, "ride", plume=0.8),
    "pop": bind(6.0, "effort", shoe_contact=(0.3, 26), wheel_contact=(0.0, 28), plume=1.0, wet=0.1),
    "landing": bind(5.0, "push", plume=1.0, wet=0.1),
}
WH = ["wheel-f-contact", "wheel-r-contact"]


def rooty(name, contacts):
    f = P[name]
    ys = [f.at(a["bone"], (a["x"], a["y"]))[1] for a in anchors if a["id"] in contacts]
    return -min(ys) if ys else None


def pose_list():
    out = []
    spec = [
        ("cruising", "Cruising", "Rollen", "Riding along on a level board, knees bent, arms low and open for balance.", "Fahrt auf ebenem Brett, Knie gebeugt, Arme tief und geöffnet zum Ausbalancieren.", WH, False),
        ("pushing", "Pushing off", "Anschieben", "Front foot on the deck, the far foot pushes on the ground behind the board with the heel lifted.", "Das vordere Bein steht auf dem Deck, der hintere Fuß schiebt mit angehobener Ferse hinter dem Brett auf dem Boden.", WH + ["contact-r"], False),
        ("crouch", "Crouch before the pop", "Hocke vor dem Absprung", "Deep knee bend with the torso forward, both feet over the trucks.", "Tiefe Kniebeuge mit vorgeneigtem Rumpf, beide Füße über den Achsen.", WH, False),
        ("ollie-air", "Ollie in the air", "Ollie in der Luft", "Board and rider airborne, knees tucked, arms out; no contact.", "Brett und Fahrer in der Luft, Knie angezogen, Arme ausgestreckt; kein Kontakt.", [], False),
        ("push-plant", "Foot lands for the push", "Fuß setzt zum Anschieben auf", "The pushing foot touches down beside the board.", "Der Schiebefuß setzt neben dem Brett auf.", WH + ["contact-r"], True),
        ("push-lift", "Push foot leaves the ground", "Schiebefuß hebt ab", "End of the push: the foot trails behind the board and lifts off.", "Ende des Schubs: Der Fuß schleift hinter dem Brett und hebt ab.", WH, True),
        ("push-swing", "Foot swings back to the board", "Fuß schwingt zum Brett zurück", "The pushing foot swings forward above the deck.", "Der Schiebefuß schwingt über dem Deck nach vorn.", WH, True),
        ("push-reach", "Foot lifts for the next push", "Fuß hebt zum nächsten Schub", "The rear foot leaves the deck and reaches for the ground.", "Der hintere Fuß verlässt das Deck und tastet zum Boden.", WH, True),
        ("pop", "Tail strikes the ground", "Heck schlägt auf", "The tail snaps to the ground and the board tilts about 38 degrees while the rider extends upward.", "Das Heck schlägt auf den Boden und das Brett kippt um etwa 38 Grad, während sich der Fahrer streckt.", ["tail"], True),
        ("landing", "Landing", "Landung", "Both feet back on the deck, knees absorbing the impact.", "Beide Füße wieder auf dem Deck, die Knie federn den Aufprall ab.", WH, True),
    ]
    for pid, en, de, den, dee, cont, key in spec:
        ground = "auto"
        ry = None
        if pid == "ollie-air":
            ground, ry = "authored", 0.45
        elif pid == "pop":
            ground, ry = "authored", rooty("pop", ["tail"])
        out.append(pose_record(pid, en, de, den, dee, P[pid], cont, T_[pid], ground=ground, key=key, root_y=ry))
    return out


def clips():
    ph = lambda i, f, t, en, de, b=False: (i, f, t, (en, de), b)
    c = []
    c.append(clip_record("push-cycle", "Push, glide, push", "Anschieben, Gleiten, Anschieben",
        "One push cycle of about 2 s in the frame of the board (the ground slides under the pushing foot): foot plants, pushes back, lifts, swings forward, rests on the deck for the glide and reaches again. The wheels never leave the ground.",
        "Ein Schubzyklus von etwa 2 s im Bezugssystem des Bretts (der Boden gleitet unter dem Schiebefuß): Fuß setzt auf, schiebt zurück, hebt ab, schwingt nach vorn, ruht beim Gleiten auf dem Deck und tastet erneut. Die Räder verlassen den Boden nie.",
        2.0, True, [(0, "push-plant", "smooth"), (0.35, "pushing", "smooth"), (0.5, "push-lift", "smooth"), (0.85, "push-swing", "smooth"), (1.1, "cruising", "smooth"), (1.6, "cruising", "smooth"), (1.8, "push-reach", "smooth")],
        [(WH[0], 0, 2.0), (WH[1], 0, 2.0), ("contact-r", 0, 0.46)],
        [ph("push", 0, 0.5, "Push on the ground", "Schub auf dem Boden"), ph("recover", 0.5, 1.1, "Foot returns to the deck", "Fuß kehrt zum Deck zurück"), ph("glide", 1.1, 1.8, "Gliding on the board", "Gleiten auf dem Brett"), ph("reach", 1.8, 2.0, "Reaching for the ground", "Tasten zum Boden")],
        [(0, "contact-on", ("Pushing foot touches down", "Schiebefuß setzt auf"), "contact-r"), (0.46, "contact-off", ("Pushing foot leaves the ground", "Schiebefuß hebt ab"), "contact-r"), (0, "thermal", ("Push breathing: about one breath per push", "Schubatmung: etwa ein Atemzug je Schub"), None)],
        [("breathing", "step", [(0, 2), (1.1, 1), (1.8, 2)])]))
    c.append(clip_record("ollie", "Ollie", "Ollie",
        "Crouch, snap the tail to the ground (the board tilts about 38 degrees), leap with the board for a short flight of about 0.35 s and land on both trucks. The flight is a ballistic phase without any contact.",
        "Hocke, Heck auf den Boden schlagen (das Brett kippt um etwa 38 Grad), mit dem Brett abspringen, etwa 0,35 s fliegen und auf beiden Achsen landen. Die Flugphase ist ballistisch ohne jeden Kontakt.",
        1.7, False, [(0, "cruising", "smooth"), (0.35, "crouch", "smooth"), (0.6, "pop", "smooth"), (0.82, "ollie-air", "smooth"), (1.05, "landing", "smooth"), (1.7, "cruising", "smooth")],
        [(WH[0], 0, 0.52), (WH[1], 0, 0.5), ("tail", 0.5, 0.7), (WH[0], 1.0, 1.7), (WH[1], 1.0, 1.7)],
        [ph("approach", 0, 0.35, "Approach and crouch", "Anlauf und Hocke"), ph("pop", 0.35, 0.7, "Pop and extension", "Absprung und Streckung"), ph("flight", 0.7, 1.0, "Flight", "Flugphase", True), ph("land", 1.0, 1.7, "Landing and roll-out", "Landung und Ausrollen")],
        [(0.5, "contact-on", ("Tail strikes the ground", "Heck schlägt auf"), "tail"), (0.7, "contact-off", ("Board leaves the ground", "Brett hebt ab"), "tail"), (1.0, "contact-on", ("Wheels touch down", "Räder setzen auf"), WH[0])],
        [("breathing", "step", [(0, 1), (0.3, 3), (1.1, 2), (1.7, 1)])]))
    c.append(clip_record("pump-cruise", "Pumping while cruising", "Pumpen beim Rollen",
        "Slow pumping on a level board: the knees bend and extend once per 1.6 s while the wheels stay on the ground.",
        "Langsames Pumpen auf ebenem Brett: Die Knie beugen und strecken sich einmal je 1,6 s, die Räder bleiben am Boden.",
        1.6, True, [(0, "cruising", "smooth"), (0.8, "crouch", "smooth")], [(WH[0], 0, 1.6), (WH[1], 0, 1.6)],
        [ph("pump", 0, 1.6, "Pumping", "Pumpen")], [], [("breathing", "step", [(0, 1), (1.6, 1)])]))
    return c


def thermal():
    pats = [
        breath_pattern("rest", "Resting breathing", "Ruheatmung", 14, 0.5, 0.38, 0.04, 0.45, 0.13, "nose", "gupta-2010"),
        breath_pattern("ride", "Riding, light effort", "Fahren, leichte Belastung", 18, 0.9, 0.42, 0.02, 0.50, 0.06, "nose-and-mouth", "ainsworth-2011"),
        breath_pattern("push", "Pushing, moderate effort", "Anschieben, mäßige Belastung", 28, 1.5, 0.44, 0.02, 0.52, 0.02, "mouth", "ainsworth-2011"),
        breath_pattern("effort", "Brief held-breath effort", "Kurze Anstrengung mit angehaltenem Atem", 20, 1.2, 0.30, 0.30, 0.30, 0.10, "mouth", "ainsworth-2011"),
    ]
    return {
        "environment": dict(ENV_OUTDOOR), "baseMet": 2.5, "body": physiology(MASS, H, True, 19),
        "respiration": {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95, "patterns": pats, "motion": [{"bone": "torso", "angleDeg": 1.2, "dy": 0.004}, {"bone": "pelvis", "angleDeg": 0.5, "dy": 0.003}], "source": "authored-estimate", "reference": "hoeppe-1981"},
        "passengers": [], "operating": [], "regions": REGIONS,
        "emitters": nose_mouth_emitters(-150, -100) + [plume_emitter("hoodie-torso")],
        "assumptions": [
            tx("DuBois skin area for 1.78 m and 68 kg; a cotton fleece hoodie over a T-shirt (0.43 clo trunk, 0.30 clo sleeves), cotton twill trousers (0.24 clo) and a knit beanie (0.15 clo). Garment values are authored estimates guided by ISO 9920 (1 clo = 0.155 m2K/W); layer resistances include the enclosed air.", "DuBois-Hautfläche für 1,78 m und 68 kg; Baumwollfleece-Hoodie über T-Shirt (0,43 clo Rumpf, 0,30 clo Ärmel), Baumwoll-Twill-Hose (0,24 clo) und Strickmütze (0,15 clo). Die Kleidungswerte sind Autorenschätzungen nach ISO 9920 (1 clo = 0,155 m2K/W); die Schichtwiderstände enthalten die eingeschlossene Luft."),
            tx("Metabolic rate 2.5 met rolling, 5 met pushing, 6 met for the ollie (Compendium of Physical Activities: skateboarding 5.0, competitive 6.0; MET values mapped to thermal met as an estimate). Basal metabolism by Mifflin-St Jeor for a 19-year-old man.", "Stoffwechsel 2,5 met beim Rollen, 5 met beim Anschieben, 6 met beim Ollie (Compendium of Physical Activities: Skateboarden 5,0, Wettkampf 6,0; MET-Werte als Schätzung auf thermisches met übertragen). Grundumsatz nach Mifflin-St Jeor für einen 19-jährigen Mann."),
            tx("Respiration patterns are authored estimates; the first pattern (rest) is the reference of the chest motion. Exhaled air 34 degrees Celsius and 95 % humidity. Airflow caused by the rolling board (about 3 m/s) is not part of the ambient wind of the reference environment.", "Die Atemmuster sind Autorenschätzungen; das erste Muster (Ruhe) ist die Referenz der Brustbewegung. Ausatemluft 34 Grad Celsius und 95 % Feuchte. Der Fahrtwind des rollenden Bretts (etwa 3 m/s) ist nicht im Umgebungswind der Referenzumgebung enthalten."),
            tx("Deck, trucks and wheels are passive surfaces warmed by sun and ground contact: black grip tape absorbs 90 % of the sun, bare aluminium trucks emit little (emissivity 0.15).", "Deck, Achsen und Räder sind passive Flächen, die von Sonne und Bodenkontakt erwärmt werden: Das schwarze Griptape absorbiert 90 % der Sonne, blanke Aluminiumachsen strahlen wenig ab (Emissionsgrad 0,15)."),
        ],
    }


DEC = {}


def decisions():
    d = {}
    d[500] = dict(
        silhouette=("One solid dark rider of eleven overlapping forms (head, trunk, pelvis, a thigh, a shank and a shoe per leg, an upper arm and a forearm with hand per arm) standing on one solid board slab; every joint is closed by overlap.", "Ein flächig dunkler Fahrer aus elf überlappenden Formen (Kopf, Rumpf, Becken, je Bein Oberschenkel, Unterschenkel und Schuh, je Arm Oberarm und Unterarm mit Hand) auf einer einzigen flächigen Brettplatte; jedes Gelenk ist durch Überlappung geschlossen."),
        retained=("Crouched riding stance, forward lean, board length 0.81 m with raised nose and tail and the 0.085 m ride height; real height and width of the default pose are kept.", "Geduckte Fahrhaltung, Vorlage, Brettlänge 0,81 m mit hochgezogenen Enden und 0,085 m Fahrhöhe; Höhe und Breite der Standardpose bleiben real."),
        merged=("Deck, trucks and wheels form one slab between the trucks (the 0.03 m gaps between wheels and deck are below the 0.075 m minimum gap); beanie and face are one head; hand and forearm are one arm form; sleeve and glove share the arm.", "Deck, Achsen und Räder bilden zwischen den Achsen eine Platte (die 0,03 m Lücken zwischen Rädern und Deck liegen unter dem Mindestabstand von 0,075 m); Mütze und Gesicht sind ein Kopf; Hand und Unterarm sind eine Armform; Ärmel und Hand teilen sich den Arm."),
        omitted=("Face, ear, hood, pocket, fingers, laces, soles, grip tape, bearings and wheel outlines.", "Gesicht, Ohr, Kapuze, Tasche, Finger, Schnürung, Sohlen, Griptape, Lager und Radkonturen."),
        thinElements=("The 0.012 m deck is not drawn as a line: the slab fills the 0.085 m ride height. Arms are drawn 0.10 m wide where the real sleeve is 0.09 to 0.10 m (feature minimum 0.2 mm = 0.1 m); nothing is thickened beyond 15 mm.", "Das 0,012 m dicke Deck wird nicht als Linie gezeichnet: Die Platte füllt die Fahrhöhe von 0,085 m. Arme sind 0,10 m breit gezeichnet, wo der Ärmel real 0,09 bis 0,10 m misst (Mindestbreite 0,2 mm = 0,1 m); nichts wird um mehr als 15 mm verdickt."),
        hatching=("None.", "Keine."),
        styles=("Solid dark fill with a 0.15 mm outline; the board slab is darker than the rider so the contact line stays readable.", "Flächig dunkle Füllung mit 0,15-mm-Kontur; die Brettplatte ist dunkler als der Fahrer, damit die Kontaktlinie lesbar bleibt."),
        overlays=("The slab carries deck, trucks and wheels as merged regions; the head carries face and beanie; each forearm carries sleeve and hand; each leg form carries trousers, the shoe its own region.", "Die Platte trägt Deck, Achsen und Räder als zusammengefasste Regionen; der Kopf trägt Gesicht und Mütze; jeder Unterarm trägt Ärmel und Hand; jede Beinform trägt die Hose, der Schuh seine eigene Region."))
    d[200] = dict(
        silhouette=("Articulated figure: head with beanie, hoodie with hood, trousers per leg in thigh and shank, sleeve in upper arm and forearm, tapered hand, shoe with sole, and the board as a thickened deck with two truck blocks and two wheels.", "Gegliederte Figur: Kopf mit Mütze, Hoodie mit Kapuze, Hose je Bein in Ober- und Unterschenkel, Ärmel in Ober- und Unterarm, spitz zulaufende Hand, Schuh mit Sohle und das Brett als verstärktes Deck mit zwei Achsblöcken und zwei Rädern."),
        retained=("Separate tones for hoodie, trousers, skin, beanie and shoe; wheels as circles under the deck; all poses animate on the same bones.", "Getrennte Töne für Hoodie, Hose, Haut, Mütze und Schuh; Räder als Kreise unter dem Deck; alle Posen werden auf denselben Knochen animiert."),
        merged=("Sock and shoe; hand and fingers; truck hanger and baseplate; trousers and pelvis seat are overlapped, not outlined twice.", "Socke und Schuh; Hand und Finger; Achsbügel und Grundplatte; Hose und Gesäß überlappen sich und werden nicht doppelt umrissen."),
        omitted=("Face details, thumb, cuffs, seams, laces, grip tape, bearings.", "Gesichtsdetails, Daumen, Bündchen, Nähte, Schnürung, Griptape, Lager."),
        thinElements=("The deck is drawn 0.030 m thick instead of 0.012 m (minimum feature 0.2 mm = 0.04 m is approached by also overlapping the truck blocks); the ride height of 0.097 m is kept. Wheels are drawn 0.060 m instead of 0.054 m in diameter.", "Das Deck wird 0,030 m statt 0,012 m dick gezeichnet (die Mindestbreite von 0,2 mm = 0,04 m wird durch überlappende Achsblöcke erreicht); die Fahrhöhe von 0,097 m bleibt. Räder werden mit 0,060 m statt 0,054 m Durchmesser gezeichnet."),
        hatching=("None.", "Keine."),
        styles=("0.18 mm outlines; light skin and shoe, mid-grey hoodie, dark trousers and beanie, white wheels.", "0,18-mm-Konturen; helle Haut und Schuhe, mittelgrauer Hoodie, dunkle Hose und Mütze, weiße Räder."),
        overlays=("Every region is its own form: face, beanie, hoodie (torso), sleeve, hand, trousers, shoe, deck, trucks and wheels.", "Jede Region ist eine eigene Form: Gesicht, Mütze, Hoodie (Rumpf), Ärmel, Hand, Hose, Schuh, Deck, Achsen und Räder."))
    d[100] = dict(
        silhouette=("Clothing outlines of hoodie with bunched hood and hem band, trousers with a cuff line, mitten hands with thumb, two-part shoe, face profile with eye under a beanie with a cuff line, and the deck with trucks and wheels.", "Kleidungsumrisse von Hoodie mit Kapuze und Saumbündchen, Hose mit Umschlaglinie, Fäustlingshände mit Daumen, zweiteiliger Schuh, Gesichtsprofil mit Auge unter einer Mütze mit Umschlaglinie, dazu Deck mit Achsen und Rädern."),
        retained=("Hood, hem band, trouser cuff, thumb, sole, beanie cuff and eye; wheel diameter 0.054 m is true.", "Kapuze, Saumbündchen, Hosenumschlag, Daumen, Sohle, Mützenumschlag und Auge; der Raddurchmesser von 0,054 m ist maßgetreu."),
        merged=("Fingers into the mitten; truck hanger and baseplate into one truck form; sock into the shoe.", "Finger im Fäustling; Achsbügel und Grundplatte als eine Achsform; Socke im Schuh."),
        omitted=("Ear, mouth, drawstrings, pocket, seams, laces, grip tape, bearings, beanie ribs.", "Ohr, Mund, Kordeln, Tasche, Nähte, Schnürung, Griptape, Lager, Mützenrippen."),
        thinElements=("The deck is drawn at its real 0.012 m (0.12 mm) as a dark filled form under a 0.2 mm outline pen and joined to the truck forms; the thumb is 0.024 m wide (0.24 mm).", "Das Deck ist mit seiner realen Dicke von 0,012 m (0,12 mm) als dunkle Füllung unter einer 0,2-mm-Kontur gezeichnet und mit den Achsformen verbunden; der Daumen ist 0,024 m breit (0,24 mm)."),
        hatching=("None; tone fills carry the materials.", "Keine; Tonflächen tragen die Materialien."),
        styles=("0.20 mm outlines, 0.13 mm detail lines; tones as at 1:200.", "0,20-mm-Konturen, 0,13-mm-Detaillinien; Töne wie bei 1:200."),
        overlays=("Regions map one to one onto forms; hood and torso share the hoodie region, the cuff line sits inside the trousers region.", "Regionen entsprechen den Formen eins zu eins; Kapuze und Rumpf teilen sich die Hoodie-Region, die Umschlaglinie liegt in der Hosenregion."))
    d[50] = dict(
        silhouette=("Full articulation: face profile with ear, eye, brow and mouth under a ribbed beanie with cuff, hoodie with hood, drawstrings, pocket and hem band, sleeves with cuffs, mittens with thumb and finger lines, trousers with cuffs and seams, vulcanised shoes with sole and laces, and a deck with grip tape line, baseplates, hangers, wheels and bearings.", "Volle Gliederung: Gesichtsprofil mit Ohr, Auge, Braue und Mund unter einer gerippten Mütze mit Umschlag, Hoodie mit Kapuze, Kordeln, Tasche und Saumbündchen, Ärmel mit Bündchen, Fäustlinge mit Daumen und Fingerlinien, Hose mit Umschlägen und Nähten, vulkanisierte Schuhe mit Sohle und Schnürung sowie Deck mit Grip-Tape-Linie, Grundplatten, Bügeln, Rädern und Lagern."),
        retained=("Every construction element of the board and the clothing down to bearings, drawstrings and laces.", "Jedes Konstruktionselement von Brett und Kleidung bis hin zu Lagern, Kordeln und Schnürung."),
        merged=("Fingers are three lines on one mitten; the sock is part of the shoe region.", "Finger sind drei Linien auf einem Fäustling; die Socke gehört zur Schuhregion."),
        omitted=("Nostril, eyelashes, fabric folds, shoe stitching, deck graphic (below 0.1 mm paper).", "Nasenloch, Wimpern, Stofffalten, Schuhnähte, Deckgrafik (unter 0,1 mm Papier)."),
        thinElements=("Deck 0.012 m = 0.24 mm; grip tape and laces are 0.13 mm lines inside their forms; no feature is thickened.", "Deck 0,012 m = 0,24 mm; Griptape und Schnürung sind 0,13-mm-Linien innerhalb ihrer Formen; nichts wird verdickt."),
        hatching=("Knit rib hatch on the beanie (0.5 mm spacing, 0.13 mm pen).", "Strickrippen-Schraffur auf der Mütze (0,5 mm Abstand, 0,13 mm Stift)."),
        styles=("0.25 mm silhouettes, 0.18 mm detail, 0.13 mm texture; tones as at 1:100.", "0,25-mm-Silhouetten, 0,18-mm-Detail, 0,13-mm-Textur; Töne wie bei 1:100."),
        overlays=("Every region has its own form; detail lines lie inside the form of the region they describe.", "Jede Region hat ihre eigene Form; Detaillinien liegen innerhalb der Form der Region, die sie beschreiben."))
    return d


PENS = {100: dict(body=0.2, detail=0.13, texture=0.13, glazing=0.13, ground=0.3), 50: dict(body=0.25, detail=0.18, texture=0.13, glazing=0.13, ground=0.35)}
VR = {500: (350, 1000), 200: (150, 350), 100: (75, 150), 50: (20, 75)}


def build():
    import skate_decisions
    shapes = all_shapes()
    d = skate_decisions.decisions()
    reps = {"500": skate_decisions.unavailable500(), "200": skate_decisions.unavailable200()}
    for sc in (100, 50):
        reps[str(sc)] = rep_authored(sc, VR[sc], PENS[sc], skate_decisions.HATCHES.get(sc, []), shapes[sc], d[sc])
    f = P["cruising"]
    xs0, ys0, xs1, ys1 = bounds(f, shapes[50])
    dims = {"heightM": round(ys1 - ys0, 3), "widthM": round(xs1 - xs0, 3), "depthM": 0.3, "massKg": round(MASS + 2.25, 2)}
    print("dims", dims)
    for sc in (100,):
        b = bounds(f, shapes[sc])
        print(sc, "h", round(b[3] - b[1], 3), "w", round(b[2] - b[0], 3), "y0", round(b[1], 3))
    a = {
        "id": "skateboarder", "category": "humans",
        "title": tx("Skateboarder", "Skateboarder:in"),
        "summary": tx("A 19-year-old, 1.78 m skateboarder in beanie, hooded sweatshirt, cotton trousers and vulcanised shoes on a 0.81 m street deck: cruising, pushing off, a crouch and a tail-snap ollie with a flight phase; two individually drawn scales (1:100 and 1:50); 1:500 and 1:200 are declared unavailable.", "Ein 19-jähriger, 1,78 m großer Skateboarder in Mütze, Kapuzenpullover, Baumwollhose und vulkanisierten Schuhen auf einem 0,81 m langen Street-Deck: Rollen, Anschieben, Hocke und ein Ollie mit Flugphase; zwei einzeln gezeichnete Maßstäbe (1:100 und 1:50); 1:500 und 1:200 sind als nicht verfügbar erklärt."),
        "tags": ["human", "skateboarder", "youth", "sport", "board", "flight-phase", "wheels"],
        "provenance": {"author": "Semio staffage content worker NW-H2", "method": "handcrafted", "created": "2026-10-29", "references": refs("de-leva-1996", "dubois-1916", "iso-9920-2007", "ainsworth-2011", "ashrae-2021", "gagge-1971", "gupta-2010", "hoeppe-1981", "mifflin-1990", "incropera-2011", extra=[{"id": "board-spec", "citation": "Typical commercial street skateboard: 8.0 in x 31.9 in seven-ply maple deck, wheelbase 14.5 in, 54 mm wheels, aluminium trucks, mass about 2.2 kg. Dimensions are authored from catalogue ranges, not from a single source."}]),
            "notes": tx("Newly authored for semio.staffage/v2: a side view of a rider turned about 45 degrees to the direction of travel, so the torso is drawn 0.28 m deep and the shoes at full length. Poses are solved by inverse kinematics in the board frame; the ground slides under the pushing foot in the push cycle (root motion none).", "Neu erstellt für semio.staffage/v2: Seitenansicht eines um etwa 45 Grad zur Fahrtrichtung gedrehten Fahrers, daher ist der Rumpf 0,28 m tief und sind die Schuhe in voller Länge gezeichnet. Die Posen sind per inverser Kinematik im Brettsystem gelöst; im Schubzyklus gleitet der Boden unter dem Schiebefuß (Wurzelbewegung none).")},
        "dimensions": dims, "materials": MAT,
        "rig": {"kind": "anatomical", "root": "board", "bones": bones}, "anchors": anchors,
        "representations": reps, "symbols": [], "poses": pose_list(), "clips": clips(), "thermal": thermal(),
        "structure": structure([("Quasi-static model (the frame of the board accelerates, so inertial loads are not applied): segment masses as point masses at de Leva centres, the board 1.9 kg plus two 0.175 kg wheel sets; the wheel contacts (and the tail tip during the pop) are unilateral supports with low rolling friction. In the flight phase no contact is active.", "Quasi-statisches Modell (das Bezugssystem des Bretts beschleunigt, Trägheitslasten werden daher nicht angesetzt): Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva, das Brett 1,9 kg plus zwei Radsätze zu 0,175 kg; die Radkontakte (und beim Absprung die Heckspitze) sind einseitige Auflager mit geringer Rollreibung. In der Flugphase ist kein Kontakt aktiv.")]),
        "presentation": {"defaultPose": "cruising", "defaultClip": "push-cycle", "defaultScale": 100, "viewpoints": ["elevation"]},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored",
    }
    return a


if __name__ == "__main__":
    dump(asset_doc(build()), out_path("skateboarder"), depth=5)
    print("written")
