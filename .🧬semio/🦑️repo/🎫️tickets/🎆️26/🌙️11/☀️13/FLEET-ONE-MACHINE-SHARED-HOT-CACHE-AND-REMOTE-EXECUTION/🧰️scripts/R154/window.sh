#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R154
SCR=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R154
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 5; waited=$((waited+5)); [ $waited -lt 1500 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R154 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
fail(){ echo "RED: $1"; python3 $D/land.py restore $SCR/bk; rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $SCR/bk
python3 $D/land.py apply $SCR/bk || fail apply
$D/battery.sh $SCR/after $D/fast.list > /dev/null
python3 $D/compare.py $SCR/base/summary.txt $SCR/after/summary.txt | grep -E "WORSE|->"; python3 $D/compare.py $SCR/base/summary.txt $SCR/after/summary.txt > /dev/null || fail tests
grep -E "fail=[1-9]" $SCR/after/summary.txt | grep -E "stale-facts|final-line|publication-capture|owner-tag|owner-named" && fail "new law red"
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"
