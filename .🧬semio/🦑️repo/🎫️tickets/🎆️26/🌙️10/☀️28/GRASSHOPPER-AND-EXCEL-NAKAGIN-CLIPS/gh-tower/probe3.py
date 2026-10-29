import importlib, sys
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
d = m.canvas("tower"); s = m.evaluate(d, {"n": 2})
for oid in ("p0","p0s","rot","fl","mc","mv","cull","yrot","slots","ori","uzs","stg","zero"):
    o=d.objects[oid]; sp=m.catalogue()[o.component]
    for out in sp.outputs[:1]:
        t = s.tree(oid, out.name); ps=t.paths()
        print(oid, out.name, len(ps), ps[:3], len(t.items()), m.describe(t.items()[0])[:80] if t.items() else None)
print([ (v, d.views[v]["objects"]) for v in d.views])
print(m.panel_rows(s.tree("st","Count")), s.items("st","Length"))
print(s.items("stc"), s.items("stl"))
print([(w.src,w.out,w.dst,w.inp) for w in d.wires if w.dst in ("ori","st","slots","mv")])
