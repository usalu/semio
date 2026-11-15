import glob,json,os,sys
# Anchored edits that add the supervised sidecars to the replica module. usage: land-replica.py <module root> [--revert]
root=sys.argv[1];revert="--revert" in sys.argv
def read(p): return open(os.path.join(root,p),encoding="utf8").read()
def write(p,s): open(os.path.join(root,p),"w",encoding="utf8").write(s)
SCRIPT=glob.glob(os.path.join(os.path.dirname(root.rstrip("/")),"..","..","..","..","..","*"))  # unused
edits=[]
def edit(path,old,new):
    edits.append((path,old,new))

CHILD_SCRIPT=sys.argv[2] if len(sys.argv)>2 and not sys.argv[2].startswith("--") else None
script_path=CHILD_SCRIPT or "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts"
side='"sidecars": {"enabled": true, "startAfterSeconds": 30, "graceSeconds": 10, "restart": {"initialMilliseconds": 1000, "maximumMilliseconds": 60000, "stableMilliseconds": 60000}, "children": [\n  {"name": "build-agent", "script": "%s", "arguments": ["agent"], "roles": ["member", "server"], "requires": "platform-peer", "enabled": true},\n  {"name": "build-scheduler", "script": "%s", "arguments": ["scheduler"], "roles": ["server"], "requires": "none", "enabled": true}\n ]}'%(script_path,script_path)
edit("🗺️topology/🔣️.json",'"catchUpEvents": 20000}}\n}','"catchUpEvents": 20000}},\n '+side+'\n}')
edit("🗺️topology/🧬️schema/🔣️.json",'  "Topology": {','''  "SidecarChild": {
   "description": "One child the replica daemon supervises beside itself: a script below the worktree with its arguments, the machines it runs on (`member`: every machine, `server`: the group server) and what it needs (`platform-peer`: another machine of the same platform has attested within ten minutes).",
   "type": "object",
   "additionalProperties": false,
   "required": ["name", "script", "arguments", "roles", "requires", "enabled"],
   "properties": {
    "name": {"type": "string", "pattern": "^[a-z][a-z0-9-]{0,39}$"},
    "script": {"type": "string", "minLength": 1, "maxLength": 240},
    "arguments": {"type": "array", "maxItems": 16, "items": {"type": "string", "maxLength": 128}},
    "roles": {"type": "array", "minItems": 1, "maxItems": 2, "uniqueItems": true, "items": {"enum": ["member", "server"]}},
    "requires": {"enum": ["none", "platform-peer"]},
    "enabled": {"type": "boolean"}
   }
  },
  "Sidecars": {
   "description": "The supervised sidecars: the master switch, how long a healthy daemon waits before it starts them, the restart back-off (first delay, ceiling, the run time after which the count of failures starts again), the grace a stopping child gets before it is killed, and the children.",
   "type": "object",
   "additionalProperties": false,
   "required": ["enabled", "startAfterSeconds", "graceSeconds", "restart", "children"],
   "properties": {
    "enabled": {"type": "boolean"},
    "startAfterSeconds": {"type": "integer", "minimum": 0, "maximum": 3600},
    "graceSeconds": {"type": "integer", "minimum": 1, "maximum": 300},
    "restart": {
     "type": "object",
     "additionalProperties": false,
     "required": ["initialMilliseconds", "maximumMilliseconds", "stableMilliseconds"],
     "properties": {
      "initialMilliseconds": {"type": "integer", "minimum": 10, "maximum": 600000},
      "maximumMilliseconds": {"type": "integer", "minimum": 10, "maximum": 3600000},
      "stableMilliseconds": {"type": "integer", "minimum": 10, "maximum": 86400000}
     }
    },
    "children": {"type": "array", "maxItems": 8, "items": {"$ref": "#/definitions/SidecarChild"}}
   }
  },
  "Topology": {''')
