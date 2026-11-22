"""🧒️ Generator of `👤️humans/parent-with-child-on-shoulders`: a 1.80 m parent carrying a 30-month child astride the shoulders in a crowd.

Derived from `parent-carrying-toddler` (rig, outlines, the four handcrafted volumes of parent and toddler, the toddler as a passenger) by a
similarity of 1.80/1.70 about the ground origin (the toddler grows with it to 0.92 m, a child of about 30 months after WHO 2006); the child's
subtree is re-hung from the parent's forearm onto the thorax and moved as a rigid rest translation onto the shoulders, its thighs set 30 mm
further apart to straddle the neck; all poses, clips, texts and the crowd scenario are authored here. Run: `python3 gen_parent_with_child_on_shoulders.py`."""
import copy
import json
import math

from hw17lib import *

SLUG = "parent-with-child-on-shoulders"
STATURE, BASE_STATURE = 1.80, 1.70
S_ = STATURE / BASE_STATURE
PARENT_KG, PARENT_CLOTHES, CHILD_KG, AGE = 79.0, 2.4, 13.6, 38
THIGH_SPREAD = 0.07
SEAT = (-0.09, 0.0)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def remass(a):
    """⚖️ Parent segments rescaled to 79 kg with 2.4 kg of winter clothing (male de Leva shares kept from the base), the child's to 13.6 kg (WHO 2006, 30 months)."""
    child = set(subtree(a, "toddler-torso"))
    pk_ = sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b and b["id"] not in child)
    ck_ = sum(b["mass"]["kg"] for b in a["rig"]["bones"] if "mass" in b and b["id"] in child)
    for b in a["rig"]["bones"]:
        if "mass" in b:
            f = CHILD_KG / ck_ if b["id"] in child else (PARENT_KG + PARENT_CLOTHES) / pk_
            b["mass"]["kg"] = round(b["mass"]["kg"] * f, 4)


def rehang(a, rest):
    """🪝️ The child's subtree hangs from the thorax and sits on the shoulders at rest (a pure rest translation of the subtree)."""
    th, tt = rest["thorax"], rest["toddler-torso"]
    top = [th["o"][i] + th["u"][i] * next(b for b in a["rig"]["bones"] if b["id"] == "thorax")["lengthM"] for i in range(3)]
    o_new = [top[0] + SEAT[0], top[1] + SEAT[1], 0.0]
    d = [o_new[i] - tt["o"][i] for i in range(3)]
    shift_rest(a, "toddler-torso", d)
    bone = next(b for b in a["rig"]["bones"] if b["id"] == "toddler-torso")
    rel = [o_new[i] - th["o"][i] for i in range(3)]
    bone.update({"parent": "thorax", "x": rnd(dot(rel, th["u"])), "y": rnd(dot(rel, th["v"])), "z": rnd(dot(rel, th["w"])),
                 "angleDeg": round(math.degrees(math.atan2(dot(tt["u"], th["v"]), dot(tt["u"], th["u"]))), 4)})
    bone["joint"] = {"type": "revolute", "minDeg": -10, "maxDeg": 110}
    for side, sg in (("l", 1), ("r", -1)):
        shift_rest(a, f"toddler-thigh-{side}", (0, 0, sg * THIGH_SPREAD))
        tb = next(b for b in a["rig"]["bones"] if b["id"] == f"toddler-thigh-{side}")
        tb["z"] = rnd(tb.get("z", 0) + sg * THIGH_SPREAD)
        tb["joint"] = {"type": "revolute", "minDeg": -20, "maxDeg": 130}
    return d


CHILD = {"toddler-torso": {"angleDeg": 95.5}, "toddler-neck": {"angleDeg": -2}, "toddler-head": {"angleDeg": -3},
         "toddler-thigh-l": {"angleDeg": 82, "abductionDeg": 14}, "toddler-thigh-r": {"angleDeg": 82, "abductionDeg": -14},
         "toddler-shank-l": {"angleDeg": -40}, "toddler-shank-r": {"angleDeg": -40}, "toddler-foot-l": {"angleDeg": 4}, "toddler-foot-r": {"angleDeg": 4}}
