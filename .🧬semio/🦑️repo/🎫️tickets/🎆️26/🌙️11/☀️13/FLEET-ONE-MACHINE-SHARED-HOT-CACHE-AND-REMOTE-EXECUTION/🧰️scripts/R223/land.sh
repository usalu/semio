#!/bin/zsh
# R223 landing: declared waits agree with the stall detector (tool hook heartbeat is a step line), hook failure reasons, tools named before verification. Test mode: R223_ROOT=<scratch root> R223_NOWINDOW=1 R223_NOSETTLE=1 [R223_FORCE_RED=1] [R223_NOTESTS=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R223_ROOT:-$REAL}
T=${R223_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R223
TSV=$S/land.tsv
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
inwin(){ mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; }
if [ -z "$R223_NOWINDOW" ]; then until inwin; do mm=$((10#$(date -u +%M))); [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; [ -n "$R223_NOWINDOW" ] || inwin || { echo "window ended while waiting for the lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 4; }; done
[ -n "$R223_NOWINDOW" ] || inwin || { rmdir "$T/landing.lock"; echo "window ended before the lock was taken"; exit 4; }
echo "claude-R223 declared-wait-step-line-hook-reason-tool-naming $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
checkbases(){ while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"
 if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || { echo "live differs from base, nothing (more) written: $live"; return 1; }
 elif [ "$1" = first ]; then [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; return 1; }; fi
done < "$TSV"; }
checkbases first || exit 2
restore(){ while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then cp "$S/base/$rel" "$live"; else rm -f "$live"; fi; done < "$TSV"; B="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; for d in "$B/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree/🧪️tests/📣️declared-wait" "$B/🧰️tool-setup/🧪️tests/🔍️hook-reason"; do rmdir "$d" 2>/dev/null; done; echo "restored"; }
# new files first, the replica does not apply a landing atomically: wait, re-check the bases, then the modified files
while IFS=$'\t' read -r kind rel; do [ "$kind" = A ] || continue; live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R223_NOSETTLE" ] || sleep 120
checkbases again || { restore; exit 2; }
while IFS=$'\t' read -r kind rel; do [ "$kind" = M ] || continue; live="$ROOT/$rel"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R223_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R223_NOTESTS" ] && { echo "GREEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
B="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; TS="$B/🧰️tool-setup"; PT="$B/🌐️remote-execution/🛰️sidecar/🚦️ready/🌲️process-tree"
FILES=("$PT/🧪️tests/📣️declared-wait/🟦️.ts" "$PT/🧪️tests/🟦️.ts" "$PT/🧪️tests/🪟️windows-progress/🟦️.ts" "$PT/🧪️tests/🤝️holder-custody/🟦️.ts" "$PT/🧪️tests/🛡️shared-helper/🟦️.ts" "$TS/⏱️hook-bound/🧪️tests/🟦️.ts" "$TS/🧪️tests/🔍️hook-reason/🟦️.ts" "$TS/🧪️tests/🟦️.ts" "$TS/🧪️tests/🪜️steps/🟦️.ts" "$TS/⌨️command/🧪️tests/🟦️.ts" "$TS/🧪️tests/📮️dispatch/🟦️.ts")
for f in "${FILES[@]}"; do
 cd "$(dirname "$f")"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 19 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${f#$B/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr '\n' ' ')"
 bad=$(echo "$out"|grep -E '^\(fail\)')
 [ -z "$bad" ] || { echo "$bad"|head -20; FAILED=1; }
done
[ -z "$FAILED" ] || { restore; exit 3; }
cd "$ROOT" && $BUN -e 'await import("'"$TS"'/⌨️command/🟦️.ts");await import("'"$TS"'/⏱️hook-bound/🟦️.ts");await import("'"$PT"'/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
