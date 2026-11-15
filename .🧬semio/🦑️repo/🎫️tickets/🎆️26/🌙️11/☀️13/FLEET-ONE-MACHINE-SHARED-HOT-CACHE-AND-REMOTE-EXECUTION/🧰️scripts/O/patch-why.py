#!/usr/bin/env python3
"""Package O landing 1: a worker that is absent says why (reason code + EN/DE text in the load report, the scheduler row and `repo fleet status`).
Usage: patch-why.py <build-root> <new-files-dir> [--dry]. Anchored: every anchor must occur exactly once or nothing is written."""
import sys, pathlib, shutil
B = pathlib.Path(sys.argv[1]); NEW = pathlib.Path(sys.argv[2]); DRY = "--dry" in sys.argv
RE = B / "🌐️remote-execution"
edits = {}

def read(p):
    return edits.get(p) if p in edits else p.read_text()

def sub(p, old, new, count=1):
    t = read(p)
    n = t.count(old)
    assert n == count, f"{p.name}: anchor occurs {n}x (want {count}): {old[:80]!r}"
    edits[p] = t.replace(old, new)

# 1 schema
S = B / "🏃️execution/🧬️schema/🌐️remote-execution.json"
sub(S, '''        "network": {
          "oneOf": [
            {
              "$ref": "#/definitions/NetworkPolicy"
            },
            {
              "type": "null"
            }
          ]
        }
      }
    },
    "LoadReport": {''', '''        "network": {
          "oneOf": [
            {
              "$ref": "#/definitions/NetworkPolicy"
            },
            {
              "type": "null"
            }
          ]
        },
        "why": {
          "$ref": "#/definitions/WorkerWhy"
        }
      }
    },
    "WorkerWhy": {
      "description": "Why a worker is absent: a closed reason code and the same sentence in English and German.",
      "type": "object",
      "additionalProperties": false,
      "required": [
        "code",
        "en",
        "de"
      ],
      "properties": {
        "code": {
          "enum": [
            "starting",
            "no-scheduler",
            "scheduler-unreachable",
            "identity-unregistered",
            "no-toolchain",
            "provisioning",
            "sandbox-unavailable",
            "disk-full",
            "policy-off",
            "not-registered",
            "failed"
          ]
        },
        "en": {
          "type": "string",
          "minLength": 1,
          "maxLength": 400
        },
        "de": {
          "type": "string",
          "minLength": 1,
          "maxLength": 400
        }
      }
    },
    "LoadReport": {''')
sub(S, '''        "confinement",
        "actions"
      ],
      "properties": {
        "concurrency": {''', '''        "confinement",
        "actions",
        "minimumFreeGigabytes"
      ],
      "properties": {
        "minimumFreeGigabytes": {
          "type": "integer",
          "minimum": 0,
          "maximum": 100000,
          "description": "The worker does not start while the volume of its work directory has less free space than this."
        },
        "concurrency": {''')

# 2 policy
sub(B / "🔣️policy.json", '''      "confinement": "best-effort",
      "actions": "rustc"
    }''', '''      "confinement": "best-effort",
      "actions": "rustc",
      "minimumFreeGigabytes": 20
    }''')

# 3 load module
L = RE / "🌡️load/🟦️.ts"
sub(L, 'export type RemoteExecutionWorkerStateV1=Readonly<{state:"absent"|"idle"|"busy"|"refusing";concurrency:number;running:number;sandbox:RemoteExecutionSandboxV1|null;confinement:RemoteExecutionConfinementV1|null;network:RemoteExecutionNetworkV1|null}>;',
 'export type RemoteExecutionWorkerStateV1=Readonly<{state:"absent"|"idle"|"busy"|"refusing";concurrency:number;running:number;sandbox:RemoteExecutionSandboxV1|null;confinement:RemoteExecutionConfinementV1|null;network:RemoteExecutionNetworkV1|null;why?:RemoteExecutionWorkerWhyV1}>;\n\nexport {remoteExecutionWorkerWhyOfMessageV1,remoteExecutionWorkerWhyV1,type RemoteExecutionWorkerWhyCodeV1,type RemoteExecutionWorkerWhyV1} from "./🔎️why/🟦️.ts";')
sub(L, 'import {requireRemoteExecutionShapeV1,', 'import type {RemoteExecutionWorkerWhyV1} from "./🔎️why/🟦️.ts";\nimport {requireRemoteExecutionShapeV1,')

