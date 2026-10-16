import sys, pickle; sys.path.insert(0,'.')
from woracle import *
src = sys.argv[1]; tags = sys.argv[2].split(',') if len(sys.argv) > 2 else None
root = mksb('cur', src)
print('neutralized', neutralize(root))
res = runall(root, tags)
base = pickle.load(open(SCR + '/base.pkl', 'rb'))
base = {k: v for k, v in base.items() if k in res}
compare(base, res)
