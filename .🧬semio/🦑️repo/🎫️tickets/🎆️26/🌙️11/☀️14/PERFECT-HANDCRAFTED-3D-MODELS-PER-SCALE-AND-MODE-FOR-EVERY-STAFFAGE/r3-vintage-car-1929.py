"""🏁️ Round 3 author of vehicles/vintage-car-1929: writes the handcrafted volumes of every scale to volumes/vehicles/vintage-car-1929.json. Usage: python3 r3-vintage-car-1929.py [50 100 200 500]"""
import os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(HERE, "r3-vintage-car-1929-kit.py")).read())
want = [int(a) for a in sys.argv[1:]] or [50, 100, 200, 500]
scales = []
for part in ("s50", "coarse", "decisions"):
    exec(open(os.path.join(HERE, f"r3-vintage-car-1929-{part}.py")).read())
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