SPINE = {"lumbar": {"angleDeg": 1.5}, "thorax": {"angleDeg": 1.0}, "neck": {"angleDeg": -1}, "head": {"angleDeg": 0}}
HANDS = {"fingers-l": {"abductionDeg": -30}, "fingertips-l": {"abductionDeg": -35}, "thumb-l": {"angleDeg": -8},
         "fingers-r": {"abductionDeg": 30}, "fingertips-r": {"abductionDeg": 35}, "thumb-r": {"angleDeg": -6}}
SOLVED = {}
LIMB_SOLIDS = True


def solve_holds(doc):
    """🤝️ The parent's hands close round the child's ankles; the child's mittens rest on the parent's beanie (Gauss-Newton through the library kinematics)."""
    d = copy.deepcopy(doc)
    for p in d["asset"]["poses"]:
        p["bones"].update({**SPINE, **CHILD, **HANDS})
    path = f"{GEN}/pwc-hold.json"
    json.dump(d, open(path, "w"))
    fr = frames(path, ["standing-carry"])["standing-carry"]["bones"]
    along = lambda b, f: [fr[b]["o"][i] + f * (fr[b]["tip"][i] - fr[b]["o"][i]) for i in range(3)]
    head = fr["head"]
    out = {}
    for side, sg in (("l", 1), ("r", -1)):
        tgt = along(f"toddler-shank-{side}", 0.85)
        mk = lambda x, side=side, sg=sg: {f"upper-arm-{side}": {"angleDeg": round(x[0], 2), "abductionDeg": round(sg * -8, 2), "twistDeg": round(sg * max(-85, min(85, x[3])), 2)}, f"forearm-{side}": {"angleDeg": round(min(x[1], 146), 2)}, f"hand-{side}": {"angleDeg": round(x[2], 2)}}
        rs = lambda st, side=side, tgt=tgt: [st["anchors"][f"grip-{side}"][i] - tgt[i] for i in range(3)]
        x, r = solve_pose(d, "standing-carry", mk, rs, [20, 140, 0, 30], h=0.5, iters=10)
        print(f"[DEBUG] grip {side}", [round(v, 2) for v in x], [round(v * 1000, 1) for v in r])
        out.update(mk(x))
        top = [head["o"][i] + 0.85 * (head["tip"][i] - head["o"][i]) for i in range(3)]
        tg2 = [top[0] + 0.03, top[1], sg * 0.06]
        mk2 = lambda x, side=side: {f"toddler-arm-{side}": {"angleDeg": round(x[0], 2), "abductionDeg": round(x[1], 2)}, f"toddler-forearm-{side}": {"angleDeg": round(x[2], 2)}, f"toddler-hand-{side}": {"angleDeg": 10}}
        rs2 = lambda st, side=side, tg2=tg2: [st["bones"][f"toddler-hand-{side}"]["o"][i] - tg2[i] for i in range(3)]
        x, r = solve_pose(d, "standing-carry", mk2, rs2, [40, 0, 60], h=0.5, iters=8)
        print(f"[DEBUG] child hand {side}", [round(v, 2) for v in x], [round(v * 1000, 1) for v in r])
        out.update(mk2(x))
    return out


def binding(met, breathing, child_met, child_breathing, sun=0.2):
    """🌡️ Pose binding outdoors in a winter crowd: the child's thighs cover the scarf and coat shoulders, the parent's gloves hold the child's boots."""
    r = {"face": {"exposedFraction": 1, "sunlitFraction": sun}, "beanie": {"exposedFraction": 0.75, "sunlitFraction": 0.1, "contact": {"fraction": 0.2, "tempC": 31}},
         "scarf": {"exposedFraction": 0.6, "sunlitFraction": sun, "contact": {"fraction": 0.3, "tempC": 31}}, "coat-torso": {"exposedFraction": 0.9, "sunlitFraction": sun, "contact": {"fraction": 0.06, "tempC": 30}},
         "coat-sleeves": {"exposedFraction": 1, "sunlitFraction": sun}, "gloves": {"exposedFraction": 0.7, "sunlitFraction": sun, "contact": {"fraction": 0.3, "tempC": 20}},
         "jeans": {"exposedFraction": 0.95, "sunlitFraction": sun}, "boots": {"exposedFraction": 0.85, "sunlitFraction": 0.09, "contact": {"fraction": 0.15, "tempC": 6}},
         "toddler-face": {"exposedFraction": 1, "sunlitFraction": sun + 0.1}, "toddler-hat": {"exposedFraction": 1, "sunlitFraction": sun + 0.15},
         "toddler-suit": {"exposedFraction": 0.82, "sunlitFraction": sun, "contact": {"fraction": 0.12, "tempC": 31}}, "toddler-mittens": {"exposedFraction": 0.8, "sunlitFraction": sun, "contact": {"fraction": 0.2, "tempC": 28}},
         "toddler-boots": {"exposedFraction": 0.7, "sunlitFraction": sun, "contact": {"fraction": 0.3, "tempC": 22}}}
    return {"activityMet": met, "breathing": breathing, "passengers": {"toddler": {"activityMet": child_met, "breathing": child_breathing}}, "regions": r, "emitters": {"plume": 0.5, "toddler-plume": 0.4}, "operating": {}}


