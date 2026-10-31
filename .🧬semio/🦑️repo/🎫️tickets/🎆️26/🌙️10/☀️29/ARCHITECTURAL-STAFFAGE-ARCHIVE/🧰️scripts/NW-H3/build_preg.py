from a_preg_geom import *
from h3asm import *
from h3check import check

aid = "pregnant-person"
SKN = dubois(71, 1.66)
EXTRA = {"hytten-chamberlain-1980": "Hytten, F. E. and Chamberlain, G. (1980). Clinical Physiology in Obstetrics. Blackwell, Oxford; composition of the weight gain of pregnancy (fetus, placenta, amniotic fluid, uterus, breast, blood, fluid, fat), used as guidance, values authored.",
         "butte-2004": "Butte, N. F., Wong, W. W., Treuth, M. S., Ellis, K. J. and O'Brian Smith, E. (2004). Energy requirements during pregnancy based on total energy expenditure and energy deposition. American Journal of Clinical Nutrition 79(6), 1078-1087 (basal metabolism rises steadily across gestation, about 10.7 kcal per week on average).",
         "lof-2005": "Lof, M., Olausson, H., Bostrom, K., Janerot-Sjöberg, B., Sohlstrom, A. and Forsum, E. (2005). Changes in basal metabolic rate during pregnancy in relation to changes in body weight and composition, cardiac output, insulin-like growth factor I, and thyroid hormones and in relation to fetal growth. American Journal of Clinical Nutrition 81(3), 678-685.",
         "lomauro-aliverti-2015": "LoMauro, A. and Aliverti, A. (2015). Respiratory physiology of pregnancy. Breathe 11(4), 297-301 (minute ventilation rises by up to 48 % through a larger tidal volume at an almost unchanged rate; the diaphragm is displaced upwards and rib-cage breathing dominates)."}
mats = [m for m in BASE["materials"] if m["id"] in ("skin", "hair", "cotton-underwear", "cotton-sock")] + [
    material("knit-wool-sweater", "Oversized wool-blend knit sweater", "Übergroßer Strickpullover aus Wollmischung", 0.92, 0.6, cp=1400),
    material("stretch-leggings", "Cotton-elastane maternity leggings", "Baumwoll-Elastan-Umstandsleggings", 0.93, 0.85, cp=1300),
    material("leather-ankle-boot", "Leather ankle boots with rubber sole", "Lederstiefeletten mit Gummisohle", 0.9, 0.8, cp=1500),
]
cap_skin = 13000
f_s = fcl_of(0.09 + 0.30 + 0.20)
s_layers = [garment("tee", "Cotton maternity T-shirt", "Baumwoll-Umstandsshirt", 0.09, "cotton-underwear", 0.0008, 150, f_s),
            garment("knit", "Oversized knit sweater", "Übergroßer Strickpullover", 0.30 + 0.20, "knit-wool-sweater", 0.008, 650, f_s)]
f_b = fcl_of(0.09 + 0.12)
b_layers = [garment("tee", "Cotton maternity T-shirt, stretched", "Baumwoll-Umstandsshirt, gedehnt", 0.09, "cotton-underwear", 0.0006, 120, f_b),
            garment("knit", "Knit sweater stretched over the abdomen", "Über den Bauch gespannter Strickpullover", 0.12, "knit-wool-sweater", 0.005, 400, f_b)]
f_l = fcl_of(0.04 + 0.15)
l_layers = [garment("briefs", "Cotton maternity briefs", "Baumwoll-Umstandsslip", 0.04, "cotton-underwear", 0.0008, 100, f_l),
            garment("leggings", "Maternity leggings", "Umstandsleggings", 0.15, "stretch-leggings", 0.001, 320, f_l)]
bo_layers = [garment("socks", "Cotton socks", "Baumwollsocken", 0.03, "cotton-sock", 0.002, 120, 1.0),
             garment("boot", "Leather ankle boot", "Lederstiefelette", 0.05, "leather-ankle-boot", 0.004, 900, 1.0)]
