import sys, os, importlib
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import lib, verify

# usage: go.py <asset-id> <module> [scales...]   (module has BUILD={scale: fn} and NOTES={scale:(en,de)})
aid, modname = sys.argv[1], sys.argv[2]
scales = [int(x) for x in sys.argv[3:]]
mod = importlib.import_module(modname)
scales = scales or sorted(mod.BUILD)
for sc in scales:
    F, P = mod.BUILD[sc]()
    lib.commit(mod.A, sc, F.shapes, P.shapes, mod.NOTES[sc][0], mod.NOTES[sc][1])
    print("committed", aid, sc, len(F.shapes), len(P.shapes))
bad = 0
for sc in scales:
    lib.run_eval(aid, sc, 5)
    bad += verify.verify(aid, sc)
print("TOTAL problems", bad)
