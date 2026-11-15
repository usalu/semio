"""R13 payload: lent fleet work never outranks a machine's own requests, only a ready machine lends, and the graph memory governor always makes progress.
EDITS are (path, old, new) anchored edits (old must occur exactly once; applied when old is gone and new present); an old of APPEND appends new to the file; TRANSFORMS are idempotent text functions."""
import json,re
TITLE="lent class in host capacity, readiness marker of lending machines, bounded memory headroom wait of the graph governor"
F="🧰️framework"
LIB=F+"/🛍️products/🦑️repo/🔨️modules/📚️library"
B=LIB+"/🏗️build"
HC=F+"/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
RE=B+"/🌐️remote-execution"
MEM=B+"/🧠️memory"
APPEND="<<APPEND>>"

POLICY_ADD={"waitBoundMilliseconds":60000,"fleetFallbackMilliseconds":180000,"fleetAskMilliseconds":10000,"grantMegabytes":2048,"refusalExitCode":75}
MSG={
"en":{
"waiting":"graph: waiting for memory headroom ({used} of {budget} MB reserved, {running} projects running): {key} needs {need} MB and has waited {seconds} s; after {limit} s it runs alone if it fits the {available} MB one project is granted, otherwise the run is refused with exit 75",
"learning":"graph: no memory peak learned for {kind} projects yet ({used} MB assumed, {budget} MB headroom), inferring one at a time: {key} has waited {seconds} s for the running project, at most {limit} s",
"admittedAlone":"graph: {key} waited {seconds} s for {need} MB of headroom ({budget} MB free, {running} project(s) running) and now runs because it fits the {available} MB one project is granted",
"refused":"graph: {key} needs {need} MB but one project is granted {available} MB here and {budget} MB are free after {seconds} s: refused with exit 75 (try again later or let the fleet infer it)",
"fleet":"graph: waited {seconds} s for memory headroom, asking the fleet to infer the remaining {count} project(s)",
},
"de":{
"waiting":"Graph: wartet auf Speicherreserve ({used} von {budget} MB reserviert, {running} Projekte laufen): {key} braucht {need} MB und wartet seit {seconds} s; nach {limit} s läuft es allein, wenn es in die {available} MB passt, die ein Projekt bekommt, sonst wird der Lauf mit Exit 75 abgewiesen",
"learning":"Graph: für {kind}-Projekte ist noch kein Speicherbedarf gelernt ({used} MB angenommen, {budget} MB Reserve), Projekte werden einzeln abgeleitet: {key} wartet seit {seconds} s auf das laufende Projekt, höchstens {limit} s",
"admittedAlone":"Graph: {key} hat {seconds} s auf {need} MB Reserve gewartet ({budget} MB frei, {running} Projekt(e) laufen) und läuft jetzt, weil es in die {available} MB passt, die ein Projekt bekommt",
"refused":"Graph: {key} braucht {need} MB, ein Projekt bekommt hier aber {available} MB und nach {seconds} s sind {budget} MB frei: abgewiesen mit Exit 75 (später erneut versuchen oder von der Flotte ableiten lassen)",
"fleet":"Graph: nach {seconds} s Warten auf Speicherreserve wird die Flotte gebeten, die übrigen {count} Projekt(e) abzuleiten",
}}

def render(language,key,fields):
    return re.sub(r"\{([a-z]+)\}",lambda m:str(fields[m.group(1)]),MSG[language][key])

def fixture_json(text):
    d=json.loads(text)
    d["policy"].update(POLICY_ADD)
    for lang in ("en","de"):d["messages"][lang].update(MSG[lang])
    waiting={"used":1234,"budget":3072,"running":5,"key":"@semio-tech/stdio-semio-rs","need":465,"seconds":12,"limit":60,"available":2048}
    learning={"kind":"language:rust","used":256,"budget":900,"key":"@semio-tech/stdio-semio","seconds":7,"limit":60}
    alone={"key":"@semio-tech/stdio-semio-rs","seconds":61,"need":465,"budget":413,"running":1,"available":2048}
    refused={"key":"@semio-tech/stdio-semio-rs","need":4096,"available":2048,"budget":413,"seconds":61}
    fleet={"seconds":181,"count":14}
    langs={"en_US":["en"],"de_DE":["de"],"":["en","de"]}
    cases=[c for c in d["messageCases"] if c["key"] not in ("waiting","learning","admittedAlone","refused","fleet")]
    def rows(key,fields):
        out=[]
        for lang,names in langs.items():
            env={"LANG":lang} if lang else {}
            out.append({"key":key,"fields":fields,"environment":env,"expect":[render(n,key,{"running":0,"kind":"",**fields}) for n in names]})
        return out
    head=rows("waiting",waiting)
    tail=[]
    for key,fields in (("learning",learning),("admittedAlone",alone),("refused",refused),("fleet",fleet)):
        r=rows(key,fields)
        tail+= r[:2] if key=="learning" else r
    d["messageCases"]=head+[c for c in cases]+tail
    return json.dumps(d,indent=1,ensure_ascii=False)+"\n"

def fixture_schema_json(text):
    d=json.loads(text)
    enum=d["properties"]["messageCases"]["items"]["properties"]["key"]["enum"]
    for k in ("admittedAlone","refused","fleet"):
        if k not in enum:enum.append(k)
    return json.dumps(d,indent=2,ensure_ascii=False)+"\n"

EDITS=[]
def edit(path,old,new):EDITS.append((path,old,new))