def poses(base_poses):
    """🧍️ Standing with the child on the shoulders, the child waving, and the walking key poses of the base gait with the hold."""
    hold = {**HANDS, **SOLVED}
    stand = {**dict(next(p for p in base_poses if p["id"] == "standing-carry")["bones"]), **SPINE, **CHILD, **hold}
    four = ["heel-l", "contact-l", "heel-r", "contact-r"]
    out = [{"id": "shoulder-carry", "title": tx("Child on the shoulders", "Kind auf den Schultern"),
            "description": tx("Standing at hip width in the crowd, the 30-month child sitting astride the neck with its thighs over the shoulders; both hands hold the child's ankles in front of the chest, the child's mittens rest on the parent's beanie.",
                              "In Hüftbreite in der Menge stehend, das 30 Monate alte Kind rittlings im Nacken, die Oberschenkel über den Schultern; beide Hände halten die Knöchel des Kindes vor der Brust, die Fäustlinge des Kindes ruhen auf der Mütze des Elternteils."),
            "bones": stand, "ground": "auto", "contacts": four, "thermal": binding(1.6, "rest", 1.1, "rest"), "loads": []}]
    wave = {**stand, "toddler-arm-r": {"angleDeg": 150, "abductionDeg": 0}, "toddler-forearm-r": {"angleDeg": 25}, "toddler-hand-r": {"angleDeg": 0}, "toddler-head": {"angleDeg": 1}, "toddler-torso": {"angleDeg": 97.5}}
    out.append({"id": "child-waving", "title": tx("Child waving", "Kind winkt"),
                "description": tx("The child lifts its right arm high and waves to the parade, the left mitten stays on the parent's head; the parent stands still and holds the ankles.",
                                  "Das Kind hebt den rechten Arm hoch und winkt dem Umzug, der linke Fäustling bleibt auf dem Kopf des Elternteils; das Elternteil steht still und hält die Knöchel."),
                "bones": wave, "ground": "auto", "contacts": four, "thermal": binding(1.6, "rest", 1.5, "excited"), "loads": []})
    wave2 = {**wave, "toddler-forearm-r": {"angleDeg": 65}, "toddler-hand-r": {"angleDeg": 20}}
    out.append({"id": "child-waving-back", "key": True, "title": tx("Child waving, hand folded", "Kind winkt, Hand eingeklappt"),
                "description": tx("The other end of the wave: the child's forearm folds towards its hat.", "Das andere Ende des Winkens: Der Unterarm des Kindes klappt zur Mütze."),
                "bones": wave2, "ground": "auto", "contacts": four, "thermal": binding(1.6, "rest", 1.5, "excited"), "loads": []})
    for bp in base_poses:
        if bp["id"] in ("standing-carry", "hoisting"):
            continue
        bones = {k: v for k, v in bp["bones"].items() if k.split("-")[0] in ("lumbar", "thorax", "neck", "head", "thigh", "shank", "foot", "toe")}
        bones = {**stand, **bones}
        bones["toddler-torso"] = {"angleDeg": 95.5 + (bp["bones"].get("toddler-torso", {}).get("angleDeg", -2) + 2) * 0.8}
        p = {"id": bp["id"], "title": bp["title"], "description": bp["description"]}
        if bp.get("key"):
            p["key"] = True
        p.update({"bones": bones, "ground": "auto", "contacts": bp["contacts"], "thermal": binding(3.2, "walk", 1.2, "rest"), "loads": []})
        out.append(p)
    hs = next(p for p in out if p["id"] == "heel-strike-l")
    hs["title"] = tx("Walking, left heel strike", "Gehen, linker Fersenaufsatz")
    hs["description"] = tx("A walking step through the crowd with the child on the shoulders: the left heel strikes, the right foot pushes off, the hands keep their hold on the ankles.",
                           "Ein Schritt durch die Menge mit dem Kind auf den Schultern: Die linke Ferse setzt auf, der rechte Fuß stößt ab, die Hände halten die Knöchel fest.")
    return out


