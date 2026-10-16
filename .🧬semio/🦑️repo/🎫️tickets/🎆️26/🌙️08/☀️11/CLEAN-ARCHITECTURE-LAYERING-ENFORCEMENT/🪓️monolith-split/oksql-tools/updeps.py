# 🧭 Upward-reference scan of the four SQLite-host crates: names every extern crate each mounted file mentions and flags edges that point up the rank order or to the kernel.
# usage: updeps.py [crate-key...]
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools')
import okcrates as C
from wp1tree import tree
model = C.build()[1]
KEYS = sys.argv[1:] or ['os-store-space-history', 'os-store-text-io', 'os-store-codec-registry', 'os-io']
RANKI = {k: i for i, k in enumerate(C.RANK)}
IDENT = {C.crate_name(k).replace('-', '_'): k for k in C.RANK}
for k in KEYS:
    files = [f for f in tree([C.node_abs(k, model) + '/🦀️.rs'])]
    hits = collections.defaultdict(set)
    for f in files:
        t = open(f, encoding='utf8').read()
        for m in re.finditer(r'\b(semio_framework_os_[a-z0-9_]+)\b', t):
            if m.group(1) in IDENT: hits[IDENT[m.group(1)]].add(os.path.relpath(f, C.M)[:90])
        if re.search(r'\bsemio_framework_os_kernel\b', t): hits['KERNEL'].add(os.path.relpath(f, C.M)[:90])
    print('#####', k, len(files), 'files')
    for h, fs in sorted(hits.items()):
        up = h == 'KERNEL' or RANKI.get(h, -1) >= RANKI[k]
        print('  ', 'UP  ' if up else 'down', h, len(fs), sorted(fs)[:3] if up else '')
