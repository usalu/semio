"""🔎 Paths crossing into another module (`use super::sibling::Name`, `crate::module::Name`) whose target is declared without `pub` in that module file."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import scan_resolve as S
from rs import *

S.load_crates()
code = sys.argv[1]
adir = ART + "/" + CODE[code]
PATH = re.compile(r'(?<![\w:])(super(?:::super)*|crate|self)((?:::\w+)+)')
def ancestors(f, parent):
    out = []
    while f: out.append(f); f = parent.get(f)
    return out
for lib, c in sorted(S.CRATES.items()):
    if not c["dir"].startswith(adir): continue
    files, parent = S.tree(c["root"])
    root = c["root"]
    for f, src in files.items():
        raws = []
        for m in S.USEP.finditer(src):
            body = re.sub(r'\s+', '', m.group(2))
            if body.startswith(("super", "crate", "self")):
                n, g = S.use_names(body)
                for p in re.findall(r'(?:super(?:::super)*|crate|self)(?:::\w+)+', body.replace("{", "").replace("}", "")): raws.append((p, m.start()))
                raws.extend(((x, m.start()) for x in re.findall(r'(super(?:::super)*|crate|self)::(\w+(?:::\w+)*)::\{([^}]*)\}', body) if False))
                for pre, mid, items in re.findall(r'(super(?:::super)*|crate|self)::(\w+(?:::\w+)*)::\{([^{}]*)\}', body):
                    for it in items.split(","):
                        it = it.split("@@")[0].split("as")[0] if False else re.sub(r'as\w+$', '', it)
                        if re.fullmatch(r'\w+', it): raws.append((f"{pre}::{mid}::{it}", m.start()))
        for m in PATH.finditer(re.sub(r'(?:^|(?<=[;{}\s]))(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]+;', lambda mm: " " * len(mm.group(0)), src)):
            raws.append((m.group(0), m.start()))
        for p, pos in raws:
            segs = p.split("::")
            first = segs[0]; rest = segs[1:]
            cur = f
            ok = True
            if first == "crate": cur = root
            elif first == "super":
                n_sup = 1 + sum(1 for x in rest if x == "super")
                rest = [x for x in rest if x != "super"]
                for _ in range(n_sup):
                    cur = parent.get(cur)
                    if cur is None: ok = False; break
            elif first == "self": pass
            if not ok or len(rest) < 2: continue
            for seg in rest[:-1]:
                kids = S.child_of(cur, files, parent)
                if seg in kids: cur = kids[seg]
                else: ok = False; break
            if not ok: continue
            name = rest[-1]
            if cur in ancestors(f, parent): continue
            pub, _ = S.vis_scope(cur, 0, files, parent, lib, 0, True)
            allv, _ = S.vis_scope(cur, 0, files, parent, lib, 0, False)
            if name in pub or name not in allv: continue
            kids = S.child_of(cur, files, parent)
            if name in kids: continue
            print(f"{rel(f)}:{src.count(chr(10), 0, pos) + 1}: `{p}` — `{name}` is not pub in {rel(cur).split('/')[-2] if cur else ''} ({c['name']})")