hair_layers = [{"id": "hair", "title": T("Shoulder-length hair", "Schulterlanges Haar"), "kind": "hair", "material": "hair", "thicknessM": 0.02, "resistanceM2KW": 0.06, "vapourResistanceM2PaW": 9.6, "heatCapacityJM2K": 350, "source": "authored-estimate"}]
cap = lambda ls, base=cap_skin: base + sum(l["heatCapacityJM2K"] for l in ls)
regions = [
    body_region(FACE, "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", SKN * 0.07, 1.0, "head", 0.06, 0.07, 0.2, "sphere", 0.5, cap_skin, [], ref="dubois-1916"),
    body_region(HAIR, "Hair", "Haar", "hair", "hair", SKN * 0.03, 1.0, "head", 0.03, 0.03, 0.2, "sphere", 0.8, cap(hair_layers, 8000), hair_layers),
    body_region(HAND, "Bare hands", "Unbedeckte Hände", "skin", "skin", SKN * 0.05, 1.0, "hand", 0.04, 0.05, 0.1, "cylinder", 0.2, cap_skin, [], ref="dubois-1916"),
    body_region(SWEAT, "T-shirt and sweater on chest, back and arms", "T-Shirt und Pullover an Brust, Rücken und Armen", "clothing", "knit-wool-sweater", SKN * 0.39, f_s, "trunk", 0.40, 0.39, 0.3, "vertical-plate", 0.45, cap(s_layers), s_layers),
    body_region(BELLY_R, "Fabric stretched over the abdomen", "Über den Bauch gespannter Stoff", "clothing", "knit-wool-sweater", SKN * 0.10, f_b, "trunk", 0.14, 0.10, 0.3, "sphere", 0.55, cap(b_layers), b_layers),
    body_region(LEG, "Leggings over the legs", "Leggings über den Beinen", "clothing", "stretch-leggings", SKN * 0.28, f_l, "leg", 0.27, 0.28, 0.13, "cylinder", 0.1, cap(l_layers), l_layers),
    body_region(BOOT, "Socks and ankle boots", "Socken und Stiefeletten", "surface", "leather-ankle-boot", SKN * 0.08, 1.0, "foot", 0.06, 0.08, 0.1, "horizontal-plate-up", 0.1, cap(bo_layers), bo_layers),
]
resp = {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95,
        "patterns": [
            {"id": "rest-third-trimester", "title": T("Quiet standing, third trimester", "Ruhiges Stehen, drittes Trimester"), "rateBpm": 15, "tidalVolumeL": 0.70, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "lomauro-aliverti-2015"},
            {"id": "walking-third-trimester", "title": T("Slow walking, third trimester", "Langsames Gehen, drittes Trimester"), "rateBpm": 21, "tidalVolumeL": 1.05, "inhaleFraction": 0.45, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.03, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "lomauro-aliverti-2015"},
            {"id": "deep-stretching-breath", "title": T("Deep breath while stretching the back", "Tiefer Atemzug beim Dehnen des Rückens"), "rateBpm": 10, "tidalVolumeL": 1.0, "inhaleFraction": 0.42, "pauseAfterInhaleFraction": 0.06, "exhaleFraction": 0.46, "pauseAfterExhaleFraction": 0.06, "route": "nose", "source": "authored-estimate", "reference": "lomauro-aliverti-2015"}],
        "motion": [{"bone": "torso", "angleDeg": 0.9, "dy": 0.004}, {"bone": "belly", "angleDeg": 0.3, "dy": 0.0}], "source": "authored-estimate", "reference": "lomauro-aliverti-2015"}