policy_lines="".join('        "%s": %s,\n'%(k,json.dumps(v)) for k,v in POLICY_ADD.items())
edit(MEM+"/🧬️schema/🔣️.json",
'        "limitEnvironment": "SEMIO_GRAPH_MEMORY_LIMIT_MB",\n        "stopCollectMinimumMilliseconds": 1000\n',
'        "limitEnvironment": "SEMIO_GRAPH_MEMORY_LIMIT_MB",\n'+policy_lines+'        "stopCollectMinimumMilliseconds": 1000\n')
edit(MEM+"/🧬️schema/🔣️.json",
'und am Ende jeder Welle.",\n  "definitions"',
'und am Ende jeder Welle. EN: The wait for headroom is bounded: when at most one project runs and the project at the head of the queue has waited `waitBoundMilliseconds`, the smallest pending project is admitted if its estimate fits the grant of one project (`grantMegabytes`, or the owner limit when the machine declares one), otherwise the run is refused with the typed error `BUILD_GRAPH_MEMORY_REFUSED` (exit code `refusalExitCode`, 75) that names what it needs and what one project is granted; every waiting line names the project, what it needs and how long it has waited, and after `fleetFallbackMilliseconds` the governor asks the fleet (when a fleet port is given, answered within `fleetAskMilliseconds`) for the remaining projects. DE: Das Warten auf Speicherreserve ist begrenzt: läuft höchstens ein Projekt und wartet das erste Projekt der Schlange seit `waitBoundMilliseconds`, wird das kleinste wartende Projekt zugelassen, wenn sein Bedarf in die Zusage eines Projekts passt (`grantMegabytes`, oder die Projektgrenze, wenn die Maschine eine erklärt), sonst wird der Lauf mit dem typisierten Fehler `BUILD_GRAPH_MEMORY_REFUSED` (Exit-Code `refusalExitCode`, 75) abgewiesen, der nennt, was es braucht und was ein Projekt bekommt; jede Wartezeile nennt das Projekt, seinen Bedarf und wie lange es wartet, und nach `fleetFallbackMilliseconds` bittet der Governor die Flotte (wenn ein Flottenport gegeben ist, Antwort binnen `fleetAskMilliseconds`) um die übrigen Projekte.",\n  "definitions"')
edit(MEM+"/🧬️schema/🔣️.json",
'          "waiting": "graph: waiting for memory headroom ({used} of {budget} MB reserved, {running} projects running)",\n',
'          "waiting": %s,\n          "admittedAlone": %s,\n          "refused": %s,\n          "fleet": %s,\n'%tuple(json.dumps(MSG["en"][k],ensure_ascii=False) for k in ("waiting","admittedAlone","refused","fleet")))
edit(MEM+"/🧬️schema/🔣️.json",
'          "waiting": "Graph: wartet auf Speicherreserve ({used} von {budget} MB reserviert, {running} Projekte laufen)",\n',
'          "waiting": %s,\n          "admittedAlone": %s,\n          "refused": %s,\n          "fleet": %s,\n'%tuple(json.dumps(MSG["de"][k],ensure_ascii=False) for k in ("waiting","admittedAlone","refused","fleet")))
edit(MEM+"/🧬️schema/🔣️.json",
'          "learning": "graph: no memory peak learned for {kind} projects yet ({used} MB assumed, {budget} MB headroom), inferring one at a time",\n',
'          "learning": %s,\n'%json.dumps(MSG["en"]["learning"],ensure_ascii=False))
edit(MEM+"/🧬️schema/🔣️.json",
'          "learning": "Graph: für {kind}-Projekte ist noch kein Speicherbedarf gelernt ({used} MB angenommen, {budget} MB Reserve), Projekte werden einzeln abgeleitet",\n',
'          "learning": %s,\n'%json.dumps(MSG["de"]["learning"],ensure_ascii=False))

