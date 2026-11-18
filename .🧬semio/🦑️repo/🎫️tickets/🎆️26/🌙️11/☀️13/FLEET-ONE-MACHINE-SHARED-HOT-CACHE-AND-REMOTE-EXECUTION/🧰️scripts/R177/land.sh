#!/bin/zsh
# R177 landing: live <- staged for the rows of land.tsv; base copy = restore; new files (empty base) are removed on restore.
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R177
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
rows=("${(@f)$(cat $S/land.tsv)}")
for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; st=${rest%%$'\t'*}; base=${rest#*$'\t'}
  if [ -n "$base" ]; then [ "$(shasum -a 256 < $base)" = "$(shasum -a 256 < $live)" ] || { echo "live file moved: $live"; exit 2; }; else [ ! -e "$live" ] || { echo "new file exists: $live"; exit 2; }; fi; done
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 170 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R177 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
restore(){ for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; base=${rest#*$'\t'}; if [ -n "$base" ]; then cp "$base" "$live"; else rm -f "$live"; rmdir "$(dirname $live)" 2>/dev/null; fi; done; echo "restored $(date -u +%H:%M:%S)"; }
fail(){ echo "RED: $1"; restore; release; exit 3; }
for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; st=${rest%%$'\t'*}; mkdir -p "$(dirname $live)"; cp "$st" "$live" || fail copy; done
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
N="$B/🕸️graph/📥️inference/📦️project-nodes"; X="$B/🕸️graph/📥️load/🤝️selected-exchange"; R="$B/🧠️memory/📋️slice-report"; G="$B/🧪️tests/🕸️graph"
for f in "$X/🧪️tests/🫀️idle-claim/🟦️.ts" "$R/🧪️tests/🪜️read-steps/🟦️.ts" "$N/📥️take/🧪️tests/🔒️decided/🟦️.ts" "$G/🧊️stale-facts/🟦️.ts" "$G/🌊️seal-under-churn/🟦️.ts" "$G/🩹️repair-rounds/🟦️.ts" "$R/🧪️tests/🟦️.ts" "$X/🧪️tests/🟦️.ts" "$N/📥️take/🧪️tests/🟦️.ts"; do
  d=$(dirname "$f"); b=$(basename "$f"); out=$(cd "$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./$b --timeout 120000 2>&1); echo "${f#$B/} $(echo "$out"|/usr/bin/grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"; echo "$out"|/usr/bin/grep -q "^ *0 fail" || fail "$f"; done
echo "GREEN $START $(date -u +%H:%M:%S)"; release