thermal = OrderedDict()
thermal["environment"] = {"airTempC": 16, "meanRadiantTempC": 16, "skyTempC": 4, "groundTempC": 15, "relativeHumidity": 0.65, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
thermal["baseMet"] = 1.3
bas = round(mifflin("f", 58, 165, 31) * 1.0 + 15.6, 1)
thermal["body"] = {"species": "human", "massKg": 71, "skinAreaM2": SKN, "basalMetabolismW": bas, "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 100, "minSkinBloodFlowLM2H": 1.5, "source": "authored-estimate", "reference": "butte-2004"}
thermal["respiration"] = resp
thermal["operating"] = []
thermal["passengers"] = []
thermal["regions"] = regions
thermal["emitters"] = breath_emitters(plume_region=SWEAT, plume_d=0.3)
thermal["assumptions"] = [
    T(f"Third trimester, week 36: 71 kg (58 kg before pregnancy plus 13 kg gain), 1.66 m, skin area {SKN} m2 after DuBois and DuBois (1916) at the current weight; the abdomen is its own region (10 % of the skin, 14 % of the metabolism) because the fetal unit adds heat production and a stretched, thinner garment layer.",
      f"Drittes Trimester, Woche 36: 71 kg (58 kg vor der Schwangerschaft plus 13 kg Zunahme), 1,66 m, Hautfläche {SKN} m2 nach DuBois und DuBois (1916) beim aktuellen Gewicht; der Bauch ist eine eigene Region (10 % der Haut, 14 % des Stoffwechsels), weil die Frucht Wärme erzeugt und der Stoff dort gedehnt und dünner ist."),
    T(f"Basal metabolism {bas} W = Mifflin-St Jeor of the pre-pregnancy woman (58 kg, 165 cm, 31 years: {mifflin('f', 58, 165, 31)} W) plus about 15.6 W (+25 %): the mean rise of about 10.7 kcal per gestational week reported by Butte et al. (2004) would give about 19 W at week 36 and Lof et al. (2005) found a significant rise from week 32; the +25 % is an authored estimate.",
      f"Grundumsatz {bas} W = Mifflin-St Jeor der Frau vor der Schwangerschaft (58 kg, 165 cm, 31 Jahre: {mifflin('f', 58, 165, 31)} W) plus etwa 15,6 W (+25 %): der mittlere Anstieg von etwa 10,7 kcal je Schwangerschaftswoche nach Butte et al. (2004) ergäbe in Woche 36 etwa 19 W und Lof et al. (2005) fanden einen signifikanten Anstieg ab Woche 32; die +25 % sind geschätzt."),
    T("Skin blood flow limits are raised (1.5 minimum, 100 L/(m2h) maximum against 1.0 and 90 for the Gagge default) because pregnancy brings peripheral vasodilation; both numbers are authored estimates.",
      "Die Grenzen der Hautdurchblutung sind angehoben (Minimum 1,5, Maximum 100 L/(m2h) gegenüber 1,0 und 90 im Gagge-Standard), weil die Schwangerschaft eine periphere Gefäßerweiterung bringt; beide Zahlen sind geschätzt."),
    T("Tidal volume rises from about 0.5 to 0.7 L at an almost unchanged rate (minute ventilation up to 48 % in LoMauro and Aliverti 2015); the exact values are authored. The chest wall moves more than the abdomen because the diaphragm is displaced upwards.",
      "Das Atemzugvolumen steigt von etwa 0,5 auf 0,7 L bei fast gleicher Frequenz (Atemminutenvolumen bis zu 48 % nach LoMauro und Aliverti 2015); die genauen Werte sind geschätzt. Der Brustkorb bewegt sich stärker als der Bauch, weil das Zwerchfell nach oben verlagert ist."),
]
EXP = {SWEAT: 0.92, BELLY_R: 0.9}


def tb(met, br, plume):
    return binding(regions, {"body-plume": plume}, met, br, exposed=EXP, contact={BOOT: {"fraction": 0.3, "tempC": 15}})


TI = {"hand-on-back": (T("Standing with a hand on the lower back", "Stehen mit der Hand am unteren Rücken"), T("Leaning back about 5 degrees with the lumbar curve increased, near hand on the lower back, far hand under the abdomen, head up.", "Rund 5 Grad nach hinten gelehnt mit verstärkter Lendenkrümmung, nahe Hand am unteren Rücken, ferne Hand unter dem Bauch, Kopf aufrecht.")),
      "cradling-belly": (T("Cradling the abdomen", "Den Bauch umfassen"), T("Both hands on the front of the abdomen, head bowed towards it, slight lean back.", "Beide Hände auf der Vorderseite des Bauchs, Kopf dem Bauch zugeneigt, leichte Rücklage.")),
      "stretching-back": (T("Stretching the back", "Den Rücken dehnen"), T("Both hands on the lumbar spine, torso leaning back 12 degrees, head tilted up, deep breath.", "Beide Hände an der Lendenwirbelsäule, Oberkörper 12 Grad nach hinten, Kopf nach oben geneigt, tiefer Atemzug."))}
TBK = {"hand-on-back": tb(1.3, "rest-third-trimester", 0.6), "cradling-belly": tb(1.2, "rest-third-trimester", 0.6), "stretching-back": tb(1.4, "deep-stretching-breath", 0.7)}
LOADS = [{"id": "fetal-unit-weight", "kind": "occupant", "title": T("Weight of the fetal unit (fetus, placenta, fluid, uterus)", "Gewicht der Frucht (Fetus, Plazenta, Fruchtwasser, Gebärmutter)"), "anchor": "belly-cog", "fxN": 0, "fyN": -0.0, "source": "authored-estimate"}]
LOADS = []
poses = [{"id": k, "title": TI[k][0], "description": TI[k][1], "bones": POSES[k][0], "ground": "auto", "contacts": CONTACTS, "thermal": TBK[k], "loads": LOADS} for k in ("hand-on-back", "cradling-belly", "stretching-back")]

# ---- waddle walk
G = geom_of(a)
spec = dict(heel=(0, 0.35), b0=0.11, b1=0.66, X0=0.29, clear=0.055, delta_keys=[(0, 12), (0.11, 0), (0.35, 0), (0.66, -20), (0.8, -6), (0.9, 5), (1.0, 12)])
NF, TT, VV = 39, 1.3, 0.85
frames, G = gait_frames(a, NF, TT, VV, spec, ypelvis_nominal=G["hip_y"] - 0.004, bob=0.03)
LEGS = cycle_offsets(a, G, frames, spec, NF)


def walk_key(i):
    off = {k: dict(v) for k, v in LEGS[i][0].items()}
    root = {"x": frames[i]["xp"], "y": frames[i]["yp"] - HIP}
    ph = 2 * math.pi * i / NF
    off["pelvis"] = {"angleDeg": round(-3 + 0.8 * math.sin(ph), 4)}
    off["torso"] = {"angleDeg": round(7 + 1.5 * math.sin(ph + 0.6), 4)}
    off["neck"] = {"angleDeg": -6.0}
    off["head"] = {"angleDeg": 3.0}
    hands_to(off, root, {"r": ("torso", 0.07, -0.085), "l": ("world", frames[i]["xp"] + 0.215 + 0.0, 1.02 + 0.0)}, {"r": -78, "l": -30})
    return pose_clean(off), root


WK = [walk_key(i) for i in range(NF)]
poses.append({"id": "waddle-contact", "title": T("Slow walking, left heel strike", "Langsames Gehen, linker Fersenkontakt"), "description": T("Frame 0 of the waddle cycle: shorter stride, torso leaning back, near hand on the back, far hand under the abdomen.", "Bild 0 des Watschelzyklus: kürzerer Schritt, Oberkörper zurückgelehnt, nahe Hand am Rücken, ferne Hand unter dem Bauch."),
              "bones": WK[0][0], "ground": "auto", "contacts": ["heel-l", "ball-r"], "thermal": tb(2.2, "walking-third-trimester", 0.8), "loads": []})
keys = [{"t": round(i * TT / NF, 5), "pose": "waddle-contact", "ease": "smooth", "bones": WK[i][0]} for i in range(NF)]
keys.append({"t": TT, "pose": "waddle-contact", "ease": "smooth", "bones": WK[0][0]})
contacts, ev = [], []
for sd, ph in (("l", 0.0), ("r", 0.5)):
    for w in ("heel", "ball"):
        for f, t in spans_of(spec, ph, TT, w):
            contacts.append({"anchor": f"{w}-{sd}", "fromS": f, "toS": t})
for c in contacts:
    ev.append((c["fromS"], "contact-on", f"{c['anchor']} on", f"{c['anchor']} auf", c["anchor"]))
    ev.append((c["toS"], "contact-off", f"{c['anchor']} off", f"{c['anchor']} ab", c["anchor"]))
ev = sorted([e for e in ev if 0 <= e[0] <= TT], key=lambda e: e[0])
c1 = {"id": "waddle-walk", "title": T("Slow walk in the third trimester", "Langsamer Gang im dritten Trimester"),
      "description": T("Level walking at 0.85 m/s, 92 steps per minute with a shorter stride; feet planted without sliding, torso leaning back to balance the abdomen, hands supporting back and abdomen.", "Gehen in der Ebene mit 0,85 m/s, 92 Schritten pro Minute und kürzerem Schritt; Füße gleiten nicht, Oberkörper lehnt zurück, um den Bauch auszugleichen, die Hände stützen Rücken und Bauch."),
      "durationS": TT, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": round(VV * TT, 6), "keys": keys, "contacts": contacts,
      "phases": [{"id": "double-support-left-lead", "fromS": 0, "toS": round(0.16 * TT, 4), "title": T("Double support, left foot leading", "Doppelstandphase, linker Fuß vorn")},
                 {"id": "left-single-support", "fromS": round(0.16 * TT, 4), "toS": round(0.5 * TT, 4), "title": T("Left single support", "Linke Einbeinstandphase")},
                 {"id": "double-support-right-lead", "fromS": round(0.5 * TT, 4), "toS": round(0.66 * TT, 4), "title": T("Double support, right foot leading", "Doppelstandphase, rechter Fuß vorn")},
                 {"id": "right-single-support", "fromS": round(0.66 * TT, 4), "toS": TT, "title": T("Right single support", "Rechte Einbeinstandphase")}],
      "events": clip_events(ev),
      "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.2}, {"t": TT, "v": 2.2}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": TT, "v": 1}]}]}
