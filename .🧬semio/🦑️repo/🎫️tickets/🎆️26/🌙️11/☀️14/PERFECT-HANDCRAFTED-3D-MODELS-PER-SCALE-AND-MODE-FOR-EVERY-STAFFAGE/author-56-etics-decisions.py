import json, os
T = os.path.dirname(os.path.abspath(__file__))
en = " The room-side face of the core slab is marked as a section face (-u): the single region counts one wall face as in the 1:50 model, where only the outer layer's face counts; the room face is the conditioned boundary of the same heat path, not a second surface."
de = " Die raumseitige Fläche der Kernplatte ist als Schnittfläche (-u) markiert: die eine Region zählt wie im 1:50-Modell, wo nur die Außenfläche der äußersten Schicht zählt, eine Wandfläche; die Raumfläche ist die Randbedingung desselben Wärmepfads, keine zweite Oberfläche."
for slug in ("etics-brick-wall-12", "etics-brick-wall-30"):
    p = os.path.join(T, "volumes", "components", slug + ".json")
    cur = json.load(open(p))
    for sc in ("100", "200", "500"):
        for d in cur["volumes"][sc]["decisions"]:
            if d["en"].startswith("Thermal:") and "section face (-u)" not in d["en"]:
                d["en"] += en
                d["de"] += de
    json.dump(cur, open(p, "w"), ensure_ascii=False, indent=1)
