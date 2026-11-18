#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R155
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R155
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🔬️slice-lab"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 5; waited=$((waited+5)); [ $waited -lt 1500 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R155 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land.py restore $SCR/bk; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SCR/bk
python3 $D/land.py apply $SCR/bk || { release; echo "apply refused"; exit 4; }
for f in "$L/🧪️tests" "$L/🧪️tests/🧊️frozen"; do (cd "$f" && env -u SEMIO_FLEET_GROUP -u SEMIO_REMOTE_CACHE_ENDPOINT $BUN test ./🟦️.ts --timeout 180000 2>&1 | tail -6 | tee $SCR/out.txt | grep -q " 0 fail") || fail "tests in $f"; done
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