hb = POSES["hand-on-back"][0]
legs0 = {b: dict(v) for b, v in hb.items() if b.split("-")[0] in ("thigh", "shank", "foot", "toe")}
cb = POSES["cradling-belly"][0]
sb = POSES["stretching-back"][0]
c2 = {"id": "rest-and-breathe", "title": T("Resting and breathing", "Ausruhen und Atmen"), "description": T("A six-second loop of quiet standing: the hand moves from the abdomen to the lower back and a deep breath lifts the chest; the stance is held.", "Eine Schleife von sechs Sekunden ruhigen Stehens: Die Hand wechselt vom Bauch zum unteren Rücken und ein tiefer Atemzug hebt den Brustkorb; die Standposition bleibt."),
      "durationS": 6, "fps": 30, "loop": True, "rootMotion": "none", "keys": [
          {"t": 0, "pose": "hand-on-back", "ease": "smooth"},
          {"t": 2, "pose": "cradling-belly", "ease": "smooth", "bones": legs0},
          {"t": 4, "pose": "stretching-back", "ease": "smooth", "bones": legs0},
          {"t": 6, "pose": "hand-on-back", "ease": "smooth"}],
      "contacts": [{"anchor": c, "fromS": 0, "toS": 6} for c in CONTACTS],
      "phases": [{"id": "support-belly", "fromS": 0, "toS": 2, "title": T("Supporting the abdomen", "Den Bauch stützen")}, {"id": "stretch", "fromS": 2, "toS": 6, "title": T("Stretching the back", "Den Rücken dehnen")}],
      "events": [], "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3.5, "v": 2}, {"t": 5.5, "v": 0}, {"t": 6, "v": 0}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 4, "v": 1.4}, {"t": 6, "v": 1.3}]}]}
clips = [c1, c2]

P = lambda b, d, t, g: {"body": b, "detail": d, "texture": t, "glazing": None, "ground": g}
D = decisions
R = {}
R["500"] = {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000}, "pens": P(0.13, None, None, 0.18), "hatches": [], "geometry": {"elevation": rep_500()},
  "decisions": D(
    dict(silhouette="The profile of a woman of 1.65 m (3.3 mm) with the abdomen as its own rounded mass projecting 0.23 m (0.46 mm) in front of the torso, a leaned-back torso with the lumbar hollow, the near arm bent back to the hip as arm plus forearm-and-hand, the far arm forward, two leg columns with ankle boots.",
         retained="The abdomen bulge and the lumbar hollow behind it (they carry the whole identity), the bent elbow behind the back, both arms (the far hand reaches 0.38 m forward and sets the extent), the exact stature and the 0.73 m extent.",
         merged="Face and hair into one head mass; sweater and fabric over the belly stay two masses that overlap; hand and forearm into one tapered link; toe and boot into one foot mass.",
         omitted="Neck, ear, thumb, fingers, cuffs, knit texture, boot sole line.",
         thinElements="The 0.09 m hands are not drawn separately, the forearm link is widened by at most 8 mm to 0.10 m (within the 15 mm tolerance) so that it prints as a 0.2 mm feature.",
         hatching="None.", styles="Mid-grey sweater, lighter abdomen overlay, dark leggings and hair, mid boots; pen 0.13 mm only; light mode dark lines on white, dark mode inverted.",
         overlays="The torso mass paints the sweater, the abdomen mass its own fabric value; head lists face and hair, forearm links list sweater and hands (labelled merged)."),
    dict(silhouette="Das Profil einer Frau von 1,65 m (3,3 mm) mit dem Bauch als eigener runder Masse, die 0,23 m (0,46 mm) vor dem Rumpf vorsteht, zurückgelehntem Rumpf mit Lendenhohlkreuz, dem nahen Arm zur Hüfte abgewinkelt als Oberarm sowie Unterarm mit Hand, dem fernen Arm nach vorn, zwei Beinsäulen mit Stiefeletten.",
         retained="Die Bauchwölbung und das Hohlkreuz dahinter (sie tragen die ganze Identität), der abgewinkelte Ellbogen hinter dem Rücken, beide Arme (die ferne Hand reicht 0,38 m nach vorn und bestimmt die Ausdehnung), genaue Körpergröße und die Ausdehnung von 0,73 m.",
         merged="Gesicht und Haar zu einer Kopfmasse; Pullover und Stoff über dem Bauch bleiben zwei sich überlappende Massen; Hand und Unterarm zu einem verjüngten Glied; Zehe und Stiefelette zu einer Fußmasse.",
         omitted="Hals, Ohr, Daumen, Finger, Bündchen, Strickstruktur, Sohlenlinie.",
         thinElements="Die 0,09 m breiten Hände werden nicht einzeln gezeichnet, das Unterarmglied wird um höchstens 8 mm auf 0,10 m verbreitert (innerhalb der Toleranz von 15 mm), damit es als 0,2-mm-Einzelheit druckt.",
         hatching="Keine.", styles="Mittelgrauer Pullover, hellere Bauchauflage, dunkle Leggings und Haare, mittlere Stiefeletten; nur Strichstärke 0,13 mm; hell: dunkle Linien auf Weiß, dunkel: invertiert.",
         overlays="Die Rumpfmasse färbt den Pullover, die Bauchmasse ihren eigenen Stoffwert; der Kopf nennt Gesicht und Haar, die Unterarmglieder nennen Pullover und Hände (als verschmolzen gekennzeichnet)."))}
