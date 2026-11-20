#!/bin/zsh
# R222 landing: closed tool recipe (type field, typed refusal), bounded named tool hook and fleet claim wait, native Windows memory. Test mode: R222_ROOT=<scratch root> R222_NOWINDOW=1 [R222_FORCE_RED=1] [R222_NOTESTS=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R222_ROOT:-$REAL}
T=${R222_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R222
TSV=$S/land.tsv
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
if [ -z "$R222_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; [ -n "$R222_NOWINDOW" ] || { mm=$((10#$(date -u +%M))); { [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; } || { echo "window ended while waiting for the lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 4; }; }; done
# a lock taken after the window ended is released at once
[ -n "$R222_NOWINDOW" ] || { mm=$((10#$(date -u +%M))); { [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; } || { rmdir "$T/landing.lock"; echo "window ended before the lock was taken"; exit 4; }; }
echo "claude-R222 closed-tool-recipe-bounded-tool-hook-native-windows-memory $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
while IFS=$'\t' read -r kind rel; do
 live="$ROOT/$rel"
 if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || { echo "live differs from base, nothing written: $live"; exit 2; }
 else [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; fi
done < "$TSV"
restore(){ while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then cp "$S/base/$rel" "$live"; else rm -f "$live"; fi; done < "$TSV"; F0="$ROOT/🧰️framework"; for d in "$F0/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪟️native-memory/🧪️tests/🐍️oracle" "$F0/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪟️native-memory/🧪️tests/🧫️fixtures" "$F0/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪟️native-memory/🧪️tests" "$F0/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪟️native-memory/🧬️schema" "$F0/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪟️native-memory" "$F0/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📦️sdk/📥️source/📐️projection/🧪️tests/🪟️live-recipe" "$F0/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛰️fleet-fetch/🧪️tests/📣️claim-wait" "$F0/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⏱️hook-bound/🧪️tests" "$F0/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⏱️hook-bound/🧬️schema" "$F0/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup/⏱️hook-bound"; do rmdir "$d" 2>/dev/null; done; echo "restored"; }
while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R222_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R222_NOTESTS" ] && { echo "GREEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
F="$ROOT/🧰️framework"; P="$F/🔨️modules/🏃️process"; LB="$F/🛍️products/🦑️repo/🔨️modules/📚️library"; H="$P/🏗️build/🌡️host-capacity"; TL="$LB/⚡️caching/🚀️bootstrap/🛠️tools"; TS="$LB/🏗️build/🧰️tool-setup"; FF="$LB/⚡️caching/🚀️bootstrap/🛰️fleet-fetch"; W="$P/🧪️testing/👷️workers"; PG="$P/🏗️build/🗣️language/📣️progress"
FILES=("$TL/📦️sdk/📥️source/📐️projection/🧪️tests/🪟️live-recipe/🟦️.ts" "$TL/🏗️bazel/🧪️tests/📥️pins/🟦️.ts" "$TL/📦️sdk/📥️source/📐️projection/🧪️tests/🟦️.ts" "$PG/🧪️tests/🏷️typed-refusals/🟦️.ts" "$PG/🧪️tests/🟦️.ts" "$PG/🧪️tests/🪟️wrapped-refusal/🟦️.ts" "$H/🪟️native-memory/🧪️tests/🟦️.ts" "$H/🧪️tests/🪟️windows/🟦️.ts" "$H/🧪️tests/📡️sampling/🟦️.ts" "$H/🧪️tests/🔬️footprint/🟦️.ts" "$H/🧪️tests/🧺️unmanaged/🟦️.ts" "$H/🧪️tests/🚪️gate/🟦️.ts" "$H/🧪️tests/🪂️partial-start/🟦️.ts" "$H/🧪️tests/🕰️stale-sample/🟦️.ts" "$H/🧪️tests/🟦️.ts" "$H/🧪️tests/📐️estimates/🟦️.ts" "$W/📏️file-budgets/🧪️tests/🟦️.ts" "$W/🧪️tests/🧱️capped-run/🟦️.ts" "$LB/🏗️build/🌡️host-capacity/🧪️tests/🪟️partial/🟦️.ts" "$TS/⏱️hook-bound/🧪️tests/🟦️.ts" "$TS/🧪️tests/🟦️.ts" "$TS/⌨️command/🧪️tests/🟦️.ts" "$TS/🧪️tests/🪜️steps/🟦️.ts" "$TS/🧪️tests/📮️dispatch/🟦️.ts" "$FF/🧪️tests/🟦️.ts" "$FF/🧪️tests/📣️claim-wait/🟦️.ts" "$FF/🎟️work-claim/🧪️tests/🟦️.ts")
for f in "${FILES[@]}"; do
 cd "$(dirname "$f")"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${f#$F/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr '\n' ' ')"
 bad=$(echo "$out"|grep -E '^\(fail\)')
 [ -z "$bad" ] || { echo "$bad"|head -20; FAILED=1; }
done
[ -z "$FAILED" ] || { restore; exit 3; }
cd "$ROOT" && $BUN -e 'await import("'"$H"'/📡️sampling/🟦️.ts");await import("'"$H"'/🪟️native-memory/🟦️.ts");await import("'"$TS"'/⌨️command/🟦️.ts");await import("'"$FF"'/🟦️.ts");await import("'"$TL"'/📦️sdk/📥️source/📐️projection/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
