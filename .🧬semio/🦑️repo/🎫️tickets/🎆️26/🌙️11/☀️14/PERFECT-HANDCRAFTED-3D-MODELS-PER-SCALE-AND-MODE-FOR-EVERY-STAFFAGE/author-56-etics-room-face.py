import json, sys, os
T = os.path.dirname(os.path.abspath(__file__))
targets = {"100": "th-wall-core", "200": "th-wall-masonry", "500": "th-wall"}
for slug in ("etics-brick-wall-12", "etics-brick-wall-30"):
    p = os.path.join(T, "volumes", "components", slug + ".json")
    cur = json.load(open(p))
    for sc, sid in targets.items():
        for d in cur["volumes"][sc]["thermal"]:
            if d["id"] == sid:
                f = d.setdefault("sectionFaces", [])
                if "-u" not in f:
                    f.append("-u")
    json.dump(cur, open(p, "w"), ensure_ascii=False, indent=1)
    print("patched", slug)
