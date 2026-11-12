import sys; sys.path.insert(0, ".")
from silhouette_edit import *
def mutate(solids):
    for s in solids:
        if s["id"].startswith("leaf-") and s["kind"] == "loft":
            for sec in s["sections"]:
                sec["rx"], sec["ry"] = sec["ry"], sec["rx"]
print(patch_volume("agave", mutate))
