#!/bin/zsh
# R214 landing: leaf pieces travel with owner records. Test mode: R214_ROOT=<scratch root> R214_NOWINDOW=1 [R214_FORCE_RED=1]
REAL=/Users/ueli/Documents/semio
ROOT=${R214_ROOT:-$REAL}
T=${R214_LOCKBASE:-$REAL/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R214
TSV=$T/🧰️scripts/R214/land.tsv
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
h(){ shasum -a 256 < "$1" | cut -c1-64; }
if [ -z "$R214_NOWINDOW" ]; then until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; [ $mm -ge 5 ] && [ $mm -lt 28 ] && { echo "window missed"; exit 4; }; sleep 2; done; fi
mkdir -p "$T"
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 150 ] || { echo "no lock: $(cat $T/landing.lock/owner 2>/dev/null)"; exit 1; }; done
echo "claude-R214 leaf-pieces-travel-with-records $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"' EXIT
while IFS=$'\t' read -r kind rel; do
 live="$ROOT/$rel"
 if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || { echo "live differs from base, nothing written: $live"; exit 2; }
 else [ ! -e "$live" ] || { echo "new file exists, nothing written: $live"; exit 2; }; fi
done < "$TSV"
restore(){ while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then cp "$S/base/$rel" "$live"; else rm -f "$live"; fi; done < "$TSV"; for d in "$ROOT/$B/🪪️graph-state/📦️project-records/🍃️leaves/🧪️tests" "$ROOT/$B/🪪️graph-state/📦️project-records/🍃️leaves/🧬️schema" "$ROOT/$B/🪪️graph-state/📦️project-records/🍃️leaves"; do rmdir "$d" 2>/dev/null; done; echo "restored"; while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; if [ "$kind" = M ]; then [ "$(h "$S/base/$rel")" = "$(h "$live")" ] || echo "RESTORE MISMATCH $live"; else [ ! -e "$live" ] || echo "RESTORE MISMATCH $live"; fi; done < "$TSV"; }
while IFS=$'\t' read -r kind rel; do live="$ROOT/$rel"; mkdir -p "$(dirname "$live")"; cp "$S/new/$rel" "$live" || { restore; exit 3; }; done < "$TSV"
[ -n "$R214_FORCE_RED" ] && { echo "forced red"; restore; exit 3; }
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
G="$ROOT/$B/🪪️graph-state"; PR="$G/📦️project-records"; PN="$ROOT/$B/🕸️graph/📥️inference/📦️project-nodes"
FILES=("$PR/🍃️leaves/🧪️tests/🟦️.ts" "$PR/🧪️tests/🟦️.ts" "$PR/🧪️tests/🚚️closures/🟦️.ts" "$PR/🧪️tests/📮️retry/🟦️.ts" "$PR/🧪️tests/🗜️packed/🟦️.ts" "$PR/🧪️tests/🌍️two-platforms/🟦️.ts" "$PR/🧪️tests/🤝️pool-reader/🟦️.ts" "$PR/🧪️tests/🌊️seed-stream/🟦️.ts" "$PR/🧪️tests/✂️after-cut/🟦️.ts" "$PR/🧪️tests/🏷️owner-token/🟦️.ts" "$PR/📮️unsent/🧪️tests/🟦️.ts" "$PR/🚚️entry-lane/🧪️tests/🟦️.ts" "$G/🛰️exchange/🧯️refusal/🧪️tests/🟦️.ts" "$G/🧪️tests/🟦️.ts" "$PN/🧪️tests/📥️take/🟦️.ts" "$PN/🧪️tests/🤝️selected-fleet/🟦️.ts")
for f in "${FILES[@]}"; do
 cd "$(dirname "$f")"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "${f#$ROOT/$B/}: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }
done
cd "$ROOT" && $BUN -e 'await import("'"$PR"'/🟦️.ts");await import("'"$G"'/🛰️exchange/🟦️.ts");console.log("import smoke ok")' || { restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
