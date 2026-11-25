"""🚗️ Ops writer of `sectional-garage-door`: the two attachments of the spring cables, `cable-eye` on the bottom panel and `drum-cable` on the spring shaft, each stood at the centre plane for the left and the right cable; at 1:500, where only the two drums are drawn on the shaft, `drum-cable` lay 76 to 80 mm off them. Each becomes one anchor per cable at the lateral position of its stud and drum (D45), and the cable pull and its reaction of every pose become one load per cable with half the force, each with its own clip track: `python3 make/sectional-garage-door.py` writes `ops/sectional-garage-door.json`."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from mc import *
a = asset("sectional-garage-door")
STUD, DRUM = 1.178, 1.185
eye = [
    {"id": "cable-eye-left", "bone": "panel-1", "x": 0.06, "y": 0.01, "z": STUD, "kind": "attachment", "title": {"en": "Left cable stud under the bottom roller carrier", "de": "Linker Seilbolzen unter dem unteren Rollenhalter"}},
    {"id": "cable-eye-right", "bone": "panel-1", "x": 0.06, "y": 0.01, "z": -STUD, "kind": "attachment", "title": {"en": "Right cable stud under the bottom roller carrier", "de": "Rechter Seilbolzen unter dem unteren Rollenhalter"}},
]
drum = [
    {"id": "drum-cable-left", "bone": "spring-shaft", "x": -0.08, "y": 0, "z": DRUM, "kind": "attachment", "title": {"en": "Cable leaving the left drum", "de": "Seilabgang der linken Trommel"}},
    {"id": "drum-cable-right", "bone": "spring-shaft", "x": -0.08, "y": 0, "z": -DRUM, "kind": "attachment", "title": {"en": "Cable leaving the right drum", "de": "Seilabgang der rechten Trommel"}},
]
a["anchors"] = split_anchor(a, "cable-eye", eye)
a["anchors"] = split_anchor(a, "drum-cable", drum)
ops = [put(["anchors"], a["anchors"])]
side = lambda stem: {"left": (f"{stem}-left", {"en": ", left cable", "de": ", linkes Seil"}), "right": (f"{stem}-right", {"en": ", right cable", "de": ", rechtes Seil"})}
first = split_loads(a, "cable-eye", side("cable-eye"))
for o in first:
    next(p for p in a["poses"] if p["id"] == o["path"][1]["id"])["loads"] = o["value"]
ops += split_loads(a, "drum-cable", side("drum-cable"))
for clip in a.get("clips", []):
    tracks, changed = [], False
    for track in clip.get("tracks", []):
        if track.get("target") in ("load:cable-pull", "load:cable-reaction"):
            changed = True
            tracks += [dict(track, target=f"{track['target']}-{side}") for side in ("left", "right")]
        else:
            tracks.append(track)
    if changed or any(t.get("target", "").startswith("load:cable-") for t in clip.get("tracks", [])):
        ops.append(put(["clips", {"id": clip["id"]}, "tracks"], tracks))
write("sectional-garage-door", ops)
