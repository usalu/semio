import sys; sys.path.insert(0, ".")
from silhouette_edit import *
R = {"fr-r1b-blade", "fr-r1b-rachis", "fr-l1b-blade", "fr-l1b-rachis"}
def num(x):
    v = round(x / 0.85, 5)
    return int(v) if v == int(v) else v
def mutate(solids):
    for s in solids:
        if not s["id"].startswith("fr-"): continue
        s.pop("repeat", None)
        if s["id"] in R:
            if s["kind"] == "capsule": s["p1"][0] = num(s["p1"][0])
            else:
                for sec in s["sections"]: sec["at"][0] = num(sec["at"][0])
print(patch_volume("date-palm", mutate))
