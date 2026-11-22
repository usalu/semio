"""🧶️ Poses, clips, physics and every text of `elderly-person-knitting` (English first, German second)."""
import copy
import math
import hw06lib as L
import knit_base as K

t = L.t


def _pose(src, pid, title, description, bones=None, met=None, breathing=None, contacts=None, key=None):
    p = copy.deepcopy(src)
    p["id"] = pid; p["title"] = title; p["description"] = description
    for b, off in (bones or {}).items():
        p["bones"][b] = off
    if met is not None: p["thermal"]["activityMet"] = met
    if breathing: p["thermal"]["breathing"] = breathing
    if contacts is not None: p["contacts"] = contacts
    if key is not None:
        if key: p["key"] = True
        else: p.pop("key", None)
    return p


def poses(asset):
    P = {p["id"]: p for p in asset["poses"]}
    up = P["seated-upright"]["bones"]
    hunch = {"lumbar": {"angleDeg": -5.0}, "torso": {"angleDeg": -7.0}, "neck": {"angleDeg": -4.0}, "head": {"angleDeg": -16.0}}
    knitting = _pose(P["seated-upright"], "knitting", t("Knitting", "Stricken"),
                     t("Upright on the bench a hand's breadth off the backrest with the rounded upper back of her age, head bowed over the work: the left hand holds the needle with the stitches, the right needle crosses it from the front, the work hangs over her lap.",
                       "Aufrecht auf der Bank, eine Handbreit vor der Lehne, mit dem altersgerundeten oberen Rücken, den Kopf über die Arbeit geneigt: Die linke Hand hält die Nadel mit den Maschen, die rechte Nadel kreuzt sie von vorn, das Strickstück hängt über ihrem Schoß."),
                     hunch, met=1.2, breathing="knitting-rest")
    inspect = _pose(P["reclined-reading"], "inspecting-work", t("Holding up the work", "Strickstück prüfen"),
                    t("Leaning back against the back slats, both needles raised to eye level to count the rows through her reading glasses.",
                      "An die Rückenlatten gelehnt, beide Nadeln auf Augenhöhe gehoben, um durch die Lesebrille die Reihen zu zählen."),
                    {"neck": {"angleDeg": -6.0}, "head": {"angleDeg": -4.0}}, met=1.1, breathing="knitting-rest")
    reach = _pose(P["reaching-for-cup"], "reaching-for-yarn", t("Reaching for the yarn", "Nach der Wolle greifen"),
                  t("The work lowered onto the left thigh in the left hand, the right hand reaching down beside her to pull more yarn from the ball on the seat.",
                    "Das Strickstück in der linken Hand auf den linken Oberschenkel gesenkt, die rechte Hand greift neben sich, um vom Knäuel auf der Sitzfläche Wolle nachzuziehen."),
                  {"lumbar": {"angleDeg": -6.5}, "torso": {"angleDeg": -4.0}}, met=1.3, breathing="knitting-rest")
    rest_arms = {"forearm-l": dict(up["forearm-l"], angleDeg=66.0), "forearm-r": dict(up["forearm-r"], angleDeg=66.0),
                 "hand-l": dict(up["hand-l"], angleDeg=6.0), "hand-r": dict(up["hand-r"], angleDeg=6.0),
                 "arm-l": dict(up["arm-l"], angleDeg=4.0), "arm-r": dict(up["arm-r"], angleDeg=4.0),
                 "lumbar": {"angleDeg": -4.0}, "torso": {"angleDeg": -6.0}, "neck": {"angleDeg": -2.0}, "head": {"angleDeg": -6.0}}
    resting = _pose(P["seated-upright"], "resting-hands-in-lap", t("Resting, hands in the lap", "Ausruhen, Hände im Schoß"),
                    t("A pause between rows: forearms lowered, both hands with the needles resting on the thighs, head raised to look around.",
                      "Eine Pause zwischen den Reihen: Unterarme gesenkt, beide Hände mit den Nadeln auf den Oberschenkeln, der Kopf gehoben, sie schaut sich um."),
                    rest_arms, met=1.0, breathing="knitting-rest")
    lean = _pose(P["leaning-forward"], "leaning-forward", P["leaning-forward"]["title"],
                 t("Slid to the front edge with the weight shifted forward over the feet, ready to stand up, the knitting gathered in both hands above the thighs.",
                   "An die Vorderkante gerutscht, das Gewicht nach vorn über die Füße verlagert, bereit zum Aufstehen, das Strickzeug in beiden Händen über den Oberschenkeln."),
                 met=1.8, breathing="light-activity")
    rising = _pose(P["rising"], "rising", P["rising"]["title"],
                   t("Pelvis lifted off the seat, trunk still inclined, the legs extending slowly over the planted feet.",
                     "Becken von der Sitzfläche abgehoben, Rumpf noch geneigt, die Beine strecken sich langsam über den stehenden Füßen."),
                   met=2.4, breathing="light-activity")
    standing = _pose(P["standing"], "standing", P["standing"]["title"],
                     t("Upright on both feet in front of the bench, the knitting held in both hands at waist height.",
                       "Aufrecht auf beiden Füßen vor der Bank, das Strickzeug in beiden Händen auf Hüfthöhe."),
                     {"lumbar": {"angleDeg": -4.0}, "torso": {"angleDeg": -5.0}}, met=1.4, breathing="standing-rest")
    return [knitting, inspect, reach, resting, lean, rising, standing]


def hang_work(asset):
    """Every pose lets the work hang from the left needle: the offset of the `work` bone turns it back to the vertical plane
    through the needle, leaning 19 degrees forward over the lap as in the knitting pose."""
    bones = asset["rig"]["bones"]
    work = next(b for b in bones if b["id"] == "work")
    rest_m = [[c[i] for c in L.turned(([1, 0, 0], [0, 1, 0], [0, 0, 1]), work["angleDeg"], work["abductionDeg"], work["twistDeg"])] for i in range(3)]
    from knit_build import frame_from

    def offset(pose_bones):
        b = {k: v for k, v in pose_bones.items() if k != "work"}
        fr = L.chain(bones, b)
        nl = fr["needle-l"]; d = nl["u"]
        target = frame_from([0.35, -1.0, 0.0], up=d)
        m = L.mmul(L.mT(rest_m), L.mmul(L.mT(L.mat(nl)), target))
        a, ab, tw = L.euler_of(m)
        return {"angleDeg": round(a, 3), "abductionDeg": round(ab, 3), "twistDeg": round(tw, 3)}
    for p in asset["poses"]:
        p["bones"]["work"] = offset(p["bones"])
    for c in asset["clips"]:
        P = {p["id"]: p for p in asset["poses"]}
        for key in c["keys"]:
            if "bones" in key and key["bones"]:
                merged = dict(P[key["pose"]]["bones"]); merged.update(key["bones"])
                key["bones"]["work"] = offset(merged)


