import importlib, sys
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
n = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
d = m.canvas("tower"); s = m.evaluate(d)
cat = m.catalogue()
for oid, o in d.objects.items():
    sp = cat[o.component]
    print(oid, o.component, sp.kind, [p.nickname for p in sp.inputs], [p.nickname for p in sp.outputs])
print(sorted(cat)[:200])
import time
t=time.time(); s2=m.evaluate(d,{"n":5}); print(time.time()-t, s2.items("st","Count"), s2.items("stl") if False else "", s2.items("st","Length"))
print(m.panel_rows(s2.tree("stc"))); print(m.panel_rows(s2.tree("stl")))
print(m.panel_rows(s.tree("gp"))); print(m.panel_rows(s.tree("tot")), m.panel_rows(s.tree("tn")), m.panel_rows(s.tree("hc")))
ts = n.towers()
for t in ts: print(t)
sl = n.slots(); print(len(sl), sl[0])
print(m.panel_rows(s.tree("p0"))[:3], m.panel_rows(s.tree("sx")), m.panel_rows(s.tree("hp")))
print(m.panel_rows(s.tree("capital"))[:2], m.panel_rows(s.tree("cw")), m.panel_rows(s.tree("czz")))