edit(MEM+"/🟨️.mjs",
'/** 🖥️ The memory of the host in megabytes as this process may use it',
'''/** ⛔️ The typed refusal of a graph read that can neither wait for headroom nor run: the project at the head needs more than one project is granted here. It carries the exit code 75 (temporary failure: try again later or let the fleet infer it) and what it needs and is granted.
 * @see ./🧬️schema/🔣️.json */
export class GraphMemoryRefusedErrorV1 extends Error{
 constructor(detail){
  super(detail.message);
  this.name="GraphMemoryRefusedErrorV1";this.code="BUILD_GRAPH_MEMORY_REFUSED";this.exitCode=P.refusalExitCode;this.detail=Object.freeze({key:detail.key,needBytes:detail.needBytes,availableBytes:detail.availableBytes,freeBytes:detail.freeBytes,seconds:detail.seconds});
 }
}

/** 🛰️ A project the fleet took over while it waited for headroom: the caller reads its rows from the fleet instead of inferring it here. */
export class GraphOwnerFleetErrorV1 extends Error{
 constructor(key){super("Graph project taken over by the fleet: "+key);this.name="GraphOwnerFleetErrorV1";this.code="BUILD_GRAPH_OWNER_FLEET";this.detail=Object.freeze({key});}
}

/** 🖥️ The memory of the host in megabytes as this process may use it''')
edit(MEM+"/🟨️.mjs","onStop,costs,estimateBytes=","onStop,costs,fleet,estimateBytes=")
edit(MEM+"/🟨️.mjs","lastCollect=-Infinity;\n",
"lastCollect=-Infinity,lastDone=-Infinity,fleetAsked=false;\n const grantBytes=ownerLimitBytes<Infinity?ownerLimitBytes:policy.grantMegabytes*MEGABYTE;\n")
edit(MEM+"/🟨️.mjs",
'''    const free=sample(),next=queue[0],known=learned.has(next.kind),estimate=peakOf(next.kind),state={headroom:free,running:owners.size,inflight:inflight(),estimate,learned:known};
    if(graphOwnerAdmissionV1(state)){''',
'''    const free=sample(),head=queue[0],waitedMilliseconds=now()-Math.max(head.since,lastAdmit,lastDone),shown=Math.max(0,free);
    let next=head,known=learned.has(next.kind),estimate=peakOf(next.kind),state={headroom:free,running:owners.size,inflight:inflight(),estimate,learned:known},forced=false;
    const describe=()=>({key:next.key??next.kind,kind:next.kind,seconds:Math.round((now()-Math.max(head.since,lastAdmit,lastDone))/1000),need:megabytes(estimate),limit:Math.round(policy.waitBoundMilliseconds/1000),available:megabytes(grantBytes),count:queue.length});
    if(!graphOwnerAdmissionV1(state)){
     if(fleet!==undefined&&!fleetAsked&&waitedMilliseconds>=policy.fleetFallbackMilliseconds){
      fleetAsked=true;const keys=queue.map(row=>row.key??row.kind);say("fleet",{...describe(),used:megabytes(estimate),budget:megabytes(shown)});
      let taken=[];try{taken=[...await Promise.race([Promise.resolve(fleet({keys,seconds:Math.round(waitedMilliseconds/1000)})),sleep(policy.fleetAskMilliseconds,signal).then(()=>[])])??[]];}catch(error){signal?.throwIfAborted();}
      for(const row of queue.splice(0)){if(taken.includes(row.key??row.kind))row.reject(new GraphOwnerFleetErrorV1(row.key??row.kind));else queue.push(row);}
      continue;
     }
     if(owners.size<=1&&waitedMilliseconds>=policy.waitBoundMilliseconds){
      next=queue.reduce((best,row)=>peakOf(row.kind)<peakOf(best.kind)?row:best,head);known=learned.has(next.kind);estimate=peakOf(next.kind);state={...state,estimate,learned:known};
      const fields={...describe(),used:megabytes(estimate),budget:megabytes(shown)};
      if(estimate>grantBytes){say("refused",fields);throw new GraphMemoryRefusedErrorV1({key:fields.key,needBytes:estimate,availableBytes:grantBytes,freeBytes:shown,seconds:fields.seconds,message:renderGraphMemoryMessagesV1("refused",{running:owners.size,...fields},environment).join("\\n")});}
      forced=true;say("admittedAlone",fields);
     }
    }
    if(forced||graphOwnerAdmissionV1(state)){''')
edit(MEM+"/🟨️.mjs","    sample();\n    if(key!==undefined)try{costs?.record","    sample();lastDone=now();\n    if(key!==undefined)try{costs?.record")
edit(MEM+"/🟨️.mjs","queue.shift();const owner={","queue.splice(queue.indexOf(next),1);const owner={")
edit(MEM+"/🟨️.mjs",
'''     if(!known)say("learning",{kind:next.kind,used:megabytes(estimate),budget:megabytes(free)});
     else say("waiting",{used:megabytes(state.inflight.reduce((sum,row)=>sum+Math.max(row.estimate,row.taken),0)),budget:megabytes(free+state.inflight.reduce((sum,row)=>sum+row.taken,0))});''',
'''     if(!known)say("learning",{...describe(),used:megabytes(estimate),budget:megabytes(shown)});
     else say("waiting",{...describe(),used:megabytes(state.inflight.reduce((sum,row)=>sum+Math.max(row.estimate,row.taken),0)),budget:megabytes(Math.max(0,free+state.inflight.reduce((sum,row)=>sum+row.taken,0)))});''')
edit(MEM+"/🟨️.mjs",
" * ONE project can be the problem: with `ownerLimitBytes`",
" * The wait is bounded: when at most one owner runs and the head of the queue has waited `waitBoundMilliseconds`, the smallest pending owner is admitted if its estimate fits the grant of one owner (`ownerLimitBytes`, else `grantMegabytes`), otherwise every waiter fails with {@link GraphMemoryRefusedErrorV1} (exit code 75); with a `fleet` port, after `fleetFallbackMilliseconds` the fleet is asked for the pending keys and the owners it takes fail with {@link GraphOwnerFleetErrorV1}.\n * ONE project can be the problem: with `ownerLimitBytes`")

