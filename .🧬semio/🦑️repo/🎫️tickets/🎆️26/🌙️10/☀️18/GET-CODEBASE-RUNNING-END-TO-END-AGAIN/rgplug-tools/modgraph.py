import sys, json, re, os, subprocess
sys.path.insert(0, os.path.dirname(__file__))
from rgp import *
m = sys.argv[1]
files = closure(m)
base = os.path.dirname(os.path.dirname(os.path.dirname(m)))
for f in files:
    if not f["p"].endswith(".rs"): continue
    t = rd(f["p"])
    sup = len(re.findall(r'\bsuper::', t)); glob = len(re.findall(r'use super::\*', t)); cr = len(re.findall(r'\bcrate::', t))
    sl = sorted(set(re.findall(r'\bcrate::([a-z_0-9]+(?:::[a-z_0-9]+)?)', t)))[:6]
    print("%5d %7d super=%d glob=%d crate=%d %s %s" % (f["l"], f["b"], sup, glob, cr, f["p"].replace(base, "~")[-70:], sl))
