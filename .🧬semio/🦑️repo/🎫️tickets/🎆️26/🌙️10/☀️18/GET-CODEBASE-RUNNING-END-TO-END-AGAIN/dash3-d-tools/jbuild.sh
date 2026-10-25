#!/bin/sh
# usage: jbuild.sh <tag>   builds the journeys test executable graph-free; prints compiler-artifact path into <tag>.exe
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
O="$C/🗑️generated/dash3-d"; mkdir -p "$O"; tag=$1
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.toml"
nice -n 10 bun repo cargo --store=shared --configuration=prototype -- test --manifest-path "$M" -p semio-dashboard-journeys --test journeys --no-run --locked --message-format=json > "$O/$tag.json" 2> "$O/$tag.err"
rc=$?
python3 - "$O/$tag.json" "$O/$tag.exe" <<'PY'
import json,sys
exe=[]
for l in open(sys.argv[1],errors="replace"):
    try:r=json.loads(l)
    except Exception: continue
    if r.get("reason")=="compiler-artifact" and r.get("target",{}).get("name")=="journeys" and "test" in r["target"]["kind"] and r.get("executable"): exe.append(r["executable"])
open(sys.argv[2],"w").write("\n".join(exe))
PY
echo "rc=$rc exe=$(cat "$O/$tag.exe")" > "$O/$tag.done"
