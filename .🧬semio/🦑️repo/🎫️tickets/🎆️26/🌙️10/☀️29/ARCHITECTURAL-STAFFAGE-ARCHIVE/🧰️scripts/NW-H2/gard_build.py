"""Builds S/🗂️catalog/👤️humans/kneeling-gardener/🔣️.json"""
from lib import *
from common_build import *
from gard_rig import *
import gard_art, gard_decisions

anchors = build_anchors(L)
A_SKIN = dubois(H, MASS)
P = poses()
f = P["kneeling-weeding"]
# key poses of the digging stroke: only the working arm changes
def dig(tip, hd, torso=32):
    g = kneel((0.05, 0.50), torso_deg=torso, head_deg=torso + 16, far_dx=0.05)
    c, s = rot(hd)
    wr = (tip[0] - (0.07 + TROWEL_L) * c, tip[1] - (0.07 + TROWEL_L) * s)
    arm_to(g, "r", wr, hd, -1)
    g.rel("trowel", 0)
    arm_to(g, "l", (0.06, 0.42), -80, -1)
    return g
P["dig-up"] = dig((0.64, 0.22), -28, 30)
P["dig-in"] = dig((0.70, 0.0), -55, 32)
P["dig-pull"] = dig((0.54, 0.0), -68, 36)
for n in ("dig-up", "dig-in", "dig-pull"):
    assert not joint_report(P[n]), (n, joint_report(P[n]))

