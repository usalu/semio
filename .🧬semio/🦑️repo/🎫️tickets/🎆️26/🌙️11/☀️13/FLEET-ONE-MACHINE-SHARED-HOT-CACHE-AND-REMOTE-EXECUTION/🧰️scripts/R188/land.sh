#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R188
F=/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
NB="$F/📦️artifacts/🏗️native-build/🟦️.ts"
C="$F/🦀️cargo-workspace/🔒️lock/📐️closure"
M="$S/mirror/🔨️modules/🏃️process"
[ "$(shasum -a 256 < $S/base/native-build.ts)" = "$(shasum -a 256 < $NB)" ] || { echo "live native-build moved"; exit 2; }
[ ! -e "$C" ] || { echo "closure dir exists"; exit 2; }
mm=$(date -u +%M); { [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; } || { echo "outside window $mm"; exit 4; }
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R188 closure-lock-projection $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
trap release EXIT
restore(){ cp $S/base/native-build.ts "$NB"; rm -rf "$C"; echo "restored $(date -u +%H:%M:%S)"; }
fail(){ echo "RED: $1"; restore; exit 3; }
[ "$(shasum -a 256 < $S/base/native-build.ts)" = "$(shasum -a 256 < $NB)" ] || fail "moved under lock"
mkdir -p "$C/🧬️schema" "$C/🧪️tests" || fail mkdir
cp "$M/🦀️cargo-workspace/🔒️lock/📐️closure/🟦️.ts" "$C/🟦️.ts" && cp "$M/🦀️cargo-workspace/🔒️lock/📐️closure/🧬️schema/🔣️.json" "$C/🧬️schema/🔣️.json" && cp "$M/🦀️cargo-workspace/🔒️lock/📐️closure/🧪️tests/🟦️.ts" "$C/🧪️tests/🟦️.ts" || fail copy
cp "$M/📦️artifacts/🏗️native-build/🟦️.ts" "$NB" || fail copy-nb
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
cd "$C/🧪️tests" || fail cd
out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "closure tests: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
echo "$out"|grep -q "^ *0 fail" || { echo "$out" | tail -30; fail closure-tests; }
cd "$F/📦️artifacts/🏗️native-build" || fail cd2
loaded=$(env -u SEMIO_FLEET_GROUP $BUN -e 'const m=await import("./🟦️.ts");console.log(Object.keys(m).length)' 2>&1 | tail -1); echo "native-build loads: $loaded exports"
case "$loaded" in ''|*[!0-9]*) fail native-build-load;; esac
echo "GREEN $START $(date -u +%H:%M:%S)"
