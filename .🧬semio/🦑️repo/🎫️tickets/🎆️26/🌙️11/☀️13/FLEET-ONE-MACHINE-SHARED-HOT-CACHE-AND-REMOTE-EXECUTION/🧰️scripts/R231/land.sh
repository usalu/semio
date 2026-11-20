#!/bin/zsh
ROOT=${R231_ROOT:-/Users/ueli/Documents/semio}
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
LOCK=${R231_LOCK:-$T/landing.lock}
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/60403654-b0e9-4c0d-a7d3-fba5340a3de9/scratchpad/R231/stage
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
READLOG=/Users/ueli/Documents/semio/.cache/f/graph-server/read.log
if [ -z "$R231_REHEARSE" ]; then
 if kill -0 18668 2>/dev/null && ! grep -q "\[graph-seal\]" "$READLOG"; then echo "read pid 18668 not sealed: nothing done"; exit 5; fi
 [ "$R231_SEALED_BY_COORDINATOR" = "1" ] || { echo "set R231_SEALED_BY_COORDINATOR=1 after the coordinator's message"; exit 5; }
 until mm=$((10#$(date -u +%M))); [ $mm -lt 5 ] || { [ $mm -ge 30 ] && [ $mm -lt 35 ]; }; do [ $mm -ge 35 ] && [ $mm -lt 49 ] && { echo "window missed"; exit 4; }; sleep 3; done
fi
waited=0; until mkdir "$LOCK" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 120 ] || { echo "no lock: $(cat $LOCK/owner)"; exit 1; }; done
echo "claude-R231 replica-compaction-automatic $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$LOCK/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
trap 'rm -f "$LOCK/owner"; rmdir "$LOCK"' EXIT
FILES=("${(@f)$(cat $S/land.tsv)}")
for f in $FILES; do cmp -s "$S/base/$f" "$ROOT/$f" || { echo "live moved: $f (nothing written)"; exit 2; }; done
restore(){ for f in $FILES; do cp "$S/base/$f" "$ROOT/$f"; done; for f in $FILES; do cmp -s "$S/base/$f" "$ROOT/$f" || echo "RESTORE MISMATCH $f"; done; echo "restored"; }
for f in $FILES; do cp "$S/new/$f" "$ROOT/$f" || { restore; exit 3; }; done
export BUN_RUNTIME_TRANSPILER_CACHE_PATH=0
for d in "$ROOT/$M/🗺️topology/🧪️tests" "$ROOT/$M/🗜️compaction/🧪️tests"; do
 cd "$d"; out=$(env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE -u SEMIO_REMOTE_CACHE_TOKEN nice -n 10 $BUN test ./🟦️.ts --timeout 180000 2>&1); echo "$d: $(echo "$out"|grep -E '^ *[0-9]+ (pass|fail)'|tr -d ' '|tr '\n' ' ')"
 echo "$out"|grep -q "^ *0 fail" || { echo "$out"|tail -30; restore; exit 3; }
done
[ -n "$R231_FORCE_RED" ] && { echo forced red; restore; exit 3; }
echo "GREEN $START $(date -u +%H:%M:%S)"
