"""🎒️ Builds `👤️humans/schoolchild-with-backpack`: an eight-year-old walking to school with a loaded school backpack.

Derived by hand from the archive's `running-child` (a six-year-old): per-segment growth factors from 6 to 8 years (legs 1.18, arms 1.15,
trunk 1.11, neck 1.10, head 1.03; sitting-height ratio 0.555 to 0.535 after Fredriks et al. 2005), the small backpack redrawn as a
school backpack (taller, wider, deeper), the T-shirt and shorts turned into a long-sleeved jersey top and cotton trousers, a new
walking gait solved by inverse kinematics of the grown rig, new poses, clips, thermal and structural data and all texts.

Run: python3 build_schoolchild.py
@see ./lib.py ./gait.py ./tools.py
"""
import copy
import math

from gait import contact_spans, solve_cycle
from lib import T, affine_bone, load, r, rescale
from tools import (dubois, drop_shapes, feature_loop, rename_layer, rename_material, rename_region, replace_text, save, set_dims, shapes, solids, summarize, tone_of)

SLUG = "schoolchild-with-backpack"
STATURE = 1.28
BODY_KG = 25.2
CLOTHES_KG = 0.7
PACK_KG = 2.6

F = {"head": 1.03, "neck": 1.10, "pelvis": 1.11, "torso": 1.11, "pack": 1.11, "clavicle-l": 1.11, "clavicle-r": 1.11, "ilium-l": 1.11, "ilium-r": 1.11}
for side in "lr":
    for b in ("thigh", "shank", "foot", "toe"):
        F[f"{b}-{side}"] = 1.18
    for b in ("arm", "forearm", "hand"):
        F[f"{b}-{side}"] = 1.15

GAIT = dict(T=0.86, stride=0.90, footFlat=0.1, heelOff=0.38, toeOff=0.6, pitchStrike=13, pitchOff=-30, pitchSwing=6, clearance=0.035, clearancePeak=0.35, midStance=0.3, hipAhead=0.07, kneeMid=12, kneeMin=3, armBias=1, armSwing=13, elbowBias=12, elbowSwing=14, lean=-5, leanSwing=0.6)
GAIT["speed"] = GAIT["stride"] / GAIT["T"]
N_KEYS = 16


def geometry(doc):
    affine_bone(doc, "pack", (0.04, 0.03, 0.0), (1.12, 1.45, 1.25))
    affine_bone(doc, "pack", (0.04, 0.033, 0.0), (1.0, 1.19, 1.0), scales=("200",))
    affine_bone(doc, "pack", (0.04, 0.033, 0.0), (1.0, 1.1, 1.0), scales=("500",))
    for sc, view, s in shapes(doc):
        if s["id"] == "pack-seam" and view == "elevation":
            outline = next(x for x in doc["asset"]["representations"][sc]["geometry"]["elevation"] if x["id"] == "pack")["points"]
            u = s["points"][0][0]
            vs = []
            for (u0, v0), (u1, v1) in zip(outline, outline[1:] + outline[:1]):
                if (u0 - u) * (u1 - u) <= 0 and u0 != u1:
                    vs.append(v0 + (v1 - v0) * (u - u0) / (u1 - u0))
            s["points"] = [[u, r(min(vs) + 0.006)], [u, r(max(vs) - 0.006)]]
    rename_region(doc, "tshirt", "top")
    rename_region(doc, "shorts", "trousers")
    rename_material(doc, "cotton-short", "cotton-twill")
    rename_material(doc, "nylon-pack", "polyester-pack")
    rename_layer(doc, "top", "tshirt", "top")
    rename_layer(doc, "trousers", "shorts", "trousers")
    for sc, view, s in shapes(doc):
        base = s["id"].rsplit("-", 1)[0]
        side = s["id"].rsplit("-", 1)[-1]
        if base in ("forearm", "upper-arm") and s.get("region") == "arm-skin":
            s["region"], s["material"] = "top", "cotton-jersey"
            s["tone"] = tone_of(doc, sc, view, f"sleeve-{side}") or s["tone"]
        elif base in ("thigh", "shank") and s.get("region") == "leg-skin":
            s["region"], s["material"] = "trousers", "cotton-twill"
            s["tone"] = tone_of(doc, sc, view, f"shorts-leg-{side}") or s["tone"]
        if "regions" in s:
            s["regions"] = list(dict.fromkeys({"leg-skin": "trousers"}.get(x, x) for x in s["regions"]))
    drop_shapes(doc, lambda s: s["id"].startswith(("sleeve-hem-", "shorts-hem-")))
    for sc, lst, s in solids(doc):
        base = s["id"].rsplit("-", 1)[0]
        if lst == "body" and base in ("forearm", "upper-arm"):
            s["material"] = "cotton-jersey"
            s["tone"] = 0.62
        if lst == "body" and base in ("thigh", "shank", "patella"):
            s["material"] = "cotton-twill"
            s["tone"] = 0.3
        if lst == "thermal" and s.get("region") == "arm-skin" and s["id"].startswith(("th-forearm-", "th-upper-arm-")):
            s["region"] = "top"
            if sc == "50":
                s["layer"] = "top"
            s["material"] = "cotton-jersey"
        if lst == "thermal" and s.get("region") == "leg-skin":
            s["region"] = "trousers"
            if sc == "50":
                s["layer"] = "trousers"
            if "of" not in s:
                s["material"] = "cotton-twill"
    rename_region(doc, "arm-skin", "hand-skin")


