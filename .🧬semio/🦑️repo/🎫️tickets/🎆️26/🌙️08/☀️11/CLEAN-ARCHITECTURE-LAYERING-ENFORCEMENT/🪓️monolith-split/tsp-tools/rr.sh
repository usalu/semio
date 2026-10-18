#!/bin/zsh
# usage: rr.sh <substring...> : rerun currently-failing (per latest merged state) files matching all substrings, print summary
cd /Users/ueli/Documents/semio
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
python3 - "$@" <<PY
import json,sys,subprocess,re
F="$F"
r=json.load(open(F+"/tsp-tools/baseline-merged.json"))
pats=sys.argv[1:]
for x in r:
    if x["status"]=="pass" or not all(p in x["path"] for p in pats): continue
    p=x["path"]
    o=subprocess.run(["nice","bun","test","./"+p],capture_output=True,text=True,cwd="/Users/ueli/Documents/semio")
    out=o.stdout+o.stderr
    m=re.search(r"(\d+) pass",out); n=re.search(r"(\d+) fail",out)
    errs=re.findall(r"\nerror: (.{0,160})",out)
    print(("PASS " if o.returncode==0 and m else "FAIL "), (m.group(1) if m else "0"),"/",(n.group(1) if n else "0"), p.split("🪆️subsets/")[-1][-90:], "|", (errs[0] if errs else "").strip()[:150])
PY
