import sys, glob; sys.path.insert(0,'.')
from woracle import *
a = sys.argv[1]; b = sys.argv[2]
tags = sys.argv[3].split(',')
def load(prefix, tag): return norm(open(f"{SCR}/err-{prefix}-{tag}.json").read())
base = {t: load(a, t) for t in tags}; cur = {t: load(b, t) for t in tags}
compare(base, cur)
