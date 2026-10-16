# 🔍 Probe of the sync carve: the unresolved paths of the rewriter and the files that keep kernel-relative `crate::` paths.
import os, sys, re, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import oksql
import carve
b = carve.solve(['os-store-sync', 'os-store-sync-native', 'os-store-sync-wasm'], auto=True)
rep = b.rw.rep
print('unresolved', len(rep.unresolved))
for u in rep.unresolved[:40]: print('  ', str(u)[:240])
for p, t in sorted(b.out.items()):
    n = len(re.findall(r'\bcrate::os_\w+', t))
    if n: print(n, p[-80:])