R["200"] = {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": P(0.18, 0.10, None, 0.2), "hatches": [], "geometry": {"elevation": rep_200()},
  "decisions": D(
    dict(silhouette="Main masses separated: face, bob hair, neck, sweater with the abdomen cover as a second round mass, both arms as sleeve, forearm and hand, legs as leggings plus ankle boot; 8.2 mm tall.",
         retained="The abdomen as an independent form that moves with the torso bone, the lumbar hollow in the back outline, elbow and knee joints as separate masses, boots with toe pieces.",
         merged="Thumb into the hand, sweater hem into the sweater, sock into the boot.",
         omitted="Fingers, ear, eye, cuffs, knit, collar, boot sole.", thinElements="Fingers dropped; the hands are mitten blobs of 0.09 m; no strokes needed.",
         hatching="None.", styles="Body pen 0.18 mm; fill tones carry the materials: mid sweater, lighter abdomen cover, dark leggings, hair and boots, light skin.",
         overlays="One region per mass; both sleeves share the sweater value, the abdomen cover shows the abdomen fabric value."),
    dict(silhouette="Hauptmassen getrennt: Gesicht, Bobfrisur, Hals, Pullover mit der Bauchabdeckung als zweiter runder Masse, beide Arme als Ärmel, Unterarm und Hand, Beine als Leggings mit Stiefelette; 8,2 mm hoch.",
         retained="Der Bauch als eigenständige Form, die sich mit dem Rumpfknochen bewegt, das Hohlkreuz in der Rückenkontur, Ellbogen- und Kniegelenke als getrennte Massen, Stiefeletten mit Zehenteilen.",
         merged="Daumen in die Hand, Pulloversaum in den Pullover, Socke in die Stiefelette.",
         omitted="Finger, Ohr, Auge, Bündchen, Strick, Kragen, Stiefelsohle.", thinElements="Finger entfallen; die Hände sind Fäustlingsflecken von 0,09 m; keine Striche nötig.",
         hatching="Keine.", styles="Körperstift 0,18 mm; die Füllungstöne tragen die Materialien: mittlerer Pullover, hellere Bauchabdeckung, dunkle Leggings, Haare und Stiefeletten, helle Haut.",
         overlays="Eine Region je Masse; beide Ärmel teilen den Pulloverwert, die Bauchabdeckung zeigt den Bauchstoffwert."))}
