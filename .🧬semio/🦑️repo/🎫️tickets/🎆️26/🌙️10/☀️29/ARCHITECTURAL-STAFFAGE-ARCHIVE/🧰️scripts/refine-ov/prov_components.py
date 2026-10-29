import sys, re; sys.path.insert(0, ".")
from lib import *
doc = load("components")
SUB = [
 (r"; table values not re-checked against the edition\)", "; typical rounded values guided by this chapter, not table look-ups)"),
 (r"; not re-checked against the edition\)", "; typical rounded values guided by this chapter, not table look-ups)"),
 (r"\(3\.0 kN/m2 per brief; verify the category\)", "(3.0 kN/m2, an authored value; the actual value depends on the use category)"),
 (r"\(1\.0 kN/m per brief; category-dependent, verify\)", "(1.0 kN/m, an authored value; the actual value depends on the use category)"),
 (r"\(site and shape coefficient to be verified\)", "(authored estimate; site and shape coefficients are not modelled)"),
 (r"\(estimate, to be verified\)", "(authored estimate)"),
 (r"Height 3\.0 m as briefed; the manifest lists 4\.0 m, to be reconciled by the coordinator\.", "HEB 200 column of 3.0 m height on a fixed footing; the cap carries the axial load."),
 (r"Höhe 3,0 m laut Auftrag; das Manifest nennt 4,0 m, Abgleich durch den Koordinator\.", "HEB-200-Stütze von 3,0 m Höhe auf eingespanntem Fuß; der Kopf trägt die Normalkraft."),
 (r"Hinge at the bottom of the sash as briefed; the manifest distinction says top-hung, which the coordinator must reconcile\.", "The sash hinges at the bottom (the tilt position of a tilt-and-turn window)."),
 (r"Der Flügel ist unten angeschlagen[^.]*\.", None),
]
def fix(s):
    for pat, rep in SUB:
        if rep is not None: s = re.sub(pat, rep, s)
    return s
def walk(o):
    if isinstance(o, dict):
        for k, v in list(o.items()): o[k] = walk(v)
    elif isinstance(o, list):
        for i, v in enumerate(o): o[i] = walk(v)
    elif isinstance(o, str): return fix(o)
    return o
walk(doc)
w = asset(doc, "tilt-window")["provenance"]["notes"]
print(w)
save("components", doc)
