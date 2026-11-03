"""🔎 `recv.field` accesses (not method calls) in hand-written io/scene/editor code whose field name exists in no struct of the artifact (or the shared scene/compliance crates)."""
import re, sys, os, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, files = index_types(adir)
cache = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "all-fields.pkl")
if os.path.exists(cache): ALL = pickle.load(open(cache, "rb"))
else:
    s2, _, _ = index_types(NORM)
    ALL = {f for defs in s2.values() for _, fl in defs for f in fl}
    pickle.dump(ALL, open(cache, "wb"))
fields = set(ALL)
for s, defs in structs.items():
    for _, fl in defs: fields |= set(fl)
SKIP = set("len is_empty iter 0 1 2 3 4 5 6 7 8 9".split())
for f, src in files.items():
    if "/🧪️tests/" in f: continue
    for m in re.finditer(r'(?<![\w.:])([a-z_]\w*(?:\[[^\]]*\])?)\.([a-z_]\w*)\b(?!\s*[(:!])', src):
        fld = m.group(2)
        if fld in fields or fld in SKIP: continue
        if re.match(r'(std|core|self|crate|super|use|mod|pub|fn|let)$', m.group(1)): continue
        line = src.count(chr(10), 0, m.start()) + 1
        print(f"{rel(f)}:{line}: `{m.group(0)}` — no struct has field `{fld}`")