def masses(doc):
    a = doc["asset"]
    total = 0.0
    for b in a["rig"]["bones"]:
        if "mass" in b and b["id"] != "pack":
            b["mass"]["kg"] = b["mass"]["kg"] * F[b["id"]] ** 3
            total += b["mass"]["kg"]
    k = (BODY_KG + CLOTHES_KG) / total
    for b in a["rig"]["bones"]:
        if "mass" in b:
            if b["id"] == "pack":
                b["mass"]["kg"] = PACK_KG
                b["mass"]["comX"], b["mass"]["comY"] = 0.09, 0.07
                b["mass"]["source"] = "authored-estimate"
                b["mass"]["reference"] = "dguv-2021"
            else:
                b["mass"]["kg"] = r(b["mass"]["kg"] * k, 3)
                b["mass"]["reference"] = "jensen-1986"
    a["dimensions"]["massKg"] = r(sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b), 2)


def bones_text(doc):
    for b in doc["asset"]["rig"]["bones"]:
        if b["id"] == "torso":
            b["title"] = T("Trunk with long-sleeved top", "Rumpf mit Langarmshirt")
        if b["id"] == "pack":
            b["title"] = T("School backpack", "Schulrucksack")
            b["joint"] = {"type": "revolute", "minDeg": -6, "maxDeg": 6}
        if b["id"].startswith("thigh-"):
            b["title"] = T(("Left" if b["id"].endswith("l") else "Right") + " thigh in trousers", ("Linker" if b["id"].endswith("l") else "Rechter") + " Oberschenkel in der Hose")


