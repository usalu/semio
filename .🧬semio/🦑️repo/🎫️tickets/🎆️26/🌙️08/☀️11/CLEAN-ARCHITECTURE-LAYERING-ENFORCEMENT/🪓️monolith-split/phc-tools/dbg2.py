import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, hcmod, carve
from carve import Carver
ws, E, base, base_ws = plan_host.main()
roots = plan_host.prepare_roots()
C = Carver(ws, E, base)
C.build_new(roots)
print('pass1', C.pass1())
G = plan_host.P('🎭️guest-runtime/🦀️.rs')
print(ws.read(G)[:600])
C.build_new(roots)
print('pass2', C.pass2())
print(ws.read(G)[:1500])
C.report.dump()
