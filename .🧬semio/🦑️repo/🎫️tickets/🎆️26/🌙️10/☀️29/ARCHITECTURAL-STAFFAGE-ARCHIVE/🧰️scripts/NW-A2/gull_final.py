import sys
ay, fl, ls100 = sys.argv[1], sys.argv[2], sys.argv[3]
r = open('/tmp/gr2.bak').read()
i = r.index('def rep100'); k = r.index('def rep200')
s100 = r[i:k].replace('[(0.0, 0.0), (0.1, 0.0),', f'[({ls100}, 0.0), (0.1, 0.0),')
seg = r[k:]
seg = seg.replace('[(0.0, 0.0), (0.1, 0.0),', '[(-0.03, 0.0), (0.1, 0.0),')
seg = seg.replace('[(-0.07, -0.05), (0.08, -0.04), (0.12, 0.0), (0.08, 0.04), (-0.07, 0.05)]', '[(-0.07, -0.05), (0.06, -0.04), (0.075, 0.0), (0.06, 0.04), (-0.07, 0.05)]')
seg = seg.replace('[(-0.07, -0.04), (0.07, -0.038), (0.115, 0.0), (0.07, 0.038), (-0.07, 0.04)]', '[(-0.07, -0.04), (0.06, -0.038), (0.085, 0.0), (0.06, 0.038), (-0.07, 0.04)]')
seg = seg.replace('(0.236, 0.04), (0.12, 0.052), (-0.05, 0.05)]', f'(0.236, 0.04), (0.12, {ay}), (-0.05, 0.05)]')
seg = seg.replace('d.line("bill", "head", [(-0.02, 0.0), (0.13, -0.002)]', 'd.line("bill", "head", [(0.0, 0.0), (0.13, -0.002)]')
open('gull_reps.py', 'w').write(r[:i] + s100 + seg)
s = open('/tmp/ga.bak').read().replace('glide_legs = (-168, -176, -172, -172)', 'glide_legs = (-125, -130, -115, -100)').replace('legs=(-52, -78, -72, -25)', f'legs=({fl})')
open('gull_asset.py', 'w').write(s)
import build_gull
from gapcheck import gaps
A = build_gull.full()
res = []
for kk in ("200", "100", "50"):
    g = gaps(A, kk); seen = set()
    res.append([(x[0], x[1], x[2], x[3]) for x in g if (x[1], x[2], x[0]) not in seen and not seen.add((x[1], x[2], x[0]))])
print(ay, fl, ls100, res)
