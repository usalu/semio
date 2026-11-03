import json,re
B="🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/"
def tx(en,de): return {"en":en,"de":de}
p=B+"gas-hob/🔣️.json"
s=open(p,encoding="utf-8",newline="").read()
i=s.index('"id": "flicker"'); j=s.index('"loop": false',i); assert j-i<2500
s=s[:j]+'"loop": true'+s[j+len('"loop": false'):]
open(p,"w",encoding="utf-8",newline="").write(s)
p=B+"gas-boiler/🔣️.json"
s=open(p,encoding="utf-8",newline="").read()
def tr(target,v): return {"target":target,"ease":"step","keys":[{"t":0,"v":v},{"t":4,"v":v}]}
clip={"id":"steady-burn","title":tx("Steady burn","Dauerbrand"),
 "description":tx("The boiler heats at 14 kW and the flame in the sight window ripples every 4 seconds; the clip returns to its first pose and loops.","Der Kessel heizt mit 14 kW und die Flamme im Schauglas wellt sich alle 4 Sekunden; der Clip kehrt in seine erste Pose zurück und läuft in einer Schleife."),
 "durationS":4,"fps":15,"loop":True,"rootMotion":"none",
 "keys":[{"t":0,"pose":"heating","ease":"smooth"},{"t":1,"pose":"heating","ease":"smooth","bones":{"flame":{"dy":-0.012}}},{"t":2,"pose":"heating","ease":"smooth","bones":{"flame":{"dy":-0.004}}},{"t":3,"pose":"heating","ease":"smooth","bones":{"flame":{"dy":-0.01}}},{"t":4,"pose":"heating","ease":"smooth"}],
 "contacts":[{"anchor":"wall-bracket","fromS":0,"toS":4}],
 "phases":[{"id":"burn","fromS":0,"toS":4,"title":tx("Burning","Brennt")}],
 "events":[{"t":1,"kind":"thermal","title":tx("Flame dip","Flammenabsenkung"),"target":"burner"}],
 "tracks":[tr("operating:burner",0.7),tr("operating:controls",1),tr("emitter:flue-gas",1)]}
t=json.dumps(clip,indent=2,ensure_ascii=False).replace("\n","\n      ")
mark='\n      }\n    ],\n    "thermal": {'
assert s.count(mark)==1
s=s.replace(mark,'\n      },\n      '+t+'\n    ],\n    "thermal": {')
open(p,"w",encoding="utf-8",newline="").write(s); json.loads(s); print("ok")