def clips(base_clips):
    """🎞️ The walking loop of the base gait with the child on the shoulders, and a waving loop."""
    walk = copy.deepcopy(next(c for c in base_clips if c["id"] == "walk-carrying"))
    walk.update({"id": "walk-with-child-on-shoulders", "title": tx("Walking with the child on the shoulders", "Gehen mit dem Kind auf den Schultern"),
                 "description": tx("A walking cycle of one second at a slow 1.18 m per stride through a crowd: heel strike, foot flat and push-off with planted feet, the parent's hands holding the child's ankles; the child rides along with a small nod.",
                                   "Ein Gehzyklus von einer Sekunde mit langsamen 1,18 m je Doppelschritt durch eine Menge: Fersenaufsatz, Fußsohle und Abstoß mit festen Füßen, die Hände des Elternteils halten die Knöchel des Kindes; das Kind reitet mit einem kleinen Nicken mit.")})
    walk["tracks"] = [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 1, "v": 1}]}, {"target": "activityMet", "ease": "linear", "keys": [{"t": 0, "v": 3.2}, {"t": 1, "v": 3.2}]}]
    four = ("heel-l", "contact-l", "heel-r", "contact-r")
    wave = {"id": "wave-to-the-parade", "title": tx("Waving to the parade", "Dem Umzug zuwinken"),
            "description": tx("A two-second loop: the child waves with the raised right arm, the forearm swinging between straight and folded, the parent sways a little and keeps the hold.",
                              "Eine Schleife von zwei Sekunden: Das Kind winkt mit dem gehobenen rechten Arm, der Unterarm schwingt zwischen gestreckt und eingeklappt, das Elternteil wiegt sich ein wenig und hält fest."),
            "durationS": 2, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "child-waving", "ease": "smooth"}, {"t": 0.5, "pose": "child-waving-back", "ease": "smooth", "bones": {"thorax": {"angleDeg": 1.8}}},
                     {"t": 1.0, "pose": "child-waving", "ease": "smooth"}, {"t": 1.5, "pose": "child-waving-back", "ease": "smooth", "bones": {"thorax": {"angleDeg": 0.4}}},
                     {"t": 2.0, "pose": "child-waving", "ease": "smooth"}],
            "contacts": [{"anchor": k, "fromS": 0, "toS": 2} for k in four],
            "phases": [{"id": "wave", "fromS": 0, "toS": 2, "title": tx("Waving", "Winken")}],
            "events": [{"t": 0, "kind": "motion", "title": tx("Wave starts", "Winken beginnt")}],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 2, "v": 0}]}]}
    return [walk, wave]


def seat_links(a):
    """🦴️ Structure: per scale one member from the child's pelvis down to the parent's cervical spine (the load path of the seat on the shoulders), placed in the default pose."""
    path = f"{GEN}/pwc-seat.json"
    json.dump({"asset": a}, open(path, "w"))
    st = frames(path, ["shoulder-carry"])["shoulder-carry"]["bones"]
    tt, nk = st["toddler-torso"], st["neck"]
    for key, vol in a["volumes"].items():
        if vol["status"] != "authored":
            continue
        cand = [s for s in vol["structure"] if s.get("bone") == "neck" and s.get("part") == "member" and s["kind"] in ("capsule", "ellipsoid")]
        c = cand[0]
        loc = [(c["p0"][i] + c["p1"][i]) / 2 for i in range(3)] if c["kind"] == "capsule" else (c.get("transform") or {}).get("translation", [0, 0, 0])
        wpt = [nk["o"][i] + loc[0] * nk["u"][i] + loc[1] * nk["v"][i] + loc[2] * nk["w"][i] for i in range(3)]
        rel = [wpt[i] - tt["o"][i] for i in range(3)]
        p1 = [round(dot(rel, tt["u"]), 5), round(dot(rel, tt["v"]), 5), round(dot(rel, tt["w"]), 5)]
        r = {"50": 0.008, "100": 0.01, "200": 0.016, "500": 0.038}[key]
        vol["structure"].append({"id": "child-seat-link", "kind": "capsule", "bone": "toddler-torso", "material": "bone", "role": "body", "tone": 0.85,
                                 "p0": [0.0, 0.0, 0.0], "p1": p1, "r0": r, "r1": r, "member": "toddler-torso", "part": "member"})


