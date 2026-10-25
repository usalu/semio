import os,sys,tempfile
def patch(path,subs):
    src=open(path,encoding="utf-8").read()
    for old,new,count in subs:
        n=src.count(old)
        if n!=count: sys.exit(f"{path}: anchor count {n}!={count}: {old[:90]}")
        src=src.replace(old,new)
    d=os.path.dirname(path);fd,tmp=tempfile.mkstemp(dir=d,prefix=".patch-");os.write(fd,src.encode());os.close(fd);os.rename(tmp,path);print("patched",path[-40:])
G=sys.argv[1]
patch(G+"/🔌️load/🟨️.mjs",[
('  waiting:"graph: waiting for the graph build of a peer to finish",\n','  waiting:"graph: waiting for the graph build of a peer ({seconds} s in the queue, generation {generation} was the newest when this command queued)",\n  covered:"graph: generation {generation} was sealed by a peer after this command began and already covers it, nothing is checked again",\n',1),
('  waiting:"Graph: wartet auf den Graph-Aufbau eines anderen Prozesses",\n','  waiting:"Graph: wartet auf den Graph-Aufbau eines anderen Prozesses ({seconds} s in der Warteschlange, Generation {generation} war beim Einreihen die neueste)",\n  covered:"Graph: Generation {generation} wurde von einem anderen Prozess nach dem Beginn dieses Befehls versiegelt und deckt ihn bereits ab, nichts wird erneut geprüft",\n',1),
(' const identity=await host.identity(),roots=',' const requestedAt=Date.now(),identity=await host.identity(),roots=',1),
('  const base={identity,implementation,policy:builder?"fresh":policy,membership:host.membership,compute,signal:host.signal,progress,builder:builder?body=>body():needsBuild};','  const base={identity,implementation,policy:builder?"fresh":policy,membership:host.membership,compute,signal:host.signal,progress,requestedAt,builder:builder?body=>body():needsBuild};',1),
('  if(result.needsBuild)return{needsBuild:true};','  if(result.needsBuild)return{needsBuild:true,generation:(await engine.read(host.signal))?.pointer.generation??-1};',1),
('  if(result.served==="current")host.say(result.current.manifest.implementation===implementation?"current":"stale",{generation});','  if(result.served==="current")host.say(result.current.manifest.implementation===implementation?"current":"stale",{generation});\n  else if(result.served==="covered")host.say("covered",{generation});',1),
(' const built=await host.withBuilder((store,control)=>attempt(true,store,control));\n',' const queued=Date.now();let held=false;\n const notice=setInterval(()=>{if(!held)host.say("waiting",{seconds:Math.round((Date.now()-queued)/1000),generation:first.generation});},GENERATION_POLICY_V1.waitNoticeMilliseconds);notice.unref?.();\n let built;try{built=await host.withBuilder((store,control)=>{held=true;return attempt(true,store,control);});}finally{clearInterval(notice);}\n',1),
])
import json
p=G+"/🧬️schema/🔣️.json"
s=open(p,encoding="utf-8").read()
subs=[
('     "waiting": "graph: waiting for the graph build of a peer to finish",\n','     "waiting": "graph: waiting for the graph build of a peer ({seconds} s in the queue, generation {generation} was the newest when this command queued)",\n     "covered": "graph: generation {generation} was sealed by a peer after this command began and already covers it, nothing is checked again",\n'),
('     "waiting": "Graph: wartet auf den Graph-Aufbau eines anderen Prozesses",\n','     "waiting": "Graph: wartet auf den Graph-Aufbau eines anderen Prozesses ({seconds} s in der Warteschlange, Generation {generation} war beim Einreihen die neueste)",\n     "covered": "Graph: Generation {generation} wurde von einem anderen Prozess nach dem Beginn dieses Befehls versiegelt und deckt ihn bereits ab, nichts wird erneut geprüft",\n'),
('    "reasonRows": 16\n','    "reasonRows": 16,\n    "stampVerification": false,\n    "waitNoticeMilliseconds": 20000\n'),
('    "reason": {\n     "type": "string",\n     "minLength": 1,\n     "maxLength": 256\n    }\n   }\n  },\n  "OwnerRow"','    "reason": {\n     "type": "string",\n     "minLength": 1,\n     "maxLength": 256\n    },\n    "verified": {\n     "type": "object",\n     "additionalProperties": false,\n     "required": [\n      "atMs",\n      "count",\n      "owners"\n     ],\n     "properties": {\n      "atMs": {\n       "type": "integer",\n       "minimum": 0,\n       "maximum": 9007199254740991\n      },\n      "count": {\n       "type": "integer",\n       "minimum": 0,\n       "maximum": 65536\n      },\n      "owners": {\n       "type": "string"\n      }\n     }\n    }\n   }\n  },\n  "OwnerRow"'),
]
for o,n in subs:
    if s.count(o)!=1: sys.exit("schema anchor "+o[:60]+" count "+str(s.count(o)))
    s=s.replace(o,n)
json.loads(s)
d=os.path.dirname(p);fd,tmp=tempfile.mkstemp(dir=d,prefix=".patch-");os.write(fd,s.encode());os.close(fd);os.rename(tmp,p);print("schema ok")
