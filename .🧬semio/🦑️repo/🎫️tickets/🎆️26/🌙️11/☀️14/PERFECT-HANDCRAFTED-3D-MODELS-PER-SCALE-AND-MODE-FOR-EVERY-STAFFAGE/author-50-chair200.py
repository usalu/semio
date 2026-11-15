import json
P = "volumes/furniture/office-swivel-chair.json"
d = json.load(open(P))
v = d["volumes"]["200"]
T = [[0.04, 0.02], [0.185, 0.02], [0.185, -0.115], [0.215, -0.115], [0.215, 0.19], [0.185, 0.19], [0.185, 0.05], [0.04, 0.05]]
arm = {"id": "armrest", "kind": "extrude", "bone": "seat", "material": "pa-plastic", "role": "body", "tone": 0.4, "profile": T, "plane": "xy", "from": 0.2545, "to": 0.2855, "repeat": {"count": 2, "translation": [0.0, 0.0, -0.54]}}
i = next(k for k, s in enumerate(v["body"]) if s["id"] in ("armrest", "armrest-post"))
v["body"] = [s for s in v["body"] if s["id"] not in ("armrest", "armrest-post", "armrest-pad")]
v["body"].insert(i, arm)
i = next(k for k, s in enumerate(v["thermal"]) if s["id"].startswith("th-armrest"))
v["thermal"] = [s for s in v["thermal"] if not s["id"].startswith("th-armrest")]
v["thermal"].insert(i, {"id": "th-armrest", "of": "armrest", "region": "chair-plastic", "layer": "pa-shell"})
i = next(k for k, s in enumerate(v["structure"]) if s["id"].startswith("member-armrest"))
v["structure"] = [s for s in v["structure"] if not s["id"].startswith("member-armrest")]
v["structure"].insert(i, {"id": "member-armrest", "of": "armrest", "member": "seat", "part": "member"})
v["decisions"][0]["en"] = v["decisions"][0]["en"].replace("one triangular armrest panel per side merging post, bracket and pad as the 1:200 drawing shows", "one T-shaped armrest per side (30 mm post and 30 mm pad, 305 mm long) as the corrected 1:200 drawing shows")
v["decisions"][0]["de"] = v["decisions"][0]["de"].replace("je Seite eine dreieckige Armlehnenplatte aus Stütze, Winkel und Auflage wie in der Zeichnung 1:200", "je Seite eine T-förmige Armlehne (Stütze 30 mm, Auflage 30 mm, 305 mm lang) wie in der korrigierten Zeichnung 1:200")
json.dump(d, open(P, "w"), ensure_ascii=False, indent=1)
