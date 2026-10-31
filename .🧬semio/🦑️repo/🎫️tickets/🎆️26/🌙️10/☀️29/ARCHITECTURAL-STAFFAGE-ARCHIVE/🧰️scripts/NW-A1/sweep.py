import sys, itertools, subprocess, json
import sheep, sheep_asset, gapscan
res = []
for sh_, sf, sfg in itertools.product((-0.2, -0.28, 0.2, 0.28), (0.2, 0.28, -0.2), (-0.2, -0.28, 0.0)):
    sheep.STG_H, sheep.STG_F, sheep.STG_FG = sh_, sf, sfg
    sheep_asset.STG_H = sh_
    import importlib
    # re-bind module constants used inside sheep functions
    sheep_asset.main()
    sk = sheep.make_skel()
    a = json.load(open("/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/sheep/🔣️.json"))["asset"]
    tot = {d: len(gapscan.scan(a, sk, d)) for d in (500, 200, 100, 50)}
    res.append((sum(tot.values()), tot, sh_, sf, sfg))
res.sort(key=lambda r: r[0])
for r in res[:6]: print(r)
