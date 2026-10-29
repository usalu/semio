import importlib, sys
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
d = m.canvas("tower")
s = m.evaluate(d)
for oid in d.objects:
    try:
        t = s.tree(oid)
        print(oid, s.state(oid), m.describe(t) if hasattr(m, "describe") else t.count)
    except Exception as e:
        print(oid, "ERR", repr(e)[:100])
print(s.items("st", "Count"), s.items("st","Length"))
print(d.objects["side"].settings)
print(m.panel_rows(s.tree("gp")))
