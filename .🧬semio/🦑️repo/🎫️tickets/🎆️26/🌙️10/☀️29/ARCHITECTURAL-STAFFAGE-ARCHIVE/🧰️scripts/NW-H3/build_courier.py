from a_courier_geom import *
from h3asm import *
from h3check import check

aid = "courier-with-hand-truck"
SKN = dubois(82, 1.82)
mats = [m for m in BASE["materials"] if m["id"] in ("skin", "hair", "cotton-underwear", "cotton-sock")] + [
    material("hivis-softshell", "High-visibility polyester softshell jacket", "Warnschutz-Softshelljacke aus Polyester", 0.9, 0.55, cp=1300),
    material("fleece-liner", "Fleece liner", "Fleece-Innenjacke", 0.92, 0.6, cp=1300),
    material("cotton-workwear", "Cotton-polyester work trousers", "Arbeitshose aus Baumwoll-Polyester", 0.92, 0.7, cp=1300),
    material("leather-work-boot", "Leather work boots with rubber sole", "Lederarbeitsstiefel mit Gummisohle", 0.9, 0.75, cp=1500),
    material("fabric-glove", "Fabric work gloves with grip coating", "Arbeitshandschuhe aus Stoff mit Grifffläche", 0.9, 0.7, cp=1400),
    material("wool-cap", "Knitted cap with brim", "Strickmütze mit Schirm", 0.92, 0.7, cp=1300),
    material("steel-tube", "Painted steel and aluminium tube", "Lackiertes Stahl- und Aluminiumrohr", 0.85, 0.7, cp=470, rho=7800, k=50),
    material("tyre-rubber", "Pneumatic rubber tyre", "Luftreifen aus Gummi", 0.92, 0.9, cp=1100, rho=1100),
    material("cardboard", "Corrugated cardboard parcel", "Wellpappe-Paket", 0.9, 0.6, cp=1400, rho=200, k=0.06),
]
cap_skin = 13000
f_j = fcl_of(0.25 + 0.30 + 0.25)
j_layers = [garment("shirt", "Cotton long-sleeve shirt", "Baumwollhemd, langarm", 0.25, "cotton-shirt" if False else "cotton-underwear", 0.0012, 160, f_j),
            garment("fleece", "Fleece liner", "Fleece-Innenjacke", 0.30, "fleece-liner", 0.004, 450, f_j),
            garment("softshell", "High-visibility softshell jacket", "Warnschutz-Softshelljacke", 0.25, "hivis-softshell", 0.002, 300, f_j)]
f_t = fcl_of(0.04 + 0.35)
t_layers = [garment("briefs", "Cotton briefs", "Baumwollslip", 0.04, "cotton-underwear", 0.0008, 100, f_t),
            garment("workwear", "Cotton-polyester cargo trousers", "Cargohose aus Baumwoll-Polyester", 0.35, "cotton-workwear", 0.0015, 480, f_t)]
b_layers = [garment("socks", "Thick cotton socks", "Dicke Baumwollsocken", 0.05, "cotton-sock", 0.003, 160, 1.0),
            garment("boot", "Leather work boot", "Lederarbeitsstiefel", 0.06, "leather-work-boot", 0.005, 1100, 1.0)]
