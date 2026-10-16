# 🌳 Mounted file trees of the WP1 crates (cfg-agnostic #[path]/include! closure from the crate's root files).
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from okwp1sets import ROOTS, M
PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]\s*(?:\n\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)')
INC_RX = re.compile(r'\binclude!\(\s*"([^"]*)"\s*\)')
def tree(roots):
    seen = []; stack = list(roots)
    while stack:
        f = stack.pop(0)
        if f in seen: continue
        seen.append(f)
        t = open(f, encoding='utf8').read()
        base = os.path.dirname(f)
        for m in PATH_RX.finditer(t):
            p = os.path.normpath(os.path.join(base, m.group(1)))
            if p.endswith('.rs') and os.path.exists(p): stack.append(p)
        for m in INC_RX.finditer(t):
            p = os.path.normpath(os.path.join(base, m.group(1)))
            if p.endswith('.rs') and os.path.exists(p): stack.append(p)
    return seen
if __name__ == '__main__':
    for k, roots in ROOTS.items():
        files = tree([M + r for r in roots])
        print('#####', k, len(files))
        for f in files: print('  ', f.replace(M, '').replace('/Users/ueli/Documents/semio/', ''), sum(1 for _ in open(f, encoding='utf8')))