edit(MEM+"/🟨️.d.mts","readonly limitEnvironment:string;readonly stopCollectMinimumMilliseconds:number}",
"readonly limitEnvironment:string;readonly stopCollectMinimumMilliseconds:number;readonly waitBoundMilliseconds:number;readonly fleetFallbackMilliseconds:number;readonly fleetAskMilliseconds:number;readonly grantMegabytes:number;readonly refusalExitCode:number}")
edit(MEM+"/🟨️.d.mts",'export type GraphMemoryMessageKeyV1="waiting"|"serial"|"resumed"|"learning"|"stopped";','export type GraphMemoryMessageKeyV1="waiting"|"serial"|"resumed"|"learning"|"stopped"|"admittedAlone"|"refused"|"fleet";')
edit(MEM+"/🟨️.d.mts","costs?:GraphOwnerCostPortV1;estimateBytes?:number;peaks?:GraphOwnerPeaksV1;","costs?:GraphOwnerCostPortV1;fleet?:GraphFleetPortV1;estimateBytes?:number;peaks?:GraphOwnerPeaksV1;")
edit(MEM+"/🟨️.d.mts",APPEND,'''/** 🛰️ Asks the fleet to infer the pending projects `keys`; answers the keys it takes over. */
export type GraphFleetPortV1=(ask:{keys:readonly string[];seconds:number})=>Promise<Iterable<string>|undefined>|Iterable<string>|undefined;
/** ⛔️ The typed refusal (exit code 75) of a graph read whose next project needs more than one project is granted. */
export class GraphMemoryRefusedErrorV1 extends Error{readonly code:"BUILD_GRAPH_MEMORY_REFUSED";readonly exitCode:75;readonly detail:Readonly<{key:string;needBytes:number;availableBytes:number;freeBytes:number;seconds:number}>;constructor(detail:{key:string;needBytes:number;availableBytes:number;freeBytes:number;seconds:number;message:string})}
/** 🛰️ A project the fleet took over while it waited for headroom. */
export class GraphOwnerFleetErrorV1 extends Error{readonly code:"BUILD_GRAPH_OWNER_FLEET";readonly detail:Readonly<{key:string}>;constructor(key:string)}''')

edit(HC+"/🧬️schema/🔣️.json",
'''        "operation": {
          "type": "string",
          "pattern": "^[a-z][a-z0-9-]*$"
        },
        "priority": {
          "enum": [
            "batch",
            "interactive"
          ]
        },''',
'''        "operation": {
          "type": "string",
          "pattern": "^[a-z][a-z0-9-]*$"
        },
        "priority": {
          "enum": [
            "batch",
            "interactive",
            "lent"
          ],
          "description": "The class of a request. `interactive` is served before `batch`; `lent` is work of another machine of the fleet running here: it is served after every request of this machine, never ages into a higher class, and is admitted only while no request of this machine waits, so a lent action takes only capacity the local user is not asking for."
        },''')
edit(HC+"/📏️decision/🟦️.ts",'export type HostRequestV1=Readonly<{id:string;operation:string;priority:"batch"|"interactive";','export type HostRequestV1=Readonly<{id:string;operation:string;priority:"batch"|"interactive"|"lent";')
edit(HC+"/📏️decision/🟦️.ts",'(waiter.request.priority==="interactive"?0:1),held=','(waiter.request.priority==="interactive"?0:waiter.request.priority==="lent"?4:1),held=')
edit(HC+"/📏️decision/🟦️.ts",'/** 🔢️ Orders waiters for service: interactive before batch (','/** 🔢️ Orders waiters for service: interactive before batch, and the lent class of another machine after everything (')
edit(HC+"/📏️decision/🟦️.ts",'let blocked=false,waits=false,admitted=0,','let blocked=false,waits=false,localWaits=false,admitted=0,')
edit(HC+"/📏️decision/🟦️.ts",'const refuse=(reason:HostReasonV1):void=>{if(!grows&&!elastic)waits=true;','const refuse=(reason:HostReasonV1):void=>{if(request.priority!=="lent")localWaits=true;if(!grows&&!elastic)waits=true;')
edit(HC+"/📏️decision/🟦️.ts",'  if(hard){refuse(hard);continue;}\n  if(grows){','  if(hard){refuse(hard);continue;}\n  if(request.priority==="lent"&&localWaits){refuse("queue-order");continue;}\n  if(grows){')
edit(HC+"/📐️estimates/🟦️.ts",'return base==="interactive"||standing==="interactive"||estimate.quick?"interactive":"batch";','return base==="lent"?"lent":base==="interactive"||standing==="interactive"||estimate.quick?"interactive":"batch";')

