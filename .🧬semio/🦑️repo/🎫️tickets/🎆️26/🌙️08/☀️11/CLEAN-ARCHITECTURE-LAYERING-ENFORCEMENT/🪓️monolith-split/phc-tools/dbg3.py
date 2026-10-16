import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, hcmod, carve
from carve import Carver
ws, E, base, base_ws = plan_host.main()
roots = plan_host.prepare_roots()
C = Carver(ws, E, base)
C.build_new(roots)
SH = plan_host.P('🧵️shard/🦀️.rs')
fm = C.new.by_file[SH]
toks = fm.toks
for i,t in enumerate(toks):
    if t.x=='BrokerCapabilityGrant' and toks[i-1].x==':' and toks[i-3].x=='super':
        S = C.scope_at(fm, i-3)
        segs=['super','BrokerCapabilityGrant']
        r, om = C.old_resolve(S, segs)
        print('old_resolve', r, getattr(r,'ext',None), getattr(r,'rest',None), r.kind)
        print(C.patch_qualified(S, fm, toks, i-3, i+1, segs))
        break
