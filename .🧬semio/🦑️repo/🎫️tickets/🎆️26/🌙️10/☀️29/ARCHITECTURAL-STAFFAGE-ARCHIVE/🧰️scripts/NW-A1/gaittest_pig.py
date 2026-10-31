import sys
from pig import *
sk = make_skel()
Gh = dict(ac0=-88, ks=10, flex=40, ah0=-52, ahroll=12, ahflex=14, lift=0.06)
Gf = dict(ac0=-90, ks=8, flex=55, ah0=-53, ahroll=12, ahflex=14, lift=0.07, scap0=-62, scapamp=6)
xs = {"hn": 0.03, "hf": 0.03, "fn": 0.68, "ff": 0.68}
order = {"hn": 0.0, "fn": 0.25, "hf": 0.5, "ff": 0.75}
legs = [Leg(sk, k, FOOT, order[k], xs[k]) for k in ("hn", "fn", "hf", "ff")]
duty, T_, speed = 0.667, 1.0, 0.42
S = speed * duty * T_
worst = 0
for k in range(12):
    ph = k / 12
    offs, res = solve_phase(sk, {}, legs, ph + 1e-9, duty, S, Gh, Gf)
    fr = sk.frames(offs)
    line = []
    for l in legs:
        p = sk.anchor_w(fr, l.anchor)
        st, tgt = res[l.key]
        err = math.hypot(p[0] - tgt[0], p[1] - tgt[1])
        worst = max(worst, err)
        line.append("%s %s (%.3f,%.3f) err %.3f" % (l.key, "S" if st else "w", p[0], p[1], err))
    for b in sk.bones:
        v = offs.get(b["id"], 0); lo, hi = sk.lim[b["id"]]
        if not lo - 1e-6 <= v <= hi + 1e-6: line.append("LIMIT %s %.1f [%s,%s]" % (b["id"], v, lo, hi))
    print(k, " | ".join(line))
print("worst", worst)