def knit_clip(asset):
    """Two stitches of 1.2 s: insert the right needle into the front loop, wrap the yarn round its tip, draw the new loop through,
    slip the old stitch off the left needle; the left hand feeds the next stitch towards the tip."""
    P = {p["id"]: p for p in asset["poses"]}
    base = P["knitting"]["bones"]

    def at(dl_f, dl_h, dr_f, dr_ab, dr_h, dr_tw):
        b = {}
        fl = dict(base["forearm-l"]); fl["angleDeg"] = round(fl["angleDeg"] + dl_f, 3); b["forearm-l"] = fl
        hl = dict(base["hand-l"]); hl["angleDeg"] = round(hl["angleDeg"] + dl_h, 3); b["hand-l"] = hl
        fr = dict(base["forearm-r"]); fr["angleDeg"] = round(fr["angleDeg"] + dr_f, 3); fr["abductionDeg"] = round(fr.get("abductionDeg", 0) + dr_ab, 3); b["forearm-r"] = fr
        hr = dict(base["hand-r"]); hr["angleDeg"] = round(hr["angleDeg"] + dr_h, 3); hr["twistDeg"] = round(hr.get("twistDeg", 0) + dr_tw, 3); b["hand-r"] = hr
        return b
    steps = [(0, 0, 0, 0, 0, 0), (0, 2, 3, 3, 6, 0), (0, 2, 4, 3, 9, 14), (0, 1, -3, 1, -4, 6), (2, -3, -2, 0, -2, 0)]
    keys = []
    for n in range(2):
        for i, s in enumerate(steps):
            keys.append({"t": round(n * 1.2 + i * 0.24, 3), "pose": "knitting", "ease": "smooth", "bones": at(*s)})
    keys.append({"t": 2.4, "pose": "knitting", "ease": "smooth", "bones": at(*steps[0])})
    seated = ["seat", "thigh-support", "heel-l", "ball-l", "heel-r", "ball-r"]
    phases, events = [], []
    names = [("insert", t("Needle into the stitch", "Nadel in die Masche")), ("wrap", t("Yarn round the needle", "Faden um die Nadel")),
             ("draw", t("New loop drawn through", "Neue Schlinge durchgezogen")), ("slip", t("Old stitch slipped off", "Alte Masche abgestreift"))]
    for n in range(2):
        for i, (pid, title) in enumerate(names):
            f0 = n * 1.2 + i * 0.3
            phases.append({"id": f"{pid}-{n + 1}", "fromS": round(f0, 3), "toS": round(f0 + 0.3, 3), "title": title})
        events.append({"t": round(n * 1.2 + 1.08, 3), "kind": "motion", "title": t(f"Stitch {n + 1} done", f"Masche {n + 1} fertig")})
    return {"id": "knit-stitches", "title": t("Knitting stitches", "Maschen stricken"),
            "description": t("Plain knitting at an unhurried pace of 50 stitches a minute: in each 1.2 s stitch the right needle goes into the front of the loop, the right hand wraps the yarn round its tip, draws the new loop through and slips the old stitch off while the left hand feeds the next one; two stitches make one loop of the clip.",
                             "Glatt rechts in ruhigem Tempo von 50 Maschen pro Minute: In jeder 1,2-s-Masche sticht die rechte Nadel vorn in die Schlinge, die rechte Hand legt den Faden um die Spitze, zieht die neue Schlinge durch und streift die alte Masche ab, während die linke Hand die nächste nachschiebt; zwei Maschen bilden eine Schleife des Clips."),
            "durationS": 2.4, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys,
            "contacts": [{"anchor": a, "fromS": 0, "toS": 2.4} for a in seated], "phases": phases, "events": events,
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 2.4, "v": 0}]},
                       {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.3}, {"t": 2.4, "v": 1.3}]}]}


def idle_clip():
    return {"id": "idle-breathing", "title": t("Resting between rows", "Ruhen zwischen den Reihen"),
            "description": t("Hands with the needles in her lap, quiet breathing at 16 per minute (an older adult at rest) with a slight rise of the chest and shoulders.",
                             "Die Hände mit den Nadeln im Schoß, ruhiges Atmen mit 16 Zügen pro Minute (ältere Person in Ruhe) mit leichtem Heben von Brustkorb und Schultern."),
            "durationS": 3.75, "fps": 30, "loop": True, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "resting-hands-in-lap", "ease": "smooth"},
                     {"t": 1.6, "pose": "resting-hands-in-lap", "ease": "smooth", "bones": {"clavicle-l": {"angleDeg": 1.2}, "clavicle-r": {"angleDeg": 1.2}, "head": {"angleDeg": -4.5}}},
                     {"t": 3.75, "pose": "resting-hands-in-lap", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 3.75} for a in ["seat", "thigh-support", "heel-l", "ball-l", "heel-r", "ball-r"]],
            "phases": [{"id": "pause", "fromS": 0, "toS": 3.75, "title": t("Pause in the lap", "Pause im Schoß")}], "events": [],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 3.75, "v": 0}]}]}


