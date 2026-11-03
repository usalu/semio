import sys, os, importlib
A2 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "🧰️A2")
sys.path.insert(0, A2)
import lib
# usage: rebuild.py <gen-module> [scales...]  -- regenerates front/plan from the A2 builders without evaluating (the queued driver verifies)
mod = importlib.import_module(sys.argv[1])
scales = [int(x) for x in sys.argv[2:]] or sorted(mod.BUILD)
for sc in scales:
    F, P = mod.BUILD[sc]()
    lib.commit(mod.A, sc, F.shapes, P.shapes, mod.NOTES[sc][0], mod.NOTES[sc][1])
    print("committed", mod.A.aid, sc, len(F.shapes), len(P.shapes))
