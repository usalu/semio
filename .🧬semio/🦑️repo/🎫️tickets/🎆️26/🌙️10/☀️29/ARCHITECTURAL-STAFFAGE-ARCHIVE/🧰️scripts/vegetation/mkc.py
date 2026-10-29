import json,sys
# usage: mkc.py <id> : design/<id>-cs.json (hand-authored texts and physical data) + design/<id>-spec.json (pose lean values) -> design/<id>-c.json with @placeholders for loads, sway/gust keys and tracks
i=sys.argv[1]; cs=json.load(open(f"design/{i}-cs.json")); sp=json.load(open(f"design/{i}-spec.json")); R=json.load(open("design/refs.json"))
J=lambda v:json.dumps(v,separators=(",",":"),ensure_ascii=False)
bones=lambda d:{k:{"angleDeg":v} for k,v in d.items()}
em=lambda f:{e:round(f*cs["emitScale"].get(e,1),3) for e in cs["emitters"]}
def pose(pid,title,desc,b,f,extra=None):
    return {"id":pid,"title":title,"description":desc,"bones":b,"ground":"authored","contacts":[],"thermal":{"operating":{},"emitters":em(f)},"loads":"@LOADS_%s@"%pid}
poses=[pose("calm",cs["poses"]["calm"]["title"],cs["poses"]["calm"]["desc"],cs["calmBones"],0.8),
 pose("windy",cs["poses"]["windy"]["title"],cs["poses"]["windy"]["desc"],bones(sp["windy"]),1.0),
 pose("storm",cs["poses"]["storm"]["title"],cs["poses"]["storm"]["desc"],bones(sp["storm"]),0.5)]
for x in cs.get("extraPoses",[]): poses.append(x)
P=sp["period"]
ph=lambda a:[{"id":n,"fromS":f,"toS":t,"title":{"en":e,"de":d}} for n,f,t,e,d in a]
loadids=cs["gustLoads"]
clips=[
{"id":"wind-sway","title":{"en":"Wind sway","de":"Windschwingung"},"description":cs["clips"]["sway"],"durationS":P,"fps":30,"loop":True,"rootMotion":"none","keys":"@SWAY@","contacts":[],"phases":ph([("sway",0,P,"Sway cycle","Schwingungszyklus")]),"events":[{"t":0,"kind":"motion","title":{"en":"Cycle start at mean lean","de":"Zyklusbeginn in mittlerer Neigung"}}],"tracks":[{"target":"emitter:"+cs["emitters"][0],"ease":"smooth","keys":[{"t":0,"v":1},{"t":P/2,"v":0.9},{"t":P,"v":1}]}]},
{"id":"gust","title":{"en":"Gust","de":"Windböe"},"description":cs["clips"]["gust"],"durationS":8,"fps":30,"loop":False,"rootMotion":"none","keys":"@GUST@","contacts":[],"phases":ph([("rise",0,2.5,"Gust front","Böenfront"),("peak",2.5,3.6,"Peak","Spitze"),("decay",3.6,8,"Decay and recovery","Abklingen und Erholung")]),"events":[{"t":2.5,"kind":"load-on","title":{"en":"Peak gust load","de":"Böenspitze"}},{"t":5,"kind":"load-off","title":{"en":"Load below the mean again","de":"Last wieder unter dem Mittelwert"}}],"tracks":"@GUSTTRACKS@"},
{"id":"diurnal-transpiration","title":{"en":"Day of transpiration","de":"Transpirationstag"},"description":cs["clips"]["diurnal"],"durationS":24,"fps":10,"loop":True,"rootMotion":"none","keys":[{"t":0,"pose":"calm","ease":"smooth"},{"t":12,"pose":"calm","ease":"smooth"},{"t":24,"pose":"calm","ease":"smooth"}],"contacts":[],"phases":ph([("night",0,6,"Night","Nacht"),("morning",6,12,"Morning to noon","Morgen bis Mittag"),("afternoon",12,18,"Afternoon","Nachmittag"),("evening",18,24,"Evening and night","Abend und Nacht")]),"events":[{"t":6,"kind":"thermal","title":{"en":"Sunrise","de":"Sonnenaufgang"}},{"t":12,"kind":"thermal","title":{"en":"Solar noon, peak transpiration","de":"Sonnenhöchststand, maximale Transpiration"}},{"t":18,"kind":"thermal","title":{"en":"Sunset","de":"Sonnenuntergang"}}],"tracks":"@DIURNAL@"}]
clips+=cs.get("extraClips",[])
env={"airTempC":22,"meanRadiantTempC":25,"relativeHumidity":0.55,"windMS":2,"windDirectionDeg":0,"solarWM2":700,"pressurePa":101325}
env.update(cs.get("env",{}))
emitters=[dict(e) for e in cs["emitterDefs"]]
doc={"title":cs["title"],"summary":cs["summary"],"tags":cs["tags"],
"provenance":{"author":"Semio staffage vegetation content worker","method":"handcrafted","created":"2026-10-29","references":[{"id":r,"citation":R[r]} for r in cs["refs"]],"notes":cs["notes"]},
"dimensions":cs["dimensions"],"materials":cs["materials"],"poses":poses,"clips":clips,
"thermal":{"environment":env,"regions":cs["regions"],"emitters":emitters,"assumptions":cs["thermalAssumptions"]},
"structure":{"gravityMS2":9.80665,"dynamics":"quasi-static","supports":["base-support"],"assumptions":cs["structureAssumptions"],"clinical":False},
"presentation":{"defaultPose":"calm","defaultClip":"wind-sway","viewpoints":["elevation"]},
"exports":["svg","pdf","dxf","dwg","png-sequence","mp4","webm","component"],"completion":"authored"}
s=J(doc)
import re
s=re.sub(r'"(@[A-Za-z_]+@)"',r'[\1]',s)
open(f"design/{i}-c.json","w").write(s)
