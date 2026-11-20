#!/bin/zsh
# R218 landing: verified dependency store without exclusive access; one typed lease timeout. Test mode: R218_ROOT=<scratch root> R218_NOWINDOW=1 [R218_FORCE_RED=1] [R218_NOTESTS=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R218_ROOT:-$REAL}
T=${R218_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R218
TSV=$S/land.tsv
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
if [ -z "$R218_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 120 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R218 verified-dependency-store-and-typed-lease-timeout $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
while IFS=$'\t' read -r kind rel; do
 live="$ROOT/$rel"
 if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || { echo "live differs from base, nothing written: $live"; exit 2; }
 else [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; fi
done < "$TSV"
restore(){ while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then cp "$S/base/$rel" "$live"; else rm -f "$live"; fi; done < "$TSV"; rmdir "$ROOT/🧰️framework/🔨️modules/🏃️process/🔒️leases/🧪️tests/⌛️timeout-refusal" 2>/dev/null; echo "restored"; }
while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R218_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R218_NOTESTS" ] && { echo "GREEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_ARTIFACT_DIR=$S/art
F="$ROOT/🧰️framework"; P="$F/🔨️modules/🏃️process"; LB="$F/🛍️products/🦑️repo/🔨️modules/📚️library"; ST="$LB/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup"
FILES=("$P/🔒️leases/🧪️tests/⌛️timeout-refusal/🟦️.ts" "$P/🔒️leases/🧭️order/🧪️tests/🟦️.ts" "$P/🔒️leases/🧭️order/🧪️tests/🌍️ground/🟦️.ts" "$P/🏗️build/🗣️language/📣️progress/🧪️tests/🏷️typed-refusals/🟦️.ts" "$P/🏗️build/🗣️language/📣️progress/🧪️tests/🟦️.ts" "$P/🏗️build/🗣️language/📣️progress/🧪️tests/🪟️wrapped-refusal/🟦️.ts" "$LB/🏗️build/🏡️resident/🧪️tests/🧯️failure/🟦️.ts" "$ST/🧪️tests/⏳️busy-funding/🟦️.ts" "$ST/🧪️tests/🟦️.ts" "$ST/🧪️tests/🌊️moving-tree/🟦️.ts" "$ST/♻️retirement/🧪️tests/🔁️busy-driver/🟦️.ts" "$ST/♻️retirement/🧪️tests/🟦️.ts" "$LB/⚡️caching/📖️texts/🧪️tests/🟦️.ts" "$P/🔒️leases/⏳️wait/🧪️tests/🟦️.ts" "$P/🔒️leases/🧪️tests/🪦️stale-queue/🟦️.ts")
for f in "${FILES[@]}"; do
 cd "$(dirname "$f")"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${f#$F/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr '\n' ' ')"
 bad=$(echo "$out"|grep -E '^\(fail\)'|grep -v "the real process ancestry equals an independent walk with ps")
 [ -z "$bad" ] || { echo "$bad"|head -20; FAILED=1; }
done
[ -z "$FAILED" ] || { restore; exit 3; }
cd "$ROOT" && $BUN -e 'await import("'"$P"'/🔒️leases/🟦️.ts");await import("'"$ST"'/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