edit(HC+"/🧪️tests/🟦️.ts",'import {describeHostReasonV1,formatHostCapacityErrorV1,hostCapacityErrorV1,evaluateHostCapacityV1,','import {describeHostReasonV1,formatHostCapacityErrorV1,hostCapacityErrorV1,hostRequestPriorityV1,evaluateHostCapacityV1,')
edit(HC+"/🧪️tests/🟦️.ts",'priority:"batch"|"interactive"="batch"):HostWaiterV1=>','priority:HostWaiterV1["request"]["priority"]="batch"):HostWaiterV1=>')
edit(HC+"/🧪️tests/🟦️.ts",APPEND,'''
test("The lent class is never admitted while a request of this machine waits, although a plain batch request of the same size overtakes a head held back by foreign pressure",()=>{
 const sample=fair(10000),big=asker("big",5200,0);
 const control=evaluateHostCapacityV1(policy,sample,[],[big,asker("small",500,1000)],2000);
 expect(verdictOf(control,"big")).toMatchObject({verdict:"wait",reason:"memory"});
 expect(verdictOf(control,"small")).toMatchObject({verdict:"admit"});
 const lent=evaluateHostCapacityV1(policy,sample,[],[big,asker("small",500,1000,"lent")],2000);
 expect(verdictOf(lent,"big")).toMatchObject({verdict:"wait",reason:"memory"});
 expect(verdictOf(lent,"small")).toMatchObject({verdict:"wait",reason:"queue-order"});
 expect(verdictOf(evaluateHostCapacityV1(policy,sample,[],[asker("small",500,1000,"lent")],2000),"small")).toMatchObject({verdict:"admit"});
 expect(verdictOf(evaluateHostCapacityV1(policy,sample,[],[big,asker("small",500,0,"lent")],policy.wait.agingMilliseconds*10),"small")).toMatchObject({verdict:"wait",reason:"queue-order"});
});

test("A lent request is served after every other request, takes what they leave, and a request of this machine that is admitted does not hold it back",()=>{
 const sample=fair(10000),name=(row:HostWaiterV1)=>row.request.id.replace(/[xg]+$/u,"");
 expect(orderHostWaitersV1([asker("lent",100,0,"lent"),asker("batch",100,5),asker("inter",100,9,"interactive")],[]).map(name)).toEqual(["inter","batch","lent"]);
 const both=evaluateHostCapacityV1(policy,sample,[],[asker("local",800,5),asker("lent",800,0,"lent")],10);
 expect(verdictOf(both,"local")).toMatchObject({verdict:"admit"});expect(verdictOf(both,"lent")).toMatchObject({verdict:"admit"});
 const tight=evaluateHostCapacityV1(policy,fair(8000),[],[asker("local",800,5),asker("lent",800,0,"lent")],10);
 expect(verdictOf(tight,"local")).toMatchObject({verdict:"admit"});expect(verdictOf(tight,"lent")).toMatchObject({verdict:"wait"});
});

test("Property over 600 random queues: a lent request changes no other decision, is admitted only when no other request waits, and the same request as batch is admitted often while others wait",()=>{
 const random=seededRandomV1(20261115);let overtook=0,lentAdmitted=0;
 for(let index=0;index<600;index++){
  const base=randomHostCaseV1(random,index),own=(id:string)=>({request:{id,operation:"exec",priority:"lent" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:512+Math.floor(random()*4)*512,perJobMegabytes:0,cpuPerJob:1},arrivedAtMs:0,pid:1});
  const lent=own(`lent${index}xxxxxx`),batch={...lent,request:{...lent.request,priority:"batch" as const}};
  const without=evaluateHostCapacityV1(policy,base.host,base.ledger,base.queue,base.nowMs),withLent=evaluateHostCapacityV1(policy,base.host,base.ledger,[...base.queue,lent],base.nowMs);
  expect(withLent.decisions.filter(row=>row.id!==lent.request.id),String(index)).toEqual(without.decisions);
  const verdict=withLent.decisions.find(row=>row.id===lent.request.id)!,othersWait=without.decisions.some(row=>row.verdict==="wait");
  if(verdict.verdict==="admit"){lentAdmitted++;expect(othersWait,String(index)).toBe(false);}
  if(othersWait){expect(verdict.verdict,String(index)).toBe("wait");const control=evaluateHostCapacityV1(policy,base.host,base.ledger,[...base.queue,batch],base.nowMs).decisions.find(row=>row.id===lent.request.id)!;if(control.verdict==="admit")overtook++;}
 }
 expect(overtook,"the control: as batch the request would have overtaken a waiting one").toBeGreaterThan(10);expect(lentAdmitted).toBeGreaterThan(10);
});

test("The request schema names the lent class, Ajv agrees, and the class of a lent request never changes",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true}),check=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Request"}),request=fixture.cases[0]!.queue[0]!.request;
 for(const priority of ["batch","interactive","lent"]){expect(check({...request,priority}),priority).toBe(true);expect(isHostCapacityV1("Request",{...request,priority}),priority).toBe(true);}
 expect(check({...request,priority:"urgent"})).toBe(false);
 expect(hostRequestPriorityV1("lent","interactive",{quick:true} as never)).toBe("lent");
});
''')

