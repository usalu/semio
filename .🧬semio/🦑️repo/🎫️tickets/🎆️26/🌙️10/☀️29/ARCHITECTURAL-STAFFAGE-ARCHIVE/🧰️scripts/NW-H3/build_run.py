from a_run_geom import *
from h3asm import *
from h3check import check

aid = "runner-with-blade-prosthesis"
SKR = round(dubois(68.5, 1.80) * 0.95, 3)
EXTRA = {"ossur-cheetah-xtreme": "Manufacturer class data for J-shaped carbon-fibre running blades (e.g. Ossur Cheetah family): blade mass about 0.5-1.4 kg, attached below a carbon socket through a pyramid adapter, rubber toe pad; values authored from typical product ranges, not from one data sheet.",
         "acsm-guidelines": "ACSM's Guidelines for Exercise Testing and Prescription, 11th ed. (2021): metabolic equation for level running VO2 (ml/kg/min) = 0.2 x speed (m/min) + 3.5, 1 MET = 3.5 ml/kg/min.",
         "weyand-2009": "Weyand, P. G., Bundle, M. W., McGowan, C. P., Grabowski, A., Brown, M. B., Kram, R. and Herr, H. (2009). The fastest runner on artificial legs: different limbs, similar function? Journal of Applied Physiology 107(3), 903-911 (guidance for the qualitative asymmetry of contact and swing times only)."}
mats = [m for m in BASE["materials"] if m["id"] in ("skin", "hair", "cotton-sock")] + [
    material("running-singlet", "Synthetic running singlet", "Synthetisches Lauftop", 0.93, 0.5, cp=1300),
    material("compression-shorts", "Elastane compression shorts", "Kompressionsshorts aus Elastan", 0.93, 0.85, cp=1300),
    material("running-shoe", "Running shoe, mesh upper and foam sole", "Laufschuh mit Mesh-Obermaterial und Schaumsohle", 0.9, 0.6, cp=1700),
    material("silicone-liner", "Silicone gel liner", "Silikon-Gelliner", 0.95, 0.7, cp=1500, k=0.2),
    material("carbon-socket", "Carbon-fibre socket and pylon", "Karbonschaft mit Pylon", 0.88, 0.85, cp=800, rho=1600, k=5),
    material("carbon-blade", "Carbon-fibre spring blade with rubber toe", "Karbon-Sprungfeder mit Gummispitze", 0.88, 0.85, cp=800, rho=1600, k=5),
]
cap_skin = 13000
f_s = fcl_of(0.05)
sg = [garment("singlet", "Synthetic running singlet", "Synthetisches Lauftop", 0.05, "running-singlet", 0.0006, 80, f_s)]
f_h = fcl_of(0.04)
sh_l = [garment("shorts", "Compression shorts", "Kompressionsshorts", 0.04, "compression-shorts", 0.001, 120, f_h)]
shoe_l = [garment("socks", "Thin running socks", "Dünne Laufsocken", 0.02, "cotton-sock", 0.001, 60, 1.0), garment("shoe", "Running shoe", "Laufschuh", 0.03, "running-shoe", 0.006, 500, 1.0)]
soc_l = [{"id": "liner", "title": T("Silicone gel liner on the residual limb", "Silikon-Gelliner auf dem Stumpf"), "kind": "polymer", "material": "silicone-liner", "thicknessM": 0.006, "resistanceM2KW": 0.03, "vapourResistanceM2PaW": 5000, "heatCapacityJM2K": 700, "source": "authored-estimate"},
         {"id": "shell", "title": T("Carbon socket shell", "Karbon-Schaftschale"), "kind": "shell", "material": "carbon-socket", "thicknessM": 0.004, "resistanceM2KW": 0.0008, "vapourResistanceM2PaW": 100000, "heatCapacityJM2K": 3200, "source": "authored-estimate"}]
