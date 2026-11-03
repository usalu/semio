import sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from ndplib import *
d = load()
MAT = ["clay", "casi", "concrete", "aac"]
GRP = ["g1", "g2", "g3", "g4"]
def k_en(m, mat, g):
    if m == "tl":
        return {("clay", "g1"): .75, ("clay", "g2"): .70, ("clay", "g3"): .50, ("clay", "g4"): .35, ("casi", "g2"): .65}.get((mat, g), .80)
    if m == "lw":
        return (.30 if g in ("g1", "g2") else .25) if mat == "clay" else .45
    if mat == "clay": return {"g1": .55, "g2": .45}.get(g, .35)
    if mat in ("casi", "concrete"): return .55 if g == "g1" else .45
    return .55
def k_de(m, mat, g):
    if m == "tl": return .70
    if mat == "clay": return {"g1": .60, "g2": .50, "g3": .45, "g4": .40}[g]
    if mat in ("casi", "concrete"): return .60 if g == "g1" else .50
    return .60
NAME = {"gp": "general purpose", "tl": "thin layer", "lw": "lightweight"}
for m in NAME:
    for mat in MAT:
        for g in GRP:
            put(d, "1996", f"k.{m}.{mat}.{g}", "3.6.1.2 Table 3.3 / NA", f"K {NAME[m]} {mat} {g}", k_en(m, mat, g), k_de(m, mat, g), "", f"K of f_k = K f_b^alpha f_m^beta, row key mortar.material.group, as used by the artifact; K for {NAME[m]} mortar, material {mat}, unit group {g}", part="1")
    ab = {"gp": ((.70, .30), (.65, .25)), "tl": ((.85, 0.0), (.85, 0.0)), "lw": ((.70, .30), (.65, .25))}[m]
    put(d, "1996", f"alpha.{m}", "3.6.1.2", f"alpha {NAME[m]}", ab[0][0], ab[1][0], "", "Exponent of f_b in f_k", part="1")
    put(d, "1996", f"beta.{m}", "3.6.1.2", f"beta {NAME[m]}", ab[0][1], ab[1][1], "", "Exponent of f_m in f_k", part="1")
put(d, "1996", "k-e", "3.7.2", "K_E", 1000.0, 950.0, "", "E = K_E f_k", part="1")
BANDS = [("ge15", 15.0), ("ge10", 10.0), ("ge5", 5.0), ("ge2-5", 2.5), ("lt2-5", 0.0)]
def fvk0(annex, mortar, mat, m):
    if annex == "de":
        if mortar == "tl": return .22
        if mortar == "lw": return .15
        return .26 if m >= 15 else .22 if m >= 10 else .18 if m >= 5 else .08
    if mortar == "lw": return .15
    if mortar == "tl": return {"clay": .30, "casi": .40}.get(mat, .30)
    if mat == "clay": return .30 if m >= 10 else .20 if m >= 2.5 else .10
    return .20 if m >= 10 else .15 if m >= 2.5 else .10
for mortar in ("gp", "tl", "lw"):
    for mat in MAT:
        for band, lo in (BANDS if mortar == "gp" else [("all", 0.0)]):
            put(d, "1996", f"fvk0.{mortar}.{mat}.{band}", "3.6.2 Table 3.4 / NA", f"f_vk0 {NAME[mortar]} {mat} {band}", fvk0("en", mortar, mat, lo), fvk0("de", mortar, mat, lo), "MPa", "Initial shear strength; band is the mortar compressive strength class interval; as used by the artifact", part="1")
for direction in (1, 2):
    for mat in ("clay", "casi", "concrete", "aac"):
        for strong in (True, False):
            if direction == 1: v = .10 if mat == "clay" else (.10 if strong else .05)
            else: v = .20 if (mat == "aac" or not strong) else .40
            put(d, "1996", f"fxk.x{direction}.{mat}.{'strong' if strong else 'weak'}", "3.6.3 Table 3.6", f"f_xk{direction} {mat} {'f_m >= 5' if strong else 'f_m < 5'}", v, v, "MPa", "Flexural strength; strong means mortar f_m >= 5 MPa; as used by the artifact", part="1")
save(d); print(len(d))
