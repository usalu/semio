#!/bin/zsh
# usage: battery.sh <map.json> <label>
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R177
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 R177_MAP=$1
N="$B/🕸️graph/📥️inference/📦️project-nodes"; X="$B/🕸️graph/📥️load/🤝️selected-exchange"; R="$B/🧠️memory/📋️slice-report"; G="$B/🧪️tests/🕸️graph"
files=("$G/🧊️stale-facts/🟦️.ts" "$G/🌊️seal-under-churn/🟦️.ts" "$G/🩹️repair-rounds/🟦️.ts" "$G/🔄️candidate/🟦️.ts" "$G/🧲️adopt-registry/🟦️.ts" "$R/🧪️tests/🟦️.ts" "$R/🧪️tests/🏁️final-line/🟦️.ts" "$X/🧪️tests/🟦️.ts" "$X/🧪️tests/🔌️attempt/🟦️.ts" "$X/🧪️tests/🔗️wiring/🟦️.ts" "$N/📥️take/🧪️tests/🟦️.ts" "$N/📥️take/🧪️tests/🚦️no-progress/🟦️.ts" "$N/📥️take/🧪️tests/🪧️tail-report/🟦️.ts" "$N/🧪️tests/🐘️never-here/🟦️.ts" "$N/🧪️tests/🩻️heavy-refusal/🟦️.ts" "$N/🧪️tests/🔭️look-job/🟦️.ts" "$N/🧪️tests/🛣️lanes/🟦️.ts" "$N/🧪️tests/🤝️selected-fleet/🟦️.ts" "$N/🧪️tests/🪂️set-aside/🟦️.ts" "$N/🧪️tests/🛑️too-heavy/🟦️.ts" "$N/🧪️tests/🔁️store-changed/🟦️.ts")
for f in $files; do [ -f "$f" ] || { echo "MISSING $f"; continue; }; d=$(dirname "$f"); b=$(basename "$f"); out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test --preload $S/preload.ts ./$b --timeout 180000 2>&1); echo "$2 ${f#$B/} $(echo "$out"|/usr/bin/grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; done
