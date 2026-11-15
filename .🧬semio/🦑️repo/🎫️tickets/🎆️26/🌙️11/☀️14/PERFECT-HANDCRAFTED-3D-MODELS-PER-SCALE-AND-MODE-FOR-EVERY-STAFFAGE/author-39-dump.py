import json, sys
CAT = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/"
slug = sys.argv[1]
a = json.load(open(CAT + slug + "/🔣️.json"))["asset"]
scales = sys.argv[2:] or ["50", "100", "200", "500"]
for k in scales:
    r = a["representations"][k]
    print("=== scale", k, r.get("status"), r.get("validRange"), "sections" in r and r.get("sections"))
    g = r.get("geometry", {})
    for vn, v in g.items():
        print("# view", vn, "n=", len(v))
        for sh in v:
            d = {x: sh[x] for x in sh if x not in ("tone", "z", "material", "hatch", "penMm")}
            for key in ("points", "outline"):
                if key in d:
                    p = d[key]
                    d[key] = [[round(c, 4) for c in q] for q in p] if len(p) <= 12 else "%d pts %s..%s" % (len(p), [round(c, 3) for c in p[0]], [round(c, 3) for c in p[-1]])
            print(json.dumps(d, ensure_ascii=False))
    print("decisions", json.dumps(r.get("decisions"), ensure_ascii=False)[:600])
