import importlib, sys, os
from lib import *
from pv import render

OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/V2/"
os.makedirs(OUT, exist_ok=True)
aid = sys.argv[1]
mode = sys.argv[2] if len(sys.argv) > 2 else "preview"
mod = importlib.import_module("spec_" + aid.replace("-", "_"))
b = Builder(aid)
mod.build(b)
b.report()
for sc in SCALES:
    render(b.asset, b.views[sc], sc, OUT + "%s-%s.png" % (aid, sc))
if mode == "write":
    b.finish(mod.decisions())
    print("written", aid)
