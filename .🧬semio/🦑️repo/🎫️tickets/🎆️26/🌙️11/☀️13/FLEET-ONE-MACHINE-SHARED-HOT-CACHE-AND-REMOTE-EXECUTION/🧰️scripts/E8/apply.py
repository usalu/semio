import json,os,re,sys,hashlib
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
S=os.path.dirname(os.path.abspath(__file__))
READY=B+"🌐️remote-execution/🛰️sidecar/🚦️ready/"
FS=B+"🚢️fleet-status/"
def rd(p): return open(p,encoding="utf8").read()
def wr(p,t):
    os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(t)
def sub(text,old,new,count=1):
    assert text.count(old)==count,("anchor count",text.count(old),old[:90])
    return text.replace(old,new)
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()[:12] if os.path.exists(p) else "new"
files={}
def edit(path,fn):
    before=sha(path);t=rd(path);t2=fn(t)
    if t2!=t: wr(path,t2)
    files[path.replace(B,"")]=(before,sha(path))

# A silence module
def silence(t):
    if "readyWorkerLivenessV1" in t: return t
    return t.rstrip("\n")+"\n"+rd(S+"/liveness.ts.part")
edit(READY+"💔️silence/🟦️.ts",silence)
def silence_test(t):
    if "worker liveness" in t: return t
    t=sub(t,'import {expect,test} from "bun:test";\nimport {readySidecarSilenceV1,readySilenceNoticeV1,readySilenceWatchV1} from "../🟦️.ts";',
     'import {expect,test} from "bun:test";\nimport {spawnSync} from "node:child_process";\nimport {join} from "node:path";\nimport {readySidecarSilenceV1,readySilenceNoticeV1,readySilenceWatchV1,readyWorkerLivenessV1} from "../🟦️.ts";\nimport {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";')
    return t.rstrip("\n")+"\n"+rd(S+"/liveness.test.part")
edit(READY+"💔️silence/🧪️tests/🟦️.ts",silence_test)
if not os.path.exists(READY+"💔️silence/🧪️tests/🐍️.py"): wr(READY+"💔️silence/🧪️tests/🐍️.py",rd(S+"/liveness.py"))
files["…/💔️silence/🧪️tests/🐍️.py"]=("new",sha(READY+"💔️silence/🧪️tests/🐍️.py"))
# B workers reader
for src,dst in(("workers.ts","🩺️workers/🟦️.ts"),("workers.test.ts","🩺️workers/🧪️tests/🟦️.ts")):
    if not os.path.exists(READY+dst): wr(READY+dst,rd(S+"/"+src))
    files["…/"+dst]=("new",sha(READY+dst))

