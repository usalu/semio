import sys, re
sys.path.insert(0, ".")
from verification_textedit import edit_spans
F = float(sys.argv[1]); P = sys.argv[2]
def num(x): 
    v = round(float(x) * F, 5); return repr(int(v)) if v == int(v) else repr(v)
def scale(t):
    t = re.sub(r'("p1": \[)([-\d.]+)(, 0, 0\])', lambda m: m.group(1) + num(m.group(2)) + m.group(3), t)
    t = re.sub(r'("at": \[)([-\d.]+)(, 0, 0\])', lambda m: m.group(1) + num(m.group(2)) + m.group(3), t)
    return t
ids = [("fr-r1b-blade", "loft"), ("fr-r1b-rachis", "capsule"), ("fr-l1b-blade", "loft"), ("fr-l1b-rachis", "capsule")]
print(edit_spans(P, [(f'"id": "{i}",\n          "kind": "{k}",', scale) for i, k in ids]))
