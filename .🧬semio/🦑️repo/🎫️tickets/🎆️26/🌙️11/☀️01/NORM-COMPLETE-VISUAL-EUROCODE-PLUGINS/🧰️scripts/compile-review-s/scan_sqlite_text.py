"""🔎 sqlite snapshot writers: Cell::Text(&row.field) / admit_strings(&[&row.field]) / String-only uses of fields that are typed enums."""
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
kinds = {}
for s, defs in structs.items():
    for _, fl in defs:
        for f, t in fl.items():
            b = base(t)
            kinds.setdefault(f, set()).add("enum" if b in enums and b not in structs else "string" if b == "String" else "other")
enum_only = {f for f, k in kinds.items() if k == {"enum"}}
mixed = {f for f, k in kinds.items() if "enum" in k and len(k) > 1}
io = adir + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io"
for f, src in files.items():
    if not f.startswith(io) or "sqlite" not in f or "/🧪️tests/" in f: continue
    for m in re.finditer(r'Cell::Text\(\s*(?:&)?\s*(\w+(?:\.\w+)*)\s*\)|admit_strings\([^;]*?&\[([^\]]*)\]|Cell::Text\(\s*&?(\w+)\.as_str\(\)', src):
        exprs = [m.group(1)] if m.group(1) else re.findall(r'&?\s*(\w+(?:\.\w+)+)', m.group(2) or "") if m.group(2) else [m.group(3)]
        for e in exprs:
            if not e: continue
            fld = e.split(".")[-1]
            tag = "enum" if fld in enum_only else "MIXED" if fld in mixed else None
            if tag and not re.search(r'_text\(\s*' + re.escape(e), src[max(0, m.start()-10):m.end()]):
                out = src.count("\n", 0, m.start()) + 1
                print(f"{rel(f)}:{out}: `{e}` is {tag}-typed but written as text")