def thermal(a):
    """🌡️ Texts of the passenger and the winter crowd case."""
    t = a["thermal"]
    t["environment"] = {"airTempC": 4, "meanRadiantTempC": 5, "skyTempC": -6, "groundTempC": 3, "relativeHumidity": 0.8, "windMS": 1.0, "windDirectionDeg": 180, "solarWM2": 150, "diffuseFraction": 0.6, "pressurePa": 101325}
    t["baseMet"] = 1.6
    skin = round(dubois(PARENT_KG, STATURE), 3)
    bmr = (10 * PARENT_KG + 6.25 * STATURE * 100 - 5 * AGE + 5) * 4184 / 86400
    t["body"].update({"massKg": PARENT_KG, "skinAreaM2": skin, "basalMetabolismW": round(bmr, 1), "source": "authored-estimate", "reference": "mifflin-1990"})
    ps = t["passengers"][0]
    ps["title"] = tx("Child (30 months) on the shoulders", "Kind (30 Monate) auf den Schultern")
    cskin = round(dubois(CHILD_KG, 0.915), 3)
    ps["physiology"].update({"massKg": CHILD_KG, "skinAreaM2": cskin, "basalMetabolismW": 35.5})
    pats = ps["respiration"]["patterns"]
    pats[:] = [p for p in pats if p["id"] == "rest"] + [{"id": "excited", "title": tx("Child excited, calling out", "Kind aufgeregt, ruft"), "rateBpm": 34, "tidalVolumeL": 0.13, "inhaleFraction": 0.3, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.64, "pauseAfterExhaleFraction": 0.04, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "who-2006"}]
    t["assumptions"] = [
        tx(f"DuBois skin area {skin} m2 for the parent ({STATURE} m, {PARENT_KG:.0f} kg) and {cskin} m2 for the child (0.915 m, {CHILD_KG} kg; DuBois is less accurate for small children). Parent: sweater under a wool coat, jeans, boots, scarf, beanie and leather gloves; child: padded overall, knit hat, mittens and rubber boots; garment values are authored estimates guided by ISO 9920 (1 clo = 0.155 m2K/W).",
           f"DuBois-Hautfläche {skin} m2 für das Elternteil ({STATURE} m, {PARENT_KG:.0f} kg) und {cskin} m2 für das Kind (0,915 m, {CHILD_KG} kg; DuBois ist für kleine Kinder ungenauer). Elternteil: Pullover unter einem Wollmantel, Jeans, Stiefel, Schal, Mütze und Lederhandschuhe; Kind: gefütterter Overall, Strickmütze, Fäustlinge und Gummistiefel; die Kleidungswerte sind an ISO 9920 orientierte Schätzungen (1 clo = 0,155 m2K/W)."),
        tx("Carrying 13.6 kg on the shoulders while standing is taken as 1.6 met and walking slowly with it as 3.2 met (Ainsworth et al. 2011, carrying a child, light effort); the child sits at 1.1 met and 1.5 met while waving. The child's thighs lie on the scarf and the coat shoulders and its seat on the back of the neck, which the bindings state as contact fractions at 31 degrees C.",
           "Das Tragen von 13,6 kg auf den Schultern im Stehen gilt als 1,6 met, langsames Gehen damit als 3,2 met (Ainsworth et al. 2011, Kind tragen, leichte Anstrengung); das Kind sitzt mit 1,1 met und 1,5 met beim Winken. Die Oberschenkel des Kindes liegen auf dem Schal und den Mantelschultern und sein Gesäß im Nacken, was die Bindungen als Kontaktanteile bei 31 Grad C angeben."),
        tx("Reference case: a winter street parade in the afternoon, 4 degrees C, 80 % humidity, 1 m/s wind, 150 W/m2 of low sun and a clear sky at -6 degrees C.",
           "Referenzfall: ein winterlicher Straßenumzug am Nachmittag, 4 Grad C, 80 % Feuchte, 1 m/s Wind, 150 W/m2 tiefe Sonne und klarer Himmel bei -6 Grad C.")]


