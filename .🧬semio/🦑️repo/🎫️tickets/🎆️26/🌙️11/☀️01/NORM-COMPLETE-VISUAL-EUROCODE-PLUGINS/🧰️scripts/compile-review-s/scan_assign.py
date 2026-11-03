"""🔎 Field assignments / pushes mixing String and typed enums: `x.field = "..".into()` for enum fields, `x.field = Enum::V` for String fields."""
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
fk = {}
for s, defs in structs.items():
    for _, fl in defs:
        for f, t in fl.items():
            if t.startswith(("Option", "Vec")): continue
            b = base(t)
            fk.setdefault(f, set()).add("string" if b == "String" else "enum" if b in enums and b not in structs else "other")
enum_only = {f for f, k in fk.items() if k == {"enum"}}
string_only = {f for f, k in fk.items() if k == {"string"}}
for f, src in files.items():
    for m in re.finditer(r'\.(\w+)\s*=\s*([^;=]{1,80});', src):
        fld, rhs = m.group(1), m.group(2).strip()
        if fld in enum_only and re.match(r'("S"(\.into\(\)|\.to_string\(\)|\.to_owned\(\))?|String::from\(|format!\()', rhs):
            print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: enum field `.{fld}` assigned string `{rhs[:50]}`")
        if fld in string_only:
            pm = re.fullmatch(r'(\w+)::(\w+)', rhs)
            if pm and pm.group(1) in enums and pm.group(1) not in structs:
                print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: String field `.{fld}` assigned enum `{rhs[:50]}`")
