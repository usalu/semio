"""🚋 Tram: upper articulation link and pin kept under the bellows crown; door hangers kept inside the leaf plane."""
import json

PATH = "volumes/vehicles/tram.json"
doc = json.load(open(PATH))
V = doc["volumes"]
get = lambda sc, sid: next(s for s in V[sc]["structure"] if s.get("id") == sid)
get("50", "articulation-upper")["transform"]["translation"] = [0.0, 2.91, 0.0]
get("50", "articulation-upper-pin")["origin"] = [0, 2.82, 0.0]
for d in ("a1", "a2", "b1", "b2"):
    c = get("50", f"leaf-carriage-{d}")
    c["size"][2] = 0.08
    c["transform"]["translation"][2] = -1.132
    c = get("100", f"leaf-carriage-{d}")
    c["size"][2] = 0.09
    c["transform"]["translation"][2] = -1.125
json.dump(doc, open(PATH, "w"), ensure_ascii=False)
