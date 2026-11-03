import collections,json,sys
sys.path.insert(0,".")
from mk import *
import a,b
items=[]
for name,(spec,note) in a.S.items():
    for rel,edits in spec.items():items.append((rel,edits,note))
items+=b.B
try:
    import c
    items+=c.C
    import d
    items+=d.D
    import f
    items+=f.F
    import g
    items+=g.G
    import h
    items+=h.H
    import i
    items+=i.I
except ImportError:pass
g=collections.OrderedDict()
for rel,edits,note in items:g.setdefault(crate_of(rel),[]).append((rel,edits,note))
rows=[]
for cr,lst in g.items():
    spec={};notes=[]
    for rel,edits,note in lst:
        spec.setdefault(rel,[]).extend(edits);notes.append(note)
    build(cr,spec)
    rows.append({"crate":cr,"files":len(spec),"sites":sum(len(e) for e in spec.values()),"notes":notes,"check":open(f"{OUT}/{cr}.check").read()})
json.dump(rows,open(f"{OUT}/index.json","w"),ensure_ascii=False,indent=1)
print(len(rows),"patches",sum(r["sites"] for r in rows),"edits")
