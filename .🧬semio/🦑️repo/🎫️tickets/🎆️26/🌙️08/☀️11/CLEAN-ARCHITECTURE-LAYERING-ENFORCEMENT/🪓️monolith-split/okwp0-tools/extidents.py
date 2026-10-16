# 🔎 External crate identifiers used by each WP1 file (token level): first path segments before `::` that are crate-like.
import os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from okrs import tokenize
from okwp1sets import ROOTS, M
from wp1tree import tree
LOCAL = {'crate', 'super', 'self', 'std', 'core', 'alloc', 'Self'}
def idents(path):
    src = open(path, encoding='utf8').read()
    toks = [t for t in tokenize(src) if t[0] not in ('ws', 'comment')]
    out = collections.Counter()
    for i, t in enumerate(toks):
        if t[0] == 'id' and i + 2 < len(toks) and toks[i + 1][1] == ':' and toks[i + 2][1] == ':':
            prev = toks[i - 1][1] if i else ''
            if prev == ':' and i >= 2 and toks[i - 2][1] == ':' : 
                if i >= 3 and toks[i-3][0] == 'id': continue
            nm = t[1]
            if nm in LOCAL: continue
            if nm.startswith('semio_framework') or nm in ('serde', 'serde_json', 'protocol', 'pack', 'vcs', 'dsl_derive', 'value_derive', 'blake3', 'tokio', 'futures', 'web_sys', 'js_sys', 'wasm_bindgen', 'dsl', 'dsl_grammar', 'semio_format', 'spr', 'store', 'ureq', 'tempfile', 'regex'):
                out[nm] += 1
    return out
if __name__ == '__main__':
    for k, roots in ROOTS.items():
        files = tree([M + r for r in roots])
        print('#####', k)
        agg = collections.defaultdict(collections.Counter)
        for f in files:
            kind = 'test' if '/🧪️test' in f or 'tests/' in f else 'lib'
            for nm, n in idents(f).items(): agg[nm][kind] += n
        for nm, c in sorted(agg.items()): print('  ', nm, dict(c))