edit(RE+"/🟦️.ts",'|"queue-order"|"ramp"|"quiet-limit"|"slots"|"disk";','|"queue-order"|"ramp"|"quiet-limit"|"slots"|"disk"|"not-ready";')
edit(RE+"/🌡️load/🟦️.ts",'as a batch request of one build job','as a request of the lent class (served after every request of this machine, admitted only while none of them waits) for one build job')
edit(RE+"/🌡️load/🟦️.ts",'request:{id:"remote-candidate",operation:"remote-execution",priority:"batch",','request:{id:"remote-candidate",operation:"remote-execution",priority:"lent",')
edit(RE+"/🌡️load/🔎️why/🟦️.ts",'|"policy-off"|"not-registered"|','|"policy-off"|"bootstrap-pending"|"not-registered"|')
edit(RE+"/🌡️load/🔎️why/🟦️.ts",' "not-registered":{en:',' "bootstrap-pending":{en:"this machine has not finished its native bootstrap ({detail}), so it lends no work to the fleet yet",de:"diese Maschine hat ihr natives Bootstrap noch nicht abgeschlossen ({detail}), sie leiht der Flotte daher noch keine Arbeit"},\n "not-registered":{en:')
edit(RE+"/🛰️agent/🩺️readiness/🟦️.ts",'import {loadFleetSignerV1}','import {existsSync} from "node:fs";\nimport {join} from "node:path";\nimport {loadFleetSignerV1}')
edit(RE+"/🛰️agent/🩺️readiness/🟦️.ts",'the policy runs actions, the machine has','the policy runs actions, the native bootstrap of the machine has finished (the file `readyMarker` of the policy exists), the machine has')
edit(RE+"/🛰️agent/🩺️readiness/🟦️.ts",' if(loadFleetSignerV1(root,"worker")===undefined)',' const marker=join(root,policy.readyMarker);\n if(!existsSync(marker))return remoteExecutionWorkerWhyV1("bootstrap-pending",marker);\n if(loadFleetSignerV1(root,"worker")===undefined)')
edit(RE+"/🛰️agent/🟦️.ts",'view?:()=>Promise<RemoteExecutionHostViewV1>;policy?:HostCapacityPolicyV1;now?:','view?:()=>Promise<RemoteExecutionHostViewV1>;ready?:()=>boolean;policy?:HostCapacityPolicyV1;now?:')
edit(RE+"/🛰️agent/🟦️.ts",'A confinement the policy requires but the host lacks stops the worker (and the agent) with an error. */','A confinement the policy requires but the host lacks stops the worker (and the agent) with an error. With `ready` a machine lends only while it answers true (its native bootstrap marker exists): otherwise every admission refuses with `not-ready` and every report says `bootstrap-pending` without headroom, so the scheduler places nothing there. */')
edit(RE+"/🛰️agent/🟦️.ts",'const admit=async(running:number):Promise<RemoteExecutionAdmissionV1>=>{\n','const admit=async(running:number):Promise<RemoteExecutionAdmissionV1>=>{\n  if(options.ready!==undefined&&!options.ready())return {admit:false,reason:"not-ready"};\n')
edit(RE+"/🛰️agent/🟦️.ts",'  return diskWhy!==undefined&&state.state==="refusing"?','  if(options.ready!==undefined&&!options.ready())return {...built,headroom:false,worker:{...built.worker,...built.worker.state==="idle"||built.worker.state==="busy"?{state:"refusing" as const}:{},why:remoteExecutionWorkerWhyV1("bootstrap-pending")}};\n  return diskWhy!==undefined&&state.state==="refusing"?')
edit(RE+"/🛰️sidecar/👷️agent/🟦️.ts",'import {availableParallelism,homedir} from "node:os";','import {existsSync} from "node:fs";\nimport {availableParallelism,homedir} from "node:os";')
edit(RE+"/🛰️sidecar/👷️agent/🟦️.ts",'const directory=join(context.root,".cache","f","remote-execution"),why=','const directory=join(context.root,".cache","f","remote-execution"),marker=join(context.root,policy.worker.readyMarker),why=')
edit(RE+"/🛰️sidecar/👷️agent/🟦️.ts",
'say(`joining ${scheduler}${why!==undefined?` without a worker: ${why.en}`:` offering rustc for ${rustc!.toolchain.targets.length} target(s)`}`);',
'say(`joining ${scheduler}${why!==undefined?` without a worker: ${why.en} / ohne Worker: ${why.de}`:` ready (native bootstrap done, marker ${marker}), offering rustc for ${rustc!.toolchain.targets.length} target(s) / bereit (natives Bootstrap abgeschlossen), bietet rustc für ${rustc!.toolchain.targets.length} Ziel(e) an`}`);')
edit(RE+"/🛰️sidecar/👷️agent/🟦️.ts",'worker:why===undefined,disk:policy.worker.disk,','worker:why===undefined,ready:()=>existsSync(marker),disk:policy.worker.disk,')
edit(B+"/🔣️policy.json",'      "actions": "rustc",\n      "disk": {','      "actions": "rustc",\n      "readyMarker": ".cache/f/native-bootstrap.done",\n      "disk": {')
edit(B+"/🏃️execution/🧬️schema/🌐️remote-execution.json",'        "actions",\n        "disk"\n      ],\n      "properties": {\n        "disk": {','        "actions",\n        "readyMarker",\n        "disk"\n      ],\n      "properties": {\n        "readyMarker": {\n          "type": "string",\n          "minLength": 1,\n          "maxLength": 256,\n          "description": "The file below the checkout whose existence says that the native bootstrap of the machine has finished (written by the bootstrap only after exit 0, removed when it starts again). A machine lends work to the fleet only while it exists."\n        },\n        "disk": {')
edit(B+"/🏃️execution/🧬️schema/🌐️remote-execution.json",'            "policy-off",\n            "not-registered",','            "policy-off",\n            "bootstrap-pending",\n            "not-registered",')
edit(B+"/🏃️execution/🧬️schema/🌐️remote-execution.json",'        "slots",\n        "disk"\n      ],\n      "description": "Why a worker\'s own admission refused an action."','        "slots",\n        "disk",\n        "not-ready"\n      ],\n      "description": "Why a worker\'s own admission refused an action."')

edit(RE+"/🧪️tests/🔎️why/🟦️.ts",'import {buildRemoteExecutionLoadReportV1,type','import {admitRemoteExecutionActionV1,buildRemoteExecutionLoadReportV1,type')
edit(RE+"/🧪️tests/🔎️why/🟦️.ts",'import {mkdtempSync,rmSync} from "node:fs";','import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";')
edit(RE+"/🧪️tests/🔎️why/🟦️.ts",'"disk-full","policy-off","not-registered","no-headroom","failed"];','"disk-full","policy-off","bootstrap-pending","not-registered","no-headroom","failed"];')
edit(RE+"/🧪️tests/🔎️why/🟦️.ts",
''' expect(check({actions:"none"})).toBe("policy-off");
 expect(check()).toBe("identity-unregistered");''',
''' expect(check({actions:"none"})).toBe("policy-off");
 expect(check()).toBe("bootstrap-pending");
 mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(join(root,".cache","f","native-bootstrap.done"),"2026-11-15T00:00:00Z\\n");
 expect(check()).toBe("identity-unregistered");''')
