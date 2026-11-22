"""📝️ HW-09 physical scenario, provenance and per-scale decisions of `traveller-with-rolling-suitcase`.

Anthropometry: stature 1.82 m, mass 76 kg (DIN 33402-2 men: about the 85th percentile of stature; ANSUR II male median 1.76 m),
DuBois area 0.007184 · 76^0.425 · 182^0.725 = 1.97 m², basal metabolism the sex-neutral mean of the Mifflin–St Jeor equations at
40 years: 1620 kcal/day = 78.4 W.
Activity: walking 1.2 m/s pulling a 8.75 kg cabin trolley 3.3 met (Compendium 2011 17190 walking 2.8–3.2 mph 3.5 MET, 17170 2.5 mph
3.0 MET; rolling resistance of small hard wheels Crr ≈ 0.03 → 2.6 N, 3 W external work), standing with the upright trolley 1.3 met,
tilting the trolley onto its wheels 2.0 met.
Suitcase: IATA cabin baggage guideline 55 × 40 × 23 cm including wheels and handle; 2.8 kg shell plus 5.7 kg contents.
"""
import copy
import re
from hw09lib import T_, merge_refs
from e_scooter_texts import REFS

DUBOIS = 1.97
EXTRA = [{"id": "iata-cabin-2023", "citation": "IATA (2023). Cabin baggage guidance: recommended maximum size 56 × 45 × 25 cm including wheels, handles and side pockets; typical hard-shell cabin trolleys 55 × 40 × 23 cm, 2.5 to 3.2 kg empty."},
         {"id": "incropera-2011", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley. Appendix A (thermophysical properties of polymers and aluminium)."}]
DROP = ("1.78", "1,78", "1.80", "1,80", "a man of", "ein Mann")


def keep(text):
    parts = re.split(r"(?<=[.;])\s+", text)
    return " ".join(p for p in parts if not any(w in p for w in DROP))


def finish(asset, com, person, case_regions, K, MASS):
    asset["summary"] = T_(
        "A 1.82 m traveller in a light camel overcoat and scarf, a leather shoulder bag on the right hip, walks at 1.2 m/s and pulls a hard-shell cabin trolley (55 × 40 × 23 cm, 8.5 kg) behind the left hand: the suitcase tilts back about its axle until its handle meets the hand, and its two wheels roll on the pavement. The figure also stands with the trolley upright on its wheels and feet and tilts it onto its wheels before walking off.",
        "Eine 1,82 m große reisende Person in hellem kamelfarbenem Mantel und Schal, eine Ledertasche über der rechten Hüfte, geht mit 1,2 m/s und zieht einen Hartschalen-Kabinentrolley (55 × 40 × 23 cm, 8,5 kg) hinter der linken Hand her: Der Koffer kippt um seine Achse, bis sein Griff die Hand trifft, und seine zwei Räder rollen auf dem Gehweg. Die Figur steht auch mit dem aufrechten Trolley auf Rädern und Füßen und kippt ihn vor dem Losgehen auf die Räder.",
    )
    asset["provenance"] = {
        "author": "HW-09, Staffage Human Collection Wave (urban mobility)",
        "method": "handcrafted",
        "created": "2026-11-22",
        "references": merge_refs(com["provenance"]["references"], REFS, EXTRA),
        "notes": T_(
            "Walker derived from walking-commuter (rig with finger bones, outlines, volumes and its heel-to-toe walk keys), scaled isotropically by 1.02 to 1.82 m with segment masses rescaled to 76 kg; the coat is re-toned to a light camel cloth (solar absorptivity 0.55). The cabin trolley is authored here for every scale: rounded shell, wheel housings, feet, two telescopic tubes and a grip, two wheels of 77.9 mm, whose radius is chosen so one walk cycle of 1.22 m is exactly five wheel turns. The left arm is solved by damped least-squares inverse kinematics on the library's evaluate so the hand stays on the grip, and the trolley tilts about its axle so the wheels stay on the ground. Generator: T/🧰️scripts/HW-09/build_traveller.py and poses_traveller.ts.",
            "Die gehende Person ist aus walking-commuter abgeleitet (Rig mit Fingerknochen, Umrisse, Volumen und seine Fersen-Ballen-Gangschlüssel), isotrop mit 1,02 auf 1,82 m skaliert, Segmentmassen auf 76 kg umgerechnet; der Mantel ist in hellem Kamelton umgefärbt (solare Absorption 0,55). Der Kabinentrolley ist hier für jeden Maßstab gezeichnet: gerundete Schale, Radgehäuse, Füße, zwei Teleskoprohre und ein Griff, zwei Räder von 77,9 mm, deren Radius so gewählt ist, dass ein Gangzyklus von 1,22 m genau fünf Radumdrehungen ergibt. Der linke Arm ist mit gedämpfter inverser Kinematik der kleinsten Quadrate auf evaluate der Bibliothek gelöst, sodass die Hand am Griff bleibt, und der Trolley kippt um seine Achse, sodass die Räder am Boden bleiben. Generator: T/🧰️scripts/HW-09/build_traveller.py und poses_traveller.ts.",
        ),
    }
    for r in person["regions"]:
        r["areaM2"] = round(r["areaM2"] * DUBOIS / 1.894 / (K * K) if r.get("model") == "body" else r["areaM2"] * K * K, 4)
        if r["id"] == "coat":
            r["title"] = T_("Light camel wool overcoat", "Heller kamelfarbener Wollmantel")
    th = com["thermal"]
    resp = copy.deepcopy(th["respiration"])
    body = copy.deepcopy(th["body"])
    body.update(massKg=MASS, skinAreaM2=DUBOIS, basalMetabolismW=78.4, source="literature", reference="mifflin-1990")
    asset["thermal"] = {
        "environment": copy.deepcopy(th["environment"]),
        "baseMet": 1.3,
        "body": body,
        "respiration": resp,
        "passengers": [],
        "operating": [],
        "regions": person["regions"] + case_regions,
        "emitters": copy.deepcopy(th["emitters"]),
        "assumptions": [
            T_("Body regions sum to the DuBois area of 1.97 m2 (76 kg, 182 cm); clothed regions carry the area factor 1 + 0.28 clo. The leather bag and the trolley (shell 0.86 m2, handle, grip, wheels and housings 0.13 m2) are passive surfaces.", "Die Körperregionen ergeben die DuBois-Fläche von 1,97 m2 (76 kg, 182 cm); bekleidete Regionen tragen den Flächenfaktor 1 + 0,28 clo. Ledertasche und Trolley (Schale 0,86 m2, Griff, Griffstück, Räder und Gehäuse 0,13 m2) sind passive Flächen."),
            T_("Walking at 1.2 m/s while pulling the 8.75 kg trolley is 3.3 met (Compendium 2011, codes 17170 and 17190); the small hard wheels roll with Crr about 0.03, so the hand pulls 2.6 N and does 3 W of external work. Standing with the upright trolley is 1.3 met, tilting it onto its wheels 2.0 met.", "Gehen mit 1,2 m/s und dem 8,75 kg schweren Trolley entspricht 3,3 met (Compendium 2011, Codes 17170 und 17190); die kleinen harten Räder rollen mit Crr etwa 0,03, daher zieht die Hand mit 2,6 N und leistet 3 W äußere Arbeit. Stehen mit dem aufrechten Trolley entspricht 1,3 met, das Kippen auf die Räder 2,0 met."),
            T_("The trolley carries no electrical element; the total power is the metabolic heat of the traveller.", "Der Trolley hat kein elektrisches Element; die Gesamtleistung ist die Stoffwechselwärme der reisenden Person."),
        ] + [a for a in th["assumptions"] if not any(w in a["en"] for w in ("1.78", "1.894", "72 kg"))],
    }
    asset["structure"] = {
        "gravityMS2": 9.80665, "dynamics": "inertial", "supports": [],
        "assumptions": [
            T_("The trolley hangs from the pelvis through the case-lift and case-slide joints and tilts about its axle; its wheel patches are a ground contact in every pose, its feet only when it stands upright. The solver closes the loop through the pelvis, so part of the trolley's 85 N reaches the hand and the shoulder as in pulling.", "Der Trolley hängt über die Gelenke case-lift und case-slide am Becken und kippt um seine Achse; seine Radaufstandsflächen sind in jeder Pose ein Bodenkontakt, seine Füße nur, wenn er aufrecht steht. Der Löser schließt die Kette über das Becken, daher erreicht ein Teil der 85 N des Trolleys wie beim Ziehen Hand und Schulter."),
        ] + [a for a in com["structure"]["assumptions"] if not any(w in a["en"] for w in ("1.78", "72 kg"))],
        "clinical": False,
    }
    CASE = {
        "50": T_("the trolley at 1:50: rounded polycarbonate shell with its zip line, two wheel housings, two feet, two telescopic tubes, the grip and two wheels with hubs", "der Trolley 1:50: gerundete Polycarbonatschale mit Reißverschlusslinie, zwei Radgehäuse, zwei Füße, zwei Teleskoprohre, der Griff und zwei Räder mit Naben"),
        "100": T_("the trolley at 1:100: rounded shell, housings, feet, two tubes, grip and two wheels; zip and hubs are below the scale", "der Trolley 1:100: gerundete Schale, Gehäuse, Füße, zwei Rohre, Griff und zwei Räder; Reißverschluss und Naben liegen unter dem Maßstab"),
        "200": T_("the trolley at 1:200: shell with the housings merged in, two tubes and the grip, two wheels; the feet are dropped", "der Trolley 1:200: Schale mit eingeschlossenen Gehäusen, zwei Rohre und Griff, zwei Räder; die Füße entfallen"),
        "500": T_("the trolley at 1:500: one shell block, the handle as one 75 mm bar and the wheels as one roller across the width", "der Trolley 1:500: ein Schalenblock, der Griff als ein 75-mm-Stab und die Räder als eine Walze über die Breite"),
    }
    for s, rep in asset["representations"].items():
        c = com["representations"][s]["decisions"]
        d = {k: T_("Traveller: " + (keep(c[k]["en"]) or "as the source figure at this scale."), "Person: " + (keep(c[k]["de"]) or "wie die Ausgangsfigur in diesem Maßstab.")) for k in c}
        d["silhouette"] = T_(f"The traveller at 1:{s} keeps the outlines of the walking commuter at this scale, scaled to 1.82 m, the coat re-toned light; {CASE[s]['en']}. Every trolley shape is a swept solid on its bone, so tilting and rolling read in the three views.", f"Die reisende Person im Maßstab 1:{s} behält die Umrisse des Pendlers dieses Maßstabs, auf 1,82 m skaliert, der Mantel hell umgefärbt; {CASE[s]['de']}. Jede Trolleyform ist ein gezogener Körper an ihrem Knochen, sodass Kippen und Rollen in den drei Ansichten lesbar sind.")
        d["overlays"] = T_(d["overlays"]["en"] + " The trolley shell names suitcase-shell; tubes, grip, housings, feet and wheels name handle-and-wheels.", d["overlays"]["de"] + " Die Trolleyschale nennt suitcase-shell; Rohre, Griff, Gehäuse, Füße und Räder nennen handle-and-wheels.")
        rep["decisions"] = d
    for s, vol in asset["volumes"].items():
        walker = [T_(keep(x["en"]), keep(x["de"])) for x in com["volumes"][s]["decisions"]]
        walker = [x for x in walker if x["en"] and x["de"]]
        vol["decisions"] = [T_(f"Traveller at 1:{s}: the body volumes of the walking commuter scaled by 1.02 on the same bones.", f"Person im Maßstab 1:{s}: die Körpervolumen des Pendlers, mit 1,02 skaliert, an denselben Knochen.")] + walker + [
            T_(f"Trolley at 1:{s}: {CASE[s]['en']}; the shell is a rounded box, the tubes declared slender, the wheels turned profiles on the wheel bone; the thermal list refers to the body solids (shell: suitcase-shell, everything else: handle-and-wheels), the structure list carries the shell and wheels as members, a contents mass and the axle joint.", f"Trolley im Maßstab 1:{s}: {CASE[s]['de']}; die Schale ist ein gerundeter Quader, die Rohre sind als schlank erklärt, die Räder Drehprofile am Radknochen; die thermische Liste verweist auf die Körper (Schale: suitcase-shell, alles andere: handle-and-wheels), die Tragwerksliste führt Schale und Räder als Glieder, eine Inhaltsmasse und das Achsgelenk.")]
