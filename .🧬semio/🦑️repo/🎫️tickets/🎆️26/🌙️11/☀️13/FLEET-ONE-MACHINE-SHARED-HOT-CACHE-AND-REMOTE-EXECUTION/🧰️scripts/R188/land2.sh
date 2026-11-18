#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R188
C=/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🦀️cargo-workspace/🔒️lock/📐️closure
M="$S/mirror/🔨️modules/🏃️process/🦀️cargo-workspace/🔒️lock/📐️closure"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1"; }
until mm=$(date -u +%M); [ $mm -ge 30 ] && [ $mm -lt 35 ]; do [ $mm -ge 35 ] && [ $mm -lt 59 ] && { echo "window missed"; exit 4; }; sleep 3; done
[ "$(h $S/base2/ts.ts)" = "$(h $C/🟦️.ts)" ] && [ "$(h $S/base2/schema.json)" = "$(h $C/🧬️schema/🔣️.json)" ] && [ "$(h $S/base2/test.ts)" = "$(h $C/🧪️tests/🟦️.ts)" ] || { echo "live moved"; exit 2; }
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R188 manifest-projection $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
restore(){ cp $S/base2/ts.ts "$C/🟦️.ts"; cp $S/base2/schema.json "$C/🧬️schema/🔣️.json"; cp $S/base2/test.ts "$C/🧪️tests/🟦️.ts"; echo "restored"; }
cp "$M/🟦️.ts" "$C/🟦️.ts" && cp "$M/🧬️schema/🔣️.json" "$C/🧬️schema/🔣️.json" && cp "$M/🧪️tests/🟦️.ts" "$C/🧪️tests/🟦️.ts" || { restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
cd "$C/🧪️tests"
out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "closure tests: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
