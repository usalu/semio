"""🐾 Derives with exact fractions the pathline vectors of the flow-view fixture and compares them with the fixture: one parcel path sampled at 20 Hz (x = 0.4 a, y = 1.5 + 0.2 a², a the parcel age in seconds, last sample at its age now 1.02 s) and what a trail of it keeps.

Derivation (trail 0.33 s): the tail is where the parcel was 0.33 s ago, at age 0.69 s, 4/5 of the way from the sample of 0.65 s to the one of 0.70 s; a kept point that the parcel left `ago` seconds ago has the fade 1 - ago / 0.33; four opacity steps of 0.9 / 4 group the segments by the mean fade of their ends.
Run: python3 pathline-fixture.py <fixture.json> (prints the derived trail and exits 1 when the fixture differs)
"""
import json
import sys
from fractions import Fraction as F

ages = [F(i, 20) for i in range(21)] + [F(102, 100)]
point = lambda a: [float(F(4, 10) * a), float(F(3, 2) + F(2, 10) * a * a), 0.0]
trail = F(33, 100)
tail_age = ages[-1] - trail
lo = max(i for i, a in enumerate(ages) if a <= tail_age)
f = (tail_age - ages[lo]) / (ages[lo + 1] - ages[lo])
a, b = point(ages[lo]), point(ages[lo + 1])
tail = [round(a[i] + (b[i] - a[i]) * float(f), 12) for i in range(3)]
kept = ages[lo + 1:]
ago = [trail] + [ages[-1] - x for x in kept]
fade = [1 - x / trail for x in ago]
bands = 4
band = [min(bands - 1, int((fade[i] + fade[i + 1]) / 2 * bands)) for i in range(len(fade) - 1)]
runs = []
for i, value in enumerate(band):
    if runs and runs[-1]["band"] == value:
        runs[-1]["points"] += 1
    else:
        runs.append({"start": i, "points": 2, "band": value, "opacity": float(F(9, 10) * (value + 1) / bands)})
long_trail = F(2)
fixture = json.load(open(sys.argv[1], encoding="utf-8"))["pathlines"]
derived = {"trailS": float(trail), "tailFraction": float(f), "tail": tail, "points": len(ago), "agoS": [float(x) for x in ago], "fade": [round(float(x), 12) for x in fade], "runs": runs}
print(json.dumps(derived))
same = fixture["trail"] == derived and fixture["path"]["ageS"] == [float(x) for x in ages] and fixture["path"]["points"] == [point(x) for x in ages] and fixture["long"]["firstFade"] == float(1 - ages[-1] / F(2)) and fixture["whole"]["agoS0"] == float(ages[-1])
sys.exit(0 if same else 1)
