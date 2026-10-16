import json, os, subprocess, sys
from common import *
import model as M
import widen as W
W.make_view()
roots = {k: M.ROOT[k] for k in M.WP2}
env = dict(os.environ, VIEW=W.VIEW.rsplit('/🧰️framework', 1)[0], ROOTS=json.dumps(roots))
p = subprocess.run(['bun', os.path.dirname(os.path.abspath(__file__)) + '/census_view.ts'], capture_output=True, text=True, env=env)
print(p.stdout + p.stderr[-800:])
