import os, re

A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
p = os.path.join(A2, "gen_pigeon.py")
L = open(p, encoding="utf-8").read().split("\n")
out = []
for l in L:
    m = re.match(r'^(\s*)P\.limb_plan\((tag \+ "-coverts"|tag \+ "-primaries"|"near-primaries"|"near-coverts"|"far-wing"), ("[a-z]+-" \+ t|"[a-z]+-[nf]"), (wl|-?0\.\d+), [^,]+, [^,]+, like=(.*)$', l)
    if m:
        ind, name, bone, _, rest = m.groups()
        if "coverts" in name:
            bar = 'P.bar(%s, -0.03, 0.08, 0.028)' % bone
            rng = "min(sd * 0.045, sd * 0.075), max(sd * 0.045, sd * 0.075)" if "tag" in name else "-0.075, -0.045"
        elif "primaries" in name:
            bar = 'P.bar(%s, 0.0, 0.15, 0.013)' % bone
            rng = "min(sd * 0.05, sd * 0.075), max(sd * 0.05, sd * 0.075)" if "tag" in name else "-0.075, -0.035"
        else:
            bar = 'P.bar(%s, 0.0, 0.15, 0.025)' % bone
            rng = "0.035, 0.075"
        l = '%sP.ext(%s, %s, %s, %s, like=%s' % (ind, name, bone, bar, rng, rest)
    out.append(l)
open(p, "w", encoding="utf-8").write("\n".join(out))
print(sum(1 for l in out if "P.ext(" in l))

q = os.path.join(A2, "gen_pig.py")
s = open(q, encoding="utf-8").read()
s = s.replace('F.limb_front("cannon-h-" + t, "metatarsus-h" + t, hl, 0.05, 0.044, like=', 'F.limb_front("cannon-h-" + t, "metatarsus-h" + t, hl, 0.05, 0.044, over=pad, under=pad, like=')
s = s.replace('F.limb_front("cannon-f-" + t, "metacarpus-f" + t, fl, 0.052, 0.046, like=', 'F.limb_front("cannon-f-" + t, "metacarpus-f" + t, fl, 0.052, 0.046, over=pad, under=pad, like=')
s = s.replace("    for sd, t in ((Fr, \"f\"), (N, \"n\")):\n        zh, zf = 1.0, 5.0", "    pad = 0.0 if scale <= 100 else 0.03\n    for sd, t in ((Fr, \"f\"), (N, \"n\")):\n        zh, zf = 1.0, 5.0", 1)
open(q, "w", encoding="utf-8").write(s)
print("pad" in s)
