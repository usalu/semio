#!/bin/zsh
# R200 landing: shared tool/provider groups per script input cache. Test mode: R200_ROOT=<scratch mirror root> R200_LOCKBASE=<dir> R200_NOWINDOW=1 R200_NOTESTS=1 [R200_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R200_ROOT:-$REAL}
T=${R200_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R200
REL="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs"
SI="$ROOT/$REL"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
LIVE="$SI/🟨️.mjs"; BASE="$S/s/si.base"; NEW="$S/s/si.new.mjs"
LAWDIR="$SI/🧪️tests/🗂️shared-groups"; LAW="$LAWDIR/🟦️.ts"; LAWSRC="$S/law/law.ts"
h(){ shasum -a 256 < "$1"; }
if [ -z "$R200_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R200 shared-tool-groups $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
[ "$(h $BASE)" = "$(h $LIVE)" ] || { echo "live differs from base, nothing written: $LIVE"; exit 2; }
[ ! -e "$LAW" ] || { echo "law exists, nothing written"; exit 2; }
restore(){ cp "$BASE" "$LIVE"; rm -f "$LAW"; rmdir "$LAWDIR" 2>/dev/null; echo "restored"; [ "$(h $BASE)" = "$(h $LIVE)" ] || echo "RESTORE MISMATCH"; [ ! -e "$LAWDIR" ] || echo "LAW DIR LEFT"; }
cp "$NEW" "$LIVE" || { restore; exit 3; }
mkdir -p "$LAWDIR" && cp "$LAWSRC" "$LAW" || { restore; exit 3; }
[ -n "$R200_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R200_NOTESTS" ] && { echo "WRITTEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
while read -r rel; do
 [ -z "$rel" ] && continue
 d=$(dirname "${SI%/*}/$rel"); n=$(basename "$rel")
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./$n --timeout 180000 2>&1)
 echo "$out" | grep -qE "^ *0 fail" || { echo "$out" > "$S/land-fail.log"; echo "red: $rel"; echo "$out" | grep -E "^\(fail\)"; restore; exit 3; }
done < "$S/land-tests.txt"
cd "$ROOT" && $BUN -e 'await import("'"$SI"'/🟨️.mjs");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