edit(RE+"/🧪️tests/🔎️why/🟦️.ts",APPEND,'''
test("a machine lends nothing while a request of its own waits, and lends again when nothing waits",()=>{
 const nowMs=Date.now(),local={request:{id:"localaaaa1",operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:65000,perJobMegabytes:0,cpuPerJob:1},arrivedAtMs:nowMs-1000,pid:1};
 expect(admitRemoteExecutionActionV1({view,running:0,concurrency:4,nowMs})).toEqual({admit:true});
 expect(admitRemoteExecutionActionV1({view:{...view,waiters:[local]} as never,running:0,concurrency:4,nowMs})).toEqual({admit:false,reason:"queue-order"});
 expect(admitRemoteExecutionActionV1({view:{...view,waiters:[]} as never,running:0,concurrency:4,nowMs})).toEqual({admit:true});
});

test("an agent whose machine has not finished its bootstrap refuses every action, reports no headroom and says why; once ready it reports headroom again",async()=>{
 const root=temporary(),fake=await fakeScheduler(403),controller=new AbortController();let ready=false;
 const agent=runRemoteExecutionAgentV1({...agentOptions(root,fake.url,controller.signal),ready:()=>ready});
 const waiting=await until(()=>fake.reports.find(row=>row.worker.why?.code==="bootstrap-pending"));
 expect(waiting.headroom).toBe(false);expect(waiting.worker.why!.en).toContain("native bootstrap");expect(waiting.worker.why!.de).toContain("Bootstrap");
 for(const row of fake.reports)requireRemoteExecutionShapeV1("LoadReport",row,"closed");
 ready=true;
 const lending=await until(()=>fake.reports.filter(row=>row.worker.why?.code!=="bootstrap-pending").pop());
 expect(lending.headroom).toBe(true);
 controller.abort();await agent.catch(()=>undefined);
});
''')

edit(RE+"/🛰️sidecar/🧪️tests/🟦️.ts",'"s-"));scratch.push(directory);\n const worktree=join(directory,"worktree"),member=join(directory,"member"),store=join(directory,"store");\n for(const [root,origin] of[[worktree,"g1-server"],[member,"g1-m09"]] as const){mkdirSync(join(root,".cache","f"),{recursive:true});','"s-"));scratch.push(directory);\n const worktree=join(directory,"worktree"),member=join(directory,"member"),store=join(directory,"store");\n for(const [root,origin] of[[worktree,"g1-server"],[member,"g1-m09"]] as const){mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(join(root,".cache","f","native-bootstrap.done"),"ok\\n");')

