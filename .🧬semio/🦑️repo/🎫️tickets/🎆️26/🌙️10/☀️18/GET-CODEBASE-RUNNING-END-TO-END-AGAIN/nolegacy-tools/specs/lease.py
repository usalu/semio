"""🔒️ Batch `nl-lease`: one rule for every lease holder. The `legacy` holder state (a registry entry of older code without a standing, reclaimed after 4 h by its ticket age) is gone: a registry entry carries a standing or it is not read, and a holder whose standing cannot be read is `unknown` and never decided against. Schema, fixtures, Python oracle, status, fleet status, queue notices and docs (EN+DE) follow."""
import json

TITLE = "🔒️ Staged batch `nl-lease` (lane nolegacy): the lease holder state `legacy` and its 4 h reclaim are removed; a registry entry without a standing is not read, a holder without a readable standing is `unknown` and never stopped; schema, corpus, Python oracle, status, fleet status, queue notices and docs EN+DE follow."
P = "🧰️framework/🔨️modules/🏃️process/"
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
R = "🧰️framework/🛍️products/🦑️repo/"
UNKNOWN_EN, UNKNOWN_DE = "standing unknown: no live registry entry", "Zustand unbekannt: kein lebender Registereintrag"


def dump(doc, indent):
    return json.dumps(doc, indent=indent, ensure_ascii=False) + "\n"


def rename_key(row, old, new, value=None):
    return {(new if key == old else key): ((value if value is not None else val) if key == old else val) for key, val in row.items()}


def enum_unknown(node):
    if isinstance(node, dict):
        if isinstance(node.get("enum"), list) and "legacy" in node["enum"] and "stale" in node["enum"]:
            node["enum"] = ["unknown" if value == "legacy" else value for value in node["enum"]]
        for value in node.values(): enum_unknown(value)
    elif isinstance(node, list):
        for value in node: enum_unknown(value)
    return node


def standing_schema(text):
    doc = json.loads(text)
    defs = doc["definitions"]
    hits = []
    def strip(node):
        if isinstance(node, dict):
            if "legacyAfterMilliseconds" in node.get("properties", {}):
                del node["properties"]["legacyAfterMilliseconds"]; node["required"].remove("legacyAfterMilliseconds"); hits.append("schema")
            if "legacyAfterMilliseconds" in node and isinstance(node["legacyAfterMilliseconds"], int):
                del node["legacyAfterMilliseconds"]; hits.append("const")
            for value in node.values(): strip(value)
        elif isinstance(node, list):
            for value in node: strip(value)
    strip(doc)
    assert sorted(hits) == ["const", "schema"], hits
    messages = defs["Messages"]["const"]
    messages = rename_key(messages, "state.legacy", "state.unknown", {"en": UNKNOWN_EN, "de": UNKNOWN_DE})
    messages = rename_key(messages, "status.legacy", "status.unknown")
    defs["Messages"]["const"] = messages
    for key in ("state.unknown", "status.unknown"):
        assert key in json.dumps(defs["Messages"]), key
    if "properties" in defs["Messages"]:
        defs["Messages"] = enum_unknown(defs["Messages"])
    item = defs["Corpus"]["properties"]["terminateCases"]["items"]["properties"]["input"]
    for field in ("ticketArrivalMs", "holdSinceMs"):
        item["required"].remove(field); del item["properties"][field]
    enum_unknown(doc)
    out = dump(doc, 1)
    assert "legacy" not in out and "ticketArrivalMs" not in out, "legacy left in the standing schema"
    return out


def standing_fixture(text):
    doc = json.loads(text)
    kept = []
    for case in doc["terminateCases"]:
        if case["input"]["state"] == "legacy": continue
        for field in ("ticketArrivalMs", "holdSinceMs"): case["input"].pop(field, None)
        kept.append(case)
    assert len(kept) == len(doc["terminateCases"]) - 4
    kept.append({"id": "unknown-holder-is-never-stopped", "input": {"mode": "terminate", "state": "unknown", "nowMs": 1700004460100, "requestAtMs": 1700004000000, "progressAtMs": 1699990001000, "registeredStartedAtMs": 1699990000000, "osStartedAtMs": 1699990000000, "signalable": True, "self": False}, "expected": {"terminate": False}})
    kept.append({"id": "silent-holder-is-never-stopped", "input": {"mode": "terminate", "state": "silent", "nowMs": 1700004460100, "requestAtMs": 1700004000000, "progressAtMs": 1699990001000, "registeredStartedAtMs": 1699990000000, "osStartedAtMs": 1699990000000, "signalable": True, "self": False}, "expected": {"terminate": False}})
    doc["terminateCases"] = kept
    for case in doc["stateCases"]:
        if case["id"] == "legacy":
            case["id"] = "unknown"; case["expected"]["state"] = "unknown"
    out = dump(doc, 1)
    assert "legacy" not in out and "ticketArrivalMs" not in out
    return out


