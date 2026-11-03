"""🔎 Derive coverage: a struct/enum deriving Default/Copy/Eq/Hash/Ord/PartialEq/Clone/Debug whose field type is a repo enum/struct lacking that derive (E0277); enum Default without #[default]."""
import re, sys, os
S = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/compile-review-s"
sys.path.insert(0, S)
from rs import *
code = sys.argv[1]
adir = ART + "/" + CODE[code]
ITEM = re.compile(r'((?:\s*#\[[^\]]*\])*)\s*(?:pub(?:\([^)]*\))?\s+)?(struct|enum)\s+(\w+)\s*(<[^>{(;]*>)?\s*(\{|\()')
defs = {}
MAN = {}
files = {}
for f in rs_files(adir):
    try: s = strip(open(f, encoding="utf-8").read())
    except Exception: continue
    files[f] = s
    for m in ITEM.finditer(s):
        attrs, kind, name, gen, op = m.groups()
        if gen: continue
        der = set(re.findall(r'\b(\w+)\b', " ".join(re.findall(r'derive\(([^)]*)\)', attrs))))
        e = match_close(s, m.end() - 1)
        if e < 0: continue
        body = s[m.end():e]
        defs.setdefault(name, []).append((f, kind, der, body, attrs, s.count("\n", 0, m.start()) + 1))
    for m in re.finditer(r'\bimpl\s*(?:<[^>]*>)?\s*(Default|Clone|Copy|PartialEq|Eq|Hash|Ord|PartialOrd|Debug)\s+for\s+(\w+)', s):
        MAN.setdefault(m.group(2), set()).add(m.group(1))
CHECK = ["Default", "Copy", "Eq", "Hash", "Ord", "PartialEq", "PartialOrd", "Clone", "Debug"]
def types_of(kind, body, op=None):
    out = []
    if kind == "enum":
        for part in split_top(body):
            part = re.sub(r'#\[[^\]]*\]', '', part)
            for t in re.findall(r'(?::|\(|,)\s*([A-Za-z_][\w:]*(?:<[^>]*>)?)', part): out.append(t)
    else:
        for part in split_top(body):
            part = re.sub(r'#\[[^\]]*\]', '', part).strip()
            m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(?:\w+\s*:\s*)?(.+)$', part, re.S)
            if m: out.append(re.sub(r'\s+', '', m.group(1)))
    return out
def leaf_names(t): return re.findall(r'[A-Z]\w*', t)
for name, ds in defs.items():
    for f, kind, der, body, attrs, line in ds:
        if kind == "enum" and "Default" in der and "#[default]" not in re.sub(r'\s+', '', attrs + body).replace("#[default]", "#[default]") and "#[default]" not in body:
            print(f"enum-default-no-marker {rel(f)}:{line}: {name}")
        for t in types_of(kind, body):
            for ln in leaf_names(t):
                if ln not in defs or len(defs[ln]) != 1 or ln == name: continue
                _, k2, d2, b2, _, l2 = defs[ln][0]
                d2 = d2 | MAN.get(ln, set())
                if t.startswith(("Vec<", "BTreeMap<", "HashMap<", "BTreeSet<", "HashSet<", "Box<", "[")) and "Default" in der and False: continue
                for d in CHECK:
                    if d in der and d not in d2 and not (d == "Default" and k2 == "struct" and False):
                        if d == "Default" and re.sub(r'(Option|Vec|Box)<.*', '', t) != t and t.startswith(("Option<", "Vec<")): continue
                        if t.startswith(("Vec<", "Option<", "Box<", "BTreeMap<", "HashMap<", "BTreeSet<", "HashSet<", "[")) and d in ("Default", "Copy"): continue
                        if d in ("Eq", "Hash", "Ord", "PartialOrd", "PartialEq", "Clone", "Debug", "Copy", "Default"):
                            print(f"missing-derive {rel(f)}:{line}: {name} derives {d} but field type {ln} does not ({rel(defs[ln][0][0])}:{l2})")
