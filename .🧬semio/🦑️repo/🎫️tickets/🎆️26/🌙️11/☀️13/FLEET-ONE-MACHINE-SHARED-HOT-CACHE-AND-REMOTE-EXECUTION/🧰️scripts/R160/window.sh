#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R160
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R160
until [ $((10#$(date -u +%M))) -ge 30 ] && [ $((10#$(date -u +%M))) -lt 35 ]; do sleep 1; done
waited=0
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; waited=$((waited+2)); [ $waited -lt 240 ] || { echo "never got the lock: $(cat $T/landing.lock/owner)"; exit 1; }; done
echo "claude-R160 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S); echo "lock taken $START after ${waited}s"
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; }
fail(){ echo "RED: $1"; python3 $D/land.py restore runtime $S/bk-runtime; python3 $D/land.py restore tests $S/bk-tests; release; echo "restored $(date -u +%H:%M:%S)"; exit 3; }
rm -rf $S/bk-tests $S/bk-runtime $S/after
python3 $D/land.py apply tests $S/bk-tests || { python3 $D/land.py restore tests $S/bk-tests; release; exit 2; }
python3 $D/land.py apply runtime $S/bk-runtime || fail apply
$D/battery.sh $S/after
python3 $D/compare.py $S/base/summary.txt $S/after/summary.txt || fail worse
grep -q "plan-scale|3pass|0fail" $S/after/summary.txt || fail plan-scale
grep -q "🛰️plan|13pass|0fail" $S/after/summary.txt || grep -q "🛰️plan|[0-9]*pass|0fail" $S/after/summary.txt || fail plan
grep -q "growth|[0-9]*pass|[1-4]fail" $S/after/summary.txt || fail growth-shape
G=$S/after/$(echo "⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧪️tests/📈️growth" | md5 -q).txt
grep -E "^\(fail\) (every declared ceiling|ceilings of the same)" $G && fail growth-members
grep -E '^\+ +"' $G | grep -E "targets|tasks" && fail growth-undeclared
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
release
cat $S/after/summary.txt | cut -c1-220
