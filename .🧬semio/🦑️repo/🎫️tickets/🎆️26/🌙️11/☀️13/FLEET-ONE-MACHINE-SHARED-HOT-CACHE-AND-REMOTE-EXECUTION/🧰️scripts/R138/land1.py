import sys,os,shutil,json
ROOT="/Users/ueli/Documents/semio/"
L=ROOT+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R138/"
R=L+"🏡️resident/"
HOST=R+"📨️protocol/📍️endpoint/🏠️host/"
files={
 HOST+"🟨️.mjs":S+"m/"+HOST[len(ROOT):]+"🟨️.mjs",
 HOST+"🟨️.d.mts":S+"m/"+HOST[len(ROOT):]+"🟨️.d.mts",
 R+"🧪️tests/🆔️host-identity/🟦️.ts":S+"m/"+R[len(ROOT):]+"🧪️tests/🆔️host-identity/🟦️.ts",
}
edits=[
 (R+"🏛️service/🌐️host/🟨️.mjs",'startedAt,rssMb:Math.round(process.memoryUsage().rss/MiB*10)/10,','startedAt,...runtime.identity?{identity:{...runtime.identity,tag:runtime.tag,directory:runtime.directory,endpoint:runtime.endpoint,log:runtime.log}}:{},rssMb:Math.round(process.memoryUsage().rss/MiB*10)/10,'),
 (R+"🧬️schema/🔣️.json",'''    "implementation": {
     "type": "string",
     "pattern": "^[0-9a-f]{64}$"
    },
    "startedAt": {
     "type": "integer",
     "minimum": 0
    },
    "rssMb": {''','''    "implementation": {
     "type": "string",
     "pattern": "^[0-9a-f]{64}$"
    },
    "startedAt": {
     "type": "integer",
     "minimum": 0
    },
    "identity": {
     "$ref": "#/definitions/HostIdentity"
    },
    "rssMb": {'''),
 (R+"🧬️schema/🔣️.json",'''  "HostWorkspace": {''','''  "HostIdentity": {
   "description": "HostIdentityV1: what one graph service is keyed by (sealed implementation digest, checkout root, machine state root) and where it lives.",
   "type": "object",
   "additionalProperties": false,
   "required": ["implementation", "root", "state", "tag", "directory", "endpoint", "log"],
   "properties": {
    "implementation": {"type": "string", "pattern": "^[0-9a-f]{64}$"},
    "root": {"type": "string"},
    "state": {"type": "string"},
    "tag": {"type": "string", "pattern": "^[0-9a-f]{12}$"},
    "directory": {"type": "string"},
    "endpoint": {"type": "string"},
    "log": {"type": "string"}
   }
  },
  "HostWorkspace": {'''),
 (R+"🧬️schema/🔣️.json",'''      "host-down": "No repo graph service is running on this host.",''','''      "host-down": "No repo graph service is running on this host.",
      "host-identity": "Identity: code {code}, checkout {root}, machine state {state}, since {started}; {inFlight} running, {queued} waiting; socket {endpoint}, log {log}.",'''),
 (R+"🧬️schema/🔣️.json",'''      "host-down": "Auf diesem Rechner läuft kein Repo-Graphdienst.",''','''      "host-down": "Auf diesem Rechner läuft kein Repo-Graphdienst.",
      "host-identity": "Identität: Code {code}, Checkout {root}, Maschinenzustand {state}, seit {started}; {inFlight} laufen, {queued} warten; Socket {endpoint}, Log {log}.",'''),
 (L+"🧭️invocation/🧪️tests/🛰️plan/🟦️.ts",'afterAll(()=>rmSync(base,{recursive:true,force:true}));','const runBase=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","cp-"));\nafterAll(()=>{rmSync(base,{recursive:true,force:true});rmSync(runBase,{recursive:true,force:true});});'),
 (L+"🧭️invocation/🧪️tests/🛰️plan/🟦️.ts",'join(base,"run")','join(runBase,"run")'),
 (L+"🧭️invocation/🧪️tests/🛰️plan/🟦️.ts",'join(base,"slow")','join(runBase,"slow")'),
 (R+"⌨️command/🌐️host/🟨️.mjs",'''   if(found.memory)print(say("host-memory"''','''   if(found.identity)print(say("host-identity",{code:found.identity.implementation.slice(0,12),root:found.identity.root,state:found.identity.state||"(checkout)",started:new Date(found.startedAt).toISOString(),inFlight:found.inFlight,queued:found.queue,endpoint:found.identity.endpoint,log:found.identity.log}));
   if(found.memory)print(say("host-memory"'''),
]
BK=S+"backup/"
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
