"""🔎 Enum-typed field compared with a string literal, matched against string patterns, or used as Vec<String>/&str: `x.f == "a"`, `match x.f { "a" =>`, `.contains(&x.f)`, `.join` / `.push(x.f.clone())` candidates. certain = every struct field of that name is enum-typed."""
import re, sys, os, pickle
S = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/compile-review-s"
sys.path.insert(0, S)
from rs import *
code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, files = index_types(adir)
cache = S + "/../../🗑️generated/compile-review-s/norm-enums.pkl"
for k, v in pickle.load(open(cache, "rb")).items(): enums.setdefault(k, [("", v)])
def base(t):
    for _ in range(3): t = re.sub(r'^(?:Option|Vec|Box)<(.*)>$', r'\1', t)
    return t.split("::")[-1]
ek = {}
for s, defs in structs.items():
    for _, fl in defs:
        for f, t in fl.items():
            if t.startswith("Vec<") or t.startswith("Option<"): continue
            ek.setdefault(f, {})[s] = base(t)
def cls(fld):
    tys = ek.get(fld)
    if not tys: return None
    en = [s for s, b in tys.items() if b in enums and b not in structs]
    if not en: return None
    return "certain" if len(en) == len(tys) else "mixed"
P = [
 (re.compile(r'\.(\w+)\s*(?:==|!=)\s*"S"'), "cmp-literal"),
 (re.compile(r'"S"\s*(?:==|!=)\s*[\w&*]*(?:\w+\.)+(\w+)\b(?!\s*[(\w])'), "cmp-literal-rev"),
 (re.compile(r'\bmatch\s+[^{};]*?\.(\w+)\s*\{\s*(?:\|\s*)?"S"'), "match-literal"),
 (re.compile(r'\.contains\(\s*&?\s*\w+(?:\.\w+)*\.(\w+)\s*\)'), "contains-field"),
]
for f, src in files.items():
    for rx, kind in P:
        for m in rx.finditer(src):
            c = cls(m.group(1))
            if c is None: continue
            line = src.count("\n", 0, m.start()) + 1
            print(f"{c} {kind} {rel(f)}:{line}: .{m.group(1)}")