hair_l = [{"id": "hair", "title": T("Cropped hair", "Kurzgeschnittenes Haar"), "kind": "hair", "material": "hair", "thicknessM": 0.008, "resistanceM2KW": 0.025, "vapourResistanceM2PaW": 4, "heatCapacityJM2K": 120, "source": "authored-estimate"}]
cap = lambda ls, base=cap_skin: base + sum(l["heatCapacityJM2K"] for l in ls)
regions = [
    body_region(FACE, "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", SKR * 0.07, 1.0, "head", 0.06, 0.07, 0.2, "sphere", 0.5, cap_skin, [], ref="dubois-1916"),
    body_region(HAIR, "Cropped hair", "Kurzes Haar", "hair", "hair", SKR * 0.03, 1.0, "head", 0.03, 0.03, 0.2, "sphere", 0.8, cap(hair_l, 8000), hair_l),
    body_region(ARM, "Bare arms", "Unbedeckte Arme", "skin", "skin", SKR * 0.13, 1.0, "arm", 0.08, 0.13, 0.07, "cylinder", 0.3, cap_skin, [], ref="dubois-1916"),
    body_region(HAND, "Bare hands", "Unbedeckte Hände", "skin", "skin", SKR * 0.05, 1.0, "hand", 0.03, 0.05, 0.1, "cylinder", 0.2, cap_skin, [], ref="dubois-1916"),
    body_region(SING, "Running singlet on the trunk", "Lauftop am Rumpf", "clothing", "running-singlet", SKR * 0.30, f_s, "trunk", 0.34, 0.30, 0.3, "vertical-plate", 0.45, cap(sg), sg),
    body_region(SHORT, "Compression shorts on hips and upper thighs", "Kompressionsshorts an Hüfte und Oberschenkeln", "clothing", "compression-shorts", SKR * 0.14, f_h, "leg", 0.13, 0.14, 0.12, "cylinder", 0.2, cap(sh_l), sh_l),
    body_region(LEGSK, "Bare skin of the thigh and the intact lower leg", "Freie Haut von Oberschenkel und unversehrtem Unterschenkel", "skin", "skin", SKR * 0.20, 1.0, "leg", 0.22, 0.20, 0.09, "cylinder", 0.1, cap_skin, [], ref="dubois-1916"),
    body_region(SHOE, "Sock and running shoe of the intact foot", "Socke und Laufschuh des unversehrten Fußes", "surface", "running-shoe", SKR * 0.05, 1.0, "foot", 0.03, 0.05, 0.1, "horizontal-plate-up", 0.1, cap(shoe_l), shoe_l),
    body_region(SOCK, "Carbon socket over the residual limb (liner, shell and pylon)", "Karbonschaft über dem Stumpf (Liner, Schale und Pylon)", "surface", "carbon-socket", SKR * 0.03 * 1.8, 1.0, "leg", 0.08, 0.03, 0.1, "cylinder", 0.1, cap(soc_l, 8000), soc_l),
    {"id": BLADE, "title": T("Carbon-fibre running blade with rubber toe", "Karbon-Laufblatt mit Gummispitze"), "kind": "surface", "model": "passive", "material": "carbon-blade", "areaM2": 0.09, "characteristicLengthM": 0.07,
     "geometry": "vertical-plate", "skyViewFactor": 0.35, "layers": [], "heatCapacityJM2K": 8000, "source": "authored-estimate", "reference": "ossur-cheetah-xtreme"},
]
resp = {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 35, "exhaledRelativeHumidity": 0.97,
        "patterns": [
            {"id": "rest-standing", "title": T("Standing at rest", "Ruhiges Stehen"), "rateBpm": 12, "tidalVolumeL": 0.55, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
            {"id": "running", "title": T("Running at 3.1 m/s, about 11.5 met", "Laufen mit 3,1 m/s, etwa 11,5 met"), "rateBpm": 36, "tidalVolumeL": 1.95, "inhaleFraction": 0.47, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.49, "pauseAfterExhaleFraction": 0.02, "route": "mouth", "source": "authored-estimate", "reference": "acsm-guidelines"},
            {"id": "recovery", "title": T("Recovery after the run, hands on hips", "Erholung nach dem Lauf, Hände in den Hüften"), "rateBpm": 28, "tidalVolumeL": 1.6, "inhaleFraction": 0.45, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.02, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"}],
        "motion": [{"bone": "torso", "angleDeg": 1.4, "dy": 0.007}], "source": "authored-estimate", "reference": "gupta-2010"}
thermal = OrderedDict()
thermal["environment"] = {"airTempC": 18, "meanRadiantTempC": 24, "skyTempC": 8, "groundTempC": 24, "relativeHumidity": 0.55, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 600, "diffuseFraction": 0.25, "pressurePa": 101325}
thermal["baseMet"] = 1.6
thermal["body"] = {"species": "human", "massKg": 68.5, "skinAreaM2": SKR, "basalMetabolismW": round(mifflin("m", 68.5, 180, 29) * 0.98, 1), "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 1.0, "source": "authored-estimate", "reference": "mifflin-1990"}
thermal["respiration"] = resp
thermal["operating"] = []
thermal["passengers"] = []
thermal["regions"] = regions
thermal["emitters"] = breath_emitters(plume_region=SING, plume_d=0.35)
thermal["assumptions"] = [
    T(f"Right transtibial amputation: the skin area {SKR} m2 is the DuBois and DuBois (1916) value for 68.5 kg and 1.80 m times 0.95 (lower leg and foot are about 5 % of the body surface, rule-of-nines proportions); the residual limb is enclosed by liner and carbon socket, which hold heat and block sweat evaporation (vapour resistance of the liner 5000 Pa m2/W, authored).",
      f"Rechtsseitige Unterschenkelamputation: Die Hautfläche {SKR} m2 ist der DuBois-Wert (1916) für 68,5 kg und 1,80 m mal 0,95 (Unterschenkel und Fuß sind etwa 5 % der Körperoberfläche, Neunerregel); der Stumpf ist von Liner und Karbonschaft umschlossen, die Wärme halten und die Schweißverdunstung sperren (Dampfwiderstand des Liners 5000 Pa m2/W, geschätzt)."),
    T("Level running at 3.1 m/s (184 m/min) gives 0.2 x 184 + 3.5 = 40 ml/(kg min) = 11.5 met after the ACSM running equation; ventilation of about 70 L/min (36 per minute, 1.95 L) is an authored estimate for that load. Runners with blades are reported to run at a similar cost (Weyand et al. 2009); no extra cost is applied.",
      "Ebenes Laufen mit 3,1 m/s (204 m/min) ergibt 0,2 x 204 + 3,5 = 44 ml/(kg min) = 11,5 met nach der ACSM-Laufgleichung; eine Lüftung von etwa 70 L/min (36 je Minute, 1,95 L) ist für diese Last geschätzt. Läufer mit Prothesenfeder laufen berichtet bei ähnlichem Aufwand (Weyand et al. 2009); ein Mehraufwand wird nicht angesetzt."),
    T("Sunny morning: 600 W/m2 on a normal surface with 25 % diffuse, 18 degrees air; the carbon blade and socket absorb about 85 % of the sun and conduct only through the toe contact (about 5 % of the blade area at stance).",
      "Sonniger Morgen: 600 W/m2 auf eine normale Fläche mit 25 % diffus, 18 Grad Luft; Karbonfeder und Schaft nehmen etwa 85 % der Sonne auf und leiten nur über den Zehenkontakt (etwa 5 % der Federfläche im Stütz)."),
    T("Basal metabolism after Mifflin-St Jeor for 29 years, 68.5 kg, 180 cm, times 0.98 for the missing lower leg mass.", "Grundumsatz nach Mifflin-St Jeor für 29 Jahre, 68,5 kg, 180 cm, mal 0,98 für die fehlende Unterschenkelmasse."),
]
SUN = {FACE: 0.6, HAIR: 0.6, ARM: 0.6, HAND: 0.5, SING: 0.6, SHORT: 0.5, LEGSK: 0.5, SHOE: 0.3, SOCK: 0.5, BLADE: 0.5}
EXP = {SING: 0.97, SHORT: 0.95}


def tb(met, br, plume, contact=None, work=0.0):
    t = binding(regions, {"body-plume": plume}, met, br, exposed=EXP, sun=SUN, contact=contact or {})
    return t


CONTACT_STAND = {SHOE: {"fraction": 0.3, "tempC": 24}, BLADE: {"fraction": 0.05, "tempC": 24}}
TBR = lambda c: tb(11.5, "running", 1.0, c)
TI = {"standing-ready": (T("Standing ready", "Bereitstehen"), T("Upright with relaxed arms before the start, weight shared by the intact foot and the blade; the blade tip stands level with the toes.", "Aufrecht mit locker hängenden Armen vor dem Start, das Gewicht verteilt sich auf den gesunden Fuß und die Feder; die Federspitze steht auf Höhe der Zehen.")),
      "hands-on-hips": (T("Recovering, hands on hips", "Erholen, Hände in den Hüften"), T("After the run: head back, elbows out, deep breaths.", "Nach dem Lauf: Kopf im Nacken, Ellbogen nach außen, tiefe Atemzüge."))}
poses = []
poses.append({"id": "standing-ready", "title": TI["standing-ready"][0], "description": TI["standing-ready"][1], "bones": POSES["standing-ready"][0], "ground": "auto", "contacts": FEET, "thermal": tb(1.5, "rest-standing", 0.6, CONTACT_STAND), "loads": []})
poses.append({"id": "hands-on-hips", "title": TI["hands-on-hips"][0], "description": TI["hands-on-hips"][1], "bones": POSES["hands-on-hips"][0], "ground": "auto", "contacts": FEET, "thermal": tb(4.0, "recovery", 1.0, CONTACT_STAND), "loads": []})
IDX = {"left-forefoot-strike": 0, "blade-midstance": 14, "flight": 8}
TT_ = {"left-forefoot-strike": (T("Left forefoot strike", "Aufsetzen des linken Vorfußes"), T("The intact foot lands on the ball while the blade leg swings through with a flexed knee.", "Der gesunde Fuß landet auf dem Ballen, während das Federbein mit gebeugtem Knie durchschwingt.")),
       "blade-midstance": (T("Blade at mid-stance", "Feder im mittleren Stütz"), T("The rubber toe of the blade is planted and the blade is compressed by about 11 degrees; the intact leg drives forward.", "Die Gummispitze der Feder steht und die Feder ist um etwa 11 Grad eingefedert; das gesunde Bein schwingt nach vorn.")),
       "flight": (T("Flight phase", "Flugphase"), T("Both feet are off the ground; the pelvis is at its highest.", "Beide Füße sind in der Luft; das Becken ist am höchsten."))}
for k, i in IDX.items():
    off, root, xp, ypel, u = FR[i]
    ct = contacts_at(i)
    pd = {"id": k, "title": TT_[k][0], "description": TT_[k][1], "bones": off, "ground": "auto" if ct else "authored", "contacts": ct, "thermal": TBR(CONTACT_STAND if ct else {}), "loads": []}
    if not ct:
        pd["root"] = {"y": round(ypel - HIP, 5)}
    poses.append(pd)
for i in range(N):
    off, root, xp, ypel, u = FR[i]
    ct = contacts_at(i)
    poses.append({"id": f"run-{i:02d}", "title": T(f"Running cycle frame {i}", f"Laufzyklus, Bild {i}"), "description": T("Key pose of the running cycle (planted contacts, authored pelvis height).", "Schlüsselpose des Laufzyklus (feste Kontakte, vorgegebene Beckenhöhe)."), "key": True,
                  "bones": off, "root": {"y": round(ypel - HIP, 5)}, "ground": "authored", "contacts": ct, "thermal": TBR({}), "loads": []})
keys = [{"t": round(i * T_ / N, 5), "pose": f"run-{i:02d}", "ease": "smooth"} for i in range(N)]
keys.append({"t": T_, "pose": "run-00", "ease": "smooth"})
cl = SL["b1"] * T_
cr0, cr1 = 0.5 * T_, (0.5 + STIP["b1"]) * T_
contacts = [{"anchor": "ball-l", "fromS": 0.0, "toS": round(cl, 4)}, {"anchor": "blade-toe", "fromS": round(cr0, 4), "toS": round(cr1, 4)}]
ev = [(0.0, "contact-on", "Left forefoot strike", "Linker Vorfußkontakt", "ball-l"), (cl, "contact-off", "Left toe-off", "Linker Abdruck", "ball-l"), (cr0, "contact-on", "Blade tip touches down", "Federspitze setzt auf", "blade-toe"), (cr1, "contact-off", "Blade push-off", "Federabstoß", "blade-toe")]
c1 = {"id": "run-cycle", "title": T("Running with the blade at 3.1 m/s", "Laufen mit der Feder bei 3,1 m/s"),
      "description": T("One stride in 0.7 s (171 steps per minute, 2.15 m): the blade side has a shorter ground contact (0.21 s) than the intact side (0.25 s) and a longer flight before it, the blade flexes about 11 degrees under load, arms swing at the elbow angle of 90 degrees.", "Ein Doppelschritt in 0,7 s (171 Schritte pro Minute, 2,15 m): Die Federseite hat einen kürzeren Bodenkontakt (0,21 s) als die gesunde Seite (0,25 s) und davor eine längere Flugzeit, die Feder federt unter Last um etwa 11 Grad ein, die Arme schwingen mit 90 Grad Ellbogenwinkel."),
      "durationS": T_, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": 2.1465, "keys": keys, "contacts": contacts,
      "phases": [{"id": "left-stance", "fromS": 0.0, "toS": round(cl, 4), "title": T("Stance on the intact foot", "Standphase auf dem gesunden Fuß")},
                 {"id": "flight-one", "fromS": round(cl, 4), "toS": round(cr0, 4), "title": T("Flight after the intact foot", "Flugphase nach dem gesunden Fuß"), "ballistic": True},
                 {"id": "blade-stance", "fromS": round(cr0, 4), "toS": round(cr1, 4), "title": T("Stance on the blade", "Standphase auf der Feder")},
                 {"id": "flight-two", "fromS": round(cr1, 4), "toS": T_, "title": T("Flight after the blade", "Flugphase nach der Feder"), "ballistic": True}],
      "events": clip_events(sorted(ev, key=lambda e: e[0])),
      "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 11.5}, {"t": T_, "v": 11.5}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": T_, "v": 1}]}]}
