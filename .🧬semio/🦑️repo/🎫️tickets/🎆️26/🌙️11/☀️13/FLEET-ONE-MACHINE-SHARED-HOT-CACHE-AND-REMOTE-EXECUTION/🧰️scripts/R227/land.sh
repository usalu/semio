#!/bin/zsh
# R227 landing: affected reverse index. Phase 1: new files (inert) under the lock; wait 120 s; phase 2: re-check bases under the lock, anchored edits of 4 files, tests, exact restore on red.
# Test mode: R227_ROOT=<scratch root> R227_LOCKBASE=<scratch dir> R227_NOWINDOW=1 R227_WAIT=0 [R227_FORCE_RED=1] [R227_NOTESTS=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R227_ROOT:-$REAL}
T=${R227_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
D=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R227
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R227-land
TSV=$D/land.tsv
[ -d "$S/stage" ] && [ -d "$S/base" ] || { mkdir -p "$S"; tar -xf "$D/stage.tar" -C "$S" || { echo "no stage"; exit 5; }; }
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
window(){ until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do { [ $mm -ge 5 ] && [ $mm -lt 28 ]; } && { echo "window missed"; exit 4; }; { [ $mm -ge 35 ] && [ $mm -lt 58 ]; } && { echo "window missed"; exit 4; }; sleep 5; done; }
lock(){ mkdir -p "$T"; waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done; echo "claude-R227 affected-reverse-index $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; HELD=1; }
unlock(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; HELD=0; }
HELD=0; trap '[ "$HELD" = 1 ] && { rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock" 2>/dev/null; }' EXIT
[ -z "$R227_NOWINDOW" ] && window
lock; echo "phase 1 lock $(date -u +%H:%M:%S)"
while IFS=$'\t' read -r kind rel; do [ "$kind" = N ] && { [ ! -e "$ROOT/$rel" ] || { echo "new file exists, nothing written: $rel"; exit 2; }; }; done < "$TSV"
while IFS=$'\t' read -r kind rel; do [ "$kind" = N ] && { mkdir -p "$(dirname "$ROOT/$rel")"; cp "$S/stage/$rel" "$ROOT/$rel"; }; done < "$TSV"
echo "new files written"; unlock
sleep ${R227_WAIT:-120}
[ -z "$R227_NOWINDOW" ] && window
lock; START=$(date -u +%H:%M:%S); echo "phase 2 lock $START"
while IFS=$'\t' read -r kind rel; do [ "$kind" = M ] && { [ "$(h "$S/base/$rel")" = "$(h "$ROOT/$rel")" ] || { echo "live differs from base, nothing modified: $rel (new files stay, inert)"; exit 2; }; }; done < "$TSV"
rm -rf "$S/new"; python3 "$D/edits.py" "$ROOT" "$S/new" >/dev/null || { echo "anchor mismatch, nothing modified"; exit 2; }
restore(){ while IFS=$'\t' read -r kind rel; do [ "$kind" = M ] && cp "$S/base/$rel" "$ROOT/$rel"; done < "$TSV"; echo "modified files restored"; while IFS=$'\t' read -r kind rel; do [ "$kind" = M ] && { [ "$(h "$S/base/$rel")" = "$(h "$ROOT/$rel")" ] || echo "RESTORE MISMATCH $rel"; }; done < "$TSV"; }
while IFS=$'\t' read -r kind rel; do [ "$kind" = M ] && { cp "$S/new/$rel" "$ROOT/$rel" || { restore; exit 3; }; }; done < "$TSV"
[ -n "$R227_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R227_NOTESTS" ] && { echo "GREEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
G="$ROOT/$B/🕸️graph"; A="$G/🎯️affected-index/🧪️tests"; GT="$G/🕰️generations/🧪️tests"
for f in "$A/🧮️oracle" "$A/📏️size" "$A/💾️store" "$A/🗣️texts" "$A/🚦️plan" "$A/🔏️seal-hook" "$GT/🔁️owner-replacement" "$GT/🌊️waves" "$GT/🔖️journal" "$GT/🏷️renamed-owner"; do
 cd "$f"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 300000 2>&1)
 echo "$out" | grep -q "^ *0 fail" || { echo "$out" | tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$ROOT/$B"'/🕸️graph/🕰️host/🟦️.ts");await import("'"$ROOT/$B"'/⌨️command/⚙️runtime/🟦️.ts");await import("'"$G"'/🎯️affected-index/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