MAT = [
    {"id": "skin", "title": tx("Skin", "Haut"), "emissivity": 0.98, "solarAbsorptivity": 0.6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "straw", "title": tx("Woven straw hat over hair", "Strohhut über Haar"), "emissivity": 0.9, "solarAbsorptivity": 0.45, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "linen", "title": tx("Linen shirt", "Leinenhemd"), "emissivity": 0.92, "solarAbsorptivity": 0.35, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "canvas", "title": tx("Cotton canvas trousers", "Baumwoll-Canvas-Hose"), "emissivity": 0.92, "solarAbsorptivity": 0.65, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "glove", "title": tx("Cotton and leather gardening glove", "Garten-Handschuh aus Baumwolle und Leder"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "source": "authored-estimate", "reference": "iso-9920-2007"},
    {"id": "rubber-clog", "title": tx("Rubber garden clog with sock", "Gummi-Gartenclog mit Socke"), "emissivity": 0.92, "solarAbsorptivity": 0.7, "source": "authored-estimate"},
    {"id": "eva-foam", "title": tx("Closed-cell EVA foam kneeling pad", "Kniekissen aus geschlossenzelligem EVA-Schaum"), "emissivity": 0.92, "solarAbsorptivity": 0.7, "densityKgM3": 60, "conductivityWmK": 0.04, "specificHeatJKgK": 2000, "source": "authored-estimate", "reference": "incropera-2011"},
    {"id": "steel", "title": tx("Hand trowel (steel blade, plastic handle)", "Handschaufel (Stahlblatt, Kunststoffgriff)"), "emissivity": 0.3, "solarAbsorptivity": 0.5, "densityKgM3": 7800, "conductivityWmK": 50, "specificHeatJKgK": 490, "source": "literature", "reference": "incropera-2011"},
    {"id": "polyethylene", "title": tx("Flexible polyethylene trug with weeds", "Flexibler Polyethylen-Trog mit Unkraut"), "emissivity": 0.92, "solarAbsorptivity": 0.6, "densityKgM3": 950, "specificHeatJKgK": 2300, "source": "authored-estimate", "reference": "incropera-2011"},
]
fc = dict(shirt=1.2, trs=1.18, hat=1.1)


def regions():
    R = []
    R.append(region("face", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.04 * A_SKIN, 0.15, "sphere", 0.35, [], 15000, "head", 0.06, 0.04, ref="dubois-1916"))
    R.append(region("hat", "Straw hat and hair", "Strohhut und Haar", "clothing", "straw", 0.04 * A_SKIN * fc["hat"] * 3, 0.3, "horizontal-plate-up", 0.8, [
        layer("hair-pack", "Hair and air under the hat", "Haar und Luft unter dem Hut", "hair", "straw", 0.02, 0.06, 300),
        layer("straw", "Woven straw", "Strohgeflecht", "clothing", "straw", 0.003, 0.02, 500)], 3000, "head", 0.04, 0.04))
    R.append(region("shirt-torso", "Linen shirt, trunk", "Leinenhemd, Rumpf", "clothing", "linen", 0.36 * A_SKIN * fc["shirt"], 0.3, "cylinder", 0.5, [
        layer("torso-air", "Air between skin and shirt", "Luft zwischen Haut und Hemd", "air-gap", "linen", 0.006, 0.03),
        layer("shirt", "Linen long-sleeve shirt (0.25 clo)", "Leinenhemd mit langen Ärmeln (0,25 clo)", "clothing", "linen", 0.0008, clo_r(0.25, fc["shirt"]), 250)], 18000, "trunk", 0.40, 0.36))
    R.append(region("shirt-sleeves", "Linen sleeves above the elbow", "Leinenärmel oberhalb des Ellenbogens", "clothing", "linen", 0.07 * A_SKIN * fc["shirt"], 0.09, "cylinder", 0.5, [
        layer("sleeve-air", "Air in the sleeve", "Luft im Ärmel", "air-gap", "linen", 0.006, 0.03),
        layer("sleeve", "Linen sleeve (0.25 clo)", "Leinenärmel (0,25 clo)", "clothing", "linen", 0.0008, clo_r(0.25, fc["shirt"]), 250)], 14000, "arm", 0.05, 0.07))
    R.append(region("forearms", "Bare forearms (sleeves rolled)", "Nackte Unterarme (Ärmel hochgekrempelt)", "skin", "skin", 0.07 * A_SKIN, 0.07, "cylinder", 0.5, [], 14000, "arm", 0.04, 0.07, ref="dubois-1916"))
    R.append(region("gloves", "Gardening gloves", "Gartenhandschuhe", "clothing", "glove", 0.05 * A_SKIN * 1.1, 0.06, "cylinder", 0.5, [
        layer("glove-air", "Air in the glove", "Luft im Handschuh", "air-gap", "glove", 0.003, 0.02),
        layer("glove", "Cotton glove with leather palm (0.1 clo)", "Baumwollhandschuh mit Lederhandfläche (0,1 clo)", "clothing", "glove", 0.0015, clo_r(0.10, 1.1), 400)], 6000, "hand", 0.02, 0.05))
    R.append(region("trousers", "Canvas trousers with knee patches", "Canvas-Hose mit Kniepolstern", "clothing", "canvas", 0.31 * A_SKIN * fc["trs"], 0.12, "cylinder", 0.4, [
        layer("leg-air", "Air between skin and trousers", "Luft zwischen Haut und Hose", "air-gap", "canvas", 0.006, 0.03),
        layer("canvas", "Cotton canvas (0.28 clo)", "Baumwoll-Canvas (0,28 clo)", "clothing", "canvas", 0.001, clo_r(0.28, fc["trs"]), 600)], 18000, "leg", 0.34, 0.31))
    R.append(region("clogs", "Garden clogs with socks", "Gartenclogs mit Socken", "clothing", "rubber-clog", 0.06 * A_SKIN, 0.12, "horizontal-plate-up", 0.3, [
        layer("sock", "Cotton sock (0.04 clo)", "Baumwollsocke (0,04 clo)", "clothing", "rubber-clog", 0.002, 0.0062, 300),
        layer("clog-air", "Air in the clog", "Luft im Clog", "air-gap", "rubber-clog", 0.01, 0.04),
        layer("clog", "Rubber clog shell", "Clog-Schale aus Gummi", "shell", "rubber-clog", 0.005, 0.02, 1200)], 7000, "foot", 0.05, 0.06))
    R.append(region("pad", "Kneeling pad", "Kniekissen", "surface", "eva-foam", 0.28, 0.5, "horizontal-plate-up", 0.7, [], 4000, None, None, None, model="passive", ref="incropera-2011"))
    R.append(region("trowel", "Hand trowel", "Handschaufel", "surface", "steel", 0.025, 0.3, "cylinder", 0.5, [], 4000, None, None, None, model="passive", ref="incropera-2011"))
    R.append(region("trug", "Trug with weeds", "Trog mit Unkraut", "surface", "polyethylene", 0.62, 0.5, "vertical-plate", 0.5, [], 3000, None, None, None, model="passive", ref="incropera-2011"))
    return R


REG = regions()
RIDS = [r["id"] for r in REG]


def bind(act, br, sun=0.35, knee_contact=0.3, glove_soil=0.0, plume=0.5):
    reg = {}
    for rid in RIDS:
        if rid in ("face",):
            reg[rid] = rbind(1.0, 0.15)
        elif rid == "hat":
            reg[rid] = rbind(1.0, 0.85)
        elif rid == "gloves":
            reg[rid] = rbind(1.0 - glove_soil, sun, contact=(glove_soil, 17) if glove_soil else None)
        elif rid == "trousers":
            reg[rid] = rbind(1.0 - knee_contact, sun * 0.8, contact=(knee_contact, 22))
        elif rid == "clogs":
            reg[rid] = rbind(0.8, sun * 0.6, contact=(0.2, 22))
        elif rid == "pad":
            reg[rid] = rbind(0.6, 0.85, contact=(0.4, 19))
        elif rid == "trowel":
            reg[rid] = rbind(1.0, 0.7, contact=(0.1, 17))
        elif rid == "trug":
            reg[rid] = rbind(1.0, 0.8)
        elif rid in ("forearms",):
            reg[rid] = rbind(1.0, sun)
        else:
            reg[rid] = rbind(1.0, sun)
    return {"activityMet": act, "breathing": br, "regions": reg, "emitters": {"plume": plume}}


TB = {
    "kneeling-weeding": bind(3.5, "work", glove_soil=0.0),
    "kneeling-upright": bind(1.8, "rest", plume=0.5),
    "reaching-trug": bind(3.8, "work", plume=0.8),
    "dig-up": bind(4.0, "work", plume=0.8),
    "dig-in": bind(4.2, "effort", plume=1.0),
    "dig-pull": bind(4.2, "effort", plume=1.0),
}
GROUND = ["pad-ground", "shin-l", "shin-r", "instep-l", "instep-r"]


def pose_list():
    out = []
    spec = [
        ("kneeling-weeding", "Kneeling and weeding", "Kniend jäten", "Kneeling on the pad, torso inclined 32 degrees, the left hand presses on the thigh and the right hand holds the trowel in the soil.", "Kniend auf dem Kissen, Rumpf um 32 Grad geneigt, die linke Hand stützt sich auf den Oberschenkel, die rechte hält die Schaufel im Boden.", False),
        ("kneeling-upright", "Kneeling upright", "Aufrecht knien", "Sitting upright on the knees with the trowel resting in front of the chest, looking along the bed.", "Aufrecht auf den Knien mit der Schaufel vor der Brust, der Blick folgt dem Beet.", False),
        ("reaching-trug", "Reaching to the trug", "Zum Trog greifen", "Leaning far forward to drop weeds into the trug, the left hand steadies on the knee.", "Weit nach vorn gebeugt, um Unkraut in den Trog zu legen, die linke Hand stützt sich am Knie.", False),
        ("dig-up", "Trowel raised", "Schaufel angehoben", "Key pose of the digging stroke: trowel raised above the soil.", "Schlüsselpose des Grabens: Schaufel über dem Boden.", True),
        ("dig-in", "Trowel in the soil", "Schaufel im Boden", "Key pose of the digging stroke: blade thrust into the soil.", "Schlüsselpose des Grabens: Blatt in den Boden gestoßen.", True),
        ("dig-pull", "Trowel pulled back", "Schaufel zurückgezogen", "Key pose of the digging stroke: soil pulled towards the knees.", "Schlüsselpose des Grabens: Erde zu den Knien gezogen.", True),
    ]
    for pid, en, de, den, dee, key in spec:
        out.append(pose_record(pid, en, de, den, dee, P[pid], GROUND, TB[pid], key=key))
    return out


def clips():
    ph = lambda i, fr, t, en, de: (i, fr, t, (en, de), False)
    c = []
    c.append(clip_record("dig-cycle", "Digging stroke", "Grabbewegung",
        "Loop of 1.6 s: the trowel is raised, thrust into the soil, pulled back and raised again while the knees, the pad and the pressing left hand stay still.",
        "Schleife von 1,6 s: Die Schaufel wird gehoben, in den Boden gestoßen, zurückgezogen und wieder gehoben, während Knie, Kissen und die stützende linke Hand ruhen.",
        1.6, True, [(0, "dig-up", "smooth"), (0.4, "dig-in", "smooth"), (0.9, "dig-pull", "smooth"), (1.3, "kneeling-weeding", "smooth")],
        [(g, 0, 1.6) for g in GROUND],
        [ph("raise", 0, 0.4, "Raise and thrust", "Heben und Stoßen"), ph("pull", 0.4, 1.0, "Pull", "Ziehen"), ph("reset", 1.0, 1.6, "Return", "Rückführung")],
        [(0.4, "motion", ("Blade enters the soil", "Blatt dringt in den Boden ein"), None), (0.9, "thermal", ("Exhalation at the end of the pull", "Ausatmen am Ende des Zugs"), None)],
        [("breathing", "step", [(0, 1), (0.4, 2), (1.3, 1)])]))
    c.append(clip_record("reach-to-trug", "Dropping weeds into the trug", "Unkraut in den Trog legen",
        "Weeds are pulled, the gardener leans forward to the trug, lets them fall and returns to weeding; one slow round of 4 s.",
        "Unkraut wird gezupft, die Gärtnerin beugt sich zum Trog, lässt es fallen und kehrt zum Jäten zurück; eine langsame Runde von 4 s.",
        4.0, False, [(0, "kneeling-weeding", "smooth"), (1.2, "kneeling-weeding", "smooth"), (2.0, "reaching-trug", "smooth"), (2.6, "reaching-trug", "smooth"), (3.6, "kneeling-weeding", "smooth")],
        [(g, 0, 4.0) for g in GROUND],
        [ph("pull", 0, 1.2, "Pulling weeds", "Unkraut zupfen"), ph("reach", 1.2, 2.6, "Reaching the trug", "Zum Trog greifen"), ph("return", 2.6, 4.0, "Back to weeding", "Zurück zum Jäten")],
        [(2.3, "load-off", ("Weeds are released into the trug", "Unkraut fällt in den Trog"), None)],
        [("breathing", "step", [(0, 1), (1.2, 2), (2.6, 1)])]))
    return c


def thermal():
    pats = [
        breath_pattern("rest", "Resting breathing", "Ruheatmung", 14, 0.5, 0.38, 0.04, 0.45, 0.13, "nose", "gupta-2010"),
        breath_pattern("work", "Light garden work", "Leichte Gartenarbeit", 18, 0.9, 0.42, 0.02, 0.50, 0.06, "nose-and-mouth", "ainsworth-2011"),
        breath_pattern("effort", "Digging effort", "Anstrengung beim Graben", 24, 1.3, 0.44, 0.02, 0.52, 0.02, "mouth", "ainsworth-2011"),
    ]
    env = {"airTempC": 21, "meanRadiantTempC": 26, "skyTempC": 10, "groundTempC": 20, "relativeHumidity": 0.55, "windMS": 0.5, "windDirectionDeg": 180, "solarWM2": 700, "diffuseFraction": 0.2, "pressurePa": 101325}
    return {"environment": env, "baseMet": 1.8, "body": physiology(MASS, H, False, 58),
        "respiration": {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95, "patterns": pats, "motion": [{"bone": "torso", "angleDeg": 1.0, "dy": 0.004}, {"bone": "pelvis", "angleDeg": 0.4, "dy": 0.003}], "source": "authored-estimate", "reference": "hoeppe-1981"},
        "passengers": [], "operating": [], "regions": REG,
        "emitters": nose_mouth_emitters(-150, -100) + [plume_emitter("shirt-torso", 0.3)],
        "assumptions": [
            tx("DuBois skin area for 1.66 m and 64 kg; a linen long-sleeve shirt with rolled sleeves (0.25 clo), canvas trousers (0.28 clo), cotton gloves (0.10 clo), a straw hat shading the face and rubber clogs. Garment values are authored estimates guided by ISO 9920 (1 clo = 0.155 m2K/W); layer resistances include the enclosed air.", "DuBois-Hautfläche für 1,66 m und 64 kg; Leinenhemd mit hochgekrempelten Ärmeln (0,25 clo), Canvas-Hose (0,28 clo), Baumwollhandschuhe (0,10 clo), ein Strohhut, der das Gesicht beschattet, und Gummiclogs. Die Kleidungswerte sind Autorenschätzungen nach ISO 9920 (1 clo = 0,155 m2K/W); die Schichtwiderstände enthalten die eingeschlossene Luft."),
            tx("Metabolic rate 3.5 to 4.2 met for weeding and digging, 1.8 met kneeling upright (Compendium of Physical Activities lists garden work between about 3 and 5 METs; mapped to thermal met as an estimate). Basal metabolism by Mifflin-St Jeor for a 58-year-old woman.", "Stoffwechsel 3,5 bis 4,2 met beim Jäten und Graben, 1,8 met beim aufrechten Knien (das Compendium of Physical Activities nennt für Gartenarbeit etwa 3 bis 5 MET; als Schätzung auf thermisches met übertragen). Grundumsatz nach Mifflin-St Jeor für eine 58-jährige Frau."),
            tx("The shins and insteps rest on the foam pad (conduction fraction 0.3 at 22 degrees Celsius); the pad stands on soil at 20 degrees Celsius. The first respiration pattern (rest) is the reference of the chest motion.", "Schienbeine und Fußrücken ruhen auf dem Schaumkissen (Leitungsanteil 0,3 bei 22 Grad Celsius); das Kissen steht auf Boden mit 20 Grad Celsius. Das erste Atemmuster (Ruhe) ist die Referenz der Brustbewegung.")]}


PENS = {100: dict(body=0.2, detail=0.13, texture=0.13, glazing=0.13, ground=0.3), 50: dict(body=0.25, detail=0.18, texture=0.13, glazing=0.13, ground=0.35)}
VR = {500: (350, 1000), 200: (150, 350), 100: (75, 150), 50: (20, 75)}


def build():
    shapes = gard_art.all_shapes()
    d = gard_decisions.decisions()
    reps = {"500": gard_decisions.unavailable500(), "200": gard_decisions.unavailable200()}
    hat = gard_decisions.HATCHES
    for sc in (100, 50):
        reps[str(sc)] = rep_authored(sc, VR[sc], PENS[sc], hat.get(sc, []), shapes[sc], d[sc])
    xs0, ys0, xs1, ys1 = bounds(P["kneeling-weeding"], shapes[50])
    b100 = bounds(P["kneeling-weeding"], shapes[100])
    dims = {"heightM": round(((ys1 - ys0) + (b100[3] - b100[1])) / 2, 3), "widthM": round(xs1 - xs0, 3), "depthM": 0.5, "massKg": round(MASS + 0.6 + 1.6 + 0.25, 2)}
    print("dims", dims)
    for sc in (100,):
        b = bounds(P["kneeling-weeding"], shapes[sc])
        print(sc, "h", round(b[3] - b[1], 3), "w", round(b[2] - b[0], 3), "y0", round(b[1], 3))
    return {
        "id": "kneeling-gardener", "category": "humans", "title": tx("Kneeling gardener", "Kniende Gärtnerin"),
        "summary": tx("A 58-year-old, 1.66 m gardener kneeling on a foam pad in a straw hat, rolled linen sleeves, canvas trousers and clogs, weeding with a hand trowel beside a trug: weeding, upright kneeling and reaching to the trug, a digging loop and a trug round; two individually drawn scales (1:100 and 1:50); 1:500 and 1:200 are declared unavailable.", "Eine 58-jährige, 1,66 m große Gärtnerin kniet in Strohhut, hochgekrempeltem Leinenhemd, Canvas-Hose und Clogs auf einem Schaumkissen und jätet mit der Handschaufel neben einem Trog: Jäten, aufrechtes Knien und Greifen zum Trog, eine Grabschleife und eine Troglauf-Runde; zwei einzeln gezeichnete Maßstäbe (1:100 und 1:50); 1:500 und 1:200 sind als nicht verfügbar erklärt."),
        "tags": ["human", "gardener", "kneeling", "garden", "tool", "low-posture", "adult"],
        "provenance": {"author": "Semio staffage content worker NW-H2", "method": "handcrafted", "created": "2026-10-29",
            "references": refs("de-leva-1996", "dubois-1916", "iso-9920-2007", "ainsworth-2011", "ashrae-2021", "gagge-1971", "gupta-2010", "hoeppe-1981", "mifflin-1990", "incropera-2011", extra=[{"id": "garden-spec", "citation": "Typical garden equipment: 0.6 x 0.3 x 0.04 m EVA kneeling pad, steel hand trowel about 0.3 m long and 0.25 kg, flexible 0.5 m polyethylene trug. Dimensions and masses are authored estimates from catalogue ranges."}]),
            "notes": tx("Newly authored for semio.staffage/v2: a low posture solved by inverse kinematics with the shins lying on the pad; the pad is the rig root, so every pose keeps the ground contact of the pad.", "Neu erstellt für semio.staffage/v2: eine niedrige Haltung, per inverser Kinematik gelöst, mit den Schienbeinen auf dem Kissen; das Kissen ist die Rig-Wurzel, daher behält jede Pose den Bodenkontakt des Kissens.")},
        "dimensions": dims, "materials": MAT,
        "rig": {"kind": "anatomical", "root": "pad", "bones": bones}, "anchors": anchors,
        "representations": reps, "symbols": [], "poses": pose_list(), "clips": clips(), "thermal": thermal(),
        "structure": structure([("Inertial model: segment masses as point masses at de Leva centres, pad 0.6 kg, trowel 0.25 kg on the right hand and trug 1.6 kg; the pad on the soil is the permanent unilateral support, the shins and insteps rest on the pad.", "Trägheitsmodell: Segmentmassen als Punktmassen in den Schwerpunkten nach de Leva, Kissen 0,6 kg, Schaufel 0,25 kg an der rechten Hand und Trog 1,6 kg; das Kissen auf dem Boden ist das einseitige Auflager, Schienbeine und Fußrücken ruhen auf dem Kissen.")]),
        "presentation": {"defaultPose": "kneeling-weeding", "defaultClip": "dig-cycle", "defaultScale": 100, "viewpoints": ["elevation"]},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}


if __name__ == "__main__":
    dump(asset_doc(build()), out_path("kneeling-gardener"), depth=5)
    print("written")