hh = POSES["hands-on-hips"][0]
c2 = {"id": "recover-breathing", "title": T("Recovering after the run", "Erholen nach dem Lauf"), "description": T("Five-second loop: hands on the hips, the head nods back with each deep breath and the chest lifts; the stance is held.", "Eine Schleife von fünf Sekunden: Hände in den Hüften, der Kopf nickt mit jedem tiefen Atemzug nach hinten und der Brustkorb hebt sich; die Standposition bleibt."),
      "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none", "keys": [
          {"t": 0, "pose": "hands-on-hips", "ease": "smooth"},
          {"t": 1.25, "pose": "hands-on-hips", "ease": "smooth", "bones": {"head": {"angleDeg": 18}, "neck": {"angleDeg": 6}, "torso": {"angleDeg": 4}}},
          {"t": 2.5, "pose": "hands-on-hips", "ease": "smooth", "bones": {"head": {"angleDeg": 12}, "neck": {"angleDeg": 3}, "torso": {"angleDeg": 1}}},
          {"t": 3.75, "pose": "hands-on-hips", "ease": "smooth", "bones": {"head": {"angleDeg": 17}, "neck": {"angleDeg": 6}, "torso": {"angleDeg": 3.5}}},
          {"t": 5, "pose": "hands-on-hips", "ease": "smooth"}],
      "contacts": [{"anchor": c, "fromS": 0, "toS": 5} for c in FEET],
      "phases": [{"id": "recovery", "fromS": 0, "toS": 5, "title": T("Recovery breathing", "Erholungsatmung")}], "events": [],
      "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 2}, {"t": 5, "v": 2}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 4.0}, {"t": 5, "v": 3.4}]}]}
clips = [c1, c2]

P = lambda b, d, t, g: {"body": b, "detail": d, "texture": t, "glazing": None, "ground": g}
D = decisions
R = {}
r50 = rep_50()
for x in r50:
    if x["id"] in ("socket", "blade-upper", "blade-curl"):
        x["hatch"] = "carbon"
R["500"] = {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000}, "pens": P(0.13, None, None, 0.18), "hatches": [], "geometry": {"elevation": rep_500()},
  "decisions": D(
    dict(silhouette="An upright athletic figure of 1.79 m (3.6 mm): head mass, one torso mass with the shorts, two arms as shoulder link and forearm-with-hand link, the intact leg as thigh, shank and shoe, and on the other side thigh, a dark socket and the blade as a curved J of two 0.13 mm strokes ending in a black toe pad.",
         retained="The curved J of the blade (it is the identity of the asset and of the running gait), the socket as a darker block below the knee, the bare calf of the other leg, the elbows, exact stature.",
         merged="Face and hair; singlet and shorts in one torso mass; forearm and hand; toe and shoe; pylon into the socket.",
         omitted="Neck, ear, fingers, thumb, bib, laces, sole line, adapter, laminate.",
         thinElements="The 24 mm thick blade cannot be a filled shape at 0.05 mm; it is drawn as two 0.13 mm strokes (65 mm real, a thickening far beyond the 15 mm tolerance, but the stroke follows the true centre line and chord); the rubber toe is a real 90 mm black pad.",
         hatching="None.", styles="Light skin tone for arms and leg, mid singlet, dark socket and shorts, near-black toe pad; pen 0.13 mm only; light mode dark lines on white, dark mode inverted.",
         overlays="The torso mass lists singlet and shorts; the forearm links list arm and hand skin; socket and blade strokes keep their own region."),
    dict(silhouette="Eine aufrechte athletische Figur von 1,79 m (3,6 mm): Kopfmasse, eine Rumpfmasse mit den Shorts, zwei Arme als Schulterglied und Glied aus Unterarm mit Hand, das gesunde Bein aus Ober- und Unterschenkel und Schuh, auf der anderen Seite Oberschenkel, ein dunkler Schaft und die Feder als gebogenes J aus zwei 0,13-mm-Strichen mit schwarzer Zehenkappe.",
         retained="Das gebogene J der Feder (es ist die Identität des Assets und des Laufschritts), der Schaft als dunkler Block unter dem Knie, die nackte Wade des anderen Beins, die Ellbogen, genaue Körpergröße.",
         merged="Gesicht und Haar; Top und Shorts in einer Rumpfmasse; Unterarm und Hand; Zehe und Schuh; Pylon im Schaft.",
         omitted="Hals, Ohr, Finger, Daumen, Startnummer, Schnürung, Sohlenlinie, Adapter, Laminat.",
         thinElements="Die 24 mm dicke Feder kann bei 0,05 mm keine gefüllte Form sein; sie wird als zwei 0,13-mm-Striche gezeichnet (65 mm real, weit über der Toleranz von 15 mm, doch der Strich folgt der wahren Mittellinie und Sehne); die Gummispitze ist eine echte 90-mm-Kappe in Schwarz.",
         hatching="Keine.", styles="Helle Hauttöne für Arme und Bein, mittleres Top, dunkler Schaft und dunkle Shorts, fast schwarze Zehenkappe; nur Strichstärke 0,13 mm; hell: dunkle Linien auf Weiß, dunkel: invertiert.",
         overlays="Die Rumpfmasse nennt Top und Shorts; die Unterarmglieder nennen Arm- und Handhaut; Schaft und Federstriche behalten ihre eigene Region."))}
