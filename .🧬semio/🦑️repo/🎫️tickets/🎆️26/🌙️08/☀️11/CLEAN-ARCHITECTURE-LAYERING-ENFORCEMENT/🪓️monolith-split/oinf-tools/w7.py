import sys; sys.path.insert(0,'.')
from wengine import *
allfns, files = load_world()
defs, unres = resolve_calls(allfns)
apply_hints(allfns, files)
fixpoint(allfns, defs)
tak = [f for f in allfns if f.has_param]
dist = collections.Counter(len(f.req) for f in tak)
print(sorted(dist.items()))
for f in sorted(tak, key=lambda f:-len(f.req))[:40]: print(len(f.req), f, sorted(f.req))
