#!/usr/bin/env python3
"""🐍️ Independent oracle of the worker liveness rule of `../🟦️.ts`. It shares no code with it: it reads only the closed policy of the readiness schema, takes JSON lines `{status, heartbeatMs, nowMs}` on stdin and prints one JSON line `{liveness, ageMs, heartbeatAgeMs, missedBeats}` per case.
usage: 🐍️.py < cases.jsonl
Python 3.9 compatible."""
import json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))
POLICY=json.load(open(os.path.join(HERE,"..","..","🧬️schema","🔣️.json"),encoding="utf8"))["definitions"]["Policy"]["const"]
WORK=("running","waiting","backoff")
def judge(case):
    status,heartbeat,now=case["status"],case["heartbeatMs"],case["nowMs"]
    beat_age=None if heartbeat is None else max(0,now-heartbeat)
    updated=status.get("updatedMs") if isinstance(status,dict) else None
    if not isinstance(updated,(int,float)) or isinstance(updated,bool): return {"liveness":"absent","ageMs":None,"heartbeatAgeMs":beat_age,"missedBeats":0}
    declared=status.get("beatMilliseconds")
    beat=min(declared,POLICY["offlineMilliseconds"]/POLICY["silentBeatsMaximum"]) if isinstance(declared,(int,float)) and not isinstance(declared,bool) and declared>0 else POLICY["statusHeartbeatMilliseconds"]
    age=max(0,now-updated);missed=int(age//beat)
    newest=age if beat_age is None else min(age,beat_age)
    if newest>=POLICY["offlineMilliseconds"]: liveness="offline"
    elif status.get("ready") is False and status.get("state") in WORK: liveness="silent" if missed>=POLICY["silentBeatsMaximum"] else "working"
    else: liveness="ready" if status.get("ready") is True else "idle"
    return {"liveness":liveness,"ageMs":age,"heartbeatAgeMs":beat_age,"missedBeats":missed}
if __name__=="__main__":
    for line in sys.stdin:
        if line.strip(): print(json.dumps(judge(json.loads(line))))
