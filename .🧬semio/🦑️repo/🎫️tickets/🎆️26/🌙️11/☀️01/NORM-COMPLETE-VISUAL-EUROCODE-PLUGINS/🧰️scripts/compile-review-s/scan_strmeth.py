"""🔎 String-only methods applied to a field name that is enum-typed in at least one struct of the artifact (ambiguous names included) — manual review list."""
import re, sys, os, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, files = index_types(adir)
cache = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "norm-enums.pkl")
for k, v in pickle.load(open(cache, "rb")).items(): enums.setdefault(k, [("", v)])
def base(t):
    for _ in range(3): t = re.sub(r'^(?:Option|Vec|Box)<(.*)>$', r'\1', t)
    return t.split("::")[-1]
ek = {}
for s, defs in structs.items():
    for _, fl in defs:
        for f, t in fl.items():
            b = base(t)
            ek.setdefault(f, {})[s] = b
STR = "trim trim_start trim_end to_ascii_lowercase to_ascii_uppercase to_lowercase to_uppercase replace contains starts_with ends_with split chars len is_empty as_bytes eq_ignore_ascii_case parse lines find strip_prefix strip_suffix".split()
PAT = re.compile(r'\.(\w+)\.(' + "|".join(STR) + r')\s*\(')
has_as_str = {t for t in enums if True}
for f, src in files.items():
    for m in PAT.finditer(src):
        fld, meth = m.groups()
        tys = ek.get(fld)
        if not tys: continue
        en = {s: b for s, b in tys.items() if b in enums and b not in structs}
        if not en: continue
        if len(en) == len(tys) or True:
            print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `.{fld}.{meth}(` — enum in {sorted(en.items())[:3]}{' (+ others string)' if len(en) < len(tys) else ''}")