R["200"] = {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": P(0.18, 0.10, None, 0.2), "hatches": [], "geometry": {"elevation": rep_200()},
  "decisions": D(
    dict(silhouette="Main masses separated: face, cropped hair, neck, singlet, pelvis shorts, bare arms as upper arm, forearm and hand, intact leg as thigh with shorts, shank and shoe, the other leg as thigh, carbon socket and the blade as two strokes plus the black toe pad; 8.9 mm tall.",
         retained="The socket with its tapering pylon, the blade J, the contrast between the shod foot and the rubber toe, knees and elbows as separate masses.",
         merged="Thumb and fingers into the hand blob, pylon and adapter into the socket.", omitted="Ear, eye, fingers, bib, laces, seams.",
         thinElements="Blade strokes 0.18 mm (36 mm real, vs 24 mm blade thickness: thickened, stroke follows the centre line).",
         hatching="None.", styles="Body pen 0.18 mm, detail pen 0.10 mm; light arms and legs, mid singlet, dark shorts and socket, black toe pad.",
         overlays="One region per mass; shorts shapes on both thighs share the shorts value; socket and blade strokes carry their own regions."),
    dict(silhouette="Hauptmassen getrennt: Gesicht, kurzes Haar, Hals, Top, Becken-Shorts, nackte Arme als Oberarm, Unterarm und Hand, das gesunde Bein als Oberschenkel mit Shorts, Unterschenkel und Schuh, das andere Bein als Oberschenkel, Karbonschaft und die Feder als zwei Striche plus schwarze Zehenkappe; 8,9 mm hoch.",
         retained="Der Schaft mit dem sich verjüngenden Pylon, das J der Feder, der Kontrast zwischen beschuhtem Fuß und Gummispitze, Knie und Ellbogen als getrennte Massen.",
         merged="Daumen und Finger im Handfleck, Pylon und Adapter im Schaft.", omitted="Ohr, Auge, Finger, Startnummer, Schnürung, Nähte.",
         thinElements="Federstriche 0,18 mm (36 mm real gegenüber 24 mm Federdicke: verdickt, der Strich folgt der Mittellinie).",
         hatching="Keine.", styles="Körperstift 0,18 mm, Detailstift 0,10 mm; helle Arme und Beine, mittleres Top, dunkle Shorts und dunkler Schaft, schwarze Zehenkappe.",
         overlays="Eine Region je Masse; die Shorts-Formen an beiden Oberschenkeln teilen den Shortswert; Schaft und Federstriche tragen eigene Regionen."))}
