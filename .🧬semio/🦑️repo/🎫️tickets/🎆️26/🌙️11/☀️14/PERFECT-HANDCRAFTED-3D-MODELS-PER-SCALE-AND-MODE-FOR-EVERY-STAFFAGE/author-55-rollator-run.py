import sys; sys.path.insert(0, ".")
from author_55_lib import *
import author_55_rollator as R
sl = R.SLUG
def apply():
    d = load(sl); restore_bones(sl, d, {"neck"})
    for n in (50, 100, 200, 500):
        v = d["volumes"][str(n)]; skinfix(v, R.SKIN); b, t, st = R.props(n)
        replace_props(v, R.FB, R.REG, b, t, st); add_decisions(v, *R.D[n])
    for n in (50, 100, 200, 500): dedupe(d["volumes"][str(n)])
    R.post(d); save(sl, d)
if "build" in sys.argv:
    print(run_build(sl)); apply()
    for n, ex in ((100, ()), (200, ("--sections", "--shift")), (500, ("--sections", "--shift"))):
        print(fit(sl, n, R.FB, 3, ex))
else: apply()
print(fails(sl))
