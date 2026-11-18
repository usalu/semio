#!/bin/zsh
# R203 landing: unit ledger line and per-task hash lines at the end of an Nx run. Test mode: R203_ROOT=<scratch root> R203_NOWINDOW=1 R203_LOCKBASE=<dir> [R203_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R203_ROOT:-$REAL}
T=${R203_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
K=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R203
LIB="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
CNT="$LIB/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📊️counters"
SET="$LIB/🏗️build/🌐️remote-cache/🧶️nx/🪢️slot-settle"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$CNT/🟦️.ts|$K/counters.base.ts|$K/counters.new.ts" "$SET/🟦️.ts|$K/slot-settle.base.ts|$K/slot-settle.new.ts" "$LIB/⚡️caching/🚀️bootstrap/📜️script.ts|$K/script.base.ts|$K/script.new.ts")
NEWF=("$CNT/🧪️tests/🟦️.ts|$K/counters.test.ts.txt" "$SET/🧪️tests/🟦️.ts|$K/settle.test.ts.txt")
h(){ shasum -a 256 < "$1"; }
inwin(){ mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; }
if [ -z "$R203_NOWINDOW" ]; then until inwin; do mm=$((10#$(date -u +%M))); [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ -n "$R203_NOWINDOW" ] || inwin || { echo "window closed while waiting for the lock"; exit 4; }; [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R203 unit-ledger-and-task-hash-lines $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
for row in "${NEWF[@]}"; do f=${row%%|*}; [ -e "$f" ] && { echo "new test file exists, nothing written: $f"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; for row in "${NEWF[@]}"; do f=${row%%|*}; rm -f "$f"; rmdir "${f%/*}" 2>/dev/null; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
for row in "${NEWF[@]}"; do f=${row%%|*}; src=${row#*|}; mkdir -p "${f%/*}" && cp "$src" "$f" || { restore; exit 3; }; done
[ -n "$R203_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
for d in "$CNT/🧪️tests" "$SET/🧪️tests"; do
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${d##*/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -20; restore; exit 3; }
done
cd "$ROOT" && $BUN build --no-bundle "$LIB/⚡️caching/🚀️bootstrap/📜️script.ts" >/dev/null && $BUN -e 'const c=await import("'"$CNT"'/🟦️.ts"),s=await import("'"$SET"'/🟦️.ts");console.log("import smoke",typeof c.unitLedgerV1,typeof s.nxTaskLedgerV1)' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