g_layers = [garment("glove", "Fabric work glove", "Arbeitshandschuh", 0.10, "fabric-glove", 0.002, 250, 1.0)]
c_layers = [garment("cap", "Knitted cap with brim", "Strickmütze mit Schirm", 0.30, "wool-cap", 0.004, 350, 1.0)]
cap = lambda ls, base=cap_skin: base + sum(l["heatCapacityJM2K"] for l in ls)
regions = [
    body_region(FACE, "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", SKN * 0.07, 1.0, "head", 0.07, 0.07, 0.2, "sphere", 0.5, cap_skin, [], ref="dubois-1916"),
    body_region(CAP, "Cap over the scalp", "Mütze über der Kopfhaut", "clothing", "wool-cap", SKN * 0.03, 1.0, "head", 0.04, 0.03, 0.2, "sphere", 0.8, cap(c_layers, 8000), c_layers),
    body_region(GLOVE, "Gloved hands", "Behandschuhte Hände", "clothing", "fabric-glove", SKN * 0.05, 1.0, "hand", 0.04, 0.05, 0.1, "cylinder", 0.2, cap(g_layers), g_layers),
    body_region(JACKET, "Shirt, fleece and high-visibility jacket on trunk and arms", "Hemd, Fleece und Warnschutzjacke an Rumpf und Armen", "clothing", "hivis-softshell", SKN * 0.49, f_j, "trunk", 0.54, 0.49, 0.3, "vertical-plate", 0.45, cap(j_layers), j_layers),
    body_region(TROU, "Cargo trousers over the legs", "Cargohose über den Beinen", "clothing", "cotton-workwear", SKN * 0.30, f_t, "leg", 0.28, 0.30, 0.14, "cylinder", 0.1, cap(t_layers), t_layers),
    body_region(BOOT, "Socks and work boots", "Socken und Arbeitsstiefel", "surface", "leather-work-boot", SKN * 0.06, 1.0, "foot", 0.03, 0.06, 0.1, "horizontal-plate-up", 0.1, cap(b_layers), b_layers),
    {"id": TRUCK, "title": T("Hand truck frame, nose plate and handle", "Sackkarrenrahmen, Schaufel und Griff"), "kind": "surface", "model": "passive", "material": "steel-tube", "areaM2": 0.55, "characteristicLengthM": 0.03,
     "geometry": "cylinder", "skyViewFactor": 0.4, "layers": [], "heatCapacityJM2K": 10000, "source": "authored-estimate"},
    {"id": TYRE, "title": T("Pneumatic tyres of the hand truck", "Luftreifen der Sackkarre"), "kind": "tyre", "model": "passive", "material": "tyre-rubber", "areaM2": 0.07, "characteristicLengthM": 0.26,
     "geometry": "cylinder", "skyViewFactor": 0.2, "layers": [], "heatCapacityJM2K": 25000, "source": "authored-estimate"},
    {"id": PARC, "title": T("Stack of three cardboard parcels", "Stapel aus drei Pappkartons"), "kind": "surface", "model": "passive", "material": "cardboard", "areaM2": 1.6, "characteristicLengthM": 0.4,
     "geometry": "vertical-plate", "skyViewFactor": 0.35, "layers": [], "heatCapacityJM2K": 4000, "source": "authored-estimate"},
]
resp = {"noseEmitter": "nose-breath", "mouthEmitter": "mouth-breath", "exhaledTempC": 34, "exhaledRelativeHumidity": 0.95,
        "patterns": [
            {"id": "rest-standing", "title": T("Standing and waiting", "Stehen und Warten"), "rateBpm": 13, "tidalVolumeL": 0.55, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
            {"id": "pushing-walk", "title": T("Walking with the loaded hand truck", "Gehen mit der beladenen Sackkarre"), "rateBpm": 24, "tidalVolumeL": 1.3, "inhaleFraction": 0.45, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.03, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"},
            {"id": "tilting-effort", "title": T("Tilting the load, held breath effort", "Last kippen, angehaltene Atmung bei Anstrengung"), "rateBpm": 20, "tidalVolumeL": 1.0, "inhaleFraction": 0.40, "pauseAfterInhaleFraction": 0.10, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.05, "route": "mouth", "source": "authored-estimate", "reference": "icrp-66-1994"}],
        "motion": [{"bone": "torso", "angleDeg": 0.6, "dy": 0.004}], "source": "authored-estimate", "reference": "gupta-2010"}
thermal = OrderedDict()
thermal["environment"] = {"airTempC": 6, "meanRadiantTempC": 6, "skyTempC": -6, "groundTempC": 7, "relativeHumidity": 0.8, "windMS": 2.5, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
thermal["baseMet"] = 1.5
thermal["body"] = {"species": "human", "massKg": 82, "skinAreaM2": SKN, "basalMetabolismW": mifflin("m", 82, 182, 34), "metWM2": 58.15, "coreSetPointC": 36.8, "skinSetPointC": 34.1, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 1.0, "source": "authored-estimate", "reference": "mifflin-1990"}
thermal["respiration"] = resp
thermal["operating"] = []
thermal["passengers"] = []
thermal["regions"] = regions
thermal["emitters"] = breath_emitters(plume_region=JACKET)
thermal["assumptions"] = [
    T(f"Skin area {SKN} m2 after DuBois and DuBois (1916) for 82 kg and 1.82 m; skin fractions 0.07 face and neck, 0.03 scalp under the cap, 0.05 hands, 0.49 trunk and arms, 0.30 legs, 0.06 feet (rule-of-nines proportions, authored).",
      f"Hautfläche {SKN} m2 nach DuBois und DuBois (1916) für 82 kg und 1,82 m; Hautanteile 0,07 Gesicht und Hals, 0,03 Kopfhaut unter der Mütze, 0,05 Hände, 0,49 Rumpf und Arme, 0,30 Beine, 0,06 Füße (Neunerregel-Proportionen, geschätzt)."),
    T("Basal metabolism from the Mifflin-St Jeor equation (34 years, 82 kg, 182 cm); clothing in clo after ISO 9920 (1 clo = 0.155 m2K/W), digits authored; delivery work with a 49 kg load is rated at about 3.2 met while walking (ISO 8996 heavy-hand-work range, authored).",
      "Grundumsatz aus der Mifflin-St-Jeor-Gleichung (34 Jahre, 82 kg, 182 cm); Kleidung in clo nach ISO 9920 (1 clo = 0,155 m2K/W), Werte geschätzt; Lieferarbeit mit 49 kg Last wird beim Gehen mit etwa 3,2 met angesetzt (ISO 8996, Bereich schwerer Handarbeit, geschätzt)."),
    T("External work is the rolling and gradient resistance of the loaded truck, about 12 W at 0.95 m/s (rolling coefficient 0.02, authored); the load and the truck are passive surfaces without internal heat.",
      "Die äußere Arbeit ist der Roll- und Steigungswiderstand der beladenen Karre, etwa 12 W bei 0,95 m/s (Rollwiderstandsbeiwert 0,02, geschätzt); Last und Karre sind passive Flächen ohne innere Wärme."),
    T("The parcels' heat capacity of 4 kJ/(m2K) lumps cardboard and the coupled outer contents; the cardboard is not heated by the body.",
      "Die Wärmekapazität der Pakete von 4 kJ/(m2K) fasst Pappe und den angekoppelten äußeren Inhalt zusammen; die Pappe wird nicht vom Körper erwärmt."),
]
EXP = {JACKET: 0.9}


def tb(met, br, plume, work=0.0):
    t = binding(regions, {"body-plume": plume}, met, br, exposed=EXP, contact={BOOT: {"fraction": 0.3, "tempC": 7}})
    if work:
        t["externalWorkW"] = work
    return t


# ---- poses
mpos = {k: POSES[k] for k in POSES}
# walking frames
G = geom_of(a)
spec = dict(heel=(0, 0.34), b0=0.10, b1=0.64, X0=0.30, clear=0.075, delta_keys=[(0, 15), (0.10, 0), (0.34, 0), (0.64, -25), (0.78, -8), (0.9, 6), (1.0, 15)])
NF, TT, VV = 36, 1.2, 0.95
frames, G = gait_frames(a, NF, TT, VV, spec, ypelvis_nominal=G["hip_y"] - 0.005, bob=0.035)
LEGS = cycle_offsets(a, G, frames, spec, NF)


def walk_key(i):
    off = {k: dict(v) for k, v in LEGS[i][0].items()}
    root = {"x": frames[i]["xp"], "y": frames[i]["yp"] - HIP}
    ph = 2 * math.pi * i / NF
    off["torso"] = {"angleDeg": round(-6 + 1.2 * math.sin(2 * ph), 4)}
    off["neck"] = {"angleDeg": 3.0}
    off["head"] = {"angleDeg": round(1.0 - 0.8 * math.sin(2 * ph), 4)}
    off["truck-lift"] = {"dx": round(-root["y"], 5)}
    off["hand-truck"] = {"angleDeg": 0.0}
    arms_to_handle(off, root, -8.0)
    return pose_clean(off), root


WK = [walk_key(i) for i in range(NF)]
off0, root0 = WK[0]
walk_pose_bones = off0
poses = []
TIT = {
    "waiting": (T("Waiting with the loaded truck", "Warten mit der beladenen Karre"), T("Feet together, truck tilted back 22 degrees on its wheels, both gloved hands on the handle, parcels leaning against the frame.", "Füße zusammen, Karre 22 Grad nach hinten auf den Rädern geneigt, beide behandschuhten Hände am Griff, Pakete lehnen am Rahmen.")),
    "tilted-back": (T("Tilting the load back", "Last zurückkippen"), T("The courier pulls the handle towards the body to tip the load onto the wheels, truck tilted back 31 degrees, arms nearly straight.", "Der Kurier zieht den Griff zum Körper, um die Last auf die Räder zu kippen, Karre 31 Grad geneigt, Arme fast gestreckt.")),
    "parked-upright": (T("Parked upright", "Aufrecht abgestellt"), T("Truck stood nearly upright (12 degrees), one hand resting on the handle, the other arm hanging, courier slightly bent.", "Karre fast aufrecht abgestellt (12 Grad), eine Hand ruht auf dem Griff, der andere Arm hängt, der Kurier leicht vorgebeugt.")),
}
CT = {"waiting": FEET + ["wheel-contact"], "tilted-back": FEET + ["wheel-contact"], "parked-upright": FEET + ["wheel-contact"]}
LOADS = [{"id": "parcels-weight", "kind": "carried", "title": T("Weight of the three parcels", "Gewicht der drei Pakete"), "anchor": "parcels-cog", "fxN": 0, "fyN": -round(PARCELS * 9.80665, 2), "source": "authored-estimate"}]
TBK = {"waiting": tb(1.5, "rest-standing", 0.6), "tilted-back": tb(2.4, "tilting-effort", 0.8, 5.0), "parked-upright": tb(1.6, "rest-standing", 0.6)}
for k, nm in (("waiting", "waiting"), ("tilted-back", "tilted-back"), ("parked-upright", "parked-upright")):
    poses.append({"id": nm, "title": TIT[k][0], "description": TIT[k][1], "bones": POSES[k][0], "ground": "auto", "contacts": CT[k], "thermal": TBK[k], "loads": LOADS})
poses.append({"id": "walking-contact", "title": T("Walking, left heel strike", "Gehen, linker Fersenkontakt"), "description": T("Frame 0 of the push-walk cycle: left heel strikes, right foot pushes off, truck rolling beside the courier.", "Bild 0 des Schiebe-Gehzyklus: Der linke Fuß setzt mit der Ferse auf, der rechte stößt ab, die Karre rollt vor dem Kurier."),
              "bones": walk_pose_bones, "ground": "auto", "contacts": ["heel-l", "ball-r", "wheel-contact"], "thermal": tb(3.2, "pushing-walk", 1.0, 12.0), "loads": LOADS})
PBY = {p["id"]: p for p in poses}

keys = []
for i in range(NF):
    keys.append({"t": round(i * TT / NF, 5), "pose": "walking-contact", "ease": "smooth", "bones": WK[i][0]})
keys.append({"t": TT, "pose": "walking-contact", "ease": "smooth", "bones": WK[0][0]})
contacts = []
for sd, ph in (("l", 0.0), ("r", 0.5)):
    for w in ("heel", "ball"):
        for f, t in spans_of(spec, ph, TT, w):
            contacts.append({"anchor": f"{w}-{sd}", "fromS": f, "toS": t})
ev = []
for c in contacts:
    ev.append((c["fromS"], "contact-on", f"{c['anchor']} on", f"{c['anchor']} auf", c["anchor"]))
    ev.append((c["toS"], "contact-off", f"{c['anchor']} off", f"{c['anchor']} ab", c["anchor"]))
ev = [e for e in ev if 0 <= e[0] <= TT]
ev.sort(key=lambda e: e[0])
c1 = {"id": "push-walk", "title": T("Walking with the loaded hand truck", "Gehen mit der beladenen Sackkarre"),
      "description": T("Level walking at 0.95 m/s with a 49 kg load, 100 steps per minute; feet planted without sliding, torso leaning 6 degrees forward, both hands on the handle, truck wheels rolling beside the courier with the axle height held on the ground.", "Gehen in der Ebene mit 0,95 m/s und 49 kg Last, 100 Schritte pro Minute; Füße gleiten nicht, Oberkörper 6 Grad nach vorn geneigt, beide Hände am Griff, die Karrenräder rollen vor dem Kurier, die Achshöhe bleibt am Boden."),
      "durationS": TT, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": round(VV * TT, 6), "keys": keys, "contacts": contacts,
      "phases": [{"id": "double-support-left-lead", "fromS": 0, "toS": round(0.14 * TT, 4), "title": T("Double support, left foot leading", "Doppelstandphase, linker Fuß vorn")},
                 {"id": "left-single-support", "fromS": round(0.14 * TT, 4), "toS": round(0.5 * TT, 4), "title": T("Left single support", "Linke Einbeinstandphase")},
                 {"id": "double-support-right-lead", "fromS": round(0.5 * TT, 4), "toS": round(0.64 * TT, 4), "title": T("Double support, right foot leading", "Doppelstandphase, rechter Fuß vorn")},
                 {"id": "right-single-support", "fromS": round(0.64 * TT, 4), "toS": TT, "title": T("Right single support", "Rechte Einbeinstandphase")}],
      "events": clip_events(ev),
      "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3.2}, {"t": TT, "v": 3.2}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": TT, "v": 1}]},
                 {"target": "externalWorkW", "ease": "smooth", "keys": [{"t": 0, "v": 12}, {"t": TT, "v": 12}]}]}

