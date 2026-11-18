#!/usr/bin/env python3
"""Independent oracle of the fence budget: replays the runs of the fixture against the policy of the schema and prints, per run and step, whether the fence is full. Usage: oracle.py <schema.json> <fixture.json>"""
import json,sys
schema=json.load(open(sys.argv[1],encoding="utf8"));fixture=json.load(open(sys.argv[2],encoding="utf8"))
p=schema["definitions"]["Policy"]["const"]
out=[]
for run in fixture["runs"]:
    last_end=None;last_dur=0;rows=[]
    for s in run["steps"]:
        gap=min(p["maximumGapMilliseconds"],p["factor"]*last_dur)
        due=bool(s["final"]) or last_end is None or s["nowMs"]-last_end>=gap
        rows.append(due)
        if due:last_end=s["nowMs"]+s["durationMs"];last_dur=s["durationMs"]
    out.append(rows)
print(json.dumps(out))
