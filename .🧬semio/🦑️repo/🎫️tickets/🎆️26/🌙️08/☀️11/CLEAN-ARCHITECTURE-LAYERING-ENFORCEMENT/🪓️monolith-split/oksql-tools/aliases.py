# 🧭️ Kernel-alias scan: paths in a crate's mounted tree that only resolve through the kernel facade (`store::`, `crate::os_*`, `crate::io::`, ...), which a real crate cannot see.
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools')
import okcrates as C
from wp1tree import tree
from okrs import tokenize
model = C.build()[1]
KEYS = sys.argv[1:] or ['os-store-space-history', 'os-store-text-io', 'os-store-codec-registry', 'os-io']
RX = re.compile(r'(?<![\w:])(store|vcs|spr|dsl|dsl_grammar|semio_format|semio_framework_os_kernel)::|\bcrate::(os_\w+|io|io_schema|sqlite_snapshot|test_support|extension|os_store)\b')
for k in KEYS:
    root = C.node_abs(k, model) + '/🦀️.rs'
    files = tree([root]); hits = collections.defaultdict(list)
    for f in files:
        text = re.sub(r'(?m)^\s*//.*$', '', open(f, encoding='utf8').read())
        for m in RX.finditer(text):
            hits[os.path.relpath(f, C.M)].append(text[max(0, m.start() - 20): m.end() + 40])
    print('#####', k, len(files), 'files', sum(len(v) for v in hits.values()), 'hits')
    for f, v in sorted(hits.items()): print('  ', len(v), f[-80:], '|', v[0][:110])