MODEL_OLD = '''def terminate(case):
    i=case["input"];rules=policy["reclaim"]
    if i["mode"]!="terminate" or i["self"]:return {"terminate":False}
    legacy=i["state"]=="legacy"
    if not (i["state"]=="stuck" or (legacy and i["nowMs"]-i["holdSinceMs"]>=rules["legacyAfterMilliseconds"])):return {"terminate":False}
    if not legacy and ("requestAtMs" not in i or i["nowMs"]-i["requestAtMs"]<rules["graceMilliseconds"]):return {"terminate":False,"refusal":"grace"}
    if not legacy and i["progressAtMs"]>i["requestAtMs"]:return {"terminate":False,"refusal":"progressed"}
    if "osStartedAtMs" not in i:return {"terminate":False,"refusal":"unknown"}
    tolerance=rules["identityToleranceMilliseconds"]
    bad=i["osStartedAtMs"]>i["ticketArrivalMs"]+tolerance if legacy else ("registeredStartedAtMs" not in i or abs(i["osStartedAtMs"]-i["registeredStartedAtMs"])>tolerance)
    if bad:return {"terminate":False,"refusal":"identity"}
'''
MODEL_NEW = '''def terminate(case):
    i=case["input"];rules=policy["reclaim"]
    if i["mode"]!="terminate" or i["self"] or i["state"]!="stuck":return {"terminate":False}
    if "requestAtMs" not in i or i["nowMs"]-i["requestAtMs"]<rules["graceMilliseconds"]:return {"terminate":False,"refusal":"grace"}
    if i["progressAtMs"]>i["requestAtMs"]:return {"terminate":False,"refusal":"progressed"}
    if "osStartedAtMs" not in i:return {"terminate":False,"refusal":"unknown"}
    if "registeredStartedAtMs" not in i or abs(i["osStartedAtMs"]-i["registeredStartedAtMs"])>rules["identityToleranceMilliseconds"]:return {"terminate":False,"refusal":"identity"}
'''

