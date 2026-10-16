import sys,os
HERE=os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0,HERE)
import carve
crates=sys.argv[1:]
b=carve.solve(crates)
for k,v in sorted(b.rw.rep.cross_examples.items()):
    print(k, b.rw.rep.cross.get(k), v[:8])
