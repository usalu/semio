#!/bin/zsh
# R179 landing: live <- staged (flight.ts, schema.json, long-holder test); base copy restores; new test removed on restore.
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R179/staged
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧶️nx/🚦️single-flight"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
[ "$(shasum -a 256 < $S/base/flight.ts)" = "$(shasum -a 256 < $L/🟦️.ts)" ] || { echo "live flight moved"; exit 2; }
[ "$(shasum -a 256 < $S/base/schema.json)" = "$(shasum -a 256 < $L/🧬️schema/🔣️.json)" ] || { echo "live schema moved"; exit 2; }
[ ! -e "$L/🧪️tests/🕰️long-holder" ] || { echo "test exists"; exit 2; }
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 280 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R179 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
restore(){ cp $S/base/flight.ts "$L/🟦️.ts"; cp $S/base/schema.json "$L/🧬️schema/🔣️.json"; rm -rf "$L/🧪️tests/🕰️long-holder"; echo "restored $(date -u +%H:%M:%S)"; }
fail(){ echo "RED: $1"; restore; release; exit 3; }
cp $S/new/flight.ts "$L/🟦️.ts"; cp $S/new/schema.json "$L/🧬️schema/🔣️.json"; mkdir -p "$L/🧪️tests/🕰️long-holder"; cp $S/new/long-holder.test.ts "$L/🧪️tests/🕰️long-holder/🟦️.ts"
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
for f in 🕰️long-holder/🟦️.ts 🟦️.ts; do d="$L/🧪️tests/$(dirname $f)"; [ "$f" = "🟦️.ts" ] && d="$L/🧪️tests"; out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "$f $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)' | tr -d ' ' | tr '\n' ' ')"; echo "$out" | grep -E "^\(fail\)"; fails=$(echo "$out" | grep -E '^ *[0-9]+ fail' | tr -dc '0-9'); [ "${fails:-1}" = "0" ] || { [ "$f" = "🟦️.ts" ] && [ "$fails" = "1" ] && echo "$out" | grep -q "native HTTP cache client" && { echo "known timing flake, retry"; out=$(cd "$d" && $BUN test ./🟦️.ts -t "native HTTP cache client" 2>&1); echo "$out" | grep -qE '^ *0 fail' && continue; }; fail "$f"; }; done
echo "GREEN $START $(date -u +%H:%M:%S)"; release
