"""🔎 Framework derive coverage: a struct/enum deriving ToValue/FromValue/CanonicalJsonTree/RetireOwned/RetainedClone/DslRecord whose field type (a repo enum/struct of the same artifact) lacks that derive (E0277)."""
import re, sys, os
S = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/compile-review-s"
sys.path.insert(0, S)
from rs import *
code = sys.argv[1]
adir = ART + "/" + CODE[code]
ITEM = re.compile(r'((?:\s*#\[[^\]]*\])*)\s*(?:pub(?:\([^)]*\))?\s+)?(struct|enum)\s+(\w+)\s*(<[^>{(;]*>)?\s*(\{|\()')
D = ["ToValue", "FromValue", "CanonicalJsonTree", "RetireOwned", "RetainedClone"]
defs = {}
for f in rs_files(adir):
    try: s = strip(open(f, encoding="utf-8").read(), keep_strings=False)
    except Exception: continue
    for m in ITEM.finditer(s):
        attrs, kind, name, gen, op = m.groups()
        if gen: continue
        der = set(re.findall(r'\w+', " ".join(re.findall(r'derive\(([^)]*)\)', attrs))))
        e = match_close(s, m.end() - 1)
        if e < 0: continue
        defs.setdefault(name, []).append((f, kind, der, s[m.end():e], s.count("\n", 0, m.start()) + 1))
def ftypes(kind, body):
    out = []
    if kind == "enum":
        for part in split_top(body):
            part = re.sub(r'#\[[^\]]*\]', '', part)
            out += re.findall(r'(?::|\(|,)\s*([A-Za-z_][\w:]*(?:<[^>]*>)?)', part)
    else:
        for part in split_top(body):
            part = re.sub(r'#\[[^\]]*\]', '', part).strip()
            m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(?:\w+\s*:\s*)?(.+)$', part, re.S)
            if m: out.append(re.sub(r'\s+', '', m.group(1)))
    return out
for name, ds in defs.items():
    for f, kind, der, body, line in ds:
        for t in ftypes(kind, body):
            for ln in set(re.findall(r'[A-Z]\w*', t)):
                if ln not in defs or len(defs[ln]) != 1 or ln == name: continue
                d2 = defs[ln][0][2]
                for d in D:
                    if d in der and d not in d2:
                        print(f"{rel(f)}:{line}: {name} derives {d}; field type {ln} does not ({rel(defs[ln][0][0])}:{defs[ln][0][4]})")
