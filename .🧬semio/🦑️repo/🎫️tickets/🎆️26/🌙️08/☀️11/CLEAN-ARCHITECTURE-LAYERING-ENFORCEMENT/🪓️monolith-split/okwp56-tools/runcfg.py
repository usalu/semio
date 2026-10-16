# ▶️ Runs further oracle configs on an existing sandbox A of u56.py (same tag): usage runcfg.py <tag> <cfg,cfg>
import sys, os, json
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools'); sys.path.insert(0, HERE)
import okoracle as O
O.GEN = HERE + '/../🗑️generated'
tag = sys.argv[1]
A = O.SCR + '/ok-union-' + tag + '/A'
assert os.path.isdir(A), A
for name in sys.argv[2].split(','):
    ea, ta = O.run(A, 'A', name, tag)
    a, ae, aex = O.parse_keyed(ea, A, lambda f: f)
    print(f'== {name}: A internal={sum(a.values())} external={sum(ae.values())} sec={round(ta)}', flush=True)
