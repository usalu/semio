import sys, os, re
sys.path.insert(0, ".")
from emit import *
lay = Layout("plan3"); w = lay.w
em = Emitter(lay)
bad = 0; n = 0
for o in lay.outs:
    for u in o.units:
        it = u.it
        if it.kind != "impl": continue
        n += 1
        # inherent impl: type atom must be in same crate
        if not it.impl_trait:
            if u.atom.item.kind in ("struct", "enum", "union", "type") and lay.crate_of[id(u.atom)] != o.crate:
                bad += 1; print("INHERENT impl away from type", u.atom.name)
            continue
        # trait impl: trait local? find atom for first ident in impl_trait
        ids = re.findall(r'[A-Za-z_]\w*', it.impl_trait)
        tr = None
        for i in ids:
            for a in w.defs.get(i, []):
                if a.kind == "trait" and id(a) in lay.crate_of: tr = a; break
            if tr: break
        tcrate = lay.crate_of.get(id(tr)) if tr else None
        scrate = lay.crate_of.get(id(u.atom))
        if tcrate is not None and tcrate != o.crate and scrate != o.crate:
            bad += 1; print("ORPHAN?", u.atom.name, it.impl_trait, "impl crate", lay.names[o.crate], "trait crate", lay.names[tcrate], "type crate", lay.names[scrate])
        if tcrate is None and scrate != o.crate:
            bad += 1; print("FOREIGN TRAIT impl away from type", u.atom.name, it.impl_trait, lay.names[o.crate], lay.names[scrate])
print("impls", n, "suspects", bad)
