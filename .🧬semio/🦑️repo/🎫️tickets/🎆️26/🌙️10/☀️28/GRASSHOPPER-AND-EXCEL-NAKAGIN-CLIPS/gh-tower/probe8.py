import importlib, sys
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
n = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
sl = n.slots()
def spec(on):
    return {"objects":[{"id":"all","component":"Panel","settings":{"lines":[s.type for s in sl]}},
     {"id":"pb","component":"Panel","settings":{"lines":[str(s.type=="B") for s in sl]}},
     {"id":"pd","component":"Panel","settings":{"lines":[str(s.type=="D") for s in sl]}},
     {"id":"db","component":"Dispatch","settings":{}},{"id":"dd","component":"Dispatch","settings":{}},
     {"id":"cb","component":"List Length","settings":{}},{"id":"cd","component":"List Length","settings":{}},
     {"id":"sw","component":"Boolean Toggle","settings":{"value":on}},
     {"id":"mul","component":"Multiplication","settings":{}},{"id":"add","component":"Addition","settings":{}},{"id":"sub","component":"Subtraction","settings":{}},
     {"id":"rb","component":"Panel","settings":{"lines":[""]}},{"id":"rd","component":"Panel","settings":{"lines":[""]}}],
     "wires":["all.Stream>db.List","pb.Stream>db.Pattern","all.Stream>dd.List","pd.Stream>dd.Pattern","db.List A>cb.List","dd.List A>cd.List","cb.Length>mul.A","sw.Boolean>mul.B","cd.Length>add.A","mul.Result>add.B","cb.Length>sub.A","mul.Result>sub.B","sub.Result>rb.Stream","add.Result>rd.Stream"]}
print([p.name for p in m.catalogue()["Dispatch"].outputs],[p.name for p in m.catalogue()["List Length"].outputs])
for on in (False, True):
    d = m.define(spec(on), "swap"); s = m.evaluate(d)
    print(on, m.panel_rows(s.tree("rb")), m.panel_rows(s.tree("rd")), s.state("db"), s.messages["db"])
d=m.canvas("tower"); s=m.evaluate(d); t=s.tree("cullp"); print(t.paths()[-1], t.branch(t.paths()[-1]), [p for p in t.paths() if t.branch(p)])