R["100"] = {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": P(0.2, 0.1, None, 0.25), "hatches": [], "geometry": {"elevation": rep_100()},
  "decisions": D(
    dict(silhouette="Articulated: cropped hair with ear, face profile, singlet with neck line, compression shorts, mitten hands with thumb, running shoe with sole line and toe piece; the socket with a liner edge and the blade as a real 0.24 mm laminate in two curved pieces with the rubber toe; 17.9 mm tall.",
         retained="The blade laminate as a filled curved shape, the liner edge at the socket's top, the sole line of the shoe.",
         merged="Eye and mouth not drawn; fingers into mittens; pylon into the socket.", omitted="Eye (0.16 mm), bib number, laces, adapter bolts, carbon weave.",
         thinElements="The blade is drawn at its real 24 mm = 0.24 mm; the toe pad 0.3 mm.", hatching="None - the fill tones carry the materials; the carbon weave appears at 1:50.",
         styles="Body 0.2 mm, detail 0.1 mm; skin light, singlet mid, shorts and socket dark, blade darker, toe pad black.",
         overlays="Regions as at 1:200; the liner edge carries the socket value."),
    dict(silhouette="Gegliedert: kurzes Haar mit Ohr, Gesichtsprofil, Top mit Halsausschnitt, Kompressionsshorts, Fäustlingshände mit Daumen, Laufschuh mit Sohlenlinie und Zehenteil; der Schaft mit Linerkante und die Feder als echtes 0,24-mm-Laminat in zwei gebogenen Teilen mit Gummispitze; 17,9 mm hoch.",
         retained="Das Federlaminat als gefüllte gebogene Form, die Linerkante am oberen Schaftrand, die Sohlenlinie des Schuhs.",
         merged="Auge und Mund werden nicht gezeichnet; Finger zu Fäustlingen; Pylon im Schaft.", omitted="Auge (0,16 mm), Startnummer, Schnürung, Adapterschrauben, Karbongewebe.",
         thinElements="Die Feder ist in wahrer Dicke von 24 mm = 0,24 mm gezeichnet; die Zehenkappe 0,3 mm.", hatching="Keine - die Füllungstöne tragen die Materialien; das Karbongewebe erscheint bei 1:50.",
         styles="Körper 0,2 mm, Detail 0,1 mm; Haut hell, Top mittel, Shorts und Schaft dunkel, Feder dunkler, Zehenkappe schwarz.",
         overlays="Regionen wie bei 1:200; die Linerkante trägt den Schaftwert."))}
