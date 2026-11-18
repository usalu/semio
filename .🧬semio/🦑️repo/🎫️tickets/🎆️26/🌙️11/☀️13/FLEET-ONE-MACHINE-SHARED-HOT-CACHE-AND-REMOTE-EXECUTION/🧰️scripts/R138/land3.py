import sys,os,shutil,json
ROOT="/Users/ueli/Documents/semio/"
L=ROOT+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
D="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R138/"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R138/"
R=L+"🏡️resident/"
HOST=R+"📨️protocol/📍️endpoint/🏠️host/"
HOSTSVC=R+"🏛️service/🌐️host/"
files={R+"🧪️tests/🪪️owner-idle/🟦️.ts":D+"owner-test.ts"}
SCH=R+"🧬️schema/🔣️.json"
edits=[
 (R+"🏛️service/🌐️host/🟨️.mjs",'import {failureRefusalV1} from "../🧯️failure/🟨️.mjs";','import {failureRefusalV1,serviceRouteV1} from "../🧯️failure/🟨️.mjs";'),
 (R+"🏛️service/🌐️host/🟨️.mjs",'requireCapability=true,onExit=()=>{}}=options;','requireCapability=true,onExit=()=>{},environment=process.env}=options;'),
 (R+"🏛️service/🌐️host/🟨️.mjs",'instance=randomBytes(8).toString("hex"),startedAt=now();','instance=randomBytes(8).toString("hex"),startedAt=now();\n const route=serviceRouteV1(environment),idleLimit=route==="direct"?Math.min(policy.implicitIdleShutdownMilliseconds??policy.idleShutdownMilliseconds,policy.idleShutdownMilliseconds):policy.idleShutdownMilliseconds,owner={route,pid:process.ppid,idleSeconds:Math.ceil(idleLimit/1000)};'),
 (R+"🏛️service/🌐️host/🟨️.mjs",'now()-lastActivity>policy.idleShutdownMilliseconds','now()-lastActivity>idleLimit'),
 (R+"🏛️service/🌐️host/🟨️.mjs",'Math.min(policy.idleShutdownMilliseconds/4,30000)','Math.min(idleLimit/4,30000)'),
 (R+"🏛️service/🌐️host/🟨️.mjs",'implementation,...(stopping?{stopping:true}:{})}','implementation,owner,...(stopping?{stopping:true}:{})}'),
 (R+"🏛️service/🌐️host/🟨️.mjs",'endpoint:runtime.endpoint,log:runtime.log}}:{},','endpoint:runtime.endpoint,log:runtime.log}}:{},owner,'),
 (SCH,'"idleShutdownMilliseconds": 900000,','"idleShutdownMilliseconds": 900000,\n    "implicitIdleShutdownMilliseconds": 120000,'),
 (SCH,'''    "stopping": {
     "type": "boolean"
    }
   }
  },
  "Request": {''','''    "stopping": {
     "type": "boolean"
    },
    "owner": {
     "$ref": "#/definitions/HostOwner"
    }
   }
  },
  "Request": {'''),
 (SCH,'''    "identity": {
     "$ref": "#/definitions/HostIdentity"
    },''','''    "identity": {
     "$ref": "#/definitions/HostIdentity"
    },
    "owner": {
     "$ref": "#/definitions/HostOwner"
    },'''),
 (SCH,'''  "HostIdentity": {''','''  "HostOwner": {
   "description": "HostOwnerV1: why a service lives. `launchd`: a launchd job keeps it; `foreground`: run by `repo service host run`; `direct`: started implicitly by a repo command (pid is that command) and ended after the short idle time.",
   "type": "object",
   "additionalProperties": false,
   "required": ["route", "pid", "idleSeconds"],
   "properties": {
    "route": {"enum": ["launchd", "direct", "foreground"]},
    "pid": {"type": "integer", "minimum": 0},
    "idleSeconds": {"type": "integer", "minimum": 0}
   }
  },
  "HostIdentity": {'''),
 (SCH,'      "host-down": "No repo graph service is running on this host.",','      "host-down": "No repo graph service is running on this host.",\n      "host-owner-direct": "Owner: started implicitly by a repo command (pid {pid}); ends after {seconds} s without a client.",\n      "host-owner-launchd": "Owner: its launchd job keeps it; ends after {seconds} s idle.",\n      "host-owner-foreground": "Owner: run in the foreground (pid {pid}); ends after {seconds} s idle.",'),
 (SCH,'      "host-down": "Auf diesem Rechner läuft kein Repo-Graphdienst.",','      "host-down": "Auf diesem Rechner läuft kein Repo-Graphdienst.",\n      "host-owner-direct": "Besitzer: von einem repo-Befehl implizit gestartet (pid {pid}); endet nach {seconds} s ohne Client.",\n      "host-owner-launchd": "Besitzer: sein launchd-Job hält ihn; endet nach {seconds} s Leerlauf.",\n      "host-owner-foreground": "Besitzer: im Vordergrund ausgeführt (pid {pid}); endet nach {seconds} s Leerlauf.",'),
 (R+"⌨️command/🌐️host/🟨️.mjs",'   if(found.memory)print(say("host-memory"','   if(found.owner)print(say("host-owner-"+found.owner.route,{pid:found.owner.pid,seconds:found.owner.idleSeconds}));\n   if(found.memory)print(say("host-memory"'),
 (R+"📨️protocol/🎚️policy/🟨️.d.mts",'readonly idleShutdownMilliseconds:number;','readonly idleShutdownMilliseconds:number;readonly implicitIdleShutdownMilliseconds:number;'),
 (R+"📨️protocol/🎚️policy/🟨️.d.mts",'"idleShutdownMilliseconds"|','"idleShutdownMilliseconds"|"implicitIdleShutdownMilliseconds"|'),
 (R+"🏛️service/🌐️host/🟨️.d.mts",'export interface HostServiceOptionsV1{runtime:HostEndpointsV1;implementation:string;','export interface HostServiceOptionsV1{runtime:HostEndpointsV1;implementation:string;environment?:Record<string,string|undefined>;'),
]
BK=S+"backup3/"
def apply():
 os.makedirs(BK,exist_ok=True)
 for dst,src in files.items():
  if os.path.exists(dst):shutil.copy2(dst,BK+dst[len(ROOT):].replace("/","|"))
  else:open(BK+dst[len(ROOT):].replace("/","|")+".absent","w").close()
 for f,_,_ in edits:
  k=BK+f[len(ROOT):].replace("/","|")
  if not os.path.exists(k):shutil.copy2(f,k)
 for f,a,b in edits:
  s=open(f).read();assert s.count(a)==1,(f,a[:60],s.count(a));open(f,"w").write(s.replace(a,b))
 for dst,src in files.items():
  os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copy2(src,dst)
 json.load(open(R+"🧬️schema/🔣️.json"))
def restore():
 for k in os.listdir(BK):
  if k.endswith(".absent"):
   p=ROOT+k[:-7].replace("|","/")
   if os.path.exists(p):os.remove(p)
  else:shutil.copy2(BK+k,ROOT+k.replace("|","/"))
if __name__=="__main__":
 {"apply":apply,"restore":restore}[sys.argv[1]]()