# tilt clip: legs fixed to waiting
wl = {b: dict(v) for b, v in POSES["waiting"][0].items() if b.split("-")[0] in ("thigh", "shank", "foot", "toe")}
root_w = {"x": 0.0, "y": ground_y(a, POSES["waiting"][0], CT["waiting"])}


def tilt_key(tilt, torso, neck, head, hand):
    off = {k: dict(v) for k, v in wl.items()}
    off["torso"] = {"angleDeg": torso}; off["neck"] = {"angleDeg": neck}; off["head"] = {"angleDeg": head}
    off["truck-lift"] = {"dx": POSES["waiting"][0]["truck-lift"]["dx"]}
    off["hand-truck"] = {"angleDeg": tilt}
    arms_to_handle(off, root_w, hand)
    return pose_clean(off)


c2 = {"id": "tilt-and-settle", "title": T("Tipping the load onto the wheels", "Last auf die Räder kippen"),
      "description": T("Non-looping, five seconds: from waiting the courier pulls the handle in, the truck tilts from 22 to 31 degrees with a short effort breath, then settles back; feet and wheel stay put.", "Ohne Wiederholung, fünf Sekunden: Aus dem Warten zieht der Kurier den Griff heran, die Karre kippt von 22 auf 31 Grad bei kurzer Anstrengungsatmung und setzt sich dann zurück; Füße und Rad bleiben stehen."),
      "durationS": 5, "fps": 30, "loop": False, "rootMotion": "none", "keys": [
          {"t": 0, "pose": "waiting", "ease": "smooth"},
          {"t": 1.2, "pose": "waiting", "ease": "smooth", "bones": tilt_key(5.0, -2, 3, 1, -12)},
          {"t": 2.4, "pose": "tilted-back", "ease": "smooth", "bones": {**wl, **{k: v for k, v in tilt_key(9.0, -1, 3, 2, -22).items() if k not in wl}}},
          {"t": 3.8, "pose": "waiting", "ease": "smooth", "bones": tilt_key(3.0, -2, 3, 1, -12)},
          {"t": 5, "pose": "waiting", "ease": "smooth"}],
      "contacts": [{"anchor": c, "fromS": 0, "toS": 5} for c in CT["waiting"]],
      "phases": [{"id": "pull-in", "fromS": 0, "toS": 2.4, "title": T("Pulling the handle in", "Griff heranziehen")}, {"id": "settle", "fromS": 2.4, "toS": 5, "title": T("Settling back", "Zurücksetzen")}],
      "events": clip_events([(2.4, "load-on", "Load rests fully on the wheels", "Last ruht ganz auf den Rädern", "parcels-weight")]),
      "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.0, "v": 2}, {"t": 3.2, "v": 0}, {"t": 5, "v": 0}]},
                 {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.5}, {"t": 2.4, "v": 2.6}, {"t": 5, "v": 1.5}]}]}
