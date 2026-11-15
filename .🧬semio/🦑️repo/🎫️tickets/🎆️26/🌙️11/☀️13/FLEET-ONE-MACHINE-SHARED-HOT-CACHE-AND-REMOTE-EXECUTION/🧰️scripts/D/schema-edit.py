import json,sys,copy
B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SP=B+"🏃️execution/🧬️schema/🌐️remote-execution.json"
PP=B+"🔣️policy.json"
sd=json.load(open(SP,encoding="utf8"));pd=json.load(open(PP,encoding="utf8"))
D=sd["definitions"]
if "LoadReport" in D: print("already applied");sys.exit(0)
I=lambda lo=0,hi=None:{"type":"integer","minimum":lo,**({"maximum":hi} if hi is not None else {})}
S=lambda lo=1,hi=128:{"type":"string","minLength":lo,"maxLength":hi}
def obj(props,req=None,desc=None):
    o={"type":"object","additionalProperties":False,"required":req if req is not None else list(props.keys()),"properties":props}
    if desc: o={"description":desc,**o}
    return o
D["Origin"]={"type":"string","pattern":"^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$","description":"The origin id of a machine of the fleet (for example g1-m03)."}
D["Pressure"]={"enum":["normal","warn","critical"],"description":"The pressure level of the host capacity sample."}
D["Confinement"]={"enum":["full","none"],"description":"Whether the sandbox of a worker confines the file system of an action (`none` for the private input root alone)."}
D["NetworkPolicy"]={"enum":["denied","unenforced","allowed"],"description":"What a worker does about the network of an action: `denied` (the sandbox cuts it), `unenforced` (denial is asked for but the strategy cannot enforce it), `allowed` (the policy lets actions use it)."}
D["PlatformIdentity"]=obj({"key":S(1,160),"os":S(1,32),"arch":S(1,32),"toolchain":{"oneOf":[{"type":"string","pattern":"^[0-9a-f]{16}$"},{"type":"null"}]},"target":{"oneOf":[S(1,64),{"type":"null"}]}},desc="The platform key of a machine: operating system, architecture and the build id of its Rust toolchain (the first 16 hex digits of the sha256 of `rustc -vV`); units are shared and offloaded only between equal keys.")
D["MachineCounters"]=obj({"unit":obj({"localHit":I(),"remoteHit":I(),"miss":I(),"stored":I()}),"offload":obj({"out":I(),"refused":I(),"fellBack":I()})},desc="Counters of the rustc wrapper and unit cache of one machine, read from `.cache/f/build-counters.json` (zeros when absent).")
D["WorkerState"]=obj({"state":{"enum":["absent","idle","busy","refusing"]},"concurrency":I(0,512),"running":I(0,512),"sandbox":{"oneOf":[{"$ref":"#/definitions/Sandbox"},{"type":"null"}]},"confinement":{"oneOf":[{"$ref":"#/definitions/Confinement"},{"type":"null"}]},"network":{"oneOf":[{"$ref":"#/definitions/NetworkPolicy"},{"type":"null"}]}},desc="What the build worker of a machine does right now.")
D["LoadReport"]=obj({"version":{"const":1},"origin":{"$ref":"#/definitions/Origin"},"platform":{"$ref":"#/definitions/PlatformIdentity"},"capacity":obj({"cores":I(1,4096),"memoryMegabytes":I(1)}),"running":I(0,100000),"queued":I(0,100000),"pressure":{"$ref":"#/definitions/Pressure"},"freeMegabytes":I(0),"load":{"oneOf":[{"type":"number","minimum":0},{"type":"null"}]},"headroom":{"type":"boolean"},"wallMs":I(0,9007199254740991),"worker":{"$ref":"#/definitions/WorkerState"},"counters":{"$ref":"#/definitions/MachineCounters"}},desc="What every machine sends to the scheduler every few seconds: who it is, its platform key, capacity, running and queued work, the pressure level of the host capacity sample, free memory, load, its own admission verdict (`headroom`) and its wall clock.")
D["LoadRow"]=obj({"origin":{"$ref":"#/definitions/Origin"},"platform":{"$ref":"#/definitions/PlatformIdentity"},"capacity":obj({"cores":I(1,4096),"memoryMegabytes":I(1)}),"running":I(0,100000),"queued":I(0,100000),"pressure":{"$ref":"#/definitions/Pressure"},"freeMegabytes":I(0),"load":{"oneOf":[{"type":"number","minimum":0},{"type":"null"}]},"headroom":{"type":"boolean"},"ageSeconds":I(0),"worker":{"$ref":"#/definitions/WorkerState"},"offloadedIn":I(),"offloadedOut":I(),"refused":I(),"counters":{"$ref":"#/definitions/MachineCounters"}},desc="One machine of the live table.")
D["TaskRow"]=obj({"id":S(1,64),"action":{"type":"string","pattern":"^[0-9a-f]{12}$"},"stage":{"enum":["QUEUED","EXECUTING"]},"worker":{"oneOf":[S(1,128),{"type":"null"}]},"requester":{"oneOf":[{"$ref":"#/definitions/Origin"},{"type":"null"}]},"elapsedSeconds":I()},desc="One action the scheduler holds: queued or executing.")
D["FleetLoad"]=obj({"version":{"const":1},"service":{"const":"semio-remote-execution"},"wallMs":I(0,9007199254740991),"expireSeconds":I(1),"machines":{"type":"array","items":{"$ref":"#/definitions/LoadRow"}},"tasks":{"type":"array","items":{"$ref":"#/definitions/TaskRow"}},"queued":I(),"executing":I(),"counters":{"$ref":"#/definitions/Counter"}},desc="What `GET /v1/fleet/load` answers: the live table of the fleet.")
D["Refusal"]={"enum":["memory","swap","swap-activity","pressure-critical","pressure-warn","files","attaches","load","queue-order","ramp","quiet-limit","slots"],"description":"Why a worker's own admission refused an action."}
reg=D["WorkerRegistration"]
reg["required"]+=["origin","capacity","confinement","network"]
reg["properties"].update({"origin":{"$ref":"#/definitions/Origin"},"capacity":obj({"cores":I(1,4096),"memoryMegabytes":I(1)}),"confinement":{"$ref":"#/definitions/Confinement"},"network":{"$ref":"#/definitions/NetworkPolicy"}})
tr=D["TaskResult"]
tr["properties"]["outcome"]={"enum":["completed","failed","cancelled","refused"]}
tr["properties"]["refusal"]={"oneOf":[{"$ref":"#/definitions/Refusal"},{"type":"null"}]}
tr["required"]=tr["required"]
wr=D["WorkerRow"]
wr["required"]+=["origin","confinement","network","headroom","offloadedIn","refused"]
wr["properties"].update({"origin":{"$ref":"#/definitions/Origin"},"confinement":{"$ref":"#/definitions/Confinement"},"network":{"$ref":"#/definitions/NetworkPolicy"},"headroom":{"type":"boolean"},"offloadedIn":I(),"refused":I()})
c=D["Counter"]
c["required"]+=["refused","offloadRejected"]
c["properties"].update({"refused":I(),"offloadRejected":I()})
sp=D["ServerPolicy"]
sp["required"]+=["loadExpireSeconds","refusalCooldownSeconds"]
sp["properties"].update({"loadExpireSeconds":I(1,3600),"refusalCooldownSeconds":I(1,3600)})
wp=D["WorkerPolicy"]
wp["required"]+=["loadIntervalSeconds","network","confinement"]
wp["properties"].update({"loadIntervalSeconds":I(1,300),"network":{"enum":["deny","allow"],"description":"`deny`: every action runs without network (a strategy that cannot enforce it is reported as `unenforced`); `allow`: actions may use the network."},"confinement":{"enum":["required","best-effort"],"description":"`required`: a worker whose host has no confining sandbox refuses to start; `best-effort`: it starts with the private input root alone and reports `confinement: none`."}})
r=pd["remoteExecution"]
r["server"].update({"maximumAttempts":2,"loadExpireSeconds":15,"refusalCooldownSeconds":10})
r["worker"].update({"loadIntervalSeconds":5,"network":"deny","confinement":"best-effort"})
json.dump(sd,open(SP,"w",encoding="utf8"),indent=2,ensure_ascii=False);open(SP,"a").write("\n")
pol=json.load(open(PP,encoding="utf8"))
pol["remoteExecution"]=pd["remoteExecution"]
json.dump(pol,open(PP,"w",encoding="utf8"),indent=2,ensure_ascii=False);open(PP,"a").write("\n")
print("ok")
