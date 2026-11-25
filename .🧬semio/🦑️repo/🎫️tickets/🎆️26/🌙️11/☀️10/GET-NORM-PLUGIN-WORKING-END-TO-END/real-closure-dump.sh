#!/bin/sh
cd "$(git rev-parse --show-toplevel)" || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
python3 "$T/real-closure-jobs.py" "$1" > "$T/🗑️generated/dump-jobs.json" || exit 1
DUMP=1 node --no-warnings "$T/real-closure-probe.mjs" "$T/🗑️generated/dump-jobs.json" | python3 -c '
import json,sys
for r in json.load(sys.stdin):
    if r["unit"]!="lib[cfg(test)]": continue
    for p,b,l,k in sorted(r["inputs"],key=lambda x:-x[1]): print(f"{b:8d} {l:5d} {k:8s} {p}")
    print(r["files"],r["bytes"],r["lines"])
'
