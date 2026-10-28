import sys, itertools
sys.path.insert(0, str(__import__("pathlib").Path(__file__).parent))
from tune22 import *
rows = []
for ex, cdh, ig, ds in itertools.product((4, 8, 16), (8000, 11000, 14000), (100, 250, 400), (.6, .7, .8)):
    setv(climate__utilization_exponent_cooling=ex, climate__cooling_degree_hours=cdh, climate__internal_gains_summer=ig, climate__direct_share_summer=ds)
    o = check()
    rows.append((o["curv"], ex, cdh, ig, ds, o["opt"], o["rmin"], o["or"], o["min"], o["share"], o["sav"]))
rows.sort()
for r in rows[:30]: print(r)
