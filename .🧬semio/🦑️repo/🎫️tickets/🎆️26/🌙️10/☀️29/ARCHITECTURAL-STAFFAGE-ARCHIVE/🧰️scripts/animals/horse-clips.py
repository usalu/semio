# [scratch] writes 50-clips.json: walk key times and the graze-and-swish override keys from authored amplitudes
import json, math
T, N = 0.95, 8
keys = ",\n".join("            " + json.dumps({"t": round(k*T/N, 5), "pose": f"walk-phase-{k}", "ease": "smooth"}) for k in range(N))
def sp(a, f, t): return {"anchor": a, "fromS": f, "toS": t}
spans = [sp("contact-hn", 0, 0.608), sp("contact-fn", 0.2375, 0.8455), sp("contact-hf", 0.475, 0.95), sp("contact-hf", 0, 0.133), sp("contact-ff", 0.7125, 0.95), sp("contact-ff", 0, 0.3705)]
ev = lambda t, k, en, de, tg=None: {"t": t, "kind": k, "title": {"en": en, "de": de}, **({"target": tg} if tg else {})}
walk = {"id": "walk-cycle", "title": {"en": "Walking, four-beat", "de": "Schritt im Viertakt"},
 "description": {"en": "Lateral-sequence walk after Hildebrand (1965) and Muybridge: near hind, near fore, far hind, far fore touch down at quarter-cycle intervals; each hoof is on the ground 64 % of the cycle, so two to four hooves always carry. The head nods once per stride and the tail sways gently.", "de": "Schritt in Seitenfolge nach Hildebrand (1965) und Muybridge: nahes Hinterbein, nahes Vorderbein, fernes Hinterbein, fernes Vorderbein setzen im Viertelzyklus auf; jeder Huf steht 64 % des Zyklus am Boden, sodass stets zwei bis vier Hufe tragen. Der Kopf nickt einmal pro Schritt und der Schweif pendelt leicht."},
 "durationS": T, "fps": 30, "loop": True, "rootMotion": "plant", "advanceM": 1.25, "KEYS": None, "contacts": spans,
 "phases": [{"id": "near-hind-strike", "fromS": 0, "toS": 0.2375, "title": {"en": "Near hind strikes first", "de": "Nahes Hinterbein setzt zuerst auf"}}, {"id": "near-fore-strike", "fromS": 0.2375, "toS": 0.475, "title": {"en": "Near fore follows", "de": "Nahes Vorderbein folgt"}}, {"id": "far-hind-strike", "fromS": 0.475, "toS": 0.7125, "title": {"en": "Far hind strikes", "de": "Fernes Hinterbein setzt auf"}}, {"id": "far-fore-strike", "fromS": 0.7125, "toS": 0.95, "title": {"en": "Far fore closes the sequence", "de": "Fernes Vorderbein schließt die Folge"}}],
 "events": [ev(0, "contact-on", "Near hind hoof touches down", "Nahes Hinterbein setzt auf", "contact-hn"), ev(0.2375, "contact-on", "Near fore hoof touches down", "Nahes Vorderbein setzt auf", "contact-fn"), ev(0.475, "contact-on", "Far hind hoof touches down", "Fernes Hinterbein setzt auf", "contact-hf"), ev(0.7125, "contact-on", "Far fore hoof touches down", "Fernes Vorderbein setzt auf", "contact-ff"),
  ev(0.608, "contact-off", "Near hind hoof lifts", "Nahes Hinterbein hebt ab", "contact-hn"), ev(0.8455, "contact-off", "Near fore hoof lifts", "Nahes Vorderbein hebt ab", "contact-fn"), ev(0.133, "contact-off", "Far hind hoof lifts", "Fernes Hinterbein hebt ab", "contact-hf"), ev(0.3705, "contact-off", "Far fore hoof lifts", "Fernes Vorderbein hebt ab", "contact-ff"),
  ev(0, "motion", "Head at lowest point of the nod", "Kopf am tiefsten Punkt des Nickens")],
 "tracks": [{"target": "emitter:breath-nose", "ease": "smooth", "keys": [{"t": 0, "v": 0.9}, {"t": 0.475, "v": 1.0}, {"t": 0.95, "v": 0.9}]}]}
