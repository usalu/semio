import sys, re, collections
sys.path.insert(0, ".")
from prod import *
files = walk(LIBROOT, ["crate"])
def topmod(f):
    mp = f.modpath
    return mp[1] if len(mp) > 1 else "(root-glue)"
MODS = set(topmod(f) for f in files.values())
edges = collections.defaultdict(lambda: collections.defaultdict(set))
mode = sys.argv[1] if len(sys.argv) > 1 else "prod"
for p, f in files.items():
    t = topmod(f)
    code = prod_code(f, files) if mode == "prod" else f.code
    seen = set()
    for m in re.finditer(r'\bcrate::', code):
        paths,_ = parse_use_tree(code, m.start())
        if paths and code[m.start()-1] not in ' \t\n(<,[&=!{|' : pass
        # in-code path: just grab a plain path
        mm = re.compile(r'crate((?:::[A-Za-z_]\w*)+)').match(code, m.start())
        if code.startswith('crate::{', m.start()):
            for pp in paths:
                if len(pp) > 1 and pp[1] in MODS and pp[1] != t:
                    edges[t][pp[1]].add("::".join(pp[2:]) or "(mod)")
                elif len(pp) > 1 and pp[1] not in MODS:
                    edges[t]["FLAT"].add(pp[1])
        elif mm:
            segs = mm.group(1).split("::")[1:]
            if segs[0] in MODS and segs[0] != t:
                edges[t][segs[0]].add("::".join(segs[1:3]) or "(mod)")
            elif segs[0] not in MODS:
                edges[t]["FLAT"].add(segs[0])
for t in sorted(edges):
    print("==", t)
    for d, items in sorted(edges[t].items()):
        print("   ->", d, sorted(items)[:60])
