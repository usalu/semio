#!/bin/zsh
# usage: rr.sh listfile tag  -> hoist run, prints summary
C=$(ls -d /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/*/*/*/GET-CODEBASE-RUNNING-END-TO-END-AGAIN)
rm -f $C/🗑️generated/tst3c-out/$2.jsonl
TST3C_HOIST=1 python3 $C/tst3c-tools/run.py $1 $C/🗑️generated/tst3c-out/$2.jsonl $2 3000
python3 - <<PY
import json
for l in open("$C/🗑️generated/tst3c-out/$2.jsonl",encoding="utf-8"):
    r=json.loads(l); print("OK " if r["rc"]==0 else "RED",r["pass_"],r["fail"],r["file"][-90:])
PY
