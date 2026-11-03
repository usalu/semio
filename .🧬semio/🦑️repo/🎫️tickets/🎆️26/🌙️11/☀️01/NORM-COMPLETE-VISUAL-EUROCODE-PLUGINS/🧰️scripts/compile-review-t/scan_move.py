"""🔎 E0507 class: a non-Copy enum-typed field (all structs with that field name agree) used by value from a path (`k: row.kind,` / `f(row.kind)` / `let x = row.kind;`) without `&`/`.clone()`/`match`; the receiver may be an owned value, so this is a candidate list."""
import re, sys, os, pickle
S = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/compile-review-s"
sys.path.insert(0, S)
from rs import *
code = sys.argv[1]
adir = ART + "/" + CODE[code]
structs, enums, files = index_types(adir)
ITEM = re.compile(r'((?:\s*#\[[^\]]*\])*)\s*(?:pub(?:\([^)]*\))?\s+)?enum\s+(\w+)')
nocopy = set()
for f, src in files.items():
    for m in ITEM.finditer(src):
        der = " ".join(re.findall(r'derive\(([^)]*)\)', m.group(1)))
        if "Copy" not in re.findall(r'\w+', der): nocopy.add(m.group(2))
for k, v in pickle.load(open(S + "/../../🗑️generated/compile-review-s/norm-enums.pkl", "rb")).items(): pass
ek = {}
for s, defs in structs.items():
    for _, fl in defs:
        for fld, t in fl.items(): ek.setdefault(fld, set()).add(t)
cand = {f for f, ts in ek.items() if ts and all(t in nocopy and t in enums for t in ts)}
P = [re.compile(r'(?:^|[({,]\s*|:\s*|=\s*)(?<![&.\w])([a-z_]\w*(?:\.\w+)*)\.(' + "|".join(map(re.escape, sorted(cand))) + r')\s*(?=[,;})])') ] if cand else []
for f, src in files.items():
    for rx in P:
        for m in rx.finditer(src):
            line_start = src.rfind("\n", 0, m.start()) + 1
            ctx = src[line_start:m.end() + 10]
            if re.search(r'(==|!=|matches!|match\s|if let|while let|=>)\s*[^,;]*$', src[line_start:m.start() + 1]): continue
            print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: {m.group(1)}.{m.group(2)}")
