"""📝️ HW-09 physical scenario, provenance and per-scale decisions of `person-pushing-bicycle` (design input of build_person_pushing_bicycle.py).

Anthropometry: stature 1.80 m, mass 75 kg, sex-neutral build (DIN 33402-2 adults 18–65: 50th percentile men 1.75 m, women 1.63 m;
the figure is a tall adult near the male 75th percentile), DuBois area 0.007184 · 75^0.425 · 180^0.725 = 1.94 m², basal metabolism the
sex-neutral mean of the Mifflin–St Jeor equations at 35 years, 1622 kcal/day = 78.6 W.
Activity: walking at 1.1 m/s on a firm surface is 3.0 MET (Ainsworth 2011, code 17170, walking 2.5 mph); pushing a 19 kg bicycle adds
the rolling resistance (Crr 0.007 → 1.3 N, 1.4 W external work); standing and holding the bicycle 1.5 met (ISO 8996 standing, light
arm work); lifting the rear onto the centre stand 3.5 met.
"""
import copy
import re
from hw09lib import T_, merge_refs
from e_scooter_texts import REFS

DUBOIS = 1.94
DROP = ("stroller", "Kinderwagen", "toddler", "Kleinkind", "child", "Kind ", "Kindes", "passenger", "Passagier", "caster", "Lenkroll", "canopy", "Verdeck", "push bar", "Schiebebügel", "Schubbügel", "pushchair", "Buggy", "snowsuit", "Schneeanzug", "seat sling", "Sitzbespannung", "basket", "Korb", "wheel", "Rad ", "Räder", "Rädern", "tyre", "Reifen", "hood", "bumper", "Schutzbügel", "kerb", "Bordstein", "web", "Scheibe")


def strip(text):
    """🧹️ Keeps the sentences of a parent-with-stroller decision that describe the adult."""
    parts = re.split(r"(?<=[.;])\s+", text)
    kept = [p for p in parts if not any(w in p for w in DROP)]
    return " ".join(kept)


