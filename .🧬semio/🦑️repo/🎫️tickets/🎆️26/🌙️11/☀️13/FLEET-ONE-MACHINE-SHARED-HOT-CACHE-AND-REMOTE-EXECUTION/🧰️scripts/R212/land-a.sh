#!/bin/zsh
# R212 landing A: the slice lab judges the physical footprint like the slice guard. Test mode: R212_ROOT=<mirror> R212_NOWINDOW=1 R212_LOCKBASE=<dir> [R212_FORCE_RED=1] R212_SKIPTESTS=1
REAL=/Users/ueli/Documents/semio
ROOT=${R212_ROOT:-$REAL}
T=${R212_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R212/a
LAB="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$LAB/🟦️.ts|$S/base/lab.ts|$S/new/lab.ts" "$LAB/🧬️schema/🔣️.json|$S/base/lab.schema.json|$S/new/lab.schema.json" "$LAB/🧪️tests/🟦️.ts|$S/base/lab.test.ts|$S/new/lab.test.ts")
h(){ shasum -a 256 < "$1"; }
if [ -z "$R212_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 120 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R212 slice-lab-footprint $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE DIFFERS $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
[ -n "$R212_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R212_SKIPTESTS" ] && { echo "tests skipped (mock)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
TESTS=("$LAB/🧪️tests/🟦️.ts" "$LAB/🧪️tests/🧊️frozen/🟦️.ts")
for f in "${TESTS[@]}"; do
 cd "${f:h}"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./${f:t} --timeout 180000 2>&1)
 echo "${f#$ROOT/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr '\n' ' ')"
 echo "$out"|grep -qE "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'const m=await import("'"$LAB"'/🟦️.ts");if(typeof m.sliceLabFootprintV1!=="function")throw Error("export");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
