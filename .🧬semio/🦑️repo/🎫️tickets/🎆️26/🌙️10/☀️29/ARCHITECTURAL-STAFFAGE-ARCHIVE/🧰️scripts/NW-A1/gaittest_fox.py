import sys
from fox import *
sk = make_skel()
Gh = dict(ac0=-85, ks=10, flex=50, ah0=-48, ahroll=12, ahflex=14, lift=0.07)
Gf = dict(ac0=-90, ks=8, flex=60, ah0=-42, ahroll=12, ahflex=14, lift=0.07, scap0=-60, scapamp=6)
xs = {"hn": 0.04, "hf": 0.04, "fn": 0.47, "ff": 0.47}
order = {"hn": 0.0, "ff": 0.0, "hf": 0.5, "fn": 0.5}
legs = [Leg(sk, k, FOOT, order[k], xs[k]) for k in ("hn", "fn", "hf", "ff")]
duty, T_, speed = 0.4, 0.5, 1.2
S = speed * duty * T_
worst = 0
for k in range(10):
    ph = k / 10
    offs, res = solve_phase(sk, {}, legs, ph + 1e-9, duty, S, Gh, Gf, extra_y=-0.03*(0.5+0.5*math.cos(4*math.pi*(ph-0.45))))
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
