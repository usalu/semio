"""🔎 Struct literals and field uses that still treat converted enums as String (typed-enums v4-v6) or the reverse."""
import re, sys, pickle, os
import os as _o, sys as _s; _s.path.insert(0, _o.path.dirname(_o.path.abspath(__file__)))
from rs import *

code = sys.argv[1]
adir = ART + "/" + CODE[code]
cache = os.path.join(os.path.dirname(__file__), "..", "..", "🗑️generated", "compile-review-s", "norm-enums.pkl")
if os.path.exists(cache):
    ENUMS = pickle.load(open(cache, "rb"))
else:
    _, e, _ = index_types(NORM)
    ENUMS = {k: sorted({x for _, v in vs for x in v}) for k, vs in e.items()}
    pickle.dump(ENUMS, open(cache, "wb"))
structs, enums, files = index_types(adir)
for k, v in ENUMS.items(): enums.setdefault(k, [("", v)])

def base(t):
    t = re.sub(r'^(?:Option|Vec|Box|Arc|Rc)<(.*)>$', r'\1', t)
    t = re.sub(r'^(?:Option|Vec|Box|Arc|Rc)<(.*)>$', r'\1', t)
    return t.split("::")[-1]

def kind(t):
    b = base(t)
    if b in ("String", "&str", "&'staticstr"): return "string"
    if b in enums and b not in structs: return "enum"
    return "other"

fkinds = {}
for s, defs in structs.items():
    for _, flds in defs:
        for f, t in flds.items(): fkinds.setdefault(f, set()).add(kind(t))
has_as_str = set()
for f, src in files.items():
    for m in re.finditer(r'impl\s+(?:<[^>]*>\s*)?(\w+)\s*\{', src):
        e = match_close(src, m.end() - 1)
        if re.search(r'\bfn\s+as_str\b', src[m.end():e]): has_as_str.add(m.group(1))

STRLIKE = re.compile(r'^(?:"S"(?:\.(?:to_string|to_owned|into)\(\))?|String::from\(.*\)|format!\(.*\)|String::new\(\)|.*\.to_string\(\)\s*$)', re.S)
PATH = re.compile(r'^(\w+)::(\w+)(?:\(.*\)|\{.*\})?$', re.S)
out = []
for f, src in files.items():
    if "/node_modules/" in f: continue
    for m in re.finditer(r'(?<![\w:$])([A-Z]\w*)\s*\{', src):
        name = m.group(1)
        if name not in structs: continue
        pre = src[max(0, m.start() - 14):m.start()]
        if re.search(r'(struct|enum|impl|trait|for|mod|dyn|where|->|\bis)\s*$', pre): continue
        e = match_close(src, m.end() - 1)
        if e < 0: continue
        body = src[m.end():e]
        if "$" in body: continue
        defs = [d for _, d in structs[name]]
        for part in split_top(body, angles=False):
            part = part.strip()
            fm = re.match(r'(\w+)\s*:\s*(.+)$', part, re.S)
            if not fm: continue
            fld, expr = fm.group(1), fm.group(2).strip()
            kinds = {kind(d[fld]) for d in defs if fld in d}
            if len(kinds) != 1: continue
            k = kinds.pop()
            line = src.count("\n", 0, m.start()) + 1
            if k == "enum" and STRLIKE.match(expr) and not re.search(r'parse|from_str|ok_or', expr):
                out.append((f, line, f"literal {name}.{fld}: enum field gets string expr `{expr[:60]}`"))
            if k == "string":
                pm = PATH.match(expr)
                if pm and pm.group(1) in enums and pm.group(1) not in ("Option", "Some", "Ok", "Err", "String"):
                    out.append((f, line, f"literal {name}.{fld}: String field gets enum path `{expr[:60]}`"))
for fld, ks in fkinds.items():
    if ks == {"enum"}:
        pats = [r'\.%s\s*[!=]=\s*"S"' % fld, r'\.%s\.(is_empty|len|starts_with|contains|to_lowercase|to_uppercase|eq_ignore_ascii_case|chars|trim)\(' % fld, r'\.%s\.as_str\(\)' % fld, r'"S"\s*[!=]=\s*\w+(?:\.\w+)*\.%s\b' % fld]
    elif ks == {"string"}:
        pats = [r'\.%s\s*[!=]=\s*[A-Z]\w+::[A-Z]\w*\b' % fld]
    else: continue
    for f, src in files.items():
        for pat in pats:
            for m in re.finditer(pat, src):
                if "as_str" in pat and any(e in has_as_str for d in structs.values() for _, fl in d if fld in fl for e in [base(fl[fld])]): continue
                out.append((f, src.count("\n", 0, m.start()) + 1, f"use of `.{fld}` ({'enum' if 'enum' in ks else 'string'} field everywhere): `{m.group(0)[:70]}`"))
for f, l, msg in sorted(set(out)):
    print(f"{rel(f)}:{l}: {msg}")
print(f"# {len(set(out))} findings", file=sys.stderr)