G, W = 6.4, 2*math.pi/3.2
gk = []
for i in range(32):
    t = round(i*0.2, 2); c = 2*math.pi*t/0.8
    b = {"jaw": {"angleDeg": round(-8 + 6*math.cos(c), 1)}, "head": {"angleDeg": round(65 + 3*math.sin(2*math.pi*t/G), 1)}, "neck-3": {"angleDeg": round(5 + 2*math.sin(2*math.pi*t/G + 1), 1)},
         "tail-1": {"angleDeg": round(6 + 14*math.sin(W*t), 1)}, "tail-2": {"angleDeg": round(3 + 20*math.sin(W*t - 0.9), 1)}, "tail-3": {"angleDeg": round(26*math.sin(W*t - 1.8), 1)}, "tail-4": {"angleDeg": round(28*math.sin(W*t - 2.7), 1)},
         "ear-n": {"angleDeg": round(12 + 10*math.sin(2*math.pi*t/1.6), 1)}, "ear-f": {"angleDeg": round(18 - 8*math.sin(2*math.pi*t/1.6 + 1), 1)}}
    gk.append({"t": t, "pose": "grazing-head-down", "ease": "smooth", "bones": b})
gkeys = ",\n".join("            " + json.dumps(k) for k in gk)
graze = {"id": "graze-and-swish", "title": {"en": "Grazing and swishing the tail", "de": "Grasen und Schweifschlagen"},
 "description": {"en": "The standing horse grazes with the head down: the jaw chews at 75 strokes per minute, the head and ears move slightly, and the tail swishes twice to drive off flies. All four hooves stay planted and the clip loops.", "de": "Das stehende Pferd grast mit gesenktem Kopf: der Unterkiefer kaut mit 75 Zügen pro Minute, Kopf und Ohren bewegen sich leicht, und der Schweif schlägt zweimal gegen Fliegen. Alle vier Hufe bleiben stehen, der Clip läuft als Schleife."},
 "durationS": G, "fps": 30, "loop": True, "rootMotion": "none", "KEYS": None,
 "contacts": [sp(c, 0, G) for c in ("contact-hn", "contact-hf", "contact-fn", "contact-ff")],
 "phases": [{"id": "swish-1", "fromS": 0, "toS": 3.2, "title": {"en": "First swish while chewing", "de": "Erstes Schlagen beim Kauen"}}, {"id": "swish-2", "fromS": 3.2, "toS": 6.4, "title": {"en": "Second swish and snort", "de": "Zweites Schlagen und Schnauben"}}],
 "events": [ev(0.4, "motion", "Tail swish begins", "Schweifschlagen beginnt"), ev(3.6, "motion", "Second tail swish begins", "Zweites Schweifschlagen beginnt"), ev(3.0, "exhale", "Snort", "Schnauben")],
 "tracks": [{"target": "emitter:snort", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 2.8, "v": 0}, {"t": 3.1, "v": 1}, {"t": 3.4, "v": 0}, {"t": 6.4, "v": 0}]}, {"target": "emitter:breath-nose", "ease": "smooth", "keys": [{"t": 0, "v": 0.6}, {"t": 3.1, "v": 0.4}, {"t": 6.4, "v": 0.6}]}, {"target": "activityMet", "ease": "smooth", "keys": [{"t": 0, "v": 1.9}, {"t": 3.2, "v": 2.0}, {"t": 6.4, "v": 1.9}]}]}
def fmt(c, ks):
    s = json.dumps(c, ensure_ascii=False, indent=2)
    s = s.replace('"KEYS": null', '"keys": [\n' + ks + '\n          ]')
    return "\n".join("        " + l for l in s.split("\n"))
open("horse/50-clips.json", "w").write('      "clips": [\n' + fmt(walk, keys) + ",\n" + fmt(graze, gkeys) + "\n      ],\n")