# C fleet status schema
def schema_edit(t):
    if "WorkersFact" in t: return t
    t=sub(t,'"scheduler",\n      "fleet"\n     ]','"scheduler",\n      "fleet",\n      "workers"\n     ]')
    t=sub(t,'"fleet": {\n     "$ref": "#/definitions/FleetFact"\n    }','"fleet": {\n     "$ref": "#/definitions/FleetFact"\n    },\n    "workers": {\n     "$ref": "#/definitions/WorkersFact"\n    }')
    t=sub(t,'"fleet": {\n     "$ref": "#/definitions/FleetSection"\n    }','"fleet": {\n     "$ref": "#/definitions/FleetSection"\n    },\n    "workers": {\n     "$ref": "#/definitions/WorkersSection"\n    }')
    msgs={
     "workers":("Workers: {ready} ready, {working} working, {idle} idle, {silent} silent, {offline} offline, {absent} without a record","Worker: {ready} bereit, {working} arbeiten, {idle} untätig, {silent} still, {offline} offline, {absent} ohne Eintrag"),
     "workers.row":("  {origin} {platform}: {state}, stage {stage} ({index}/{count}), attempt {attempts}, record {record}, machine {machine}{refusal}","  {origin} {platform}: {state}, Stufe {stage} ({index}/{count}), Versuch {attempts}, Eintrag {record}, Maschine {machine}{refusal}"),
     "workers.refusal":(", {reason}",", {reason}"),
     "workers.state.absent":("no record","ohne Eintrag"),"workers.state.offline":("offline","offline"),"workers.state.silent":("silent","still"),"workers.state.working":("working","arbeitet"),"workers.state.ready":("ready","bereit"),"workers.state.idle":("idle","untätig"),
     "finding.worker-offline":("worker {origin} showed no sign of life for {age}; its record still says {recorded} in stage {stage} and is stale","Worker {origin} zeigte {age} lang kein Lebenszeichen; sein Eintrag sagt noch {recorded} in Stufe {stage} und ist veraltet"),
     "finding.worker-silent":("worker {origin} lives but wrote no readiness record for {age} although it is {recorded} in stage {stage}","Worker {origin} lebt, schrieb aber {age} lang keinen Bereitschaftseintrag, obwohl er {recorded} in Stufe {stage} ist"),
     "finding.worker-absent":("worker {origin} is enrolled but showed no sign of life and never wrote a readiness record","Worker {origin} ist eingetragen, zeigte aber kein Lebenszeichen und schrieb nie einen Bereitschaftseintrag"),
    }
    block="".join('    %s: {\n     "en": %s,\n     "de": %s\n    },\n'%(json.dumps(k),json.dumps(en,ensure_ascii=False),json.dumps(de,ensure_ascii=False)) for k,(en,de) in msgs.items())
    anchor='    "error.syntax": {'
    t=sub(t,anchor,block+anchor)
    defs='''  },
  "WorkerRow": {
   "type": "object",
   "additionalProperties": false,
   "required": ["origin", "platform", "liveness", "recorded", "stage", "stageIndex", "stageCount", "attempts", "recordAgeSeconds", "heartbeatAgeSeconds", "refusal"],
   "properties": {
    "origin": {"type": "string"},
    "platform": {"type": ["string", "null"]},
    "liveness": {"enum": ["absent", "offline", "silent", "working", "ready", "idle"]},
    "recorded": {"type": ["string", "null"]},
    "stage": {"type": ["string", "null"]},
    "stageIndex": {"type": "integer", "minimum": 0},
    "stageCount": {"type": "integer", "minimum": 0},
    "attempts": {"type": "integer", "minimum": 0},
    "recordAgeSeconds": {"type": ["integer", "null"], "minimum": 0},
    "heartbeatAgeSeconds": {"type": ["integer", "null"], "minimum": 0},
    "refusal": {"anyOf": [{"type": "null"}, {"type": "object", "additionalProperties": false, "required": ["code", "en", "de"], "properties": {"code": {"type": "string"}, "en": {"type": "string"}, "de": {"type": "string"}}}]}
   }
  },
  "WorkersFact": {
   "description": "The workers of the fleet as their readiness records and the heartbeats of their machines say, judged by the age rule of the readiness schema: a record alone never proves a worker lives.",
   "type": "object",
   "additionalProperties": false,
   "required": ["offlineSeconds", "rows"],
   "properties": {
    "offlineSeconds": {"type": "integer", "minimum": 0},
    "rows": {"type": "array", "items": {"$ref": "#/definitions/WorkerRow"}}
   }
  },
  "WorkersSection": {
   "type": "object",
   "additionalProperties": false,
   "required": ["offlineSeconds", "counts", "rows"],
   "properties": {
    "offlineSeconds": {"type": "integer", "minimum": 0},
    "counts": {"type": "object", "additionalProperties": false, "required": ["ready", "working", "idle", "silent", "offline", "absent"], "properties": {"ready": {"type": "integer", "minimum": 0}, "working": {"type": "integer", "minimum": 0}, "idle": {"type": "integer", "minimum": 0}, "silent": {"type": "integer", "minimum": 0}, "offline": {"type": "integer", "minimum": 0}, "absent": {"type": "integer", "minimum": 0}}},
    "rows": {"type": "array", "items": {"$ref": "#/definitions/WorkerRow"}}
   }
  }
 }
}
'''
    tail='  }\n }\n}\n'
    assert t.endswith(tail)
    return t[:-len(tail)]+defs
edit(FS+"🧬️schema/🔣️.json",schema_edit)
json.loads(rd(FS+"🧬️schema/🔣️.json"))