edit("🗺️topology/🧬️schema/🔣️.json",'    "replica": {"$ref": "#/definitions/Replica"}\n   }','    "replica": {"$ref": "#/definitions/Replica"},\n    "sidecars": {"$ref": "#/definitions/Sidecars"}\n   }')
edit("🗺️topology/🟦️.ts",'import buildPolicy from "../../📚️library/🏗️build/🔣️policy.json" with {type:"json"};','import buildPolicy from "../../📚️library/🏗️build/🔣️policy.json" with {type:"json"};\nimport type {SidecarSettingsV1} from "../🔁️replica/🧷️sidecars/🟦️.ts";')
edit("🗺️topology/🟦️.ts",'catchUpEvents:number}>}>}>;','catchUpEvents:number}>}>;sidecars?:SidecarSettingsV1}>;')
edit("🛡️resilience/🧬️schema/🔣️.json",'''    "failures": {
     "type": "array",
     "maxItems": 8,
     "items": {
      "$ref": "#/definitions/Failure"
     }
    }
   }
  },
  "JitterCase": {''','''    "failures": {
     "type": "array",
     "maxItems": 8,
     "items": {
      "$ref": "#/definitions/Failure"
     }
    },
    "sidecars": {
     "type": "array",
     "maxItems": 8,
     "items": {
      "$ref": "#/definitions/Sidecar"
     }
    }
   }
  },
  "Sidecar": {
   "type": "object",
   "additionalProperties": false,
   "required": [
    "lastExit",
    "name",
    "nextStartMs",
    "pid",
    "restarts",
    "sinceMs",
    "state"
   ],
   "properties": {
    "name": {
     "type": "string",
     "pattern": "^[a-z][a-z0-9-]{0,39}$"
    },
    "state": {
     "enum": [
      "off",
      "waiting",
      "running",
      "backoff",
      "stopped"
     ]
    },
    "pid": {
     "anyOf": [
      {
       "type": "null"
      },
      {
       "$ref": "#/definitions/Count"
      }
     ]
    },
    "restarts": {
     "$ref": "#/definitions/Count"
    },
    "sinceMs": {
     "$ref": "#/definitions/Count"
    },
    "lastExit": {
     "anyOf": [
      {
       "type": "null"
      },
      {
       "type": "string",
       "maxLength": 512
      }
     ]
    },
    "nextStartMs": {
     "anyOf": [
      {
       "type": "null"
      },
      {
       "$ref": "#/definitions/Count"
      }
     ]
    }
   }
  },
  "JitterCase": {''')
edit("🛡️resilience/🟦️.ts",'pending:Readonly<{code:string;ok:false;reason:string}>|null;failures:readonly FailureSummaryV1[]}>;','pending:Readonly<{code:string;ok:false;reason:string}>|null;failures:readonly FailureSummaryV1[];sidecars?:readonly SidecarStateV1[]}>;')
edit("🛡️resilience/🟦️.ts",'import schema from "./🧬️schema/🔣️.json" with {type:"json"};','import schema from "./🧬️schema/🔣️.json" with {type:"json"};\nimport type {SidecarStateV1} from "../🔁️replica/🧷️sidecars/🟦️.ts";')
edit("🔁️replica/🧬️schema/🔣️.json",'''"lastMs": { "$ref": "#/definitions/Count" } } }
                }
              }
            }
          ]
        },
        "git": {''','''"lastMs": { "$ref": "#/definitions/Count" } } }
                },
                "sidecars": {
                  "type": "array",
                  "maxItems": 8,
                  "description": "The children the daemon supervises beside itself (the build agent and, on the group server, the scheduler): state, process id, restarts and the last exit.",
                  "items": { "type": "object", "additionalProperties": false, "required": ["name", "state", "pid", "restarts", "sinceMs", "lastExit", "nextStartMs"], "properties": { "name": { "type": "string", "pattern": "^[a-z][a-z0-9-]{0,39}$" }, "state": { "enum": ["off", "waiting", "running", "backoff", "stopped"] }, "pid": { "oneOf": [{ "type": "null" }, { "$ref": "#/definitions/Count" }] }, "restarts": { "$ref": "#/definitions/Count" }, "sinceMs": { "$ref": "#/definitions/Count" }, "lastExit": { "oneOf": [{ "type": "null" }, { "type": "string", "maxLength": 512 }] }, "nextStartMs": { "oneOf": [{ "type": "null" }, { "$ref": "#/definitions/Count" }] } } }
                }
              }
            }
          ]
        },
        "git": {''')
