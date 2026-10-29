# [scratch] emits the dairy-cow clip fragment: authored keys, spans and texts, with arithmetic for the contact seams
import json, math, sys
ADV = float(sys.argv[2]) if len(sys.argv) > 2 else 1.0
def T(en, de): return {"en": en, "de": de}
def ev(t, kind, en, de, target=None):
    e = {"t": t, "kind": kind, "title": T(en, de)}
    if target: e["target"] = target
    return e
NAMES = {"hn": ("Near hind", "Nahe Hinterklaue"), "fn": ("Near fore", "Nahe Vorderklaue"), "hf": ("Far hind", "Ferne Hinterklaue"), "ff": ("Far fore", "Ferne Vorderklaue")}
# walk
TW, D = 1.4, 0.68
offs = {"hn": 0, "fn": .25, "hf": .5, "ff": .75}
spans, events = [], []
for ft, o in offs.items():
    a, b = round(o*TW, 4), round(o*TW + D*TW, 4)
    if b <= TW: spans.append({"anchor": f"contact-{ft}", "fromS": a, "toS": b})
    else: spans += [{"anchor": f"contact-{ft}", "fromS": a, "toS": TW}, {"anchor": f"contact-{ft}", "fromS": 0, "toS": round(b - TW, 4)}]
    events.append(ev(a, "contact-on", f"{NAMES[ft][0]} hoof touches down", f"{NAMES[ft][1]} setzt auf", f"contact-{ft}"))
    events.append(ev(round(b % TW, 4), "contact-off", f"{NAMES[ft][0]} hoof lifts", f"{NAMES[ft][1]} hebt ab", f"contact-{ft}"))
events += [ev(0, "inhale", "Inhalation", "Einatmen"), ev(0.7, "exhale", "Exhalation", "Ausatmen")]
walk = {"id": "walk-cycle", "title": T("Walking, four-beat lateral sequence", "Schritt im Viertakt in Seitenfolge"),
  "description": T("Lateral-sequence walk after Hildebrand: near hind, near fore, far hind, far fore touch down at quarter-cycle intervals; each hoof is on the ground 68 % of the cycle, so two to four hooves always carry, and the swinging hoof flexes hock or carpus to clear the ground.",
                   "Schritt in Seitenfolge nach Hildebrand: nahes Hinterbein, nahes Vorderbein, fernes Hinterbein, fernes Vorderbein setzen im Viertelzyklus auf; jede Klaue steht 68 % des Zyklus am Boden, sodass stets zwei bis vier Klauen tragen, und die schwingende Klaue beugt Sprunggelenk oder Vorderfußwurzel, um den Boden zu verlassen."),
  "durationS": TW, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": ADV,
  "keys": [{"t": round(k*TW/8, 4), "pose": f"walk-phase-{k}", "ease": "smooth"} for k in range(8)], "contacts": spans,
  "phases": [{"id": f"{ft}-strike", "fromS": round(o*TW, 4), "toS": round(o*TW + TW/4, 4), "title": T(f"{NAMES[ft][0]} hoof strikes", f"{NAMES[ft][1]} setzt auf")} for ft, o in offs.items()],
  "events": events,
  "tracks": [{"target": "emitter:breath-nose", "ease": "smooth", "keys": [{"t": 0, "v": 0.7}, {"t": 0.7, "v": 0.9}, {"t": 1.4, "v": 0.7}]}]}
# cud chewing
ALL = [{"anchor": f"contact-{f}", "fromS": 0, "toS": 5} for f in ("hn", "hf", "fn", "ff")]
keys = []
for c in range(5):
    for dt, j in ((0, -3), (0.4, -13), (0.8, -5)):
        t = round(c + dt, 3)
        keys.append({"t": t, "pose": "standing-chewing", "ease": "smooth", "bones": {
            "jaw": {"angleDeg": j}, "head": {"angleDeg": round(2 + 1.5*math.sin(2*math.pi*t/2.5), 2)},
            "tail-1": {"angleDeg": round(12*math.sin(2*math.pi*t/5), 1)}, "tail-2": {"angleDeg": round(4 + 8*math.sin(2*math.pi*t/5 - 0.7), 1)},
            "ear-n": {"angleDeg": 26 if 2.0 <= t <= 2.4 else 6}}})