def stand_clip(asset):
    src = next(c for c in asset["clips"] if c["id"] == "stand-up")
    c = copy.deepcopy(src)
    k = 5.5 / 4.0
    for key in c["keys"]:
        key["t"] = round(key["t"] * k, 3)
        if key["pose"] == "seated-upright": key["pose"] = "knitting"
    for s in c["contacts"]: s["fromS"] = round(s["fromS"] * k, 3); s["toS"] = round(s["toS"] * k, 3)
    for s in c["phases"]: s["fromS"] = round(s["fromS"] * k, 3); s["toS"] = round(s["toS"] * k, 3)
    for e in c["events"]: e["t"] = round(e["t"] * k, 3)
    for tr in c["tracks"]:
        for key in tr["keys"]: key["t"] = round(key["t"] * k, 3)
    c["durationS"] = 5.5
    c["title"] = t("Standing up slowly", "Langsam aufstehen")
    c["description"] = t("Sit-to-stand at the pace of an 80-year-old (5.5 s instead of about 3 s): she slides to the front edge, leans the trunk over the feet, lifts the pelvis off the seat and rises 0.38 m while both feet stay planted, the knitting gathered in her hands; the activity rises from 1.2 to about 2.4 met and falls again.",
                         "Aufstehen im Tempo einer 80-Jährigen (5,5 s statt etwa 3 s): Sie rutscht an die Vorderkante, neigt den Rumpf über die Füße, hebt das Becken von der Sitzfläche und richtet sich 0,38 m auf, während beide Füße stehen bleiben, das Strickzeug in den Händen; die Aktivität steigt von 1,2 auf etwa 2,4 met und fällt wieder.")
    c["tracks"] = [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.2}, {"t": 1.65, "v": 2.4}, {"t": 3.85, "v": 1.9}, {"t": 5.5, "v": 1.4}]},
                   {"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 1.4, "v": 1}, {"t": 4.4, "v": 2}, {"t": 5.5, "v": 2}]}]
    return c


def inspect_clip():
    return {"id": "check-and-fetch-yarn", "title": t("Checking the work and fetching yarn", "Arbeit prüfen und Wolle holen"),
            "description": t("From knitting she leans back against the back slats and holds the work up to count the rows, sits up again and reaches beside her for the ball of wool to pull more yarn, then returns to knitting.",
                             "Aus dem Stricken lehnt sie sich an die Rückenlatten und hält das Strickstück hoch, um die Reihen zu zählen, richtet sich wieder auf, greift neben sich nach dem Wollknäuel, um Faden nachzuziehen, und strickt dann weiter."),
            "durationS": 7.0, "fps": 30, "loop": False, "rootMotion": "none",
            "keys": [{"t": 0, "pose": "knitting", "ease": "smooth"}, {"t": 1.6, "pose": "inspecting-work", "ease": "smooth"}, {"t": 3.0, "pose": "inspecting-work", "ease": "smooth"},
                     {"t": 4.4, "pose": "reaching-for-yarn", "ease": "smooth"}, {"t": 5.4, "pose": "reaching-for-yarn", "ease": "smooth"}, {"t": 7.0, "pose": "knitting", "ease": "smooth"}],
            "contacts": [{"anchor": a, "fromS": 0, "toS": 7.0} for a in ["seat", "thigh-support", "heel-l", "ball-l", "heel-r", "ball-r"]] + [{"anchor": "backrest", "fromS": 1.2, "toS": 3.4}],
            "phases": [{"id": "recline", "fromS": 0, "toS": 1.6, "title": t("Leaning back", "Zurücklehnen")}, {"id": "count", "fromS": 1.6, "toS": 3.0, "title": t("Counting the rows", "Reihen zählen")},
                       {"id": "reach", "fromS": 3.0, "toS": 5.4, "title": t("Pulling yarn from the ball", "Faden vom Knäuel ziehen")}, {"id": "resume", "fromS": 5.4, "toS": 7.0, "title": t("Back to knitting", "Zurück zum Stricken")}],
            "events": [{"t": 1.2, "kind": "contact-on", "title": t("Back touches the backrest", "Rücken berührt die Lehne"), "target": "backrest"},
                       {"t": 3.4, "kind": "contact-off", "title": t("Back leaves the backrest", "Rücken löst sich von der Lehne"), "target": "backrest"}],
            "tracks": [{"target": "breathing", "ease": "step", "keys": [{"t": 0, "v": 0}, {"t": 7.0, "v": 0}]},
                       {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.2}, {"t": 1.6, "v": 1.1}, {"t": 4.4, "v": 1.3}, {"t": 7.0, "v": 1.2}]}]}


MATERIALS = {
    "wool-cardigan": (t("Moss-stitch wool cardigan", "Strickjacke aus Wolle im Perlmuster"), 0.9, 0.55, "literature", "ashrae-2021"),
    "wool-trousers": (t("Dark wool-blend trousers", "Dunkle Hose aus Wollmischgewebe"), 0.92, 0.75, "literature", "ashrae-2021"),
    "leather-shoe": (t("Dark leather comfort shoe", "Dunkler Lederbequemschuh"), 0.93, 0.8, "authored-estimate", None),
    "wool-yarn": (t("Light wool yarn (knitted work, ball)", "Helle Wolle (Strickstück, Knäuel)"), 0.9, 0.35, "literature", "ashrae-2021"),
    "hair": (t("Grey hair", "Graues Haar"), 0.98, 0.5, "authored-estimate", None),
    "cotton-underwear": (t("Cotton underwear and fine roll-neck jumper", "Baumwollwäsche und feiner Rollkragenpullover"), 0.93, 0.5, "authored-estimate", None),
    "cotton-sock": (t("Knee-high cotton socks", "Kniestrümpfe aus Baumwolle"), 0.93, 0.6, "authored-estimate", None),
}


