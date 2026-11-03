import os, re

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")


def edit(name, fn):
    p = os.path.join(A2, name)
    s = open(p, encoding="utf-8").read()
    s2 = fn(s)
    open(p, "w", encoding="utf-8").write(s2)
    print(name, s != s2)


edit("lib.py", lambda s: s.replace(
    "    for sh in rep[\"geometry\"][\"elevation\"]:\n        hit = A.evs.get((scale, sh[\"id\"]))",
    "    for sh in rep[\"geometry\"][\"elevation\"]:\n        if sh.get(\"solid\", {}).get(\"kind\") == \"extruded\" and sh[\"bone\"] not in A.spread and (scale, sh[\"id\"]) not in A.evs:\n            del sh[\"solid\"]\n        hit = A.evs.get((scale, sh[\"id\"]))", 1))

edit("gen_pig.py", lambda s: s.replace(
    'F.limb_front("forearm-" + t, "forearm-f" + t, fl, 0.08, 0.06, like=', 'F.limb_front("forearm-" + t, "forearm-f" + t, fl, 0.08, 0.06, over=pad, under=pad, like=').replace(
    'F.limb_front("hoof-h-" + t, "hoof-h" + t, hl, 0.058, 0.075, like=', 'F.limb_front("hoof-h-" + t, "hoof-h" + t, hl, 0.058, 0.075, over=pad, like=').replace(
    'F.limb_front("hoof-f-" + t, "hoof-f" + t, fl, 0.058, 0.075, like=', 'F.limb_front("hoof-f-" + t, "hoof-f" + t, fl, 0.058, 0.075, over=pad, like='))

edit("gen_fox.py", lambda s: s.replace(
    'fw=(0.038 * k, 0.044 * k), fz=5.06,', 'fw=(0.038 * k, 0.044 * k), over=pad, fz=5.06,').replace(
    'fw=(0.04 * k, 0.046 * k), fz=1.06,', 'fw=(0.04 * k, 0.046 * k), over=pad, fz=1.06,'))

edit("gen_sheep.py", lambda s: s.replace(
    'dict(id="gaskin-{s}", bone="shank-h{s}", flat=0.118, fw=(0.09, 0.07), fz=1.2,', 'dict(id="gaskin-{s}", bone="shank-h{s}", flat=0.118, fw=(0.09, 0.07), over=(0.0 if scale <= 100 else 0.02), under=(0.0 if scale <= 100 else 0.02), fz=1.2,'))
