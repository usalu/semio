#!/bin/zsh
setopt nobgnice
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R143
export R143_SCRATCH=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R143
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; done
echo "claude-R143 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
python3 $D/land2.py || { rm -r "$T/landing.lock"; echo LAND-FAILED; exit 3; }
python3 $D/battery2.py all 150 | sed 's#🧰️framework/##' > $R143_SCRATCH/battery2-after.txt
cat $R143_SCRATCH/battery2-after.txt
python3 $D/compare.py $R143_SCRATCH/battery2-base.txt $R143_SCRATCH/battery2-after.txt || { echo RED; python3 $D/restore2.py; rm -r "$T/landing.lock"; exit 4; }
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