R["100"] = {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": P(0.2, 0.1, None, 0.25),
  "hatches": [{"id": "knit", "title": T("Knit rows", "Strickreihen"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.8, "penMm": 0.1}], "geometry": {"elevation": rep_100()},
  "decisions": D(
    dict(silhouette="Articulated clothing: sleeves with cuff lines, a sweater with hem and collar over a stretched abdomen cover with a linea line, leggings, ankle boots with sole line, bob hair with ear, mitten hands with thumb; 16.5 mm tall.",
         retained="The abdomen cover with its single fold line, the sweater hem that stops above the abdomen's lowest point, face profile with nose and chin.",
         merged="Eye and mouth are not drawn; knit texture is the hatch only; fingers form mittens.", omitted="Eye (0.16 mm), cable pattern, boot zip, seams.",
         thinElements="Cuff lines and the fold line are 0.1 mm detail strokes; no filled feature below 0.2 mm.",
         hatching="Sweater, sleeves and abdomen cover: vertical knit rows at 0.8 mm spacing, 0.10 mm pen.", styles="Body 0.2 mm, detail 0.1 mm; sweater mid-grey with knit rows, leggings and boots dark, hair darkest, skin light.",
         overlays="Regions as at 1:200; the fold line carries the abdomen value."),
    dict(silhouette="Gegliederte Kleidung: Ärmel mit Bündchenlinien, Pullover mit Saum und Kragen über einer gespannten Bauchabdeckung mit Linea-Linie, Leggings, Stiefeletten mit Sohlenlinie, Bobfrisur mit Ohr, Fäustlingshände mit Daumen; 16,5 mm hoch.",
         retained="Die Bauchabdeckung mit ihrer einzelnen Faltenlinie, der Pulloversaum, der über dem tiefsten Punkt des Bauchs endet, Gesichtsprofil mit Nase und Kinn.",
         merged="Auge und Mund werden nicht gezeichnet; die Strickstruktur ist nur die Schraffur; Finger bilden Fäustlinge.", omitted="Auge (0,16 mm), Zopfmuster, Stiefeletten-Reißverschluss, Nähte.",
         thinElements="Bündchenlinien und Faltenlinie sind 0,1-mm-Detailstriche; keine gefüllte Einzelheit unter 0,2 mm.",
         hatching="Pullover, Ärmel und Bauchabdeckung: senkrechte Strickreihen mit 0,8 mm Abstand, 0,10 mm Stift.", styles="Körper 0,2 mm, Detail 0,1 mm; Pullover mittelgrau mit Strickreihen, Leggings und Stiefeletten dunkel, Haare am dunkelsten, Haut hell.",
         overlays="Regionen wie bei 1:200; die Faltenlinie trägt den Bauchwert."))}
R["50"] = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": P(0.25, 0.13, 0.1, 0.3),
  "hatches": [{"id": "knit", "title": T("Knit rows", "Strickreihen"), "pattern": "lines", "angleDeg": 90, "spacingMm": 0.6, "penMm": 0.1}], "geometry": {"elevation": rep_50()},
  "decisions": D(
    dict(silhouette="Fully articulated: rib hem and cuffs, collar, stretch lines across the abdomen, finger separations, leggings seam, boots with real soles and zips, eye, mouth, brow-free profile and hair strands; 33 mm tall.",
         retained="Everything of 1:100 plus the abdomen stretch lines and the seam of the leggings.", merged="Zip teeth are one stroke; hair is three strand strokes.", omitted="Pores, stitching, knit loops, eyelashes.",
         thinElements="Fingers, seams, stretch lines and zips are 0.10 mm texture lines; the rib hem is a real 0.04 m = 0.8 mm band.",
         hatching="Knit rows at 0.6 mm on sweater, sleeves and abdomen cover.", styles="Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; tones as at 1:100.",
         overlays="One region per shape; stretch lines carry the abdomen value, the seam the legging value."),
    dict(silhouette="Voll gegliedert: Rippensaum und -bündchen, Kragen, Dehnungslinien über dem Bauch, Fingertrennungen, Leggings-Naht, Stiefeletten mit echten Sohlen und Reißverschlüssen, Auge, Mund, Profil ohne Braue und Haarsträhnen; 33 mm hoch.",
         retained="Alles aus 1:100 und die Dehnungslinien am Bauch sowie die Naht der Leggings.", merged="Reißverschlusszähne sind ein Strich; das Haar besteht aus drei Strähnenstrichen.", omitted="Poren, Stiche, Strickmaschen, Wimpern.",
         thinElements="Finger, Nähte, Dehnungslinien und Reißverschlüsse sind 0,10-mm-Texturlinien; der Rippensaum ist ein echtes Band von 0,04 m = 0,8 mm.",
         hatching="Strickreihen mit 0,6 mm auf Pullover, Ärmeln und Bauchabdeckung.", styles="Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Töne wie bei 1:100.",
         overlays="Eine Region je Form; Dehnungslinien tragen den Bauchwert, die Naht den Leggingswert."))}
