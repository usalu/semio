#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
D=$T/🧰️scripts/R171
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R171
LIVE="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🟦️.ts"
m=$((10#$(date -u +%M))); [ $m -lt 5 ] || { [ $m -ge 30 ] && [ $m -lt 35 ]; } || { echo "not in a landing window ($m)"; exit 9; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "lock held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R171 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
echo "start $(date -u +%H:%M:%S)"
cp "$LIVE" $S/live.before.ts; shaB=$(shasum -a 256 "$LIVE" | cut -c1-12)
release(){ rm -f "$T/landing.lock/owner"; rmdir "$T/landing.lock"; echo "released $(date -u +%H:%M:%S)"; }
restore(){ cp $S/live.before.ts "$LIVE"; echo "restored $(shasum -a 256 "$LIVE" | cut -c1-12) (before $shaB)"; }
python3 $D/patches.py "$LIVE" $S/live.after.ts || { release; exit 2; }
cp $S/live.after.ts "$LIVE"; echo "applied $(shasum -a 256 "$LIVE" | cut -c1-12)"
$D/battery.sh > $S/after.txt 2>&1; cat $S/after.txt
ok=1
head -10 $S/baseline.txt | diff - <(head -10 $S/after.txt) > /dev/null || ok=0
grep -q "selected-exchange/🧪️tests: 17/0" $S/after.txt || ok=0
grep -q "🔌️attempt: 3/0" $S/after.txt || ok=0
grep -q "🔗️wiring: 3/0" $S/after.txt || ok=0
if [ $ok = 0 ]; then restore; fi
release; echo "ok=$ok"
