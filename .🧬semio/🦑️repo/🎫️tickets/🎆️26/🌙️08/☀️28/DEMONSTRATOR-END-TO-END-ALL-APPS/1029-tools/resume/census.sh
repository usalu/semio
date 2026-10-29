#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
bun "$C/1028-tools/gatecensus/census.ts" "$C/1029-tools/resume/roots.txt" > "$C/🗑️generated/1029/resume-census.json" 2>&1
python3 - <<PY
import json,sys
t=open("$C/🗑️generated/1029/resume-census.json").read()
d=json.loads(t[t.index("{"):])
for r in d["rows"]:
    if r["name"] in open("$C/1029-tools/resume/roots.txt").read().split():
        print(r["name"],r["bytes"],"/",r["lim"]["bytes"],r["files"],"files",r["lines"],"lines")
PY
