"""🔎 `param.method(` where the fn parameter has a unique named type T (enum/struct of the repo) and no impl of T, repo trait or blanket impl defines `method`."""
import re, sys, os, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *
cache = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "impl-index.pkl")
METH, BLANKET = pickle.load(open(cache, "rb"))
STD = set("clone to_string to_owned into try_into eq ne cmp partial_cmp hash fmt as_ref as_mut borrow default to_value from_value iter into_iter len is_empty map unwrap expect max min clamp cloned copied is_some is_none ok_or ok_or_else and_then".split())
code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, files = index_types(adir)
shared_s, shared_e, _ = ({}, {}, None)
cache2 = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "norm-types.pkl")
if os.path.exists(cache2): known = pickle.load(open(cache2, "rb"))
else:
    s2, e2, _ = index_types(NORM)
    known = {"structs": {k: [set(fl) for _, fl in v] for k, v in s2.items()}, "enums": set(e2)}
    pickle.dump(known, open(cache2, "wb"))
FN = re.compile(r'\bfn\s+(\w+)\s*(?:<[^>]*>)?\s*\(')
derived = {}
for f, src in files.items():
    for m in re.finditer(r'#\[derive\(([^\)]*)\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum)\s+(\w+)', src):
        derived.setdefault(m.group(2), set()).update(x.strip().split("::")[-1] for x in m.group(1).split(","))
for f, src in files.items():
    for m in FN.finditer(src):
        pe = match_close(src, m.end() - 1)
        if pe < 0: continue
        rest = src[pe + 1:pe + 300]
        bm = re.match(r'[^{;]*?(\{)', rest, re.S)
        if not bm or ";" in rest[:bm.start(1)]: continue
        bs = pe + 1 + bm.start(1)
        be = match_close(src, bs)
        if be < 0: continue
        body = src[bs + 1:be]
        for p in split_top(src[m.end():pe]):
            pm = re.match(r'\s*(?:mut\s+)?(\w+)\s*:\s*&?\s*(?:mut\s+)?(?:\'\w+\s+)?((?:\w+::)*)([A-Z]\w*)\s*$', p)
            if not pm: continue
            name, T = pm.group(1), pm.group(3)
            if T not in enums and T not in structs and T not in known["enums"] and T not in known["structs"] and T not in METH: continue
            ms = METH.get(T, set())
            for cm in re.finditer(r'(?<![\w.])%s\.(\w+)\s*(?:::<[^>]*>)?\(' % re.escape(name), body):
                meth = cm.group(1)
                if meth in STD or meth in BLANKET or meth in ms: continue
                if T in structs and any(meth in fl for _, fl in structs[T]): continue
                print(f"{rel(f)}:{src.count(chr(10), 0, bs + 1 + cm.start()) + 1}: `{name}.{meth}()` — type {T} has no method {meth}")