asset = {
    "id": aid, "category": "humans",
    "title": T("Pregnant Person", "Schwangere Person"),
    "summary": T("A woman of 1.66 m in week 36 of pregnancy (71 kg, 13 kg gained) in an oversized knit sweater, maternity leggings and ankle boots, standing with the torso leaned back and the lumbar curve increased. The abdomen is its own bone and thermal region; physiology (metabolism, ventilation, skin blood flow) is adjusted for the third trimester.",
                 "Eine Frau von 1,66 m in der 36. Schwangerschaftswoche (71 kg, 13 kg zugenommen) in übergroßem Strickpullover, Umstandsleggings und Stiefeletten, die mit zurückgelehntem Oberkörper und verstärkter Lendenkrümmung steht. Der Bauch ist ein eigener Knochen und eine eigene thermische Region; Physiologie (Stoffwechsel, Atmung, Hautdurchblutung) ist für das dritte Trimester angepasst."),
    "tags": ["human", "adult", "woman", "pregnant", "third-trimester", "standing", "waddling", "physiology-adjusted"],
    "provenance": {"author": "Semio staffage content worker NW-H3", "method": "handcrafted", "created": "2026-10-29",
                   "references": refs(["de-leva-1996", "winter-2009", "drillis-contini-1966", "perry-burnfield-2010", "dubois-1916", "iso-9920-2007", "iso-7730-2005", "ashrae-2021", "gupta-2010", "gagge-1971", "mifflin-1990", "icrp-66-1994"], EXTRA) + [{"id": k, "citation": v} for k, v in EXTRA.items()],
                   "notes": T("Rig scaled to 1.66 m with de Leva female fractions for the non-gravid body mass plus a separate abdomen bone (6 kg fetal unit, guided by Hytten and Chamberlain). Lordotic posture and the shortened waddling stride are authored; metabolism, ventilation and skin blood flow are third-trimester adjustments guided by Butte 2004, Lof 2005 and LoMauro and Aliverti 2015 and tagged as authored estimates.",
                              "Skelett auf 1,66 m skaliert mit de-Leva-Frauenanteilen für die Körpermasse ohne Schwangerschaft plus einem eigenen Bauchknochen (6 kg Frucht, nach Hytten und Chamberlain). Lordotische Haltung und der verkürzte Watschelschritt sind geschätzt; Stoffwechsel, Atmung und Hautdurchblutung sind Anpassungen für das dritte Trimester nach Butte 2004, Lof 2005 sowie LoMauro und Aliverti 2015 und als Schätzwerte gekennzeichnet.")},
    "dimensions": {"heightM": 1.65, "widthM": 0.72, "depthM": 0.4, "massKg": MASS},
    "materials": mats, "rig": rig, "anchors": anchors, "representations": OrderedDict((k, R[k]) for k in ("500", "200", "100", "50")), "symbols": [],
    "poses": poses, "clips": clips, "thermal": thermal,
    "structure": {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": [], "assumptions": [
        T("Segments are point masses at the de Leva centres of the non-gravid body; the 6 kg fetal unit is a point mass on the abdomen bone 0.11 m in front of the torso axis.", "Segmente sind Punktmassen in den de-Leva-Schwerpunkten des nicht schwangeren Körpers; die 6 kg schwere Frucht ist eine Punktmasse auf dem Bauchknochen, 0,11 m vor der Rumpfachse."),
        T("Contacts are heel and ball of each foot; the shifted centre of mass is balanced by the backward lean of the torso.", "Kontakte sind Ferse und Ballen jedes Fußes; der verlagerte Schwerpunkt wird durch die Rücklage des Oberkörpers ausgeglichen.")], "clinical": False},
    "presentation": {"defaultPose": "hand-on-back", "defaultClip": "waddle-walk", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}
errs = check(asset)
if "--write" in sys.argv:
    write_asset(aid, "humans", asset)
    print("written")
