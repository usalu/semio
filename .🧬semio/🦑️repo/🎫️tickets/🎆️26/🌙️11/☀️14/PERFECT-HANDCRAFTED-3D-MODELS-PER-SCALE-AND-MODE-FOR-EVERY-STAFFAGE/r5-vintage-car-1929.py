"""🏎️ Round 5 author of vehicles/vintage-car-1929: the round 3 kit with the track corrected to 1.40 m (tyre centres at z = +-0.70, Voisin C11 1422 mm, C14 1400 mm), the round 5 lists of every scale; writes volumes/vehicles/vintage-car-1929.json. Usage: python3 r5-vintage-car-1929.py [50 100 200 500]"""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "r3-vintage-car-1929-kit.py")).read())
ZW = 0.70
want = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
scales = []
for part in ("r5-vintage-car-1929-s50", "r5-vintage-car-1929-coarse", "r5-vintage-car-1929-decisions"):
    exec(open(os.path.join(HERE, f"{part}.py"), encoding="utf-8").read())
for den in want:
    s = Scale(den)
    s.body = globals()[f"body{den}"]()
    globals()[f"thermal{den}"](s)
    globals()[f"structure{den}"](s)
    globals()[f"decisions{den}"](s)
    ids = [d["id"] for d in s.body]
    assert len(ids) == len(set(ids)), [i for i in ids if ids.count(i) > 1]
    scales.append(s)
save_v(SLUG, scales)
