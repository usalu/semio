"""🔎 Explicit `use` items (and `crate_name::module::Item` expression paths) that name an item the target module does not export (E0432/E0603/E0425-style), resolved through #[path] mounts and pub re-exports."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import scan_resolve as S
from rs import *

S.load_crates()
code = sys.argv[1]
adir = ART + "/" + CODE[code]

def items_of_use(body):
    body = re.sub(r'\s+as\s+', '@@', re.sub(r'\s+', ' ', body)).replace(" ", "")
    out = []
    def expand(prefix, b):
        for p in split_top(b):
            if p.startswith("{"): expand(prefix, p[1:-1]); continue
            if "{" in p:
                i = p.index("{"); expand(prefix + p[:i], p[i+1:p.rindex("}")]); continue
            full = (prefix + p).split("@@")[0]
            if full.endswith("::*") or full.endswith("::self") or full == "*": continue
            out.append(full)
    expand("", body)
    return out

def resolve_module(f, parent, files, lib, segs):
    cur, curlib, first = f, lib, segs[0]
    al = S.aliases_of(files)
    i = 0
    if first == "crate": cur = S.CRATES[lib]["root"]; i = 1
    elif first == "self": i = 1
    elif first == "super":
        while i < len(segs) and segs[i] == "super":
            cur = parent.get(cur)
            if cur is None: return None
            i += 1
    elif first in S.CRATES or (first in al and al[first] in S.CRATES):
        curlib = first if first in S.CRATES else al[first]
        cur = S.CRATES[curlib]["root"]; i = 1
        files2, parent2 = S.tree(cur)
        return walk(cur, curlib, files2, parent2, segs, i, True)
    else:
        kids = S.child_of(f, files, parent)
        if first in kids: return walk(kids[first], lib, files, parent, segs, 1, False)
        return None
    return walk(cur, curlib, files, parent, segs, i, False)

def walk(cur, lib, files, parent, segs, i, foreign):
    while i < len(segs) - 1:
        kids = S.child_of(cur, files, parent)
        if segs[i] in kids: cur = kids[segs[i]]; i += 1
        else: return None
    return cur, lib, files, parent, segs[-1], foreign

for lib, c in sorted(S.CRATES.items()):
    if not c["dir"].startswith(adir): continue
    files, parent = S.tree(c["root"])
    ancestors = lambda f: [x for x in iter(lambda g=[f]: (g.__setitem__(0, parent.get(g[0])) or g[0]), None)]
    for f, src in files.items():
        for m in S.USEP.finditer(src):
            for item in items_of_use(m.group(2)):
                segs = item.split("::")
                if len(segs) < 2: continue
                r = resolve_module(f, parent, files, lib, segs)
                if not r: continue
                cur, tlib, tfiles, tparent, name, foreign = r
                if len(segs) == 2 and segs[0] in ("crate", "self", "super") and False: continue
                same_crate = (tlib == lib)
                anc = set()
                g = f
                while g: anc.add(g); g = parent.get(g)
                pub_only = foreign or (cur not in anc)
                vis, opaque = S.vis_scope(cur, 0, tfiles, tparent, tlib, 0, pub_only)
                if opaque or name in vis: continue
                if name in S.child_of(cur, tfiles, tparent): continue
                allv, _ = S.vis_scope(cur, 0, tfiles, tparent, tlib, 0, False)
                if name in allv: why = "not pub / not exported"
                elif name in S.crate_names_all(): why = "does not exist there (defined elsewhere)"
                else: continue
                print(f"{rel(f)}:{src.count(chr(10), 0, m.start()) + 1}: `use {item}` — `{name}` {why} in {rel(cur).split('/')[-2]} ({c['name']})")
