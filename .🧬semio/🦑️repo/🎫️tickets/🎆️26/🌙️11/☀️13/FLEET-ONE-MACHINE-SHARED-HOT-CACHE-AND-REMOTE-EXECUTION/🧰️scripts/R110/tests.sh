#!/bin/bash
# usage: tests.sh <outdir>
R=/Users/ueli/Documents/semio
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
L="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
OUT="$1"; mkdir -p "$OUT"
FILES=(
"$L/🌐️remote-execution/🛰️sidecar/🚦️ready/🧪️tests/🟦️.ts"
"$L/🌐️remote-execution/🛰️sidecar/🧪️tests/🟦️.ts"
"$L/🌐️remote-cache/🛰️fleet/🧪️tests/🟦️.ts"
"$L/🌐️remote-cache/🛰️fleet/🪪️identity/🧪️tests/🟦️.ts"
"$L/🪪️graph-state/📍️cursor/🧪️tests/🟦️.ts"
"$L/🪪️graph-state/📦️project-records/🧪️tests/🟦️.ts"
"$L/🪪️graph-state/📦️project-records/🧪️tests/🤝️pool-reader/🟦️.ts"
"$L/🪪️graph-state/📦️project-records/🧪️tests/🚚️closures/🟦️.ts"
"$L/🪪️graph-state/🙋️wanted/💰️cost/🧪️tests/🟦️.ts"
"$L/🧠️memory/📋️slice-report/🧪️tests/🟦️.ts"
"$L/🕸️graph/🕰️generations/🌐️remote-exchange/🧪️tests/🟦️.ts"
"$L/🕸️graph/📥️inference/📦️project-nodes/🔱️warm/🧪️tests/🟦️.ts"
"$L/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🟦️.ts"
"$L/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🪦️reaper/🟦️.ts"
"$L/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧪️tests/🎚️lanes/🟦️.ts"
)
[ -n "$EXTRA" ] && FILES+=($EXTRA)
i=0
run(){ f="$1"; n="$2"; d=$(dirname "$f"); b=$(basename "$f"); (cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN "$B" test "./$b" --timeout 180000 > "$OUT/$n.txt" 2>&1); echo "$n $(grep -E '^ *[0-9]+ (pass|fail)' "$OUT/$n.txt" | tr '\n' ' ') :: ${f#$L/}"; }
for f in "${FILES[@]}"; do i=$((i+1)); run "$f" "$i" & if (( i % 4 == 0 )); then wait; fi; done; wait