edit("🔁️replica/🟦️.ts",'driftMilliseconds?:number;opened?:(replica:ReplicaV1)=>void}>;','driftMilliseconds?:number;opened?:(replica:ReplicaV1)=>void;sidecars?:SidecarSettingsV1}>;')
edit("🔁️replica/🟦️.ts",'import {codeDirectoryV1,createFailuresV1,','import {createSidecarsV1,type SidecarSettingsV1} from "./🧷️sidecars/🟦️.ts";\nimport {codeDirectoryV1,createFailuresV1,')
edit("🔁️replica/🟦️.ts",' const booted=attester.start(signal).catch(()=>{});',' const booted=attester.start(signal).catch(()=>{});\n const sidecars=options.sidecars===undefined?undefined:createSidecarsV1({settings:options.sidecars,facts:{worktree:settings.worktree,storeRoot:settings.storeRoot,originId:settings.originId,groupId:settings.groupId,platform:process.platform==="darwin"?"macos":process.platform==="win32"?"windows":"linux"}});')
edit("🔁️replica/🟦️.ts",'    const wall=Date.now();\n    if(wall-healthAt>=(options.attestMilliseconds??healthMilliseconds)){healthAt=wall;await writeHealthV1(settings.worktree,{version:1,wallMs:wall,pid:process.pid,code:attester.running()===""?"0".repeat(64):attester.running(),store:guard.state(),storeSinceMs:guard.since(),pending:attester.pending(),failures:[...replica.failures.summary()]})','    const wall=Date.now();\n    sidecars?.tick(guard.state()==="ok"&&attester.running()!==""&&!parked);\n    if(wall-healthAt>=(options.attestMilliseconds??healthMilliseconds)){healthAt=wall;await writeHealthV1(settings.worktree,{version:1,wallMs:wall,pid:process.pid,code:attester.running()===""?"0".repeat(64):attester.running(),store:guard.state(),storeSinceMs:guard.since(),pending:attester.pending(),failures:[...replica.failures.summary()],...sidecars===undefined?{}:{sidecars:sidecars.report()}})')
edit("🔁️replica/🟦️.ts",'  clearInterval(ticker);\n','  await sidecars?.stop().catch(()=>{});\n  clearInterval(ticker);\n')
edit("🔁️replica/🟦️.ts",'failures:beat.failures}','failures:beat.failures,...beat.sidecars===undefined?{}:{sidecars:beat.sidecars}}')
edit("🔁️replica/⌨️command/🟦️.ts",'compaction:defaults.compaction}};','compaction:defaults.compaction,...topology.sidecars===undefined?{}:{sidecars:topology.sidecars}}};')
edit("🔁️replica/⌨️command/🟦️.ts",'  for(const failure of report.health.failures)say("status.health.failure",{kind:failure.kind,count:failure.count,seconds:seconds(failure.lastMs)});','  for(const failure of report.health.failures)say("status.health.failure",{kind:failure.kind,count:failure.count,seconds:seconds(failure.lastMs)});\n  for(const item of report.health.sidecars??[])say("status.health.sidecar",{name:item.name,state:`@status.sidecar.${item.state}`,pid:item.pid??"-",restarts:item.restarts,seconds:seconds(item.sinceMs),exit:item.lastExit??"-"});')
edit("🔁️replica/📖️texts/🔣️.json",'''    "status.health.failure": {''','''    "status.health.sidecar": {
      "en": "Sidecar {name}: {state} (process {pid}, {restarts} restart(s), {seconds} s in this state, last exit {exit}).",
      "de": "Sidecar {name}: {state} (Prozess {pid}, {restarts} Neustart(s), seit {seconds} s in diesem Zustand, letztes Ende {exit})."
    },
    "status.sidecar.off": {
      "en": "off on this machine",
      "de": "auf dieser Maschine aus"
    },
    "status.sidecar.waiting": {
      "en": "waiting for a healthy daemon",
      "de": "wartet auf einen gesunden Daemon"
    },
    "status.sidecar.running": {
      "en": "running",
      "de": "läuft"
    },
    "status.sidecar.backoff": {
      "en": "failed, restarts with back-off",
      "de": "ausgefallen, startet mit Wartezeit neu"
    },
    "status.sidecar.stopped": {
      "en": "stopped with the daemon",
      "de": "mit dem Daemon beendet"
    },
    "status.health.failure": {''')
if revert:
    edits=[(p,n,o) for (p,o,n) in reversed(edits)]
state=[]
for p,o,n in edits:
    s=read(p)
    if s.count(o)!=1:
        print("ANCHOR",s.count(o),p,repr(o[:70]));sys.exit(1)
cache={}
for p,o,n in edits:
    s=cache.get(p) or read(p)
    if s.count(o)!=1: print("ANCHOR after earlier edit",p,repr(o[:70]));sys.exit(1)
    cache[p]=s.replace(o,n)
for p,s in cache.items(): write(p,s)
print("reverted" if revert else "applied",len(edits),"edits in",len(cache),"files")