chew = {"id": "cud-chewing", "title": T("Ruminating while standing", "Wiederkäuen im Stehen"),
  "description": T("The standing cow chews the cud at one jaw cycle per second (Beauchemin 1991) with a slight nod of the head, a flick of the near ear and a slow swish of the tail; nasal breathing at 24 breaths per minute.",
                   "Die stehende Kuh käut mit einem Kieferzyklus pro Sekunde wieder (Beauchemin 1991), nickt leicht mit dem Kopf, schlägt einmal mit dem nahen Ohr und wedelt langsam mit dem Schwanz; Nasenatmung mit 24 Atemzügen pro Minute."),
  "durationS": 5, "fps": 30, "loop": True, "rootMotion": "none", "keys": keys, "contacts": ALL,
  "phases": [{"id": "chew-a", "fromS": 0, "toS": 2.5, "title": T("First breath cycle", "Erster Atemzyklus")}, {"id": "chew-b", "fromS": 2.5, "toS": 5, "title": T("Second breath cycle with ear flick", "Zweiter Atemzyklus mit Ohrenschlag")}],
  "events": [ev(0, "inhale", "Inhalation", "Einatmen"), ev(1.125, "exhale", "Exhalation", "Ausatmen"), ev(2.5, "inhale", "Inhalation", "Einatmen"), ev(3.625, "exhale", "Exhalation", "Ausatmen"), ev(2.0, "motion", "Near ear flicks", "Nahes Ohr schlägt", "ear-n")],
  "tracks": [{"target": "emitter:breath-nose", "ease": "smooth", "keys": [{"t": 0, "v": 0.3}, {"t": 1.125, "v": 0.6}, {"t": 2.5, "v": 0.3}, {"t": 3.625, "v": 0.6}, {"t": 5, "v": 0.3}]}]}
# lie down
LD = 11
lie = {"id": "lie-down", "title": T("Lying down", "Abliegen"),
  "description": T("Non-looping transition after Krohn and Munksgaard (1993): the standing cow lowers its head and sniffs the floor, folds both fore legs and kneels on the carpi, then flexes the hind legs and sinks the haunches until the sternum, hocks and hooves rest on the ground, the far fore leg stretching forward. Contact spans overlap so the cow is supported at every instant.",
                   "Nicht wiederholter Übergang nach Krohn und Munksgaard (1993): die stehende Kuh senkt den Kopf und beschnuppert den Boden, schlägt beide Vorderbeine ein und kniet auf den Vorderfußwurzeln, beugt dann die Hinterbeine und lässt die Hinterhand absinken, bis Brustbein, Sprunggelenke und Klauen auf dem Boden ruhen, das ferne Vorderbein streckt sich nach vorn. Die Kontaktspannen überlappen, sodass die Kuh in jedem Augenblick gestützt ist."),
  "durationS": LD, "fps": 30, "loop": False, "rootMotion": "none",
  "keys": [{"t": 0, "pose": "standing-chewing", "ease": "smooth"},
           {"t": 1.5, "pose": "standing-chewing", "ease": "smooth", "bones": {"neck": {"angleDeg": -28}, "head": {"angleDeg": 22}, "jaw": {"angleDeg": -6}, "ear-n": {"angleDeg": 14}}},
           {"t": 3.6, "pose": "kneeling-front", "ease": "smooth"}, {"t": 6.4, "pose": "lowering-hindquarters", "ease": "smooth"},
           {"t": 7.6, "pose": "lying-resting", "ease": "smooth", "bones": json.load(open(__file__.replace("cow-clips.py", "dairy-cow/far-fore-lift.json")))}, {"t": 9.0, "pose": "lying-resting", "ease": "smooth"}, {"t": 11, "pose": "lying-resting", "ease": "smooth", "bones": {"jaw": {"angleDeg": -10}}}],
  "contacts": [{"anchor": "contact-hn", "fromS": 0, "toS": LD}, {"anchor": "contact-hf", "fromS": 0, "toS": LD},
               {"anchor": "contact-fn", "fromS": 0, "toS": 3.8}, {"anchor": "contact-ff", "fromS": 0, "toS": 3.8}, {"anchor": "contact-ff", "fromS": 8.6, "toS": LD},
               {"anchor": "contact-carpus-fn", "fromS": 3.2, "toS": LD}, {"anchor": "contact-carpus-ff", "fromS": 3.2, "toS": 7.4},
               {"anchor": "contact-hock-hn", "fromS": 7.0, "toS": LD}, {"anchor": "contact-hock-hf", "fromS": 7.0, "toS": LD}, {"anchor": "contact-sternum", "fromS": 7.8, "toS": LD}],
  "phases": [{"id": "sniff", "fromS": 0, "toS": 2.5, "title": T("Lowering the head and sniffing", "Kopf senken und schnuppern")},
             {"id": "kneel", "fromS": 2.5, "toS": 4.5, "title": T("Kneeling on the carpi", "Auf die Vorderfußwurzeln knien")},
             {"id": "sink", "fromS": 4.5, "toS": 9.0, "title": T("Sinking the hindquarters", "Hinterhand absenken")},
             {"id": "settle", "fromS": 9.0, "toS": LD, "title": T("Settling and ruminating", "Zur Ruhe kommen und wiederkäuen")}],
  "events": [ev(3.2, "contact-on", "Near fore carpus touches down", "Nahe Vorderfußwurzel setzt auf", "contact-carpus-fn"), ev(3.2, "contact-on", "Far fore carpus touches down", "Ferne Vorderfußwurzel setzt auf", "contact-carpus-ff"),
             ev(3.8, "contact-off", "Fore hooves lift", "Vorderklauen heben ab", "contact-fn"), ev(7.0, "contact-on", "Hocks touch down", "Sprunggelenke setzen auf", "contact-hock-hn"),
             ev(7.8, "contact-on", "Sternum touches down", "Brustbein setzt auf", "contact-sternum"), ev(7.4, "contact-off", "Far fore carpus lifts as the leg stretches forward", "Ferne Vorderfußwurzel hebt ab, das Bein streckt sich nach vorn", "contact-carpus-ff"), ev(8.6, "contact-on", "Far fore hoof touches down in front", "Ferne Vorderklaue setzt vorn auf", "contact-ff")],
  "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.95}, {"t": 3.6, "v": 3.6}, {"t": 6.4, "v": 3.4}, {"t": 9, "v": 2.55}, {"t": 11, "v": 2.55}]},
             {"target": "emitter:breath-nose", "ease": "smooth", "keys": [{"t": 0, "v": 0.5}, {"t": 3.6, "v": 0.8}, {"t": 6.4, "v": 0.8}, {"t": 11, "v": 0.45}]}]}