# 4 agent
A = RE / "🛰️agent/🟦️.ts"
sub(A, 'import {runRemoteExecutionWorkerV1,', 'import {remoteExecutionWorkerWhyOfMessageV1,remoteExecutionWorkerWhyV1,type RemoteExecutionWorkerWhyV1} from "../🌡️load/🔎️why/🟦️.ts";\nimport {runRemoteExecutionWorkerV1,')
sub(A, 'loadIntervalSeconds:number;worker?:boolean;', 'loadIntervalSeconds:number;worker?:boolean;why?:RemoteExecutionWorkerWhyV1;retryWorkerSeconds?:number;')
sub(A, 'confinement:null,network:null};\n', 'confinement:null,network:null};\n let why:RemoteExecutionWorkerWhyV1|undefined=options.worker===false?options.why:remoteExecutionWorkerWhyV1("starting");\n')
sub(A, 'worker:state,counters:readRemoteExecutionCountersV1(options.root)', 'worker:why!==undefined&&state.state==="absent"?{...state,why}:state,counters:readRemoteExecutionCountersV1(options.root)')
sub(A, ' const worker=options.worker===false?Promise.resolve():runRemoteExecutionWorkerV1({...options,origin,properties,signal:controller.signal,admit,observe:next=>{state=next;},...capacity?{capacity}:{}}).catch(error=>{controller.abort();throw error;});',
''' const remember=(next:RemoteExecutionWorkerStateV1):void=>{state=next;if(next.state!=="absent")why=undefined;};
 const onEvent=(event:RemoteExecutionWorkerEventV1):void=>{
  if(event.stage==="registered")why=undefined;
  else if(event.stage==="reconnecting"){why=remoteExecutionWorkerWhyOfMessageV1(event.detail);state={...state,state:"absent",running:0};}
  options.onEvent?.(event);
 };
 const runWorker=async():Promise<void>=>{
  for(;;){
   try{await runRemoteExecutionWorkerV1({...options,origin,properties,signal:controller.signal,admit,observe:remember,onEvent,...capacity?{capacity}:{}});return;}
   catch(error){
    if(options.retryWorkerSeconds===undefined||controller.signal.aborted){controller.abort();throw error;}
    why=remoteExecutionWorkerWhyOfMessageV1((error as Error).message);state={...state,state:"absent",running:0};
    await new Promise<void>(done=>{const timer=setTimeout(done,options.retryWorkerSeconds!*1000);controller.signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});});
    if(controller.signal.aborted)return;
   }
  }
 };
 const worker=options.worker===false?Promise.resolve():runWorker();''')

# 5 scheduler
SC = RE / "🗓️scheduler/🟦️.ts"
sub(SC, 'import type {RemoteExecutionLoadReportV1} from "../🌡️load/🟦️.ts";', 'import type {RemoteExecutionLoadReportV1,RemoteExecutionWorkerStateV1} from "../🌡️load/🟦️.ts";\nimport {remoteExecutionWorkerWhyV1} from "../🌡️load/🔎️why/🟦️.ts";\n\nconst withoutWhyV1=(state:RemoteExecutionWorkerStateV1):Omit<RemoteExecutionWorkerStateV1,"why">=>{const {why:_why,...rest}=state;return rest;};')
sub(SC, 'state=worker===undefined?{...report.worker,state:"absent" as const,running:0}:{...report.worker,running:worker.running.size,', 'state=worker===undefined?{...report.worker,state:"absent" as const,running:0,why:report.worker.why??remoteExecutionWorkerWhyV1("not-registered")}:{...withoutWhyV1(report.worker),running:worker.running.size,')

# 6 status reader
ST = RE / "🚢️status/🟦️.ts"
sub(ST, 'worker:"absent"|"idle"|"busy"|"refusing";offloadedIn:number;offloadedOut:number;refused:number;cacheHit:number;cacheMiss:number;cacheSkipped?:number}>;', 'worker:"absent"|"idle"|"busy"|"refusing";why?:Readonly<{code:string;en:string;de:string}>;offloadedIn:number;offloadedOut:number;refused:number;cacheHit:number;cacheMiss:number;cacheSkipped?:number}>;')
sub(ST, 'worker:{state:RemoteExecutionFleetMachineV1["worker"]};', 'worker:{state:RemoteExecutionFleetMachineV1["worker"];why?:{code:string;en:string;de:string}};')
sub(ST, 'worker:row.worker.state,offloadedIn:', 'worker:row.worker.state,...row.worker.why===undefined?{}:{why:row.worker.why},offloadedIn:')

# 7 fleet status
F = B / "🚢️fleet-status"
sub(F / "🟦️.ts", 'worker:"absent"|"idle"|"busy"|"refusing";offloadedIn:number;offloadedOut:number;refused:number;cacheHit:number;cacheMiss:number;cacheSkipped?:number}>[];', 'worker:"absent"|"idle"|"busy"|"refusing";why?:Readonly<{code:string;en:string;de:string}>;offloadedIn:number;offloadedOut:number;refused:number;cacheHit:number;cacheMiss:number;cacheSkipped?:number}>[];')
sub(F / "🟦️.ts", '''cacheSkipped:machine.cacheSkipped??0,age:duration(machine.ageSeconds)}));
''', '''cacheSkipped:machine.cacheSkipped??0,age:duration(machine.ageSeconds)}));
  for(const machine of fleet.machines as Row[])if(machine.worker==="absent"&&machine.why!==undefined)lines.push(say("fleet.why",{origin:machine.origin,reason:machine.why[language]}));
''')
sub(F / "🧬️schema/🔣️.json", '''    "cacheSkipped": {"type": "integer", "minimum": 0}
   }
  },
  "FleetFact": {''', '''    "cacheSkipped": {"type": "integer", "minimum": 0},
    "why": {"type": "object", "additionalProperties": false, "required": ["code", "en", "de"], "properties": {"code": {"type": "string"}, "en": {"type": "string"}, "de": {"type": "string"}}}
   }
  },
  "FleetFact": {''')
sub(F / "🧬️schema/🔣️.json", '''    "fleet.noheadroom": {''', '''    "fleet.why": {
     "en": "    {origin} has no working build worker: {reason}",
     "de": "    {origin} hat keinen arbeitenden Build-Worker: {reason}"
    },
    "fleet.noheadroom": {''')

for p, t in edits.items():
    print("edit", p.relative_to(B))
if not DRY:
    for p, t in edits.items():
        p.write_text(t)
    for name, dest in (("why.ts", RE / "🌡️load/🔎️why/🟦️.ts"), ("readiness.ts", RE / "🛰️agent/🩺️readiness/🟦️.ts")):
        dest.parent.mkdir(parents=True, exist_ok=True)
        text = (NEW / name).read_text().replace('"../../🌡️load/🟦️.ts"', '"../../🌡️load/🔎️why/🟦️.ts"')
        dest.write_text(text)
        print("new", dest.relative_to(B))
