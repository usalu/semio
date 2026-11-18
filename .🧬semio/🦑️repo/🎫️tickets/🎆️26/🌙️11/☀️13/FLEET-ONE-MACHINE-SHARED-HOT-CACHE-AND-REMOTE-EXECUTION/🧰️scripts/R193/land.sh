#!/bin/zsh
# R193 landing: live <- staged for the rows of land.tsv; base copy = restore. Only inside a window (hh:00-05, hh:30-35 UTC). usage: land.sh [check]
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R193
B=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
F="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
rows=("${(@f)$(cat $S/land.tsv)}")
bases(){ for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; base=${rest#*$'\t'}; [ "$(shasum -a 256 < $base)" = "$(shasum -a 256 < $live)" ] || { echo "live file moved: $live"; return 1; }; done; }
bases || exit 2
[ "$1" = "check" ] && { echo "bases match"; exit 0; }
m=$(date -u +%M); m=$((10#$m)); { [ $m -lt 5 ] || { [ $m -ge 30 ] && [ $m -lt 35 ]; }; } || { echo "outside window ($m)"; exit 4; }
waited=0; until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 120 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R193 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock" 2>/dev/null; }
trap release EXIT
bases || exit 2
restore(){ for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; base=${rest#*$'\t'}; cp "$base" "$live"; done; echo "restored $(date -u +%H:%M:%S)"; }
fail(){ echo "RED: $1"; restore; exit 3; }
for r in $rows; do live=${r%%$'\t'*}; rest=${r#*$'\t'}; st=${rest%%$'\t'*}; cp "$st" "$live" || fail copy; done
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
for f in "$F/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/🧪️tests/🟦️.ts" "$F/📖️texts/🧪️tests/🟦️.ts"; do
  d=$(dirname "$f"); b=$(basename "$f"); out=$(cd "$d" && env -u BUN_OPTIONS -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $B test ./$b --timeout 600000 2>&1); echo "${f#$F/} $(echo "$out"|/usr/bin/grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
  echo "$out"|/usr/bin/grep -q "^ *0 fail" || { echo "$out" | /usr/bin/grep -E '^\(fail\)' | head; fail "$f"; }; done
echo "GREEN $START $(date -u +%H:%M:%S)"