clips = [c1, c2]

P = lambda b, d, t, g: {"body": b, "detail": d, "texture": t, "glazing": None, "ground": g}
D = lambda en, de: decisions(en, de)
R = {}
R["500"] = {"status": "authored", "denominator": 500, "validRange": {"minDenominator": 350, "maxDenominator": 1000}, "pens": P(0.13, None, None, 0.18), "hatches": [], "geometry": {"elevation": rep_500()},
  "decisions": D(
    dict(silhouette="An upright 1.82 m figure (3.6 mm) with a cap head, one torso mass, the near arm as three links reaching the handle, two leg columns with boots, and beside it the leaning hand truck: three filled parcel blocks, one wheel disc and the frame as 0.13 mm strokes.",
         retained="The cap brim that identifies a courier, the forward-reaching arm to the handle, the three parcel blocks that give the load its height, the wheel and the tilt of the frame, exact stature and the 1.16 m extent.",
         merged="Face and cap in one head mass; jacket in one torso mass; the nose plate and the handle are single strokes.",
         omitted="The far arm, neck, cuffs, tape, labels, hub, tread, laces, reflective bands.",
         thinElements="The 28 mm frame tubes and the 12 mm nose plate are strokes of 0.13 mm (65 mm real) - drawn as lines, not thickened shapes; the handle bend is one stroke.",
         hatching="None.", styles="Jacket mid-grey, trousers darker, boots light, parcels in three greys, tyre near black; pen 0.13 mm only; light mode dark lines on white, dark mode inverted.",
         overlays="The head mass lists face and cap, the torso mass the jacket; legs, boots, frame strokes, wheel and each parcel keep their own region."),
    dict(silhouette="Eine aufrechte Figur von 1,82 m (3,6 mm) mit Mützenkopf, einer Rumpfmasse, dem nahen Arm als drei Gliedern zum Griff, zwei Beinsäulen mit Stiefeln und daneben die geneigte Sackkarre: drei gefüllte Paketblöcke, eine Radscheibe und der Rahmen als 0,13-mm-Striche.",
         retained="Der Mützenschirm, der den Kurier kennzeichnet, der nach vorn greifende Arm zum Griff, die drei Paketblöcke, die der Last ihre Höhe geben, das Rad und die Neigung des Rahmens, genaue Körpergröße und die Ausdehnung von 1,16 m.",
         merged="Gesicht und Mütze in einer Kopfmasse; Jacke in einer Rumpfmasse; Schaufel und Griff sind einzelne Striche.",
         omitted="Der ferne Arm, Hals, Bündchen, Klebeband, Etiketten, Nabe, Profil, Schnürung, Reflexstreifen.",
         thinElements="Die 28 mm dicken Rahmenrohre und die 12 mm dicke Schaufel sind Striche von 0,13 mm (65 mm real) - als Linien gezeichnet, nicht als verdickte Formen; die Griffbiegung ist ein Strich.",
         hatching="Keine.", styles="Jacke mittelgrau, Hose dunkler, Stiefel hell, Pakete in drei Grautönen, Reifen fast schwarz; nur Strichstärke 0,13 mm; hell: dunkle Linien auf Weiß, dunkel: invertiert.",
         overlays="Die Kopfmasse nennt Gesicht und Mütze, die Rumpfmasse die Jacke; Beine, Stiefel, Rahmenstriche, Rad und jedes Paket behalten ihre eigene Region."))}
