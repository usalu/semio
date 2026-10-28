import sys, itertools
sys.path.insert(0, str(__import__("pathlib").Path(__file__).parent))
from tune22 import *
rows = []
for ig, cdh, ex, ds, g in itertools.product((50, 150, 250), (10000, 14000, 18000), (3, 6), (.45, .55), (.45, .5, .55)):
    setv(climate__internal_gains_summer=ig, climate__cooling_degree_hours=cdh, climate__utilization_exponent_cooling=ex, climate__direct_share_summer=ds, envelope__g_value=g)
    o = check()
    rows.append((o["curv"], ig, cdh, ex, ds, g, o["opt"], o["rmin"], o["or"][2], o["min"], o["share"]))
rows.sort()
for r in rows[:25]: print(r)
