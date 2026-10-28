"""🐍️ Independent oracle for leases standing."""
import json,re,statistics,sys
import math
half=lambda value:math.floor(value+0.5)

schema=json.load(open(sys.argv[1]))
corpus=json.load(open(sys.argv[2]))
policy=schema["definitions"]["Policy"]["const"]
advance={row["id"]:row["advanceMilliseconds"] for row in policy["priority"]["classes"]}

def priority(case):
    explicit=case["environment"].get(policy["priority"]["environment"])
    if explicit in advance:return explicit
    return "interactive" if case["interactiveTerminal"] and not case["agentSession"] else policy["priority"]["default"]

def simulate(jobs,fifo):
    pending=sorted(jobs,key=lambda j:(j["arrival"],j["id"]));waiting=[];done=[];time=0;index=0
    filed=lambda j:j["arrival"] if fifo else j["arrival"]-advance[j["class"]]/1000
    while index<len(pending) or waiting:
        while index<len(pending) and pending[index]["arrival"]<=time:
            waiting.append(pending[index]);index+=1
        if not waiting:
            time=pending[index]["arrival"];continue
        waiting.sort(key=lambda j:(filed(j),j["id"]));job=waiting.pop(0)
        done.append({"id":job["id"],"start":time,"wait":time-job["arrival"]});time+=job["service"]
    return done

def stuck_after(resource):
    rows=[row for row in policy["freshness"]["stuckAfterMilliseconds"] if resource.startswith(row["prefix"])]
    return max(rows,key=lambda row:len(row["prefix"]))["milliseconds"]

def state(case):
    holder=case.get("standing")
    if holder is None:return {"state":"unknown"}
    waiter=case.get("waiter")
    if waiter and waiter["loadedAtMs"]>holder["implementation"]["loadedAtMs"]:
        ids=sorted(i for i in case.get("bound",[]) if i in holder["implementation"]["components"] and i in waiter["components"] and holder["implementation"]["components"][i]!=waiter["components"][i])
        if ids:return {"state":"stale","staleIds":ids}
    silent=max(0,case["nowMs"]-max(holder["progressAtMs"],case["holdSinceMs"]))
    if silent>=stuck_after(case["resource"]):return {"state":"stuck"}
    if silent>=policy["freshness"]["silentAfterMilliseconds"]:return {"state":"silent"}
    return {"state":"progressing"}

def eta(case):
    rules=policy["eta"];done=case.get("done");total=case.get("total")
    if done is not None and total and done<=total and done/total>=rules["minimumFraction"] and case["phaseElapsedMs"]>0:
        return {"seconds":half(case["phaseElapsedMs"]/done*(total-done)/1000),"source":"progress","overdue":False}
    rows=sorted(case["historyMs"][-rules["maximumSamples"]:])
    if len(rows)<rules["minimumSamples"]:return None
    middle=statistics.median(rows)
    if case["elapsedMs"]>middle*rules["overdueFactor"]:return {"seconds":0,"source":"history","overdue":True}
    return {"seconds":half(max(0,middle-case["elapsedMs"])/1000),"source":"history","overdue":case["elapsedMs"]>middle}

def progress(case):
    resource=case["resource"];rules=policy["progress"]
    if any(resource.startswith(prefix) for prefix in rules["ignoredPrefixes"]):return None
    for family in rules["families"]:
        if resource.startswith(family["prefix"]):
            match=re.match(family["pattern"],resource)
            if match:return {"phase":match[1],"done":int(match[2]),"total":int(match[3])}
    match=re.match(rules["counter"],resource)
    if match:return {"phase":match[1][:rules["maximumPhaseCharacters"]],"done":int(match[2])}
    return {"phase":resource[:rules["maximumPhaseCharacters"]]}

def accept(case):
    request=case["request"];holder=case["standing"]
    if request["holderPid"]!=case["pid"] or request["holderStartedAtMs"]!=case["startedAtMs"]:return {"accept":False,"refusal":"target"}
    if case["nowMs"]-request["atMs"]>policy["yield"]["requestMaximumAgeMilliseconds"]:return {"accept":False,"refusal":"expired"}
    if not case["holdsExclusive"]:return {"accept":False,"refusal":"not-held"}
    if request["reason"]=="stale":
        ids=sorted(i for i in case["bound"] if i in case["moved"] and i in holder["implementation"]["components"] and i in request["requesterComponents"] and holder["implementation"]["components"][i]!=request["requesterComponents"][i])
        return {"accept":True,"ids":ids} if ids and holder["implementation"]["loadedAtMs"]<=request["requesterStartedAtMs"] else {"accept":False,"refusal":"not-stale"}
    silent=max(0,case["nowMs"]-max(holder["progressAtMs"],case["holdSinceMs"]))
    return {"accept":True} if silent>=stuck_after(request["resource"]) else {"accept":False,"refusal":"not-stuck"}

def activity(case):
    rules=policy["freshness"]
    if case.get("lastProbeAtMs") is not None and case["nowMs"]-case["lastProbeAtMs"]<rules["activityProbeMilliseconds"]:
        return {"sampled":False,"progressAtMs":case["progressAtMs"]}
    seen=case["sample"]
    busy=seen["treeActive"] or seen["cpuDeltaMilliseconds"]>=rules["activityMinimumCpuMilliseconds"]
    return {"sampled":True,"progressAtMs":max(case["progressAtMs"],case["nowMs"]) if busy else case["progressAtMs"]}

def terminate(case):
    i=case["input"];rules=policy["reclaim"]
    if i["mode"]!="terminate" or i["self"] or i["state"]!="stuck":return {"terminate":False}
    if "requestAtMs" not in i or i["nowMs"]-i["requestAtMs"]<rules["graceMilliseconds"]:return {"terminate":False,"refusal":"grace"}
    if i["progressAtMs"]>i["requestAtMs"]:return {"terminate":False,"refusal":"progressed"}
    if "osStartedAtMs" not in i:return {"terminate":False,"refusal":"unknown"}
    if "registeredStartedAtMs" not in i or abs(i["osStartedAtMs"]-i["registeredStartedAtMs"])>rules["identityToleranceMilliseconds"]:return {"terminate":False,"refusal":"identity"}
    if not i["signalable"]:return {"terminate":False,"refusal":"owner"}
    return {"terminate":True}

print(json.dumps({
 "yields":[accept(case) for case in corpus["yieldCases"]],
 "terminates":[terminate(case) for case in corpus["terminateCases"]],
 "progress":[progress(case) for case in corpus["progressCases"]],
 "priority":[priority(case) for case in corpus["priorityCases"]],
 "simulations":{sim["id"]:{"fifo":simulate(sim["jobs"],True),"priority":simulate(sim["jobs"],False)} for sim in corpus["simulations"]},
 "states":[state(case) for case in corpus["stateCases"]],
 "eta":[eta(case) for case in corpus["etaCases"]],
 "activity":[activity(case) for case in corpus["activityCases"]],
}))
