#!/bin/zsh
# R201 landing 1: settled grants are charged by their measurement. Test mode: R201_ROOT=<scratch mirror root> R201_NOWINDOW=1 [R201_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R201_ROOT:-$REAL}
T=${R201_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
K=$T/🧰️scripts/R201
HC="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
PO="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$ROOT/$HC/📏️decision/🟦️.ts|$K/decision.base.ts" "$ROOT/$HC/🧬️schema/🔣️.json|$K/schema.base.json" "$ROOT/$PO|$K/policy.base.json")
NEWT="$ROOT/$HC/🧪️tests/🧱️settled-read/🟦️.ts"
h(){ shasum -a 256 < "$1"; }
if [ -z "$R201_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R201 settled-grant-booking $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; base=${row#*|}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
[ -e "$NEWT" ] && { echo "new test exists, nothing written"; exit 2; }
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; base=${row#*|}; cp "$base" "$live"; done; rm -f "$NEWT"; rmdir "$(dirname $NEWT)" 2>/dev/null; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; base=${row#*|}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; [ -e "$NEWT" ] && echo "RESTORE MISMATCH new test"; }
python3 "$K/patch.py" "$ROOT" || { restore; exit 3; }
mkdir -p "$(dirname $NEWT)" && cp "$K/law.ts" "$NEWT" || { restore; exit 3; }
[ -n "$R201_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
LIBT="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests"
DIRS=("$ROOT/$HC/🧪️tests/🧱️settled-read" "$ROOT/$HC/🧪️tests/🤲️lend" "$ROOT/$HC/🧪️tests/🤫️quiet" "$ROOT/$HC/🧪️tests/🪜️phases" "$ROOT/$HC/🧪️tests/🌤️followup" "$ROOT/$HC/🧪️tests/🥇️essential" "$ROOT/$HC/🧪️tests/🔼️upgrade" "$ROOT/$HC/🧪️tests/🚪️gate" "$ROOT/$HC/🧪️tests/🧺️unmanaged" "$ROOT/$HC/🧪️tests/🪂️partial-start" "$ROOT/$HC/🧪️tests/📐️estimates" "$ROOT/$HC/🧪️tests/🔋️available" "$LIBT/🏊️farm" "$LIBT/👥️agents" "$LIBT/🧮️farm-width" "$LIBT/🧾️label")
for d in "${DIRS[@]}"; do
 [ -d "$d" ] || { echo "missing $d"; restore; exit 3; }
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "$d: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out" > "$K/land-fail.log"; echo "$out"|grep -E "^\(fail\)"; restore; exit 3; }
done
cd "$ROOT/$HC/🧪️tests" && out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "main: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
echo "$out"|grep -q "^ *0 fail" || { echo "$out" > "$K/land-fail.log"; echo "$out"|grep -E "^\(fail\)"; restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
