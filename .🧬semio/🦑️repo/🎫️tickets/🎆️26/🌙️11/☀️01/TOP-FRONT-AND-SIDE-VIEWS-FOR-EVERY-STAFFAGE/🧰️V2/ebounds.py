from lib import *

IDS = "japanese-maple lombardy-poplar london-plane-pollarded miscanthus-grass multi-stem-birch scots-pine small-leaved-lime weeping-willow".split()
for i in IDS:
    doc, _ = read_doc(i)
    a = doc["asset"]
    print(i, a["dimensions"])
    for sc in SCALES:
        print("  ", sc, ["%.2f" % v for v in elev_bounds(a, sc)])
