#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R170
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R170
F=/Users/ueli/Documents/semio/🧰️framework
O="$F/🔨️modules/🏃️process/🔒️leases/🧭️order/🧪️tests"
RES="$F/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
m=$((10#$(date -u +%M))); until [ $m -lt 5 ] || { [ $m -ge 30 ] && [ $m -lt 35 ]; }; do sleep 1; m=$((10#$(date -u +%M))); done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R170 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land.py restore $S/bk; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $S/bk; python3 $D/land.py apply $S/bk || fail apply
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
run(){ cd "$1" || fail cd; shift; for t in "$@"; do out=$(env -u SEMIO_FLEET_GROUP nice -n 10 $BUN test $t --timeout 120000 2>&1); echo "$t $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$out"|grep -q "^ *0 fail" || fail "$t"; done; }
run "$O" ./🟦️.ts ./📥️processes/🟦️.ts ./🕸️wait-for/🟦️.ts ./🪟️registry-root/🟦️.ts ./🧩️component-driver/🟦️.ts ./🧭️sites/🟦️.ts ./🌍️ground/🟦️.ts
run "$F/🔨️modules/🏃️process/🔒️leases/📈️standing/🧪️tests" ./🟦️.ts
run "$RES/🏡️resident/🧪️tests/🧯️failure" ./🟦️.ts
run "$RES/🏡️resident/🧪️tests/🥞️failure-frames" ./🟦️.ts
run "$RES/🏡️resident/🧪️tests/🏷️worker-error" ./🟦️.ts
run "$RES/🐢️speed/🧪️tests" ./🟦️.ts
echo "GREEN $START $(date -u +%H:%M:%S)"; release
