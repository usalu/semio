import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import iso16757, vdi3805
for r in iso16757.vectors()["volumes"]: print(r["id"], round(r["solidVolumeM3"],4), r["convexHullVolumeM3"] and round(r["convexHullVolumeM3"],4), r["rustBoundingBoxVolumeM3"], r["agrees"], r.get("hullAgreesWithClosedForm"))
v = vdi3805.vectors(); print(v["deltaTNominalK"], v["deltaTRelative"])
for r in v["radiators"]: print({k: round(x,5) for k, x in r.items()})
for r in v["valves"]: print(r)
print(v["authority"]); print([r for r in v["kvsTableR5"] if not r["matchesR5"]])