def texts(a):
    """🗣️ Title, summary, tags, provenance, anchors."""
    a["id"] = SLUG
    a["title"] = tx("Parent with a Child on the Shoulders", "Elternteil mit Kind auf den Schultern")
    a["summary"] = tx(
        "A parent of 1.80 m in a wool coat, scarf, beanie, jeans, leather gloves and boots carries a child of 30 months (0.92 m, 13.6 kg) in a padded overall, knit hat, mittens and rubber boots astride the shoulders to see over a crowd: the child is a second living body with its own breathing, its seat bears on the back of the parent's neck, the parent holds its ankles in front of the chest. Poses: standing with the child on the shoulders, the child waving, a walking step; loops: walking through the crowd and waving to the parade.",
        "Ein 1,80 m großes Elternteil in Wollmantel, Schal, Mütze, Jeans, Lederhandschuhen und Stiefeln trägt ein 30 Monate altes Kind (0,92 m, 13,6 kg) in gefüttertem Overall, Strickmütze, Fäustlingen und Gummistiefeln rittlings auf den Schultern, damit es über die Menge sieht: Das Kind ist ein zweiter lebender Körper mit eigener Atmung, sein Gesäß lastet im Nacken des Elternteils, das Elternteil hält seine Knöchel vor der Brust. Posen: Stehen mit dem Kind auf den Schultern, das Kind winkt, ein Gehschritt; Schleifen: durch die Menge gehen und dem Umzug zuwinken.")
    a["tags"] = ["human", "adult", "child", "toddler", "passenger", "carrying", "shoulders", "crowd", "parade", "winter", "walking"]
    pv = a["provenance"]
    pv.update({"author": "Semio staffage human worker HW-17", "method": "handcrafted", "created": "2026-11-22"})
    pv["notes"] = tx(
        "Rig, outlines and the four handcrafted volumes of parent and child are derived from the archive's parent-carrying-toddler: every length is scaled by 1.80/1.70 about the ground origin, so the parent stands 1.80 m barefoot and the child grows to 0.915 m, the WHO (2006) median of about 30 months; segment masses are rescaled to 79 kg plus 2.4 kg of clothing and to 13.6 kg. The child's subtree hangs from the parent's thorax instead of the forearm: its rest placement is moved as one rigid translation onto the shoulders (front and plan drawings moved with it, bone-local elevations and volumes unchanged), its hip joints set 30 mm further apart to straddle the neck. Poses, holds (solved through the library's kinematics), clips, texts and the winter parade are authored for this pair.",
        "Skelett, Umrisse und die vier handgefertigten Volumen von Elternteil und Kind sind aus dem Archivobjekt parent-carrying-toddler abgeleitet: Jede Länge ist um 1,80/1,70 um den Bodenursprung skaliert, so steht das Elternteil barfuß 1,80 m groß und das Kind wächst auf 0,915 m, den WHO-Median (2006) von etwa 30 Monaten; die Segmentmassen sind auf 79 kg plus 2,4 kg Kleidung und auf 13,6 kg umgerechnet. Der Teilbaum des Kindes hängt am Brustkorb statt am Unterarm des Elternteils: Seine Ruhelage ist als eine starre Verschiebung auf die Schultern versetzt (Vorder- und Draufsichten mitverschoben, knochenlokale Seitenansichten und Volumen unverändert), seine Hüftgelenke 30 mm weiter auseinander, um den Hals zu umfassen. Posen, Griffe (über die Kinematik der Bibliothek gelöst), Clips, Texte und der Winterumzug sind für dieses Paar geschaffen.")
    for an in a["anchors"]:
        if an["id"] == "toddler-seat":
            an["title"] = tx("Child's seat on the back of the parent's neck", "Gesäß des Kindes im Nacken des Elternteils")


