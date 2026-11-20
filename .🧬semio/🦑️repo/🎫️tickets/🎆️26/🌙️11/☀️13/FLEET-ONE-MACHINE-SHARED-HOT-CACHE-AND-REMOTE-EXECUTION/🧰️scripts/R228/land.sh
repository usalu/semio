#!/bin/zsh
# R228 landing, new files only: hermetic task member (state root apart from source root, view as only source, declared outputs). Test mode: R228_ROOT=<scratch root> R228_NOWINDOW=1 [R228_FORCE_RED=1] [R228_SKIPTESTS=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R228_ROOT:-$REAL}
T=${R228_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R228
TSV=${R228_TSV:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R228/land.tsv}
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏞️task-view/🧱️hermetic"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
if [ -z "$R228_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R228 hermetic-task-member $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
while IFS=$'\t' read -r kind rel; do [ ! -e "$ROOT/$rel" ] || { echo "new file exists, nothing written: $ROOT/$rel"; exit 2; }; done < "$TSV"
[ ! -e "$ROOT/$M" ] || [ -z "$(ls -A "$ROOT/$M" 2>/dev/null)" ] || { echo "member directory not empty, nothing written"; exit 2; }
restore(){ while IFS=$'\t' read -r kind rel; do rm -f "$ROOT/$rel"; done < "$TSV"; rmdir "$ROOT/$M/🧪️tests" "$ROOT/$M/🧬️schema" "$ROOT/$M" 2>/dev/null; echo "restored"; while IFS=$'\t' read -r kind rel; do [ ! -e "$ROOT/$rel" ] || echo "RESTORE MISMATCH $ROOT/$rel"; done < "$TSV"; [ ! -e "$ROOT/$M" ] || echo "RESTORE MISMATCH $ROOT/$M"; }
while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R228_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R228_SKIPTESTS" ] && { echo "tests skipped (test mode)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
cd "$ROOT/$M/🧪️tests" || { restore; exit 3; }; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "hermetic: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
echo "$out"|grep -q "^ *[0-9]* pass" && ! echo "$out"|grep -q "^(fail)" || { echo "$out"|tail -30; restore; exit 3; }
for d in "🏞️task-view/🧪️tests" "🏞️task-view/🏃️executor/🧪️tests"; do cd "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/$d" || { restore; exit 3; }; out=$(env -u SEMIO_FLEET_GROUP nice -n 19 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "$d: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$out"|grep -q "^(fail)" && { echo "$out"|tail -20; restore; exit 3; }; done
cd "$ROOT" && $BUN -e 'await import("'"$ROOT/$M/🟦️.ts"'");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
