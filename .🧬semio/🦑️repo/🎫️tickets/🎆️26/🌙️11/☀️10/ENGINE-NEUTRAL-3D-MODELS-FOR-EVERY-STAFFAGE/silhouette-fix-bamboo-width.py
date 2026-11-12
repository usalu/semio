import sys; sys.path.insert(0, ".")
from silhouette_edit import *
F = 1.7 / 1.6177
def mutate(solids):
    for s in solids:
        if s["kind"] == "ellipsoid" and s["id"].startswith("mass-"):
            s["radii"][2] = round(s["radii"][2] * F, 5)
            s["transform"]["translation"][2] = round(s["transform"]["translation"][2] * F, 5)
print(patch_volume("bamboo-clump", mutate))
