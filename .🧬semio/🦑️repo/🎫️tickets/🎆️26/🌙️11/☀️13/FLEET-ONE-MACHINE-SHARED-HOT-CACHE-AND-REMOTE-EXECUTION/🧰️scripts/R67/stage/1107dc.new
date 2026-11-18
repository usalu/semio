#!/usr/bin/env python3
"""Independent oracle of the two pure rules of the publication of a record: how many closure documents fit the budget (the longest prefix) and how long to wait before the next try (back-off of the try that failed, stretched by the jitter, never shorter than the rest of the breaker up to its maximum). Usage: oracle.py <schema.json> <fixture.json>"""
import json,sys,math
schema=json.load(open(sys.argv[1],encoding="utf8"));fixture=json.load(open(sys.argv[2],encoding="utf8"))
p=schema["definitions"]["Publish"]["const"]
def prefix(sizes,budget):
    used=0
    for i,s in enumerate(sizes):
        if used+s>budget:return i
        used+=s
    return len(sizes)
def js_round(x):return math.floor(x+0.5)
def wait(attempt,breaker,unit):
    b=p["backoffMilliseconds"];base=b[min(attempt,len(b)-1)]
    return max(js_round(base*(1+unit*p["backoffJitterPercent"]/100)),min(max(0,breaker),p["breakerWaitMaximumMilliseconds"]))
print(json.dumps({"prefix":[prefix(c["sizes"],c["budget"]) for c in fixture["prefix"]],"wait":[wait(c["attempt"],c["breaker"],c["unit"]) for c in fixture["wait"]]}))
