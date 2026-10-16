# 🧪️ Scan of every exported macro in the crates of this slice: the first segment after `$crate::` must be a name the crate root offers.
import os, re, sys, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + '/../okwp0-tools'); sys.path.insert(0, HERE + '/../okwp23-tools')
import okcrates as C
from wp1tree import tree
model = C.build()[1]
KEYS = sys.argv[1:] or ['os-store-space-history', 'os-store-text-io', 'os-store-codec-registry', 'os-io', 'os-store-sync', 'os-store-sync-native', 'os-store-sync-wasm']
for k in KEYS:
    root = C.node_abs(k, model) + '/🦀️.rs'
    files = tree([root])
    roottext = open(root, encoding='utf8').read()
    for f in files:
        t = open(f, encoding='utf8').read()
        for m in re.finditer(r'#\[macro_export\][^\n]*\n\s*macro_rules!\s*(\w+)\s*\{', t):
            start = m.end(); depth = 1; i = start
            while depth and i < len(t):
                depth += (t[i] == '{') - (t[i] == '}'); i += 1
            body = t[start:i]
            names = sorted(set(re.findall(r'\$crate::(\w+)', body)))
            missing = []
            for n in names:
                ok = re.search(r'\b(fn|struct|enum|trait|type|mod|const|static|macro_rules!)\s+' + n + r'\b', ''.join(open(x, encoding='utf8').read() for x in files if os.path.dirname(x) == os.path.dirname(root))) or re.search(r'pub\s+use\s[^;]*\b' + n + r'\b', roottext) or re.search(r'pub\s+use\s[^;]*\bas\s+' + n + r'\b', roottext)
                if not ok: missing.append(n)
            print(k, os.path.relpath(f, C.M)[-60:], m.group(1), 'uses', names, 'MISSING' if missing else 'ok', missing)