# fleet status TS
def fs_ts(t):
    if "FleetWorkersFactV1" in t: return t
    t=sub(t,'"scheduler"|"fleet";ok:boolean','"scheduler"|"fleet"|"workers";ok:boolean')
    t=sub(t,'export type FleetFindingV1=','''/** 🩺️ One worker of the fleet: what its readiness record claims (`recorded`) next to what is really known (`liveness`, from the age of the record and of the heartbeats of its machine). */
export type FleetWorkerFactV1=Readonly<{origin:string;platform:string|null;liveness:"absent"|"offline"|"silent"|"working"|"ready"|"idle";recorded:string|null;stage:string|null;stageIndex:number;stageCount:number;attempts:number;recordAgeSeconds:number|null;heartbeatAgeSeconds:number|null;refusal:Readonly<{code:string;en:string;de:string}>|null}>;
export type FleetWorkersFactV1=Readonly<{offlineSeconds:number;rows:readonly FleetWorkerFactV1[]}>;
export type FleetFindingV1=''')
    t=sub(t,' fleet?:FleetLoadFactV1;\n',' fleet?:FleetLoadFactV1;\n workers?:FleetWorkersFactV1;\n')
    t=sub(t,'scheduler?:Row;fleet?:Row;findings:','scheduler?:Row;fleet?:Row;workers?:Row;findings:')
    t=sub(t,'\nconst natural=','''
const workersSection=(fact:FleetWorkersFactV1):Row=>{const rows=[...fact.rows].sort((left,right)=>compare(left.origin,right.origin)),count=(state:string)=>rows.filter(row=>row.liveness===state).length;return{offlineSeconds:fact.offlineSeconds,counts:{ready:count("ready"),working:count("working"),idle:count("idle"),silent:count("silent"),offline:count("offline"),absent:count("absent")},rows};};

const natural=''')
    t=sub(t,' for(const source of facts.sources)if(!source.ok)say("source-failed"',''' for(const worker of facts.workers?.rows??[]){
  const seen=Math.min(worker.recordAgeSeconds??Infinity,worker.heartbeatAgeSeconds??Infinity),values={origin:worker.origin,age:fleetDurationV1(language,Number.isFinite(seen)?seen:0),recorded:worker.recorded??"-",stage:worker.stage??"-"};
  if(worker.liveness==="offline")say("worker-offline","warn",worker.origin,values);
  else if(worker.liveness==="silent")say("worker-silent","warn",worker.origin,{...values,age:fleetDurationV1(language,worker.recordAgeSeconds??0)});
  else if(worker.liveness==="absent"&&(worker.heartbeatAgeSeconds===null||worker.heartbeatAgeSeconds>=facts.workers!.offlineSeconds))say("worker-absent","info",worker.origin,{origin:worker.origin});
 }
 for(const source of facts.sources)if(!source.ok)say("source-failed"''')
    t=sub(t,'...facts.fleet===undefined?{}:{fleet:fleetSection(facts.fleet)},findings:kept','...facts.fleet===undefined?{}:{fleet:fleetSection(facts.fleet)},...facts.workers===undefined?{}:{workers:workersSection(facts.workers)},findings:kept')
    t=sub(t,' const bazel=report.bazel;\n',''' const workers=report.workers;
 if(workers!==undefined){
  const counts=workers.counts as Record<string,number>;
  lines.push(say("workers",counts));
  for(const row of workers.rows as Row[])lines.push(say("workers.row",{origin:row.origin,platform:row.platform??"-",state:say(`workers.state.${row.liveness}`),stage:row.stage??"-",index:row.stageIndex,count:row.stageCount,attempts:row.attempts,record:row.recordAgeSeconds===null?"-":duration(row.recordAgeSeconds),machine:row.heartbeatAgeSeconds===null?"-":duration(row.heartbeatAgeSeconds),refusal:row.refusal===null?"":say("workers.refusal",{reason:row.refusal[language]})}));
 }
 const bazel=report.bazel;
''')
    return t
edit(FS+"🟦️.ts",fs_ts)

