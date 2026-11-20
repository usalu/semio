#!/bin/zsh
# R206 landing: per-phase time ledger of the cargo task path. Test mode: R206_ROOT=<scratch root> R206_NOWINDOW=1 R206_NOTEST=1 R206_LOCKBASE=<dir> [R206_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R206_ROOT:-$REAL}
T=${R206_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
K=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R206
NB="$ROOT/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build"
FR="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$NB/🟦️.ts|$K/native.base.ts.txt|$K/native.new.ts.txt" "$FR|$K/front.base.ts.txt|$K/front.new.ts.txt")
NEWF=("$NB/🧮️phase-ledger/🟦️.ts|$K/ledger.new.ts.txt" "$NB/🧮️phase-ledger/🧬️schema/🔣️.json|$K/ledger-schema.new.json.txt" "$NB/🧮️phase-ledger/🧪️tests/🟦️.ts|$K/ledger-test.new.ts.txt")
h(){ shasum -a 256 < "$1"; }
inwin(){ mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; }
if [ -z "$R206_NOWINDOW" ]; then until inwin; do mm=$((10#$(date -u +%M))); [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ -n "$R206_NOWINDOW" ] || inwin || { echo "window closed while waiting for the lock"; exit 4; }; [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R206 phase-ledger $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
for row in "${NEWF[@]}"; do f=${row%%|*}; [ -e "$f" ] && { echo "new file exists, nothing written: $f"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; for row in "${NEWF[@]}"; do f=${row%%|*}; rm -f "$f"; done; rmdir "$NB/🧮️phase-ledger/🧪️tests" "$NB/🧮️phase-ledger/🧬️schema" "$NB/🧮️phase-ledger" 2>/dev/null; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
for row in "${NEWF[@]}"; do f=${row%%|*}; src=${row#*|}; mkdir -p "${f%/*}" && cp "$src" "$f" || { restore; exit 3; }; done
[ -n "$R206_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R206_NOTEST" ] && { echo "GREEN(no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
ENVU=(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN)
cd "$NB/🧮️phase-ledger/🧪️tests"; out=$($ENVU nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "phase-ledger: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -20; restore; exit 3; }
cd "$ROOT" && $BUN build --no-bundle "$NB/🟦️.ts" >/dev/null && $BUN build --no-bundle "$FR" >/dev/null && $BUN -e 'const m=await import("'"$NB"'/🧮️phase-ledger/🟦️.ts");console.log("import ok",Object.keys(m).length)' || { echo "smoke failed"; restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
