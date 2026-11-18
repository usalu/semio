#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R176
R=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
typeset -A L; L=(tree.ts "$R/🌲️process-tree/🟦️.ts" tree.json "$R/🌲️process-tree/🧬️schema/🔣️.json" ready.ts "$R/🟦️.ts" stale-test.ts "$R/🧪️tests/🕰️stale-stage/🟦️.ts")
for k in ${(k)L}; do [ "$(shasum -a 256 < $S/base/$k)" = "$(shasum -a 256 < ${L[$k]})" ] || { echo "live file moved: $k"; exit 2; }; done
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R176 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
restore(){ for k in ${(k)L}; do cp $S/base/$k ${L[$k]}; done; echo "restored $(date -u +%H:%M:%S)"; }
fail(){ echo "RED: $1"; restore; release; exit 3; }
for k in ${(k)L}; do cp $S/staged/$k ${L[$k]} || fail copy; done
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
for d in "$R/🧪️tests/🕰️stale-stage" "$R/🌲️process-tree/🧪️tests"; do cd "$d" || fail cd; out=$(env -u SEMIO_FLEET_GROUP nice -n 10 $BUN test ./🟦️.ts --timeout 120000 2>&1); echo "$d $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$out"|grep -q "^ *0 fail" || fail "$d"; done
echo "GREEN $START $(date -u +%H:%M:%S)"; release
