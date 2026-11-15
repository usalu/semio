#!/usr/bin/env python3
"""Independent oracle of the platform classes: reads the schema and the fixture and prints the expected record platform, class and serving verdict of every case as JSON lines. Usage: platform-class.oracle.py <schema.json> <fixture.json>"""
import json,sys
schema=json.load(open(sys.argv[1],encoding="utf8"));fixture=json.load(open(sys.argv[2],encoding="utf8"))
policy=schema["definitions"]["Policy"]["const"]
def klass(platform):
    for name,members in policy["classes"].items():
        if platform in members:return name
    return platform
def record(key,platform):
    return platform if any(marker in key for marker in policy["hostBound"]) else policy["neutral"]
def split(projects,server,requester):
    if klass(server)==klass(requester):return list(projects),[]
    bound=set(policy["boundOwners"]);return [p for p in projects if p not in bound],[p for p in projects if p in bound]
out={"cases":[{"name":c["name"],"record":record(c["keyText"],c["platform"]),"class":klass(c["platform"])} for c in fixture["cases"]],
     "serves":[{"server":s["server"],"requester":s["requester"],"serves":klass(s["server"])==klass(s["requester"])} for s in fixture["serves"]],
     "splits":[dict(zip(("served","left"),split(s["projects"],s["server"],s["requester"]))) for s in fixture["splits"]]}
print(json.dumps(out,ensure_ascii=False,sort_keys=True))
