#!/bin/zsh
# R195 landing: step-overdue stop, stage ending in words, process table clock. Test mode: R195_ROOT=<scratch mirror root> R195_NOWINDOW=1 [R195_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R195_ROOT:-$REAL}
T=${R195_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R195
R="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$R/🟦️.ts|$S/base/ready.ts|$S/stage/ready.ts"
"$R/🧬️schema/🔣️.json|$S/base/ready.schema.json|$S/stage/ready.schema.json"
"$R/🌲️process-tree/🟦️.ts|$S/base/tree.ts|$S/stage/tree.ts"
"$R/🌲️process-tree/🧬️schema/🔣️.json|$S/base/tree.schema.json|$S/stage/tree.schema.json"
"$R/🌲️process-tree/🧹️containment/🟦️.ts|$S/base/cont.ts|$S/stage/cont.ts"
"$R/🌲️process-tree/🧹️containment/🧬️schema/🔣️.json|$S/base/cont.schema.json|$S/stage/cont.schema.json")
h(){ shasum -a 256 < "$1"; }
if [ -z "$R195_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R195 step-overdue-and-stage-ending $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
[ -n "$R195_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R195_NOTESTS" ] && { echo "WRITTEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
DIRS=("$R/🧪️tests" "$R/🧪️tests/🪜️step-overdue" "$R/🧪️tests/🕰️stale-stage" "$R/🧪️tests/🔁️resume" "$R/🧪️tests/🧷️failure-log" "$R/🧪️tests/🪜️stage-plan" "$R/🌲️process-tree/🧪️tests" "$R/🌲️process-tree/🧹️containment/🧪️tests" "$R/🌲️process-tree/🧪️tests/🤝️holder-custody")
OUT=$S/out-land; mkdir -p $OUT; rm -f $OUT/*; n=0
for d in "${DIRS[@]}"; do n=$((n+1)); ( cd "$d"; env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 > $OUT/$n.txt 2>&1 ) & done
wait
bad=0; n=0
for d in "${DIRS[@]}"; do n=$((n+1)); echo "$d: $(grep -E '^ *[0-9]+ (pass|fail)' $OUT/$n.txt|tr -d ' '|tr '\n' ' ')"; grep -q "^ *0 fail" $OUT/$n.txt || { bad=1; tail -20 $OUT/$n.txt; }; done
[ $bad = 0 ] || { echo "RED: tolerating only baseline reds is a human decision; restoring"; restore; exit 3; }
cd "$ROOT" && $BUN -e 'await import("'"$R"'/🟦️.ts");await import("'"$R"'/🌲️process-tree/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
