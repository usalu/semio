"""🧱 c4p23 overlay builder: merges the per-target stage dirs (`c4p23-stage/<key>/<repo-relative path>`) into one repo-shaped overlay `c4p23-overlay/<repo-relative path>` for the typeck oracle and the expansion differential.
usage: overlay.py"""
import os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from c4plib import *

OV = GEN + "/c4p23-overlay"
if os.path.exists(OV):
    shutil.rmtree(OV)
n = 0
for key in sorted(os.listdir(STAGE)):
    sd = f"{STAGE}/{key}"
    if not os.path.isdir(sd):
        continue
    for dp, dn, fn in os.walk(sd):
        for f in fn:
            s = os.path.join(dp, f)
            rel = os.path.relpath(s, sd)
            d = os.path.join(OV, rel)
            os.makedirs(os.path.dirname(d), exist_ok=True)
            shutil.copyfile(s, d)
            n += 1
print("overlay files", n)