def finish(asset, par, bic, person, bike, K, MASS):
    asset["summary"] = T_(
        "A 1.80 m adult in a quilted jacket, scarf, jeans and sneakers walks on the left side of a Dutch city bicycle and pushes it by the handlebar: the left hand on the left grip, the right hand on the bar beside the stem. The bicycle is the root of the rig and rolls without slip while the walker's planted feet drive the gait; the figure also stands holding the bicycle and lifts it onto its centre stand.",
        "Eine 1,80 m große erwachsene Person in Steppjacke, Schal, Jeans und Sneakern geht links neben einem Hollandrad und schiebt es am Lenker: die linke Hand am linken Griff, die rechte Hand am Lenker neben dem Vorbau. Das Fahrrad ist die Wurzel des Modells und rollt ohne Schlupf, während die aufgesetzten Füße den Gang bestimmen; die Figur steht auch mit dem Rad und bockt es auf den Mittelständer.",
    )
    asset["provenance"] = {
        "author": "HW-09, Staffage Human Collection Wave (urban mobility)",
        "method": "handcrafted",
        "created": "2026-11-22",
        "references": merge_refs(par["provenance"]["references"], bic["provenance"]["references"], REFS),
        "notes": T_(
            "Walker derived from the parent of parent-with-stroller (rig, outlines, volumes and its planted-foot gait keys) without stroller and toddler, scaled isotropically by 1.03 (1.75 m to 1.80 m) with segment masses rescaled to 75 kg; hung from the city-bicycle frame through the walk-lift and walk-shift prismatic pair 0.34 m to the left of the bicycle's centre plane. Arms solved by damped least-squares inverse kinematics on the library's evaluate so the hands stay on the bar in every gait key; 21 strides of 1.03 m make ten wheel turns (0.2 % slip), so the walk loop rolls without slip. Generator: T/🧰️scripts/HW-09/build_person_pushing_bicycle.py and poses_person_pushing_bicycle.ts.",
            "Die gehende Person ist aus dem Elternteil von parent-with-stroller abgeleitet (Rig, Umrisse, Volumen und seine Gangschlüssel mit aufgesetzten Füßen), ohne Kinderwagen und Kleinkind, isotrop mit 1,03 skaliert (1,75 m auf 1,80 m), Segmentmassen auf 75 kg umgerechnet; sie hängt über das Schiebepaar walk-lift und walk-shift 0,34 m links der Mittelebene am Rahmen des city-bicycle. Die Arme sind mit gedämpfter inverser Kinematik der kleinsten Quadrate auf evaluate der Bibliothek gelöst, sodass die Hände in jedem Gangschlüssel am Lenker bleiben; 21 Schritte zu 1,03 m ergeben zehn Radumdrehungen (0,2 % Schlupf), daher rollt die Gehschleife ohne Schlupf. Generator: T/🧰️scripts/HW-09/build_person_pushing_bicycle.py und poses_person_pushing_bicycle.ts.",
        ),
    }
    for r in person["regions"]:
        r["areaM2"] = round(r["areaM2"] * DUBOIS / 1.83 / (K * K), 4)
    th_p, th_b = par["thermal"], bic["thermal"]
    resp = copy.deepcopy(th_p["respiration"])
    resp["patterns"] = [p for p in resp["patterns"] if p["id"] in ("pausing", "pushing")] + [
        {"id": "lifting", "title": T_("Lifting the bicycle onto its stand", "Fahrrad auf den Ständer heben"), "rateBpm": 20, "tidalVolumeL": 1.1, "inhaleFraction": 0.42, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.06, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
    ]
    body = copy.deepcopy(th_p["body"])
    body.update(massKg=MASS, skinAreaM2=DUBOIS, basalMetabolismW=78.6, source="literature", reference="mifflin-1990")
    asset["thermal"] = {
        "environment": {"airTempC": 12, "meanRadiantTempC": 13, "skyTempC": 2, "groundTempC": 14, "relativeHumidity": 0.7, "windMS": 1.5, "windDirectionDeg": 180, "solarWM2": 200, "diffuseFraction": 0.6, "pressurePa": 101325},
        "baseMet": 1.5,
        "body": body,
        "respiration": resp,
        "passengers": [],
        "operating": copy.deepcopy(th_b["operating"]),
        "regions": person["regions"] + bike["regions"],
        "emitters": [copy.deepcopy(e) for e in th_p["emitters"] if e["id"] in ("breath-nose", "breath-mouth", "plume")],
        "assumptions": [
            T_("The walker's regions sum to the DuBois area of 1.94 m2 (75 kg, 180 cm); clothed regions carry the area factor 1 + 0.28 clo. The bicycle surfaces are those of the city-bicycle asset (frame tubes, steering, mudguards, tyres, rims and spokes, saddle, drivetrain, hub brakes, lamps).", "Die Regionen der gehenden Person ergeben die DuBois-Fläche von 1,94 m2 (75 kg, 180 cm); bekleidete Regionen tragen den Flächenfaktor 1 + 0,28 clo. Die Fahrradflächen sind die des Assets city-bicycle (Rahmenrohre, Lenkung, Schutzbleche, Reifen, Felgen und Speichen, Sattel, Antrieb, Nabenbremsen, Lampen)."),
            th_p["assumptions"][2],
            T_("Walking at 1.1 m/s is 3.0 met (Compendium 2011, code 17170); pushing the 19 kg bicycle adds its rolling resistance (Crr 0.007, 1.3 N, 1.4 W external work); standing beside it and holding the bar is 1.5 met, lifting the rear onto the centre stand 3.5 met.", "Gehen mit 1,1 m/s entspricht 3,0 met (Compendium 2011, Code 17170); das Schieben des 19-kg-Fahrrads kommt mit seinem Rollwiderstand hinzu (Crr 0,007, 1,3 N, 1,4 W äußere Arbeit); Stehen daneben mit der Hand am Lenker 1,5 met, das Heben des Hecks auf den Mittelständer 3,5 met."),
            T_("Breathing follows the metabolic rate (ventilation about 25 times the oxygen uptake): 11 L/min standing, 20 L/min walking, 22 L/min lifting; the reference case is a cool, overcast autumn day (12 C, 70 % relative humidity).", "Die Atmung folgt dem Stoffwechsel (Ventilation etwa das 25-Fache der Sauerstoffaufnahme): 11 L/min im Stehen, 20 L/min beim Gehen, 22 L/min beim Heben; der Bezugsfall ist ein kühler, bedeckter Herbsttag (12 C, 70 % relative Feuchte)."),
            T_("Total power: the walker's metabolic heat plus the 3 W dynamo lamps of the bicycle (nameplate, electrical input); the bicycle has no motor.", "Gesamtleistung: die Stoffwechselwärme der gehenden Person plus die 3-W-Dynamolampen des Fahrrads (Typenschild, Eingangsleistung); das Fahrrad hat keinen Motor."),
        ] + [a for a in th_b["assumptions"] if "rider is not part" not in a["en"]],
    }
    asset["structure"] = {
        "gravityMS2": 9.80665, "dynamics": "inertial", "supports": [],
        "assumptions": [
            T_("The bicycle stands on its two tyre patches (on the centre stand: front tyre and stand feet); the walker stands on the ground with the planted soles and holds the bar, so the solver closes the loop through the walk-lift joint and shares the loads between feet and tyres by equal stiffness.", "Das Fahrrad steht auf seinen zwei Reifenaufstandsflächen (auf dem Mittelständer: Vorderreifen und Ständerfüße); die gehende Person steht mit den aufgesetzten Sohlen auf dem Boden und hält den Lenker, daher schließt der Löser die Kette über das Gelenk walk-lift und teilt die Lasten zwischen Füßen und Reifen nach gleicher Steifigkeit."),
            T_("The bicycle is upright; its lateral balance through the hands is not modelled in the elevation. Steering stays straight; the wheels roll without slip, theta = -x / r with r = 0.345 m.", "Das Fahrrad steht aufrecht; sein seitliches Gleichgewicht über die Hände wird in der Ansicht nicht modelliert. Die Lenkung bleibt gerade; die Räder rollen ohne Schlupf, theta = -x / r mit r = 0,345 m."),
            T_("Segment masses of the walker follow de Leva (1996), rescaled from the 68 kg parent figure to 75 kg; the bicycle (19 kg) keeps the masses of the city-bicycle asset.", "Die Segmentmassen der gehenden Person folgen de Leva (1996), von der 68-kg-Elternfigur auf 75 kg umgerechnet; das Fahrrad (19 kg) behält die Massen des Assets city-bicycle."),
        ],
        "clinical": False,
    }
    for s, rep in asset["representations"].items():
        c = par["representations"][s]["decisions"]
        v = bic["representations"][s]["decisions"] if bic["representations"][s]["status"] == "authored" else None
        d = {k: T_("Walker: " + (strip(c[k]["en"]) or "as the source figure at this scale."), "Person: " + (strip(c[k]["de"]) or "wie die Ausgangsfigur in diesem Maßstab.")) for k in c}
        if v is not None:
            for k in d:
                d[k] = T_(d[k]["en"] + " Bicycle: " + v[k]["en"], d[k]["de"] + " Fahrrad: " + v[k]["de"])
        else:
            d["silhouette"] = T_(d["silhouette"]["en"] + " Bicycle: drawn new for 1:500 from the body shapes of its 1:200 drawing only, the frame tubes, fork, bar, mudguards, saddle, chain case and the two tyre rings, on their own bones so the wheels and the stand stay articulated.", d["silhouette"]["de"] + " Fahrrad: für 1:500 neu gezeichnet aus den Körperformen seiner 1:200-Zeichnung allein, Rahmenrohre, Gabel, Lenker, Schutzbleche, Sattel, Kettenkasten und die zwei Reifenringe, an ihren eigenen Knochen, sodass Räder und Ständer beweglich bleiben.")
            d["omitted"] = T_(d["omitted"]["en"] + " Bicycle: spokes, lamps, cables, bell, stand feet details and every detail or texture stroke; their pens are not drawn at 1:500.", d["omitted"]["de"] + " Fahrrad: Speichen, Lampen, Züge, Klingel, Details der Ständerfüße und jeder Detail- oder Texturstrich; deren Stifte werden bei 1:500 nicht gezeichnet.")
            d["thinElements"] = T_(d["thinElements"]["en"] + " Bicycle: the 25 to 35 mm tubes are 0.05 to 0.07 mm on paper and read only through their outline (0.15 mm, the fork 0.16 mm); the tyre rings keep their true 40 mm section.", d["thinElements"]["de"] + " Fahrrad: Die 25- bis 35-mm-Rohre sind 0,05 bis 0,07 mm auf dem Papier und lesen sich nur über den 0,13-mm-Umriss; die Reifenringe behalten ihren wahren Querschnitt von 40 mm.")
        d["overlays"] = T_(d["overlays"]["en"] + " Every walker and bicycle shape names its own thermal region; walker and bicycle share no region.", d["overlays"]["de"] + " Jede Form von Person und Fahrrad nennt ihre eigene thermische Region; Person und Fahrrad teilen keine Region.")
        rep["decisions"] = d
    for s, vol in asset["volumes"].items():
        walker = [T_(strip(x["en"]), strip(x["de"])) for x in par["volumes"][s]["decisions"]]
        walker = [x for x in walker if x["en"] and x["de"]]
        vol["decisions"] = [T_(f"Walker at 1:{s}: the body volumes of the parent figure scaled by 1.03 on the same bones; the stroller and the toddler are removed.", f"Person im Maßstab 1:{s}: die Körpervolumen der Elternfigur, mit 1,03 skaliert, an denselben Knochen; Kinderwagen und Kleinkind sind entfernt.")] + walker + [T_("Bicycle: " + x["en"], "Fahrrad: " + x["de"]) for x in bic["volumes"][s]["decisions"]]