R["200"] = {"status": "authored", "denominator": 200, "validRange": {"minDenominator": 150, "maxDenominator": 350}, "pens": P(0.18, 0.10, None, 0.2), "hatches": [], "geometry": {"elevation": rep_200()},
  "decisions": D(
    dict(silhouette="Main masses separated: head, cap with brim, neck, jacket, both arms as sleeve, forearm and glove, both legs as trousers plus boot; the truck as frame strokes, nose plate, handle loop, a wheel and three parcels; 9.1 mm tall.",
         retained="Elbows and knees as separate masses, the cap brim, the reflective band as the one inner line of the jacket, the handle loop, the nose plate under the lowest parcel.",
         merged="Thumb into the glove, hem into the jacket, sock into the boot, axle stay into the wheel.",
         omitted="Fingers, ear, eye, cuffs, zip, pockets, laces, tape, hub detail.",
         thinElements="Frame tubes (28 mm) are 0.18 mm strokes; the brace under the plate is a 0.10 mm detail line; fingers dropped.",
         hatching="None; fill tones carry the materials.", styles="Body pen 0.18 mm, detail pen 0.10 mm; light skin and boots, mid jacket, dark trousers, cap and tyre.",
         overlays="One region per mass; both sleeves share the jacket value; frame, nose plate, handle and brace share the truck value; each parcel carries the parcels value."),
    dict(silhouette="Hauptmassen getrennt: Kopf, Mütze mit Schirm, Hals, Jacke, beide Arme als Ärmel, Unterarm und Handschuh, beide Beine als Hose mit Stiefel; die Karre als Rahmenstriche, Schaufel, Griffschlaufe, Rad und drei Pakete; 9,1 mm hoch.",
         retained="Ellbogen und Knie als getrennte Massen, der Mützenschirm, das Reflexband als einzige Innenlinie der Jacke, die Griffschlaufe, die Schaufel unter dem untersten Paket.",
         merged="Daumen in den Handschuh, Saum in die Jacke, Socke in den Stiefel, Achsstrebe in das Rad.",
         omitted="Finger, Ohr, Auge, Bündchen, Reißverschluss, Taschen, Schnürung, Klebeband, Nabendetail.",
         thinElements="Rahmenrohre (28 mm) sind Striche von 0,18 mm; die Strebe unter der Schaufel ist eine 0,10-mm-Detaillinie; Finger entfallen.",
         hatching="Keine; die Füllungstöne tragen die Materialien.", styles="Körperstift 0,18 mm, Detailstift 0,10 mm; helle Haut und Stiefel, mittlere Jacke, dunkle Hose, Mütze und Reifen.",
         overlays="Eine Region je Masse; beide Ärmel teilen den Jackenwert; Rahmen, Schaufel, Griff und Strebe teilen den Karrenwert; jedes Paket trägt den Paketwert."))}
