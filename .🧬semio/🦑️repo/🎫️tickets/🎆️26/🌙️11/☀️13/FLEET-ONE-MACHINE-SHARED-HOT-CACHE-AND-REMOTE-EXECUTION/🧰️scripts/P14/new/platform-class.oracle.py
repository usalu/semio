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
    return platform if any(marker in key for marker in policy["hostBound"]) else klass(platform)
out={"cases":[{"name":c["name"],"record":record(c["keyText"],c["platform"]),"class":klass(c["platform"])} for c in fixture["cases"]],
     "serves":[{"server":s["server"],"requester":s["requester"],"serves":klass(s["server"])==klass(s["requester"])} for s in fixture["serves"]]}
print(json.dumps(out,ensure_ascii=False,sort_keys=True))
