import importlib.util,json
s=importlib.util.spec_from_file_location("l","animals-appliances-components-b-lib.py");l=importlib.util.module_from_spec(s);s.loader.exec_module(l)
d=l.doc("pig")
n=d.solid("50","snout")
tr=d.field(n,"transform")
print(d.obj(tr))
d.set(d.field(tr,"translation"),0,0)  if False else None
d.setv(d.field(tr,"translation"),[0.3175,0.0,0.0])
sh=l.getshape(d,"50","elevation","snout")
pts=[[round(p[0]+0.0045,5),p[1]] for p in sh["points"]]
d.set(d.shape("50","elevation","snout"),"points",pts)
d.commit()