def readers(t):
    if "readFleetWorkersV1" in t: return t
    t=sub(t,'/** 🏭️ The readers of the real workspace. */','''/** 🩺️ Workers: every origin of the fleet store judged by the age of its readiness record and of the heartbeats of its machine, so a receipt that claims `running` with no live writer shows as offline; nothing without a fleet store. */
export async function readFleetWorkersV1(context:FleetReaderContextV1):Promise<Partial<FleetFactsV1>>{
 const [{readReadyWorkersV1,READY_OFFLINE_SECONDS_V1},{fleetStoreRootV1}]=await Promise.all([import("../../🌐️remote-execution/🛰️sidecar/🚦️ready/🩺️workers/🟦️.ts"),import("../../🌐️remote-cache/🛰️fleet/🟦️.ts")]);
 const store=context.environment.SEMIO_FLEET_STORE_ROOT??fleetStoreRootV1(context.root);
 if(store===undefined||store==="")return {};
 const rows=readReadyWorkersV1({store,nowMs:context.now()});
 return rows.length===0?{}:{workers:{offlineSeconds:READY_OFFLINE_SECONDS_V1,rows}};
}

/** 🏭️ The readers of the real workspace. */''')
    return sub(t,'scheduler:readFleetSchedulerV1,fleet:readFleetLoadV1});','scheduler:readFleetSchedulerV1,fleet:readFleetLoadV1,workers:readFleetWorkersV1});')
edit(FS+"📥️readers/🟦️.ts",readers)

def py(t):
    if "workers_section" in t: return t
    t=sub(t,'    for s in facts["sources"]:\n        if not s["ok"]: say("source-failed"','''    for w in (facts.get("workers") or {}).get("rows",[]):
        ages=[a for a in (w["recordAgeSeconds"],w["heartbeatAgeSeconds"]) if a is not None]
        values={"origin":w["origin"],"age":duration(language,min(ages) if ages else 0),"recorded":w["recorded"] if w["recorded"] is not None else "-","stage":w["stage"] if w["stage"] is not None else "-"}
        if w["liveness"]=="offline": say("worker-offline","warn",w["origin"],values)
        elif w["liveness"]=="silent": say("worker-silent","warn",w["origin"],dict(values,age=duration(language,w["recordAgeSeconds"] or 0)))
        elif w["liveness"]=="absent" and (w["heartbeatAgeSeconds"] is None or w["heartbeatAgeSeconds"]>=facts["workers"]["offlineSeconds"]): say("worker-absent","info",w["origin"],{"origin":w["origin"]})
    for s in facts["sources"]:
        if not s["ok"]: say("source-failed"''')
    t=sub(t,'**fleet_section(facts)}','**fleet_section(facts),**workers_section(facts)}')
    t=sub(t,'def generate(seed,count):','''def workers_section(facts):
    f=facts.get("workers")
    if not f: return {}
    rows=sorted(f["rows"],key=lambda r:r["origin"]);count=lambda state:len([r for r in rows if r["liveness"]==state])
    return {"workers":{"offlineSeconds":f["offlineSeconds"],"counts":{k:count(k) for k in ("ready","working","idle","silent","offline","absent")},"rows":rows}}

def generate(seed,count):''')
    t=sub(t,'        for s in facts["sources"]:\n            if s["ok"]: s.pop("error",None)','''        wr=random.Random(seed*104729+index)
        if wr.random()<0.5:
            def worker(origin):
                live=wr.choice(["absent","offline","silent","working","ready","idle"])
                none=live=="absent"
                return {"origin":origin,"platform":None if none and wr.random()<0.5 else wr.choice(["linux","macos","windows"]),"liveness":live,"recorded":None if none else wr.choice(["running","waiting","backoff","ready","observing"]),"stage":None if none else wr.choice(["tools","workspace","dependencies",None]),"stageIndex":0 if none else wr.randint(0,5),"stageCount":0 if none else 5,"attempts":0 if none else wr.randint(0,60),"recordAgeSeconds":None if none else wr.choice([0,5,95,599,600,7200,86400]),"heartbeatAgeSeconds":wr.choice([None,0,3,700,86400]),"refusal":wr.choice([None,{"code":"stage-timeout","en":"timed out","de":"Zeit überschritten"}])}
            facts["workers"]={"offlineSeconds":600,"rows":[worker(n) for n in wr.sample(["g1-m01","g1-m04","g1-m09","g1-m12","g1-server"],wr.randint(1,5))]}
        for s in facts["sources"]:
            if s["ok"]: s.pop("error",None)''')
    return t
edit(FS+"🧪️tests/🐍️.py",py)
print(json.dumps(files,ensure_ascii=False,indent=1))