def trunk_behind_head(a):
    """🙆️ The child's trunk sits behind the parent's head: in the side and front elevations it is painted just under the head shapes (the base carried the toddler in front of the chest, where its trunk lay on top)."""
    for rep in a["representations"].values():
        for view in ("elevation", "front"):
            g = rep["geometry"][view]
            head = [s["z"] for s in g if s["bone"] == "head" and s.get("filled")]
            if not head:
                continue
            trunk = sorted((s for s in g if s["bone"] == "toddler-torso"), key=lambda s: s["z"])
            for i, s in enumerate(trunk):
                s["z"] = round(min(head) - 0.004 + 0.002 * i / max(1, len(trunk)), 4)
            g.sort(key=lambda s: s["z"])


def main(probe=None):
    doc = load("parent-carrying-toddler")
    a = doc["asset"]
    scale_asset(a, S_, 1.0)
    remass(a)
    tmp = f"{GEN}/pwc-rest.json"
    json.dump(doc, open(tmp, "w"))
    rest = frames(tmp)["rest"]
    d = rehang(a, rest)
    print("[DEBUG] child moved by", [round(v, 3) for v in d])
    if probe:
        for p in a["poses"]:
            p["bones"].update(probe)
        json.dump(doc, open(tmp, "w"))
        return doc
    json.dump(doc, open(tmp, "w"))
    rest2 = frames(tmp)["rest"]
    limbs = [b for b in subtree(a, "toddler-torso") if b not in ("toddler-torso", "toddler-neck", "toddler-head")]
    extrude_views(a, [b for b in limbs if LIMB_SOLIDS] + ["upper-arm-r", "forearm-r"], rest2)
    for view in ("elevation", "front", "plan"):
        hat = next(x for x in a["representations"]["500"]["geometry"][view] if x["id"] == "toddler-hat")
        pom = copy.deepcopy(next(x for x in a["representations"]["200"]["geometry"][view] if x["id"] == "toddler-pompom"))
        pom.update({k: hat[k] for k in ("tone", "region") if k in hat})
        pom.pop("penMm", None)
        a["representations"]["500"]["geometry"][view].append(pom)
    SOLVED.update(solve_holds(doc))
    base_poses, base_clips = a["poses"], a["clips"]
    a["poses"] = poses(base_poses)
    a["clips"] = clips(base_clips)
    a["presentation"] = {"defaultPose": "shoulder-carry", "defaultClip": "wave-to-the-parade", "defaultScale": 100}
    texts(a)
    thermal(a)
    seat_links(a)
    a["structure"]["assumptions"] = [
        tx("Inertial model: segment masses as point masses at the scaled de Leva centres for the parent and the scaled WHO centres for the child; the child's 13.6 kg bear on the back of the parent's neck and shoulders (one member from the child's pelvis to the cervical spine carries the seat) and are steadied by the hands at the ankles; heel and ball contacts are unilateral supports.",
           "Trägheitsmodell: Segmentmassen als Punktmassen in den skalierten Schwerpunkten nach de Leva für das Elternteil und den skalierten WHO-Schwerpunkten für das Kind; die 13,6 kg des Kindes lasten im Nacken und auf den Schultern des Elternteils (ein Glied vom Becken des Kindes zur Halswirbelsäule trägt das Gesäß) und werden von den Händen an den Knöcheln gehalten; Fersen- und Ballenkontakte sind einseitige Auflager.")]
    a["dimensions"]["massKg"] = round(mass_sum(a), 2)
    trunk_behind_head(a)
    write(SLUG, doc)
    fit_spans(SLUG, "shoulder-carry", [(k, f"sleeve-upper-{sd}", f"sleeve-upper-{sd}") for k in ("50", "100", "200", "500") for sd in ("r",)])
    w, h, dd = volume_dims(SLUG, "shoulder-carry", 50)
    doc = load(SLUG)
    doc["asset"]["dimensions"].update({"widthM": round(w, 3), "heightM": round(h, 3), "depthM": round(dd, 3)})
    write(SLUG, doc)
    print("[DEBUG] written", SLUG, doc["asset"]["dimensions"])


if __name__ == "__main__":
    main()
