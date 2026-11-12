import sys; sys.path.insert(0, ".")
from silhouette_edit import *
LEN = {"near-forepaw": 0.035, "far-forepaw": 0.035, "near-hindpaw": 0.04, "far-hindpaw": 0.04}
def mutate(solids):
    for s in solids:
        if s["id"] in LEN:
            half = LEN[s["id"]] / 2
            z = s["transform"]["translation"][2]
            s["radii"] = [half, 0.0075, 0.0135]
            s["transform"] = {"translation": [half, 0.0, z]}
print(patch_volume("hedgehog", mutate))
