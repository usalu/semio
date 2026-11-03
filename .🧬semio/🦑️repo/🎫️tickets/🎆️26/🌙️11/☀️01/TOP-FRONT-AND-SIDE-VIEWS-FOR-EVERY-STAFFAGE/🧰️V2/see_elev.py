import sys, os
from pv import *

OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/V2/"
os.makedirs(OUT, exist_ok=True)
ids = sys.argv[2:] or "japanese-maple lombardy-poplar london-plane-pollarded miscanthus-grass multi-stem-birch scots-pine small-leaved-lime weeping-willow".split()
sc = sys.argv[1]
for i in ids:
    doc, _ = read_doc(i)
    render(doc["asset"], {"front": [], "plan": []}, sc, OUT + "elev-%s-%s.png" % (i, sc))
    print(i)
