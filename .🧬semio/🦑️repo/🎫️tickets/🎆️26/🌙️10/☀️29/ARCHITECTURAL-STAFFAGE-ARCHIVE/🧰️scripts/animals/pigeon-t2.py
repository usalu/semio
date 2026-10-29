import sys
exec(open(sys.argv[1]+"/pigeon-emit.py").read().split("P = []; CL = []")[0])
EYE0=a.anchor(a.fk({}),"eye")
for combo in (("neck-1","neck-2"),("neck-1","neck-3"),("neck-2","neck-3")):
  for d in (-0.027,0.0,0.027):
    spec={n:("off",0) for n in NECK[:3]}; 
    for c in combo: spec[c]=("solve",)
    spec["head"]=("abs",-10)
    o,e=a.solve_limb({},NECK,"eye",(EYE0[0]+d,EYE0[1]),spec)
    print(combo,d,round(e,4),{k:round(v) for k,v in o.items()})
