import sys; sys.path.insert(0, ".")
from author_55_lib import *
import author_55_cane as C
sl = C.SLUG
if "build" in sys.argv:
    print(run_build(sl))
def props():
    d = load(sl); restore_bones(sl, d, {"neck"}); 
    for n in (50, 100, 200, 500):
        v = d["volumes"][str(n)]; skinfix(v, C.SKIN); b, t, st = C.cane(n)
        replace_props(v, {"cane"}, {"cane-shaft", "cane-grip", "cane-tip"}, b, t, st); add_decisions(v, *C.D[n])
    C.post(d); save(sl, d)
if "build" in sys.argv:
    d = load(sl); restore_bones(sl, d, {"neck"}); save(sl, d)
    props()
    for n, ex in ((100, ()), (200, ("--sections", "--shift")), (500, ("--sections", "--shift"))):
        print(fit(sl, n, {"cane"}, 3, ex))
print(fails(sl))
