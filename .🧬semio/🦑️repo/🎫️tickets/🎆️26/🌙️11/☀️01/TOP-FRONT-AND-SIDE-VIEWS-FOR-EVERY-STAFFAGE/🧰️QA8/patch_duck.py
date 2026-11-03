import os, re

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
p = os.path.join(A2, "gen_duck.py")
L = open(p, encoding="utf-8").read().split("\n")
out = []
for l in L:
    ind = re.match(r"^(\s*)", l).group(1)
    if l.lstrip().startswith("F.blob(\"%s-wing\""):
        z = re.search(r"z=(.*)\)$", l).group(1)
        l = ind + 'F.ext("%%s-wing" %% tag, wing, F.bar(wing, 0.0, 0.3, 0.045), min(sd * 0.044, sd * 0.074), max(sd * 0.044, sd * 0.074), like="%%s-wing" %% tag, z=%s)' % z
    elif l.lstrip().startswith("P.cap(\"%s-wing\""):
        z = re.search(r"z=(.*)\)$", l).group(1)
        l = ind + 'P.ext("%%s-wing" %% tag, wing, P.bar(wing, 0.0, 0.3, 0.045), min(sd * 0.044, sd * 0.074), max(sd * 0.044, sd * 0.074), like="%%s-wing" %% tag, z=%s)' % z
    elif l.lstrip().startswith("F.poly(tag, wing,"):
        l = ind + 'F.ext(tag, wing, F.bar(wing, 0.0, 0.3, 0.045), min(sd * 0.044, sd * 0.074), max(sd * 0.044, sd * 0.074), filled=False, like="wing", z=4, penMm=0.15)'
    elif l.lstrip().startswith('P.cap("wing", "wing-n"'):
        l = ind + 'P.ext("wing", "wing-n", P.bar("wing-n", 0.0, 0.3, 0.045), -0.074, -0.044, filled=False, like="wing", z=4, penMm=0.15)'
    elif l.lstrip().startswith('P.cap("wing-far", "wing-f"'):
        l = ind + 'P.ext("wing-far", "wing-f", P.bar("wing-f", 0.0, 0.3, 0.045), 0.044, 0.074, filled=False, like="wing", z=4, penMm=0.15)'
    out.append(l)
open(p, "w", encoding="utf-8").write("\n".join(out))
print(sum(1 for l in out if ".ext(" in l and "wing" in l))
