from io_ import *
doc = load("parent-with-stroller"); a = doc["asset"]; th = a["thermal"]
refs = {r["id"] for r in a["provenance"]["references"]}
for rid, cit in [("schofield-1985", "Schofield, W. N. (1985). Predicting basal metabolic rate, new standards and review of previous work. Human Nutrition: Clinical Nutrition 39C (Suppl. 1), 5-41."),
                 ("mosteller-1987", "Mosteller, R. D. (1987). Simplified calculation of body-surface area. New England Journal of Medicine 317(17), 1098."),
                 ("fleming-2011", "Fleming, S. et al. (2011). Normal ranges of heart rate and respiratory rate in children from birth to 18 years of age: a systematic review of observational studies. The Lancet 377(9770), 1011-1018.")]:
    if rid not in refs: a["provenance"]["references"].append({"id": rid, "citation": cit})
a["anchors"] = [x for x in a["anchors"] if x["id"] != "child-mouth"]
a["anchors"] += [
    {"id": "child-mouth", "bone": "child", "x": 0.06, "y": 0.312, "kind": "emitter", "title": T2("Toddler's mouth", "Mund des Kleinkinds")},
    {"id": "child-nose", "bone": "child", "x": 0.068, "y": 0.337, "kind": "emitter", "title": T2("Toddler's nostrils", "Nasenlöcher des Kleinkinds")},
]
th["emitters"] = [e for e in th["emitters"] if e["id"] != "child-breath"]
th["emitters"] += [
    {"id": "child-nose-breath", "kind": "breath", "title": T2("Exhaled breath from the toddler's nostrils", "Ausatemluft aus den Nasenlöchern des Kleinkinds"), "anchor": "child-nose", "directionDeg": -30, "diameterM": 0.006, "source": "authored-estimate", "reference": "gupta-2010"},
    {"id": "child-mouth-breath", "kind": "breath", "title": T2("Exhaled breath from the toddler's mouth", "Ausatemluft aus dem Mund des Kleinkinds"), "anchor": "child-mouth", "directionDeg": -15, "diameterM": 0.008, "source": "authored-estimate", "reference": "gupta-2010"},
]
th["operating"] = []
for r in th["regions"]:
    if r["id"] == "child-skin":
        r.update(model="body", body={"segment": "head", "metabolicShare": 0.16, "skinFraction": 0.07, "subject": "toddler"}, title=T2("Toddler's face and hands", "Gesicht und Hände des Kleinkinds"))
        r.pop("operating", None)
    if r["id"] == "child-clothing":
        r.update(model="body", body={"segment": "trunk", "metabolicShare": 0.84, "skinFraction": 0.93, "subject": "toddler"})
        r.pop("operating", None)
th["passengers"] = [{
    "id": "toddler",
    "title": T2("Seated toddler (about 12 months)", "Sitzendes Kleinkind (etwa 12 Monate)"),
    "physiology": {"species": "human", "massKg": 10, "skinAreaM2": 0.45, "basalMetabolismW": 27, "metWM2": 58.15, "coreSetPointC": 37.0, "skinSetPointC": 34.5,
                   "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 6.3, "source": "authored-estimate", "reference": "schofield-1985"},
    "respiration": {"noseEmitter": "child-nose-breath", "mouthEmitter": "child-mouth-breath", "exhaledTempC": 33.5, "exhaledRelativeHumidity": 0.92,
        "patterns": [
            {"id": "seated", "title": T2("Awake, seated", "Wach, sitzend"), "rateBpm": 25, "tidalVolumeL": 0.075, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.05, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.1, "route": "nose", "source": "authored-estimate", "reference": "fleming-2011"},
            {"id": "excited", "title": T2("Excited, reaching and calling", "Aufgeregt, greifend und rufend"), "rateBpm": 34, "tidalVolumeL": 0.085, "inhaleFraction": 0.42, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.06, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "fleming-2011"}],
        "motion": [], "source": "authored-estimate", "reference": "fleming-2011"}}]
bind = {"pushing-stroller": (1.3, "seated"), "pausing-with-stroller": (1.2, "seated"), "checking-child": (1.6, "excited"), "kerb-tilt": (1.5, "excited")}
for p in a["poses"]:
    t = p["thermal"]; t.pop("operating", None)
    t["emitters"].pop("child-breath", None)
    met, br = bind.get(p["id"], (1.3, "seated"))
    t["passengers"] = {"toddler": {"activityMet": met, "breathing": br}}
    t["emitters"] = {k: v for k, v in t["emitters"].items()}
for c in a["clips"]:
    c["tracks"] = [tr for tr in c["tracks"] if tr["target"] != "emitter:child-breath"]
    c.setdefault("tracks", [])
for asm in th["assumptions"]:
    if "second warm body" in asm["en"] or "continuous-equivalent" in asm["en"]:
        pass
th["assumptions"] = [x for x in th["assumptions"] if "toddler" not in x["en"].lower() or "Pushing" in x["en"]] 
th["assumptions"].insert(0, T2(
 "The toddler (10 kg, 12 months, Mosteller area 0.45 m2) is a passenger with its own physiology: basal metabolism 27 W (Schofield 1985, boys under 3 years), 1.2-1.6 met, 25-34 breaths per minute at 75-85 mL (about 8 mL/kg, estimate; rate range Fleming 2011), nose breathing at rest and nose-and-mouth when excited. Its face and hands (0.03 m2) and its padded snowsuit (0.42 m2, about 1.1 clo) are body regions with the subject toddler.",
 "Das Kleinkind (10 kg, 12 Monate, Mosteller-Fläche 0,45 m2) ist ein Passagier mit eigener Physiologie: Grundumsatz 27 W (Schofield 1985, Jungen unter 3 Jahren), 1,2 bis 1,6 met, 25 bis 34 Atemzüge pro Minute mit 75 bis 85 mL (etwa 8 mL/kg, Schätzwert; Frequenzbereich Fleming 2011), Nasenatmung in Ruhe und Nasen-Mund-Atmung bei Erregung. Gesicht und Hände (0,03 m2) und der wattierte Schneeanzug (0,42 m2, etwa 1,1 clo) sind Körperregionen mit dem Subjekt toddler."))
a["presentation"]["defaultScale"] = 100
save("parent-with-stroller", doc)
print("ok")