# graze and chew
gk = [{"t": 0, "pose": "standing-chewing", "ease": "smooth"}, {"t": 2.2, "pose": "grazing-head-down", "ease": "smooth"}]
for i, t in enumerate((3.2, 4.6, 6.0)):
    gk += [{"t": t, "pose": "grazing-head-down", "ease": "smooth", "bones": {"jaw": {"angleDeg": -14}}}, {"t": round(t + 0.5, 2), "pose": "grazing-head-down", "ease": "smooth", "bones": {"jaw": {"angleDeg": -2}, "head": {"angleDeg": 49}}}]
gk += [{"t": 8.0, "pose": "grazing-head-down", "ease": "smooth"}, {"t": 10.0, "pose": "standing-chewing", "ease": "smooth"}, {"t": 12, "pose": "standing-chewing", "ease": "smooth", "bones": {"jaw": {"angleDeg": -12}}}]
graze = {"id": "graze-and-chew", "title": T("Grazing, then chewing", "Weiden, dann Wiederkäuen"),
  "description": T("The cow lowers the head to the sward, takes three bites by tearing grass with a quick closing of the jaw, raises the head and chews the bolus standing.", "Die Kuh senkt den Kopf zum Rasen, nimmt drei Bissen, indem sie Gras mit raschem Kieferschluss abreißt, hebt den Kopf und kaut den Bissen im Stehen."),
  "durationS": 12, "fps": 30, "loop": False, "rootMotion": "none", "keys": gk,
  "contacts": [{"anchor": f"contact-{f}", "fromS": 0, "toS": 12} for f in ("hn", "hf", "fn", "ff")],
  "phases": [{"id": "lower", "fromS": 0, "toS": 2.2, "title": T("Lowering the head", "Kopf senken")}, {"id": "bite", "fromS": 2.2, "toS": 8.0, "title": T("Biting grass", "Gras abbeißen")}, {"id": "raise", "fromS": 8.0, "toS": 12, "title": T("Raising the head and chewing", "Kopf heben und kauen")}],
  "events": [ev(3.7, "motion", "First bite", "Erster Biss"), ev(5.1, "motion", "Second bite", "Zweiter Biss"), ev(6.5, "motion", "Third bite", "Dritter Biss"), ev(0, "inhale", "Inhalation", "Einatmen"), ev(1.125, "exhale", "Exhalation", "Ausatmen")],
  "tracks": [{"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 2.95}, {"t": 2.2, "v": 3.4}, {"t": 8, "v": 3.4}, {"t": 12, "v": 2.95}]}, {"target": "emitter:breath-nose", "ease": "smooth", "keys": [{"t": 0, "v": 0.5}, {"t": 2.2, "v": 0.6}, {"t": 8, "v": 0.6}, {"t": 12, "v": 0.5}]}]}
open(sys.argv[1], "w").write('      "clips": [\n' + ",\n".join("        " + json.dumps(c, ensure_ascii=False) for c in (walk, chew, lie, graze)) + "\n      ],\n")
