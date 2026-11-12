import sys; sys.path.insert(0, ".")
from silhouette_edit import *
U = [0, 0.1643, 0.3286, 0.4929, 0.6571, 0.8214, 0.9857, 1.15]
W = [0.0425, 0.0727, 0.0887, 0.0891, 0.085, 0.0759, 0.0571, 0.004]
T = [0.02, 0.0184, 0.0168, 0.0152, 0.0136, 0.012, 0.0085, 0.005]
def mutate(solids):
    for s in solids:
        if s["id"] == "leaf-s":
            s["sections"] = [{"at": [u, 0, 0], "rx": w, "ry": t, "exponent": 2.4, "rotation": [0.70711, 0, 0.70711, 0]} for u, w, t in zip(U, W, T)]
print(patch_volume("agave", mutate))
