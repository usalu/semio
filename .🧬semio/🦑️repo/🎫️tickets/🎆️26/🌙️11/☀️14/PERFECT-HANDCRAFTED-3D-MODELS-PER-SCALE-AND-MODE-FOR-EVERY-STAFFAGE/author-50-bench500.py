import json
P = "volumes/furniture/park-bench.json"
d = json.load(open(P))
v = d["volumes"]["500"]
S = "steel-painted"
def box(id, bone, size, at, tone=0.35):
    return {"id": id, "kind": "box", "bone": bone, "material": S, "role": "body", "tone": tone, "size": size, "transform": {"translation": at}}
def post(id, bone):
    return {"id": id, "kind": "extrude", "bone": bone, "material": S, "role": "body", "tone": 0.4, "profile": [[0.0, 0.0763], [0.0, 0.1363], [0.85, 0.3142], [0.85, 0.2542]], "plane": "yz", "from": 0.04, "to": 0.2}
body = []
for p, bone in (("l", "end-left"), ("r", "end-right")):
    body += [box(p + "-foot", bone, [0.16, 0.05, 0.55], [0.12, 0.025, -0.025]), box(p + "-front-leg", bone, [0.16, 0.634, 0.06], [0.12, 0.317, -0.26], 0.4), post(p + "-rear-post", bone)]
body.append({"id": "seat-block", "kind": "box", "bone": "seat", "material": "oak-timber", "role": "body", "tone": 0.58, "size": [1.8, 0.045, 0.49], "transform": {"translation": [0.9, 0.0225, -0.055]}})
body.append({"id": "back-block", "kind": "extrude", "bone": "backrest", "material": "oak-timber", "role": "body", "tone": 0.58, "profile": [[-0.17, 0.133], [-0.17, 0.211], [0.2, 0.2884], [0.2, 0.2104]], "plane": "yz", "from": 0.0, "to": 1.8})
v["body"] = body
for s in v["structure"]:
    if s.get("part") == "support":
        s["size"] = [0.16, 0.05, 0.4]; s["transform"]["translation"][1] = 0.025
    if s.get("part") == "mass":
        s["size"] = [0.12, 0.2, 0.058]
v["decisions"][0]["en"] = "Modelled anew against the 1:500 drawings: each end is a 50 mm foot, a 60 mm front column and a raked 60 mm post (the rake of the drawn parallelogram), all 160 mm wide as the elevation draws the frame; the seat is the drawn 45 mm plank 490 mm deep, the back a 78 mm raked slab on the drawn parallelogram (the 75 mm smallest feature of the backrest bone) that runs down to the seat because the elevation draws seat and back as one block; armrest, slats, bolts and bearers are merged."
v["decisions"][0]["de"] = "Neu modelliert gegen die Zeichnungen 1:500: jede Seite besteht aus einem 50-mm-Fuß, einer 60-mm-Vordersäule und einem geneigten 60-mm-Pfosten (die Neigung des gezeichneten Parallelogramms), alle 160 mm breit wie das Gestell in der Ansicht; der Sitz ist die gezeichnete 45-mm-Bohle von 490 mm Tiefe, die Lehne eine 78 mm dicke geneigte Platte auf dem gezeichneten Parallelogramm (das kleinste Merkmal 75 mm des Lehnenknochens), die bis zum Sitz reicht, weil die Ansicht Sitz und Lehne als einen Block zeichnet; Armlehne, Latten, Schrauben und Auflager sind zusammengefasst."
json.dump(d, open(P, "w"), ensure_ascii=False, indent=1)
