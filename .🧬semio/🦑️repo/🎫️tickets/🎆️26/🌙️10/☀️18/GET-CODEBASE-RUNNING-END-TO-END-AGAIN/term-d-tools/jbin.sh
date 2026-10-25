#!/bin/sh
# usage: jbin.sh <tag>  builds the semio binary graph-free; path in <tag>.exe
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/term-d"; mkdir -p "$O"; tag=$1
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- build -p semio-framework-repo-dashboard --bin semio --message-format=json > "$O/$tag.json" 2> "$O/$tag.err"
rc=$?
python3 - "$O/$tag.json" "$O/$tag.exe" <<'PY'
import json,sys
exe=[]
for l in open(sys.argv[1],errors="replace"):
    try:r=json.loads(l)
    except Exception: continue
    if r.get("reason")=="compiler-artifact" and r.get("target",{}).get("name")=="semio" and "bin" in r["target"]["kind"] and r.get("executable"): exe.append(r["executable"])
open(sys.argv[2],"w").write("\n".join(exe))
PY
echo "rc=$rc exe=$(cat "$O/$tag.exe")" > "$O/$tag.done"
