#!/bin/zsh
# R219 landing (coordinator variant: new files first, replica settle, then modified files): whole-workspace generation shared by the fleet (look, whole-read gate, current fetch). Test mode: R219_ROOT=<scratch root> R219_LOCKBASE=<scratch dir> R219_NOWINDOW=1 [R219_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R219_ROOT:-$REAL}
T=${R219_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R219
TSV=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R219/land.tsv
[ -d "$S/new" ] && [ -d "$S/base" ] || { mkdir -p "$S"; tar -xf "$(dirname "$TSV")/stage.tar" -C "$S" || { echo "no stage"; exit 5; }; }
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
if [ -z "$R219_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-coordinator-R219 whole-generation-fleet-gate $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
while IFS=$'\t' read -r kind rel; do
 live="$ROOT/$rel"
 if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || { echo "live differs from base, nothing written: $live"; exit 2; }
 else [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; fi
done < "$TSV"
restore(){ while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then cp "$S/base/$rel" "$live"; else rm -f "$live"; rmdir -p "$(dirname "$live")" 2>/dev/null; fi; done < "$TSV"; echo "restored"; while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || echo "RESTORE MISMATCH $live"; else [ ! -e "$live" ] || echo "RESTORE MISMATCH $live"; fi; done < "$TSV"; }
while IFS=$'\t' read -r kind rel; do [ "$kind" = N ] || continue; live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
echo "new files written $(date -u +%H:%M:%S), waiting ${R219_SETTLE:-120}s for the replica"; sleep ${R219_SETTLE:-120}
while IFS=$'\t' read -r kind rel; do [ "$kind" = M ] || continue; [ "$(h "$S/base/$rel")" = "$(h "$ROOT/$rel")" ] || { echo "live differs from base after settle, new files removed, nothing else written: $ROOT/$rel"; while IFS=$'\t' read -r k2 r2; do [ "$k2" = N ] && { rm -f "$ROOT/$r2"; rmdir -p "$(dirname "$ROOT/$r2")" 2>/dev/null; }; done < "$TSV"; exit 2; }; done < "$TSV"
while IFS=$'\t' read -r kind rel; do [ "$kind" = M ] || continue; live="$ROOT/$rel"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R219_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
GEN="$ROOT/$B/🕸️graph/🕰️generations"; LOAD="$ROOT/$B/🕸️graph/📥️load/🤝️selected-exchange"; WG="$GEN/🤝️whole-gate"
FILES=("$WG/🧪️tests/🟦️.ts" "$WG/🧪️tests/🌍️machines/🟦️.ts" "$WG/🧪️tests/📏️head-size/🟦️.ts" "$GEN/🌐️remote-exchange/🧪️tests/🟦️.ts" "$GEN/🌐️remote-exchange/🧪️tests/🔭️scope/🟦️.ts" "$GEN/🌐️remote-exchange/🧪️tests/🏁️head-rank/🟦️.ts" "$GEN/🌐️remote-exchange/🧪️tests/📡️wave-delta/🟦️.ts" "$LOAD/🧪️tests/🟦️.ts" "$LOAD/🧪️tests/🔗️wiring/🟦️.ts" "$LOAD/🧪️tests/🖥️real-server/🟦️.ts" "$GEN/🔌️load/🧪️tests/🔒️current-never-builds/🟦️.ts" "$GEN/🔌️load/🧪️tests/🪑idle-builder/🟦️.ts" "$GEN/🔌️load/🧪️tests/🟦️.ts")
for f in "${FILES[@]}"; do
 cd "$(dirname "$f")"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${f#$ROOT/$B/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 known=""; [ "$f" = "$GEN/🔌️load/🧪️tests/🟦️.ts" ] && known="progress counters are the shared graph phases"; [ "$f" = "$GEN/🔌️load/🧪️tests/🪑idle-builder/🟦️.ts" ] && known="two readers on one state root|a refresh that is cut off keeps every published wave"
 if [ -n "$known" ]; then
  fails=$(echo "$out"|grep '^(fail)'|grep -Evc "$known"); [ "$fails" = 0 ] || { echo "$out"|tail -30; restore; exit 3; }
 else echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }; fi
done
cd "$ROOT" && $BUN -e 'await import("'"$ROOT/$B"'/🕸️graph/🕰️host/🟦️.ts");await import("'"$WG"'/🟦️.ts");await import("'"$GEN"'/🔌️load/🟨️.mjs");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