def materials(doc):
    for m in doc["asset"]["materials"]:
        if m["id"] == "skin":
            m["title"] = T("Child skin", "Kinderhaut")
        if m["id"] == "cotton-jersey":
            m["title"] = T("Cotton jersey, long-sleeved top", "Baumwolljersey, Langarmshirt")
        if m["id"] == "cotton-twill":
            m["title"] = T("Cotton twill trousers, navy", "Baumwoll-Twillhose, marineblau")
            m["solarAbsorptivity"] = 0.8
        if m["id"] == "polyester-pack":
            m.update({"title": T("Polyester backpack fabric, 600 denier, coated", "Polyester-Rucksackgewebe, 600 Denier, beschichtet"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "source": "authored-estimate"})
            m.pop("reference", None)
        if m["id"] == "cotton-underwear":
            m["title"] = T("Cotton vest and briefs", "Baumwollunterhemd und -slip")
        if m["id"] == "cotton-sock":
            m["title"] = T("Cotton socks", "Baumwollsocken")


def thermal(doc):
    a = doc["asset"]
    t = a["thermal"]
    t["environment"] = {"airTempC": 12, "meanRadiantTempC": 11, "skyTempC": 0, "groundTempC": 10, "relativeHumidity": 0.75, "windMS": 1.2, "windDirectionDeg": 180, "solarWM2": 180, "diffuseFraction": 0.5, "pressurePa": 101325}
    t["baseMet"] = 1.3
    area = dubois(BODY_KG, STATURE)
    t["body"].update({"massKg": BODY_KG, "skinAreaM2": r(area, 3), "basalMetabolismW": 50.2, "source": "authored-estimate", "reference": "schofield-1985"})
    clo = 0.155
    layer = lambda lid, title, mat, th, c, cap: {"id": lid, "title": title, "kind": "clothing", "material": mat, "thicknessM": th, "resistanceM2KW": r(c * clo, 5), "vapourResistanceM2PaW": r(c * clo / (0.0165 * 0.38), 2), "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "iso-9920-2007"}
    spec = {
        "face-skin": (0.05, 0.08, 1.0, T("Face and neck skin", "Haut von Gesicht und Hals")),
        "hair": (0.035, 0.02, 1.0, T("Short hair", "Kurzes Haar")),
        "hand-skin": (0.05, 0.04, 1.0, T("Bare hands", "Nackte Hände")),
        "top": (0.44, 0.55, 1.15, T("Vest and long-sleeved jersey top on trunk and arms", "Unterhemd und Langarmshirt aus Jersey an Rumpf und Armen")),
        "trousers": (0.32, 0.27, 1.12, T("Briefs and cotton trousers on hips and legs", "Slip und Baumwollhose an Hüfte und Beinen")),
        "socks": (0.02, 0.01, 1.05, T("Cotton socks above the shoes", "Baumwollsocken über den Schuhen")),
        "shoes": (0.085, 0.03, 1.1, T("Trainers", "Turnschuhe")),
    }
    for reg in t["regions"]:
        if reg["id"] in spec:
            sf, ms, fcl, title = spec[reg["id"]]
            reg["title"] = title
            reg["areaM2"] = r(sf * area * fcl, 4)
            reg["body"]["skinFraction"] = sf
            reg["body"]["metabolicShare"] = ms
        if reg["id"] == "hand-skin":
            reg["body"]["segment"] = "arm"
            reg["characteristicLengthM"] = 0.05
        if reg["id"] == "top":
            reg["layers"] = [layer("briefs", T("Cotton vest", "Baumwollunterhemd"), "cotton-underwear", 0.0006, 0.06, 70), layer("top", T("Long-sleeved cotton jersey top", "Langarmshirt aus Baumwolljersey"), "cotton-jersey", 0.0012, 0.2, 200)]
            reg["heatCapacityJM2K"] = 13270
            reg["characteristicLengthM"] = 0.22
        if reg["id"] == "trousers":
            reg["layers"] = [layer("trousers", T("Cotton twill trousers over briefs", "Baumwoll-Twillhose über dem Slip"), "cotton-twill", 0.0011, 0.22, 260)]
            reg["heatCapacityJM2K"] = 13260
            reg["characteristicLengthM"] = 0.08
            reg["skyViewFactor"] = 0.2
        if reg["id"] == "socks":
            reg["layers"][0]["title"] = T("Cotton socks", "Baumwollsocken")
        if reg["id"] == "shoes":
            reg["layers"][0]["title"] = T("Trainer with rubber sole", "Turnschuh mit Gummisohle")
        if reg["id"] == "backpack":
            reg["title"] = T("School backpack", "Schulrucksack")
            reg["areaM2"] = 0.42
            reg["characteristicLengthM"] = 0.3
            reg["ratedPowerW"] = 0
            reg["ratedPowerBasis"] = "passive"
    t["regions"] = [reg for reg in t["regions"] if reg["id"] != "leg-skin"]
    t["respiration"]["patterns"] = [
        {"id": "rest-child", "title": T("Child at rest, standing", "Kind in Ruhe, stehend"), "rateBpm": 20, "tidalVolumeL": 0.21, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "rowland-2005"},
        {"id": "walking-child", "title": T("Child walking with a backpack", "Kind beim Gehen mit Rucksack"), "rateBpm": 26, "tidalVolumeL": 0.48, "inhaleFraction": 0.42, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.48, "pauseAfterExhaleFraction": 0.08, "route": "nose", "source": "authored-estimate", "reference": "rowland-2005"},
        {"id": "hitch-effort", "title": T("Short effort of hitching the backpack", "Kurze Anstrengung beim Hochziehen des Rucksacks"), "rateBpm": 24, "tidalVolumeL": 0.35, "inhaleFraction": 0.45, "pauseAfterInhaleFraction": 0.06, "exhaleFraction": 0.42, "pauseAfterExhaleFraction": 0.07, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "rowland-2005"},
    ]
    t["respiration"]["motion"] = [{"bone": "torso", "angleDeg": 0.7, "dy": 0.0022}]
    for e in t["emitters"]:
        if e["kind"] == "breath":
            e["diameterM"] = r(e["diameterM"] * 1.03, 4)
        if e["id"] == "body-plume":
            e["region"] = "top"
            e["diameterM"] = 0.17
    t["assumptions"] = [
        T(f"Skin area {area:.2f} m2 after DuBois and DuBois (1916) for an eight-year-old of {BODY_KG} kg and {STATURE} m (WHO 2007 growth reference, medians of both sexes); the {doc['asset']['dimensions']['massKg']} kg of the asset add {CLOTHES_KG} kg of clothes and shoes and the {PACK_KG} kg loaded backpack. Skin fractions authored for a school child: face and neck 0.05, scalp 0.035, hands 0.05, trunk and arms 0.44, hips and legs 0.32, ankles 0.02, feet 0.085.",
          f"Hautfläche {area:.2f} m2 nach DuBois und DuBois (1916) für ein achtjähriges Kind mit {BODY_KG} kg und {STATURE} m (WHO-Wachstumsreferenz 2007, Mediane beider Geschlechter); die {doc['asset']['dimensions']['massKg']} kg des Objekts enthalten {CLOTHES_KG} kg Kleidung und Schuhe und den {PACK_KG} kg schweren beladenen Rucksack. Hautanteile für ein Schulkind geschätzt: Gesicht und Hals 0,05, Kopfhaut 0,035, Hände 0,05, Rumpf und Arme 0,44, Hüfte und Beine 0,32, Knöchel 0,02, Füße 0,085."),
        T("Autumn school clothing of about 0.5 clo: vest 0.06, long-sleeved jersey top 0.20, briefs and cotton trousers 0.22, socks 0.02 and trainers 0.02 clo (ISO 9920 garment tables, authored estimates); vapour resistance R/(0.0165 K/Pa x 0.38). The reference morning is 12 degrees C, light wind and a low autumn sun.",
          "Herbstliche Schulkleidung von etwa 0,5 clo: Unterhemd 0,06, Langarmshirt aus Jersey 0,20, Slip und Baumwollhose 0,22, Socken 0,02 und Turnschuhe 0,02 clo (Kleidungstabellen ISO 9920, Schätzwerte); Dampfwiderstand R/(0,0165 K/Pa x 0,38). Der Referenzmorgen hat 12 Grad C, leichten Wind und eine tief stehende Herbstsonne."),
        T("Activity: standing 1.3 met, walking at 1.05 m/s with a backpack of about 10 % of the body mass 3.0 met (ISO 8996 walking rates referred to the child's surface, guided by the youth compendium of Butte et al. 2018), hitching the backpack 1.8 met; the basal rate 50 W is the mean of the Schofield (1985) equations for girls and boys of 3 to 10 years.",
          "Aktivität: Stehen 1,3 met, Gehen mit 1,05 m/s und einem Rucksack von etwa 10 % der Körpermasse 3,0 met (Gehraten nach ISO 8996 auf die Kinderoberfläche bezogen, orientiert am Jugendkompendium von Butte et al. 2018), Hochziehen des Rucksacks 1,8 met; der Grundumsatz von 50 W ist das Mittel der Schofield-Gleichungen (1985) für Mädchen und Jungen von 3 bis 10 Jahren."),
        T("Child breathing: about 8 mL/kg at rest (0.21 L at 20 per minute), walking 26 per minute with 0.48 L (about 12.5 L/min); all authored estimates guided by general paediatric exercise physiology (Rowland 2005), not measured data.",
          "Kinderatmung: etwa 8 mL/kg in Ruhe (0,21 L bei 20 je Minute), Gehen 26 je Minute mit 0,48 L (etwa 12,5 L/min); alles Schätzwerte nach allgemeiner Kinder-Belastungsphysiologie (Rowland 2005), keine Messwerte."),
    ]


REGIONS = ["face-skin", "hair", "hand-skin", "top", "trousers", "socks", "shoes", "backpack"]


def binding(met, breathing, sun=0.5, plume=0.6, ground=None):
    regions = {rid: {"exposedFraction": 1, "sunlitFraction": sun} for rid in REGIONS}
    regions["top"]["exposedFraction"] = 0.78
    regions["backpack"]["sunlitFraction"] = round(sun * 0.7, 2)
    if ground is not None:
        regions["shoes"]["contact"] = {"fraction": ground, "tempC": 10}
    return {"activityMet": met, "breathing": breathing, "regions": regions, "emitters": {"body-plume": plume}}


STAND = {"thigh-l": {"angleDeg": 2}, "thigh-r": {"angleDeg": 2}, "shank-l": {"angleDeg": -4}, "shank-r": {"angleDeg": -4}, "foot-l": {"angleDeg": 2}, "foot-r": {"angleDeg": 2}, "toe-l": {"angleDeg": 0}, "toe-r": {"angleDeg": 0}}


def poses_and_clips(doc):
    a = doc["asset"]
    cyc = solve_cycle(a, dict(GAIT), N_KEYS)
    print(f"[DEBUG] gait h0 {cyc['h0']:.4f} drop {cyc['drop']:.4f} clearance {cyc['clearance']:.4f}")
    upper = lambda k: {"neck": {"angleDeg": 3}, "head": {"angleDeg": 2}, "hand-l": {"angleDeg": 0}, "hand-r": {"angleDeg": 0}, "pack": {"angleDeg": 0}}
    keyed = [dict(k["bones"], **upper(k)) for k in cyc["keys"]]
    at = lambda frac: keyed[round(frac * N_KEYS) % N_KEYS]
    walk_contacts = lambda frac: [f"{an}-{side}" for side in ("l", "r") for an in cyc["keys"][round(frac * N_KEYS) % N_KEYS]["contacts"][side]]
    poses = [
        {"id": "walking-midstance", "title": T("Walking, left midstance", "Gehen, linke Standphasenmitte"),
         "description": T("The left foot flat on the ground under the hip, the right leg swinging through with the knee bent, the trunk leaning 5 degrees forward against the pull of the backpack, the arms swinging in opposition.",
                          "Der linke Fuß steht flach unter der Hüfte, das rechte Bein schwingt mit gebeugtem Knie durch, der Rumpf neigt sich 5 Grad nach vorn gegen den Zug des Rucksacks, die Arme schwingen gegenläufig."),
         "bones": at(0.3), "ground": "auto", "contacts": walk_contacts(0.3), "thermal": binding(3.0, "walking-child", plume=0.8), "loads": []},
        {"id": "walking-heel-strike", "title": T("Walking, left heel strike", "Gehen, linker Fersenaufsatz"),
         "description": T("The left heel touches down a step ahead while the right forefoot still pushes off behind: the short double support of a school child's walk.",
                          "Die linke Ferse setzt einen Schritt voraus auf, während der rechte Vorfuß hinten noch abdrückt: die kurze Doppelstandphase im Gang eines Schulkinds."),
         "bones": at(0.0), "ground": "auto", "contacts": walk_contacts(0.0), "thermal": binding(3.0, "walking-child", plume=0.8), "loads": []},
        {"id": "holding-straps", "title": T("Standing, holding the straps", "Stehend, die Gurte haltend"),
         "description": T("Standing on both feet, both hands gripping the shoulder straps in front of the chest, elbows bent, shoulders slightly drawn up under the weight of the backpack.",
                          "Auf beiden Füßen stehend, beide Hände greifen die Schultergurte vor der Brust, Ellbogen gebeugt, Schultern unter dem Gewicht des Rucksacks leicht hochgezogen."),
         "bones": dict(STAND, **{"torso": {"angleDeg": -3}, "neck": {"angleDeg": 2}, "head": {"angleDeg": 1}, "clavicle-l": {"angleDeg": 6}, "clavicle-r": {"angleDeg": 6}, "arm-l": {"angleDeg": 18}, "arm-r": {"angleDeg": 18}, "forearm-l": {"angleDeg": 118}, "forearm-r": {"angleDeg": 118}, "hand-l": {"angleDeg": -24}, "hand-r": {"angleDeg": -24}, "pack": {"angleDeg": 0}}),
         "ground": "auto", "contacts": ["heel-l", "ball-l", "heel-r", "ball-r"], "thermal": binding(1.3, "rest-child", ground=0.3), "loads": []},
        {"id": "waiting-at-kerb", "title": T("Waiting at the kerb", "Am Bordstein wartend"),
         "description": T("Standing still at the kerb, weight on both feet, arms hanging, head raised to watch the traffic light, the backpack pulling the trunk slightly back.",
                          "Still am Bordstein stehend, Gewicht auf beiden Füßen, Arme hängend, Kopf zur Ampel erhoben, der Rucksack zieht den Rumpf leicht nach hinten."),
         "bones": dict(STAND, **{"torso": {"angleDeg": -2}, "neck": {"angleDeg": 4}, "head": {"angleDeg": 6}, "arm-l": {"angleDeg": 3}, "arm-r": {"angleDeg": 5}, "forearm-l": {"angleDeg": 10}, "forearm-r": {"angleDeg": 12}, "hand-l": {"angleDeg": 0}, "hand-r": {"angleDeg": 0}, "pack": {"angleDeg": 0}}),
         "ground": "auto", "contacts": ["heel-l", "ball-l", "heel-r", "ball-r"], "thermal": binding(1.3, "rest-child", ground=0.3), "loads": []},
    ]
    for p in poses:
        for bid in ("clavicle-l", "clavicle-r"):
            p["bones"].setdefault(bid, {"angleDeg": 0})
    a["poses"] = poses
    Tc = GAIT["T"]
    spans = contact_spans(GAIT)
    keys = [{"t": k["t"], "pose": "walking-midstance", "ease": "smooth", "bones": dict(keyed[i], **{"clavicle-l": {"angleDeg": 0}, "clavicle-r": {"angleDeg": 0}, "pack": {"angleDeg": r(0.8 * math.cos(4 * math.pi * k["t"] / Tc), 3)}})} for i, k in enumerate(cyc["keys"])]
    hs, ff, ho, to = 0.0, GAIT["footFlat"] * Tc, GAIT["heelOff"] * Tc, GAIT["toeOff"] * Tc
    events = []
    for side, sh, en, de in (("l", 0.0, "Left", "Linke"), ("r", Tc / 2, "Right", "Rechte")):
        events.append({"t": r((hs + sh) % Tc, 4), "kind": "contact-on", "title": T(f"{en} heel strike", f"{de} Ferse setzt auf"), "target": f"heel-{side}"})
        events.append({"t": r((to + sh) % Tc, 4), "kind": "contact-off", "title": T(f"{en} toe-off", f"{de}r Abdruck" if side == "l" else "Rechter Abdruck"), "target": f"ball-{side}"})
    events.sort(key=lambda e: e["t"])
    walk = {"id": "walk-to-school", "title": T("Walking to school", "Gehen zur Schule"),
            "description": T(f"Level walking at {GAIT['speed']:.2f} m/s with {120 / Tc:.0f} steps per minute and a stride of {GAIT['stride']:.2f} m: heel strike, foot flat, heel rise and toe-off on each foot with a short double support, the knee bending about 25 degrees after heel strike and 60 degrees in swing, the arms swinging in opposition and the backpack bobbing with the hips. Joint angles from two-link inverse kinematics of smooth foot paths of the grown rig, so the planted feet do not slide.",
                             f"Gehen in der Ebene mit {GAIT['speed']:.2f} m/s, {120 / Tc:.0f} Schritten pro Minute und {GAIT['stride']:.2f} m Doppelschrittlänge: Fersenaufsatz, flacher Fuß, Fersenhebung und Abdruck an jedem Fuß mit kurzer Doppelstandphase, das Knie beugt sich nach dem Aufsatz um etwa 25 Grad und im Schwung um 60 Grad, die Arme schwingen gegenläufig und der Rucksack wippt mit der Hüfte. Gelenkwinkel aus zweigliedriger inverser Kinematik glatter Fußbahnen des gewachsenen Rigs, sodass die aufgesetzten Füße nicht gleiten."),
            "durationS": Tc, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": GAIT["stride"], "keys": keys, "contacts": spans,
            "phases": [
                {"id": "double-support-left-lead", "fromS": 0.0, "toS": r(0.1 * Tc, 4), "title": T("Double support, left foot leading", "Doppelstandphase, linker Fuß vorn")},
                {"id": "left-single-support", "fromS": r(0.1 * Tc, 4), "toS": r(0.5 * Tc, 4), "title": T("Left single support, right leg swinging", "Linke Einzelstandphase, rechtes Bein schwingt")},
                {"id": "double-support-right-lead", "fromS": r(0.5 * Tc, 4), "toS": r(0.6 * Tc, 4), "title": T("Double support, right foot leading", "Doppelstandphase, rechter Fuß vorn")},
                {"id": "right-single-support", "fromS": r(0.6 * Tc, 4), "toS": r(Tc, 4), "title": T("Right single support, left leg swinging", "Rechte Einzelstandphase, linkes Bein schwingt")},
            ],
            "events": events,
            "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 3.0}, {"t": Tc, "v": 3.0}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": Tc, "v": 1}]}]}
    kerb = a["poses"][3]["bones"]
    straps = a["poses"][2]["bones"]
    hitch_peak = dict(copy.deepcopy(straps), **{"clavicle-l": {"angleDeg": 16}, "clavicle-r": {"angleDeg": 16}, "torso": {"angleDeg": -6}, "pack": {"angleDeg": 3}, "thigh-l": {"angleDeg": 6}, "thigh-r": {"angleDeg": 6}, "shank-l": {"angleDeg": -12}, "shank-r": {"angleDeg": -12}, "foot-l": {"angleDeg": 6}, "foot-r": {"angleDeg": 6}})
    hitch = {"id": "hitch-backpack", "title": T("Hitching the backpack up", "Rucksack hochziehen"),
             "description": T("From waiting at the kerb the child grips both straps, bobs down at the knees and shrugs the shoulders to hitch the heavy backpack higher, then lets the arms down again.",
                              "Vom Warten am Bordstein greift das Kind beide Gurte, federt in den Knien und zieht die Schultern hoch, um den schweren Rucksack höher zu rücken, und lässt die Arme dann wieder sinken."),
             "durationS": 2.4, "fps": 30, "loop": False, "rootMotion": "plant",
             "keys": [{"t": 0, "pose": "waiting-at-kerb", "ease": "smooth"}, {"t": 0.7, "pose": "holding-straps", "ease": "smooth"}, {"t": 1.15, "pose": "holding-straps", "ease": "smooth", "bones": hitch_peak}, {"t": 1.6, "pose": "holding-straps", "ease": "smooth"}, {"t": 2.4, "pose": "waiting-at-kerb", "ease": "smooth"}],
             "contacts": [{"anchor": x, "fromS": 0, "toS": 2.4} for x in ("heel-l", "ball-l", "heel-r", "ball-r")],
             "phases": [{"id": "grip", "fromS": 0, "toS": 0.7, "title": T("Gripping the straps", "Gurte greifen")}, {"id": "hitch", "fromS": 0.7, "toS": 1.6, "title": T("Bobbing and shrugging", "Federn und Schultern hochziehen")}, {"id": "release", "fromS": 1.6, "toS": 2.4, "title": T("Letting go", "Loslassen")}],
             "events": [{"t": 1.15, "kind": "motion", "title": T("Backpack hitched up", "Rucksack hochgerückt"), "target": "pack"}],
             "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 1.15, "v": 1.8}, {"t": 2.4, "v": 1.3}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 0.7, "v": 2}, {"t": 1.9, "v": 0}]}]}
    sway = lambda d: dict(copy.deepcopy(kerb), **{"torso": {"angleDeg": -2 + d}, "head": {"angleDeg": 6 - d}, "arm-l": {"angleDeg": 3 - d}, "arm-r": {"angleDeg": 5 + d}})
    idle = {"id": "idle-breathing", "title": T("Waiting and breathing", "Warten und atmen"),
            "description": T("Waiting at the kerb for the green light: quiet breathing at 20 breaths per minute with a slow sway of the trunk and the arms.",
                             "Am Bordstein auf Grün warten: ruhiges Atmen mit 20 Atemzügen je Minute und langsamem Pendeln von Rumpf und Armen."),
            "durationS": 6.0, "fps": 15, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "waiting-at-kerb", "ease": "smooth", "bones": sway(0)}, {"t": 1.5, "pose": "waiting-at-kerb", "ease": "smooth", "bones": sway(0.8)}, {"t": 3.0, "pose": "waiting-at-kerb", "ease": "smooth", "bones": sway(0)}, {"t": 4.5, "pose": "waiting-at-kerb", "ease": "smooth", "bones": sway(-0.8)}],
            "contacts": [{"anchor": x, "fromS": 0, "toS": 6.0} for x in ("heel-l", "ball-l", "heel-r", "ball-r")],
            "phases": [{"id": "waiting", "fromS": 0, "toS": 6.0, "title": T("Waiting", "Warten")}], "events": [],
            "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 6.0, "v": 1.3}]}, {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 6.0, "v": 0}]}]}
    a["clips"] = [walk, hitch, idle]
    a["presentation"] = {"defaultPose": "walking-midstance", "defaultClip": "walk-to-school", "defaultScale": 100}


