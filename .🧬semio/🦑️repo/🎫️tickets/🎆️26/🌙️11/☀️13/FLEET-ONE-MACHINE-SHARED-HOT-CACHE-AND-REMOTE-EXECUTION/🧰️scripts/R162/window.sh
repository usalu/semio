#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R162
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R162
TD="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🧪️tests"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
until [ $((10#$(date -u +%M))) -lt 5 ]; do sleep 1; done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R162 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land.py restore $SCR/bk; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SCR/bk; python3 $D/land.py apply $SCR/bk || fail apply
cd $TD || fail cd
env -u SEMIO_FLEET_GROUP SEMIO_TEST_ARTIFACT_DIR=$SCR/art nice -n 10 $BUN test ./🟦️.ts ./🌊️churn/🟦️.ts ./♻️refresh/🟦️.ts ./⏩️frontier/🟦️.ts ./🧾️receipt/🟦️.ts --timeout 180000 > $SCR/after.txt 2>&1
grep -E "^\(fail\)| pass| fail|Ran " $SCR/after.txt
grep -q " 0 fail" $SCR/after.txt || fail tests
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