R["50"] = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": P(0.25, 0.13, 0.1, 0.3),
  "hatches": [{"id": "carbon", "title": T("Carbon weave", "Karbongewebe"), "pattern": "cross", "angleDeg": 45, "spacingMm": 0.5, "penMm": 0.1}], "geometry": {"elevation": r50},
  "decisions": D(
    dict(silhouette="Fully articulated: finger separations, muscle lines of the arms and calf, singlet with armhole, neck line, bib outline and hem band, shoe with real sole, laces, eye, brow and mouth, the socket with trim, pyramid adapter, seam and carbon cross-hatch, the blade with bolt, laminate lines, carbon weave and the rubber toe tread; 35.8 mm tall.",
         retained="Everything of 1:100 plus the details the prosthesis is known for: adapter, laminate and tread.", merged="Hair is a cap without strands; the weave is a hatch.", omitted="Pores, stitching, shoe logo, bib print.",
         thinElements="Blade 0.48 mm real thickness, laminate lines and tread 0.10 mm texture lines, adapter 0.8 mm.",
         hatching="Carbon cross-hatch at 45 degrees, 0.5 mm spacing, 0.10 mm pen, only on socket and blade.", styles="Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; tones as at 1:100.",
         overlays="One region per shape; adapter and trim carry the socket value, laminate and tread the blade value."),
    dict(silhouette="Voll gegliedert: Fingertrennungen, Muskellinien an Armen und Wade, Top mit Armausschnitt, Halsausschnitt, Startnummernumriss und Saumband, Schuh mit echter Sohle, Schnürung, Auge, Braue und Mund, der Schaft mit Rand, Pyramidenadapter, Naht und Karbonkreuzschraffur, die Feder mit Schraube, Laminatlinien, Karbongewebe und Profil der Gummispitze; 35,8 mm hoch.",
         retained="Alles aus 1:100 und die Einzelheiten, für die die Prothese bekannt ist: Adapter, Laminat und Profil.", merged="Das Haar ist eine Kappe ohne Strähnen; das Gewebe ist eine Schraffur.", omitted="Poren, Stiche, Schuhlogo, Startnummernaufdruck.",
         thinElements="Feder in wahrer Dicke 0,48 mm, Laminatlinien und Profil als 0,10-mm-Texturlinien, Adapter 0,8 mm.",
         hatching="Karbon-Kreuzschraffur im Winkel 45 Grad, 0,5 mm Abstand, 0,10 mm Stift, nur an Schaft und Feder.", styles="Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Töne wie bei 1:100.",
         overlays="Eine Region je Form; Adapter und Rand tragen den Schaftwert, Laminat und Profil den Federwert."))}