def hand_at_500(doc):
    """At 1:500 the hand is merged into the sleeved forearm: the drawing lists both regions, the thermal surface is cut at the wrist."""
    a = doc["asset"]
    for view in ("elevation", "front", "plan"):
        for s in a["representations"]["500"]["geometry"][view]:
            if s["id"] in ("forearm-l", "forearm-r"):
                s.pop("region", None)
                s["regions"] = ["top", "hand-skin"]
    th = a["volumes"]["500"]["thermal"]
    out = []
    for s in th:
        out.append(s)
        if s["id"] in ("th-forearm-l", "th-forearm-r"):
            wrist = next(b["lengthM"] for b in a["rig"]["bones"] if b["id"] == s["bone"])
            secs = s["sections"]
            cut = min(range(len(secs)), key=lambda i: abs(secs[i]["at"][0] - wrist))
            hand = copy.deepcopy(s)
            hand["id"] = s["id"].replace("forearm", "hand")
            hand["region"], hand["material"], hand["tone"] = "hand-skin", "skin", 0.84
            hand["sections"] = copy.deepcopy(secs[cut:])
            s["sections"] = secs[: cut + 1]
            out.append(hand)
    a["volumes"]["500"]["thermal"] = out


def texts(doc):
    import texts_schoolchild as X
    a = doc["asset"]
    a["title"], a["summary"], a["tags"] = X.TITLE, X.SUMMARY, X.TAGS
    prov = a["provenance"]
    prov["author"] = "Semio staffage human worker HW-05"
    prov["created"] = "2026-11-22"
    prov["notes"] = X.NOTES
    have = {ref["id"] for ref in prov["references"]}
    prov["references"] += [ref for ref in X.NEW_REFERENCES if ref["id"] not in have]
    for sc, dec in X.DECISIONS.items():
        a["representations"][sc]["decisions"] = dec
    for sc in ("50", "100", "200", "500"):
        vd = a["volumes"][sc]["decisions"]
        replace_text(vd[2], X.STRUCTURE_PAIRS)
        a["volumes"][sc]["decisions"] = [X.VOLUME_BODY[sc], X.VOLUME_THERMAL[sc], vd[2]]
    a["structure"]["assumptions"] = X.STRUCTURE_ASSUMPTIONS
    sym = a["symbols"][0]
    sym["id"] = "walking-schoolchild-marker"
    sym["title"], sym["description"] = X.SYMBOL["title"], X.SYMBOL["description"]


def build():
    doc = rescale(load("running-child"), F)
    a = doc["asset"]
    a["id"] = SLUG
    geometry(doc)
    masses(doc)
    bones_text(doc)
    materials(doc)
    thermal(doc)
    poses_and_clips(doc)
    hand_at_500(doc)
    texts(doc)
    return doc


if __name__ == "__main__":
    doc = build()
    print("[DEBUG] dims", set_dims(doc, SLUG))
    found = feature_loop(doc, SLUG)
    summarize(found)
