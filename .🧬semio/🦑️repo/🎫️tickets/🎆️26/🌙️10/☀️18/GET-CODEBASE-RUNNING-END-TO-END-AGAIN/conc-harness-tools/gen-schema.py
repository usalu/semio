#!/usr/bin/env python3
"""Writes src/schema.json: the closed contract of the concurrent-session harness (workloads, lease classes, pass criteria, measurements, report, fixture)."""
import json,os
here=os.path.dirname(os.path.abspath(__file__))
ID="^[a-z][a-z0-9]*(-[a-z0-9]+)*$"
def ref(n): return {"$ref":"#/definitions/"+n}
def obj(props,required=None,extra=False):
    return {"type":"object","additionalProperties":extra,"required":list(props) if required is None else required,"properties":props}
def arr(item,**kw): return {"type":"array","items":item,**kw}
def uniq(item,**kw): return {"type":"array","uniqueItems":True,"items":item,**kw}
integer=lambda minimum=0:{"type":"integer","minimum":minimum}
number=lambda minimum=0:{"type":"number","minimum":minimum}
string={"type":"string"}
nstring={"type":"string","minLength":1}
policy={
 "version":1,"id":ID,"taskId":"^\\S+:\\S+$","nonce":"^[0-9a-z]{6,32}$",
 "maximumSessions":8,"defaultSessions":3,"maximumSteps":64,"maximumPathLength":255,
 "maximumStepMilliseconds":1800000,"maximumPauseMilliseconds":600000,
 "maximumLines":200000,"maximumLogBytes":268435456,"tailLines":40,"episodeGapMilliseconds":20000,
 "scriptRow":"^\\[script\\] (\\{.*\\})$","queuePrefix":"lease-queue:",
 "ignoredPrefixes":["javascript-source-provider:","javascript-preserving-source:","cargo:source-admission:","build:parser-source:","build:graph-phase:","cache-scan:","cache-delete:","boot-recovery:","dependencies:","resource-directory-admission:","compiler:capacity-subdivision"],
 "bazelWait":"Another command \\(pid=(\\d+)\\) is running\\. Waiting for it to complete on the server",
 "bazelContinuation":"^(?:INFO: |WARNING: )?(?:Another command \\(pid=|Waiting for it to complete)",
 "bazelResource":"bazel:server",
 "cargoWait":"^\\s*Blocking waiting for file lock on (.+?)\\s*$","cargoResourcePrefix":"cargo:",
 "taskReport":"SEMIO_BUILD_TASK_REPORT","leaseTrace":"SEMIO_LEASE_TRACE","nxSkewMilliseconds":2000,
 "relations":["related","unrelated","external"],
}
D={}
D["Policy"]={"const":policy}
D["Tool"]={"enum":["nx","bazel"]}
D["Language"]={"enum":["ts","rs","go","py","text"]}
D["EditOp"]={"enum":["comment","private-body","public-api","whitespace"]}
D["Id"]={"type":"string","pattern":ID}
D["TaskId"]={"type":"string","pattern":"^\\S+:\\S+$"}
D["Path"]={"type":"string","minLength":1,"maxLength":255}
D["Paths"]=uniq(ref("Path"))
D["LocalizedText"]=obj({"en":nstring,"de":nstring})
D["Mode"]={"enum":["shared","exclusive"]}
D["Relation"]={"enum":["related","unrelated","external"]}
D["Observed"]={"enum":["exact","window","ambiguous","none"]}
D["Status"]={"enum":["pass","fail","skip"]}
D["Scope"]={"enum":["workspace","project","host"]}
D["EditStep"]=obj({"kind":{"const":"edit"},"file":ref("Path"),"op":ref("EditOp"),"language":ref("Language")})
D["RevertStep"]=obj({"kind":{"const":"revert"}})
D["RunStep"]=obj({"kind":{"const":"run"},"target":ref("TaskId"),"outputs":ref("Paths")},["kind","target"])
D["RepoStep"]=obj({"kind":{"const":"repo"},"args":arr(nstring,minItems=1) if False else {"type":"array","minItems":1,"items":nstring},"outputs":ref("Paths"),"stdout":{"type":"boolean"}},["kind","args"])
D["BarrierStep"]=obj({"kind":{"const":"barrier"},"id":ref("Id")})
D["PauseStep"]=obj({"kind":{"const":"pause"},"milliseconds":{"type":"integer","minimum":1,"maximum":600000}})
D["Step"]={"oneOf":[ref(x) for x in ("EditStep","RevertStep","RunStep","RepoStep","BarrierStep","PauseStep")]}
D["SetupStep"]={"oneOf":[ref(x) for x in ("EditStep","RunStep","RepoStep")]}
D["Session"]=obj({"id":ref("Id"),"area":ref("Id"),"group":ref("Id"),"delayMilliseconds":{"type":"integer","minimum":0,"maximum":600000},"steps":{"type":"array","minItems":1,"maxItems":64,"items":ref("Step")}},["id","area","group","steps"])
D["Workload"]=obj({"id":ref("Id"),"title":ref("LocalizedText"),"summary":ref("LocalizedText"),"criteria":ref("Id"),"tools":{"type":"array","minItems":1,"uniqueItems":True,"items":ref("Tool")},"setup":{"type":"array","maxItems":64,"items":ref("SetupStep")},"sessions":{"type":"array","minItems":1,"maxItems":8,"items":ref("Session")}},["id","title","summary","criteria","setup","sessions"])
D["LeaseClass"]=obj({"id":ref("Id"),"label":ref("LocalizedText"),"scope":ref("Scope"),"patterns":{"type":"array","minItems":1,"items":nstring}})
D["Owners"]={"type":"array","uniqueItems":True,"minItems":1,"items":ref("Id")}
ruleHead={"id":ref("Id"),"title":ref("LocalizedText"),"owners":ref("Owners")}
def rule(kind,extra):
    props={**ruleHead,"kind":{"const":kind},**extra}
    return obj(props)
