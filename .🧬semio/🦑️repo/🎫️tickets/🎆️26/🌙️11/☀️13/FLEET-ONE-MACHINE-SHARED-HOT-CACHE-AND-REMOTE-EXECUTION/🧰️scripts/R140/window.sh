#!/bin/zsh
setopt nobgnice
cd /Users/ueli/Documents/semio || exit 1
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION
D=$T/🧰️scripts/R140
export R140_SCRATCH=/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R140
until mkdir "$T/landing.lock" 2>/dev/null; do sleep 2; done
echo "claude-R140 $(date -u +%Y-%m-%dT%H:%M:%S)Z" > "$T/landing.lock/owner"
START=$(date -u +%H:%M:%S)
python3 $D/land.py || { python3 $D/restore.py; rm -r "$T/landing.lock"; echo LAND-FAILED; exit 3; }
python3 $D/battery.py all 150 | sed 's#🏗️build/##' > $R140_SCRATCH/battery-after.txt
cat $R140_SCRATCH/battery-after.txt
python3 $D/compare.py $R140_SCRATCH/battery-base.txt $R140_SCRATCH/battery-after.txt || { echo RED; python3 $D/restore.py; rm -r "$T/landing.lock"; exit 4; }
echo "GREEN started $START ended $(date -u +%H:%M:%S)"
rm -r "$T/landing.lock"