def materials(asset):
    for m in asset["materials"]:
        if m["id"] in MATERIALS:
            title, e, a, src, ref = MATERIALS[m["id"]]
            m["title"] = title; m["emissivity"] = e; m["solarAbsorptivity"] = a; m["source"] = src
            if ref: m["reference"] = ref
            else: m.pop("reference", None)
    asset["materials"] += [
        {"id": "aluminium-needle", "title": t("Anodised aluminium knitting needles", "Eloxierte Aluminium-Stricknadeln"), "emissivity": 0.77, "solarAbsorptivity": 0.6, "source": "literature", "reference": "ashrae-2021"},
        {"id": "glasses", "title": t("Acetate frame of the reading glasses", "Acetatfassung der Lesebrille"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "source": "authored-estimate"},
    ]


def thermal(asset):
    th = asset["thermal"]
    skin = 0.007184 * K.BODY ** 0.425 * (K.STATURE * 100) ** 0.725
    bmr = (10 * K.BODY + 6.25 * K.STATURE * 100 - 5 * 80 - 161) * 4184 / 86400
    th["body"].update({"massKg": K.BODY, "skinAreaM2": round(skin, 3), "basalMetabolismW": round(bmr, 1), "coreSetPointC": 36.7, "skinSetPointC": 33.9,
                       "maxSkinBloodFlowLM2H": 70, "source": "authored-estimate", "reference": "gagge-1971"})
    r = th["respiration"]
    r["patterns"] = [
        {"id": "knitting-rest", "title": t("Seated quiet knitting", "Ruhiges Stricken im Sitzen"), "rateBpm": 16, "tidalVolumeL": 0.42, "inhaleFraction": 0.38, "pauseAfterInhaleFraction": 0.04, "exhaleFraction": 0.45, "pauseAfterExhaleFraction": 0.13, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "light-activity", "title": t("Light activity: leaning, rising", "Leichte Aktivität: Vorbeugen, Aufstehen"), "rateBpm": 22, "tidalVolumeL": 0.7, "inhaleFraction": 0.44, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.5, "pauseAfterExhaleFraction": 0.04, "route": "nose-and-mouth", "source": "authored-estimate", "reference": "icrp-66-1994"},
        {"id": "standing-rest", "title": t("Quiet standing", "Ruhiges Stehen"), "rateBpm": 17, "tidalVolumeL": 0.45, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.02, "exhaleFraction": 0.47, "pauseAfterExhaleFraction": 0.11, "route": "nose", "source": "authored-estimate", "reference": "icrp-66-1994"}]
    R = {x["id"]: x for x in th["regions"]}
    R["hair"]["title"] = t("Grey hair and bun", "Graues Haar und Dutt")
    R["hair"]["layers"][0]["title"] = t("Thinning grey scalp hair with bun", "Dünner werdendes graues Kopfhaar mit Dutt")
    R["hair"]["layers"][0]["resistanceM2KW"] = 0.055
    c = R["cardigan"]
    c["title"] = t("Cardigan over a roll-neck jumper on torso, neck and arms", "Strickjacke über Rollkragenpullover an Rumpf, Hals und Armen")
    lay = {x["id"]: x for x in c["layers"]}
    lay["underwear"].update({"title": t("Bra, briefs, vest", "BH, Slip, Unterhemd"), "resistanceM2KW": round(0.06 * 0.155 * 1.144, 5), "vapourResistanceM2PaW": 1.5})
    lay["blouse"].update({"title": t("Fine roll-neck jumper (merino)", "Feiner Rollkragenpullover (Merino)"), "thicknessM": 0.0012, "resistanceM2KW": round(0.2 * 0.155 * 1.144, 5), "vapourResistanceM2PaW": 5.7})
    lay["cardigan"].update({"title": t("Thick moss-stitch cardigan, buttoned", "Dicke Strickjacke im Perlmuster, geknöpft"), "thicknessM": 0.007, "resistanceM2KW": round(0.36 * 0.155 * 1.144, 5), "vapourResistanceM2PaW": 10.2})
    tr = R["trousers"]
    tr["title"] = t("Wool-blend trousers over the legs", "Hose aus Wollmischgewebe über den Beinen")
    tr["layers"][0].update({"title": t("Lined wool-blend trousers", "Gefütterte Hose aus Wollmischgewebe"), "thicknessM": 0.0016, "resistanceM2KW": round(0.28 * 0.155 * 1.073, 5), "vapourResistanceM2PaW": 7.4})
    sh = R["shoes"]
    sh["title"] = t("Knee socks and leather shoes", "Kniestrümpfe und Lederschuhe")
    sl = {x["id"]: x for x in sh["layers"]}
    sl["socks"].update({"title": t("Knee-high cotton socks", "Kniestrümpfe aus Baumwolle"), "resistanceM2KW": round(0.03 * 0.155, 5), "vapourResistanceM2PaW": 0.74})
    sl["shoe"].update({"title": t("Leather comfort shoe, cushioned sole", "Lederbequemschuh mit gepolsterter Sohle"), "thicknessM": 0.004, "resistanceM2KW": round(0.04 * 0.155, 5), "vapourResistanceM2PaW": 1.9})
    k = R["knitting"]
    k.update({"title": t("Knitting: needles and work", "Strickzeug: Nadeln und Strickstück"), "material": "wool-yarn", "areaM2": 0.037, "heatCapacityJM2K": 900, "geometry": "vertical-plate", "characteristicLengthM": 0.12})
    th["regions"].append({"id": "yarn", "title": t("Ball of wool", "Wollknäuel"), "kind": "surface", "model": "passive", "material": "wool-yarn", "areaM2": 0.031,
                          "characteristicLengthM": 0.1, "geometry": "sphere", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": 1400, "source": "authored-estimate", "reference": "iso-9920-2007"})
    for p in asset["poses"]:
        p["thermal"]["regions"]["yarn"] = {"exposedFraction": 0.92, "sunlitFraction": 0.55, "contact": {"fraction": 0.08, "tempC": 15}}
        p["thermal"]["regions"]["knitting"] = {"exposedFraction": 0.9, "sunlitFraction": 0.6}
    th["assumptions"] = [
        t(f"Skin area {skin:.3f} m2 after DuBois and DuBois (1916) for {K.BODY:.0f} kg and {K.STATURE} m; basal metabolism {bmr:.1f} W after Mifflin-St Jeor (1990) for a woman of 80; skin fractions authored (rule-of-nines proportions) as for the adult figure she is derived from.",
          f"Hautfläche {skin:.3f} m2 nach DuBois und DuBois (1916) für {K.BODY:.0f} kg und {K.STATURE} m; Grundumsatz {bmr:.1f} W nach Mifflin-St Jeor (1990) für eine 80-jährige Frau; Hautanteile geschätzt (Neunerregel) wie bei der Erwachsenenfigur, von der sie abgeleitet ist.".replace(f"{skin:.3f}", f"{skin:.3f}".replace(".", ",")).replace(f"{K.STATURE} m", f"{K.STATURE} m".replace(".", ",")).replace(f"{bmr:.1f} W", f"{bmr:.1f} W".replace(".", ","))),
        t("Garment clo values follow the ISO 9920 tables as authored estimates (underwear with vest 0.06, fine roll-neck jumper 0.20, thick cardigan 0.36, lined trousers 0.28, knee socks 0.03, leather shoes 0.04 clo; about 1.0 clo in all); resistances are referred to the outer area.",
          "Die clo-Werte der Kleidungsstücke folgen den Tabellen von ISO 9920 als Schätzwerte (Unterwäsche mit Unterhemd 0,06, feiner Rollkragenpullover 0,20, dicke Strickjacke 0,36, gefütterte Hose 0,28, Kniestrümpfe 0,03, Lederschuhe 0,04 clo; insgesamt etwa 1,0 clo); Widerstände sind auf die Außenfläche bezogen."),
        t("Knitting while seated is 1.2 met (ISO 8996, seated light hand work, about 70 W/m2; the Compendium of Physical Activities lists knitting and sewing, light effort, at 1.3 MET); thermoregulation of an older body is weaker: lower core and skin set points and a maximum skin blood flow of 70 instead of 90 L/(m2 h) (authored after Kenney and Munce 2003).",
          "Stricken im Sitzen ist 1,2 met (ISO 8996, leichte Handarbeit im Sitzen, etwa 70 W/m2; das Compendium of Physical Activities nennt Stricken und Nähen, leicht, mit 1,3 MET); die Thermoregulation eines älteren Körpers ist schwächer: niedrigere Kern- und Hautsollwerte und eine maximale Hautdurchblutung von 70 statt 90 L/(m2 h) (geschätzt nach Kenney und Munce 2003)."),
        t("Breathing patterns are authored estimates for an older woman at rest (about 0.42 L at 16 per minute) and ICRP 66 light-activity ventilation; the bench, the knitting and the ball of wool are passive surfaces; the bench conducts into trousers and cardigan at the contact fractions of each pose (seat 15 degrees C).",
          "Atemmuster sind Schätzwerte für eine ältere Frau in Ruhe (etwa 0,42 L bei 16 je Minute) und die Lüftung bei leichter Aktivität nach ICRP 66; Bank, Strickzeug und Wollknäuel sind passive Oberflächen; die Bank leitet an den Kontaktanteilen jeder Haltung in Hose und Strickjacke (Sitz 15 Grad C)."),
    ]


def finish(asset, f_leg):
    materials(asset)
    thermal(asset)
    clips = [knit_clip(asset), idle_clip(), stand_clip(asset), inspect_clip()]
    asset["clips"] = clips
    hang_work(asset)
    asset["presentation"] = {"defaultPose": "knitting", "defaultClip": "knit-stitches", "defaultScale": 100}
    asset["id"] = "elderly-person-knitting"
    asset["title"] = t("Elderly Person Knitting", "Strickende ältere Person")
    asset["summary"] = t("An 80-year-old woman in a buttoned cardigan and reading glasses knits on a timber park bench: two needles in her hands, the work hanging over her lap and a ball of wool on the seat beside her; with stitch-by-stitch knitting, a pause in the lap, counting the rows and a slow sit-to-stand.",
                         "Eine 80-jährige Frau in geknöpfter Strickjacke und mit Lesebrille strickt auf einer hölzernen Parkbank: zwei Nadeln in den Händen, das Strickstück hängt über ihrem Schoß, ein Wollknäuel liegt neben ihr auf der Sitzfläche; mit Stricken Masche für Masche, einer Pause im Schoß, dem Zählen der Reihen und langsamem Aufstehen.")
    asset["tags"] = ["human", "adult", "elderly", "seated", "knitting", "bench", "female", "care"]
    provenance(asset, f_leg)
    decisions(asset)


REFERENCES = [
    {"id": "din-33402-2", "citation": "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte; body height of women aged 61 to 65 (P5 1.51 m, P50 1.61 m, P95 1.71 m)."},
    {"id": "sorkin-1999", "citation": "Sorkin, J. D., Muller, D. C. and Andres, R. (1999). Longitudinal change in height of men and women: implications for interpretation of the body mass index. American Journal of Epidemiology 150(9), 969-977."},
    {"id": "ainsworth-2011", "citation": "Ainsworth, B. E. et al. (2011). 2011 Compendium of Physical Activities: a second update of codes and MET values. Medicine and Science in Sports and Exercise 43(8), 1575-1581; knitting, sewing, light effort, sitting: 1.3 MET."},
    {"id": "kenney-munce-2003", "citation": "Kenney, W. L. and Munce, T. A. (2003). Invited review: aging and human temperature regulation. Journal of Applied Physiology 95(6), 2598-2603."},
    {"id": "knitting-needle-catalogues", "citation": "Straight knitting needle and yarn catalogues (aluminium needles 4 mm x 30-35 cm, about 12 g a pair needle; 100 g wool balls about 9-10 cm across); authored-estimate."},
]


def provenance(asset, f_leg):
    pr = asset["provenance"]
    pr["author"] = "Semio staffage human worker HW-06"
    pr["created"] = "2026-11-22"
    pr["method"] = "handcrafted"
    pr["references"] = pr["references"] + REFERENCES
    pr["notes"] = [
        t(f"An 80-year-old woman of {K.STATURE:.2f} m and {K.BODY:.0f} kg: stature after DIN 33402-2 (women 61 to 65, P50 1.61 m) minus the age loss of about 1 cm per decade after 65 (Sorkin et al. 1999), which puts her near the 55th percentile of her age; BMI 24.3. She is derived from the seated reader's handcrafted body: every person bone, 2D shape of the four scales and 3D solid is scaled about her seat bones by {K.S:.4f}, shanks and feet by {f_leg:.4f} so her soles stay flat on the ground in front of the unchanged 1.20 m bench (her lower legs are 1.5 cm longer than Winter's stature share; cross-sections that sat exactly at the smallest feature of a scale keep it). Segment masses are the reader's de Leva (1996) female fractions times {K.BODY / K.BODY_OLD:.3f}, including 2.9 kg of clothing and shoes.",
          f"Eine 80-jährige Frau von {K.STATURE:.2f} m und {K.BODY:.0f} kg: Körperhöhe nach DIN 33402-2 (Frauen 61 bis 65, P50 1,61 m) abzüglich des Altersverlusts von etwa 1 cm pro Jahrzehnt nach 65 (Sorkin et al. 1999), was sie nahe dem 55. Perzentil ihres Alters einordnet; BMI 24,3. Sie ist vom handgearbeiteten Körper der sitzenden Leserin abgeleitet: Jeder Personenknochen, jede 2D-Form der vier Maßstäbe und jeder 3D-Körper ist um ihre Sitzbeinhöcker mit {K.S:.4f} skaliert, Unterschenkel und Füße mit {f_leg:.4f}, damit ihre Sohlen vor der unveränderten 1,20-m-Bank flach auf dem Boden stehen (ihre Unterschenkel sind 1,5 cm länger als Winters Anteil an der Körperhöhe; Querschnitte, die genau auf dem kleinsten Merkmal eines Maßstabs lagen, behalten es). Die Segmentmassen sind die weiblichen Anteile nach de Leva (1996) der Leserin mal {K.BODY / K.BODY_OLD:.3f}, einschließlich 2,9 kg Kleidung und Schuhe.".replace(f"{K.S:.4f}", f"{K.S:.4f}".replace(".", ",")).replace(f"{f_leg:.4f}", f"{f_leg:.4f}".replace(".", ",")).replace(f"{K.BODY / K.BODY_OLD:.3f}", f"{K.BODY / K.BODY_OLD:.3f}".replace(".", ",")).replace(f"{K.STATURE:.2f} m", f"{K.STATURE:.2f} m".replace(".", ","))),
        t("Her own: a rounded upper back and forward head in every seated pose (lumbar and thoracic flexion of 5 and 7 degrees on top of the reader's posture), grey hair in a bun, reading glasses (drawn at 1:50 and in the 1:100 front view; the 1.5 mm frame is below the smallest 3D feature of every scale and is not modelled), a buttoned moss-stitch cardigan over a fine roll-neck jumper, dark lined wool-blend trousers, knee socks and dark leather comfort shoes. The knitting is two 4 mm aluminium needles of 30 cm, the work (a scarf of 15 x 10 cm in progress, 4 mm thick) hanging from the left needle, and a 100 g ball of light wool on the seat to her right; the needle rest placements are solved so that in the knitting pose the tips cross 4 cm above and 8 cm ahead of her hands.",
          "Ihr Eigenes: ein gerundeter oberer Rücken und vorgeschobener Kopf in jeder Sitzhaltung (Lenden- und Brustbeugung von 5 und 7 Grad zusätzlich zur Haltung der Leserin), graues Haar im Dutt, eine Lesebrille (gezeichnet in 1:50 und in der Vorderansicht 1:100; die 1,5 mm starke Fassung liegt unter dem kleinsten 3D-Merkmal jedes Maßstabs und ist nicht modelliert), eine geknöpfte Strickjacke im Perlmuster über einem feinen Rollkragenpullover, eine dunkle gefütterte Hose aus Wollmischgewebe, Kniestrümpfe und dunkle Lederbequemschuhe. Das Strickzeug sind zwei 4-mm-Aluminiumnadeln von 30 cm, das Strickstück (ein entstehender Schal von 15 x 10 cm, 4 mm dick) an der linken Nadel und ein 100-g-Knäuel heller Wolle rechts neben ihr auf der Sitzfläche; die Ruhelagen der Nadeln sind so gelöst, dass sich ihre Spitzen in der Strickhaltung 4 cm über und 8 cm vor ihren Händen kreuzen."),
        t("Unchanged from the reader and kept as authored: the 1.20 m painted timber park bench (ground root, end frames, seat and backrest as fixed bones, seat 0.44 m high at the front, raked 5 degrees), the anatomical rig with its clavicles and hip bones solved in three dimensions, the seat, thigh and backrest supports, the stand-up key poses and the skeleton of all four scales.",
          "Von der Leserin unverändert übernommen: die 1,20 m lange gestrichene Holzparkbank (Bodenwurzel, Wangen, Sitz und Lehne als feste Knochen, Sitz vorn 0,44 m hoch, 5 Grad geneigt), das anatomische Rig mit dreidimensional gelösten Schlüsselbeinen und Hüftbeinen, die Auflager an Sitz, Oberschenkel und Lehne, die Schlüsselhaltungen des Aufstehens und das Skelett aller vier Maßstäbe."),
    ]


OVERLAYS = t("One region per shape; the needles and the work share the region of the knitting, the ball has its own; the layer order of a view is the order of the parts along its viewing direction (the work lies in front of the cardigan in the front view, the thighs over the shanks because the knees are nearest).",
             "Eine Region je Form; Nadeln und Strickstück teilen die Region des Strickzeugs, das Knäuel hat eine eigene; die Ebenenfolge einer Ansicht ist die Folge der Teile entlang ihrer Blickrichtung (das Strickstück liegt in der Vorderansicht vor der Strickjacke, die Oberschenkel über den Unterschenkeln, weil die Knie am nächsten liegen).")
RETAINED = t("Ids, tones, regions and materials are one per part in all three views and follow the body solids of this scale.",
             "Kennungen, Tonwerte, Regionen und Materialien sind je Teil in allen drei Ansichten gleich und folgen den Körpern dieses Maßstabs.")
DECISIONS = {
    "50": {
        "silhouette": t("Every filled shape is the true outline of the 1:50 body solids of its part in the knitting pose: bench (posts, front legs, seat rails, stretchers, tie, four seat slats, three back slats, bolt heads), the woman (trouser seat, cardigan with hem, roll-neck collar, neck, head with nose, ears, eyes, brows and mouth, grey hair with bun, sleeves, forearms, cuffs, wrists, hands with fingers and thumb, thighs, trouser legs, leather shoes with soles and toe parts), the work hanging from the left needle and the ball of wool on the seat. Sleeves and forearms are revolved profiles, the needles prisms along their bones and the work a plate swept across its thickness, so the knitting reads right in every pose.",
                        "Jede gefüllte Form ist der wahre Umriss der 1:50-Körper ihres Teils in der Strickhaltung: Bank (Pfosten, Vorderbeine, Sitzzargen, Streben, Querverbindung, vier Sitzlatten, drei Rückenlatten, Schraubenköpfe), die Frau (Hosenboden, Strickjacke mit Bund, Rollkragen, Hals, Kopf mit Nase, Ohren, Augen, Brauen und Mund, graues Haar mit Dutt, Ärmel, Unterarme, Bündchen, Handgelenke, Hände mit Fingern und Daumen, Oberschenkel, Hosenbeine, Lederschuhe mit Sohlen und Vorderteilen), das Strickstück an der linken Nadel und das Wollknäuel auf der Sitzfläche. Ärmel und Unterarme sind Drehprofile, die Nadeln Prismen entlang ihrer Knochen und das Strickstück eine über seine Dicke gezogene Platte, so liest sich das Stricken in jeder Haltung richtig."),
        "merged": t("Palm, thenar, fingers and thumb are one hand shape; the nose belongs to the head outline; the stitches on the needles belong to the work.",
                    "Handfläche, Daumenballen, Finger und Daumen sind eine Handform; die Nase gehört zum Kopfumriss; die Maschen auf den Nadeln gehören zum Strickstück."),
        "omitted": t("The strand of yarn from the ball to the needle (1 mm), shoe laces, trouser seams, the far eye, brow and ear in the side view, the face details and glasses in the plan.",
                     "Der Faden vom Knäuel zur Nadel (1 mm), Schnürsenkel, Hosennähte, das ferne Auge, Braue und Ohr in der Seitenansicht, die Gesichtsdetails und die Brille im Grundriss."),
        "thinElements": t("The 4 mm needles are detail lines (0.13 mm) along their true axes; the reading glasses are 0.13 mm detail outlines (lenses, bridge, near temple) inside the head; the placket is a line, the six buttons 0.3 mm dots.",
                          "Die 4-mm-Nadeln sind Detaillinien (0,13 mm) entlang ihrer wahren Achsen; die Lesebrille sind 0,13-mm-Detailumrisse (Gläser, Steg, nahes Bügelende) im Kopf; die Knopfleiste ist eine Linie, die sechs Knöpfe 0,3-mm-Punkte."),
        "hatching": t("Cardigan trunk, sleeves and forearms: moss stitch as a 45-degree cross-hatch at 0.7 mm; the work: horizontal rows at 0.45 mm; hem, cuffs and collar stay plain as ribbing.",
                      "Rumpf, Ärmel und Unterarme der Strickjacke: Perlmuster als 45-Grad-Kreuzschraffur mit 0,7 mm; das Strickstück: waagrechte Reihen mit 0,45 mm; Bund, Bündchen und Kragen bleiben als Rippen glatt."),
        "styles": t("Body 0.25 mm, detail 0.13 mm; grey hair at a light tone, trousers and shoes dark.", "Körper 0,25 mm, Detail 0,13 mm; graues Haar in hellem Ton, Hose und Schuhe dunkel."),
    },
    "100": {
        "silhouette": t("The true outlines of the 1:100 body in the knitting pose: bench with end frames part by part, one seat and three back slats, the woman with cardigan, collar, head with nose and ears, hair and bun, sleeves, forearms, mitten hands with thumbs, thighs, trouser legs and two-part shoes, the two needles, the work and the ball of wool.",
                        "Die wahren Umrisse des 1:100-Körpers in der Strickhaltung: Bank mit Wangen Teil für Teil, ein Sitz und drei Rückenlatten, die Frau mit Strickjacke, Kragen, Kopf mit Nase und Ohren, Haar und Dutt, Ärmeln, Unterarmen, Fäustlingshänden mit Daumen, Oberschenkeln, Hosenbeinen und zweiteiligen Schuhen, die beiden Nadeln, das Strickstück und das Wollknäuel."),
        "merged": t("Hem, cuffs, wrists, fingers, soles and the seat slats are merged as in the 3D body of this scale.", "Bund, Bündchen, Handgelenke, Finger, Sohlen und die Sitzlatten sind wie im 3D-Körper dieses Maßstabs verschmolzen."),
        "omitted": t("Eyes, brows, mouth, buttons, bolts, laces, the yarn strand; the glasses only in the front view.", "Augen, Brauen, Mund, Knöpfe, Schrauben, Schnürsenkel, der Faden; die Brille nur in der Vorderansicht."),
        "thinElements": t("Needles as 0.1 mm detail lines on their axes; the placket as a line; the glasses as two 0.4 mm lens outlines with the bridge in the front view.",
                          "Nadeln als 0,1-mm-Detaillinien auf ihren Achsen; die Knopfleiste als Linie; die Brille als zwei 0,4-mm-Glasumrisse mit Steg in der Vorderansicht."),
        "hatching": t("Cardigan: 45-degree cross-hatch at 0.8 mm; the work stays plain.", "Strickjacke: 45-Grad-Kreuzschraffur mit 0,8 mm; das Strickstück bleibt glatt."),
        "styles": t("Body 0.2 mm, detail 0.1 mm.", "Körper 0,2 mm, Detail 0,1 mm."),
    },
    "200": {
        "silhouette": t("The true outlines of the 1:200 body in the knitting pose: one shape per body segment, the bench as posts, legs with rails, one seat and one back board, the work as a 0.75 x 0.5 mm plate in front of the hands and the ball of wool as a 0.5 mm disc.",
                        "Die wahren Umrisse des 1:200-Körpers in der Strickhaltung: eine Form je Körpersegment, die Bank als Pfosten, Beine mit Zargen, ein Sitz und ein Rückenbrett, das Strickstück als 0,75 x 0,5 mm große Platte vor den Händen und das Wollknäuel als 0,5-mm-Scheibe."),
        "merged": t("Everything the 3D body of this scale merges: face details, hands as mittens, one shoe form, bench boards.", "Alles, was der 3D-Körper dieses Maßstabs verschmilzt: Gesichtsdetails, Hände als Fäustlinge, eine Schuhform, Bankbretter."),
        "omitted": t("Nose, ears, stretchers, glasses, placket; the 4 mm needles (0.02 mm on paper) are left out of drawing and body and kept as structural members only.",
                     "Nase, Ohren, Streben, Brille, Knopfleiste; die 4-mm-Nadeln (0,02 mm auf Papier) fehlen in Zeichnung und Körper und bleiben nur als Tragglieder."),
        "thinElements": t("None drawn; the work keeps its true 4 mm thickness as a declared sheet.", "Keine gezeichnet; das Strickstück behält als erklärte Fläche seine wahre Dicke von 4 mm."),
        "hatching": t("None.", "Keine."),
        "styles": t("Body 0.18 mm.", "Körper 0,18 mm."),
    },
    "500": {
        "silhouette": t("The true outlines of the generalised 1:500 body in the knitting pose: trunk, trouser seat, collar, head, hair with bun, arms, hands, thighs, shanks and shoes as one shape each, the bench as two end frames, seat and back board, the work in front of the hands and the ball of wool at the 0.2 mm feature.",
                        "Die wahren Umrisse des verallgemeinerten 1:500-Körpers in der Strickhaltung: Rumpf, Hosenboden, Kragen, Kopf, Haar mit Dutt, Arme, Hände, Oberschenkel, Unterschenkel und Schuhe je als eine Form, die Bank als zwei Wangen, Sitz und Rückenbrett, das Strickstück vor den Händen und das Wollknäuel auf dem Merkmal von 0,2 mm."),
        "merged": t("Neck into the collar, frames into one outline each.", "Hals in den Kragen, Wangen in je einen Umriss."),
        "omitted": t("All details below 75 mm: face, glasses, buttons; the needles are structural members only.", "Alle Details unter 75 mm: Gesicht, Brille, Knöpfe; die Nadeln sind nur Tragglieder."),
        "thinElements": t("None.", "Keine."),
        "hatching": t("None.", "Keine."),
        "styles": t("Body 0.13 mm.", "Körper 0,13 mm."),
    },
}
VOLUME_DECISIONS = {
    "50": [t("Body: an 80-year-old woman of 1.61 m in a moss-stitch cardigan over a fine roll-neck jumper, lined wool-blend trousers and leather comfort shoes, derived section by section from the seated reader's hand-stated body (scaled about the seat bones by 0.958, shanks and feet by 0.993): trouser seat, cardigan from its ribbed hem over hips, waist and bust to sloping shoulders, roll-neck collar, neck, skull with jaw, nose, lips, eyes, brows and ears, grey hair drawn back into a bun, sleeves, forearms with cuffs, wrists, hands with palm, thenar, three-part fingers and thumb, thighs, trouser legs and shoes with sole, toe part and lace bars.",
             "Körper: eine 80-jährige Frau von 1,61 m in einer Strickjacke im Perlmuster über einem feinen Rollkragenpullover, gefütterter Hose aus Wollmischgewebe und Lederbequemschuhen, Querschnitt für Querschnitt vom handgearbeiteten Körper der sitzenden Leserin abgeleitet (um die Sitzbeinhöcker mit 0,958 skaliert, Unterschenkel und Füße mit 0,993): Hosenboden, Strickjacke vom gerippten Bund über Hüfte, Taille und Brust bis zu den abfallenden Schultern, Rollkragen, Hals, Schädel mit Kiefer, Nase, Lippen, Augen, Brauen und Ohren, graues, zum Dutt zurückgenommenes Haar, Ärmel, Unterarme mit Bündchen, Handgelenke, Hände mit Handfläche, Daumenballen, dreigliedrigen Fingern und Daumen, Oberschenkel, Hosenbeine und Schuhe mit Sohle, Vorderteil und Schnürstegen."),
           t("Bench and knitting: the 1.20 m painted timber park bench as authored for the reader; two 4 mm aluminium needles of 30 cm as slender capsules tapering to the tip, crossing in front of her hands; the work a 150 x 100 x 4 mm knitted plate (a declared sheet) hanging from the left needle; a 100 g ball of wool of 100 mm on the second seat slat to her right.",
             "Bank und Strickzeug: die 1,20 m lange gestrichene Holzparkbank wie für die Leserin gebaut; zwei 4-mm-Aluminiumnadeln von 30 cm als schlanke, zur Spitze verjüngte Kapseln, die sich vor ihren Händen kreuzen; das Strickstück eine gestrickte Platte von 150 x 100 x 4 mm (eine erklärte Fläche) an der linken Nadel; ein 100-g-Wollknäuel von 100 mm auf der zweiten Sitzlatte rechts neben ihr."),
           t("Thermal: cardigan and shoes as nested layer shells where each layer lies (underwear and the roll-neck jumper under the trunk, the jumper under the upper sleeves; socks inside the uppers), trousers and hair as their single layer, face, ears and neck as their own skin solids, bench, knitting and ball as surfaces of their parts; breath at nostrils and lips and the body plume on the crown as outlet patches. Structure: the skeleton bone by bone as for the reader, a joint ball at every rig joint, masses at the de Leva centres; bench members, the needles and the work are their own members, the ball rests on the seat.",
             "Thermik: Strickjacke und Schuhe als geschachtelte Schichtschalen dort, wo jede Schicht liegt (Unterwäsche und Rollkragenpullover unter dem Rumpf, der Pullover unter den Oberärmeln; Socken in den Schäften), Hose und Haar als ihre einzige Schicht, Gesicht, Ohren und Hals als eigene Hautkörper, Bank, Strickzeug und Knäuel als Oberflächen ihrer Teile; Atem an Nasenlöchern und Lippen und die Körperfahne am Scheitel als Auslassflächen. Tragwerk: das Skelett Knochen für Knochen wie bei der Leserin, eine Gelenkkugel an jedem Rig-Gelenk, Massen in den Schwerpunkten nach de Leva; Bankglieder, die Nadeln und das Strickstück sind eigene Glieder, das Knäuel liegt auf dem Sitz."),
           t("Skeleton and trunk: the trunk bends in two places (more in her rounded upper back), so the cardigan is a waist part on the lumbar bone and a chest part on the thorax that overlap at T12; the hands close round the needles, so the load path runs from the knitting through the arms to the seat bones and the feet.",
             "Skelett und Rumpf: Der Rumpf beugt sich an zwei Stellen (stärker in ihrem gerundeten oberen Rücken), daher ist die Strickjacke ein Taillenteil am Lendenknochen und ein Brustteil am Brustkorb, die sich bei Th12 überlappen; die Hände schließen sich um die Nadeln, so läuft der Lastpfad vom Strickzeug durch die Arme zu den Sitzbeinhöckern und den Füßen.")],
    "100": [t("1:100 (smallest feature 15 mm): the reader's fewer, longer sections scaled to her size: hem and cuffs merged into cardigan and forearms, face details into the face, fingers into mitten hands with a thumb, one shoe form with its sole and a toe part; cross-sections that sat at the 15 mm feature keep it. Needles as slender capsules, the work as a 4 mm sheet, the ball as one ellipsoid; the bench with its end frames part by part, one seat and three back slats.",
              "1:100 (kleinstes Merkmal 15 mm): die wenigeren, längeren Querschnitte der Leserin auf ihre Größe skaliert: Bund und Bündchen in Strickjacke und Unterarme verschmolzen, Gesichtsdetails ins Gesicht, Finger in Fäustlingshände mit Daumen, eine Schuhform mit Sohle und Vorderteil; Querschnitte, die auf dem 15-mm-Merkmal lagen, behalten es. Nadeln als schlanke Kapseln, das Strickstück als 4-mm-Fläche, das Knäuel als ein Ellipsoid; die Bank mit Wangen Teil für Teil, einem Sitz und drei Rückenlatten."),
            t("Thermal: one surface per solid, the face, the ears and the neck as their own skin solids, the three outlet patches. Structure: the skeleton with merged parts, joints at every rig joint, masses from 1 kg; bench members, needles and work as members.",
              "Thermik: eine Oberfläche je Körper, Gesicht, Ohren und Hals als eigene Hautkörper, die drei Auslassflächen. Tragwerk: das Skelett mit zusammengefassten Teilen, Gelenke an jedem Rig-Gelenk, Massen ab 1 kg; Bankglieder, Nadeln und Strickstück als Glieder.")],
    "200": [t("1:200 (30 mm): one form per body segment in two to seven rows; head without nose and ears, hair as one cap with its bun; mitten hands of 30 mm; one shoe form per foot. The bench as rear posts, front legs with rails, one seat and one back board; the work as a declared 4 mm sheet, the ball as one ellipsoid; the needles (4 mm) are not part of the body at this scale.",
              "1:200 (30 mm): eine Form je Körpersegment in zwei bis sieben Reihen; Kopf ohne Nase und Ohren, Haar als eine Kappe mit Dutt; Fäustlingshände von 30 mm; eine Schuhform je Fuß. Die Bank als hintere Pfosten, Vorderbeine mit Zargen, ein Sitz und ein Rückenbrett; das Strickstück als erklärte 4-mm-Fläche, das Knäuel als ein Ellipsoid; die Nadeln (4 mm) gehören bei diesem Maßstab nicht zum Körper."),
            t("Thermal: one surface per region solid; only the total area is compared at this scale. Structure: three paired rib courses, a shoulder bar per side, one bone per limb segment, joints at the main joints, masses from 3 kg; the needles as slender members of their own (they carry the work into the hands), bench and work by reference.",
              "Thermik: eine Oberfläche je Regionskörper; bei diesem Maßstab wird nur die Gesamtfläche verglichen. Tragwerk: drei paarige Rippenzüge, ein Schulterstab je Seite, ein Knochen je Gliedmaßensegment, Gelenke an den Hauptgelenken, Massen ab 3 kg; die Nadeln als eigene schlanke Glieder (sie tragen das Strickstück in die Hände), Bank und Strickstück per Verweis.")],
    "500": [t("1:500 (75 mm): a generalised figure that still reads as an old woman knitting on a bench: trunk, trouser seat, collar with neck, head with hair cap and bun, upper arms, forearms, hands, thighs, shanks and shoes each one mass at least 75 mm thick; the bench as two end frames, a seat and a back board; the work as a declared sheet of 150 x 100 mm, the ball of wool at 100 mm.",
              "1:500 (75 mm): eine verallgemeinerte Figur, die sich noch als strickende alte Frau auf einer Bank liest: Rumpf, Hosenboden, Kragen mit Hals, Kopf mit Haarkappe und Dutt, Oberarme, Unterarme, Hände, Oberschenkel, Unterschenkel und Schuhe je eine Masse von mindestens 75 mm; die Bank als zwei Wangen, ein Sitz und ein Rückenbrett; das Strickstück als erklärte Fläche von 150 x 100 mm, das Wollknäuel mit 100 mm."),
            t("Thermal: one solid per region surface, totals only. Structure: spine, rib cage, pelvic bar, shoulder bar, skull and one bone per limb segment, masses from 6 kg; the needles as slender members, bench and work by reference.",
              "Thermik: ein Körper je Regionsoberfläche, nur Summen. Tragwerk: Wirbelsäule, Brustkorb, Beckenstab, Schulterstab, Schädel und ein Knochen je Gliedmaßensegment, Massen ab 6 kg; die Nadeln als schlanke Glieder, Bank und Strickstück per Verweis.")],
}


def decisions(asset):
    for key, d in DECISIONS.items():
        rep = asset["representations"][key]
        rep["decisions"] = {"silhouette": d["silhouette"], "retained": RETAINED, "merged": d["merged"], "omitted": d["omitted"], "thinElements": d["thinElements"],
                            "hatching": d["hatching"], "styles": d["styles"], "overlays": OVERLAYS}
        asset["volumes"][key]["decisions"] = VOLUME_DECISIONS[key]
    by = {b["id"]: b for b in asset["rig"]["bones"]}
    by["torso"]["title"] = t("Thorax with cardigan (T12/L1 to C7/T1)", "Brustkorb mit Strickjacke (Th12/L1 bis C7/Th1)")
    by["head"]["title"] = t("Head with grey bun and reading glasses", "Kopf mit grauem Dutt und Lesebrille")
    asset["structure"]["assumptions"][1] = t("Segments are point masses at the de Leva centres; the needles are 12 g bone masses in the hands, the work 0.12 kg on the left needle, the ball of wool 0.1 kg on the seat.",
                                             "Segmente sind Punktmassen in den Schwerpunkten nach de Leva; die Nadeln sind Knochenmassen von 12 g in den Händen, das Strickstück 0,12 kg an der linken Nadel, das Wollknäuel 0,1 kg auf der Sitzfläche.")
    sym = asset["symbols"][0]
    sym["description"] = t("Position dot for crowded overview plans; no direction arrow because a person knitting on a bench does not travel.",
                           "Positionspunkt für belebte Übersichtspläne; ohne Richtungspfeil, da eine strickende Person auf der Bank nicht läuft.")
