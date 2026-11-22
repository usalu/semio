"""🔍️ Exploration: writes one pocket-arm variant into the queuing asset's default pose and prints wrist, tip and elbow (then run silhouette.ts)."""
import sys
from hw17lib import *
import gen_person_queuing as g
ang, ab, tw, fa = map(float, sys.argv[1:5])
doc = load("person-queuing"); a = doc["asset"]
p = [p for p in a["poses"] if p["id"] == "hands-in-pockets"][0]
p["bones"].update(g.pockets(ang=ang, ab=ab, tw=tw, fa=fa))
write("person-queuing", doc)
fr = frames(f"{HUMANS}/person-queuing/🔣️.json", ["hands-in-pockets"])["hands-in-pockets"]["bones"]
print(sys.argv[1:], "wrist", fr["hand-l"]["o"], "tip", fr["hand-l"]["tip"], "elbow", fr["forearm-l"]["o"])
