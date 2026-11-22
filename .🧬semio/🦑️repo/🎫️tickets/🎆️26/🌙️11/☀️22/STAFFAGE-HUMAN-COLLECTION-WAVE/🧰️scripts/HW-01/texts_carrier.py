"""📝️ Titles, summary, provenance, assumptions and per-scale decisions of `office-worker-carrying-laptop` (English first, German second)."""
from hw import t
from texts_desk import REFS
from scrub import scrub

DROP = (r"\bbag|strap|watch|crossing|commuter", r"Tasche|Riemen|Uhr\b|Armbanduhr|Übergang|Pendler")

SIL = {
    "50": ("Every shape is the orthographic outline of the 1:50 model in the standing pose, bound to its bone. Side view from the right: overcoat with collar and near pocket, scarf, head with hair, ear and eye, the near arm with the forearm held level and the hand round the front edge of the closed laptop that stands on the forearm against the hip (shell, lid seam and logo), trousers, shoes with soles, the far hand behind the coat; {mm} mm tall. Front view (the figure's left is on the right of the sheet): the laptop shows its 22 mm edge beside the coat. Plan: the laptop is a narrow plate beside the hip, over the forearm.",
           "Jede Form ist der orthogonale Umriss des 1:50-Modells in der Standpose, an ihren Knochen gebunden. Seitenansicht von rechts: Mantel mit Kragen und naher Tasche, Schal, Kopf mit Haar, Ohr und Auge, der nahe Arm mit waagerecht gehaltenem Unterarm und der Hand um die Vorderkante des zugeklappten Laptops, der auf dem Unterarm an der Hüfte steht (Schale, Deckelfuge und Logo), Hose, Schuhe mit Sohlen, die ferne Hand hinter dem Mantel; {mm} mm hoch. Vorderansicht (die linke Körperseite liegt rechts im Blatt): der Laptop zeigt seine 22-mm-Kante neben dem Mantel. Grundriss: der Laptop ist eine schmale Platte neben der Hüfte, über dem Unterarm."),
    "100": ("Every shape is the orthographic outline of the 1:100 model in the standing pose. Side view from the right: overcoat with collar, scarf, head with hair and ear, the near arm carrying the closed laptop on the level forearm, trousers and shoes; {mm} mm tall. Front and plan: the laptop as its edge beside the hip.",
            "Jede Form ist der orthogonale Umriss des 1:100-Modells in der Standpose. Seitenansicht von rechts: Mantel mit Kragen, Schal, Kopf mit Haar und Ohr, der nahe Arm trägt den zugeklappten Laptop auf dem waagerechten Unterarm, Hose und Schuhe; {mm} mm hoch. Vorder- und Draufsicht: der Laptop als seine Kante neben der Hüfte."),
    "200": ("Every shape is the orthographic outline of the 1:200 model in the standing pose: coat, scarf ring, head with hair, the near sleeve with the level forearm and fist under the laptop plate, trousers and shoes; {mm} mm tall.",
            "Jede Form ist der orthogonale Umriss des 1:200-Modells in der Standpose: Mantel, Schalring, Kopf mit Haar, der nahe Ärmel mit waagerechtem Unterarm und Faust unter der Laptopplatte, Hose und Schuhe; {mm} mm hoch."),
    "500": ("Every shape is the orthographic outline of the 1:500 model in the standing pose: coat, scarf, head with hair, sleeves, the near forearm with the hand drawn as its 1:500 mass, the laptop as one plate, legs and one shoe per foot; {mm} mm tall.",
            "Jede Form ist der orthogonale Umriss des 1:500-Modells in der Standpose: Mantel, Schal, Kopf mit Haar, Ärmel, der nahe Unterarm mit der Hand als ihre 1:500-Masse, der Laptop als eine Platte, Beine und ein Schuh je Fuß; {mm} mm hoch."),
}
RET = {
    "50": t("Lapels, buttons, pocket flaps, shirt cuffs, fingers and thumb, ear, eye, the lid seam and the logo of the laptop, shoe soles and heels.", "Revers, Knöpfe, Taschenklappen, Hemdmanschetten, Finger und Daumen, Ohr, Auge, Deckelfuge und Logo des Laptops, Schuhsohlen und Absätze."),
    "100": t("Lapels, collar, ears, the thumb of the far hand and the laptop on the level forearm.", "Revers, Kragen, Ohren, der Daumen der fernen Hand und der Laptop auf dem waagerechten Unterarm."),
    "200": t("Scarf, hair, both hands and the laptop plate stay separate shapes.", "Schal, Haar, beide Hände und die Laptopplatte bleiben eigene Formen."),
    "500": t("The level near forearm with the laptop plate on it keeps the figure recognisable as someone carrying a laptop.", "Der waagerechte nahe Unterarm mit der Laptopplatte darauf hält die Figur als Person mit Laptop erkennbar."),
}
MER = {
    "50": t("The lid and base of the closed laptop are one shell with the seam as a detail line.", "Deckel und Unterteil des zugeklappten Laptops sind eine Schale mit der Fuge als Detaillinie."),
    "100": t("Lapels and collar stay; the laptop is one shell.", "Revers und Kragen bleiben; der Laptop ist eine Schale."),
    "200": t("Lapels and collar merge into the coat, nose and ears into the head, wrists into the hands.", "Revers und Kragen gehen im Mantel auf, Nase und Ohren im Kopf, Handgelenke in den Händen."),
    "500": t("Forefoot and heel merge into one shoe, the neck into scarf and head, fingers into the hands.", "Vorfuß und Ferse werden ein Schuh, der Hals geht in Schal und Kopf auf, Finger in den Händen."),
}
OMI = {
    "50": t("Ports, hinge and rubber feet of the laptop.", "Anschlüsse, Scharnier und Gummifüße des Laptops."),
    "100": t("Buttons, pockets, eyes, mouth, the lid seam and logo.", "Knöpfe, Taschen, Augen, Mund, Deckelfuge und Logo."),
    "200": t("Buttons, pockets, face details and fingers.", "Knöpfe, Taschen, Gesichtsdetails und Finger."),
    "500": t("Lapels, collar, buttons, pockets, face and the fringe of the hair.", "Revers, Kragen, Knöpfe, Taschen, Gesicht und der Pony."),
}
THIN = {
    "50": t("The laptop is drawn at its true 22 mm; mouth, brows, nose, coat front edge, back vent and seams are detail strokes.", "Der Laptop ist in seinen wahren 22 mm gezeichnet; Mund, Brauen, Nase, Mantelvorderkante, Rückenschlitz und Nähte sind Detailstriche."),
    "100": t("The laptop at its true 22 mm; the coat front edge is a detail stroke.", "Der Laptop in seinen wahren 22 mm; die Mantelvorderkante ist ein Detailstrich."),
    "200": t("The laptop is drawn 21 mm thick (its aluminium shell without the rubber feet that the 22 mm solid includes), printing as 0.1 mm plus the body pen.", "Der Laptop ist 21 mm dick gezeichnet (seine Aluminiumschale ohne die Gummifüße, die der 22-mm-Körper einschließt) und druckt als 0,1 mm plus Körperstift."),
    "500": t("The laptop shell (21 mm drawn, 22 mm modelled with its feet) prints as the 0.18 mm body pen line; the near hand is drawn as its 1:500 mass; the 1:500 body pen is 0.18 mm.", "Die Laptopschale (21 mm gezeichnet, 22 mm mit Füßen modelliert) druckt als 0,18-mm-Körperlinie; die nahe Hand ist als ihre 1:500-Masse gezeichnet; der Körperstift von 1:500 ist 0,18 mm."),
}
HATCH = {
    "50": t("Wool coat, collar and sleeves: parallel lines at 65 degrees, 0.5 mm spacing; scarf, trousers and laptop stay plain tone.", "Wollmantel, Kragen und Ärmel: parallele Linien unter 65 Grad, 0,5 mm Abstand; Schal, Hose und Laptop bleiben flache Töne."),
    "100": t("Wool coat only: fine parallel lines at 65 degrees, 0.6 mm spacing, 0.10 mm pen as a restrained material cue; trousers and laptop stay plain tone.", "Nur der Wollmantel: feine parallele Linien unter 65 Grad, 0,6 mm Abstand, Stift 0,10 mm als zurückhaltender Materialhinweis; Hose und Laptop bleiben flache Töne."),
}
STY_ADD = t("The laptop is light aluminium against the dark coat.", "Der Laptop ist helles Aluminium vor dem dunklen Mantel.")
OVER_ADD = t("The laptop carries its own passive region with the 65 W nameplate of its power supply; asleep under the arm it releases no heat.", "Der Laptop trägt seine eigene passive Region mit dem 65-W-Typenschild seines Netzteils; schlafend unter dem Arm gibt er keine Wärme ab.")
VOL = {
    "body": t("The closed laptop is one rounded box of 357 × 245 × 22 mm standing on its long edge on the level near forearm, its medial face against the coat at the hip.", "Der zugeklappte Laptop ist ein gerundeter Quader von 357 × 245 × 22 mm, der auf seiner Längskante auf dem waagerechten nahen Unterarm steht, seine Innenseite am Mantel an der Hüfte."),
    "thermal": t("The laptop is its own passive surface by reference to its body solid.", "Der Laptop ist eine eigene passive Oberfläche als Verweis auf seinen Körper."),
    "structure": t("The laptop is a member with its point mass (1.9 kg) carried by the near forearm.", "Der Laptop ist ein Glied mit seiner Punktmasse (1,9 kg), getragen vom nahen Unterarm."),
}


