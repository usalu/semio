import importlib, sys
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
c = m.catalogue()
print(sorted(c))
for n in ("List Item","Series","Dispatch","List Length","Replace Items","Equality","Cull Pattern","Number Param","Text Param","Tree Statistics","Value List","Panel"):
    if n in c: print(n, c[n].kind, [(p.name,p.nickname,p.type,p.access,p.default) for p in c[n].inputs], [p.nickname for p in c[n].outputs])