D["RuleNoFailure"]=rule("no-failure",{})
D["RuleSerialEqual"]=rule("serial-equal",{"compare":{"type":"array","minItems":1,"uniqueItems":True,"items":{"enum":["exit","outputs","stdout"]}}})
D["RuleMaxWait"]=rule("max-wait",{"classes":{"type":"array","minItems":1,"uniqueItems":True,"items":nstring},"relation":{"enum":["related","unrelated","any"]},"maximumMilliseconds":integer()})
D["Sessions"]={"type":"array","minItems":1,"uniqueItems":True,"items":nstring}
D["RuleExecutesOnce"]=rule("executes-once",{"sessions":ref("Sessions"),"excludeUncached":{"type":"boolean"}})
D["RuleMaxExecuted"]=rule("max-executed",{"sessions":ref("Sessions"),"maximum":integer(),"excludeUncached":{"type":"boolean"}})
D["Rule"]={"oneOf":[ref(x) for x in ("RuleNoFailure","RuleSerialEqual","RuleMaxWait","RuleExecutesOnce","RuleMaxExecuted")]}
D["Criteria"]=obj({"id":ref("Id"),"title":ref("LocalizedText"),"rules":{"type":"array","minItems":1,"items":ref("Rule")}})
labels=["title","workload","tool","session","group","step","wall","wait","executed","restored","cached","steps","concurrent","serial","leases","lease","count","total","maximum","relation","related","unrelated","external","holders","rules","rule","status","pass","fail","skip","value","limit","owner","acceptance","summary","criteria","sessions","milliseconds","area","setup","progress","cancelled","unproven","exact","window","ambiguous","none","start","end","exit","target","phase","list","uncached","independent","refused"]
D["Labels"]=obj({k:ref("LocalizedText") for k in labels})
D["WorkloadSet"]=obj({"version":{"const":1},"labels":ref("Labels"),"classes":{"type":"array","minItems":1,"items":ref("LeaseClass")},"criteria":{"type":"array","minItems":1,"items":ref("Criteria")},"workloads":{"type":"array","minItems":1,"items":ref("Workload")}})
D["Wait"]=obj({"class":nstring,"resource":nstring,"mode":ref("Mode"),"source":{"enum":["lease","trace","bazel","cargo"]},"precise":{"type":"boolean"},"startMs":number(-1e12),"endMs":number(-1e12),"waitMs":number(),"holderPid":{"type":["integer","null"]},"position":{"type":["integer","null"]},"relation":ref("Relation"),"holders":{"type":"array","items":nstring}},["class","resource","mode","source","precise","startMs","endMs","waitMs","holderPid","position"])
D["Observation"]=obj({"executed":ref("TaskIds"),"restored":ref("TaskIds"),"cached":ref("TaskIds")})
D["TaskIds"]=uniq(ref("TaskId"))
D["StepResult"]=obj({"index":integer(),"kind":{"enum":["run","repo"]},"command":{"type":"array","items":string},"startMs":number(-1e12),"endMs":number(-1e12),"exit":{"type":["integer","null"]},"waits":arr(ref("Wait")),"observation":{"oneOf":[ref("Observation"),{"type":"null"}]},"observed":ref("Observed"),"outputs":{"type":["string","null"]},"stdout":{"type":["string","null"]}})
D["SessionResult"]=obj({"id":ref("Id"),"group":ref("Id"),"startMs":number(-1e12),"endMs":number(-1e12),"steps":arr(ref("StepResult"))})
D["Measurement"]=obj({"workload":ref("Id"),"tool":ref("Tool"),"uncached":ref("TaskIds"),"sessions":arr(ref("SessionResult")),"serial":{"oneOf":[arr(ref("SessionResult")),{"type":"null"}]}})
D["RuleResult"]=obj({"rule":ref("Id"),"kind":{"enum":["no-failure","serial-equal","max-wait","executes-once","max-executed"]},"owners":ref("Owners"),"status":ref("Status"),"value":{"type":["number","null"]},"limit":{"type":["number","null"]},"detail":arr(string)})
D["Run"]=obj({"workload":ref("Id"),"tool":ref("Tool"),"criteria":ref("Id"),"measurement":ref("Measurement"),"rules":arr(ref("RuleResult"))})
D["OwnerTally"]=obj({"owner":ref("Id"),"passed":integer(),"failed":integer(),"skipped":integer()})
D["Report"]=obj({"version":{"const":1},"runs":arr(ref("Run")),"passed":integer(),"failed":integer(),"skipped":integer(),"owners":arr(ref("OwnerTally"))})
D["TraceRow"]=obj({"version":{"const":1},"resource":nstring,"mode":ref("Mode"),"startedMs":integer(),"acquiredMs":integer(),"pid":integer(1),"holderPid":integer(1)},["version","resource","mode","startedMs","acquiredMs","pid"])
D["Fixture"]=obj({"version":{"const":1},"lines":arr(obj({"name":nstring,"classes":arr(ref("LeaseClass")),"endMs":number(),"lines":arr(obj({"atMs":number(-1e12),"line":string})),"waits":arr(ref("Wait"))})),
 "relations":arr(obj({"name":nstring,"wait":obj({"startMs":number(-1e12),"endMs":number(-1e12)}),"self":obj({"id":nstring,"group":nstring}),"others":arr(obj({"id":nstring,"group":nstring,"intervals":arr({"type":"array","items":number(-1e12),"minItems":2,"maxItems":2})})),"relation":ref("Relation"),"holders":arr(nstring)})),
 "evaluations":arr(obj({"name":nstring,"criteria":ref("Criteria"),"measurement":ref("Measurement"),"results":arr(ref("RuleResult"))})),
 "mutations":arr(obj({"name":nstring,"text":string,"op":ref("EditOp"),"language":ref("Language"),"nonce":nstring,"result":string})),
 "independence":arr(obj({"name":nstring,"rows":arr(obj({"id":nstring,"group":nstring,"edits":arr(nstring),"touches":arr(nstring)})),"issues":arr(string)})),
 "workloads":arr(obj({"name":nstring,"set":{},"issues":arr(string)})),
 "nx":arr(obj({"name":nstring,"record":{},"ids":arr(nstring),"windowStart":number(-1e12),"windowEnd":number(-1e12),"concurrent":{"type":"boolean"},"result":{}})),
 "digests":arr(obj({"name":nstring,"parts":arr(obj({"path":nstring,"text":string})),"digest":nstring})),
 "traces":arr(obj({"name":nstring,"classes":arr(ref("LeaseClass")),"origin":number(-1e12),"text":string,"waits":arr(ref("Wait"))})),
 "merges":arr(obj({"name":nstring,"derived":arr(ref("Wait")),"traced":arr(ref("Wait")),"waits":arr(ref("Wait"))})),
 "tallies":arr(obj({"name":nstring,"runs":arr({}),"result":{}}))})
schema={"$schema":"http://json-schema.org/draft-07/schema#","title":"Concurrent Session Harness","description":"Workloads of simulated agents working in one checkout copy, the lease classes their waits are attributed to, the pass criteria, the measurements and the report of the concurrency acceptance harness; every build implementation behind the tool-neutral interface is judged by the same contract.","type":"object","additionalProperties":False,"required":["version"],"properties":{"version":{"const":1}},"definitions":D}
json.dump(schema,open(os.path.join(here,"src","schema.json"),"w",encoding="utf8"),ensure_ascii=False,indent=2);open(os.path.join(here,"src","schema.json"),"a").write("\n")
print("schema definitions:",len(D))