def fix(s, lang):
    for a, b in (("1.78 m", "1.81 m"), ("1,78 m", "1,81 m"), ("1.80 m", "1.85 m"), ("1,80 m", "1,85 m"), ("72 kg", "80 kg")):
        s = s.replace(a, b)
    return s


def TEXTS(a, stature):
    h = round(1.81 * 1.025, 2)
    a["title"] = t("Office Worker Carrying a Laptop", "Büroangestellte Person mit Laptop unter dem Arm")
    a["summary"] = t(
        f"An office worker of {stature:.2f} m (80 kg) in a knee-length wool overcoat and scarf walks from the station to the office with a closed 15.6-inch laptop carried on the level near forearm and pressed to the hip by the upper arm: heel-to-toe gait with double support at about 1.1 m/s, the far arm swinging. Poses for standing and two gait instants, loops for walking and idle breathing; 2.6 met while walking at 8 °C, a 65 W laptop asleep.",
        f"Eine büroangestellte Person von {stature:.2f} m (80 kg) in knielangem Wollmantel und Schal geht vom Bahnhof ins Büro und trägt einen zugeklappten 15,6-Zoll-Laptop auf dem waagerechten nahen Unterarm, vom Oberarm an die Hüfte gedrückt: Fersen-Zehen-Gang mit Doppelstützphase bei etwa 1,1 m/s, der ferne Arm schwingt. Posen für Stehen und zwei Gangmomente, Schleifen für Gehen und ruhiges Atmen; 2,6 met beim Gehen bei 8 °C, ein 65-W-Laptop im Ruhezustand.")
    a["tags"] = ["human", "adult", "walker", "office", "laptop", "carrying", "overcoat", "gait", "workplace"]
    pv = a["provenance"]
    pv["author"] = "Semio staffage human worker HW-01"
    pv["method"] = "handcrafted"
    pv["created"] = "2026-11-22"
    have = {x["id"] for x in pv["references"]}
    pv["references"] += [x for x in REFS if x["id"] not in have]
    pv["notes"] = t(
        f"Derived by hand from the archive's walking-commuter (rig, outlines, per-scale volumes and the heel-to-toe gait from two-link inverse kinematics after Perry and Burnfield), scaled uniformly by 1.025 to {stature:.2f} m and 80 kg (de Leva 1996 male fractions); the shoulder bag and its strap are removed. The carrying arm (upper arm 6 degrees back, elbow flexed 84 degrees, the forearm level), the closed laptop on it, the prism drawings of the near forearm and hand in front and plan, the 1:500 near hand, poses, clips and the structure are authored by worker HW-01 (2026-11-22). Generator: ticket STAFFAGE-HUMAN-COLLECTION-WAVE, scripts HW-01/build_carrier.py.",
        f"Von Hand aus dem Archivobjekt walking-commuter abgeleitet (Skelett, Umrisse, maßstabsweise Volumen und der Fersen-Zehen-Gang aus zweigliedriger inverser Kinematik nach Perry und Burnfield), gleichmäßig um 1,025 auf {stature:.2f} m und 80 kg skaliert (männliche Anteile nach de Leva 1996); Umhängetasche und Riemen entfernt. Der tragende Arm (Oberarm 6 Grad nach hinten, Ellbogen 84 Grad gebeugt, Unterarm waagerecht), der zugeklappte Laptop darauf, die Prismenzeichnungen von nahem Unterarm und naher Hand in Vorderansicht und Grundriss, die nahe Hand bei 1:500, Posen, Clips und Tragstruktur sind von HW-01 erstellt (22.11.2026). Generator: Ticket STAFFAGE-HUMAN-COLLECTION-WAVE, Skripte HW-01/build_carrier.py.")
    th = a["thermal"]
    th["assumptions"] = [{k: fix(x[k], k) for k in ("en", "de")} for x in th["assumptions"] if not ("bag" in x["en"] or "Tasche" in x["de"])]
    th["assumptions"][0] = t(f"Skin area {th['body']['skinAreaM2']:.3f} m2 after DuBois and DuBois (1916) for 80 kg and {stature:.2f} m; the regions keep the skin fractions of the base figure (0.05 face, 0.024 scalp, 0.021 neck, 0.05 hands, 0.49 trunk and arms, 0.295 legs, 0.07 feet).", f"Hautfläche {th['body']['skinAreaM2']:.3f} m2 nach DuBois und DuBois (1916) für 80 kg und {stature:.2f} m; die Regionen behalten die Hautanteile der Ausgangsfigur (0,05 Gesicht, 0,024 Kopfhaut, 0,021 Hals, 0,05 Hände, 0,49 Rumpf und Arme, 0,295 Beine, 0,07 Füße).")
    th["assumptions"].append(t("The closed laptop is a passive aluminium surface of 0.19 m2 asleep in transit; its 65 W adapter nameplate counts in the total power, its standby heat (below 1 W) is neglected.", "Der zugeklappte Laptop ist eine passive Aluminiumfläche von 0,19 m2, die unterwegs schläft; sein 65-W-Netzteiltypenschild zählt zur Gesamtleistung, seine Standby-Wärme (unter 1 W) wird vernachlässigt."))
    a["structure"]["assumptions"] = [
        t("Segments are point masses at the de Leva centres; the 1.9 kg laptop is a point mass on its own bone fixed to the near forearm, so the elbow flexors and the shoulder carry it.", "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; der 1,9 kg schwere Laptop ist eine Punktmasse auf einem eigenen, fest mit dem nahen Unterarm verbundenen Knochen, sodass Ellbogenbeuger und Schulter ihn tragen."),
        t("Contacts are the heel and the ball of each foot; supports follow the clip contact spans.", "Kontakte sind Ferse und Fußballen jedes Fußes; Auflager folgen den Kontaktspannen des Clips."),
    ]
    P = {p["id"]: p for p in a["poses"]}
    P["standing"]["title"] = t("Standing with the laptop", "Stehen mit dem Laptop")
    P["standing"]["description"] = t("Relaxed upright stance, both feet flat, the near forearm level with the closed laptop standing on it against the hip and the hand round its front edge, the far arm hanging.", "Entspannt aufrecht, beide Füße flach, der nahe Unterarm waagerecht mit dem zugeklappten Laptop darauf an der Hüfte und der Hand um seine Vorderkante, der ferne Arm hängt.")
    for p in a["poses"]:
        p["description"] = {k: v.replace("far arm swinging back", "far arm swinging back, the laptop held still").replace("ferner Arm schwingt zurück", "ferner Arm schwingt zurück, der Laptop bleibt ruhig") for k, v in p["description"].items()}
    C = {c["id"]: c for c in a["clips"]}
    C["walk-cycle"]["title"] = t("Walking with the laptop", "Gehen mit dem Laptop")
    C["walk-cycle"]["description"] = t("Level walking at about 1.1 m/s (4 km/h), 100 steps per minute; feet planted without sliding, heel strike to toe-off with double support, the far arm swinging, the near arm holding the laptop with a small counter-swing of 2 degrees. Joint angles of the legs from two-link inverse kinematics of smooth foot paths.", "Gehen in der Ebene mit etwa 1,1 m/s (4 km/h), 100 Schritte pro Minute; die Füße stehen ohne zu rutschen, Fersenaufsatz bis Zehenablösung mit Doppelstützphase, der ferne Arm schwingt, der nahe Arm hält den Laptop mit einem kleinen Gegenschwung von 2 Grad. Beinwinkel aus zweigliedriger inverser Kinematik glatter Fußbahnen.")
    C["idle-breathing"]["title"] = t("Waiting for the lift", "Warten auf den Aufzug")
    C["idle-breathing"]["description"] = t("Standing in the lobby with the laptop under the arm, the resting breathing pattern (13 per minute) and slight chest motion derived from the pattern.", "Stehen in der Eingangshalle mit dem Laptop unter dem Arm, ruhiges Atemmuster (13 pro Minute) und leichte, daraus abgeleitete Brustbewegung.")
    for sc, rep in a["representations"].items():
        mm = round(h * 1000 / int(sc), 1)
        en, de = SIL[sc]
        dec = rep["decisions"]
        dec["silhouette"] = t(en.format(mm=mm), de.format(mm=str(mm).replace(".", ",")))
        dec["retained"], dec["merged"], dec["omitted"], dec["thinElements"] = RET[sc], MER[sc], OMI[sc], THIN[sc]
        dec["hatching"] = HATCH.get(sc, scrub(dec["hatching"], DROP))
        dec["styles"] = {k: f"{scrub(dec['styles'], DROP)[k].replace('0.13 mm', '0.18 mm').replace('0,13 mm', '0,18 mm') if sc == '500' else scrub(dec['styles'], DROP)[k]} {STY_ADD[k]}" for k in ("en", "de")}
        dec["overlays"] = {k: f"{scrub(dec['overlays'], DROP)[k]} {OVER_ADD[k]}" for k in ("en", "de")}
    for sc, vol in a["volumes"].items():
        ds = [scrub({k: fix(d[k], k) for k in ("en", "de")}, DROP) for d in vol["decisions"]]
        th_i = next((i for i, d in enumerate(ds) if "hermal" in d["en"]), None)
        st_i = next((i for i, d in enumerate(ds) if "tructure" in d["en"]), len(ds) - 1)
        for i, kind in ((0, "body"), (th_i, "thermal"), (st_i, "structure")):
            if i is not None:
                ds[i] = {k: f"{ds[i][k]} {VOL[kind][k]}" for k in ("en", "de")}
        vol["decisions"] = ds