R["100"] = {"status": "authored", "denominator": 100, "validRange": {"minDenominator": 75, "maxDenominator": 150}, "pens": P(0.2, 0.1, None, 0.25),
  "hatches": [{"id": "cotton", "title": T("Cotton twill lines", "Baumwoll-Köperlinien"), "pattern": "lines", "angleDeg": 80, "spacingMm": 0.8, "penMm": 0.1}], "geometry": {"elevation": rep_100()},
  "decisions": D(
    dict(silhouette="Articulated workwear: cap with brim and ear, jacket with collar, zip and two reflective bands, sleeves with cuff lines, mitten gloves with thumb, cargo trousers with thigh pocket, boots with sole line; the truck with a real 0.28 mm frame tube, nose plate, bent handle, tyre with hub and three parcels with tape; 18.2 mm tall.",
         retained="Reflective bands (they identify the job), the cargo pocket, the tape strips on the parcels, the hub, the handle bend.",
         merged="Eye, mouth, laces and the trouser seams are not drawn; fingers form mittens.",
         omitted="Eye (0.16 mm), laces, label, tread, stitching.",
         thinElements="The frame tube is drawn at its real 28 mm = 0.28 mm; the handle is a 0.22 mm stroke; the plate is a real 0.25 mm slab.",
         hatching="Trousers only: fine vertical twill lines, 0.8 mm spacing, 0.10 mm pen.", styles="Body 0.2 mm, detail 0.1 mm; jacket mid-grey, trousers dark with twill, boots light, parcels in three greys.",
         overlays="Regions as at 1:200; the hub shares the tyre value, tape strips the parcels value, reflective bands the jacket value."),
    dict(silhouette="Gegliederte Arbeitskleidung: Mütze mit Schirm und Ohr, Jacke mit Kragen, Reißverschluss und zwei Reflexstreifen, Ärmel mit Bündchenlinien, Fäustlingshandschuhe mit Daumen, Cargohose mit Oberschenkeltasche, Stiefel mit Sohlenlinie; die Karre mit echtem 0,28-mm-Rahmenrohr, Schaufel, gebogenem Griff, Reifen mit Nabe und drei Pakete mit Klebeband; 18,2 mm hoch.",
         retained="Reflexstreifen (sie kennzeichnen den Beruf), die Cargotasche, die Klebebänder auf den Paketen, die Nabe, die Griffbiegung.",
         merged="Auge, Mund, Schnürung und Hosennähte werden nicht gezeichnet; Finger bilden Fäustlinge.",
         omitted="Auge (0,16 mm), Schnürsenkel, Etikett, Profil, Nähte.",
         thinElements="Das Rahmenrohr ist in wahrer Größe von 28 mm = 0,28 mm gezeichnet; der Griff ist ein Strich von 0,22 mm; die Schaufel ist eine echte Platte von 0,25 mm.",
         hatching="Nur die Hose: feine senkrechte Köperlinien, 0,8 mm Abstand, 0,10 mm Stift.", styles="Körper 0,2 mm, Detail 0,1 mm; Jacke mittelgrau, Hose dunkel mit Köper, Stiefel hell, Pakete in drei Grautönen.",
         overlays="Regionen wie bei 1:200; die Nabe teilt den Reifenwert, Klebebänder den Paketwert, Reflexstreifen den Jackenwert."))}
