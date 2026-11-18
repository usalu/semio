#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R192
L=/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library
BS="$L/🏗️build/🚀️bootstrap"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$BS/🔷️typescript-compiler/🟨️.mjs|$S/base/🔷️typescript-compiler/🟨️.mjs|$S/stage/🔷️typescript-compiler/🟨️.mjs"
"$BS/🔷️typescript-compiler/🟨️.d.mts|$S/base/🔷️typescript-compiler/🟨️.d.mts|$S/stage/🔷️typescript-compiler/🟨️.d.mts"
"$BS/🔷️typescript-compiler/🧬️schema/🔣️.json|$S/base/🔷️typescript-compiler/🧬️schema/🔣️.json|$S/stage/🔷️typescript-compiler/🧬️schema/🔣️.json"
"$BS/📦️parsers/🟨️.mjs|$S/base/📦️parsers/🟨️.mjs|$S/stage/📦️parsers/🟨️.mjs"
"$L/🕸️dependencies/🟦️typescript/🟨️.mjs|$S/base/dep.mjs|$S/stage/dep/🟨️.mjs"
"$BS/🔷️typescript-compiler/🧪️tests/🚫️bare-name/🟦️.ts|$S/base/law.ts|$S/stage/law.ts")
h(){ shasum -a 256 < "$1"; }
until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 3; done
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live moved: $live"; exit 2; }; done
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 120 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R192 typescript-compiler-captured-load $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; echo "restored"; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
for d in "$BS/🔷️typescript-compiler/🧪️tests" "$BS/🔷️typescript-compiler/🧪️tests/🍞️captured" "$BS/🔷️typescript-compiler/🧪️tests/🚫️bare-name"; do
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "$d: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }
done
cd $S; out=$(nice -n 10 $BUN smoke.mjs 2>&1); echo "smoke: $out"; echo "$out"|grep -q "5.9.3" || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
