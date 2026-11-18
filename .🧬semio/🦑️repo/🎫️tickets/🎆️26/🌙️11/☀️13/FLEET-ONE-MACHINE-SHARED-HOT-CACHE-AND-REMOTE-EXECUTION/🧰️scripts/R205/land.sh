#!/bin/zsh
# R205 landing: fleet hit in two round trips without the slot wait, stable schema index, generate-class law.
# Test mode: R205_ROOT=<scratch root> R205_NOWINDOW=1 R205_NOTEST=1 R205_LOCKBASE=<dir> [R205_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R205_ROOT:-$REAL}
T=${R205_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
K=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R205
LIB="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
UC="$LIB/⚡️caching/🦀️cargo/🗃️unit-cache"
DER="$ROOT/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
FILES=("$UC/🛰️fleet/📥️adopt/🟦️.ts|$K/adopt.base.ts.txt|$K/adopt.new.ts.txt" "$UC/🛰️fleet/🟦️.ts|$K/fleet.base.ts.txt|$K/fleet.new.ts.txt" "$UC/📜️script.ts|$K/script.base.ts.txt|$K/script.new.ts.txt" "$DER/🗂️schema-index/🟦️.ts|$K/schemaindex.base.ts.txt|$K/schemaindex.new.ts.txt" "$DER/🧪️tests/🔬️rust-package-commands/🟦️.ts|$K/commands.base.ts.txt|$K/commands.new.ts.txt" "$DER/📦️packages/🦀️rust/📋️project.json|$K/derive-project.base.json.txt|$K/derive-project.new.json.txt")
NEWF=("$UC/🛰️fleet/🧪️tests/⚡️pipeline/🟦️.ts|$K/test-pipeline.ts.txt" "$DER/🧪️tests/🗂️schema-index-stability/🟦️.ts|$K/test-index-stability.ts.txt" "$LIB/🏗️build/🧪️tests/🏭️generate-class/🟦️.ts|$K/test-generate-class.ts.txt")
h(){ shasum -a 256 < "$1"; }
inwin(){ mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; }
if [ -z "$R205_NOWINDOW" ]; then until inwin; do mm=$((10#$(date -u +%M))); [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ -n "$R205_NOWINDOW" ] || inwin || { echo "window closed while waiting for the lock"; exit 4; }; [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R205 fleet-hit-two-round-trips-and-stable-schema-index $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || { echo "live differs from base, nothing written: $live"; exit 2; }; done
for row in "${NEWF[@]}"; do f=${row%%|*}; [ -e "${f%/*}" ] && { echo "new test directory exists, nothing written: ${f%/*}"; exit 2; }; done
restore(){ for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; cp "$base" "$live"; done; for row in "${NEWF[@]}"; do f=${row%%|*}; rm -f "$f"; rmdir "${f%/*}" 2>/dev/null; done; echo "restored"; for row in "${FILES[@]}"; do live=${row%%|*}; rest=${row#*|}; base=${rest%%|*}; [ "$(h $base)" = "$(h $live)" ] || echo "RESTORE MISMATCH $live"; done; }
for row in "${FILES[@]}"; do live=${row%%|*}; stage=${row##*|}; cp "$stage" "$live" || { restore; exit 3; }; done
for row in "${NEWF[@]}"; do f=${row%%|*}; src=${row#*|}; mkdir -p "${f%/*}" && cp "$src" "$f" || { restore; exit 3; }; done
[ -n "$R205_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R205_NOTEST" ] && { echo "GREEN(no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 SEMIO_TEST_ARTIFACT_DIR=$(mktemp -d)
ENVU=(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN)
for d in "$UC/🛰️fleet/🧪️tests/⚡️pipeline" "$DER/🧪️tests/🗂️schema-index-stability" "$DER/🧪️tests/🗂️schema-document-index" "$LIB/🏗️build/🧪️tests/🏭️generate-class"; do
 cd "$d"; out=$($ENVU nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${d##*/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -20; restore; rm -rf "$SEMIO_TEST_ARTIFACT_DIR"; exit 3; }
done
rm -rf "$SEMIO_TEST_ARTIFACT_DIR"
cd "$ROOT" && $BUN build --no-bundle "$UC/📜️script.ts" >/dev/null && $BUN build --no-bundle "$DER/📦️packages/🦀️rust/📜️script.ts" >/dev/null && $BUN -e 'const f=await import("'"$UC"'/🛰️fleet/🟦️.ts"),a=await import("'"$UC"'/🛰️fleet/📥️adopt/🟦️.ts");console.log("import smoke",typeof f.awaitFleetSettlesV1,typeof a.downloadBlobsV1)' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
