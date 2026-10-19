#!/bin/zsh
# usage: run.sh <list-file> <name> [vitest args...]  -> prints summary; full log under 🗑️generated/osplay
cd /Users/ueli/Documents/semio
C=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
OUT=$C/🗑️generated/osplay
rm -f $OUT/$2.jsonl
shift_list=$1; name=$2; shift 2
OSPLAY_ARGS="$*" python3 $C/osplay-tools/runvt.py $shift_list $OUT/$name.jsonl 1700 1700
python3 - $OUT/$name.jsonl <<'P'
import json,sys,shutil
for l in open(sys.argv[1],encoding="utf-8"):
    r=json.loads(l); print(r["rc"],r["tests"],"|",r["files"],"|",r["secs"],"s")
    if r.get("log"): shutil.copy(r["log"], sys.argv[1].replace(".jsonl",".log")); print("log:",sys.argv[1].replace(".jsonl",".log"))
P
