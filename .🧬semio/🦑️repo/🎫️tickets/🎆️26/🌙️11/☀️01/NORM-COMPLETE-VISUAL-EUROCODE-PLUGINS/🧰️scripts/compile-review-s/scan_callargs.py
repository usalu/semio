"""🔎 Call sites passing an enum-typed field where the callee parameter is a string (or a string field where the parameter is an enum)."""
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
def kind(b):
    return "string" if b in ("String", "&str") else "enum" if b in enums and b not in structs else "other"
fk = {}
for s, defs in structs.items():
    for _, fl in defs:
        for f, t in fl.items():
            if t.startswith(("Option", "Vec")): continue
            fk.setdefault(f, set()).add(kind(base(t)))
enum_only = {f for f, k in fk.items() if k == {"enum"}}
string_only = {f for f, k in fk.items() if k == {"string"}}
sigs = {}
FN = re.compile(r'\bfn\s+(\w+)\s*(?:<[^>]*>)?\s*\(')
for f, src in files.items():
    for m in FN.finditer(src):
        e = match_close(src, m.end() - 1)
        if e < 0: continue
        params = [p for p in split_top(src[m.end():e]) if p.strip() and "self" not in p.split(":")[0]]
        ptypes = []
        for p in params:
            pm = re.match(r'\s*(?:mut\s+)?\w+\s*:\s*(.+)$', p, re.S)
            ptypes.append(re.sub(r'\s+', '', pm.group(1)) if pm else "?")
        sigs.setdefault(m.group(1), []).append(ptypes)
CALL = re.compile(r'(?<![\w.])(?:\w+::)*([a-z_]\w*)\s*\(')
for f, src in files.items():
    for m in CALL.finditer(src):
        name = m.group(1)
        if name not in sigs or len(sigs[name]) != 1: continue
        pre = src[max(0, m.start() - 4):m.start()]
        if re.search(r'fn\s*$', src[max(0, m.start() - 6):m.start()]): continue
        e = match_close(src, m.end() - 1)
        if e < 0: continue
        args = split_top(src[m.end():e], angles=False)
        ptypes = sigs[name][0]
        if len(args) != len(ptypes): continue
        for a, pt in zip(args, ptypes):
            am = re.fullmatch(r'\s*(&)?\s*(?:\w+(?:\.\w+)*?\.)(\w+)(\.clone\(\))?\s*', a)
            if not am: continue
            fld = am.group(2)
            pk = "string" if re.fullmatch(r'(&(?:\'\w+)?(?:mut)?str|String|&String|&\'\w+str|impl(Into<String>|AsRef<str>)|Option<&str>)', pt) else "enum" if base(pt.lstrip("&")) in enums and base(pt.lstrip("&")) not in structs else "other"
            if fld in enum_only and pk == "string":
                print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `{name}(.. {a.strip()} ..)` enum field passed for `{pt}`")
            if fld in string_only and pk == "enum":
                print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `{name}(.. {a.strip()} ..)` String field passed for `{pt}`")
