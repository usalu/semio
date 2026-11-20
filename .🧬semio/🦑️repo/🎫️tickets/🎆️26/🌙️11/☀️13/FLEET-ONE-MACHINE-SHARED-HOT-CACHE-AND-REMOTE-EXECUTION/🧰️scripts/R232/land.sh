#!/bin/zsh
# R232 landing: bound replans of a stale dependency retirement plan and a stable stamp for directories the bindings read pins.
# Test mode: R232_ROOT=<scratch root> R232_LOCKBASE=<scratch dir> R232_NOWINDOW=1 R232_SETTLE=0 [R232_FORCE_RED=1] [R232_NOTESTS=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R232_ROOT:-$REAL}
T=${R232_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R232
TSV=$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R232/land.tsv
[ -d "$S/new" ] && [ -d "$S/base" ] || { mkdir -p "$S"; tar -xf "$(dirname "$TSV")/stage.tar" -C "$S" || { echo "no stage"; exit 5; }; }
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
if [ -z "$R232_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 1; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R232 stable-binding-stamp-and-bounded-replan $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
while IFS=$'\t' read -r kind rel name; do
 live="$ROOT/$rel"
 if [ "$kind" = M ]; then [ "$(h "$S/base/$name")" = "$(h "$live")" ] || { echo "live differs from base, nothing written: $live"; exit 2; }
 else [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; fi
done < "$TSV"
restore(){ while IFS=$'\t' read -r kind rel name; do live="$ROOT/$rel"; if [ "$kind" = M ]; then cp "$S/base/$name" "$live"; else rm -f "$live"; rmdir "$(dirname "$live")" 2>/dev/null; fi; done < "$TSV"; echo "restored"; while IFS=$'\t' read -r kind rel name; do live="$ROOT/$rel"; if [ "$kind" = M ]; then [ "$(h "$S/base/$name")" = "$(h "$live")" ] || echo "RESTORE MISMATCH $live"; else [ ! -e "$live" ] || echo "RESTORE MISMATCH $live"; fi; done < "$TSV"; }
while IFS=$'\t' read -r kind rel name; do [ "$kind" = N ] || continue; live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$name" "$live" || { restore; exit 3; }; done < "$TSV"
echo "new files written $(date -u +%H:%M:%S), waiting ${R232_SETTLE:-120}s for the replica"; sleep ${R232_SETTLE:-120}
while IFS=$'\t' read -r kind rel name; do [ "$kind" = M ] || continue; [ "$(h "$S/base/$name")" = "$(h "$ROOT/$rel")" ] || { echo "live differs from base after settle, new files removed, nothing else written: $ROOT/$rel"; while IFS=$'\t' read -r k2 r2 n2; do [ "$k2" = N ] && { rm -f "$ROOT/$r2"; rmdir "$(dirname "$ROOT/$r2")" 2>/dev/null; }; done < "$TSV"; exit 2; }; done < "$TSV"
while IFS=$'\t' read -r kind rel name; do [ "$kind" = M ] || continue; live="$ROOT/$rel"; cp "$S/new/$name" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R232_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
[ -n "$R232_NOTESTS" ] && { echo "GREEN (no tests) $START $(date -u +%H:%M:%S)"; exit 0; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
C="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"; P="$C/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication"; R="$P/⚙️setup/♻️retirement"; D="$C/🚀️bootstrap/📦️dependencies/💾️storage"
DIRS=("$D/🧪️tests/🔖️stable-stamp" "$R/🧪️tests/🌀️bindings-moved" "$R/🧪️tests/🔁️busy-driver" "$R/🧪️tests" "$R/🧪️tests/📏️roster-bound" "$R/🧪️tests/🪦️stale-generation" "$C/📖️texts/🧪️tests" "$D/🧪️tests/🚰️deferred-flush" "$D/🧪️tests/🧾️field-diagnosis")
for d in "${DIRS[@]}"; do
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${d#$ROOT/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -Eq "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }
done
mkdir -p "$S/art"; cd "$ROOT"; out=$(SEMIO_TEST_ARTIFACT_DIR="$S/art" env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test "$D/🧪️tests/👓️reading/🟦️.ts" --timeout 180000 2>&1); echo "$out" | grep -E "^\((pass|fail)\)" | sed 's/ \[.*//' | sort > "$S/reading-live.set"
echo "reading: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
diff "$S/reading-base.set" "$S/reading-live.set" > "$S/reading.diff" || { echo "reading differs from the baseline:"; head -20 "$S/reading.diff"; restore; exit 3; }
cd "$ROOT" && $BUN -e 'const a=await import("'"$D"'/🟦️.ts");const b=await import("'"$P"'/🟦️.ts");const c=await import("'"$R"'/🟦️.ts");console.log("import smoke ok",typeof a.javascriptDependencyBindingStampV1,typeof b.requireRepositoryDependencySelectionCurrentV1,typeof c.bindingsMovedV1)' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
