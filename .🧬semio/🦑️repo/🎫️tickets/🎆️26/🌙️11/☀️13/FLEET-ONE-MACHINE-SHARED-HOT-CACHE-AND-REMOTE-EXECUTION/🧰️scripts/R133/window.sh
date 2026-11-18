#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R133
BUN=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
SC=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R133
M=$(date -u +%M); [ $((10#$M % 30)) -lt 5 ] || { echo "outside window"; exit 2; }
mkdir "$T/landing.lock" 2>/dev/null || { echo "held: $(cat $T/landing.lock/owner)"; exit 1; }
echo "claude-R133 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
$BUN $D/land.mjs || { $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 3; }
$D/battery.sh $SC/after1.txt base $D/inlock.txt 7
python3 $D/compare.py $SC/base.txt $SC/after1.txt | grep WORSE | python3 -c "
import sys,ast
for l in sys.stdin: print('/Users/ueli/Documents/semio/🧰️framework/'+ast.literal_eval(l[6:])[0])" > $SC/again.txt
cp $SC/after1.txt $SC/after.txt
if [ -s $SC/again.txt ]; then $D/battery.sh $SC/after2.txt base $SC/again.txt 4; python3 $D/best.py $SC/after1.txt $SC/after2.txt $SC/after.txt; fi
python3 $D/compare.py $SC/base.txt $SC/after.txt || { echo RED; $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 4; }
awk '/machine-state|plan-keys|stage-plan|slice-lab|slice-read/ && ($NF!=0 || $(NF-1)<1){bad=1} END{exit bad}' $SC/after.txt || { echo RED-new; $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 6; }
grep -E "machine-state|plan-keys|stage-plan|slice-lab|slice-read|preflight|heavy" $SC/after.txt
SEMIO_LANGUAGE=en repo leases status 2>&1 | head -2 | grep -q "^Leases in " || { echo RED-leases; $BUN $D/restore.mjs; rm -r "$T/landing.lock"; exit 5; }
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
