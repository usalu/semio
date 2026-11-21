"""⏱️ Round 5: re-times the two scissor-lift clips to the Genie GS-1932 data sheet (raise 16 s, lower 29 s) by a piecewise-linear time map over every key, phase, event, track and contact, and rewrites the clip texts; the clips array is replaced in place in the catalog file (indent 1), nothing else changes.

@see ./r3-machinery-scissor-lift.md
@see ./r5-brief.md
"""
import json, sys
PATH = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/scissor-lift/🔣️.json"
MAPS = {
    "raise-cycle": ([0, 3, 9, 14.5, 20, 24], [0, 3, 19, 24.5, 53.5, 57.5]),
    "start-lift-stop": ([0, 3, 9, 15.5, 21, 22], [0, 3, 19, 25.5, 54.5, 55.5]),
}
TEXTS = {
    "raise-cycle": {"en": "A 57.5 s loop at the data-sheet speeds: the platform rises to 5.8 m in 16 s, the deck slides out and back, then the stack folds down again in 29 s; motor, pump and battery follow the lift.", "de": "Eine Schleife von 57,5 s in den Geschwindigkeiten des Datenblatts: Die Bühne steigt in 16 s auf 5,8 m, die Verlängerung fährt aus und wieder ein, dann faltet sich das Paket in 29 s wieder zusammen; Motor, Pumpe und Batterie folgen dem Hub."},
    "start-lift-stop": {"en": "A 55.5 s sequence at the data-sheet speeds: key on at 1 s, two occupants step on, the platform rises in 16 s, the deck extends, then everything lowers in 29 s and the key is turned off at 54.5 s.", "de": "Eine Folge von 55,5 s in den Geschwindigkeiten des Datenblatts: Schlüssel an bei 1 s, zwei Personen steigen auf, die Bühne steigt in 16 s, die Verlängerung fährt aus, dann senkt sich alles in 29 s und der Schlüssel wird bei 54,5 s abgeschaltet."},
}


def warp(t, src, dst):
    for i in range(len(src) - 1):
        if src[i] <= t <= src[i + 1]:
            v = dst[i] + (t - src[i]) * (dst[i + 1] - dst[i]) / (src[i + 1] - src[i])
            v = round(v, 4)
            return int(v) if v == int(v) else v
    raise ValueError(t)


text = open(PATH, encoding="utf-8").read()
start = text.index('"clips": [') + len('"clips": ')
clips, length = json.JSONDecoder().raw_decode(text[start:])
for clip in clips:
    src, dst = MAPS[clip["id"]]
    if clip["durationS"] != src[-1]:
        sys.exit(f"{clip['id']} already re-timed ({clip['durationS']} s)")
    w = lambda t: warp(t, src, dst)
    clip["durationS"] = dst[-1]
    clip["description"] = TEXTS[clip["id"]]
    for k in clip["keys"]:
        k["t"] = w(k["t"])
    for p in clip["phases"]:
        p["fromS"], p["toS"] = w(p["fromS"]), w(p["toS"])
    for c in clip["contacts"]:
        c["fromS"], c["toS"] = w(c["fromS"]), w(c["toS"])
    for e in clip["events"]:
        e["t"] = w(e["t"])
    for tr in clip["tracks"]:
        for k in tr["keys"]:
            k["t"] = w(k["t"])
body = json.dumps(clips, indent=1, ensure_ascii=False).replace("\n", "\n  ")
new = text[:start] + body + text[start + length:]
json.loads(new)
open(PATH, "w", encoding="utf-8").write(new)
print("re-timed", [(c["id"], c["durationS"]) for c in clips])
