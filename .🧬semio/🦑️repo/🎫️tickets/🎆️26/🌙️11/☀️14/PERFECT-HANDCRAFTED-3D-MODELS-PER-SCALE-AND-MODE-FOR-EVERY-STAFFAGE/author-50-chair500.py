import json, sys
P = "volumes/furniture/office-swivel-chair.json"
d = json.load(open(P))
v = d["volumes"]["500"]
for s in v["body"]:
    if s["id"].startswith("arm-"):
        a, b = s["sections"]
        a.update(rx=0.02, ry=0.02); b.update(rx=0.016, ry=0.016)
        a["exponent"] = b["exponent"] = 2.0
    if s["id"] == "hub":
        s["profile"] = [[-0.003, 0.06], [0.027, 0.06]]
    if s["id"].endswith("-wheel"):
        s["profile"] = [[-0.015, 0.028], [0.015, 0.028]]
json.dump(d, open(P, "w"), ensure_ascii=False, indent=1)
