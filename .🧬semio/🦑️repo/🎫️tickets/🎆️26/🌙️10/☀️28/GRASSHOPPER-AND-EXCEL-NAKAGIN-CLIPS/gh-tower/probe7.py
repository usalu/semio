import importlib, sys
sys.path.insert(0, "C:/git/semio")
m = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧱️model.🐍️")
c = m.catalogue()
print([(p.name) for p in c["List Item"].outputs], [(p.name) for p in c["Addition"].inputs], [(p.name) for p in c["Addition"].outputs], [p.name for p in c["Series"].outputs], [p.name for p in c["Cull Index"].inputs])
kinds = list("BCDABCDA")
d = m.define({"objects":[
 {"id":"pat","component":"Panel","at":[0,0],"settings":{"lines": kinds}},
 {"id":"cnt","component":"Panel","at":[0,0],"settings":{"lines":[str(len(kinds))]}},
 {"id":"ser","component":"Series","at":[0,0],"settings":{}},
 {"id":"li","component":"List Item","at":[0,0],"settings":{}},
 {"id":"out","component":"Panel","at":[0,0],"settings":{"lines":[""]}}],
 "wires":["pat.Stream>li.List","cnt.Stream>ser.Count","ser.Series>li.Index","li.Item>out.Stream"]}, "types")
s = m.evaluate(d); print(m.panel_rows(s.tree("out")), s.state("li"), s.messages["li"])
d2 = m.define({"objects":[{"id":"cb","component":"Panel","at":[0,0],"settings":{"lines":["35"]}},{"id":"cd","component":"Panel","at":[0,0],"settings":{"lines":["35"]}},
 {"id":"add","component":"Addition","at":[0,0],"settings":{}},{"id":"res","component":"Panel","at":[0,0],"settings":{"lines":[""]}}],
 "wires":["cb.Stream>add.A","cd.Stream>add.B","add.Result>res.Stream"]}, "swap")
s = m.evaluate(d2); print(m.panel_rows(s.tree("res")))
d=m.canvas("tower")
print(m.panel_rows(m.evaluate(d, {"cullp": {"tree": {"{0;%d}"%i: [] for i in range(11)}}}).tree("stc")), m.evaluate(d, {"cullp": {"tree": {"{0;%d}"%i: [] for i in range(11)}}}).items("st","Length"))