R["50"] = {"status": "authored", "denominator": 50, "validRange": {"minDenominator": 20, "maxDenominator": 75}, "pens": P(0.25, 0.13, 0.1, 0.3),
  "hatches": [{"id": "cotton", "title": T("Cotton twill lines", "Baumwoll-Köperlinien"), "pattern": "lines", "angleDeg": 80, "spacingMm": 0.6, "penMm": 0.1}, {"id": "softshell", "title": T("Softshell weave lines", "Softshell-Gewebelinien"), "pattern": "lines", "angleDeg": 45, "spacingMm": 0.7, "penMm": 0.1}], "geometry": {"elevation": rep_50()},
  "decisions": D(
    dict(silhouette="Fully articulated: cuff straps, finger separations, chest pocket, zip, collar, reflective bands as real 0.056 mm... filled strips, knee patches, boot soles with three lace crossings, cap seam and button, eye and mouth; tyre with six tread ticks; parcel label and strap; 36.4 mm tall.",
         retained="Everything of 1:100 plus the construction details still wider than 0.1 mm.",
         merged="Eyelets merge into lace strokes; weave is a hatch, not strokes.", omitted="Pores, stitching holes, fibres, parcel print, valve of the tyre.",
         thinElements="Fingers, laces, seams and tread are 0.10 mm texture lines; reflective bands are real 28 mm = 0.56 mm strips.",
         hatching="Jacket and sleeves: 45 degree weave lines at 0.7 mm; trousers: vertical twill at 0.6 mm.", styles="Body 0.25 mm, detail 0.13 mm, texture 0.10 mm; boot soles near black, tones as at 1:100.",
         overlays="One region per shape; details inherit the garment region, the label and strap the parcels region."),
    dict(silhouette="Voll gegliedert: Bündchenriemen, Fingertrennungen, Brusttasche, Reißverschluss, Kragen, Reflexstreifen als echte Streifen, Kniepolster, Stiefelsohlen mit drei Schnürkreuzungen, Mützennaht und -knopf, Auge und Mund; Reifen mit sechs Profilstrichen; Paketetikett und -gurt; 36,4 mm hoch.",
         retained="Alles aus 1:100 und die Konstruktionsdetails, die noch breiter als 0,1 mm sind.",
         merged="Ösen gehen in den Schnürstrichen auf; das Gewebe ist eine Schraffur, keine Striche.", omitted="Poren, Stichlöcher, Fasern, Paketaufdruck, Reifenventil.",
         thinElements="Finger, Schnürsenkel, Nähte und Profil sind 0,10-mm-Texturlinien; die Reflexstreifen sind echte Streifen von 28 mm = 0,56 mm.",
         hatching="Jacke und Ärmel: Gewebelinien im Winkel 45 Grad mit 0,7 mm; Hose: senkrechter Köper mit 0,6 mm.", styles="Körper 0,25 mm, Detail 0,13 mm, Textur 0,10 mm; Stiefelsohlen fast schwarz, Töne wie bei 1:100.",
         overlays="Eine Region je Form; Details erben die Region des Kleidungsstücks, Etikett und Gurt die Paketregion."))}
