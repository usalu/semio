#!/bin/zsh
# R220 landing (known red on the base and not touched here: the followup test "a failed write is removed through the descriptor"): packed ownership observations (typed arrays, interned names, packed listings), lazy observed rows, streaming generation evidence. Test mode: R220_ROOT=<scratch root> R220_NOWINDOW=1 [R220_FORCE_RED=1] [R220_SKIPTESTS=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R220_ROOT:-$REAL}
T=${R220_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R220
TSV=${R220_TSV:-$T/🧰️scripts/R220/land.tsv}
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
M="$L/🗂️workspaces/🟦️bun"
G="$L/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
if [ -z "$R220_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R220 packed-ownership-observations $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
while IFS=$'\t' read -r kind rel; do
 live="$ROOT/$rel"
 if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || { echo "live differs from base, nothing written: $live"; exit 2; }
 else [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; fi
done < "$TSV"
restore(){ while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then cp "$S/base/$rel" "$live"; else rm -f "$live"; fi; done < "$TSV"; rmdir "$ROOT/$M/🧪️tests/🗜️packed" 2>/dev/null; echo "restored"; while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || echo "RESTORE MISMATCH $live"; else [ ! -e "$live" ] || echo "RESTORE MISMATCH $live"; fi; done < "$TSV"; }
while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R220_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R220_SKIPTESTS" ] && { echo "tests skipped (test mode)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
IN="$ROOT/$L/🏗️build/🕸️graph/📥️inference/🧭️script-inputs"
FILES=("$ROOT/$M/🧪️tests/🟦️.ts" "$ROOT/$M/🧪️tests/🧭️closure/🟦️.ts" "$ROOT/$M/🧪️tests/🌊️churn/🟦️.ts" "$ROOT/$M/🧪️tests/🧾️receipt/🟦️.ts" "$ROOT/$M/🧪️tests/🗜️packed/🟦️.ts" "$ROOT/$G/🧪️tests/🟦️.ts" "$ROOT/$G/🧪️tests/🛡️followup/🟦️.ts" "$ROOT/$G/🧪️tests/🚪️ingress/🟦️.ts" "$ROOT/$G/🧪️tests/🌡️heap-environment/🟦️.ts" "$IN/🔌️provider-closures/🧪️tests/🛃️preflight/🟦️.ts" "$IN/🪆️heavy-questions/🧪️tests/🟦️.ts")
mkdir -p "$S/art-land"
for f in "${FILES[@]}"; do
 cd "$(dirname "$f")"; out=$(SEMIO_TEST_ARTIFACT_DIR=$S/art-land env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${f#$ROOT/$L/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *[0-9]* pass" || { echo "$out"|tail -30; restore; exit 3; }
 bad=$(echo "$out"|grep "^(fail)"|grep -v -F "followup files: a failed write is removed through the descriptor"); [ -z "$bad" ] || { echo "$bad"; echo "$out"|tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$ROOT/$M/🟦️.ts"'");await import("'"$ROOT/$G/💾️storage/🟦️.ts"'");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
