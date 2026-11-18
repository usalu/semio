#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R165
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R165
TD="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🧾️incremental/🧪️tests/🏎️ports"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
m=$((10#$(date -u +%M))); until [ $m -lt 5 ] || { [ $m -ge 30 ] && [ $m -lt 35 ]; }; do sleep 1; m=$((10#$(date -u +%M))); done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 200 ] || { echo "no lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R165 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"; START=$(date -u +%H:%M:%S); echo "lock $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land-w1.py restore $SCR/bk; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SCR/bk; python3 $D/land-w1.py apply $SCR/bk || fail apply
cd $TD || fail cd
env -u SEMIO_FLEET_GROUP SEMIO_TEST_ARTIFACT_DIR=$SCR/art nice -n 10 $BUN test ./🟦️.ts --timeout 120000 > $SCR/after-w1.txt 2>&1
grep -E "^\(fail\)| pass| fail|Ran " $SCR/after-w1.txt
grep -q " 0 fail" $SCR/after-w1.txt || fail tests
echo "GREEN $START $(date -u +%H:%M:%S)"; release
