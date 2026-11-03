"""🔎 E0502 class: `f(&mut x, .., x.method()..)` / `f(&mut x, &x..)` where an explicit `&mut ident` argument is followed by a use of the same ident in a later argument."""
import re, sys, os
sys.path.insert(0, "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️scripts/compile-review-s")
from rs import *
code = sys.argv[1]
CALL = re.compile(r'(?<![\w.])([A-Za-z_][\w:]*)\s*(?:::<[^>]*>)?\(\s*&mut\s+(\w+)\s*(?=[,)])')
for f in rs_files(ART + "/" + CODE[code]):
    try: src = strip(open(f, encoding="utf-8").read())
    except Exception: continue
    for m in CALL.finditer(src):
        if m.group(1) in ("matches", "Some", "Ok", "Err", "Box::new", "RefCell::new"): continue
        o = src.index("(", m.start(1) + len(m.group(1)))
        e = match_close(src, o)
        if e < 0: continue
        args = split_top(src[o + 1:e])
        ident = m.group(2)
        for a in args[1:]:
            if re.search(r'(?<![\w.])' + re.escape(ident) + r'\b(?!\s*:)', a) and not re.match(r'\s*\|', a):
                print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: {m.group(1)}(&mut {ident}, .. {a.strip()[:80]})")
                break
