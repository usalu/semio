import importlib, sys
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
d = m.canvas("tower")
print([n for n in dir(m) if "override" in n])
for n in (11, 5, 2):
    s = m.evaluate(d, {"n": n}); print(n, s.items("stc"), s.items("stl"), s.items("n"), s.items("ser"))
print([(w.src,w.out,w.dst,w.inp) for w in d.wires if w.dst in ("ser","mv","mc","cull","cullp","uz","gz")])
print(d.objects["cullp"].settings, d.objects["cull"].settings)
