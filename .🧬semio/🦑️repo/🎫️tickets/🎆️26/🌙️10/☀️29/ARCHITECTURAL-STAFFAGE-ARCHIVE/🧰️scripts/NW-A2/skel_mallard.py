from mallard_asset import *
import sys
a = base()
for pid in sys.argv[1:]:
    skel(a, pid, f"{GEN}/skel-mallard-{pid}.png", den=5)