asset = {
    "id": aid, "category": "humans",
    "title": T("Runner with Blade Prosthesis", "Läufer mit Prothesenfeder"),
    "summary": T("A 29-year-old man of 1.80 m with a right transtibial amputation, running at 3.1 m/s on a J-shaped carbon blade attached below a carbon socket, in a singlet and compression shorts and one running shoe. The gait has a flight phase, a shorter blade contact with 11 degrees of spring deflection, and the socket holds heat. Poses: standing, recovering, forefoot strike, blade mid-stance and flight.",
                 "Ein 29-jähriger Mann von 1,80 m mit rechtsseitiger Unterschenkelamputation, der mit 3,1 m/s auf einer J-förmigen Karbonfeder unter einem Karbonschaft läuft, in Top, Kompressionsshorts und einem Laufschuh. Der Gang hat eine Flugphase, einen kürzeren Federkontakt mit 11 Grad Einfederung, und der Schaft staut Wärme. Posen: Stehen, Erholen, Vorfußaufsatz, Feder im Stütz und Flug."),
    "tags": ["human", "adult", "man", "runner", "prosthesis", "blade", "transtibial", "amputee", "running", "flight-phase", "gait"],
    "provenance": {"author": "Semio staffage content worker NW-H3", "method": "handcrafted", "created": "2026-10-29",
                   "references": refs(["de-leva-1996", "winter-2009", "drillis-contini-1966", "perry-burnfield-2010", "dubois-1916", "iso-9920-2007", "ashrae-2021", "gupta-2010", "gagge-1971", "mifflin-1990", "icrp-66-1994"], EXTRA) + [{"id": k, "citation": v} for k, v in EXTRA.items()],
                   "notes": T("Rig scaled to 1.80 m (de Leva male fractions); the right shank and foot are replaced by a socket-and-pylon bone, an upper blade bone and a lower blade-tip bone that flexes under load. The running cycle is generated from planted ball and blade-tip paths with explicit flight phases; contact times, blade stiffness (11 degrees) and all thermal and ventilation values are authored estimates guided by the cited sources.",
                              "Skelett auf 1,80 m skaliert (de Leva, männliche Anteile); Unterschenkel und Fuß rechts sind durch einen Schaft-Pylon-Knochen, einen oberen Federknochen und einen unteren Federspitzenknochen ersetzt, der unter Last einfedert. Der Laufzyklus entsteht aus festen Ballen- und Federspitzenbahnen mit ausdrücklichen Flugphasen; Kontaktzeiten, Federsteifigkeit (11 Grad) und alle thermischen und Lüftungswerte sind geschätzt und an den genannten Quellen orientiert.")},
    "dimensions": {"heightM": 1.79, "widthM": 0.41, "depthM": 0.3, "massKg": MASS},
    "materials": mats, "rig": rig, "anchors": anchors, "representations": OrderedDict((k, R[k]) for k in ("500", "200", "100", "50")), "symbols": [],
    "poses": poses, "clips": clips, "thermal": thermal,
    "structure": {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": [], "assumptions": [
        T("Segments are point masses at the de Leva centres; the residual limb with socket (1.9 kg), the blade (0.55 kg) and the blade tip (0.35 kg) replace the right shank and foot.", "Segmente sind Punktmassen in den de-Leva-Schwerpunkten; Stumpf mit Schaft (1,9 kg), Feder (0,55 kg) und Federspitze (0,35 kg) ersetzen Unterschenkel und Fuß rechts."),
        T("Contacts are heel and ball of the intact foot and the rubber toe of the blade; the clip has two ballistic flight phases.", "Kontakte sind Ferse und Ballen des gesunden Fußes und die Gummispitze der Feder; der Clip hat zwei ballistische Flugphasen.")], "clinical": False},
    "presentation": {"defaultPose": "standing-ready", "defaultClip": "run-cycle", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}
errs = check(asset)
if "--write" in sys.argv:
    write_asset(aid, "humans", asset)
    print("written")