R["50"]["decisions"]["silhouette"]["en"] = R["50"]["decisions"]["silhouette"]["en"].replace("as real 0.056 mm... filled strips", "as filled strips")

asset = {
    "id": aid, "category": "humans",
    "title": T("Courier with Hand Truck", "Kurier mit Sackkarre"),
    "summary": T("A 34-year-old parcel courier of 1.82 m in a high-visibility jacket, cargo trousers, work boots and a cap, wheeling a steel hand truck loaded with three parcels (38 kg) tilted back on its wheels. Poses: waiting, tilting the load, parked upright; clips: walking with the load and tipping it onto the wheels.",
                 "Ein 34-jähriger Paketkurier von 1,82 m in Warnschutzjacke, Cargohose, Arbeitsstiefeln und Mütze, der eine Stahl-Sackkarre mit drei Paketen (38 kg) auf den Rädern nach hinten geneigt schiebt. Posen: Warten, Last kippen, aufrecht abgestellt; Clips: Gehen mit Last und Kippen der Last auf die Räder."),
    "tags": ["human", "adult", "man", "courier", "delivery", "hand-truck", "load", "workwear", "walking"],
    "provenance": {"author": "Semio staffage content worker NW-H3", "method": "handcrafted", "created": "2026-10-29",
                   "references": refs(["de-leva-1996", "winter-2009", "drillis-contini-1966", "perry-burnfield-2010", "dubois-1916", "iso-9920-2007", "iso-8996-2021", "ashrae-2021", "gupta-2010", "gagge-1971", "mifflin-1990", "icrp-66-1994"]),
                   "notes": T("Rig scaled from the archive's human template to 1.82 m (de Leva male fractions) with an added truck frame on a vertical float so that the wheel stays on the ground, and a fixed parcels bone. The walk is generated from planted heel and ball paths (gait cycle phases after Perry and Burnfield); everything else, in particular the load masses, is authored.",
                              "Skelett aus der Menschenvorlage des Archivs auf 1,82 m skaliert (de Leva, männliche Anteile) mit einem zusätzlichen Karrenrahmen auf senkrechter Führung, damit das Rad am Boden bleibt, und einem festen Paketknochen. Der Gang entsteht aus Fersen- und Ballenbahnen mit festem Auftritt (Gangzyklus nach Perry und Burnfield); alles andere, besonders die Lastmassen, ist geschätzt.")},
    "dimensions": {"heightM": 1.82, "widthM": 1.16, "depthM": 0.6, "massKg": MASS},
    "materials": mats, "rig": rig, "anchors": anchors, "representations": OrderedDict((k, R[k]) for k in ("500", "200", "100", "50")), "symbols": [],
    "poses": poses, "clips": clips, "thermal": thermal,
    "structure": {"gravityMS2": 9.80665, "dynamics": "inertial", "supports": [], "assumptions": [
        T("Segments are point masses at the de Leva centres; the 11 kg hand truck and the 38 kg parcels are point masses on their own bones, and the parcel weight is also applied as a load at its centre.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; die 11 kg schwere Sackkarre und die 38 kg schweren Pakete sind Punktmassen auf eigenen Knochen, das Paketgewicht wird zusätzlich als Last im Schwerpunkt angesetzt."),
        T("Contacts are heel and ball of each foot and the wheel contact of the truck; the courier carries the share of the load that the handle transfers.", "Kontakte sind Ferse und Ballen jedes Fußes und der Radkontakt der Karre; der Kurier trägt den Lastanteil, den der Griff überträgt.")], "clinical": False},
    "presentation": {"defaultPose": "waiting", "defaultClip": "push-walk", "defaultScale": 100, "viewpoints": ["elevation"]},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"], "completion": "authored"}
errs = check(asset)
if "--write" in sys.argv:
    write_asset(aid, "humans", asset)
    print("written")
