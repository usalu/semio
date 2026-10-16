import re, sys, collections, pickle
sys.path.insert(0, ".")
from items import *

INTEG = [MCP + "/💡️inference/🧪️tests/⏱️binding-cancellation-law/🦀️.rs", MCP + "/🏠️workspace/🧪️tests/⏱️compile-cancellation-law/🦀️.rs"]
TOK = re.compile(r'(?<![\w$])([A-Za-z_]\w*)')
DEFKINDS = {'fn','struct','enum','trait','type','const','static','macro_rules','union','mod','other'}

def topmod(f):
    mp = f.modpath
    return mp[1] if len(mp) > 1 else "(glue)"

class Atom:
    def __init__(self, item):
        self.item = item; self.impls = []; self.name = item.name; self.kind = item.kind
        self.file = item.file; self.prod = True
    @property
    def text(self): return self.item.text + "".join(i.text for i in self.impls)
    def key(self): return (self.file.path, self.name, self.item.line0)

def build(mode="prod"):
    files = walk(LIBROOT, ["crate"])
    for p in INTEG:
        walk(p, ["integration", os.path.basename(os.path.dirname(p))], None, "#[cfg(test)]", None, files)
    allitems = []
    for p, f in files.items():
        for it in split_items(f):
            allitems.append(it)
    defs = collections.defaultdict(list)
    atoms = []
    for it in allitems:
        if it.kind == 'other' and not it.name:
            it.name = "_anon%d" % it.line0
        if it.kind in DEFKINDS and it.name:
            a = Atom(it); atoms.append(a); defs[it.name].append(a)
    # attach impls
    loose = []
    for it in allitems:
        if it.kind == 'impl':
            idents = [m.group(1) for m in TOK.finditer((it.impl_self or "") )]
            cand = [i for i in idents if i in defs and defs[i][0].kind in ('struct','enum','union','type','trait')]
            if not cand and it.impl_trait:
                cand = [m.group(1) for m in TOK.finditer(it.impl_trait) if m.group(1) in defs]
            if cand:
                tgt = defs[cand[0]]
                # prefer same-file def
                same = [a for a in tgt if a.file is it.file]
                (same or tgt)[0].impls.append(it)
            else:
                loose.append(it)
    return files, allitems, atoms, defs, loose

if __name__ == "__main__":
    files, allitems, atoms, defs, loose = build()
    print(len(allitems), len(atoms), len(loose))
    dup = {k: v for k, v in defs.items() if len(v) > 1}
    print("dup names", len(dup))
    for k, v in sorted(dup.items())[:80]:
        print(k, [(topmod(a.file), a.kind, a.item.line0) for a in v])
    print("loose impls:")
    for it in loose[:60]:
        print(topmod(it.file), it.line0, it.impl_trait, "for", it.impl_self)