YIELD_DECISION_OLD = '''/** 🔪️ What the decision to stop a holder weighs: the mode, the holder's state, the request time, progress since, both start times and whether the process may be signalled. */
export type LeaseTerminateInputV1=Readonly<{mode:LeaseReclaimModeV1;state:LeaseHolderStateV1["state"];nowMs:number;requestAtMs?:number;progressAtMs:number;registeredStartedAtMs?:number;osStartedAtMs?:number;ticketArrivalMs:number;holdSinceMs:number;signalable:boolean;self:boolean}>;
/** 🧯️ Whether a waiter may stop a holder that ignored the yield request. Only in `terminate` mode, only for a stuck holder (or a legacy one that has HELD the resource longer than the declared time, counted from its acquisition and never from its ticket, which includes the time it queued), only after the grace period since the request, never when the holder made progress since, never without a verified identity (the operating system's start time equals the registered one; for a legacy holder it precedes the ticket), never another user's process or itself. */
export function leaseTerminateDecisionV1(input:LeaseTerminateInputV1):Readonly<{terminate:boolean;refusal?:"identity"|"progressed"|"owner"|"unknown"|"grace"}>{
 if(input.mode!=="terminate"||input.self)return{terminate:false};
 const legacy=input.state==="legacy";
 if(!(input.state==="stuck"||legacy&&input.nowMs-input.holdSinceMs>=policy.reclaim.legacyAfterMilliseconds))return{terminate:false};
 if(!legacy&&(input.requestAtMs===undefined||input.nowMs-input.requestAtMs<policy.reclaim.graceMilliseconds))return{terminate:false,refusal:"grace"};
 if(!legacy&&input.progressAtMs>input.requestAtMs!)return{terminate:false,refusal:"progressed"};
 if(input.osStartedAtMs===undefined)return{terminate:false,refusal:"unknown"};
 const tolerance=policy.reclaim.identityToleranceMilliseconds;
 if(legacy?input.osStartedAtMs>input.ticketArrivalMs+tolerance:input.registeredStartedAtMs===undefined||Math.abs(input.osStartedAtMs-input.registeredStartedAtMs)>tolerance)return{terminate:false,refusal:"identity"};
'''
YIELD_DECISION_NEW = '''/** 🔪️ What the decision to stop a holder weighs: the mode, the holder's state, the request time, progress since, both start times and whether the process may be signalled. */
export type LeaseTerminateInputV1=Readonly<{mode:LeaseReclaimModeV1;state:LeaseHolderStateV1["state"];nowMs:number;requestAtMs?:number;progressAtMs:number;registeredStartedAtMs?:number;osStartedAtMs?:number;signalable:boolean;self:boolean}>;
/** 🧯️ Whether a waiter may stop a holder that ignored the yield request. Only in `terminate` mode, only for a stuck holder (its own standing says so, aged from its acquisition), only after the grace period since the request, never when the holder made progress since, never without a verified identity (the operating system's start time equals the registered one), never another user's process or itself; a holder whose standing cannot be read is never stopped. */
export function leaseTerminateDecisionV1(input:LeaseTerminateInputV1):Readonly<{terminate:boolean;refusal?:"identity"|"progressed"|"owner"|"unknown"|"grace"}>{
 if(input.mode!=="terminate"||input.self||input.state!=="stuck")return{terminate:false};
 if(input.requestAtMs===undefined||input.nowMs-input.requestAtMs<policy.reclaim.graceMilliseconds)return{terminate:false,refusal:"grace"};
 if(input.progressAtMs>input.requestAtMs)return{terminate:false,refusal:"progressed"};
 if(input.osStartedAtMs===undefined)return{terminate:false,refusal:"unknown"};
 if(input.registeredStartedAtMs===undefined||Math.abs(input.osStartedAtMs-input.registeredStartedAtMs)>policy.reclaim.identityToleranceMilliseconds)return{terminate:false,refusal:"identity"};
'''

TEST_ENTRY_OLD = '''const entryFile=(registry:string,directory:string,pid:number,startedAtMs:number,sinceMs:number,resource:string,now:number)=>{
 const folder=leaseRegistryFolderV1(registry,directory);mkdirSync(folder,{recursive:true});
 writeFileSync(`${folder}/${pid}-${startedAtMs}.json`,JSON.stringify({version:1,pid,startedAtMs,updatedAtMs:now,command:"./script.ts repo run x",directory,holds:[{resource,mode:"exclusive",sinceMs,task:1}],waits:[]}));
};'''
TEST_ENTRY_NEW = '''const entryFile=(registry:string,directory:string,pid:number,startedAtMs:number,sinceMs:number,resource:string,now:number,standing?:Readonly<Record<string,unknown>>)=>{
 const folder=leaseRegistryFolderV1(registry,directory);mkdirSync(folder,{recursive:true});
 writeFileSync(`${folder}/${pid}-${startedAtMs}.json`,JSON.stringify({version:1,pid,startedAtMs,updatedAtMs:now,command:"./script.ts repo run x",directory,...standing?{standing}:{},holds:[{resource,mode:"exclusive",sinceMs,task:1}],waits:[]}));
};'''
TEST_LAW_OLD = '''test("Law: terminate mode ages a holder from its acquisition, not from its ticket, and stops it with its tree: children first named by their start time, a recycled pid or a changed head identity is never signalled, and a refusal is said once",async()=>{'''
TEST_LAW_NEW = '''test("Law: a registry entry without a standing is not read, so its holder is unknown and never stopped however long it holds",async()=>{
 const registry=mkdtempSync(join(tmpdir(),"lease-nostanding-")),now=Date.now(),resource="repository-graph-facts:/w:shard:7",directory="/virtual/leases",head=process.ppid,startedAtMs=now-9*hours,calls:[number,string|number][]=[];
 try{
  entryFile(registry,directory,head,startedAtMs,now-8*hours,resource,now);
  expect(readLeaseRegistryEntriesV1(registry,directory,now,false)).toEqual([]);
  const ports={alive:()=>true,signal:(pid:number,signal:"SIGTERM"|"SIGKILL"|0)=>{calls.push([pid,signal]);return true;},osStartedAtMs:()=>startedAtMs,sleep:async()=>{},write:()=>{}};
  const notice=await observeLeaseHolderV1({directory,resource,headPid:head,headMode:"exclusive",headArrivalMs:now-9*hours,nowMs:now,environment:{SEMIO_LEASE_RECLAIM:"terminate",SEMIO_LANGUAGE:"en"},root:registry,ports});
  expect(notice?.holderState).toBe("unknown");expect(calls.filter(([,signal])=>signal!==0)).toEqual([]);
 }finally{rmSync(registry,{recursive:true,force:true});}
});
test("Law: terminate mode ages a holder from its acquisition, not from its ticket, and stops it with its tree: children first named by their start time, a recycled pid or a changed head identity is never signalled, and a refusal is said once",async()=>{'''
TEST_OBSERVE_OLD = '''  const observe=(ports:ReturnType<typeof make>["ports"],arrivalMs:number,holdSinceMs:number)=>{entryFile(registry,directory,head,startedAtMs,holdSinceMs,resource,now);return observeLeaseHolderV1({directory,resource,headPid:head,headMode:"exclusive",headArrivalMs:arrivalMs,nowMs:now,environment,root:registry,ports});};
  const young=make();
  await observe(young.ports,now-7*hours,now-30*60000);
  expect(young.calls,"queued 6 h, held 30 min (the process is older than its ticket): the 90 minutes of before would have stopped it").toEqual([]);'''
