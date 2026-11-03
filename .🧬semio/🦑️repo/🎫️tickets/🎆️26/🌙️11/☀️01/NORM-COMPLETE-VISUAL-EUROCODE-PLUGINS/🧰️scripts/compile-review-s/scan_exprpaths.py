"""🔎 Expression/type paths `semio_crate::module::...::item` and `crate::module::item` whose module chain or final item cannot be found in the target crate's pub surface."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import scan_resolve as S
from rs import *
S.load_crates()
code = sys.argv[1]
adir = ART + "/" + CODE[code]
PATH = re.compile(r'(?<![\w:$.])(semio_\w+|crate)((?:::[A-Za-z_]\w*)+)(?!\s*!|\w)')
out = set()
for lib, c in sorted(S.CRATES.items()):
    if not c["dir"].startswith(adir): continue
    files, parent = S.tree(c["root"])
    for f, src in files.items():
        al = S.aliases_of(files)
        body = re.sub(r'(?:^|(?<=[;{}\s]))(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]+;', lambda m: " " * len(m.group(0)), src)
        for m in PATH.finditer(body):
            root, rest = m.group(1), m.group(2).split("::")[1:]
            if root == "crate": tlib = lib
            elif root in S.CRATES: tlib = root
            else: continue
            tc = S.CRATES[tlib]
            tfiles, tparent = S.tree(tc["root"]) if tlib != lib else (files, parent)
            cur = tc["root"]; ok = True
            for k, seg in enumerate(rest):
                kids = S.child_of(cur, tfiles, tparent)
                last = k == len(rest) - 1
                pub_only = tlib != lib or cur != tc["root"] and False
                vis, opaque = S.vis_scope(cur, 0, tfiles, tparent, tlib, 0, tlib != lib)
                if seg in kids and not last: cur = kids[seg]; continue
                if seg in vis or seg in kids or opaque: break
                allv, _ = S.vis_scope(cur, 0, tfiles, tparent, tlib, 0, False)
                if seg in allv and tlib != lib: why = "not pub"
                elif seg in allv: break
                elif seg in S.crate_names_all(): why = "does not exist there"
                else: break
                out.add((rel(f), body.count(chr(10), 0, m.start()) + 1, f"`{root}{m.group(2)}` — `{seg}` {why} in {rel(cur).split('/')[-2]} ({c['name']})"))
                break
for f, l, msg in sorted(out): print(f"{f}:{l}: {msg}")
