"""Kneeling-gardener: a real sun hat (dished brim of woven straw, not a puck), 1:50 and 1:100."""
from author_33_lib import *
slug = "kneeling-gardener"
def brim_profile(t, n):
    R0, R1, top0, drop = 0.085, 0.2355, 0.2285, 0.031
    rs = [R0 + (R1 - R0) * i / n for i in range(n + 1)]
    top = [(round(top0 - drop * ((r - R0) / (R1 - R0)) ** 2, 5), round(r, 5)) for r in rs]
    bot = [(round(a - t, 5), r) for a, r in reversed(top)]
    return [list(p) for p in top + bot]
d = load(slug)
for sc, t, n in (("50", 0.0095, 6), ("100", 0.0155, 4)):
    v = d["volumes"][sc]
    for s in v["body"]:
        if s["id"] == "hat-brim": s["profile"] = brim_profile(t, n)
    v["thermal"] = [x for x in v["thermal"] if x["id"] != "th-hat-brim-hair-pack"]
save(slug, d)
for sc in (50, 100): print(sc, check(slug, sc, "volumes-silhouette,volumes-feature,volumes-bounds"))
