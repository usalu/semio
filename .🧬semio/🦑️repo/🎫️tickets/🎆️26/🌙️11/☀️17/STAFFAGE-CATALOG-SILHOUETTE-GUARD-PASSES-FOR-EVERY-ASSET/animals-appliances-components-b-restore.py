"""Puts shapes back as HEAD has them: python3 restore.py <slug> <id,id> <scale+view,...> (view letter e|f|p)"""
import sys, json, importlib.util
H = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET/"
s = importlib.util.spec_from_file_location("l", H + "animals-appliances-components-b-lib.py"); l = importlib.util.module_from_spec(s); s.loader.exec_module(l)
slug, ids, where = sys.argv[1:4]
d = l.doc(slug)
head = json.loads(l.mv.sp.head_text(l.CAT + slug))["asset"]
V = {"e": "elevation", "f": "front", "p": "plan"}
for w in where.split(","):
    sc, v = w[:-1], V[w[-1]]
    hs = {x["id"]: x for x in head["representations"][sc]["geometry"][v]}
    for i in ids.split(","):
        if i not in hs: continue
        d.replace_obj(d.shape(sc, v, i), hs[i])
d.commit()