edit(MEM+"/🧪️tests/🟦️.ts",'for(const name of english)expect(["budget","used","running","kind","key"],key).toContain(name);','for(const name of english)expect(["budget","used","running","kind","key","need","seconds","limit","available","count"],key).toContain(name);')
edit(MEM+"/🧪️tests/🟦️.ts",'of Object.keys(moduleSchema.definitions.Messages.const.en) as ("waiting"|"serial"|"resumed"|"learning"|"stopped")[]','of Object.keys(moduleSchema.definitions.Messages.const.en) as ("waiting"|"serial"|"resumed"|"learning"|"stopped"|"admittedAlone"|"refused"|"fleet")[]')
edit(MEM+"/🧪️tests/🟦️.ts",APPEND,'''
/** 🧪️ Runs one owner that holds its slot for `holdMilliseconds` of modeled time and answers when it started. */
const holdOwner=(governor:ReturnType<typeof memory.createMemoryGovernorV1>,time:FakeTime,key:string,holdMilliseconds:number,started:Record<string,number>,kind="rust")=>governor.run(async()=>{started[key]=time.now();await time.sleep(holdMilliseconds);},{kind,key});
const learnedPeaks=(megabytes:number)=>({read:()=>({rust:[megabytes*MEGABYTE]}),write:()=>undefined});

test("with one project running and a headroom below its estimate the smallest pending project runs one bound after the last progress when it fits the grant, and every wait line names project, need and seconds",async()=>{
 const time=new FakeTime(),lines:string[]=[],started:Record<string,number>={},bound=memory.GRAPH_MEMORY_POLICY_V1.waitBoundMilliseconds;
 const governor=memory.createMemoryGovernorV1({workers:2,headroom:()=>413*MEGABYTE,peaks:learnedPeaks(465),now:time.now,sleep:time.sleep,report:line=>lines.push(`${time.now()} ${line}`),environment:{SEMIO_LANGUAGE:"en"}});
 const first=holdOwner(governor,time,"first",600000,started),second=holdOwner(governor,time,"second",1000,started);let firstDone=false;void first.then(()=>{firstDone=true;});
 await time.drive(()=>started.second!==undefined);
 expect(started.first).toBe(0);
 expect(started.second!,"never before the bound, and within one poll after it").toBeGreaterThanOrEqual(bound);expect(started.second!).toBeLessThan(bound+1000);
 const waiting=lines.filter(line=>line.includes("waiting for memory headroom"));
 expect(waiting.length).toBeGreaterThan(0);
 for(const line of waiting){expect(line).toContain("second needs 465 MB");expect(line).toMatch(/has waited \\d+ s/u);expect(line).toContain("after 60 s");expect(line).toMatch(/\\(\\d+ of \\d+ MB reserved/u);}
 expect(lines.some(line=>line.includes("second waited 60 s for 465 MB")&&line.includes("fits the 2048 MB"))).toBe(true);
 await time.drive(()=>firstDone);
 governor.dispose();
});

test("a headroom below the estimate with nothing running starts the project at once, and with one running the wait never exceeds the bound after the last progress",async()=>{
 const time=new FakeTime(),started:Record<string,number>={};
 const governor=memory.createMemoryGovernorV1({workers:2,headroom:()=>-81*MEGABYTE,peaks:learnedPeaks(486),now:time.now,sleep:time.sleep});
 const a=holdOwner(governor,time,"a",600000,started),b=holdOwner(governor,time,"b",5000,started),c=holdOwner(governor,time,"c",5000,started);
 const bound=memory.GRAPH_MEMORY_POLICY_V1.waitBoundMilliseconds;
 let finished=0;void b.then(()=>{finished++;});void c.then(()=>{finished++;});
 await time.drive(()=>finished===2);
 expect(started.a).toBe(0);expect(started.b!).toBeGreaterThanOrEqual(bound);expect(started.b!).toBeLessThan(bound+1000);
 expect(started.c!,"every admission and completion is progress and restarts the bound, so c runs one bound after b is done").toBeGreaterThanOrEqual(started.b!+5000);expect(started.c!).toBeLessThan(started.b!+5000+bound+1000);
 void a;
 governor.dispose();
});

test("a project that needs more than one project is granted is refused with the typed error naming need and available, exit code 75, within the bound",async()=>{
 const time=new FakeTime(),started:Record<string,number>={},lines:string[]=[];
 const governor=memory.createMemoryGovernorV1({workers:2,headroom:()=>300*MEGABYTE,ownerLimitBytes:400*MEGABYTE,peaks:learnedPeaks(900),now:time.now,sleep:time.sleep,report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en"}});
 holdOwner(governor,time,"first",600000,started).catch(()=>undefined);
 const second=holdOwner(governor,time,"second",1000,started).then(()=>undefined,error=>error as Error&{code:string;exitCode:number;detail:{needBytes:number;availableBytes:number}});
 let outcome:unknown;void second.then(value=>{outcome=value;});
 await time.drive(()=>outcome!==undefined);
 expect(outcome).toBeInstanceOf(memory.GraphMemoryRefusedErrorV1);
 expect(outcome).toMatchObject({code:"BUILD_GRAPH_MEMORY_REFUSED",exitCode:75,detail:{needBytes:900*MEGABYTE,availableBytes:400*MEGABYTE}});
 expect((outcome as Error).message).toContain("second needs 900 MB but one project is granted 400 MB");expect((outcome as Error).message).toContain("exit 75");
 expect(time.now()).toBeLessThanOrEqual(memory.GRAPH_MEMORY_POLICY_V1.waitBoundMilliseconds+1000);
 expect(started.second).toBeUndefined();
 governor.dispose();
});

test("an owner of an unlearned kind waits for the running owner only up to the bound and its line says for how long",async()=>{
 const time=new FakeTime(),started:Record<string,number>={},lines:string[]=[];
 const governor=memory.createMemoryGovernorV1({workers:2,headroom:()=>10*1024*MEGABYTE,now:time.now,sleep:time.sleep,report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en"}});
 const first=holdOwner(governor,time,"first",600000,started,"cold"),second=holdOwner(governor,time,"second",1000,started,"cold");
 await time.drive(()=>started.second!==undefined);
 expect(started.second!).toBeGreaterThanOrEqual(memory.GRAPH_MEMORY_POLICY_V1.waitBoundMilliseconds);
 expect(lines.some(line=>line.includes("no memory peak learned for cold projects")&&/second has waited \\d+ s for the running project, at most 60 s/u.test(line))).toBe(true);
 governor.dispose();void first;
});

test("with two projects running the fleet is asked for the pending projects after the fleet bound, the owners it takes fail typed, once, and without a port the governor keeps waiting",async()=>{
 const time=new FakeTime(),started:Record<string,number>={},asked:{keys:readonly string[];seconds:number}[]=[],lines:string[]=[];
 const governor=memory.createMemoryGovernorV1({workers:3,headroom:()=>100*MEGABYTE,peaks:learnedPeaks(465),now:time.now,sleep:time.sleep,report:line=>lines.push(line),environment:{SEMIO_LANGUAGE:"en"},fleet:async ask=>{asked.push(ask);return ask.keys.filter(key=>key!=="keep");}});
 const hold=(key:string)=>holdOwner(governor,time,key,1_000_000,started).catch(error=>error as Error&{code:string});
 hold("one");hold("two");
 const taken=hold("third"),kept=hold("keep");let outcome:unknown,done=false;void taken.then(value=>{outcome=value;done=true;});
 await time.drive(()=>done);
 expect(asked.length).toBe(1);expect(asked[0]!.keys).toEqual(["third","keep"]);expect(asked[0]!.seconds).toBeGreaterThanOrEqual(memory.GRAPH_MEMORY_POLICY_V1.fleetFallbackMilliseconds/1000);
 expect(outcome).toBeInstanceOf(memory.GraphOwnerFleetErrorV1);expect(outcome).toMatchObject({code:"BUILD_GRAPH_OWNER_FLEET",detail:{key:"third"}});
 expect(lines.some(line=>line.includes("asking the fleet to infer the remaining 2 project(s)"))).toBe(true);
 expect(started.keep).toBeUndefined();
 governor.dispose();void kept;
});
''')

TRANSFORMS={
 MEM+"/🧪️tests/🔣️.json":fixture_json,
 MEM+"/🧪️tests/🧬️schema/🔣️.json":fixture_schema_json,
}
TESTS=[
 HC+"/🧪️tests/🟦️.ts",
 MEM+"/🧪️tests/🟦️.ts",
 RE+"/🧪️tests/🔎️why/🟦️.ts",
 RE+"/🛰️sidecar/🧪️tests/🟦️.ts",
]