TEST_OBSERVE_NEW = '''  const observe=(ports:ReturnType<typeof make>["ports"],arrivalMs:number,holdSinceMs:number)=>{
   entryFile(registry,directory,head,startedAtMs,holdSinceMs,resource,now,{version:1,priority:"agent",implementation:{digest:"0123456789abcdef",loadedAtMs:startedAtMs,components:{}},progressAtMs:holdSinceMs});
   writeLeaseYieldRequestV1(registry,directory,{version:1,holderPid:head,holderStartedAtMs:startedAtMs,requesterPid:process.pid,requesterStartedAtMs:leaseProcessStartedAtMsV1,resource,reason:"stuck",atMs:now-3*60000,requesterComponents:{}});
   return observeLeaseHolderV1({directory,resource,headPid:head,headMode:"exclusive",headArrivalMs:arrivalMs,nowMs:now,environment,root:registry,ports});
  };
  const young=make();
  await observe(young.ports,now-7*hours,now-10*60000);
  expect(young.calls,"queued 7 h, held 10 min: a holder ages from its acquisition, so it is not stuck and never stopped").toEqual([]);'''

EDITS = {
    P + "🔒️leases/📈️standing/🟦️.ts": [
        ("/** 🚦️ The judgement of a holder: legacy, progressing, silent, stuck or stale, with the silent time or the ids of the moved components. */", "/** 🚦️ The judgement of a holder: progressing, silent, stuck or stale by its standing, unknown when no standing of it can be read, with the silent time or the ids of the moved components. */", 1),
        ('state:"legacy"|"progressing"|"silent"|"stuck"|"stale";silentMs?:number', 'state:"unknown"|"progressing"|"silent"|"stuck"|"stale";silentMs?:number', 1),
        ("stuck or silent when nothing was reported for the declared time, legacy when it publishes no standing at all. Stale wins over stuck wins over silent. */", "stuck or silent when nothing was reported for the declared time, unknown when no standing of it can be read (no live registry entry), so nothing is ever decided against it. Stale wins over stuck wins over silent. */", 1),
        (' if(!holder)return Object.freeze({state:"legacy"});', ' if(!holder)return Object.freeze({state:"unknown"});', 1),
    ],
    P + "🔒️leases/📈️standing/🤝️yield/🟦️.ts": [
        (YIELD_DECISION_OLD, YIELD_DECISION_NEW, 1),
        ('candidate=mode==="terminate"&&(judged.state==="stuck"||judged.state==="legacy"&&entry!==undefined),', 'candidate=mode==="terminate"&&judged.state==="stuck",', 1),
        ("...osStartedAtMs===undefined?{}:{osStartedAtMs},ticketArrivalMs:input.headArrivalMs,holdSinceMs,signalable:", "...osStartedAtMs===undefined?{}:{osStartedAtMs},signalable:", 1),
    ],
    P + "🔒️leases/📈️standing/🧬️schema/🔣️.json": [standing_schema],
    P + "🔒️leases/📈️standing/🧪️tests/🧫️fixtures/🔣️.json": [standing_fixture],
    P + "🔒️leases/📈️standing/🧪️tests/🐍️model.py": [
        ('    if holder is None:return {"state":"legacy"}', '    if holder is None:return {"state":"unknown"}', 1),
        (MODEL_OLD, MODEL_NEW, 1),
    ],
    P + "🔒️leases/📈️standing/🧪️tests/🟦️.ts": [
        ("import {leaseElapsedSecondsV1,observeLeaseHolderV1,leaseTerminateDecisionV1,", "import {leaseElapsedSecondsV1,observeLeaseHolderV1,writeLeaseYieldRequestV1,leaseTerminateDecisionV1,", 1),
        ('import {leaseRegistryFolderV1} from "../../🧭️order/🗂️registry/🟦️.ts";', 'import {leaseProcessStartedAtMsV1,leaseRegistryFolderV1,readLeaseRegistryEntriesV1} from "../../🧭️order/🗂️registry/🟦️.ts";', 1),
        ('standingRow?:ReturnType<typeof standing>)=>({version:1 as const,pid,startedAtMs:now-9000000,updatedAtMs:now,command:"./script.ts repo run x",directory:"/w/lease",...standingRow?{standing:standingRow}:{},holds,waits:[]});', 'standingRow:ReturnType<typeof standing>)=>({version:1 as const,pid,startedAtMs:now-9000000,updatedAtMs:now,command:"./script.ts repo run x",directory:"/w/lease",standing:standingRow,holds,waits:[]});', 1),
        ('expect(holder(400).standing).toEqual({state:"legacy"});', 'expect(holder(400).standing).toEqual({state:"unknown"});', 1),
        ('"STUCK: no progress for 100 min","no standing published"]', '"STUCK: no progress for 100 min","%s"]' % UNKNOWN_EN, 1),
        ('"FESTGEFAHREN: seit 100 min kein Fortschritt","kein Zustand veröffentlicht"]', '"FESTGEFAHREN: seit 100 min kein Fortschritt","%s"]' % UNKNOWN_DE, 1),
        ('test("a head holder without any registry entry is unknown, not legacy: a pruned heartbeat never gets a healthy holder stopped"', 'test("a head holder without any registry entry is unknown: a pruned heartbeat never gets a healthy holder stopped"', 1),
        ('  expect(notice?.holderState).toBe("legacy");', '  expect(notice?.holderState).toBe("unknown");', 1),
        (TEST_ENTRY_OLD, TEST_ENTRY_NEW, 1),
        (TEST_LAW_OLD, TEST_LAW_NEW, 1),
        (TEST_OBSERVE_OLD, TEST_OBSERVE_NEW, 1),
    ],
    P + "🔒️leases/🧭️order/🗂️registry/🟦️.ts": [
        ("session?:string;standing?:LeaseStandingV1;holds:", "session?:string;standing:LeaseStandingV1;holds:", 1),
        ('typeof row.updatedAtMs==="number"&&(row.standing===undefined||row.standing.version===1);', 'typeof row.updatedAtMs==="number"&&row.standing?.version===1;', 1),
        ("/** 📜️ Appends one finished exclusive hold to the history of its kind", "/** 🖊️ Appends one finished exclusive hold to the history of its kind", 1),
        ("/** 📖️ The recent hold durations of one kind, oldest first. */", "/** ⏳️ The recent hold durations of one kind, oldest first. */", 1),
    ],
    P + "🔒️leases/📋️status/🟦️.ts": [
        ('export type LeaseHolderStandingV1=Readonly<{state:"legacy"|"progressing"', 'export type LeaseHolderStandingV1=Readonly<{state:"unknown"|"progressing"', 1),
        ('  if(!standing||!entry)return{state:"legacy"};', '  if(!standing||!entry)return{state:"unknown"};', 1),
        ('  if(standing.state==="legacy")return say("status.legacy",{state:say("state.legacy")});', '  if(standing.state==="unknown")return say("status.unknown",{state:say("state.unknown")});', 1),
    ],
    P + "🔒️leases/📋️status/🧬️schema/🔣️.json": [('            "legacy",\n            "progressing",', '            "unknown",\n            "progressing",', 1)],
    P + "🔒️leases/🧪️tests/📋️status/🟦️.ts": [
        ('const entry=(pid:number)=>({version:1 as const,pid,startedAtMs:0,updatedAtMs:0,command:"c"+pid,directory:"/d",holds:', 'const entry=(pid:number)=>({version:1 as const,pid,startedAtMs:0,updatedAtMs:0,command:"c"+pid,directory:"/d",standing:{version:1 as const,priority:"agent" as const,implementation:{digest:"0123456789abcdef",loadedAtMs:0,components:{}},progressAtMs:0},holds:', 1),
    ],
    P + "🔒️leases/🧭️order/🧪️tests/🕸️wait-for/🟦️.ts": [
        ('entry={version:1 as const,pid:4242,startedAtMs:1,updatedAtMs:2,command:"x",directory,holds:', 'entry={version:1 as const,pid:4242,startedAtMs:1,updatedAtMs:2,command:"x",directory,standing:{version:1 as const,priority:"agent" as const,implementation:{digest:"0123456789abcdef",loadedAtMs:1,components:{}},progressAtMs:2},holds:', 1),
    ],
    P + "🪪️agent-sessions/🧪️tests/🔗️adapters/🟦️.ts": [
        ('entries:[{version:1,pid:process.pid,startedAtMs:1,updatedAtMs:Date.now(),command:"x",directory:setup.leases,holds:', 'entries:[{version:1,pid:process.pid,startedAtMs:1,updatedAtMs:Date.now(),command:"x",directory:setup.leases,standing:{version:1,priority:"agent",implementation:{digest:"0123456789abcdef",loadedAtMs:1,components:{}},progressAtMs:1},holds:', 1),
    ],
    P + "🔒️leases/⚖️fairness/🧪️tests/🟦️.ts": [('expect(["progressing","silent","legacy"]).toContain(holderState);', 'expect(["progressing","silent"]).toContain(holderState);', 1)],
    P + "🧭️routing/📥️invocation/🏃️process/🟦️.ts": [('holderState?:"progressing"|"silent"|"stuck"|"stale"|"legacy";', 'holderState?:"progressing"|"silent"|"stuck"|"stale"|"unknown";', 1)],
    P + "🧭️routing/📥️invocation/🏃️process/🧬️schema/📈️progress.json": [
        ('            "legacy": "publishes no progress"', '            "unknown": "unknown (no live registry entry)"', 1),
        ('            "legacy": "meldet keinen Fortschritt"', '            "unknown": "unbekannt (kein lebender Registereintrag)"', 1),
        ('            "stale",\n            "legacy"\n', '            "stale",\n            "unknown"\n', 1),
    ],
    P + "🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🔣️.json": [
        ('"name": "german-holder-legacy"', '"name": "german-holder-unknown"', 1),
        ('\\"holderState\\":\\"legacy\\"}', '\\"holderState\\":\\"unknown\\"}', 1),
        ("· Halter: meldet keinen Fortschritt", "· Halter: unbekannt (kein lebender Registereintrag)", 1),
    ],
    P + "🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts": [
        ("IN ('progressing','silent','stuck','stale','legacy')", "IN ('progressing','silent','stuck','stale','unknown')", 1),
        ("const known=['progressing','silent','stuck','stale','legacy']", "const known=['progressing','silent','stuck','stale','unknown']", 1),
    ],
    L + "🏗️build/🚢️fleet-status/🟦️.ts": [
        ('export type FleetHolderStateV1="legacy"|"progressing"', 'export type FleetHolderStateV1="unknown"|"progressing"', 1),
        ('state:builder.standing?.state??"legacy"', 'state:builder.standing?.state??"unknown"', 1),
        ("if(!row)throw Error(`Unknown fleet text ${key}`);", 'if(!row)throw Error("fleet-text-undeclared:"+key);', 1),
        ("/** ✅️ Checks facts against their closed schema; the empty list means valid. */", "/** 🔬️ Checks facts against their closed schema; the empty list means valid. */", 1),
    ],
    L + "🏗️build/🚢️fleet-status/🧬️schema/🔣️.json": [lambda text: (lambda doc: dump(enum_unknown(doc), 1))(json.loads(text))],
    L + "🏗️build/🚢️fleet-status/🧪️tests/🐍️.py": [
        ('"state":s.get("state","legacy")', '"state":s.get("state","unknown")', 1),
        ('rnd.choice(["progressing","progressing","silent","stuck","stale","legacy"])', 'rnd.choice(["progressing","progressing","silent","stuck","stale","unknown"])', 1),
        ("        # keep per resource at most one exclusive holder shape irrelevant; the aggregation is defined for any input\n", "", 1),
    ],
    L + "🏗️build/🩺️doctor/🧪️tests/🔎️probes/🟦️.ts": [
        ('const entry=(pid:number,holds:unknown[],waits:unknown[],updated=now)=>({version:1,pid,startedAtMs:1000+pid,updatedAtMs:updated,command:"/w/script.ts repo cargo test",directory,holds,waits});', 'const entry=(pid:number,holds:unknown[],waits:unknown[],updated=now)=>({version:1,pid,startedAtMs:1000+pid,updatedAtMs:updated,command:"/w/script.ts repo cargo test",directory,standing:{version:1,priority:"agent",implementation:{digest:"0123456789abcdef",loadedAtMs:1000+pid,components:{}},progressAtMs:updated},holds,waits});', 1),
        ('JSON.stringify({version:1,pid:gone,startedAtMs:1,updatedAtMs:Date.now(),command:"x",directory,holds:[],waits:[]})', 'JSON.stringify({version:1,pid:gone,startedAtMs:1,updatedAtMs:Date.now(),command:"x",directory,standing:{version:1,priority:"agent",implementation:{digest:"0123456789abcdef",loadedAtMs:1,components:{}},progressAtMs:1},holds:[],waits:[]})', 1),
    ],
    R + "README.md": [
        ("or `legacy` (older code that publishes no standing), with its code age", "or `unknown` (no live registry entry carries its standing; nothing is ever decided against it), with its code age", 1),
        (" only after the operating system's start time of the pid equals the registered one (or, for a legacy holder that has HELD the resource for more than 4 h, counted from its acquisition and not from its ticket, which includes its queue wait, precedes its ticket), only for", " only after the operating system's start time of the pid equals the registered one, only for", 1),
        ("oder `legacy` (älterer Code, der keinen Zustand veröffentlicht), mit Code-Alter", "oder `unknown` (kein lebender Registry-Eintrag trägt seinen Zustand; gegen ihn wird nie etwas entschieden), mit Code-Alter", 1),
        (" nur wenn die Startzeit des Betriebssystems für die pid der registrierten entspricht (bei einem Legacy-Halter, der die Ressource länger als 4 h GEHALTEN hat, gezählt ab seiner Erlangung und nicht ab seinem Ticket, das seine Wartezeit enthält: vor seinem Ticket liegt), nur für", " nur wenn die Startzeit des Betriebssystems für die pid der registrierten entspricht, nur für", 1),
    ],
    R + "📓️build-guide.md": [
        ("or `legacy` (started before this code).", "or `unknown` (no live registry entry carries its standing; never stopped).", 1),
        ("; `reclaim.jsonl`; not on Windows; a `legacy` holder only after 4 h).", "; `reclaim.jsonl`; not on Windows).", 1),
        ("oder `legacy` (vor diesem Code gestartet).", "oder `unknown` (kein lebender Registry-Eintrag trägt seinen Zustand; nie beendet).", 1),
        ("; `reclaim.jsonl`; nicht unter Windows; ein `legacy`-Halter erst nach 4 h).", "; `reclaim.jsonl`; nicht unter Windows).", 1),
    ],
}
TESTS = [
    "./" + P + "🔒️leases/📈️standing/🧪️tests/🟦️.ts",
    "./" + P + "🔒️leases/🧪️tests/📋️status/🟦️.ts",
    "./" + P + "🔒️leases/🧭️order/🧪️tests/🕸️wait-for/🟦️.ts",
    "./" + P + "🧭️routing/📥️invocation/🏃️process/🧪️tests/📈️progress/🟦️.ts",
    "./" + L + "🏗️build/🚢️fleet-status/🧪️tests/🟦️.ts",
    "./" + L + "🏗️build/🩺️doctor/🧪️tests/🔎️probes/🟦️.ts",
    "./" + P + "🪪️agent-sessions/🧪️tests/🔗️adapters/🟦️.ts",
]
