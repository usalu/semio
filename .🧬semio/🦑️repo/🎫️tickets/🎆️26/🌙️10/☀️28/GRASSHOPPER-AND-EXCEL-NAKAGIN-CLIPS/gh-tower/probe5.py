import importlib, sys
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
d = m.canvas("tower")
for n in (11, 10, 5, 1):
    o = {"n": n}
    if n < 11: o["cullp"] = {"tree": {"{0;%d}" % s: [] for s in range(n)}}
    s = m.evaluate(d, o); print(n, s.items("stc"), s.items("stl"), s.state("cull"), s.state("st"), s.messages["cull"])
s = m.evaluate(d, {"n": 5})
print(len(s.tree("mv","Geometry").paths()), len(s.tree("cull","List").paths()), len(s.tree("cullp","Int").paths()), s.tree("cullp","Int").to_dict())
print(d.objects["cullp"].settings)
