import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import plan_host, hcmod, carve
from carve import Carver
ws, E, base, base_ws = plan_host.main()
roots = plan_host.prepare_roots()
C = Carver(ws, E, base)
C.build_new(roots)
G = plan_host.P('🎭️guest-runtime/🦀️.rs')
fm = C.new.by_file[G]
print('G uses', [(u.path, u.glob) for u in fm.uses][:10])
C.activate(C.new)
print('new lookup PackageId', hcmod.lookup(fm, 'PackageId'))
r, om = C.old_lookup(fm, 'PackageId')
print('old lookup', r, om)
oms = C.old_mods_for(fm)
print('old mods', [o.file[-60:] for o in oms], [[(u.path,u.glob) for u in o.uses] for o in oms][:1])
n = C.pass2(only_files={G})
print('pass2', n)
