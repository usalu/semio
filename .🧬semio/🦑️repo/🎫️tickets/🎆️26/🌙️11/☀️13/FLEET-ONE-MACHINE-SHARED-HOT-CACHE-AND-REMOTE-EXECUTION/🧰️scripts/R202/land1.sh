#!/bin/zsh
# R202 landing 1: farm queue order (shortest expected first with aging), hot lane first in the take pass, prime counts line. Test mode: R202_ROOT=<scratch root> R202_NOWINDOW=1 [R202_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R202_ROOT:-$REAL}
T=${R202_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
K=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R202
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R202
PNR="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes"
PN="$ROOT/$PNR"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$PN/🏊️project-farm/🏊️pool/🟨️.mjs|$K/pool.base.mjs|$S/stage/pool.new.mjs"
"$PN/🏊️project-farm/🏊️pool/🟨️.d.mts|$K/pool.base.d.mts|$S/stage/pool.new.d.mts"
"$PN/🏊️project-farm/🏊️pool/🧬️schema/🔣️.json|$K/poolschema.base.json|$S/stage/poolschema.new.json"
"$PN/📥️take/🟨️.mjs|$K/take.base.mjs|$S/stage/take.new.mjs"
"$PN/📥️take/🟨️.d.mts|$K/take.base.d.mts|$S/stage/take.new.d.mts"
"$PN/🟨️.mjs|$K/pn.base.mjs|$S/stage/pn.new.mjs")
NEWD=("$PN/🏊️project-farm/🧪️tests/⏱️queue-order|$K/law/queue-order" "$PN/📥️take/🧪️tests/🔥️hot-lane|$K/law/hot-lane")
h(){ shasum -a 256 < "$1"; }
if [ -z "$R202_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R202 farm-queue-order-hot-lane $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
for row in "${NEWD[@]}"; do d=${row%%|*}; [ -e "$d" ] && { echo "new test directory exists, nothing written: $d"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; for row in "${NEWD[@]}"; do d=${row%%|*}; [ -d "$d" ] && [ "${d##*/}" != "" ] && rm -f "$d"/🟦️.ts "$d"/🔣️.json "$d"/🐍️.py; rmdir "$d" 2>/dev/null; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; for row in "${NEWD[@]}"; do d=${row%%|*}; [ -e "$d" ] && echo "RESTORE MISMATCH new test $d"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
for row in "${NEWD[@]}"; do d=${row%%|*}; src=${row#*|}; mkdir -p "$d" && cp "$src"/* "$d"/ || { restore; exit 3; }; done
[ -n "$R202_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
DIRS=("$PN/🏊️project-farm/🧪️tests/⏱️queue-order" "$PN/📥️take/🧪️tests/🔥️hot-lane" "$PN/📥️take/🧪️tests" "$PN/📥️take/🧪️tests/📣️published" "$PN/🧪️tests/📥️take" "$PN/🔭️look-queue/🧪️tests" "$PN/🏊️project-farm/🧪️tests/🏘️hub" "$PN/🏊️project-farm/🧪️tests/🧮️admission" "$PN/🏊️project-farm/🧪️tests/🔭️snapshot" "$PN/🏊️project-farm/🧪️tests/🧯️cap" "$PN/🏊️project-farm/🧪️tests/🧊️alone-fresh" "$PN/🏊️project-farm/🧪️tests/🛑️hold" "$PN/🏊️project-farm/🧪️tests/🔱️warm-jobs" "$PN/🏊️project-farm/🧪️tests/🪢️late-refusal")
for d in "${DIRS[@]}"; do
 [ -f "$d/🟦️.ts" ] || { echo "missing $d"; restore; exit 3; }
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${d##*/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out" > "$S/land1-fail.log"; echo "$out"|grep -E "^\(fail\)"; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$PN"'/🟨️.mjs");await import("'"$PN"'/🏊️project-farm/🏊️pool/🟨️.mjs");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
