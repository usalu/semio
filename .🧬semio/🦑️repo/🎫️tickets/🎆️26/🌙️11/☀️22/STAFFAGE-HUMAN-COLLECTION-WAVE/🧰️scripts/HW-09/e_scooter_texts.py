"""📝️ HW-09 physical scenario, provenance and per-scale decisions of `e-scooter-rider` (design input of build_e_scooter_rider.py).

Anthropometry: stature 1.78 m, mass 76 kg (DIN 33402-2 male 50th percentile 1.75 m / 77 kg; ANSUR II male median 1.76 m),
DuBois area 0.007184 · 76^0.425 · 178^0.725 = 1.94 m², basal metabolism Mifflin–St Jeor (male, 30 years) 1728 kcal/day = 83.7 W.
Activity: ISO 8996:2021 Table A.2 standing light work 1.6–2.0 met; Compendium 2011 (Ainsworth) code 16030 motor scooter 3.5 MET as the
upper bound with the body supported; the kick start is a brisk leg push (3.0 met), waiting with one foot down 1.3 met.
"""
import copy
from hw09lib import T_, merge_refs

DUBOIS = 1.94
REFS = [
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020-12 Ergonomie – Körpermaße des Menschen – Teil 2: Werte. Body dimensions of the German population aged 18 to 65, 5th/50th/95th percentiles of stature, mass and segment lengths."},
    {"id": "ansur-ii-2014", "citation": "Gordon, C. C. et al. (2014). 2012 Anthropometric Survey of U.S. Army Personnel (ANSUR II): Methods and Summary Statistics. NATICK/TR-15/007, U.S. Army Natick Soldier RD&E Center."},
    {"id": "mifflin-1990", "citation": "Mifflin, M. D., St Jeor, S. T. et al. (1990). A new predictive equation for resting energy expenditure in healthy individuals. American Journal of Clinical Nutrition 51(2), 241-247."},
    {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine & Science in Sports & Exercise 43(8), 1575-1581."},
    {"id": "ekfv-2019", "citation": "Verordnung über die Teilnahme von Elektrokleinstfahrzeugen am Straßenverkehr (Elektrokleinstfahrzeuge-Verordnung, eKFV) of 6 June 2019, BGBl. I S. 756: design speed 6 to 20 km/h, continuous rated power at most 500 W, handlebar or holding rod required."},
    {"id": "en-17128-2020", "citation": "EN 17128:2020 Light motorized vehicles for the transportation of persons and goods and related facilities and not subject to type-approval on-road – Personal light electric vehicles (PLEV) – Requirements and test methods."},
]


def regions_scaled(regions, factor):
    for r in regions:
        r["areaM2"] = round(r["areaM2"] * factor, 4)
    return regions


def finish(asset, cyc, sco, rider, scooter, K, MASS):
    """✍️ Fills summary, provenance, thermal, structure and every decision text of the composed asset."""
    asset["summary"] = T_(
        "A 1.78 m adult in a vented helmet, softshell jacket, stretch trousers, gloves and canvas sneakers standing on a folding electric kick scooter with a 250 W front hub motor (nameplate) and a deck battery: the scooter is the root of the rig, the rider stands on its deck with the left foot ahead, both hands on the grips. The rider cruises at 20 km/h, kicks off from standstill, waits at a red light with one foot on the kerb side, and brakes with the weight shifted back.",
        "Eine 1,78 m große Person mit belüftetem Helm, Softshelljacke, Stretchhose, Handschuhen und Stoffschuhen steht auf einem klappbaren E-Tretroller mit 250-W-Nabenmotor im Vorderrad (Typenschild) und Akku im Trittbrett: Der Roller ist die Wurzel des Modells, die Person steht mit dem linken Fuß vorn auf dem Trittbrett, beide Hände an den Griffen. Sie fährt mit 20 km/h, stößt sich aus dem Stand ab, wartet an einer roten Ampel mit einem Fuß am Boden und bremst mit nach hinten verlagertem Gewicht.",
    )
    prov = cyc["provenance"]
    asset["provenance"] = {
        "author": "HW-09, Staffage Human Collection Wave (urban mobility)",
        "method": "handcrafted",
        "created": "2026-11-22",
        "references": merge_refs(prov["references"], sco["provenance"]["references"], REFS),
        "notes": T_(
            "Rider derived from the rider of cyclist (rig, outlines, volumes) by re-rooting on the scooter deck through the hip-lift and hip-shift prismatic pair and an isotropic anthropometric scale of 1.035 (1.72 m to 1.78 m stature), segment masses rescaled to 76 kg with de Leva fractions; the scooter is the electric-kick-scooter of the vehicles category (Xiaomi M365 class, 250 W hub motor) in its own frame, with a new 1:500 drawing. Poses are solved by damped least-squares inverse kinematics on the library's evaluate: soles on the deck mat, gloves on the grips. Generator: T/🧰️scripts/HW-09/build_e_scooter_rider.py and poses_e_scooter_rider.ts.",
            "Die fahrende Person ist aus der Fahrerin von cyclist abgeleitet (Rig, Umrisse, Volumen): auf das Trittbrett des Rollers umgehängt über das Schiebepaar hip-lift und hip-shift und isotrop anthropometrisch mit 1,035 skaliert (1,72 m auf 1,78 m Körpergröße), Segmentmassen nach den Anteilen von de Leva auf 76 kg umgerechnet; der Roller ist der electric-kick-scooter der Kategorie Fahrzeuge (Klasse Xiaomi M365, 250-W-Nabenmotor) in seinem eigenen Rahmen, mit einer neuen Zeichnung 1:500. Die Posen sind mit gedämpfter inverser Kinematik der kleinsten Quadrate auf evaluate der Bibliothek gelöst: Sohlen auf der Trittmatte, Handschuhe an den Griffen. Generator: T/🧰️scripts/HW-09/build_e_scooter_rider.py und poses_e_scooter_rider.ts.",
        ),
    }
    rider_regions = regions_scaled(rider["regions"], DUBOIS / 1.78 / (K * K))
    th_c, th_s = cyc["thermal"], sco["thermal"]
    resp = copy.deepcopy(th_c["respiration"])
    resp["patterns"] = [
        {"id": "waiting", "title": T_("Calm breathing while waiting", "Ruhige Atmung beim Warten"), "rateBpm": 13, "tidalVolumeL": 0.6, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "cruising", "title": T_("Steady breathing while cruising", "Gleichmäßige Atmung beim Fahren"), "rateBpm": 15, "tidalVolumeL": 0.75, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.03, "exhaleFraction": 0.47, "pauseAfterExhaleFraction": 0.1, "route": "nose", "source": "authored-estimate", "reference": "gupta-2010"},
        {"id": "kicking", "title": T_("Deeper breathing after kicking off", "Tiefere Atmung nach dem Abstoßen"), "rateBpm": 21, "tidalVolumeL": 1.1, "inhaleFraction": 0.42, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.06, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "gupta-2010"},
    ]
    resp["motion"] = [{"bone": "torso", "angleDeg": 1, "dy": 0.005}]
    body = copy.deepcopy(th_c["body"])
    body.update(massKg=76, skinAreaM2=DUBOIS, basalMetabolismW=83.7, source="literature", reference="mifflin-1990")
    asset["thermal"] = {
        "environment": {"airTempC": 16, "meanRadiantTempC": 18, "skyTempC": 6, "groundTempC": 21, "relativeHumidity": 0.6, "windMS": 5.6, "windDirectionDeg": 180, "solarWM2": 350, "diffuseFraction": 0.35, "pressurePa": 101325},
        "baseMet": 1.8,
        "body": body,
        "respiration": resp,
        "passengers": [],
        "operating": copy.deepcopy(th_s["operating"]),
        "regions": rider_regions + scooter["regions"],
        "emitters": copy.deepcopy(th_c["emitters"]),
        "assumptions": [
            T_("Body regions sum to the DuBois area of 1.94 m2 (76 kg, 178 cm); clothed regions carry the area factor 1 + 0.28 clo. The scooter surfaces are those of the electric-kick-scooter asset (deck with battery 0.34 m2, grip mat, stem and bar, fenders, tyres, hub motor, brake disc, rear rim).", "Die Körperregionen ergeben die DuBois-Fläche von 1,94 m2 (76 kg, 178 cm); bekleidete Regionen tragen den Flächenfaktor 1 + 0,28 clo. Die Rollerflächen sind die des Assets electric-kick-scooter (Trittbrett mit Akku 0,34 m2, Trittmatte, Lenkstange und Lenker, Schutzbleche, Reifen, Nabenmotor, Bremsscheibe, Hinterradfelge)."),
            th_c["assumptions"][1],
            T_("Standing on the moving deck is 1.8 met (ISO 8996 standing light work 1.6 to 2.0 met; the Compendium's 3.5 MET for a seated motor scooter rider is an upper bound), the kick start 3.0 met, waiting with one foot down 1.3 met, braking 2.0 met; the rider does no external work, the motor drives.", "Stehen auf dem fahrenden Trittbrett entspricht 1,8 met (ISO 8996 leichte Arbeit im Stehen 1,6 bis 2,0 met; die 3,5 MET des Compendiums für Motorrollerfahren im Sitzen sind eine obere Grenze), das Abstoßen 3,0 met, Warten mit einem Fuß am Boden 1,3 met, Bremsen 2,0 met; die Person leistet keine äußere Arbeit, der Motor treibt an."),
            T_("The relative wind of 5.6 m/s is the riding speed of 20 km/h, the design speed limit of the German eKFV; it blows the thermal plume and the breath backwards.", "Der relative Wind von 5,6 m/s ist die Fahrgeschwindigkeit von 20 km/h, die bauartbedingte Höchstgeschwindigkeit nach eKFV; er bläst thermische Fahne und Atem nach hinten."),
            T_("Nameplates: hub motor 250 W rated output (eKFV allows at most 500 W continuous), charging circuit 63 W electrical input; the scooter's operating heat (motor 55 W, battery 12 W, brake disc 890 W while braking) is released heat and is not counted as power.", "Typenschilder: Nabenmotor 250 W Nennabgabe (die eKFV erlaubt höchstens 500 W Dauerleistung), Ladeschaltung 63 W Eingangsleistung; die Betriebswärme des Rollers (Motor 55 W, Akku 12 W, Bremsscheibe 890 W beim Bremsen) ist abgegebene Wärme und zählt nicht als Leistung."),
        ] + [a for a in th_s["assumptions"] if "rider is not part" not in a["en"]],
    }
    asset["structure"] = {
        "gravityMS2": 9.80665,
        "dynamics": "inertial",
        "supports": [],
        "assumptions": [
            T_("The two tyre patches carry scooter and rider. The rider stands on the deck mat; the solver carries the rider's weight from the pelvis through the hip-lift joint into the deck, so the legs show the moments of the standing posture while the feet transfer no separate contact force.", "Die beiden Reifenaufstandsflächen tragen Roller und Person. Die Person steht auf der Trittmatte; der Löser leitet ihr Gewicht vom Becken über das Gelenk hip-lift in das Trittbrett, sodass die Beine die Momente der Standhaltung zeigen, während die Füße keine eigene Kontaktkraft übertragen."),
            T_("With one foot on the ground (waiting, kicking) that sole is an additional ground contact; load sharing follows the equal-stiffness assumption of the solver.", "Mit einem Fuß am Boden (Warten, Abstoßen) ist diese Sohle ein zusätzlicher Bodenkontakt; die Lastaufteilung folgt der Gleichsteifigkeitsannahme des Lösers."),
            T_("The scooter stays in place and the ground advances (treadmill frame); in the ride loop the wheels roll without slip, theta = -x / r.", "Der Roller bleibt stehen und der Boden läuft vorbei (Laufbandsystem); in der Fahrschleife rollen die Räder ohne Schlupf, theta = -x / r."),
        ],
        "clinical": False,
    }
    decisions(asset, cyc, sco)


SC = {
    "50": T_("the scooter of the vehicle drawing at 1:50: hollow deck with grip mat, nose and tail castings, gooseneck, steering head, folding hinge, stem with cockpit, bar and grips, fork with fenders, the front hub motor with its nameplate cap and the rear disc wheel with brake rotor", "der Roller der Fahrzeugzeichnung 1:50: hohles Trittbrett mit Trittmatte, Nasen- und Heckguss, Schwanenhals, Lenkkopf, Klappgelenk, Lenkstange mit Cockpit, Lenker und Griffen, Gabel mit Schutzblechen, der Nabenmotor im Vorderrad mit Typenschilddeckel und das hintere Scheibenrad mit Bremsscheibe"),
    "100": T_("the scooter of the vehicle drawing at 1:100: deck with mat, neck, head, stem, bar, fork, fenders and both wheels with hub and rim", "der Roller der Fahrzeugzeichnung 1:100: Trittbrett mit Matte, Hals, Lenkkopf, Lenkstange, Lenker, Gabel, Schutzbleche und beide Räder mit Nabe und Felge"),
    "200": T_("the scooter of the vehicle drawing at 1:200: deck with neck, stem, bar, fork and two wheel discs", "der Roller der Fahrzeugzeichnung 1:200: Trittbrett mit Hals, Lenkstange, Lenker, Gabel und zwei Radscheiben"),
    "500": T_("the scooter drawn new for 1:500: the deck and its neck are one slab, the stem one bar with the handlebar knob, the fork one plate and the wheels two filled discs, all on their own bones so the fold, the steering and the wheel spin stay articulated", "der für 1:500 neu gezeichnete Roller: Trittbrett und Hals sind eine Platte, die Lenkstange ein Stab mit dem Lenkerknauf, die Gabel eine Platte und die Räder zwei gefüllte Scheiben, alle an ihren eigenen Knochen, sodass Klappen, Lenken und Raddrehung beweglich bleiben"),
}


def decisions(asset, cyc, sco):
    for s, rep in asset["representations"].items():
        c = cyc["representations"][s]["decisions"]
        v = sco["representations"][s]["decisions"] if sco["representations"][s]["status"] == "authored" else None
        d = {}
        for key in ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]:
            rider_en = strip_bike(c[key]["en"])
            rider_de = strip_bike(c[key]["de"])
            d[key] = T_(f"Rider: {rider_en}", f"Person: {rider_de}")
        d["silhouette"] = T_(
            f"The rider at 1:{s} keeps the outlines of the cyclist body at this scale, scaled to 1.78 m and carried by its own bones; {SC[s]['en']}.",
            f"Die Person im Maßstab 1:{s} behält die Umrisse des Radfahrerkörpers dieses Maßstabs, auf 1,78 m skaliert und von ihren eigenen Knochen getragen; {SC[s]['de']}.",
        )
        if v is not None:
            for key in ["retained", "merged", "omitted", "thinElements"]:
                d[key] = T_(d[key]["en"] + " Scooter: " + v[key]["en"], d[key]["de"] + " Roller: " + v[key]["de"])
        else:
            d["retained"] = T_(d["retained"]["en"] + " Scooter: deck with neck, stem with bar, fork, front and rear wheel.", d["retained"]["de"] + " Roller: Trittbrett mit Hals, Lenkstange mit Lenker, Gabel, Vorder- und Hinterrad.")
            d["merged"] = T_(d["merged"]["en"] + " Scooter: deck, mat, fenders and neck into one slab; bar into the stem knob; tyre, rim, motor and disc into one wheel disc each.", d["merged"]["de"] + " Roller: Trittbrett, Matte, Schutzbleche und Hals zu einer Platte; Lenker in den Knauf der Lenkstange; Reifen, Felge, Motor und Scheibe zu je einer Radscheibe.")
            d["omitted"] = T_(d["omitted"]["en"] + " Scooter: hinge, cockpit, lamps, kickstand, stays and every detail below 0.2 mm on paper.", d["omitted"]["de"] + " Roller: Klappgelenk, Cockpit, Lampen, Seitenständer, Streben und jedes Detail unter 0,2 mm auf dem Papier.")
            d["thinElements"] = T_(d["thinElements"]["en"] + " Scooter: the 40 mm stem and the 55 mm deck are drawn at their true section, about 0.1 mm on paper, and read through the 0.13 mm outline.", d["thinElements"]["de"] + " Roller: Die 40-mm-Lenkstange und das 55-mm-Trittbrett sind in ihrem wahren Querschnitt gezeichnet, etwa 0,1 mm auf dem Papier, und lesen sich über den 0,13-mm-Umriss.")
        d["overlays"] = T_(d["overlays"]["en"] + " The scooter shapes name their thermal regions: deck slab deck-battery, deck-grip and fenders; stem, bar and fork stem-bars; front wheel tyres and hub-motor; rear wheel tyres, rear-rim and brake-disc.", d["overlays"]["de"] + " Die Rollerformen nennen ihre thermischen Regionen: Trittbrettplatte deck-battery, deck-grip und fenders; Lenkstange, Lenker und Gabel stem-bars; Vorderrad tyres und hub-motor; Hinterrad tyres, rear-rim und brake-disc.")
        rep["decisions"] = d
    for s, vol in asset["volumes"].items():
        rider = [x for x in cyc["volumes"][s]["decisions"] if "spoke" not in x["en"].lower()]
        rider = [T_(strip_bike(x["en"]).replace("a 1.72 m woman on a low-step city bicycle", "a 1.78 m adult standing on an e-scooter deck").replace("the measure of a 1.72 m woman", "the measure of the 1.72 m reference figure scaled by 1.035"), strip_bike(x["de"]).replace("einer 1,72 m großen Frau", "der 1,72-m-Bezugsfigur, mit 1,035 skaliert")) for x in rider]
        scoot = copy.deepcopy(sco["volumes"][s]["decisions"])
        vol["decisions"] = [T_(f"Rider at 1:{s}: the cyclist body volumes scaled by 1.035 on the same bones; the bicycle and its load path are replaced by the scooter.", f"Person im Maßstab 1:{s}: die Körpervolumen der Radfahrerin, mit 1,035 skaliert, an denselben Knochen; Fahrrad und Lastpfad sind durch den Roller ersetzt.")] + rider + [T_("Scooter: " + x["en"], "Roller: " + x["de"]) for x in scoot]


def strip_bike(text):
    """🚲️ Drops the sentences of a cyclist decision that describe the bicycle (the rider sentences stay)."""
    import re
    parts = re.split(r"(?<=[.;])\s+", text)
    words = ("bicycle", "Fahrrad", "bike", "wheel", "Rad ", "Räder", "spoke", "Speiche", "crank", "Kurbel", "pedal", "Pedal", "saddle", "Sattel", "fender", "Schutzblech", "carrier", "Gepäckträger", "frame", "Rahmen", "tyre", "Reifen", "chain", "Kette")
    kept = [p for p in parts if not any(w in p for w in words)]
    return " ".join(kept) if kept else "as at this scale of the source figure."
